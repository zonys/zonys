use libc::{c_int, c_void};

////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" {
    pub type zfs_handle_t;
    pub type zpool_handle_t;
    pub type libzfs_handle_t;

    pub type pool_state_t; // TODO
    pub type zpool_compat_status_t; // TODO
    pub type zfs_share_op_t; // TODO
    pub type zpool_wait_activity_t; // TODO
    pub type diff_flags_t; // TODO
    pub type renameflags_t; // TODO
    pub type mnttab; // TODO
    pub type vdev_state_t; // TODO
    pub type vdev_aux_t; // TODO
    pub type pool_scan_func_t;
    pub type pool_scrub_cmd_t;
    pub type pool_initialize_func_t;
    pub type pool_trim_func_t;
    pub type trimflags_t;
    pub type avl_tree_t;
}

#[allow(non_camel_case_types)]
pub type boolean_t = c_int;

#[allow(non_camel_case_types)]
pub type zfs_iter_f = unsafe extern "C" fn(param0: *mut zfs_handle_t, param1: *mut c_void) -> c_int;

#[allow(non_camel_case_types)]
pub type zpool_iter_f =
    unsafe extern "C" fn(param0: *mut zpool_handle_t, param2: *mut c_void) -> c_int;

#[allow(non_camel_case_types)]
pub type snapfilter_cb_t =
    unsafe extern "C" fn(param0: *mut zfs_handle_t, param1: *mut c_void) -> bool;
