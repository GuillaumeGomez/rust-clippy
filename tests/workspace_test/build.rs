#![deny(clippy::print_stdout)]
#![deny(clippy::invalid_build_cfg)]
#![allow(clippy::needless_if)]

#[cfg(windows)]
fn unused_windows_fn() {}
#[cfg(not(windows))]
fn unused_not_windows_fn() {}
#[cfg(any(windows, feature = "yellow", unix))]
fn pink() {}
// Should not warn.
#[cfg(feature = "green")]
fn pink() {}

fn main() {
    // Test for #6041
    println!("Hello");
    print!("Hello");

    if cfg!(windows) {
        let _ = 1;
    }

    if cfg!(windows) {}
    if cfg!(not(windows)) {}
    if cfg!(target_os = "freebsd") {}
    if cfg!(any(target_os = "freebsd", windows)) {}
    if cfg!(not(any(target_os = "freebsd", windows))) {}
    if cfg!(all(target_os = "freebsd", windows)) {}
    if cfg!(not(all(target_os = "freebsd", windows))) {}
    if cfg!(all(target_os = "freebsd", any(windows, not(feature = "red")))) {}

    // Should not warn.
    if cfg!(any()) {}
    if cfg!(all()) {}
    if cfg!(feature = "blue") {}
}
