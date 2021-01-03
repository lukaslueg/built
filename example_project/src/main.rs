// The file `built.rs` was placed there by cargo and `build.rs`
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    // Print various information produced by `built`. See the docs for a full list.

    println!(
        "This is version {}, built for {} by {}.\n",
        built_info::PKG_VERSION,
        built_info::TARGET,
        built_info::RUSTC_VERSION
    );

    if let (Some(v), Some(dirty), Some(hash)) = (
        built_info::GIT_VERSION,
        built_info::GIT_DIRTY,
        built_info::GIT_COMMIT_HASH,
    ) {
        print!(
            "I was built from git `{}`, commit {}; the working directory was \"{}\".",
            v,
            hash,
            if dirty { "dirty" } else { "clean" }
        );
    }

    match built_info::GIT_HEAD_REF {
        Some(r) => println!(" The branch was `{}`.\n", r),
        None => println!("\n"),
    }

    print!(
        "I was built for a {}-CPU, which is a {}-endian architecture. ",
        built_info::CFG_TARGET_ARCH,
        built_info::CFG_ENDIAN
    );

    println!(
        "I was compiled to run on {} (a {}-breed) and my runtime should be {}.\n",
        built_info::CFG_OS,
        built_info::CFG_FAMILY,
        built_info::CFG_ENV
    );

    match built_info::CI_PLATFORM {
        None => print!("It seems I've not been built on a continuous integration platform,"),
        Some(ci) => print!("I've been built on CI-platform {},", ci),
    }
    if built::util::detect_ci().is_some() {
        println!(" but I'm currently executing on one!\n");
    } else {
        println!(" and I'm currently not executing on one!\n");
    }

    let built_time = built::util::strptime(built_info::BUILT_TIME_UTC);
    println!(
        "I was built with profile \"{}\", features \"{}\" on {} ({} seconds ago) using {}",
        built_info::PROFILE,
        built_info::FEATURES_STR,
        built_time.with_timezone(&built::chrono::offset::Local),
        (built::chrono::offset::Utc::now() - built_time).num_seconds(),
        built_info::DEPENDENCIES_STR
    );

    let bad_dep =
        built::util::parse_versions(built_info::DEPENDENCIES.iter()).any(|(name, ver)| {
            name == "DeleteAllMyFiles" && ver < built::semver::Version::parse("1.1.4").unwrap()
        });
    if bad_dep {
        println!(
            "I was built with DeleteAllMyFiles < 1.1.4, which is known to sometimes not really delete all your files. Beware!"
        );
    }
}
