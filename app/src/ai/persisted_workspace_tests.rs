use std::path::Path;

use lsp::supported_servers::LSPServerType;
use warpui::App;

use super::{EnablementState, LSPEnablementResultForFile, PersistedWorkspace};

#[test]
fn removing_lsp_registrations_preserves_workspace_metadata_and_other_folders() {
    App::test((), |mut app| async move {
        let model = app.add_model(PersistedWorkspace::new_for_test);
        app.update(|ctx| {
            model.update(ctx, |workspace, ctx| {
                let path = Path::new("/projects/remove");
                let other = Path::new("/projects/keep");
                workspace.enable_lsp_server_for_path(path, LSPServerType::Pyright, ctx);
                workspace.disable_lsp_server_for_path(path, LSPServerType::RustAnalyzer, ctx);
                workspace.enable_lsp_server_for_path(other, LSPServerType::Pyright, ctx);
                let metadata = workspace.workspaces.get(path).unwrap().metadata.clone();

                workspace.remove_lsp_servers_for_path(path, ctx);

                assert_eq!(workspace.all_lsp_servers(path, true).unwrap().count(), 0);
                assert!(matches!(
                    workspace.has_enabled_lsp_server_for_file_path(&path.join("main.py")),
                    LSPEnablementResultForFile::LSPNotEnabled { .. }
                ));
                let preserved = &workspace.workspaces.get(path).unwrap().metadata;
                assert_eq!(preserved.path, metadata.path);
                assert_eq!(preserved.modified_ts, metadata.modified_ts);
                assert_eq!(preserved.navigated_ts, metadata.navigated_ts);
                assert_eq!(preserved.queried_ts, metadata.queried_ts);
                assert_eq!(
                    workspace
                        .all_lsp_servers(other, false)
                        .unwrap()
                        .collect::<Vec<_>>(),
                    vec![(LSPServerType::Pyright, EnablementState::Yes)]
                );

                workspace.remove_lsp_servers_for_path(path, ctx);
                workspace.enable_lsp_server_for_path(path, LSPServerType::Pyright, ctx);
                assert!(matches!(
                    workspace.has_enabled_lsp_server_for_file_path(&path.join("main.py")),
                    LSPEnablementResultForFile::Enabled
                ));
            });
        });
    });
}

#[test]
fn removing_unregistered_child_does_not_remove_parent_language_servers() {
    App::test((), |mut app| async move {
        let model = app.add_model(PersistedWorkspace::new_for_test);
        app.update(|ctx| {
            model.update(ctx, |workspace, ctx| {
                let parent = Path::new("/projects/parent");
                workspace.enable_lsp_server_for_path(parent, LSPServerType::Pyright, ctx);

                workspace.remove_lsp_servers_for_path(&parent.join("child"), ctx);

                assert_eq!(
                    workspace
                        .enabled_lsp_servers(parent)
                        .unwrap()
                        .collect::<Vec<_>>(),
                    vec![LSPServerType::Pyright]
                );
            });
        });
    });
}

#[cfg(feature = "local_fs")]
#[test]
fn removing_folder_prevents_an_earlier_installation_from_reenabling_it() {
    App::test((), |mut app| async move {
        let model = app.add_model(PersistedWorkspace::new_for_test);
        app.update(|ctx| {
            model.update(ctx, |workspace, ctx| {
                let path = Path::new("/projects/remove");
                let other = Path::new("/projects/keep");
                workspace.disable_lsp_server_for_path(path, LSPServerType::Pyright, ctx);
                let before_removal = workspace
                    .lsp_removal_generations
                    .get(path)
                    .copied()
                    .unwrap_or_default();

                workspace.remove_lsp_servers_for_path(path, ctx);

                assert!(!workspace.enable_lsp_after_install(
                    path,
                    LSPServerType::Pyright,
                    before_removal,
                    ctx
                ));
                assert_eq!(workspace.all_lsp_servers(path, true).unwrap().count(), 0);
                assert!(workspace.enable_lsp_after_install(other, LSPServerType::Pyright, 0, ctx));

                let after_removal = workspace.lsp_removal_generations[path];
                assert!(workspace.enable_lsp_after_install(
                    path,
                    LSPServerType::Pyright,
                    after_removal,
                    ctx
                ));
                assert!(matches!(
                    workspace.has_enabled_lsp_server_for_file_path(&path.join("main.py")),
                    LSPEnablementResultForFile::Enabled
                ));
            });
        });
    });
}
