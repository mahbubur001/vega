//! Engine behaviour end to end against "Local or mounted folder" servers (no Docker needed).

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use async_trait::async_trait;
use deployment::config::{AutoUpload, ConfigStore, Mapping, OverwriteCheck, Server, ServerType};
use deployment::credentials::Credentials;
use deployment::engine::Engine;
use deployment::prompt::*;
use deployment::sync::{CompareMode, SyncAction, SyncState};

fn isolate() {
    static HOME: OnceLock<tempfile::TempDir> = OnceLock::new();
    let home = HOME.get_or_init(|| tempfile::tempdir().unwrap());
    deployment::dirs::set_home(home.path().to_path_buf());
}

#[derive(Default)]
struct Scripted {
    conflict: Mutex<Vec<ConflictDecision>>,
    confirm: Mutex<bool>,
    log: Mutex<Vec<String>>,
}

#[async_trait]
impl Prompter for Scripted {
    async fn secret(&self, _: &SecretRequest) -> Option<SecretAnswer> {
        None
    }
    async fn host_key(&self, _: &HostKeyQuestion) -> HostKeyAnswer {
        HostKeyAnswer::Reject
    }
    async fn certificate(&self, _: &str, _: &str, _: &str) -> CertAnswer {
        CertAnswer::Reject
    }
    async fn keyboard_interactive(
        &self,
        _: &str,
        _: &str,
        _: &[(String, bool)],
    ) -> Option<Vec<String>> {
        None
    }
    async fn conflict(&self, q: &ConflictQuestion, allow_compare: bool) -> ConflictDecision {
        self.log.lock().unwrap().push(format!(
            "conflict {} compare={allow_compare}",
            q.remote_path
        ));
        let mut c = self.conflict.lock().unwrap();
        if c.is_empty() {
            ConflictDecision::Skip
        } else {
            c.remove(0)
        }
    }
    async fn confirm(&self, title: &str, _: &str, _: &str) -> bool {
        self.log.lock().unwrap().push(format!("confirm {title}"));
        *self.confirm.lock().unwrap()
    }
    async fn show_diff(&self, left: &Path, right: &Path, _: &str) {
        self.log
            .lock()
            .unwrap()
            .push(format!("diff {} {}", left.display(), right.display()));
    }
    fn notify(&self, _: Level, message: &str) {
        self.log.lock().unwrap().push(format!("notify {message}"));
    }
}

struct Fixture {
    _dir: tempfile::TempDir,
    project: PathBuf,
    remote: PathBuf,
    engine: Arc<Engine>,
    prompts: Arc<Scripted>,
}

fn slash(p: &Path) -> String {
    let s = p.to_string_lossy().replace('\\', "/");
    if s.starts_with('/') {
        s
    } else {
        format!("/{s}")
    }
}

fn fixture(configure: impl FnOnce(&mut ConfigStore, &Path)) -> Fixture {
    isolate();
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("project");
    let remote = dir.path().join("server");
    std::fs::create_dir_all(project.join("src")).unwrap();
    std::fs::create_dir_all(&remote).unwrap();
    let mut store = ConfigStore::load_with(&project, dir.path().join("servers.json"));
    let mut s = Server::new("local", "Local", ServerType::Local);
    s.root_path = slash(&remote);
    s.web_url = "https://example.com".into();
    s.mappings = vec![Mapping {
        local: "".into(),
        remote: "/www".into(),
        web: "/".into(),
    }];
    store.upsert_server(s, false).unwrap();
    store.set_default_server(Some("local")).unwrap();
    configure(&mut store, &remote);
    let prompts = Arc::new(Scripted::default());
    let engine = Engine::with_parts(
        &project,
        prompts.clone(),
        Arc::new(Credentials::memory()),
        store,
    );
    Fixture {
        _dir: dir,
        project,
        remote,
        engine,
        prompts,
    }
}

fn write(p: &Path, text: &str) {
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(p, text).unwrap();
}

fn bump_mtime(p: &Path, secs: i64) {
    let ft = filetime::FileTime::from_unix_time(1_700_000_000 + secs, 0);
    filetime::set_file_mtime(p, ft).unwrap();
}

#[tokio::test]
async fn upload_download_and_web_url() {
    let f = fixture(|_, _| {});
    let file = f.project.join("src/app.php");
    write(&file, "<?php echo 1;");
    let s = f
        .engine
        .upload(None, &[f.project.join("src")], true)
        .await
        .unwrap();
    assert_eq!(s.transferred, 1, "{s:?}");
    assert_eq!(
        std::fs::read_to_string(f.remote.join("www/src/app.php")).unwrap(),
        "<?php echo 1;"
    );
    assert_eq!(
        f.engine.web_url(None, &file).unwrap(),
        "https://example.com/src/app.php"
    );

    // Folder upload again: unchanged file is skipped (overwrite up-to-date files is off).
    let s = f
        .engine
        .upload(None, &[f.project.join("src")], true)
        .await
        .unwrap();
    assert_eq!((s.transferred, s.skipped), (0, 1));

    // Download brings back a server-side change.
    write(&f.remote.join("www/src/app.php"), "<?php echo 2;");
    *f.prompts.confirm.lock().unwrap() = true;
    let s = f
        .engine
        .download(None, std::slice::from_ref(&file))
        .await
        .unwrap();
    assert_eq!(s.transferred, 1);
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "<?php echo 2;");
    assert!(
        f.engine.is_own_write(&file),
        "auto-upload must not echo the download back"
    );
}

#[tokio::test]
async fn auto_upload_modes_and_conflicts() {
    let f = fixture(|store, _| {
        let mut o = store.options().clone();
        o.auto_upload = AutoUpload::OnExplicitSave;
        o.warn_on_newer_remote = OverwriteCheck::TimestampAndSize;
        store.set_options(o).unwrap();
    });
    let file = f.project.join("index.php");
    write(&file, "v1");
    f.engine.on_saved(&file, false).await; // auto-save: ignored in onExplicitSave mode
    assert!(!f.remote.join("www/index.php").exists());
    f.engine.on_saved(&file, true).await;
    assert_eq!(
        std::fs::read_to_string(f.remote.join("www/index.php")).unwrap(),
        "v1"
    );

    // Someone changes the server copy; the next save warns. Compare, then Skip.
    let remote_file = f.remote.join("www/index.php");
    write(&remote_file, "server edit");
    bump_mtime(&remote_file, 100);
    write(&file, "v2");
    f.prompts
        .conflict
        .lock()
        .unwrap()
        .extend([ConflictDecision::Compare, ConflictDecision::Skip]);
    f.engine.on_saved(&file, true).await;
    assert_eq!(
        std::fs::read_to_string(&remote_file).unwrap(),
        "server edit"
    );
    let log = f.prompts.log.lock().unwrap().clone();
    assert!(log.iter().any(|l| l.starts_with("diff ")), "{log:?}");
    assert!(
        log.iter().any(|l| l.ends_with("compare=false")),
        "asked again without Compare: {log:?}"
    );

    // Overwrite wins.
    f.prompts
        .conflict
        .lock()
        .unwrap()
        .push(ConflictDecision::Overwrite);
    f.engine.on_saved(&file, true).await;
    assert_eq!(std::fs::read_to_string(&remote_file).unwrap(), "v2");
}

#[tokio::test]
async fn temp_file_upload_permissions_and_groups() {
    let f = fixture(|store, remote| {
        let mut o = store.options().clone();
        o.use_temp_file = true;
        o.file_permissions = "640".into();
        store.set_options(o).unwrap();
        let mut b = Server::new("mirror", "Mirror", ServerType::Local);
        b.root_path = slash(&remote.join("mirror"));
        b.mappings = vec![Mapping {
            local: "".into(),
            remote: "/".into(),
            web: "/".into(),
        }];
        store.upsert_server(b, false).unwrap();
        let mut g = Server::new("both", "Both", ServerType::Group);
        g.members = vec!["local".into(), "mirror".into()];
        store.upsert_server(g, false).unwrap();
    });
    std::fs::create_dir_all(f.remote.join("mirror")).unwrap();
    let file = f.project.join("a.txt");
    write(&file, "hello");
    let s = f
        .engine
        .upload(Some("both"), std::slice::from_ref(&file), true)
        .await
        .unwrap();
    assert_eq!(s.transferred, 2, "{s:?}");
    assert!(f.remote.join("www/a.txt").exists());
    assert!(f.remote.join("mirror/a.txt").exists());
    assert!(
        !f.remote.join("www/.a.txt.iremote-upload").exists(),
        "temp file renamed into place"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(f.remote.join("www/a.txt"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o640);
    }
}

#[tokio::test]
async fn sync_compare_and_execute() {
    let f = fixture(|_, _| {});
    write(&f.project.join("only-local.txt"), "l");
    write(&f.remote.join("www/only-remote.txt"), "r");
    write(&f.project.join("both.txt"), "same");
    f.engine
        .upload(None, &[f.project.join("both.txt")], true)
        .await
        .unwrap();
    let (server, items) = f
        .engine
        .sync_compare(None, &[], CompareMode::Smart)
        .await
        .unwrap();
    let state = |name: &str| {
        items
            .iter()
            .find(|i| i.rel_path == name)
            .map(|i| (i.state, i.action))
    };
    assert_eq!(
        state("only-local.txt"),
        Some((SyncState::LocalOnly, SyncAction::Upload))
    );
    assert_eq!(
        state("only-remote.txt"),
        Some((SyncState::RemoteOnly, SyncAction::Download))
    );
    assert_eq!(state("both.txt"), Some((SyncState::Same, SyncAction::Skip)));
    let s = f.engine.sync_execute(&server, &items).await.unwrap();
    assert_eq!(s.transferred, 2, "{s:?}");
    assert!(f.remote.join("www/only-local.txt").exists());
    assert!(f.project.join("only-remote.txt").exists());
    let (_, again) = f
        .engine
        .sync_compare(None, &[], CompareMode::Content)
        .await
        .unwrap();
    assert!(
        again.iter().all(|i| i.state == SyncState::Same),
        "{again:#?}"
    );
}

#[tokio::test]
async fn remote_host_operations_and_editing() {
    let f = fixture(|_, _| {});
    write(&f.remote.join("www/dir/x.txt"), "x");
    let root = slash(&f.remote.join("www"));
    let items = f.engine.remote_list("local", &root).await.unwrap();
    assert_eq!(items[0].name, "dir");
    assert!(items[0].stat.is_dir());
    assert_eq!(
        items[0].local.as_deref(),
        Some(f.project.join("dir").as_path())
    );

    let x = format!("{root}/dir/x.txt");
    let edit = f.engine.remote_open("local", &x, true).await.unwrap();
    assert_eq!(std::fs::read_to_string(&edit).unwrap(), "x");
    assert!(f.engine.remote_edit(&edit).is_some());
    write(&edit, "edited remotely");
    f.engine.on_saved(&edit, true).await;
    assert_eq!(
        std::fs::read_to_string(f.remote.join("www/dir/x.txt")).unwrap(),
        "edited remotely"
    );

    f.engine
        .remote_rename("local", &x, &format!("{root}/dir/y.txt"))
        .await
        .unwrap();
    assert!(f.remote.join("www/dir/y.txt").exists());
    *f.prompts.confirm.lock().unwrap() = true;
    f.engine
        .remote_delete("local", &[format!("{root}/dir")])
        .await
        .unwrap();
    assert!(!f.remote.join("www/dir").exists());
}

#[tokio::test]
async fn remote_host_copy_chmod_and_exclude() {
    let f = fixture(|_, _| {});
    write(&f.remote.join("www/dir/a.txt"), "a");
    std::fs::create_dir_all(f.remote.join("www/dir/empty")).unwrap();
    let root = slash(&f.remote.join("www"));

    let summary = f
        .engine
        .remote_copy(
            "local",
            &[
                (format!("{root}/dir"), format!("{root}/dir copy")),
                (format!("{root}/dir/a.txt"), format!("{root}/b.txt")),
            ],
        )
        .await
        .unwrap();
    assert!(summary.failed.is_empty());
    assert_eq!(
        std::fs::read_to_string(f.remote.join("www/dir copy/a.txt")).unwrap(),
        "a"
    );
    assert!(f.remote.join("www/dir copy/empty").is_dir());
    assert_eq!(
        std::fs::read_to_string(f.remote.join("www/b.txt")).unwrap(),
        "a"
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        f.engine
            .remote_chmod("local", &format!("{root}/b.txt"), 0o600, false)
            .await
            .unwrap();
        let mode = std::fs::metadata(f.remote.join("www/b.txt"))
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, 0o600);
    }

    let dir = format!("{root}/dir");
    f.engine.exclude_remote("local", &dir, true).unwrap();
    let excluded = |items: Vec<deployment::engine::RemoteItem>| {
        items
            .into_iter()
            .find(|item| item.path == dir)
            .unwrap()
            .excluded
    };
    assert!(excluded(
        f.engine.remote_list("local", &root).await.unwrap()
    ));
    f.engine.exclude_remote("local", &dir, false).unwrap();
    assert!(!excluded(
        f.engine.remote_list("local", &root).await.unwrap()
    ));
}
