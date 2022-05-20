use crate::file_system::FileSystem;
use crate::snapshot::Snapshot;
use std::vec::IntoIter;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct RootFileSystemIterator {
    iterator: IntoIter<FileSystem>,
}

impl RootFileSystemIterator {
    pub(super) fn new(iterator: IntoIter<FileSystem>) -> Self {
        Self { iterator }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new().into_iter())
    }
}

impl Iterator for RootFileSystemIterator {
    type Item = FileSystem;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

impl Default for RootFileSystemIterator {
    fn default() -> Self {
        Self::empty()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ChildFileSystemIterator {
    iterator: IntoIter<FileSystem>,
}

impl ChildFileSystemIterator {
    pub(super) fn new(iterator: IntoIter<FileSystem>) -> Self {
        Self { iterator }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new().into_iter())
    }
}

impl Iterator for ChildFileSystemIterator {
    type Item = FileSystem;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

impl Default for ChildFileSystemIterator {
    fn default() -> Self {
        Self::empty()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct AllFileSystemIterator {
    iterator: IntoIter<FileSystem>,
}

impl AllFileSystemIterator {
    pub(super) fn new(iterator: IntoIter<FileSystem>) -> Self {
        Self { iterator }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new().into_iter())
    }
}

impl Iterator for AllFileSystemIterator {
    type Item = FileSystem;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

impl Default for AllFileSystemIterator {
    fn default() -> Self {
        Self::empty()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct FileSystemSnapshotIterator {
    iterator: IntoIter<Snapshot>,
}

impl FileSystemSnapshotIterator {
    pub(super) fn new(iterator: IntoIter<Snapshot>) -> Self {
        Self { iterator }
    }
}

impl Iterator for FileSystemSnapshotIterator {
    type Item = Snapshot;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}
