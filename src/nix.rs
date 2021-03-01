use dirs_next::{cache_dir, config_dir, data_dir, data_local_dir};
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

const D_FAVORITES: &str = "favorites";
const D_LOGS: &str = "logs";
const D_TEMPLATES: &str = "templates";

pub fn application_dir() -> Option<PathBuf> {
    None
}

pub fn application_shared_dir() -> Option<PathBuf> {
    None
}

pub fn user_application_dir() -> Option<PathBuf> {
    application_dir()
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
    None
}

pub fn favorites_dir_for(app: &str) -> Option<PathBuf> {
    data_local_dir_for(app).map(|path| path.join(D_FAVORITES))
}

pub fn log_dir() -> Option<PathBuf> {
    None
}

pub fn log_dir_for(app: &str) -> Option<PathBuf> {
    data_local_dir_for(app).map(|path| path.join(D_LOGS))
}

pub fn preference_dir() -> Option<PathBuf> {
    config_dir()
}

pub fn preference_dir_for(app: &str) -> Option<PathBuf> {
    config_dir_for(app)
}

pub use dirs_next::template_dir;

pub fn template_dir_for(app: &str) -> Option<PathBuf> {
    config_dir_for(app).map(|d| d.join(D_TEMPLATES))
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

    fn test_dir_is_none(dir: Option<PathBuf>) {
        assert!(dir.is_none())
    }

    // --------------------------------------------------------------------------------------------

    #[test]
    fn test_application_dir() {
        test_dir_is_none(crate::application_dir());
    }

    #[test]
    fn test_application_shared_dir() {
        test_dir_is_none(crate::application_shared_dir());
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
        test_user_dir(crate::cache_dir().unwrap(), ".cache");
    }

    #[test]
    fn test_cache_dir_for() {
        test_user_dir(crate::cache_dir_for("Chrome").unwrap(), ".cache/Chrome");
    }

    #[test]
    fn test_config_dir() {
        test_user_dir(crate::config_dir().unwrap(), ".config");
    }

    #[test]
    fn test_config_dir_for() {
        test_user_dir(crate::config_dir_for("Chrome").unwrap(), ".config/Chrome");
    }

    #[test]
    fn test_data_dir() {
        test_user_dir(crate::data_dir().unwrap(), ".local/share");
    }

    #[test]
    fn test_data_dir_for() {
        test_user_dir(
            crate::data_dir_for("Chrome").unwrap(),
            ".local/share/Chrome",
        );
    }

    #[test]
    fn test_data_local_dir() {
        test_user_dir(crate::data_local_dir().unwrap(), ".local/share");
    }

    #[test]
    fn test_data_local_dir_for() {
        test_user_dir(
            crate::data_local_dir_for("Chrome").unwrap(),
            ".local/share/Chrome",
        );
    }

    #[test]
    fn test_favorites_dir() {
        test_dir_is_none(crate::favorites_dir());
    }

    #[test]
    fn test_favorites_dir_for() {
        test_user_dir(
            crate::favorites_dir_for("Chrome").unwrap(),
            ".local/share/Chrome/favorites",
        );
    }

    #[test]
    fn test_log_dir() {
        test_dir_is_none(crate::log_dir());
    }

    #[test]
    fn test_log_dir_for() {
        test_user_dir(
            crate::log_dir_for("Chrome").unwrap(),
            ".local/share/Chrome/logs",
        );
    }

    #[test]
    fn test_preference_dir() {
        test_user_dir(crate::preference_dir().unwrap(), ".config");
    }

    #[test]
    fn test_preference_dir_for() {
        test_user_dir(
            crate::preference_dir_for("Chrome").unwrap(),
            ".config/Chrome",
        );
    }

    #[test]
    fn test_template_for() {
        test_dir_is_none(crate::template_dir());
    }

    #[test]
    fn test_template_dir_for() {
        test_user_dir(
            crate::template_dir_for("Chrome").unwrap(),
            ".config/Chrome/templates",
        );
    }
}
