#[repr(C)]
pub enum zfs_type_t {
    Invalid = 0,
    FileSystem = 1 << 0,
    Snapshot = 1 << 1,
    Volume = 1 << 2,
    Pool = 1 << 3,
    Bookmark = 1 << 4,
    Vdev = 1 << 5,
}
