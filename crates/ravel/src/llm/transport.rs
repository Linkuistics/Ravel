//! The HTTP transport of the LLM adapter (`docs/specs/policy-evaluation.md`,
//! *LLM endpoints and credentials*).
//!
//! Rig sends through any [`HttpClientExt`]. This one wraps a reqwest client that
//! Ravel configures, and it applies the bound on every response body before Rig
//! decodes a byte. `rig-reqwest` is not used, because its `into_response` and
//! `non_success_status_error` collect a body with no bound.
//!
//! The client follows these rules:
//!
//! - It ignores ambient proxy variables and system proxy settings.
//! - It follows no redirect, so a credential cannot reach another host.
//! - It makes no retry, and it switches off the protocol retries of reqwest.
//! - It decodes no compressed body, so the bound counts the bytes Rig receives.
//!   The reqwest features for `gzip`, `brotli`, `deflate` and `zstd` are off, so
//!   the client also sends no `Accept-Encoding`.
//! - It uses rustls with the `ring` crypto provider and the trust store of the
//!   platform.
//!
//! A fault that Rig cannot name travels inside its error as a [`Fault`], which
//! the adapter finds again with `downcast_ref`.

use std::fmt;
use std::sync::Once;

use reqwest::redirect::Policy;
use rig_core::http_client::{
    Bytes, Error, HttpClientExt, LazyBody, MultipartForm, Request, Response, Result,
    StreamingResponse,
};
use rig_core::wasm_compat::WasmCompatSend;

/// The most bytes of a successful response body.
pub const MAX_BODY_BYTES: u64 = 8 * 1024 * 1024;

/// The most bytes read of a failure body. The connection closes after them.
pub const MAX_FAILURE_BODY_BYTES: usize = 64 * 1024;

/// What went wrong below Rig, in the terms the adapter classifies.
#[derive(Debug)]
pub enum Fault {
    /// No reply arrived: the connection, the TLS handshake or the request
    /// failed. The text is the chain of causes.
    Network(String),
    /// The reply began and then ended before its body did.
    ShortBody(String),
    /// The body is longer than [`MAX_BODY_BYTES`]. `declared` is true when the
    /// reply announced the length before the first byte of the body.
    OutputLimit { declared: bool },
    /// Rig asked for a kind of request that Ravel never makes.
    Unsupported(&'static str),
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fault::Network(cause) => write!(f, "the request failed: {cause}"),
            Fault::ShortBody(cause) => write!(f, "the response body stopped short: {cause}"),
            Fault::OutputLimit { declared: true } => write!(
                f,
                "the response declares a body longer than {MAX_BODY_BYTES} bytes"
            ),
            Fault::OutputLimit { declared: false } => {
                write!(f, "the response body is longer than {MAX_BODY_BYTES} bytes")
            }
            Fault::Unsupported(what) => write!(f, "Ravel sends no {what} request"),
        }
    }
}

impl std::error::Error for Fault {}

/// The client of the adapter.
#[derive(Clone, Debug)]
pub struct Transport {
    client: reqwest::Client,
}

impl Transport {
    /// Build the client. This fails only when the platform cannot supply a TLS
    /// configuration.
    pub fn new() -> std::result::Result<Self, String> {
        install_provider();
        let client = reqwest::Client::builder()
            .no_proxy()
            .redirect(Policy::none())
            .retry(reqwest::retry::never())
            // A pooled connection would outlive its call and could carry a
            // later call's credential over an old connection. Each call opens
            // its own, and closing the body closes the connection.
            .pool_max_idle_per_host(0)
            .build()
            .map_err(|error| chain(&error))?;
        Ok(Transport { client })
    }
}

/// Install `ring` as the process default of rustls, once. reqwest is built with
/// `rustls-no-provider`, so it reads the default at the construction of a client.
/// https://docs.rs/reqwest/0.13.5/reqwest/tls/index.html
fn install_provider() {
    static INSTALL: Once = Once::new();
    INSTALL.call_once(|| {
        // Another installer in the process wins, and either provider is fine.
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
}

/// An error and each of its sources, separated by `: `. The URL is left out, so
/// the text carries only the cause.
fn chain(error: &reqwest::Error) -> String {
    use std::error::Error as _;
    let top = error.to_string();
    // `Display` of a reqwest error names the URL, which the cause does not need.
    let mut text = top.split(" for url (").next().unwrap_or(&top).to_owned();
    let mut next = error.source();
    while let Some(cause) = next {
        text.push_str(": ");
        text.push_str(&cause.to_string());
        next = cause.source();
    }
    text
}

impl HttpClientExt for Transport {
    fn send<T, U>(
        &self,
        req: Request<T>,
    ) -> impl Future<Output = Result<Response<LazyBody<U>>>> + WasmCompatSend + 'static
    where
        T: Into<Bytes>,
        T: WasmCompatSend,
        U: From<Bytes>,
        U: WasmCompatSend + 'static,
    {
        let (parts, body) = req.into_parts();
        let body: Bytes = body.into();
        let request = self
            .client
            .request(parts.method, parts.uri.to_string())
            .headers(parts.headers)
            .body(body);
        async move {
            let mut response = request
                .send()
                .await
                .map_err(|error| Error::instance(Fault::Network(chain(&error))))?;
            let status = response.status();
            let headers = response.headers().clone();
            if !status.is_success() {
                let body = failure_body(&mut response).await;
                // Dropping the response closes the connection.
                drop(response);
                return Err(Error::non_success_with_details(status, headers, body));
            }
            if response
                .content_length()
                .is_some_and(|length| length > MAX_BODY_BYTES)
            {
                return Err(Error::instance(Fault::OutputLimit { declared: true }));
            }
            let mut head = Response::builder().status(status);
            if let Some(map) = head.headers_mut() {
                *map = headers;
            }
            let body: LazyBody<U> = Box::pin(async move {
                let bytes = collect(response, MAX_BODY_BYTES)
                    .await
                    .map_err(Error::instance)?;
                Ok(U::from(Bytes::from(bytes)))
            });
            head.body(body).map_err(Error::Protocol)
        }
    }

    #[expect(
        clippy::manual_async_fn,
        reason = "the trait fixes the 'static return type, which an async fn on &self does not give"
    )]
    fn send_multipart<U>(
        &self,
        _req: Request<MultipartForm>,
    ) -> impl Future<Output = Result<Response<LazyBody<U>>>> + WasmCompatSend + 'static
    where
        U: From<Bytes>,
        U: WasmCompatSend + 'static,
    {
        async { Err(Error::instance(Fault::Unsupported("multipart"))) }
    }

    #[expect(
        clippy::manual_async_fn,
        reason = "the trait fixes the return type, which an async fn on &self does not give"
    )]
    fn send_streaming<T>(
        &self,
        _req: Request<T>,
    ) -> impl Future<Output = Result<StreamingResponse>> + WasmCompatSend
    where
        T: Into<Bytes> + WasmCompatSend,
    {
        async { Err(Error::instance(Fault::Unsupported("streaming"))) }
    }
}

/// The body of `response`, counted as the bytes arrive. A body past `limit`
/// ends the read at its first byte past the bound, and the caller drops the
/// response, which closes the connection.
async fn collect(
    mut response: reqwest::Response,
    limit: u64,
) -> std::result::Result<Vec<u8>, Fault> {
    let mut body = Vec::new();
    loop {
        match response.chunk().await {
            Ok(Some(chunk)) => {
                if body.len() as u64 + chunk.len() as u64 > limit {
                    return Err(Fault::OutputLimit { declared: false });
                }
                body.extend_from_slice(&chunk);
            }
            Ok(None) => return Ok(body),
            Err(error) => return Err(Fault::ShortBody(chain(&error))),
        }
    }
}

/// At most [`MAX_FAILURE_BODY_BYTES`] of a failure body, as text. A body that
/// cannot be read to that point gives what arrived.
async fn failure_body(response: &mut reqwest::Response) -> String {
    let mut body = Vec::new();
    while body.len() < MAX_FAILURE_BODY_BYTES {
        match response.chunk().await {
            Ok(Some(chunk)) => {
                let room = MAX_FAILURE_BODY_BYTES - body.len();
                body.extend_from_slice(&chunk[..chunk.len().min(room)]);
            }
            Ok(None) | Err(_) => break,
        }
    }
    String::from_utf8_lossy(&body).into_owned()
}
