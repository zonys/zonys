use libc::{c_char, c_int, c_void, size_t};
use nv_sys::r#extern::nvlist_t;

////////////////////////////////////////////////////////////////////////////////////////////////////

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
    pub type recvflags_t; // TODO
    pub type renameflags_t; // TODO
    pub type mnttab; // TODO
    pub type vdev_state_t; // TODO
    pub type vdev_aux_t; // TODO
    pub type pool_scan_func_t;
    pub type pool_scrub_cmd_t;
    pub type pool_initialize_func_t;
    pub type pool_trim_func_t;
    pub type trimflags_t;
}

pub type ZfsIterF = unsafe extern "C" fn(param0: *mut zfs_handle_t, param1: *mut c_void) -> c_int;
pub type ZpoolIterF =
    unsafe extern "C" fn(param0: *mut zpool_handle_t, param1: *mut c_void) -> c_int;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct sendflags_t {
    pub verbosity: c_int,
    pub replicate: bool,
    pub skipmissing: bool,
    pub doall: bool,
    pub fromorigin: bool,
    pub pad: bool,
    pub props: bool,
    pub dryrun: bool,
    pub parsable: bool,
    pub progress: bool,
    pub largeblock: bool,
    pub embed_data: bool,
    pub compress: bool,
    pub raw: bool,
    pub backup: bool,
    pub holds: bool,
    pub saved: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[link(name = "zfs")]
extern "C" {
    pub fn zpool_wait(param0: *mut zpool_handle_t, param1: zpool_wait_activity_t) -> c_int;
    pub fn zpool_wait_status(
        param0: *mut zpool_handle_t,
        param1: zpool_wait_activity_t,
        param2: *mut bool,
        param3: *mut bool,
    ) -> c_int;

    pub fn libzfs_init() -> *mut libzfs_handle_t;
    pub fn libzfs_fini(param0: *mut libzfs_handle_t);

    pub fn zpool_get_handle(param0: *mut zpool_handle_t) -> *mut libzfs_handle_t;
    pub fn zfs_get_handle(param0: *mut zfs_handle_t) -> *mut libzfs_handle_t;

    pub fn libzfs_print_on_error(param0: *mut libzfs_handle_t, param1: bool);

    pub fn zfs_save_arguments(
        param0: c_int,
        param1: *mut *mut c_char,
        param2: *mut c_char,
        param3: c_int,
    );
    pub fn zpool_log_history(param0: *mut libzfs_handle_t, param1: *const c_char) -> c_int;

    pub fn libzfs_errno(param0: *mut libzfs_handle_t) -> c_int;
    pub fn libzfs_error_init(param0: c_int) -> *const c_char;
    pub fn libzfs_error_action(param0: *mut libzfs_handle_t) -> *const c_char;
    pub fn libzfs_error_description(param0: *mut libzfs_handle_t) -> *const c_char;
    pub fn zfs_standard_error(param0: *mut libzfs_handle_t, param1: *const c_char) -> c_int;
    pub fn libzfs_mnttab_init(param0: *mut libzfs_handle_t);
    pub fn libzfs_mnttab_fini(param0: *mut libzfs_handle_t);
    pub fn libzfs_mnttab_cache(param0: *mut libzfs_handle_t, param2: bool);
    pub fn libzfs_mnttab_find(
        param0: *mut libzfs_handle_t,
        param1: *const c_char,
        param2: *mut mnttab,
    ) -> c_int;
    pub fn libzfs_mnttab_add(
        param0: *mut libzfs_handle_t,
        param1: *const c_char,
        param2: *const c_char,
        param3: *const c_char,
    );
    pub fn libzfs_mnttab_remove(param0: *mut libzfs_handle_t, param1: *const c_char);

    pub fn zpool_open(param0: *mut libzfs_handle_t, param1: *const c_char) -> *mut zpool_handle_t;
    pub fn zpool_open_calfail(
        param0: *mut libzfs_handle_t,
        param1: *const c_char,
    ) -> *mut zpool_handle_t;
    pub fn zpool_close(param0: *mut zpool_handle_t);
    pub fn zpool_get_name(param0: *mut zpool_handle_t) -> *const c_char;
    pub fn zpool_get_state(param0: *mut zpool_handle_t) -> c_int;
    pub fn zpool_state_to_name(param0: vdev_state_t, param1: vdev_aux_t) -> *const c_char;
    pub fn zpool_pool_state_to_name(param0: pool_state_t) -> *const c_char;
    pub fn zpool_free_handles(param0: *mut libzfs_handle_t);

    pub fn zpool_iter(
        param0: *mut libzfs_handle_t,
        param1: ZpoolIterF,
        param2: *mut c_void,
    ) -> c_int;
    pub fn zpool_skip_pool(param0: *const c_char) -> bool;

    pub fn zpool_create(
        param0: *mut libzfs_handle_t,
        param1: *const c_char,
        param2: *mut nvlist_t,
        param3: *mut nvlist_t,
        param4: *mut nvlist_t,
    ) -> c_int;
    pub fn zpool_destroy(param0: *mut zpool_handle_t, param1: *const c_char) -> c_int;
    pub fn zpool_add(param0: *mut zpool_handle_t, param1: *mut nvlist_t) -> c_int;

    pub fn zpool_scan(
        param0: *mut zpool_handle_t,
        param1: pool_scan_func_t,
        param2: pool_scrub_cmd_t,
    ) -> c_int;
    pub fn zpool_initialize(
        param0: *mut zpool_handle_t,
        param1: pool_initialize_func_t,
        param2: *mut nvlist_t,
    ) -> c_int;
    pub fn zpool_initialize_wait(
        param0: *mut zpool_handle_t,
        param1: pool_initialize_func_t,
        param2: *mut nvlist_t,
    ) -> c_int;
    pub fn zpool_trim(
        param0: *mut zpool_handle_t,
        param1: pool_trim_func_t,
        param2: *mut nvlist_t,
        param3: *mut trimflags_t,
    ) -> c_int;

    /*_LIBZFS_H int zpool_clear(zpool_handle_t *, const char *, nvlist_t *);
    _LIBZFS_H int zpool_reguid(zpool_handle_t *);
    _LIBZFS_H int zpool_reopen_one(zpool_handle_t *, void *);

    _LIBZFS_H int zpool_sync_one(zpool_handle_t *, void *);

    _LIBZFS_H int zpool_vdev_online(zpool_handle_t *, const char *, int,
        vdev_state_t *);
    _LIBZFS_H int zpool_vdev_offline(zpool_handle_t *, const char *, boolean_t);
    _LIBZFS_H int zpool_vdev_attach(zpool_handle_t *, const char *,
        const char *, nvlist_t *, int, boolean_t);
    _LIBZFS_H int zpool_vdev_detach(zpool_handle_t *, const char *);
    _LIBZFS_H int zpool_vdev_remove(zpool_handle_t *, const char *);
    _LIBZFS_H int zpool_vdev_remove_cancel(zpool_handle_t *);
    _LIBZFS_H int zpool_vdev_indirect_size(zpool_handle_t *, const char *,
        uint64_t *);
    _LIBZFS_H int zpool_vdev_split(zpool_handle_t *, char *, nvlist_t **,
        nvlist_t *, splitflags_t);

    _LIBZFS_H int zpool_vdev_fault(zpool_handle_t *, uint64_t, vdev_aux_t);
    _LIBZFS_H int zpool_vdev_degrade(zpool_handle_t *, uint64_t, vdev_aux_t);
    _LIBZFS_H int zpool_vdev_clear(zpool_handle_t *, uint64_t);

    _LIBZFS_H nvlist_t *zpool_find_vdev(zpool_handle_t *, const char *, boolean_t *,
        boolean_t *, boolean_t *);
    _LIBZFS_H nvlist_t *zpool_find_vdev_by_physpath(zpool_handle_t *, const char *,
        boolean_t *, boolean_t *, boolean_t *);
    _LIBZFS_H int zpool_label_disk(libzfs_handle_t *, zpool_handle_t *,
        const char *);
    _LIBZFS_H uint64_t zpool_vdev_path_to_guid(zpool_handle_t *zhp,
        const char *path);

    _LIBZFS_H const char *zpool_get_state_str(zpool_handle_t *);

    /*
     * Functions to manage pool properties
     */
    _LIBZFS_H int zpool_set_prop(zpool_handle_t *, const char *, const char *);
    _LIBZFS_H int zpool_get_prop(zpool_handle_t *, zpool_prop_t, char *,
        size_t proplen, zprop_source_t *, boolean_t literal);
    _LIBZFS_H uint64_t zpool_get_prop_int(zpool_handle_t *, zpool_prop_t,
        zprop_source_t *);
    _LIBZFS_H int zpool_props_refresh(zpool_handle_t *);

    _LIBZFS_H const char *zpool_prop_to_name(zpool_prop_t);
    _LIBZFS_H const char *zpool_prop_values(zpool_prop_t);

    /*
     * Functions to manage vdev properties
     */
    _LIBZFS_H int zpool_get_vdev_prop_value(nvlist_t *, vdev_prop_t, char *, char *,
        size_t, zprop_source_t *, boolean_t);
    _LIBZFS_H int zpool_get_vdev_prop(zpool_handle_t *, const char *, vdev_prop_t,
        char *, char *, size_t, zprop_source_t *, boolean_t);
    _LIBZFS_H int zpool_get_all_vdev_props(zpool_handle_t *, const char *,
        nvlist_t **);
    _LIBZFS_H int zpool_set_vdev_prop(zpool_handle_t *, const char *, const char *,
        const char *);

    _LIBZFS_H const char *vdev_prop_to_name(vdev_prop_t);
    _LIBZFS_H const char *vdev_prop_values(vdev_prop_t);
    _LIBZFS_H boolean_t vdev_prop_user(const char *name);
    _LIBZFS_H const char *vdev_prop_column_name(vdev_prop_t);
    _LIBZFS_H boolean_t vdev_prop_align_right(vdev_prop_t);


    _LIBZFS_H zpool_status_t zpool_get_status(zpool_handle_t *, char **,
        zpool_errata_t *);
    _LIBZFS_H zpool_status_t zpool_import_status(nvlist_t *, char **,
        zpool_errata_t *);

    /*
     * Statistics and configuration functions.
     */
    _LIBZFS_H nvlist_t *zpool_get_config(zpool_handle_t *, nvlist_t **);
    _LIBZFS_H nvlist_t *zpool_get_features(zpool_handle_t *);
    _LIBZFS_H int zpool_refresh_stats(zpool_handle_t *, boolean_t *);
    _LIBZFS_H int zpool_get_errlog(zpool_handle_t *, nvlist_t **);

    /*
     * Import and export functions
     */
    _LIBZFS_H int zpool_export(zpool_handle_t *, boolean_t, const char *);
    _LIBZFS_H int zpool_export_force(zpool_handle_t *, const char *);
    _LIBZFS_H int zpool_import(libzfs_handle_t *, nvlist_t *, const char *,
        char *altroot);
    _LIBZFS_H int zpool_import_props(libzfs_handle_t *, nvlist_t *, const char *,
        nvlist_t *, int);
    _LIBZFS_H void zpool_print_unsup_feat(nvlist_t *config);
    _LIBZFS_H char *zpool_vdev_name(libzfs_handle_t *, zpool_handle_t *, nvlist_t *,
        int name_flags);
    _LIBZFS_H int zpool_upgrade(zpool_handle_t *, uint64_t);
    _LIBZFS_H int zpool_get_history(zpool_handle_t *, nvlist_t **, uint64_t *,
        boolean_t *);
    _LIBZFS_H int zpool_events_next(libzfs_handle_t *, nvlist_t **, int *, unsigt);
    _LIBZFS_H int zpool_events_clear(libzfs_handle_t *, int *);
    _LIBZFS_H int zpool_events_seek(libzfs_handle_t *, uint64_t, int);
    _LIBZFS_H void zpool_obj_to_path_ds(zpool_handle_t *, uint64_t, uint64_t,
        char *, size_t);
    _LIBZFS_H void zpool_obj_to_path(zpool_handle_t *, uint64_t, uint64_t, char *,
        size_t);
    _LIBZFS_H int zfs_ioctl(libzfs_handle_t *, int, struct zfs_cmd *);
    _LIBZFS_H int zpool_get_physpath(zpool_handle_t *, char *, size_t);
    _LIBZFS_H void zpool_explain_recover(libzfs_handle_t *, const char *, int,
        nvlist_t *);
    */

    pub fn zpool_checkpoint(param0: *mut zpool_handle_t) -> c_int;
    pub fn zpool_discard_checkpoint(param0: *mut zpool_handle_t) -> c_int;
    pub fn zpool_is_draid_spare(param0: *const c_char) -> bool;

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
    //pub fn libzfs_add_handle(param0: *mut get_all_cb_t, param1: param1: *mut zfs_handle_t);

    pub fn zfs_iter_root(
        param0: *mut libzfs_handle_t,
        param1: ZfsIterF,
        param2: *mut c_void,
    ) -> c_int;
    pub fn zfs_iter_children(
        param0: *mut zfs_handle_t,
        param1: ZfsIterF,
        param2: *mut c_void,
    ) -> c_int;
    pub fn zfs_iter_dependants(
        param0: *mut zfs_handle_t,
        param1: bool,
        param2: ZfsIterF,
        param3: *mut c_void,
    ) -> c_int;
    pub fn zfs_iter_filesystems(
        param0: *mut zfs_handle_t,
        param1: ZfsIterF,
        param2: *mut c_void,
    ) -> c_int;
    pub fn zfs_iter_snapshots(
        param0: *mut zfs_handle_t,
        param1: bool,
        param2: ZfsIterF,
        param3: *mut c_void,
        param4: u64,
        param5: u64,
    ) -> c_int;
    pub fn zfs_iter_snapshots_sorted(
        param0: *mut zfs_handle_t,
        param1: bool,
        param2: ZfsIterF,
        param3: *mut c_void,
        param4: u64,
        param5: u64,
    ) -> c_int;
    pub fn zfs_iter_snapspec(
        param0: *mut zfs_handle_t,
        param1: *const c_char,
        param2: ZfsIterF,
        param3: *mut c_void,
    ) -> c_int;
    pub fn zfs_iter_bookmarks(
        param0: *mut zfs_handle_t,
        param1: ZfsIterF,
        param2: *mut c_void,
    ) -> c_int;
    pub fn zfs_iter_mounted(
        param0: *mut zfs_handle_t,
        param1: ZfsIterF,
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

    /*
    _LIBZFS_H int zfs_send(zfs_handle_t *, const char *, const char *,
    sendflags_t *, int, snapfilter_cb_t, void *, nvlist_t **);*/

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

    //pub fn zfs_receive(param0: *mut libzfs_handle_t)

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

    pub fn libzfs_run_process(
        param0: *const c_char,
        param1: *mut *mut c_char,
        param2: c_int,
    ) -> c_int;
    pub fn libzfs_run_process_get_stdout(
        param0: *const c_char,
        param1: *mut *mut c_char,
        param2: *mut *mut c_char,
        param3: *mut *mut *mut c_char,
        param4: *mut c_int,
    ) -> c_int;
    pub fn libzfs_run_process_get_stdout_nopath(
        param0: *const c_char,
        param1: *mut *mut c_char,
        param2: *mut *mut c_char,
        param3: *mut *mut *mut c_char,
        param4: *mut c_int,
    ) -> c_int;

    pub fn libzfs_free_str_array(param0: *mut *mut c_char, param1: c_int);

    pub fn libzfs_envvar_is_set(param0: *mut c_char) -> c_int;

    pub fn zfs_version_userland(param0: *mut c_char, param1: c_int) -> c_int;
    pub fn zfs_version_kernel(param0: *mut c_char, param1: c_int) -> c_int;
    pub fn zfs_version_print() -> c_int;

    pub fn zpool_in_use(
        param0: *mut libzfs_handle_t,
        param1: c_int,
        param2: *mut pool_state_t,
        param3: *mut *mut c_char,
        param4: *mut bool,
    ) -> c_int;

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

    pub fn zpool_enable_datasets(
        param0: *mut zpool_handle_t,
        param1: *const c_char,
        param2: c_int,
    ) -> c_int;
    pub fn zpool_disable_datasets(param0: *mut zpool_handle_t, param1: bool) -> c_int;
    pub fn zpool_disable_datasets_os(param0: *mut zpool_handle_t, param1: bool);
    pub fn zpool_disable_volume_os(param0: *const c_char);

    pub fn zpool_load_compat(
        param0: *const c_char,
        param1: *mut bool,
        param2: *mut c_char,
        param3: size_t,
    ) -> zpool_compat_status_t;

    pub fn zfs_jail(zhp: *mut zfs_handle_t, jailid: c_int, attach: c_int) -> c_int;

    pub fn zpool_nextboot(
        param0: *mut libzfs_handle_t,
        param1: u64,
        param2: u64,
        param3: *const c_char,
    ) -> c_int;
}
