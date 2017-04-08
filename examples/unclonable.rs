extern crate double;

use std::{io, fmt};
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use double::Mock;

trait FileSystem: Clone {
    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> io::Result<()>;
}

fn copy_to_all<FS: FileSystem, P: AsRef<Path>>(fs: &FS, from: P, to: &[P]) -> Vec<io::Result<()>> {
    to.iter()
        .map(|path| fs.copy(&from, &path))
        .collect::<Vec<io::Result<()>>>()
}

#[derive(Debug, Clone)]
struct MockFileSystem<'a> {
    pub copy: Mock<(PathBuf, PathBuf), Result<(), CloneableError<'a>>>,
}

impl<'a> FileSystem for MockFileSystem<'a> {
    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> io::Result<()> {
        let args = (from.as_ref().to_path_buf(), to.as_ref().to_path_buf());
        self.copy
            .call(args)
            .map_err(|err| {
                let (kind, description) = (err.kind, err.description);
                io::Error::new(kind, description)
            })
    }
}

impl<'a> Default for MockFileSystem<'a> {
    fn default() -> Self {
        MockFileSystem { copy: Mock::new(Ok(())) }
    }
}

#[derive(Debug, Clone)]
struct CloneableError<'a> {
    pub kind: io::ErrorKind,
    pub description: &'a str,
}

impl<'a> Error for CloneableError<'a> {
    fn description(&self) -> &str {
        self.description
    }
}

impl<'a> Display for CloneableError<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

fn main() {
    // Initially, the `copy` mock returns `Ok()`
    let mock = MockFileSystem::default();

    let result = copy_to_all(&mock, "from", &["to"]);

    assert!(result.iter().all(|res| res.is_ok()));

    assert_eq!(mock.copy.num_calls(), 1);
    let expected_args = (Path::new("from").to_path_buf(), Path::new("to").to_path_buf());
    assert!(mock.copy.called_with(expected_args));

    let err = CloneableError {
        kind: ErrorKind::NotFound,
        description: "test",
    };
    mock.copy.return_err(err);

    let result = copy_to_all(&mock, "from", &["to"]);

    assert!(result.iter().all(|res| res.is_err()));
}
