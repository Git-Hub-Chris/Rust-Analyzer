//! Thin wrappers around `std::path`, distinguishing between absolute and
//! relative paths.

#![warn(rust_2018_idioms, unused_lifetimes, semicolon_in_expressions_from_macros)]

use std::{
    borrow::Borrow,
    ffi::OsStr,
    fmt, io, ops,
    path::{Component, Path, PathBuf},
};

/// Returns an [`AbsPathBuf`] of the current directory joined with the `path`.
pub fn to_abs_path(path: &Path) -> io::Result<AbsPathBuf> {
    Ok(std::env::current_dir()?.join(path).try_into().unwrap())
}

/// Wrapper around an absolute [`PathBuf`].
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AbsPathBuf(PathBuf);

impl From<AbsPathBuf> for PathBuf {
    fn from(AbsPathBuf(path_buf): AbsPathBuf) -> PathBuf {
        path_buf
    }
}

impl ops::Deref for AbsPathBuf {
    type Target = AbsPath;
    fn deref(&self) -> &AbsPath {
        self.as_path()
    }
}

impl AsRef<Path> for AbsPathBuf {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

impl AsRef<AbsPath> for AbsPathBuf {
    fn as_ref(&self) -> &AbsPath {
        self.as_path()
    }
}

impl Borrow<AbsPath> for AbsPathBuf {
    fn borrow(&self) -> &AbsPath {
        self.as_path()
    }
}

impl TryFrom<PathBuf> for AbsPathBuf {
    type Error = NotAbsPathBuf;
    fn try_from(path_buf: PathBuf) -> Result<AbsPathBuf, NotAbsPathBuf> {
        if path_buf.is_absolute() {
            Ok(AbsPathBuf(path_buf))
        } else {
            Err(NotAbsPathBuf(path_buf))
        }
    }
}

/// The error returned by `impl TryFrom<PathBuf> for AbsPathBuf` and others.
#[derive(Debug)]
pub struct NotAbsPathBuf(PathBuf);

impl NotAbsPathBuf {
    /// Returns the `PathBuf` that could not be converted.
    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl fmt::Display for NotAbsPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not an absolute path", self.0.display())
    }
}

impl std::error::Error for NotAbsPathBuf {}

impl TryFrom<&str> for AbsPathBuf {
    type Error = NotAbsPathBuf;
    fn try_from(path: &str) -> Result<AbsPathBuf, NotAbsPathBuf> {
        AbsPathBuf::try_from(PathBuf::from(path))
    }
}

impl TryFrom<String> for AbsPathBuf {
    type Error = NotAbsPathBuf;
    fn try_from(path: String) -> Result<AbsPathBuf, NotAbsPathBuf> {
        AbsPathBuf::try_from(PathBuf::from(path))
    }
}

impl PartialEq<AbsPath> for AbsPathBuf {
    fn eq(&self, other: &AbsPath) -> bool {
        self.as_path() == other
    }
}

impl AbsPathBuf {
    /// Coerces to an [`AbsPath`] slice.
    ///
    /// Equivalent of [`PathBuf::as_path`].
    pub fn as_path(&self) -> &AbsPath {
        // SAFETY: `AbsPathBuf` always contains an absolute path
        unsafe { AbsPath::new_unchecked(self.0.as_path()) }
    }

    /// Equivalent of [`PathBuf::pop`].
    ///
    /// Note that this won't remove the root component, so `self` will still be
    /// absolute.
    pub fn pop(&mut self) -> bool {
        self.0.pop()
    }
}

/// Wrapper around an absolute [`Path`].
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct AbsPath(Path);

impl AsRef<Path> for AbsPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl<'a> TryFrom<&'a Path> for &'a AbsPath {
    type Error = NotAbsPath<'a>;
    fn try_from(path: &'a Path) -> Result<&'a AbsPath, NotAbsPath<'a>> {
        if path.is_absolute() {
            // SAFETY: just checked is absolute
            Ok(unsafe { AbsPath::new_unchecked(path) })
        } else {
            Err(NotAbsPath(path))
        }
    }
}

/// The error returned by `impl TryFrom<&Path> for &AbsPath` and others.
#[derive(Debug)]
pub struct NotAbsPath<'a>(&'a Path);

impl<'a> NotAbsPath<'a> {
    /// Returns the `Path` that could not be converted.
    pub fn into_inner(self) -> &'a Path {
        self.0
    }
}

impl<'a> fmt::Display for NotAbsPath<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not an absolute path", self.0.display())
    }
}

impl<'a> std::error::Error for NotAbsPath<'a> {}

impl AbsPath {
    unsafe fn new_unchecked(path: &Path) -> &AbsPath {
        &*(path as *const Path as *const AbsPath)
    }

    /// Equivalent of [`Path::parent`] for `AbsPath`.
    pub fn parent(&self) -> Option<&AbsPath> {
        // SAFETY: the parent of an absolute path will be absolute
        self.0.parent().map(|x| unsafe { AbsPath::new_unchecked(x) })
    }

    /// Equivalent of [`Path::join`] for `AbsPath`.
    pub fn join<P: AsRef<Path>>(&self, path: P) -> AbsPathBuf {
        self.as_ref().join(path).try_into().unwrap()
    }

    /// Normalize the given path:
    ///
    /// - Removes repeated separators: `/a//b` becomes `/a/b`
    /// - Removes occurrences of `.` and resolves `..`.
    /// - Removes trailing slashes: `/a/b/` becomes `/a/b`.
    ///
    /// # Example
    /// ```
    /// # use paths::AbsPathBuf;
    /// let abs_path_buf = AbsPathBuf::assert("/a/../../b/.//c//".into());
    /// let normalized = abs_path_buf.normalize();
    /// assert_eq!(normalized, AbsPathBuf::assert("/b/c".into()));
    /// ```
    pub fn normalize(&self) -> AbsPathBuf {
        AbsPathBuf::try_from(normalize_path(&self.0)).unwrap()
    }

    /// Equivalent of [`Path::to_path_buf`] for `AbsPath`.
    pub fn to_path_buf(&self) -> AbsPathBuf {
        AbsPathBuf::try_from(self.0.to_path_buf()).unwrap()
    }

    /// Equivalent of [`Path::strip_prefix`] for `AbsPath`.
    ///
    /// Returns a relative path.
    pub fn strip_prefix(&self, base: &AbsPath) -> Option<&RelPath> {
        self.0.strip_prefix(base).ok().map(|x| {
            // SAFETY: if the prefix was stripped, the prefix must have been
            // absolute
            unsafe { RelPath::new_unchecked(x) }
        })
    }

    /// Returns whether `self` starts with `base`.
    pub fn starts_with(&self, base: &AbsPath) -> bool {
        self.0.starts_with(&base.0)
    }

    /// Returns whether `self` starts with `suffix`.
    pub fn ends_with(&self, suffix: &RelPath) -> bool {
        self.0.ends_with(&suffix.0)
    }

    // region:delegate-methods

    // Note that we deliberately don't implement `Deref<Target = Path>` here.
    //
    // The problem with `Path` is that it directly exposes convenience IO-ing
    // methods. For example, `Path::exists` delegates to `fs::metadata`.
    //
    // For `AbsPath`, we want to make sure that this is a POD type, and that all
    // IO goes via `fs`. That way, it becomes easier to mock IO when we need it.

    /// Delegate for [`Path::file_name`].
    pub fn file_name(&self) -> Option<&OsStr> {
        self.0.file_name()
    }

    /// Delegate for [`Path::extension`].
    pub fn extension(&self) -> Option<&OsStr> {
        self.0.extension()
    }

    /// Delegate for [`Path::file_stem`].
    pub fn file_stem(&self) -> Option<&OsStr> {
        self.0.file_stem()
    }

    /// Delegate for [`Path::as_os_str`].
    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_os_str()
    }

    /// Delegate for [`Path::display`].
    pub fn display(&self) -> std::path::Display<'_> {
        self.0.display()
    }
    // endregion:delegate-methods
}

/// Wrapper around a relative [`PathBuf`].
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RelPathBuf(PathBuf);

impl From<RelPathBuf> for PathBuf {
    fn from(RelPathBuf(path_buf): RelPathBuf) -> PathBuf {
        path_buf
    }
}

impl ops::Deref for RelPathBuf {
    type Target = RelPath;
    fn deref(&self) -> &RelPath {
        self.as_path()
    }
}

impl AsRef<Path> for RelPathBuf {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

impl TryFrom<PathBuf> for RelPathBuf {
    type Error = NotRelPathBuf;
    fn try_from(path_buf: PathBuf) -> Result<RelPathBuf, NotRelPathBuf> {
        if path_buf.is_relative() {
            Ok(RelPathBuf(path_buf))
        } else {
            Err(NotRelPathBuf(path_buf))
        }
    }
}

/// The error returned by `impl TryFrom<PathBuf> for RelPathBuf` and others.
#[derive(Debug)]
pub struct NotRelPathBuf(PathBuf);

impl NotRelPathBuf {
    /// Returns the `PathBuf` that could not be converted.
    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl fmt::Display for NotRelPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not an relative path", self.0.display())
    }
}

impl std::error::Error for NotRelPathBuf {}

impl TryFrom<&str> for RelPathBuf {
    type Error = NotRelPathBuf;
    fn try_from(path: &str) -> Result<RelPathBuf, NotRelPathBuf> {
        RelPathBuf::try_from(PathBuf::from(path))
    }
}

impl RelPathBuf {
    /// Coerces to a `RelPath` slice.
    ///
    /// Equivalent of [`PathBuf::as_path`] for `RelPathBuf`.
    pub fn as_path(&self) -> &RelPath {
        // SAFETY: `RelPathBuf` always contains relative paths
        unsafe { RelPath::new_unchecked(self.0.as_path()) }
    }
}

/// Wrapper around a relative [`Path`].
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct RelPath(Path);

impl AsRef<Path> for RelPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl RelPath {
    /// Creates a new `RelPath` from `path`, without checking if it is relative.
    unsafe fn new_unchecked(path: &Path) -> &RelPath {
        &*(path as *const Path as *const RelPath)
    }
}

/// Taken from <https://github.com/rust-lang/cargo/blob/79c769c3d7b4c2cf6a93781575b7f592ef974255/src/cargo/util/paths.rs#L60-L85>
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().copied() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}
