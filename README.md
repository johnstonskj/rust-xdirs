# Crate xdirs

Additional functions for application paths, based on [dirs-next](https://crates.io/crates/dirs-next).

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/xdirs.svg)](https://crates.io/crates/xdirs)
[![docs.rs](https://docs.rs/xdirs/badge.svg)](https://docs.rs/xdirs)
![Build](https://github.com/johnstonskj/rust-xdirs/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/rust-xdirs/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-xdirs.svg)](https://github.com/johnstonskj/rust-xdirs/stargazers)

-----

This crate extends the set of paths used by an application to store data and configuration. It
compliments functions such as `cache_dir`, `config_dir`, and `data_dir` with versions with a
suffix `_for` that take an application name. It is not the case that, for all of such paths, it
is safe to append an application name to the generic path, and so these additional functions will
ensure the correct construction of the application-specific path.

| Generic Form       | dirs | Application-Specific Form |
| ------------------ | ---- | ------------------------- |
| `cache_dir`        | Yes  | `cache_dir_for`           |
| `config_dir`       | Yes  | `config_dir_for`          |
| `data_dir`         | Yes  | `data_dir_for`            |
| `data_local_dir`   | Yes  | `data_local_dir_for`      |
| `favorites_dir`    | No   | `favorites_dir_for`       |
| `preference_dir`   | No   | `preference_dir_for`      |
| `cache_dir`        | No   | `cache_dir_for`           |
| `template_dir`     | No   | `template_dir_for`        |

The column *dirs* denotes whether the generic form is present in the 
[dirs](https://crates.io/crates/dirs) or [dirs-next](https://crates.io/crates/dirs-next) crate.

# Example

```rust
use xdirs::{application_dir, config_dir_for, log_dir_for};

const APP_NAME: &str = env!("CARGO_PKG_NAME");

let where_is_my_app_installed = application_dir();
let where_do_i_read_my_config = config_dir_for(APP_NAME);
let where_do_i_write_log_files = log_dir_for(APP_NAME);
```

# Additional Functions

The following may be used to determine the location for installed applications.

* `application_dir`
* `application_shared_dir`
* `user_application_dir`

Finally, for systems that support a notion of an application container or bundle, the following
will provide the location to these directories. Currently these only provide values on macOS.

* `app_container_dir_for`
* `app_container_executable_dir_for`
* `user_app_container_dir_for`
* `user_app_container_executable_dir_for`

As is the case for dirs-next, this library provides the location of these directories by leveraging
the mechanisms defined by

* the [XDG base directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) and the [XDG user directory](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) specifications on Linux,
* the [Known Folder](https://msdn.microsoft.com/en-us/library/windows/desktop/bb776911(v=vs.85).aspx) system on Windows, and
* the [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6) on macOS.


-----

## Changes

**Version 0.1.0**

* Initial version, all primary functions tested on Linux (Ubuntu), macOS, and Windows.
* Documentation included, with the same form as dirs_next.

## TODO

* Support WASM target, dist-next does.
* Look into flatpack, app-image, or snap app container support.
