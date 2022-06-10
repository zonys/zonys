use crate::r#extern::r#type::{
    libzfs_handle_t, pool_initialize_func_t, pool_scan_func_t, pool_scrub_cmd_t, pool_state_t,
    pool_trim_func_t, trimflags_t, vdev_aux_t, vdev_state_t, zpool_compat_status_t, zpool_handle_t,
    zpool_iter_f, zpool_wait_activity_t,
};
use libc::{c_char, c_int, c_void, size_t};
use nv_sys::r#extern::nvlist_t;

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
    pub fn zpool_log_history(param0: *mut libzfs_handle_t, param1: *const c_char) -> c_int;
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
        param1: zpool_iter_f,
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
    pub fn zpool_in_use(
        param0: *mut libzfs_handle_t,
        param1: c_int,
        param2: *mut pool_state_t,
        param3: *mut *mut c_char,
        param4: *mut bool,
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
    pub fn zpool_nextboot(
        param0: *mut libzfs_handle_t,
        param1: u64,
        param2: u64,
        param3: *const c_char,
    ) -> c_int;

}
