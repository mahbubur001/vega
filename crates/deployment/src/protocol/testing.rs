//! Protocol contract shared by unit tests (local) and integration tests (Docker servers).

use std::sync::Mutex;

use super::*;
use crate::error::ErrorCode;
use crate::mapper::join_remote;

/// Exercises every `RemoteClient` operation under `base` (which must exist and be writable).
pub async fn contract(client: &mut dyn RemoteClient, base: &str) {
    let caps = client.capabilities();
    static RUN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let run = RUN.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let root = join_remote(base, &format!("contract-{}-{run}", std::process::id()));
    if client.stat(&root).await.unwrap().is_some() {
        delete_tree(client, &root).await.unwrap();
    }

    // mkdirp creates parents and tolerates existing dirs.
    let nested = join_remote(&root, "a/b/c");
    client.mkdirp(&nested).await.unwrap();
    client.mkdirp(&nested).await.unwrap();
    assert!(client.stat(&nested).await.unwrap().unwrap().is_dir());

    // write/read round trip with binary content.
    let data: Vec<u8> = (0..=255u8).cycle().take(70_000).collect();
    let file = join_remote(&root, "a/bin.dat");
    client.write_file(&file, &data).await.unwrap();
    assert_eq!(client.read_file(&file).await.unwrap(), data);
    let st = client.stat(&file).await.unwrap().unwrap();
    assert_eq!(st.kind, EntryType::File);
    assert_eq!(st.size, data.len() as u64);

    // upload/download with progress.
    let tmp = tempfile::tempdir().unwrap();
    let local = tmp.path().join("up.txt");
    std::fs::write(&local, "hello ünïcode\n").unwrap();
    let seen = Mutex::new(0u64);
    let progress = |done: u64, _total: u64| *seen.lock().unwrap() = done;
    let up = join_remote(&root, "a/b/up.txt");
    client.upload(&local, &up, &progress).await.unwrap();
    let back = tmp.path().join("down.txt");
    client.download(&up, &back, &no_progress).await.unwrap();
    assert_eq!(
        std::fs::read(&back).unwrap(),
        std::fs::read(&local).unwrap()
    );

    // list returns names relative to the dir.
    let mut names: Vec<String> = client
        .list(&join_remote(&root, "a"))
        .await
        .unwrap()
        .into_iter()
        .map(|e| e.name)
        .collect();
    names.sort();
    assert_eq!(names, ["b", "bin.dat"]);

    // set_mtime / chmod when supported.
    if caps.set_mtime {
        let t = 1_600_000_000_000i64;
        client.set_mtime(&up, t).await.unwrap();
        let m = client.stat(&up).await.unwrap().unwrap().mtime;
        assert!((m - t).abs() < 2000, "mtime {m} vs {t}");
    }
    if caps.chmod {
        client.chmod(&up, 0o640).await.unwrap();
        if let Some(mode) = client.stat(&up).await.unwrap().unwrap().mode {
            assert_eq!(mode & 0o777, 0o640);
        }
    }

    // rename, missing paths.
    let moved = join_remote(&root, "moved.txt");
    client.rename(&up, &moved).await.unwrap();
    assert!(client.stat(&up).await.unwrap().is_none());
    assert!(client.stat(&moved).await.unwrap().is_some());
    let err = client
        .read_file(&join_remote(&root, "missing.txt"))
        .await
        .unwrap_err();
    assert_eq!(err.code, ErrorCode::NotFound, "{err}");

    // delete.
    client.delete_file(&moved).await.unwrap();
    assert!(client.stat(&moved).await.unwrap().is_none());
    client.delete_dir(&root, true).await.unwrap();
    assert!(client.stat(&root).await.unwrap().is_none());
}
