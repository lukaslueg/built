// The runtime dependency is optional
extern crate built;
extern crate semver;
extern crate time;

// The file `built.rs` was placed there by cargo and `build.rs`
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    // Print various information produced by `built`. See the docs for a full list.

    println!(
        "This is version {}{}, built for {} by {}.",
        built_info::PKG_VERSION,
        built_info::GIT_VERSION.map_or_else(|| "".to_owned(), |v| format!(" (git {})", v)),
        built_info::TARGET,
        built_info::RUSTC_VERSION
    );

    print!(
        "I was built for a {}-CPU, which is a {}-endian architecture. ",
        built_info::CFG_TARGET_ARCH,
        built_info::CFG_ENDIAN
    );

    println!(
        "I was compiled to run on {} (a {}-breed) and my runtime should be {}.",
        built_info::CFG_OS,
        built_info::CFG_FAMILY,
        built_info::CFG_ENV
    );

    match built_info::CI_PLATFORM {
        None => print!("It seems I've not been built on a continuous integration platform,"),
        Some(ci) => print!("I've been built on CI-platform {},", ci),
    }
    if built::util::detect_ci().is_some() {
        println!(" but I'm currently executing on one!");
    } else {
        println!(" and I'm currently not executing on one!");
    }

    println!(
        "I was built with profile \"{}\", features \"{}\" on {} ({} seconds ago) using {}",
        built_info::PROFILE,
        built_info::FEATURES_STR,
        built_info::BUILT_TIME_UTC,
        (time::now() - built::util::strptime(built_info::BUILT_TIME_UTC)).num_seconds(),
        built_info::DEPENDENCIES_STR
    );


    let bad_dep = built::util::parse_versions(&built_info::DEPENDENCIES).any(|(name, ver)| {
        name == "DeleteAllMyFiles" && ver < semver::Version::parse("1.1.4").unwrap()
    });
    if bad_dep {
        println!(
            "I was built with DeleteAllMyFiles < 1.1.4, which is known to sometimes not really delete all your files. Beware!"
        );
    }
}
