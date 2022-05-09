use libc::{c_char, c_int, c_uint, c_void, size_t};

////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct jail {
    version: u32,
    path: *mut c_char,
    hostname: *mut c_char,
    jailname: *mut c_char,
    ip4s: u32,
    ip6s: u32,
    ip4: *mut c_void,
    ip6: *mut c_void,
}

////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" {
    pub fn jail_attach(jid: c_int) -> c_int;
    pub fn jail_remove(jid: c_int) -> c_int;
}

////////////////////////////////////////////////////////////////////////////////////////////////

pub const JAIL_CREATE: c_int = 0x01;
pub const JAIL_UPDATE: c_int = 0x02;
pub const JAIL_ATTACH: c_int = 0x04;
pub const JAIL_DYING: c_int = 0x08;
pub const JAIL_SET_MASK: c_int = 0x0F;
pub const JAIL_GET_MASK: c_int = 0x08;

pub const JAIL_SYS_DISABLE: c_int = 0;
pub const JAIL_SYS_NEW: c_int = 1;
pub const JAIL_SYS_INHERIT: c_int = 2;

////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct jailparam {
    pub jp_name: *mut c_char,
    pub jp_value: *mut c_void,
    pub jp_valuelen: size_t,
    pub jp_elemlen: size_t,
    pub jp_ctltype: c_int,
    pub jp_structtype: c_int,
    pub jp_flags: c_uint,
}

////////////////////////////////////////////////////////////////////////////////////////////////

#[link(name = "jail")]
extern "C" {
    pub fn jail_getid(name: *const c_char) -> c_int;
    pub fn jail_getname(jid: c_int) -> *mut c_char;

    pub fn jail_setv(flags: c_int, ...) -> c_int;
    pub fn jail_getv(flags: c_int, ...) -> c_int;

    pub fn jailparam_all(jpp: *mut *mut jailparam) -> c_int;
    pub fn jailparam_init(jp: *mut jailparam, name: *const c_char) -> c_int;
    pub fn jailparam_import(jp: *mut jailparam, name: *const c_char) -> c_int;
    pub fn jailparam_import_raw(jp: *mut jailparam, value: *mut c_void, valuelen: size_t) -> c_int;
    pub fn jailparam_set(jp: *mut jailparam, njp: c_uint, flags: c_int) -> c_int;
    pub fn jailparam_get(jp: *mut jailparam, njp: c_uint, flags: c_int) -> c_int;
    pub fn jailparam_export(jp: *mut jailparam) -> *mut c_char;
    pub fn jailparam_free(jp: *mut jailparam, njp: c_uint);
}

////////////////////////////////////////////////////////////////////////////////////////////////

pub const JP_RAWVALUE: c_uint = 0x01;
pub const JP_BOOL: c_uint = 0x02;
pub const JP_NOBOOL: c_uint = 0x04;
pub const JP_JAILSYS: c_uint = 0x08;
