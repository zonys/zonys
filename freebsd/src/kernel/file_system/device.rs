use freebsd_sys::{
    devfs_rule, devfsio_rapply, int_DRA_BACTS, int_DRB_HIDE, int_DRB_UNHIDE, int_DRC_PATHPTRN,
    DEVFS_MAGIC,
};
use nix::errno::Errno;
use nix::fcntl::{open, OFlag};
use nix::mount::{unmount, MntFlags, Nmount, NmountError};
use nix::sys::stat::Mode;
use nix::unistd::close;
use std::cmp::min;
use std::ffi::CString;
use std::os::fd::RawFd;
use std::path::{Path, PathBuf};
use std::ptr::{addr_of_mut, copy_nonoverlapping};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct DeviceFileSystemRules<'a> {
    file_system: &'a DeviceFileSystem,
}

impl<'a> DeviceFileSystemRules<'a> {
    fn new(file_system: &'a DeviceFileSystem) -> Self {
        Self { file_system }
    }

    pub fn hide(&self, pattern: &str) -> Result<(), Errno> {
        let string = match CString::new(pattern) {
            Err(_e) => unreachable!(),
            Ok(string) => string,
        };

        let mut rule = devfs_rule {
            dr_magic: DEVFS_MAGIC,
            dr_id: 0,
            dr_icond: int_DRC_PATHPTRN,
            dr_dswflags: 0,
            dr_pathptrn: [0; 200],
            dr_iacts: int_DRA_BACTS,
            dr_bacts: int_DRB_HIDE,
            dr_uid: 0,
            dr_gid: 0,
            dr_mode: 0,
            dr_incset: 0,
        };

        unsafe {
            copy_nonoverlapping(
                string.as_ptr(),
                rule.dr_pathptrn.as_mut_ptr(),
                min(rule.dr_pathptrn.len(), string.as_bytes_with_nul().len()),
            )
        }

        let result = unsafe { devfsio_rapply(self.file_system.fd, addr_of_mut!(rule)) }?;

        if result > 0 {
            return Err(Errno::from_i32(result));
        }

        Ok(())
    }

    pub fn unhide(&self, pattern: &str) -> Result<(), Errno> {
        let string = match CString::new(pattern) {
            Err(_e) => unreachable!(),
            Ok(string) => string,
        };

        let mut rule = devfs_rule {
            dr_magic: DEVFS_MAGIC,
            dr_id: 0,
            dr_icond: int_DRC_PATHPTRN,
            dr_dswflags: 0,
            dr_pathptrn: [0; 200],
            dr_iacts: int_DRA_BACTS,
            dr_bacts: int_DRB_UNHIDE,
            dr_uid: 0,
            dr_gid: 0,
            dr_mode: 0,
            dr_incset: 0,
        };

        unsafe {
            copy_nonoverlapping(
                string.as_ptr(),
                rule.dr_pathptrn.as_mut_ptr(),
                min(rule.dr_pathptrn.len(), string.as_bytes_with_nul().len()),
            )
        }

        let result = unsafe { devfsio_rapply(self.file_system.fd, addr_of_mut!(rule)) }?;

        if result > 0 {
            return Err(Errno::from_i32(result));
        }

        Ok(())
    }

    pub fn unhide_all(&self) -> Result<(), Errno> {
        self.unhide("*")
    }

    pub fn hide_all(&self) -> Result<(), Errno> {
        self.hide("*")
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct DeviceFileSystem {
    closed: bool,
    fd: RawFd,
    path: PathBuf,
}

impl Drop for DeviceFileSystem {
    fn drop(&mut self) {
        if !self.closed {
            let _ = close(self.fd);
        }
    }
}

impl DeviceFileSystem {
    pub fn mount<T>(path: T) -> Result<(), NmountError>
    where
        T: AsRef<Path>,
    {
        Nmount::new()
            .str_opt_owned("fstype", "devfs")
            .str_opt_owned("fspath", path.as_ref())
            .nmount(MntFlags::empty())
    }

    pub fn unmount(self) -> Result<(), Errno> {
        unmount(&self.path, MntFlags::MNT_FORCE)
    }

    pub fn open<T>(path: T) -> Result<Self, Errno>
    where
        T: AsRef<Path>,
    {
        Ok(Self {
            fd: open(path.as_ref(), OFlag::O_RDONLY, Mode::empty())?,
            closed: false,
            path: PathBuf::from(path.as_ref()),
        })
    }

    pub fn close(mut self) -> Result<(), Errno> {
        self.closed = true;
        close(self.fd)?;

        Ok(())
    }

    pub fn rules(&self) -> DeviceFileSystemRules<'_> {
        DeviceFileSystemRules::new(self)
    }
}
