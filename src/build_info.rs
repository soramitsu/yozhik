#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/built.rs"));

#[derive(Serialize, Debug)]
pub struct Info {
    version: &'static str,
    git_version: Option<&'static str>,
    rustc_version: &'static str,
    target_platform: &'static str,
    compiler_platform: &'static str,
    built_time: &'static str
}

pub const INFO: Info = Info {
    version: PKG_VERSION,
    git_version: GIT_VERSION,
    rustc_version: RUSTC_VERSION,
    target_platform: TARGET,
    compiler_platform: HOST,
    built_time: BUILT_TIME_UTC,
};
