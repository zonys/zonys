use crate::r#extern::r#enum::zfs_type_t;
use crate::r#extern::r#struct::{recvflags_t, sendflags_t};
use crate::r#extern::r#type::{
    avl_tree_t, libzfs_handle_t, renameflags_t, snapfilter_cb_t, zfs_handle_t, zfs_iter_f,
    zfs_share_op_t, zpool_handle_t,
};
use libc::{c_char, c_int, c_void, size_t};
use nv_sys::r#extern::nvlist_t;

////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////

#[link(name = "zfs")]
extern "C" {
    pub fn zfs_get_handle(param0: *mut zfs_handle_t) -> *mut libzfs_handle_t;
    pub fn zfs_save_arguments(
        param0: c_int,
        param1: *mut *mut c_char,
        param2: *mut c_char,
        param3: c_int,
    );
    pub fn zfs_standard_error(param0: *mut libzfs_handle_t, param1: *const c_char) -> c_int;
    pub fn zfs_open(
        param0: *mut libzfs_handle_t,
        param1: *const c_char,
        param2: zfs_type_t,
    ) -> *mut zfs_handle_t;
    pub fn zfs_handle_dup(handle: *mut zfs_handle_t) -> *mut zfs_handle_t;
    pub fn zfs_close(handle: *mut zfs_handle_t);
    pub fn zfs_get_type(handle: *const zfs_handle_t) -> c_int;
    pub fn zfs_get_underlying_type(handle: *const zfs_handle_t) -> c_int;
    pub fn zfs_get_name(handle: *const zfs_handle_t) -> *const c_char;
    pub fn zfs_get_pool_handle(handle: *const zfs_handle_t) -> *mut zpool_handle_t;
    pub fn zfs_get_pool_name(handle: *const zfs_handle_t) -> *const c_char;

    ////////////////////////////////////////////////////////////////////////////////////////////////

    /*pub fn zpop_print_one_property(
        param0: *const c_char,
        param1: *mut zpop_get_cbdata_t,
        param2: *const c_char,
        param3: *const c_char,
        param4: zprop_source_t,
        param5: *const c_char,
        param6: *const c_char,
    );*/

    /*pub fn zfs_foreach_mountpoint(
        param0: *mut libzfs_handle_t,
        param1: *mut *mut zfs_handle_t,
        param2: size_t,
        param3: ZfsIterF,
        param4: *mut c_void,
        param5: bool,
    );*/

    pub fn zfs_iter_root(
        param0: *mut libzfs_handle_t,
        param1: zfs_iter_f,
        param2: *mut c_void,
    ) -> c_int;
    pub fn zfs_iter_children(
        param0: *mut zfs_handle_t,
        param1: zfs_iter_f,
        param2: *mut c_void,
    ) -> c_int;
    pub fn zfs_iter_dependants(
        param0: *mut zfs_handle_t,
        param1: bool,
        param2: zfs_iter_f,
        param3: *mut c_void,
    ) -> c_int;
    pub fn zfs_iter_filesystems(
        param0: *mut zfs_handle_t,
        param1: zfs_iter_f,
        param2: *mut c_void,
    ) -> c_int;
    pub fn zfs_iter_snapshots(
        param0: *mut zfs_handle_t,
        param1: bool,
        param2: zfs_iter_f,
        param3: *mut c_void,
        param4: u64,
        param5: u64,
    ) -> c_int;
    pub fn zfs_iter_snapshots_sorted(
        param0: *mut zfs_handle_t,
        param1: bool,
        param2: zfs_iter_f,
        param3: *mut c_void,
        param4: u64,
        param5: u64,
    ) -> c_int;
    pub fn zfs_iter_snapspec(
        param0: *mut zfs_handle_t,
        param1: *const c_char,
        param2: zfs_iter_f,
        param3: *mut c_void,
    ) -> c_int;
    pub fn zfs_iter_bookmarks(
        param0: *mut zfs_handle_t,
        param1: zfs_iter_f,
        param2: *mut c_void,
    ) -> c_int;
    pub fn zfs_iter_mounted(
        param0: *mut zfs_handle_t,
        param1: zfs_iter_f,
        param2: *mut c_void,
    ) -> c_int;

    pub fn zfs_create(
        handle: *mut libzfs_handle_t,
        param0: *const c_char,
        param1: zfs_type_t,
        param2: *mut nvlist_t,
    ) -> c_int;
    pub fn zfs_destroy(param0: *mut zfs_handle_t, param1: bool) -> c_int;
    pub fn zfs_destroy_snaps(
        param0: *mut zfs_handle_t,
        param1: *const c_char,
        param2: bool,
    ) -> c_int;
    pub fn zfs_destroy_snaps_nvl(
        param0: *mut zfs_handle_t,
        param1: *mut nvlist_t,
        param2: bool,
    ) -> c_int;
    pub fn zfs_destroy_snaps_nvl_os(param0: *mut zfs_handle_t, param1: *mut nvlist_t) -> c_int;
    pub fn zfs_clone(
        param0: *mut zfs_handle_t,
        param1: *const c_char,
        param2: *mut nvlist_t,
    ) -> c_int;
    pub fn zfs_snapshot(
        param0: *mut libzfs_handle_t,
        param1: *const c_char,
        param2: bool,
        param3: *mut nvlist_t,
    ) -> c_int;
    pub fn zfs_snapshot_nvl(
        hdl: *mut libzfs_handle_t,
        snaps: *mut nvlist_t,
        props: *mut nvlist_t,
    ) -> c_int;
    pub fn zfs_rollback(
        param0: *mut zfs_handle_t,
        param1: *mut zfs_handle_t,
        param2: bool,
    ) -> c_int;

    pub fn zfs_rename(
        param0: *mut zfs_handle_t,
        param2: *const c_char,
        flags: renameflags_t,
    ) -> c_int;

    pub fn zfs_send(
        zhp: *mut zfs_handle_t,
        fromsnap: *const c_char,
        tosnap: *const c_char,
        flags: *mut sendflags_t,
        outfd: c_int,
        filter_func: snapfilter_cb_t,
        cb_arg: *mut c_void,
        debugnvp: *mut *mut nvlist_t,
    ) -> c_int;

    pub fn zfs_send_one(
        param0: *mut zfs_handle_t,
        param1: *const c_char,
        param2: c_int,
        param3: *mut sendflags_t,
        param4: *const c_char,
    ) -> c_int;

    /*_LIBZFS_H int zfs_send_progress(zfs_handle_t *, int, uint64_t *, uint64_t *);
    _LIBZFS_H int zfs_send_resume(libzfs_handle_t *, sendflags_t *, int outfd,
    const char *);
    _LIBZFS_H int zfs_send_saved(zfs_handle_t *, sendflags_t *, int, const char *);
    _LIBZFS_H nvlist_t *zfs_send_resume_token_to_nvlist(libzfs_handle_t *hdl,
    const char *token);

    _LIBZFS_H int zfs_promote(zfs_handle_t *);
    _LIBZFS_H int zfs_hold(zfs_handle_t *, const char *, const char *,
    boolean_t, int);
    _LIBZFS_H int zfs_hold_nvl(zfs_handle_t *, int, nvlist_t *);
    _LIBZFS_H int zfs_release(zfs_handle_t *, const char *, const char *,
    boolean_t);
    _LIBZFS_H int zfs_get_holds(zfs_handle_t *, nvlist_t **);
    _LIBZFS_H uint64_t zvol_volsize_to_reservation(zpool_handle_t *, uint64_t,
    nvlist_t *);

    typedef int (*zfs_userspace_cb_t)(void *arg, const char *domain,
    uid_t rid, uint64_t space);

    _LIBZFS_H int zfs_userspace(zfs_handle_t *, zfs_userquota_prop_t,
    zfs_userspace_cb_t, void *);

    _LIBZFS_H int zfs_get_fsacl(zfs_handle_t *, nvlist_t **);
    _LIBZFS_H int zfs_set_fsacl(zfs_handle_t *, boolean_t, nvlist_t *);
    */

    /*_LIBZFS_H int zfs_receive(libzfs_handle_t *, const char *, nvlist_t *,
    recvflags_t *, int, avl_tree_t *);*/

    pub fn zfs_receive(
        param0: *mut libzfs_handle_t,
        param1: *const c_char,
        param2: *mut nvlist_t,
        param3: *mut recvflags_t,
        param4: c_int,
        param5: *mut avl_tree_t,
    ) -> c_int;

    pub fn zfs_show_diffs(
        param0: *mut zfs_handle_t,
        param1: c_int,
        param2: *const c_char,
        param3: *const c_char,
        param4: c_int,
    ) -> c_int;

    pub fn zfs_type_to_name(param0: zfs_type_t) -> *const c_char;
    pub fn zfs_refresh_properties(param0: *mut zfs_handle_t);
    pub fn zfs_name_valid(param0: *const c_char, param1: zfs_type_t) -> c_int;
    pub fn zfs_path_to_zhandle(
        param0: *mut libzfs_handle_t,
        param1: *const c_char,
        param2: zfs_type_t,
    ) -> *mut zfs_handle_t;
    pub fn zfs_parent_name(param0: *mut zfs_handle_t, param1: *mut c_char, param2: size_t)
        -> c_int;
    pub fn zfs_dataset_exists(
        param0: *mut libzfs_handle_t,
        param1: *const c_char,
        param2: zfs_type_t,
    ) -> bool;
    pub fn zfs_spa_version(param0: *mut zfs_handle_t, param1: *mut c_int) -> c_int;
    pub fn zfs_bookmark_exists(path: *const c_char) -> bool;

    pub fn is_mounted(
        param0: *mut libzfs_handle_t,
        special: *const c_char,
        param2: *mut *mut c_char,
    ) -> bool;
    pub fn zfs_is_mounted(param0: *mut zfs_handle_t, param1: *mut *mut c_char) -> bool;
    pub fn zfs_mount(param0: *mut zfs_handle_t, param1: *const c_char, param2: c_int) -> c_int;
    pub fn zfs_mount_at(
        param0: *mut zfs_handle_t,
        param1: *const c_char,
        param2: c_int,
        param3: *const c_char,
    ) -> c_int;
    pub fn zfs_unmount(param0: *mut zfs_handle_t, param1: *const c_char, param2: c_int) -> c_int;
    pub fn zfs_unmountall(param0: *mut zfs_handle_t, param1: c_int) -> c_int;
    pub fn zfs_mount_delegation_check() -> c_int;

    pub fn zfs_is_shared(param0: *mut zfs_handle_t) -> bool;
    pub fn zfs_share(param0: *mut zfs_handle_t) -> c_int;
    pub fn zfs_unshare(param1: *mut zfs_handle_t) -> c_int;

    pub fn zfs_is_shared_nfs(param0: *mut zfs_handle_t, param1: *mut *mut c_char) -> bool;
    pub fn zfs_is_shared_smb(param0: *mut zfs_handle_t, param1: *mut *mut c_char) -> bool;
    pub fn zfs_share_nfs(param0: *mut zfs_handle_t) -> c_int;
    pub fn zfs_share_smb(param0: *mut zfs_handle_t) -> c_int;
    pub fn zfs_shareall(param0: *mut zfs_handle_t) -> c_int;
    pub fn zfs_unshare_nfs(param0: *mut zfs_handle_t, param1: *const c_char) -> c_int;
    pub fn zfs_unshare_smb(param0: *mut zfs_handle_t, param1: *const c_char) -> c_int;
    pub fn zfs_unshareall_nfs(param0: *mut zfs_handle_t) -> c_int;
    pub fn zfs_unshareall_smb(param0: *mut zfs_handle_t) -> c_int;
    pub fn zfs_unshareall_bypath(param0: *mut zfs_handle_t, param1: *const c_char) -> c_int;
    pub fn zfs_unshareall_bytype(
        param0: *mut zfs_handle_t,
        param1: *const c_char,
        param2: *const c_char,
    ) -> c_int;
    pub fn zfs_unshareall(param0: *mut zfs_handle_t) -> c_int;
    pub fn zfs_deleg_share_nfs(
        param0: *mut libzfs_handle_t,
        param1: *mut c_char,
        param2: *mut c_char,
        param3: *mut c_char,
        param4: *mut c_void,
        param5: *mut c_void,
        param6: c_int,
        param7: zfs_share_op_t,
    ) -> c_int;
    pub fn zfs_commit_nfs_shares();
    pub fn zfs_commit_smb_shares();
    pub fn zfs_commit_all_shares();
    pub fn zfs_commit_shares(param0: *const c_char);

    pub fn zfs_nicestrtonum(
        param0: *mut libzfs_handle_t,
        param1: *const c_char,
        param2: *mut u64,
    ) -> c_int;

    pub fn zfs_version_userland(param0: *mut c_char, param1: c_int) -> c_int;
    pub fn zfs_version_kernel(param0: *mut c_char, param1: c_int) -> c_int;
    pub fn zfs_version_print() -> c_int;

    pub fn zfs_smb_acl_add(
        param0: *mut libzfs_handle_t,
        param1: *mut c_char,
        param2: *mut c_char,
        param3: *mut c_char,
    ) -> c_int;
    pub fn zfs_smb_acl_remove(
        param0: *mut libzfs_handle_t,
        param1: *mut c_char,
        param2: *mut c_char,
        param3: *mut c_char,
    ) -> c_int;
    pub fn zfs_smb_acl_purge(
        param0: *mut libzfs_handle_t,
        param1: *mut c_char,
        param2: *mut c_char,
    ) -> c_int;
    pub fn zfs_smb_acl_rename(
        param0: *mut libzfs_handle_t,
        param1: *mut c_char,
        param2: *mut c_char,
        param3: *mut c_char,
        param4: *mut c_char,
    ) -> c_int;
    pub fn zfs_jail(zhp: *mut zfs_handle_t, jailid: c_int, attach: c_int) -> c_int;
}
