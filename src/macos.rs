use dirs_next::{cache_dir, config_dir, data_dir, data_local_dir, home_dir};
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn application_dir() -> Option<PathBuf> {
    Some(PathBuf::from("/Applications"))
}

pub fn application_shared_dir() -> Option<PathBuf> {
    Some(PathBuf::from("/Library/Frameworks"))
}

pub fn user_application_dir() -> Option<PathBuf> {
    home_dir().map(|path| path.join("Applications"))
}

// ------------------------------------------------------------------------------------------------

pub fn app_container_dir_for(app: &str) -> Option<PathBuf> {
    application_dir().map(|path| path.join(&format!("{}.app", app)))
}

pub fn app_container_executable_dir_for(app: &str) -> Option<PathBuf> {
    app_container_dir_for(app).map(|a| a.join("Contents/MacOS"))
}

pub fn user_app_container_dir_for(app: &str) -> Option<PathBuf> {
    user_application_dir().map(|path| path.join(&format!("{}.app", app)))
}

pub fn user_app_container_executable_dir_for(app: &str) -> Option<PathBuf> {
    user_app_container_dir_for(app).map(|a| a.join("Contents/MacOS"))
}

// ------------------------------------------------------------------------------------------------

pub fn cache_dir_for(app: &str) -> Option<PathBuf> {
    cache_dir().map(|path| path.join(app))
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
    home_dir().map(|h| h.join("Library/Favorites"))
}

pub fn favorites_dir_for(app: &str) -> Option<PathBuf> {
    favorites_dir().map(|path| path.join(app))
}

pub fn log_dir() -> Option<PathBuf> {
    home_dir().map(|h| h.join("Library/Logs"))
}

pub fn log_dir_for(app: &str) -> Option<PathBuf> {
    log_dir().map(|path| path.join(&app))
}

pub fn preference_dir() -> Option<PathBuf> {
    home_dir().map(|h| h.join("Library/Preferences"))
}

pub fn preference_dir_for(app: &str) -> Option<PathBuf> {
    preference_dir().map(|path| path.join(app))
}

pub fn template_dir() -> Option<PathBuf> {
    None
}

pub fn template_dir_for(app: &str) -> Option<PathBuf> {
    data_dir_for(app).map(|d| d.join("Templates"))
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use dirs_next::home_dir;
    use std::path::PathBuf;

    fn test_user_dir(dir: PathBuf, suffix: &str) {
        assert_eq!(
            dir.to_string_lossy().to_string(),
            format!(
                "{}/{}",
                home_dir().unwrap().to_string_lossy().to_string(),
                suffix
            )
        )
    }

    fn test_dir(dir: PathBuf, path: &str) {
        assert_eq!(dir.to_string_lossy().to_string(), path.to_string())
    }

    // --------------------------------------------------------------------------------------------

    #[test]
    fn test_application_dir() {
        test_dir(crate::application_dir().unwrap(), "/Applications");
    }

    #[test]
    fn test_application_shared_dir() {
        test_dir(
            crate::application_shared_dir().unwrap(),
            "/Library/Frameworks",
        );
    }

    #[test]
    fn test_user_application_dir() {
        test_user_dir(crate::user_application_dir().unwrap(), "Applications");
    }

    // --------------------------------------------------------------------------------------------

    #[test]
    fn test_app_container_dir_for() {
        test_dir(
            crate::app_container_dir_for("Chrome").unwrap(),
            "/Applications/Chrome.app",
        );
    }

    #[test]
    fn test_app_container_executable_dir_for() {
        test_dir(
            crate::app_container_executable_dir_for("Chrome").unwrap(),
            "/Applications/Chrome.app/Contents/MacOS",
        );
    }

    #[test]
    fn test_user_app_container_dir_for() {
        test_user_dir(
            crate::user_app_container_dir_for("Chrome").unwrap(),
            "Applications/Chrome.app",
        );
    }

    #[test]
    fn test_user_app_container_executable_dir_for() {
        test_user_dir(
            crate::user_app_container_executable_dir_for("Chrome").unwrap(),
            "Applications/Chrome.app/Contents/MacOS",
        );
    }

    // --------------------------------------------------------------------------------------------

    #[test]
    fn test_cache_dir() {
        test_user_dir(crate::cache_dir().unwrap(), "Library/Caches");
    }

    #[test]
    fn test_cache_dir_for() {
        test_user_dir(
            crate::cache_dir_for("Chrome").unwrap(),
            "Library/Caches/Chrome",
        );
    }

    #[test]
    fn test_config_dir() {
        test_user_dir(crate::config_dir().unwrap(), "Library/Application Support");
    }

    #[test]
    fn test_config_dir_for() {
        test_user_dir(
            crate::config_dir_for("Chrome").unwrap(),
            "Library/Application Support/Chrome",
        );
    }

    #[test]
    fn test_data_dir() {
        test_user_dir(crate::data_dir().unwrap(), "Library/Application Support");
    }

    #[test]
    fn test_data_dir_for() {
        test_user_dir(
            crate::data_dir_for("Chrome").unwrap(),
            "Library/Application Support/Chrome",
        );
    }

    #[test]
    fn test_data_local_dir() {
        test_user_dir(
            crate::data_local_dir().unwrap(),
            "Library/Application Support",
        );
    }

    #[test]
    fn test_data_local_dir_for() {
        test_user_dir(
            crate::data_local_dir_for("Chrome").unwrap(),
            "Library/Application Support/Chrome",
        );
    }

    #[test]
    fn test_favorites_dir() {
        test_user_dir(crate::favorites_dir().unwrap(), "Library/Favorites");
    }

    #[test]
    fn test_favorites_dir_for() {
        test_user_dir(
            crate::favorites_dir_for("Chrome").unwrap(),
            "Library/Favorites/Chrome",
        );
    }

    #[test]
    fn test_log_dir() {
        test_user_dir(crate::log_dir().unwrap(), "Library/Logs");
    }

    #[test]
    fn test_log_dir_for() {
        test_user_dir(crate::log_dir_for("Chrome").unwrap(), "Library/Logs/Chrome");
    }

    #[test]
    fn test_preference_dir() {
        test_user_dir(crate::preference_dir().unwrap(), "Library/Preferences");
    }

    #[test]
    fn test_preference_dir_for() {
        test_user_dir(
            crate::preference_dir_for("Chrome").unwrap(),
            "Library/Preferences/Chrome",
        );
    }

    #[test]
    fn test_template_for() {
        assert_eq!(crate::template_dir(), None,);
    }

    #[test]
    fn test_template_dir_for() {
        test_user_dir(
            crate::template_dir_for("Chrome").unwrap(),
            "Library/Application Support/Chrome/Templates",
        );
    }
}
