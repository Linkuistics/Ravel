//! A deterministic local HTTP server that plays scripted responses
//! (`docs/specs/policy-evaluation.md`, *Test seams*).
//!
//! It stands behind the settings of a test as an LLM endpoint, with a dummy key
//! and no network. It answers one scripted [`Reply`] for each request, in order,
//! and it records what each request carried: the method, the path, the headers
//! and the body. A test then checks the path, the credential header, the model,
//! the token limit and the body.
//!
//! It also reports how each connection ended. A reply that streams a long body
//! notes how many bytes it sent and whether the client closed the connection
//! before the whole body went out, so a test can show that Ravel stops reading
//! at its bound.
//!
//! Every reply closes its connection after the exchange (`Connection: close`),
//! and every connection has its own thread, so a reply that stalls blocks no
//! other.

use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::Value;

/// The size of one write of a streamed body.
const CHUNK: usize = 64 * 1024;

/// What the server does with the body of a reply.
#[derive(Clone, Debug)]
pub enum Body {
    /// The bytes, with a `Content-Length` of their length.
    Fixed(Vec<u8>),
    /// `prefix`, then `fill` bytes of `b'x'`, then `suffix`: `total` bytes in
    /// all. With `declared`, the head announces `Content-Length: total`.
    /// Without it, the body goes out chunked, so no length is announced. The
    /// server writes in chunks and notes whether a write failed because the
    /// client had closed the connection.
    Streamed {
        prefix: Vec<u8>,
        suffix: Vec<u8>,
        total: usize,
        declared: bool,
    },
    /// A head that announces `declared` bytes, then `sent` of them, then the
    /// close of the connection.
    Short { declared: usize, sent: Vec<u8> },
    /// A head with no body, then a wait of `hold` or until the client closes.
    Stall { hold: Duration },
}

/// One scripted response.
#[derive(Clone, Debug)]
pub struct Reply {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Body,
}

impl Reply {
    /// A JSON body with its status.
    pub fn json(status: u16, value: &Value) -> Self {
        Reply {
            status,
            headers: vec![("Content-Type".into(), "application/json".into())],
            body: Body::Fixed(serde_json::to_vec(value).expect("JSON encodes")),
        }
    }

    pub fn raw(status: u16, content_type: &str, body: &[u8]) -> Self {
        Reply {
            status,
            headers: vec![("Content-Type".into(), content_type.into())],
            body: Body::Fixed(body.to_vec()),
        }
    }

    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn with_body(mut self, body: Body) -> Self {
        self.body = body;
        self
    }

    /// A Messages-wire answer with `text`, as Anthropic sends it.
    pub fn messages(text: &str) -> Self {
        Reply::json(200, &messages_body(text)).header("request-id", "req_fixture_1")
    }
}

/// The JSON of a Messages-wire answer.
pub fn messages_body(text: &str) -> Value {
    serde_json::json!({
        "id": "msg_fixture_1",
        "type": "message",
        "role": "assistant",
        "model": "fixture-reported-model",
        "content": [{ "type": "text", "text": text }],
        "stop_reason": "end_turn",
        "stop_sequence": null,
        "usage": { "input_tokens": 11, "output_tokens": 7 }
    })
}

/// A Messages-wire answer whose text is short and whose `metadata` field fills
/// the body to `total` bytes. The body is valid JSON only when it is whole.
pub fn oversized(total: usize, declared: bool) -> Body {
    let prefix = br#"{"id":"msg_big","type":"message","role":"assistant","model":"m","content":[{"type":"text","text":"ok"}],"stop_reason":"end_turn","usage":{"input_tokens":1,"output_tokens":1},"metadata":""#.to_vec();
    let suffix = br#""}"#.to_vec();
    Body::Streamed {
        total,
        prefix,
        suffix,
        declared,
    }
}

/// What a request carried.
#[derive(Clone, Debug)]
pub struct Seen {
    pub method: String,
    pub path: String,
    /// Lower-case names.
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Seen {
    pub fn header(&self, name: &str) -> Option<&str> {
        let name = name.to_ascii_lowercase();
        self.headers
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| value.as_str())
    }

    pub fn json(&self) -> Value {
        serde_json::from_slice(&self.body).expect("the request body is JSON")
    }
}

/// How one exchange ended.
#[derive(Clone, Debug)]
pub struct Exchange {
    pub seen: Seen,
    /// The bytes of the body that the server wrote.
    pub body_bytes_sent: usize,
    /// The client closed the connection before the server wrote the whole
    /// body, or before a stalled reply ended.
    pub client_closed_early: bool,
}

#[derive(Default)]
struct State {
    script: Vec<Reply>,
    next: usize,
    done: Vec<Exchange>,
    started: usize,
}

pub struct Server {
    address: SocketAddr,
    state: Arc<(Mutex<State>, Condvar)>,
}

impl Server {
    /// Listen on a loopback port and play `script`, one reply for each request.
    /// A request past the script gets a 500 that says so.
    pub fn start(script: Vec<Reply>) -> Server {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind a loopback port");
        let address = listener.local_addr().expect("a bound address");
        let state = Arc::new((
            Mutex::new(State {
                script,
                ..State::default()
            }),
            Condvar::new(),
        ));
        let shared = Arc::clone(&state);
        thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(stream) = stream else { break };
                let shared = Arc::clone(&shared);
                thread::spawn(move || serve(stream, &shared));
            }
        });
        Server { address, state }
    }

    /// The base URL of the endpoint.
    pub fn url(&self) -> String {
        format!("http://{}", self.address)
    }

    /// Wait until `count` exchanges have ended, and return them in the order
    /// they ended. The wait is for the server side to finish writing and to
    /// see how the client ended the connection.
    pub fn exchanges(&self, count: usize) -> Vec<Exchange> {
        let (lock, changed) = &*self.state;
        let deadline = Instant::now() + Duration::from_secs(20);
        let mut state = lock.lock().unwrap();
        while state.done.len() < count {
            let left = deadline.saturating_duration_since(Instant::now());
            assert!(
                !left.is_zero(),
                "{count} exchanges expected, {} ended",
                state.done.len()
            );
            state = changed.wait_timeout(state, left).unwrap().0;
        }
        state.done.clone()
    }

    /// How many requests arrived, whether or not their exchange ended.
    pub fn requests_started(&self) -> usize {
        self.state.0.lock().unwrap().started
    }
}

fn serve(mut stream: TcpStream, shared: &(Mutex<State>, Condvar)) {
    let _ = stream.set_nodelay(true);
    let Some(seen) = read_request(&mut stream) else {
        return;
    };
    let reply = {
        let mut state = shared.0.lock().unwrap();
        state.started += 1;
        let reply = state.script.get(state.next).cloned();
        state.next += 1;
        reply
    };
    let reply = reply.unwrap_or_else(|| Reply::raw(500, "text/plain", b"the script has no reply"));
    let (sent, closed) = play(&mut stream, &reply);
    let (lock, changed) = shared;
    lock.lock().unwrap().done.push(Exchange {
        seen,
        body_bytes_sent: sent,
        client_closed_early: closed,
    });
    changed.notify_all();
}

/// Read one request: the head up to the blank line, then `Content-Length`
/// bytes of body.
fn read_request(stream: &mut TcpStream) -> Option<Seen> {
    let mut buffer = Vec::new();
    let mut byte = [0u8; 1];
    while !buffer.ends_with(b"\r\n\r\n") {
        match stream.read(&mut byte) {
            Ok(1) => buffer.push(byte[0]),
            _ => return None,
        }
    }
    let head = String::from_utf8_lossy(&buffer).into_owned();
    let mut lines = head.split("\r\n");
    let mut first = lines.next()?.split(' ');
    let method = first.next()?.to_owned();
    let path = first.next()?.to_owned();
    let headers: Vec<(String, String)> = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(name, value)| (name.trim().to_ascii_lowercase(), value.trim().to_owned()))
        .collect();
    let length: usize = headers
        .iter()
        .find(|(name, _)| name == "content-length")
        .and_then(|(_, value)| value.parse().ok())
        .unwrap_or(0);
    let mut body = vec![0u8; length];
    stream.read_exact(&mut body).ok()?;
    Some(Seen {
        method,
        path,
        headers,
        body,
    })
}

fn status_text(status: u16) -> &'static str {
    match status {
        200 => "OK",
        301 => "Moved Permanently",
        302 => "Found",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        529 => "Overloaded",
        _ => "Status",
    }
}

fn head(reply: &Reply, length: Option<usize>) -> Vec<u8> {
    let mut text = format!(
        "HTTP/1.1 {} {}\r\n",
        reply.status,
        status_text(reply.status)
    );
    for (name, value) in &reply.headers {
        text.push_str(&format!("{name}: {value}\r\n"));
    }
    match length {
        Some(length) => text.push_str(&format!("Content-Length: {length}\r\n")),
        None => text.push_str("Transfer-Encoding: chunked\r\n"),
    }
    text.push_str("Connection: close\r\n\r\n");
    text.into_bytes()
}

/// Write `reply`. The first number is the bytes of body written, and the
/// second says that the client closed the connection too early.
fn play(stream: &mut TcpStream, reply: &Reply) -> (usize, bool) {
    match &reply.body {
        Body::Fixed(bytes) => {
            let wrote = stream
                .write_all(&head(reply, Some(bytes.len())))
                .and_then(|()| stream.write_all(bytes));
            (bytes.len(), wrote.is_err())
        }
        Body::Short { declared, sent } => {
            let wrote = stream
                .write_all(&head(reply, Some(*declared)))
                .and_then(|()| stream.write_all(sent));
            (sent.len(), wrote.is_err())
        }
        Body::Stall { hold } => {
            if stream.write_all(&head(reply, Some(1))).is_err() {
                return (0, true);
            }
            // A client that closes ends the wait with an end of stream or an
            // error. One that holds the connection ends it at `hold`.
            let _ = stream.set_read_timeout(Some(*hold));
            let mut byte = [0u8; 1];
            let closed = match stream.read(&mut byte) {
                Ok(0) => true,
                Ok(_) => false,
                Err(error) => !matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut),
            };
            (0, closed)
        }
        Body::Streamed {
            prefix,
            suffix,
            total,
            declared,
        } => stream_body(stream, reply, prefix, suffix, *total, *declared),
    }
}

fn stream_body(
    stream: &mut TcpStream,
    reply: &Reply,
    prefix: &[u8],
    suffix: &[u8],
    total: usize,
    declared: bool,
) -> (usize, bool) {
    let length = declared.then_some(total);
    if stream.write_all(&head(reply, length)).is_err() {
        return (0, true);
    }
    let mut body = Vec::with_capacity(total);
    body.extend_from_slice(prefix);
    body.resize(total - suffix.len(), b'x');
    body.extend_from_slice(suffix);
    let mut sent = 0;
    let mut paused = false;
    for piece in body.chunks(CHUNK) {
        // Past 8 MiB the kernel may still hold the rest in its buffers when
        // the client closes. A pause there lets the close reach the server, so
        // that the next write fails.
        if !paused && sent >= 8 * 1024 * 1024 {
            paused = true;
            thread::sleep(Duration::from_millis(100));
        }
        let wrote = if declared {
            stream.write_all(piece)
        } else {
            stream
                .write_all(format!("{:x}\r\n", piece.len()).as_bytes())
                .and_then(|()| stream.write_all(piece))
                .and_then(|()| stream.write_all(b"\r\n"))
        };
        if wrote.is_err() {
            return (sent, true);
        }
        sent += piece.len();
    }
    if !declared && stream.write_all(b"0\r\n\r\n").is_err() {
        return (sent, true);
    }
    (sent, false)
}
