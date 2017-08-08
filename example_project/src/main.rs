pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    println!("Hello, world!");

    println!(
        "This is version {}{}, built for {} by {}.",
        built_info::PKG_VERSION,
        built_info::GIT_VERSION.map_or_else(|| "".to_owned(), |v| format!(" (git {})", v)),
        built_info::TARGET,
        built_info::RUSTC_VERSION
    );

    println!(
        "I was built for a {}-CPU, which is a {}-endian architecture.",
        built_info::CFG_TARGET_ARCH,
        built_info::CFG_ENDIAN
    );

    println!(
        "I'm running on {} (a {}-breed) and my runtime is provided by {}.",
        built_info::CFG_OS,
        built_info::CFG_FAMILY,
        built_info::CFG_ENV
    );

    println!(
        "I was built with profile \"{}\", features \"{}\" on {} using {}",
        built_info::PROFILE,
        built_info::FEATURES_STR,
        built_info::BUILT_TIME_UTC,
        built_info::DEPENDENCIES_STR
    );

}
