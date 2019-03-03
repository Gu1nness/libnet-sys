#[cfg(unix)]
extern crate pkg_config;

#[cfg(unix)]
use std::process::Command;

#[cfg(unix)]
fn main() {
    // Fall back to libnetp-config
    let output = Command::new("libnet-config")
        .arg("--libs")
        .output()
        .expect("Failed to run libnet-config. libnet could not be linked");

    parse_libs_cflags(&output.stdout);
}

/// Adapted from pkg_config
#[cfg(unix)]
fn parse_libs_cflags(output: &[u8]) {
    let words = split_flags(output);
    let parts = words.iter()
        .filter(|l| l.len() > 2)
        .map(|arg| (&arg[0..2], &arg[2..]))
        .collect::<Vec<_>>();

    for &(flag, val) in &parts {
        match flag {
            "-L" => {
                println!("cargo:rustc-link-search=native={}", val);
            }
            "-F" => {
                println!("cargo:rustc-link-search=framework={}", val);
            }
            "-l" => {
                println!("cargo:rustc-link-lib={}", val);
            }
            _ => {}
        }
    }
}

/// Copied from pkg_config
#[cfg(unix)]
fn split_flags(output: &[u8]) -> Vec<String> {
    let mut word = Vec::new();
    let mut words = Vec::new();
    let mut escaped = false;

    for &b in output {
        match b {
            _ if escaped => {
                escaped = false;
                word.push(b);
            }
            b'\\' => {
                escaped = true
            }
            b'\t' | b'\n' | b'\r' | b' ' => {
                if !word.is_empty() {
                    words.push(String::from_utf8(word).unwrap());
                    word = Vec::new();
                }
            }
            _ => word.push(b),
        }
    }

    if !word.is_empty() {
        words.push(String::from_utf8(word).unwrap());
    }

    words
}
