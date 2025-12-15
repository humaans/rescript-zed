use std::{borrow::Cow, env, fs};
use zed_extension_api::{self as zed, Result};

const SERVER_PATH: &str = "node_modules/@rescript/language-server/out/cli.js";
const PACKAGE_NAME: &str = "@rescript/language-server";

struct ReScriptExtension {
    did_find_server: bool,
}

impl ReScriptExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(
        &mut self,
        server_id: &zed::LanguageServerId,
    ) -> Result<Cow<'static, str>> {
        let server_exists = self.server_exists();

        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.into());
        }

        zed::set_language_server_installation_status(
            &server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                &server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;

        Ok(SERVER_PATH.into())
    }
}

impl zed::Extension for ReScriptExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(server_id)?;

        let current_dir = env::current_dir()
            .map_err(|e| format!("failed to get current directory: {e}"))?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                current_dir
                    .join(server_path.as_ref())
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(Some(zed::serde_json::json!({
            "extensionConfiguration": {
                "inlayHints": {
                    "enable": true
                },
                "codeLens": true,
                "signatureHelp": {
                    "enabled": true
                }
            }
        })))
    }
}

zed::register_extension!(ReScriptExtension);
