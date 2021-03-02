/*!
Additional functions for application paths, based on [dirs-next](https://crates.io/crates/dirs-next).

This crate extends the set of paths used by an application to store data and configuration. It
compliments functions such as `cache_dir`, `config_dir`, and `data_dir` with versions with a
suffix `_for` that take an application name. It is not the case that for all of such paths it
is safe to append an application name to the generic path, so these additional functions will
ensure the correct construction of the application-specific path.

| Generic Form                  | Application-Specific Form |
| ----------------------------- | ------------------------- |
| `cache_dir`                   | [`cache_dir_for`](fn.cache_dir_for.html)           |
| `config_dir`                  | [`config_dir_for`](fn.config_dir_for.html)         |
| `data_dir`                    | [`data_dir_for`](fn.data_dir_for.html)             |
| `data_local_dir`              | [`data_local_dir_for`](fn.data_local_dir_for.html) |
| [`favorites_dir`](fn.favorites_dir.html)   | [`favorites_dir_for`](fn.favorites_dir_for.html)   |
| [`log_dir`](fn.log_dir.html)  | [`log_dir_for`](fn.log_dir_for.html) |
| [`preference_dir`](fn.preference_dir.html) | [`preference_dir_for`](fn.preference_dir_for.html) |
| [`cache_dir`](fn.cache_dir.html)           | [`cache_dir_for`](fn.cache_dir_for.html)           |
| `template_dir`               | [`template_dir_for`](fn.template_dir_for.html)     |

Additionally the following may be used to determine the location for installed applications.

* [`application_dir`](fn.application_dir.html)
* [`application_shared_dir`](fn.application_shared_dir.html)
* [`user_application_dir`](fn.user_application_dir.html)

Finally, for systems that support a notion of an application container or bundle, the following
will provide the location to these directories. Currently these only provide values on macOS.

* [`app_container_dir_for`](fn.app_container_dir_for.html)
* [`app_container_executable_dir_for`](fn.app_container_executable_dir_for.html)
* [`user_app_container_dir_for`](fn.user_app_container_dir_for.html)
* [`user_app_container_executable_dir_for`](fn.user_app_container_executable_dir_for.html)

As is the case for dirs-next, this library provides the location of these directories by leveraging
the mechanisms defined by

* the [XDG base directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) and the [XDG user directory](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) specifications on Linux,
* the [Known Folder](https://msdn.microsoft.com/en-us/library/windows/desktop/bb776911(v=vs.85).aspx) system on Windows, and
* the [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6) on macOS.

# Example

```rust
use xdirs::{application_dir, config_dir_for, log_dir_for};

const APP_NAME: &str = env!("CARGO_PKG_NAME");

let where_is_my_app_installed = application_dir();
let where_do_i_read_my_config = config_dir_for(APP_NAME);
let where_do_i_write_log_files = log_dir_for(APP_NAME);
```

*/

pub use dirs_next::{cache_dir, config_dir, data_dir, data_local_dir};
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Returns the path to the system's application directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                  |
/// | ------- | -----------------------------------------| ------------------------ |
/// | Linux   | -                                        | -                        |
/// | macOS   | /Applications                            | /Applications            |
/// | Windows | `{FOLDERID_ProgramFiles}`                | C:\Program Files         |
///
/// See also [`application_shared_dir`](fn.application_shared_dir.html), and
///   [`user_application_dir`](fn.user_application_dir.html).
///
pub fn application_dir() -> Option<PathBuf> {
    sys::application_dir()
}

///
/// Returns the path to the system's application shared components directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                  |
/// | ------- | -----------------------------------------| ------------------------ |
/// | Linux   | -                                        | -                        |
/// | macOS   | /Library/Frameworks                      | /Library/Frameworks      |
/// | Windows | `{FOLDERID_ProgramFilesCommon}`          | C:\Program Files\Common Files |
///
/// See also [`application_dir`](fn.application_dir.html) and
///   [`user_application_dir`](fn.user_application_dir.html).
///
pub fn application_shared_dir() -> Option<PathBuf> {
    sys::application_shared_dir()
}

///
/// Returns the path to the user's application directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                  |
/// | ------- | -----------------------------------------| ------------------------ |
/// | Linux   | -                                        | -                        |
/// | macOS   | `$HOME`/Applications                     | /Users/Alice/Applications     |
/// | Windows | `{FOLDERID_ProgramFilesCommon}`          | C:\Program Files\Common Files |
///
/// See also [`application_dir`](fn.application_dir.html) and
///   [`application_shared_dir`](fn.application_shared_dir.html).
///
pub fn user_application_dir() -> Option<PathBuf> {
    sys::user_application_dir()
}

// ------------------------------------------------------------------------------------------------

///
/// Returns the path to the application container directory, for a given system application name.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                  |
/// | ------- | -----------------------------------------| ------------------------ |
/// | Linux   | -                                        | -                        |
/// | macOS   | /Applications/{app}                      | /Applications/MyApp.app  |
/// | Windows | -                                        | -                        |
///
pub fn app_container_dir_for(app: &str) -> Option<PathBuf> {
    sys::app_container_dir_for(app)
}

///
/// Returns the path to the application container's executable directory, for a given system
/// application name.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                  |
/// | ------- | -----------------------------------------| ------------------------ |
/// | Linux   | -                                        | -                        |
/// | macOS   | /Applications/{app}/Contents/MacOS       | /Applications/MyApp.app/Contents/MacOS  |
/// | Windows | -                                        | -                        |
///
pub fn app_container_executable_dir_for(app: &str) -> Option<PathBuf> {
    sys::app_container_executable_dir_for(app)
}

///
/// Returns the path to the application container directory, for a given user application name.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                  |
/// | ------- | -----------------------------------------| ------------------------ |
/// | Linux   | -                                        | -                        |
/// | macOS   | `$HOME`/Applications/{app}               | /Applications/MyApp.app  |
/// | Windows | -                                        | -                        |
///
pub fn user_app_container_dir_for(app: &str) -> Option<PathBuf> {
    sys::user_app_container_dir_for(app)
}

///
/// Returns the path to the application container's executable directory, for a given user
/// application name.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                  |
/// | ------- | -----------------------------------------| ------------------------ |
/// | Linux   | -                                        | -                        |
/// | macOS   | `$HOME`/Applications/{app}/Contents/MacOS | /Users/alica/Applications/MyApp.app/Contents/MacOS  |
/// | Windows | -                                        | -                        |
///
pub fn user_app_container_executable_dir_for(app: &str) -> Option<PathBuf> {
    sys::user_app_container_executable_dir_for(app)
}

// ------------------------------------------------------------------------------------------------

///
/// Returns the path to the user's cache directory for a specific application.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                               | Example                      |
/// | ------- | ----------------------------------- | ---------------------------- |
/// | Linux   | `$XDG_CACHE_HOME` or `$HOME`/.cache/{app} | /home/alice/.cache/MyApp           |
/// | macOS   | `$HOME`/Library/Caches/{app}        | /Users/Alice/Library/Caches/MyApp  |
/// | Windows | `{FOLDERID_LocalAppData}`/{app}     | C:\Users\Alice\AppData\Local\MyApp |
///
/// See also [`cache_dir`](fn.cache_dir.html).
///
pub fn cache_dir_for(app: &str) -> Option<PathBuf> {
    sys::cache_dir_for(app)
}

///
/// Returns the path to the user's config directory for a specific application.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                 | Example                          |
/// | ------- | ------------------------------------- | -------------------------------- |
/// | Linux   | `$XDG_CONFIG_HOME` or `$HOME`/.config/{app} | /home/alice/.config/MyApp              |
/// | macOS   | `$HOME`/Library/Application Support/{app}   | /Users/Alice/Library/Application Support/MyApp |
/// | Windows | `{FOLDERID_RoamingAppData}`/{app}           | C:\Users\Alice\AppData\Roaming\MyApp   |
///
/// See also [`config_dir`](fn.config_dir.html)`.
///
pub fn config_dir_for(app: &str) -> Option<PathBuf> {
    sys::config_dir_for(app)
}

///
/// Returns the path to the user's data directory for a specific application.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                                 |
/// | ------- | ---------------------------------------- | --------------------------------------- |
/// | Linux   | `$XDG_DATA_HOME` or `$HOME`/.local/share/{app} | /home/alice/.local/share/MyApp                 |
/// | macOS   | `$HOME`/Library/Application Support/{app}      | /Users/Alice/Library/Application Support/MyApp |
/// | Windows | `{FOLDERID_RoamingAppData}`/{app}              | C:\Users\Alice\AppData\Roaming\MyApp           |
///
/// See also [`data_dir`](fn.data_dir.html).
///
pub fn data_dir_for(app: &str) -> Option<PathBuf> {
    sys::data_dir_for(app)
}

///
/// Returns the path to the user's local data directory for a specific application.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                                 |
/// | ------- | ---------------------------------------- | --------------------------------------- |
/// | Linux   | `$XDG_DATA_HOME` or `$HOME`/.local/share/{app} | /home/alice/.local/share/MyApp    |
/// | macOS   | `$HOME`/Library/Application Support/{app}      | /Users/Alice/Library/Application Support/MyApp |
/// | Windows | `{FOLDERID_LocalAppData}`/{app}                | C:\Users\Alice\AppData\Local\MyApp |
///
/// See also [`data_local_dir`](fn.data_local_dir.html).
///
pub fn data_local_dir_for(app: &str) -> Option<PathBuf> {
    sys::data_local_dir_for(app)
}

///
/// Returns the path to the user's favorites directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                                 |
/// | ------- | ---------------------------------------- | --------------------------------------- |
/// | Linux   | `$XDG_DATA_HOME` or `$HOME`/.local/share | /home/alice/.local/share/favorites      |
/// | macOS   | `$HOME`/Library/Favorites                | /Users/Alice/Library/Favorites          |
/// | Windows | `{FOLDERID_Favorites}`                   | C:\Users\Alice\Favorites                |
///
/// See also [`favorites_dir_for`](fn.favorites_dir_for.html).
///
pub fn favorites_dir() -> Option<PathBuf> {
    sys::favorites_dir()
}

///
/// Returns the path to the user's favorites directory for a specific application.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                                 |
/// | ------- | ---------------------------------------- | --------------------------------------- |
/// | Linux   | `$XDG_DATA_HOME` or `$HOME`/.local/share/{app} | /home/alice/.local/share/favorites/MyApp    |
/// | macOS   | `$HOME`/Library/Favorites/{app}          | /Users/Alice/Library/Favorites/MyApp    |
/// | Windows | `{FOLDERID_Favorites}`/{app}             | C:\Users\Alice\Favorites\MyApp          |
///
/// See also [`favorites_dir`](fn.favorites_dir.html).
///
pub fn favorites_dir_for(app: &str) -> Option<PathBuf> {
    sys::favorites_dir_for(app)
}

///
/// Returns the path to the user's log file directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                                 |
/// | ------- | ---------------------------------------- | --------------------------------------- |
/// | Linux   | -                                        | -                                       |
/// | macOS   | `$HOME`/Library/Logs                     | /Users/Alice/Library/Logs               |
/// | Windows | `{FOLDERID_LocalAppData}`\Logs           | C:\Users\Alice\AppData\AppData\Local\Logs  |
///
/// See also [`log_dir_for`](fn.log_dir_for.html).
///
pub fn log_dir() -> Option<PathBuf> {
    sys::log_dir()
}

///
/// Returns the path to the user's log file directory for a specific application.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                                 |
/// | ------- | ---------------------------------------- | --------------------------------------- |
/// | Linux   | `$XDG_DATA_HOME` or `$HOME`/.local/share/{app}/logs | /home/alice/.local/share/MyApp/logs |
/// | macOS   | `$HOME`/Library/Logs/{app}               | /Users/Alice/Library/Logs/Chrome        |
/// | Windows | `{FOLDERID_LocalAppData}`\Logs\{app}     | C:\Users\Alice\AppData\AppData\Local\Logs\Chrome  |
///
/// See also [`log_dir`](fn.log_dir.html).
///
pub fn log_dir_for(app: &str) -> Option<PathBuf> {
    sys::log_dir_for(app)
}

///
/// Returns the path to the user's preference file directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                                 |
/// | ------- | ---------------------------------------- | --------------------------------------- |
/// | Linux   | `$XDG_DATA_HOME` or `$HOME`/.config      | /home/alice/.config                     |
/// | macOS   | `$HOME`/Library/Preferences              | /Users/Alice/Library/Preferences        |
/// | Windows | `{FOLDERID_RoamingAppData}`/{app}        | C:\Users\Alice\AppData\Roaming\MyApp    |
///
/// See also [`preference_dir_for`](fn.preference_dir_for.html).
///
pub fn preference_dir() -> Option<PathBuf> {
    sys::preference_dir()
}

///
/// Returns the path to the user's preference file directory for a specific application.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                                 |
/// | ------- | ---------------------------------------- | --------------------------------------- |
/// | Linux   | `$XDG_DATA_HOME` or `$HOME`/.config      | /home/alice/.config                     |
/// | macOS   | `$HOME`/Library/Preferences/{app}        | /Users/Alice/Library/Preferences/Chrome |
/// | Windows | `{FOLDERID_RoamingAppData}`/{app}        | C:\Users\Alice\AppData\Roaming\MyApp    |
///
/// See also [`preference_dir`](fn.preference_dir.html).
///
pub fn preference_dir_for(app: &str) -> Option<PathBuf> {
    sys::preference_dir_for(app)
}

///
/// Returns the path to the user's template directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                                 |
/// | ------- | ---------------------------------------- | --------------------------------------- |
/// | Linux   | `$XDG_TEMPLATES_DIR`                     | /home/alice/Templates                   |
/// | macOS   | -                                        | -                                       |
/// | Windows | `{FOLDERID_Templates}`                   | C:\Users\Alice\AppData\Roaming\Microsoft\Windows\Templates |
///
/// See also [`template_dir_for`](fn.template_dir_for.html).
///
pub fn template_dir() -> Option<PathBuf> {
    sys::template_dir()
}

///
/// Returns the path to the user's template directory for a specific application.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value
/// from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                                 |
/// | ------- | ---------------------------------------- | --------------------------------------- |
/// | Linux   | `$XDG_TEMPLATES_DIR`/{app}               | /home/alice/Templates                   |
/// | macOS   | `$HOME`Library/Application Support/{app}/Templates | /Users/Alice/Library/Application Support/Chrome/Templates |
/// | Windows | `{FOLDERID_Templates}`/{app}             | C:\Users\Alice\AppData\Roaming\Microsoft\Windows\Templates |
///
/// See also [`template_dir`](fn.template_dir.html).
///
pub fn template_dir_for(app: &str) -> Option<PathBuf> {
    sys::template_dir_for(app)
}

// ------------------------------------------------------------------------------------------------
// System-Specific Modules
// ------------------------------------------------------------------------------------------------

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "macos.rs"]
mod sys;

#[cfg(not(any(
    target_arch = "wasm32",
    windows,
    target_os = "macos",
    target_os = "ios"
)))]
#[path = "nix.rs"]
mod sys;

#[cfg(windows)]
#[path = "windows.rs"]
mod sys;
