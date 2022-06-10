use crate::r#extern::r#type::{libzfs_handle_t, mnttab};
use libc::{c_char, c_int};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[link(name = "zfs")]
extern "C" {
    pub fn libzfs_init() -> *mut libzfs_handle_t;
    pub fn libzfs_fini(param0: *mut libzfs_handle_t);
    pub fn libzfs_print_on_error(param0: *mut libzfs_handle_t, param1: bool);
    pub fn libzfs_errno(param0: *mut libzfs_handle_t) -> c_int;
    pub fn libzfs_error_init(param0: c_int) -> *const c_char;
    pub fn libzfs_error_action(param0: *mut libzfs_handle_t) -> *const c_char;
    pub fn libzfs_error_description(param0: *mut libzfs_handle_t) -> *const c_char;
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
    pub fn libzfs_free_str_array(param0: *mut *mut c_char, param1: c_int);
    pub fn libzfs_envvar_is_set(param0: *mut c_char) -> c_int;
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
    //pub fn libzfs_add_handle(param0: *mut get_all_cb_t, param1: param1: *mut zfs_handle_t);
}
