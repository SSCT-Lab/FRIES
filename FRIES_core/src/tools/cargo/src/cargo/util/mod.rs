use std::fmt;
use std::time::Duration;

pub use self::canonical_url::CanonicalUrl;
pub use self::config::{homedir, Config, ConfigValue};
pub(crate) use self::counter::MetricsCounter;
pub use self::dependency_queue::DependencyQueue;
pub use self::diagnostic_server::RustfixDiagnosticServer;
pub use self::errors::CliError;
pub use self::errors::{internal, CargoResult, CliResult};
pub use self::flock::{FileLock, Filesystem};
pub use self::graph::Graph;
pub use self::hasher::StableHasher;
pub use self::hex::{hash_u64, short_hash, to_hex};
pub use self::into_url::IntoUrl;
pub use self::into_url_with_base::IntoUrlWithBase;
pub(crate) use self::io::LimitErrorReader;
pub use self::lev_distance::{closest, closest_msg, lev_distance};
pub use self::lockserver::{LockServer, LockServerClient, LockServerStarted};
pub use self::progress::{Progress, ProgressStyle};
pub use self::queue::Queue;
pub use self::restricted_names::validate_package_name;
pub use self::rustc::Rustc;
pub use self::semver_ext::{OptVersionReq, VersionExt, VersionReqExt};
pub use self::to_semver::ToSemver;
pub use self::vcs::{existing_vcs_repo, FossilRepo, GitRepo, HgRepo, PijulRepo};
pub use self::workspace::{
    add_path_args, path_args, print_available_benches, print_available_binaries,
    print_available_examples, print_available_packages, print_available_tests,
};

pub mod auth;
mod canonical_url;
pub mod command_prelude;
pub mod config;
mod counter;
pub mod cpu;
mod dependency_queue;
pub mod diagnostic_server;
pub mod errors;
mod flock;
pub mod graph;
mod hasher;
pub mod hex;
pub mod important_paths;
pub mod interning;
pub mod into_url;
mod into_url_with_base;
mod io;
pub mod job;
pub mod lev_distance;
mod lockserver;
pub mod machine_message;
pub mod network;
pub mod profile;
mod progress;
mod queue;
pub mod restricted_names;
pub mod rustc;
mod semver_ext;
pub mod to_semver;
pub mod toml;
pub mod toml_mut;
mod vcs;
mod workspace;

pub fn elapsed(duration: Duration) -> String {
    let secs = duration.as_secs();

    if secs >= 60 {
        format!("{}m {:02}s", secs / 60, secs % 60)
    } else {
        format!("{}.{:02}s", secs, duration.subsec_nanos() / 10_000_000)
    }
}

/// Formats a number of bytes into a human readable SI-prefixed size.
/// Returns a tuple of `(quantity, units)`.
pub fn human_readable_bytes(bytes: u64) -> (f32, &'static str) {
    static UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    let bytes = bytes as f32;
    let i = ((bytes.log2() / 10.0) as usize).min(UNITS.len() - 1);
    (bytes / 1024_f32.powi(i as i32), UNITS[i])
}

pub fn iter_join_onto<W, I, T>(mut w: W, iter: I, delim: &str) -> fmt::Result
where
    W: fmt::Write,
    I: IntoIterator<Item = T>,
    T: std::fmt::Display,
{
    let mut it = iter.into_iter().peekable();
    while let Some(n) = it.next() {
        write!(w, "{}", n)?;
        if it.peek().is_some() {
            write!(w, "{}", delim)?;
        }
    }
    Ok(())
}

pub fn iter_join<I, T>(iter: I, delim: &str) -> String
where
    I: IntoIterator<Item = T>,
    T: std::fmt::Display,
{
    let mut s = String::new();
    let _ = iter_join_onto(&mut s, iter, delim);
    s
}

pub fn indented_lines(text: &str) -> String {
    text.lines()
        .map(|line| {
            if line.is_empty() {
                String::from("\n")
            } else {
                format!("  {}\n", line)
            }
        })
        .collect()
}

pub fn truncate_with_ellipsis(s: &str, max_width: usize) -> String {
    // We should truncate at grapheme-boundary and compute character-widths,
    // yet the dependencies on unicode-segmentation and unicode-width are
    // not worth it.
    let mut chars = s.chars();
    let mut prefix = (&mut chars).take(max_width - 1).collect::<String>();
    if chars.next().is_some() {
        prefix.push('…');
    }
    prefix
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_human_readable_bytes() {
        assert_eq!(human_readable_bytes(0), (0., "B"));
        assert_eq!(human_readable_bytes(8), (8., "B"));
        assert_eq!(human_readable_bytes(1000), (1000., "B"));
        assert_eq!(human_readable_bytes(1024), (1., "KiB"));
        assert_eq!(human_readable_bytes(1024 * 420 + 512), (420.5, "KiB"));
        assert_eq!(human_readable_bytes(1024 * 1024), (1., "MiB"));
        assert_eq!(
            human_readable_bytes(1024 * 1024 + 1024 * 256),
            (1.25, "MiB")
        );
        assert_eq!(human_readable_bytes(1024 * 1024 * 1024), (1., "GiB"));
        assert_eq!(
            human_readable_bytes((1024. * 1024. * 1024. * 3.1415) as u64),
            (3.1415, "GiB")
        );
        assert_eq!(human_readable_bytes(1024 * 1024 * 1024 * 1024), (1., "TiB"));
        assert_eq!(
            human_readable_bytes(1024 * 1024 * 1024 * 1024 * 1024),
            (1., "PiB")
        );
        assert_eq!(
            human_readable_bytes(1024 * 1024 * 1024 * 1024 * 1024 * 1024),
            (1., "EiB")
        );
        assert_eq!(human_readable_bytes(u64::MAX), (16., "EiB"));
    }
}