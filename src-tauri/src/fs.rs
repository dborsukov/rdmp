use log::error;
use std::process;
use std::{
    fs::DirBuilder,
    fs::File,
    path::{Path, PathBuf},
};

pub fn validate() {
    let app_dir = get_app_base_dir();

    DirBuilder::new()
        .recursive(true)
        .create(&app_dir)
        .unwrap_or_else(|err| {
            error!("Failed to create app dirs: {err}");
            process::exit(1);
        });

    let settings_file = app_dir.join("settings.json");
    if !settings_file.exists() {
        File::create(&settings_file).unwrap_or_else(|err| {
            error!("Failed to create settings.json: {err}");
            process::exit(1);
        });
    };
}

pub fn get_app_base_dir() -> PathBuf {
    let user_home_dir = tauri::api::path::home_dir().map_or_else(
        || {
            error!("Failed to locate user home dir");
            process::exit(1);
        },
        |path| path,
    );
    Path::new(&user_home_dir).join(".rdmp")
}
