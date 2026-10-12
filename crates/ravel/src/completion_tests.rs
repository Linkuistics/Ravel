use std::fs;
use std::os::unix::fs::symlink;

use super::{publish_result, SessionChannel, MAX_RESULT_BYTES};

#[test]
fn complete_opaque_values_are_published_once_without_changing_the_first_result() {
    let parent = tempfile::tempdir().unwrap();
    for result in ["done", "", "first line\nsecond line\n", "λ and 🧶"] {
        let channel = SessionChannel::allocate(parent.path()).unwrap();
        assert!(!channel.channel().appeared());
        publish_result(channel.channel().path(), result).unwrap();
        assert!(channel.channel().appeared());
        assert_eq!(channel.read_result().unwrap(), Some(result.to_owned()));
        publish_result(channel.channel().path(), "replacement").unwrap();
        assert_eq!(channel.read_result().unwrap(), Some(result.to_owned()));
        let path = channel.channel().path().to_owned();
        drop(channel);
        assert!(!path.exists());
        assert!(!path.parent().unwrap().exists());
    }
    assert_eq!(fs::read_dir(parent.path()).unwrap().count(), 0);
}

#[test]
fn stale_context_cannot_end_a_later_session() {
    let parent = tempfile::tempdir().unwrap();
    let first = SessionChannel::allocate(parent.path()).unwrap();
    let stale = first.channel().path().to_owned();
    drop(first);
    let next = SessionChannel::allocate(parent.path()).unwrap();
    assert!(publish_result(&stale, "stale").is_err());
    assert!(!next.channel().appeared());
    assert_eq!(next.read_result().unwrap(), None);
}

#[test]
fn simultaneous_publications_never_mix_or_replace_result_bytes() {
    let parent = tempfile::tempdir().unwrap();
    let channel = SessionChannel::allocate(parent.path()).unwrap();
    let results: Vec<String> = (0..8)
        .map(|n| format!("{n}:{}", "λ".repeat(4096)))
        .collect();
    let barrier = std::sync::Barrier::new(results.len());
    std::thread::scope(|scope| {
        for result in &results {
            let barrier = &barrier;
            let path = channel.channel().path();
            scope.spawn(move || {
                barrier.wait();
                publish_result(path, result).unwrap();
            });
        }
    });
    let returned = channel.read_result().unwrap().unwrap();
    assert!(results.contains(&returned));
    publish_result(channel.channel().path(), "later").unwrap();
    assert_eq!(channel.read_result().unwrap().unwrap(), returned);
}

#[test]
fn nonregular_channels_never_follow_a_link_or_block_on_a_fifo() {
    let parent = tempfile::tempdir().unwrap();
    let outside = parent.path().join("outside");
    fs::write(&outside, "untouched").unwrap();
    let link = SessionChannel::allocate(parent.path()).unwrap();
    symlink(&outside, link.channel().path()).unwrap();
    assert!(publish_result(link.channel().path(), "changed").is_err());
    assert!(link.read_result().is_err());
    assert_eq!(fs::read_to_string(outside).unwrap(), "untouched");

    let fifo = SessionChannel::allocate(parent.path()).unwrap();
    let path =
        std::ffi::CString::new(fifo.channel().path().as_os_str().as_encoded_bytes()).unwrap();
    // SAFETY: valid NUL-terminated private test path; no pointers outlive the call.
    assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
    assert!(publish_result(fifo.channel().path(), "changed").is_err());
    assert!(fifo.read_result().is_err());

    let directory = SessionChannel::allocate(parent.path()).unwrap();
    fs::create_dir(directory.channel().path()).unwrap();
    assert!(publish_result(directory.channel().path(), "changed").is_err());
    assert!(directory.read_result().is_err());
}

#[test]
fn invalid_or_oversized_results_refuse_without_publishing() {
    let parent = tempfile::tempdir().unwrap();
    let channel = SessionChannel::allocate(parent.path()).unwrap();
    assert!(publish_result(channel.channel().path(), &"x".repeat(MAX_RESULT_BYTES + 1)).is_err());
    assert!(!channel.channel().appeared());
    fs::write(channel.channel().path(), [0xff]).unwrap();
    assert!(channel.read_result().is_err());
    fs::write(channel.channel().path(), vec![b'x'; MAX_RESULT_BYTES + 1]).unwrap();
    assert!(channel.read_result().is_err());
}

#[test]
fn reading_remains_in_the_original_directory_when_the_path_is_replaced() {
    let parent = tempfile::tempdir().unwrap();
    let channel = SessionChannel::allocate(parent.path()).unwrap();
    let original = channel.channel().path().parent().unwrap();
    let held = parent.path().join("held");
    fs::rename(original, &held).unwrap();
    let replacement = parent.path().join("replacement");
    fs::create_dir(&replacement).unwrap();
    fs::write(
        replacement.join(channel.channel().path().file_name().unwrap()),
        "wrong session",
    )
    .unwrap();
    symlink(&replacement, original).unwrap();
    assert!(!channel.channel().appeared());
    assert_eq!(channel.read_result().unwrap(), None);
    fs::write(
        held.join(channel.channel().path().file_name().unwrap()),
        "original session",
    )
    .unwrap();
    assert!(channel.channel().appeared());
    assert_eq!(
        channel.read_result().unwrap(),
        Some("original session".to_owned())
    );
    // Restore the owned directory before RAII cleanup; do not ask a pathname
    // destructor to distinguish an adversarially replaced directory.
    fs::remove_file(original).unwrap();
    fs::rename(held, original).unwrap();
}

#[test]
fn a_result_requires_the_corresponding_reaped_non_cancelled_launch() {
    use keyed_launch::{End, Ended, Group};
    use std::os::unix::process::ExitStatusExt as _;
    let parent = tempfile::tempdir().unwrap();
    let channel = SessionChannel::allocate(parent.path()).unwrap();
    publish_result(channel.channel().path(), "ready").unwrap();
    let mut ending = Ended {
        end: End::Exited,
        status: std::process::ExitStatus::from_raw(0),
        elapsed: std::time::Duration::ZERO,
        signalled: false,
        group: Group::Gone,
    };
    assert_eq!(channel.result_after(&ending).unwrap(), None);
    ending.signalled = true;
    assert_eq!(
        channel.result_after(&ending).unwrap(),
        Some("ready".to_owned())
    );
    ending.group = Group::Present { pgid: 123 };
    assert!(channel.result_after(&ending).is_err());
    ending.group = Group::Gone;
    ending.end = End::Interrupted {
        signal: libc::SIGTERM,
    };
    assert_eq!(
        channel.result_after(&ending).unwrap_err().kind(),
        std::io::ErrorKind::Interrupted
    );
}

#[test]
fn cleanup_reports_a_replaced_directory_without_deleting_its_contents() {
    let parent = tempfile::tempdir().unwrap();
    let channel = SessionChannel::allocate(parent.path()).unwrap();
    publish_result(channel.channel().path(), "owned").unwrap();
    let original = channel.channel().path().parent().unwrap().to_owned();
    let moved = parent.path().join("moved");
    fs::rename(&original, &moved).unwrap();
    fs::create_dir(&original).unwrap();
    fs::write(original.join("unrelated"), "preserve").unwrap();
    let closed = channel.close();
    assert!(
        closed.is_err(),
        "cleanup reported success after its owned directory moved"
    );
    assert_eq!(
        fs::read_to_string(original.join("unrelated")).unwrap(),
        "preserve"
    );
    assert_eq!(fs::read_dir(moved).unwrap().count(), 1);
}

#[test]
fn dropping_a_replaced_directory_leaves_unrelated_contents_untouched() {
    let parent = tempfile::tempdir().unwrap();
    let channel = SessionChannel::allocate(parent.path()).unwrap();
    let original = channel.channel().path().parent().unwrap().to_owned();
    fs::rename(&original, parent.path().join("moved")).unwrap();
    fs::create_dir(&original).unwrap();
    fs::write(original.join("unrelated"), "preserve").unwrap();
    drop(channel);
    assert_eq!(
        fs::read_to_string(original.join("unrelated")).unwrap(),
        "preserve"
    );
}
