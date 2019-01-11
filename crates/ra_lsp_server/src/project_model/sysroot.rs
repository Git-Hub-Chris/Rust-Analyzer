use std::{
    path::{Path, PathBuf},
    process::Command,
};

use ra_syntax::SmolStr;
use ra_arena::{Arena, RawId, impl_arena_id};

use crate::Result;

#[derive(Debug, Clone)]
pub struct Sysroot {
    crates: Arena<SysrootCrate, SysrootCrateData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SysrootCrate(RawId);
impl_arena_id!(SysrootCrate);

#[derive(Debug, Clone)]
struct SysrootCrateData {
    name: SmolStr,
    root: PathBuf,
    deps: Vec<SysrootCrate>,
}

impl Sysroot {
    pub(crate) fn std(&self) -> Option<SysrootCrate> {
        self.by_name("std")
    }

    pub(crate) fn crates<'a>(&'a self) -> impl Iterator<Item = SysrootCrate> + 'a {
        self.crates.iter().map(|(id, _data)| id)
    }

    pub(super) fn discover(cargo_toml: &Path) -> Result<Sysroot> {
        let rustc_output = Command::new("rustc")
            .current_dir(cargo_toml.parent().unwrap())
            .args(&["--print", "sysroot"])
            .output()?;
        if !rustc_output.status.success() {
            failure::bail!("failed to locate sysroot")
        }
        let stdout = String::from_utf8(rustc_output.stdout)?;
        let sysroot_path = Path::new(stdout.trim());
        let src = sysroot_path.join("lib/rustlib/src/rust/src");
        if !src.exists() {
            failure::bail!(
                "can't load standard library from sysroot\n\
                 {:?}\n\
                 try running `rustup component add rust-src`",
                src,
            );
        }

        let mut sysroot = Sysroot {
            crates: Arena::default(),
        };
        for name in SYSROOT_CRATES.trim().lines() {
            let root = src.join(format!("lib{}", name)).join("lib.rs");
            if root.exists() {
                sysroot.crates.alloc(SysrootCrateData {
                    name: name.into(),
                    root,
                    deps: Vec::new(),
                });
            }
        }
        if let Some(std) = sysroot.std() {
            for dep in STD_DEPS.trim().lines() {
                if let Some(dep) = sysroot.by_name(dep) {
                    sysroot.crates[std].deps.push(dep)
                }
            }
        }
        Ok(sysroot)
    }

    fn by_name(&self, name: &str) -> Option<SysrootCrate> {
        self.crates
            .iter()
            .find(|(_id, data)| data.name == name)
            .map(|(id, _data)| id)
    }
}

impl SysrootCrate {
    pub(crate) fn name(self, sysroot: &Sysroot) -> &SmolStr {
        &sysroot.crates[self].name
    }
    pub(crate) fn root(self, sysroot: &Sysroot) -> &Path {
        sysroot.crates[self].root.as_path()
    }
    pub(crate) fn root_dir(self, sysroot: &Sysroot) -> &Path {
        self.root(sysroot).parent().unwrap()
    }
    pub(crate) fn deps<'a>(self, sysroot: &'a Sysroot) -> impl Iterator<Item = SysrootCrate> + 'a {
        sysroot.crates[self].deps.iter().map(|&it| it)
    }
}

const SYSROOT_CRATES: &str = "
std
core
alloc
collections
libc
panic_unwind
proc_macro
rustc_unicode
std_unicode
test
alloc_jemalloc
alloc_system
compiler_builtins
getopts
panic_unwind
panic_abort
rand
term
unwind
build_helper
rustc_asan
rustc_lsan
rustc_msan
rustc_tsan
syntax";

const STD_DEPS: &str = "
alloc_jemalloc
alloc_system
panic_abort
rand
compiler_builtins
unwind
rustc_asan
rustc_lsan
rustc_msan
rustc_tsan
build_helper";
