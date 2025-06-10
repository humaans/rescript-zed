use std::{borrow::Cow, env, fs};
use zed_extension_api::{self as zed, Result};

const SERVER_PATH: &str = "node_modules/.bin/rescript-language-server";
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
        self.did_find_server = true;
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

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(server_path.as_ref())
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(ReScriptExtension);
