use crate::devfs_rule;
use nix::{ioctl_readwrite, ioctl_write_int, ioctl_write_ptr};

////////////////////////////////////////////////////////////////////////////////////////////////////

ioctl_readwrite!(devfsio_radd, 'D', 0, devfs_rule);
ioctl_write_int!(devfsio_rdel, 'D', 1);
ioctl_write_ptr!(devfsio_rapply, 'D', 2, devfs_rule);
ioctl_write_int!(devfsio_rapplyid, 'D', 3);
ioctl_write_ptr!(devfsio_rgetnext, 'D', 4, devfs_rule);

ioctl_write_int!(devfsio_suse, 'D', 10);
ioctl_write_int!(devfsio_sapply, 'D', 11);
ioctl_write_int!(devfsio_sgetnext, 'D', 12);
