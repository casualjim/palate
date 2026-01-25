#![doc = include_str!("../README.md")]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

#[cfg(feature = "detect")]
use std::path::Path;

#[cfg(feature = "detect")]
pub use detect::*;
pub use list::FileType;

#[cfg(feature = "detect")]
mod detect;
mod list;

/// Internal resolver for file type detection.
#[derive(Clone)]
#[cfg(feature = "detect")]
enum FileTypeResolver {
    /// Static file type that is always returned.
    Static(FileType),
    /// Dynamic resolver function that examines file content.
    Dynamic(fn(&Path, &str) -> Option<FileType>),
}

#[cfg(feature = "detect")]
impl FileTypeResolver {
    /// Resolves the file type, potentially examining file content.
    fn resolve(&self, path: &Path, content: &str) -> Option<FileType> {
        match self {
            FileTypeResolver::Static(ft) => Some(*ft),
            FileTypeResolver::Dynamic(func) => func(path, content),
        }
    }
}

#[cfg(all(test, feature = "detect"))]
mod tests {
    use super::*;

    #[test]
    fn paths() {
        assert_eq!(FileType::Zsh, detect(Path::new("/etc/zprofile"), ""));
        assert_eq!(FileType::Toml, detect(Path::new(".cargo/config"), ""));
    }

    #[test]
    fn filenames() {
        assert_eq!(FileType::Json, detect(Path::new(".prettierrc"), ""));
        assert_eq!(FileType::CMake, detect(Path::new("CMakeLists.txt"), ""));
    }

    #[test]
    fn patterns() {
        assert_eq!(FileType::Scheme, detect(Path::new("highlights.scm"), ""));
        assert_eq!(
            FileType::TreeSitterQuery,
            detect(Path::new("a/b/c/queries/highlights.scm"), "")
        );
    }
}
