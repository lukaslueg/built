use crate::util::TupleArrayDisplay;
use crate::{write_str_variable, write_variable};
use std::{collections, fs, io, path};

fn package_names<'a, I>(packages: I) -> Vec<(String, String)>
where
    I: IntoIterator<Item = &'a cargo_lock::Package>,
{
    let mut res = packages
        .into_iter()
        .map(|package| (package.name.to_string(), package.version.to_string()))
        .collect::<collections::HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    res.sort_unstable();
    res
}

fn find_lockfile(base: &path::Path) -> io::Result<path::PathBuf> {
    base.ancestors()
        .find_map(|p| {
            let lockfile = p.join("Cargo.lock");
            lockfile.exists().then(|| lockfile.to_owned())
        })
        .ok_or(io::Error::other("Cargo.lock not found"))
}

#[cfg(feature = "dependency-tree")]
struct Dependencies {
    deps: Vec<(String, String)>,
    direct_deps: Vec<(String, String)>,
    indirect_deps: Vec<(String, String)>,
}

#[cfg(feature = "dependency-tree")]
impl Dependencies {
    fn new(
        mut lockfile: cargo_lock::Lockfile,
        package: &cargo_lock::Dependency,
    ) -> io::Result<Self> {
        use cargo_lock::dependency::graph::EdgeDirection;

        // cargo-lock builds the graph from `packages`. Version 1 lockfiles can
        // store the top-level package separately in a `[root]` table.
        if let Some(root) = lockfile.root.take() {
            lockfile.packages.push(root);
        }
        let tree = lockfile
            .dependency_tree()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        let graph = tree.graph();
        let root = *tree.nodes().get(package).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Local package {package} is missing from Cargo.lock"),
            )
        })?;

        let direct = graph
            .neighbors_directed(root, EdgeDirection::Outgoing)
            .filter(|&idx| idx != root)
            .collect::<collections::HashSet<_>>();
        let mut reachable = direct.clone();
        reachable.insert(root);
        let mut pending = direct.iter().copied().collect::<Vec<_>>();
        while let Some(idx) = pending.pop() {
            pending.extend(
                graph
                    .neighbors_directed(idx, EdgeDirection::Outgoing)
                    .filter(|&neighbor| reachable.insert(neighbor)),
            );
        }
        reachable.remove(&root);

        Ok(Self {
            deps: package_names(reachable.iter().map(|&idx| &graph[idx])),
            direct_deps: package_names(direct.iter().map(|&idx| &graph[idx])),
            indirect_deps: package_names(reachable.difference(&direct).map(|&idx| &graph[idx])),
        })
    }
}

#[cfg(feature = "dependency-tree")]
pub fn write_dependencies(
    manifest_location: &path::Path,
    envmap: &crate::environment::EnvironmentMap,
    mut w: &fs::File,
) -> io::Result<()> {
    use io::Write;

    let package = cargo_lock::Dependency {
        name: envmap
            .get("CARGO_PKG_NAME")
            .ok_or_else(|| io::Error::other("CARGO_PKG_NAME is missing"))?
            .parse()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?,
        version: envmap
            .get("CARGO_PKG_VERSION")
            .ok_or_else(|| io::Error::other("CARGO_PKG_VERSION is missing"))?
            .parse()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?,
        // Cargo omits the source from lockfile entries for local paths.
        source: None,
    };
    let lockfile = fs::read_to_string(find_lockfile(manifest_location)?)?
        .parse()
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
    let dependencies = Dependencies::new(lockfile, &package)?;

    write_variable!(
        w,
        "DEPENDENCIES",
        format_args!("[(&str, &str); {}]", dependencies.deps.len()),
        TupleArrayDisplay(&dependencies.deps),
        "An array of effective dependencies as documented by `Cargo.lock`."
    );
    write_str_variable!(
        w,
        "DEPENDENCIES_STR",
        dependencies
            .deps
            .iter()
            .map(|(n, v)| format!("{n} {v}"))
            .collect::<Vec<_>>()
            .join(", "),
        "The effective dependencies as a comma-separated string."
    );

    write_variable!(
        w,
        "DIRECT_DEPENDENCIES",
        format_args!("[(&str, &str); {}]", dependencies.direct_deps.len()),
        TupleArrayDisplay(&dependencies.direct_deps),
        "An array of direct dependencies as documented by `Cargo.lock`."
    );
    write_str_variable!(
        w,
        "DIRECT_DEPENDENCIES_STR",
        dependencies
            .direct_deps
            .iter()
            .map(|(n, v)| format!("{n} {v}"))
            .collect::<Vec<_>>()
            .join(", "),
        "The direct dependencies as a comma-separated string."
    );

    write_variable!(
        w,
        "INDIRECT_DEPENDENCIES",
        format_args!("[(&str, &str); {}]", dependencies.indirect_deps.len()),
        TupleArrayDisplay(&dependencies.indirect_deps),
        "An array of indirect dependencies as documented by `Cargo.lock`."
    );
    write_str_variable!(
        w,
        "INDIRECT_DEPENDENCIES_STR",
        dependencies
            .indirect_deps
            .iter()
            .map(|(n, v)| format!("{n} {v}"))
            .collect::<Vec<_>>()
            .join(", "),
        "The indirect dependencies as a comma-separated string."
    );

    Ok(())
}

#[cfg(not(feature = "dependency-tree"))]
pub fn write_dependencies(manifest_location: &path::Path, mut w: &fs::File) -> io::Result<()> {
    use io::{Read, Write};

    let mut lock_buf = String::new();
    fs::File::open(find_lockfile(manifest_location)?)?.read_to_string(&mut lock_buf)?;
    let lockfile: cargo_lock::Lockfile = lock_buf.parse().expect("Failed to parse lockfile");

    let deps = package_names(&lockfile.packages);

    write_variable!(
        w,
        "DEPENDENCIES",
        format_args!("[(&str, &str); {}]", deps.len()),
        TupleArrayDisplay(&deps),
        "An array of effective dependencies as documented by `Cargo.lock`."
    );
    write_str_variable!(
        w,
        "DEPENDENCIES_STR",
        deps.iter()
            .map(|(n, v)| format!("{n} {v}"))
            .collect::<Vec<_>>()
            .join(", "),
        "The effective dependencies as a comma-separated string."
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    static LOCK_TOML_BUFFER: &str = r#"
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 3

[[package]]
name = "foo"
version = "0.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f7dbb6acfeff1d490fba693a402456f76b344fea77a5e7cae43b5970c3332b8f"

[[package]]
name = "foobar"
version = "0.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d9c0d152c1d2a9673211b9f3c02a4786715ce730dbd5f94f2f895fc0bb9eed63"

[[package]]
name = "memchr"
version = "2.6.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8f232d6ef707e1956a43342693d2a31e72989554d58299d7a88738cc95b0d35c"

[[package]]
name = "minimal-lexical"
version = "0.2.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "68354c5c6bd36d73ff3feceb05efa59b6acb7626617f4962be322a825e61f79a"

[[package]]
name = "nom"
version = "7.1.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d273983c5a657a70a3e8f2a01329822f3b8c8172b73826411a55751e404a0a4a"
dependencies = [
 "memchr",
 "minimal-lexical",
]

[[package]]
name = "dummy"
version = "0.1.0"
dependencies = [
 "foo",
 "foobar",
 "nom",
]
"#;

    #[test]
    fn parse_deps() {
        let lockfile: cargo_lock::Lockfile =
            LOCK_TOML_BUFFER.parse().expect("Failed to parse lockfile");
        let deps = super::package_names(&lockfile.packages);
        assert_eq!(
            deps,
            [
                ("dummy".to_owned(), "0.1.0".to_owned()),
                ("foo".to_owned(), "0.0.0".to_owned()),
                ("foobar".to_owned(), "0.0.0".to_owned()),
                ("memchr".to_owned(), "2.6.3".to_owned()),
                ("minimal-lexical".to_owned(), "0.2.1".to_owned()),
                ("nom".to_owned(), "7.1.3".to_owned()),
            ]
        );
    }

    #[test]
    #[cfg(feature = "dependency-tree")]
    fn direct_deps() {
        let lockfile = LOCK_TOML_BUFFER.parse().expect("Failed to parse lockfile");
        let package = cargo_lock::Dependency {
            name: "dummy".parse().unwrap(),
            version: "0.1.0".parse().unwrap(),
            source: None,
        };
        let dependencies = super::Dependencies::new(lockfile, &package).unwrap();
        assert_eq!(
            dependencies.deps,
            [
                ("foo".to_owned(), "0.0.0".to_owned()),
                ("foobar".to_owned(), "0.0.0".to_owned()),
                ("memchr".to_owned(), "2.6.3".to_owned()),
                ("minimal-lexical".to_owned(), "0.2.1".to_owned()),
                ("nom".to_owned(), "7.1.3".to_owned()),
            ]
        );
        assert_eq!(
            dependencies.direct_deps,
            [
                ("foo".to_owned(), "0.0.0".to_owned()),
                ("foobar".to_owned(), "0.0.0".to_owned()),
                ("nom".to_owned(), "7.1.3".to_owned()),
            ]
        );
        assert_eq!(
            dependencies.indirect_deps,
            [
                ("memchr".to_owned(), "2.6.3".to_owned()),
                ("minimal-lexical".to_owned(), "0.2.1".to_owned()),
            ]
        );
    }

    #[test]
    #[cfg(feature = "dependency-tree")]
    fn source_disambiguates_package_identity() {
        let lockfile = r#"
version = 4

[[package]]
name = "Mixed-Name"
version = "1.2.3-RC.1+Build.007"
source = "registry+https://github.com/rust-lang/crates.io-index"

[[package]]
name = "Mixed-Name"
version = "1.2.3-RC.1+Build.007"
dependencies = [
 "Mixed-Name 1.2.3-RC.1+Build.007 (registry+https://github.com/rust-lang/crates.io-index)",
]
"#
        .parse()
        .unwrap();
        let package = cargo_lock::Dependency {
            name: "Mixed-Name".parse().unwrap(),
            version: "1.2.3-RC.1+Build.007".parse().unwrap(),
            source: None,
        };
        let dependencies = super::Dependencies::new(lockfile, &package).unwrap();
        assert_eq!(
            dependencies.direct_deps,
            [("Mixed-Name".to_owned(), "1.2.3-RC.1+Build.007".to_owned())]
        );
        assert_eq!(dependencies.deps, dependencies.direct_deps);
        assert!(dependencies.indirect_deps.is_empty());
    }

    #[test]
    #[cfg(feature = "dependency-tree")]
    fn legacy_root() {
        let lockfile = r#"
[root]
name = "legacy-root"
version = "1.2.3"
dependencies = ["legacy-dep 1.2.3"]

[[package]]
name = "legacy-dep"
version = "1.2.3"
"#
        .parse()
        .unwrap();
        let package = cargo_lock::Dependency {
            name: "legacy-root".parse().unwrap(),
            version: "1.2.3".parse().unwrap(),
            source: None,
        };
        let dependencies = super::Dependencies::new(lockfile, &package).unwrap();
        assert_eq!(
            dependencies.direct_deps,
            [("legacy-dep".to_owned(), "1.2.3".to_owned())]
        );
        assert_eq!(dependencies.deps, dependencies.direct_deps);
        assert!(dependencies.indirect_deps.is_empty());
    }
}
