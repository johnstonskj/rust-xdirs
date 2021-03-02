use dirs_next::{cache_dir, config_dir, data_dir, data_local_dir};
use dirs_sys_next::known_folder;
use std::path::PathBuf;
use winapi::um::knownfolders;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

const D_CACHE: &str = "Cache";

pub fn application_dir() -> Option<PathBuf> {
    known_folder(&knownfolders::FOLDERID_ProgramFiles)
}

pub fn application_shared_dir() -> Option<PathBuf> {
    known_folder(&knownfolders::FOLDERID_ProgramFilesCommon)
}

pub fn user_application_dir() -> Option<PathBuf> {
    known_folder(&knownfolders::FOLDERID_UserProgramFiles)
}

// ------------------------------------------------------------------------------------------------

pub fn app_container_dir_for(_: &str) -> Option<PathBuf> {
    None
}

pub fn app_container_executable_dir_for(app: &str) -> Option<PathBuf> {
    app_container_dir_for(app)
}

pub fn user_app_container_dir_for(_: &str) -> Option<PathBuf> {
    None
}

pub fn user_app_container_executable_dir_for(app: &str) -> Option<PathBuf> {
    user_app_container_dir_for(app)
}

// ------------------------------------------------------------------------------------------------

pub fn cache_dir_for(app: &str) -> Option<PathBuf> {
    cache_dir().map(|path| path.join(app).join(D_CACHE))
}

pub fn config_dir_for(app: &str) -> Option<PathBuf> {
    config_dir().map(|path| path.join(app))
}

pub fn data_dir_for(app: &str) -> Option<PathBuf> {
    data_dir().map(|path| path.join(app))
}

pub fn data_local_dir_for(app: &str) -> Option<PathBuf> {
    data_local_dir().map(|path| path.join(app))
}

pub fn favorites_dir() -> Option<PathBuf> {
    known_folder(&knownfolders::FOLDERID_Favorites)
}

pub fn favorites_dir_for(app: &str) -> Option<PathBuf> {
    favorites_dir().map(|path| path.join(app))
}

pub fn log_dir() -> Option<PathBuf> {
    data_local_dir().map(|h| h.join("Logs"))
}

pub fn log_dir_for(app: &str) -> Option<PathBuf> {
    log_dir().map(|path| path.join(app))
}

pub fn preference_dir() -> Option<PathBuf> {
    config_dir()
}

pub fn preference_dir_for(app: &str) -> Option<PathBuf> {
    config_dir_for(app)
}

pub use dirs_next::template_dir;

pub fn template_dir_for(app: &str) -> Option<PathBuf> {
    template_dir().map(|d| d.join(app))
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use dirs_next::home_dir;
    use std::path::PathBuf;

    const SYSTEM_DRIVE: &str = env!("SystemDrive");

    fn test_user_dir(dir: PathBuf, suffix: &str) {
        assert_eq!(
            dir.to_string_lossy().to_string(),
            format!(
                "{}\\{}",
                home_dir().unwrap().to_string_lossy().to_string(),
                suffix
            )
        )
    }

    fn test_dir(dir: PathBuf, path: &str) {
        assert_eq!(
            dir.to_string_lossy().to_string(),
            format!("{}\\{}", SYSTEM_DRIVE, path.to_string())
        )
    }

    fn test_dir_is_none(dir: Option<PathBuf>) {
        assert!(dir.is_none())
    }

    // --------------------------------------------------------------------------------------------

    #[test]
    fn test_application_dir() {
        test_dir(crate::application_dir().unwrap(), "Program Files");
    }

    #[test]
    fn test_application_shared_dir() {
        test_dir(
            crate::application_shared_dir().unwrap(),
            "Program Files\\Common Files",
        );
    }

    #[test]
    fn test_user_application_dir() {
        test_dir_is_none(crate::user_application_dir());
    }

    // --------------------------------------------------------------------------------------------

    #[test]
    fn test_app_container_dir_for() {
        test_dir_is_none(crate::app_container_dir_for("Chrome"));
    }

    #[test]
    fn test_app_container_executable_dir_for() {
        test_dir_is_none(crate::app_container_executable_dir_for("Chrome"));
    }

    #[test]
    fn test_user_app_container_dir_for() {
        test_dir_is_none(crate::user_app_container_dir_for("Chrome"));
    }

    #[test]
    fn test_user_app_container_executable_dir_for() {
        test_dir_is_none(crate::user_app_container_executable_dir_for("Chrome"));
    }

    // --------------------------------------------------------------------------------------------

    #[test]
    fn test_cache_dir() {
        test_user_dir(crate::cache_dir().unwrap(), "AppData\\Local");
    }

    #[test]
    fn test_cache_dir_for() {
        test_user_dir(
            crate::cache_dir_for("Chrome").unwrap(),
            "AppData\\Local\\Chrome\\Cache",
        );
    }

    #[test]
    fn test_config_dir() {
        test_user_dir(crate::config_dir().unwrap(), "AppData\\Roaming");
    }

    #[test]
    fn test_config_dir_for() {
        test_user_dir(
            crate::config_dir_for("Chrome").unwrap(),
            "AppData\\Roaming\\Chrome",
        );
    }

    #[test]
    fn test_data_dir() {
        test_user_dir(crate::data_dir().unwrap(), "AppData\\Roaming");
    }

    #[test]
    fn test_data_dir_for() {
        test_user_dir(
            crate::data_dir_for("Chrome").unwrap(),
            "AppData\\Roaming\\Chrome",
        );
    }

    #[test]
    fn test_data_local_dir() {
        test_user_dir(crate::data_local_dir().unwrap(), "AppData\\Local");
    }

    #[test]
    fn test_data_local_dir_for() {
        test_user_dir(
            crate::data_local_dir_for("Chrome").unwrap(),
            "AppData\\Local\\Chrome",
        );
    }

    #[test]
    fn test_favorites_dir() {
        test_user_dir(crate::favorites_dir().unwrap(), "Favorites");
    }

    #[test]
    fn test_favorites_dir_for() {
        test_user_dir(
            crate::favorites_dir_for("Chrome").unwrap(),
            "Favorites\\Chrome",
        );
    }

    #[test]
    fn test_log_dir() {
        test_user_dir(crate::log_dir().unwrap(), "AppData\\Local\\Logs");
    }

    #[test]
    fn test_log_dir_for() {
        test_user_dir(
            crate::log_dir_for("Chrome").unwrap(),
            "AppData\\Local\\Logs\\Chrome",
        );
    }

    #[test]
    fn test_preference_dir() {
        test_user_dir(crate::preference_dir().unwrap(), "AppData\\Roaming");
    }

    #[test]
    fn test_preference_dir_for() {
        test_user_dir(
            crate::preference_dir_for("Chrome").unwrap(),
            "AppData\\Roaming\\Chrome",
        );
    }

    #[test]
    fn test_template_dir() {
        test_user_dir(
            crate::template_dir().unwrap(),
            "AppData\\Roaming\\Microsoft\\Windows\\Templates",
        );
    }

    #[test]
    fn test_template_dir_for() {
        test_user_dir(
            crate::template_dir_for("Chrome").unwrap(),
            "AppData\\Roaming\\Microsoft\\Windows\\Templates\\Chrome",
        );
    }
}
