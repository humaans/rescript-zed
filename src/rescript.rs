use std::{borrow::Cow, env, fs};
use zed_extension_api::serde_json::Value;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, Result};

const SERVER_PATH: &str = "node_modules/@rescript/language-server/out/cli.js";
const PACKAGE_NAME: &str = "@rescript/language-server";

struct ReScriptExtension {
    did_find_server: bool,
}

#[derive(Debug, Default)]
struct Settings {
    version: Option<String>,
}

fn parse_settings(settings_value: Value) -> Settings {
    if let Some(obj) = settings_value.as_object() {
        return Settings {
            version: obj
                .get("version")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        };
    }

    Settings::default()
}

impl ReScriptExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).is_ok_and(|stat| stat.is_file())
    }

    fn get_lsp_settings_for_worktree(
        &mut self,
        server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Settings> {
        let settings = zed::settings::LspSettings::for_worktree(server_id.as_ref(), worktree);
        match settings {
            Err(_) => Ok(Settings::default()),
            Ok(LspSettings { settings: None, .. }) => Ok(Settings::default()),
            Ok(LspSettings {
                settings: Some(settings_value),
                ..
            }) => Ok(parse_settings(settings_value)),
        }
    }

    fn server_script_path(
        &mut self,
        server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Cow<'static, str>> {
        let server_exists = self.server_exists();

        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.into());
        }

        zed::set_language_server_installation_status(
            server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let settings = self.get_lsp_settings_for_worktree(server_id, worktree)?;

        let version = if let Some(user_version) = settings.version {
            user_version
        } else {
            zed::npm_package_latest_version(PACKAGE_NAME)?
        };

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);

            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{}' did not contain expected path '{}'",
                            PACKAGE_NAME, SERVER_PATH
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
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(server_id, worktree)?;

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
