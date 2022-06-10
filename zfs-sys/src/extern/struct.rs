use crate::r#extern::r#type::boolean_t;
use libc::c_int;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct sendflags_t {
    pub verbosity: c_int,
    pub replicate: boolean_t,
    pub skipmissing: boolean_t,
    pub doall: boolean_t,
    pub fromorigin: boolean_t,
    pub pad: boolean_t,
    pub props: boolean_t,
    pub dryrun: boolean_t,
    pub parsable: boolean_t,
    pub progress: boolean_t,
    pub largeblock: boolean_t,
    pub embed_data: boolean_t,
    pub compress: boolean_t,
    pub raw: boolean_t,
    pub backup: boolean_t,
    pub holds: boolean_t,
    pub saved: boolean_t,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct recvflags_t {
    pub verbose: boolean_t,
    pub isprefix: boolean_t,
    pub istail: boolean_t,
    pub dryrun: boolean_t,
    pub force: boolean_t,
    pub canmountoff: boolean_t,
    pub resumable: boolean_t,
    pub byteswap: boolean_t,
    pub nomount: boolean_t,
    pub holds: boolean_t,
    pub skipholds: boolean_t,
    pub domount: boolean_t,
    pub forceunmount: boolean_t,
}
