#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type RCL_Instance_Record;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn shmget(__key: key_t, __size: size_t, __shmflg: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn shmat(__shmid: libc::c_int, __shmaddr: *const libc::c_void,
             __shmflg: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn shmdt(__shmaddr: *const libc::c_void) -> libc::c_int;
    /* functions used by drivers */
    #[no_mangle]
    fn RCL_SetDriverData(instance: RCL_Instance, data: *mut libc::c_void);
    #[no_mangle]
    fn RCL_GetDriverData(instance: RCL_Instance) -> *mut libc::c_void;
    #[no_mangle]
    fn RCL_GetDriverParameter(instance: RCL_Instance) -> *mut libc::c_char;
    #[no_mangle]
    fn RCL_CheckDriverOptions(instance: RCL_Instance,
                              options: *mut *const libc::c_char);
    #[no_mangle]
    fn RCL_GetDriverOption(instance: RCL_Instance, name: *mut libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn RCL_AddSample(instance: RCL_Instance, sample_time: *mut timespec,
                     offset: libc::c_double, leap: libc::c_int)
     -> libc::c_int;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
    /* Normalise a timespec, by adding or subtracting seconds to bring
   its nanosecond field into range */
    #[no_mangle]
    fn UTI_NormaliseTimespec(ts: *mut timespec);
    /* Calculate result = a - b and return as a double */
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec)
     -> libc::c_double;
}
pub type __time_t = libc::c_long;
pub type __key_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type key_t = __key_t;
pub type time_t = __time_t;
pub type RCL_Instance = *mut RCL_Instance_Record;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefclockDriver {
    pub init: Option<unsafe extern "C" fn(_: RCL_Instance) -> libc::c_int>,
    pub fini: Option<unsafe extern "C" fn(_: RCL_Instance) -> ()>,
    pub poll: Option<unsafe extern "C" fn(_: RCL_Instance) -> libc::c_int>,
}
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shmTime {
    pub mode: libc::c_int,
    pub count: libc::c_int,
    pub clockTimeStampSec: time_t,
    pub clockTimeStampUSec: libc::c_int,
    pub receiveTimeStampSec: time_t,
    pub receiveTimeStampUSec: libc::c_int,
    pub leap: libc::c_int,
    pub precision: libc::c_int,
    pub nsamples: libc::c_int,
    pub valid: libc::c_int,
    pub clockTimeStampNSec: libc::c_int,
    pub receiveTimeStampNSec: libc::c_int,
    pub dummy: [libc::c_int; 8],
}
unsafe extern "C" fn shm_initialise(mut instance: RCL_Instance)
 -> libc::c_int {
    let mut options: [*const libc::c_char; 2] =
        [b"perm\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char];
    let mut id: libc::c_int = 0;
    let mut param: libc::c_int = 0;
    let mut perm: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shm: *mut shmTime = 0 as *mut shmTime;
    RCL_CheckDriverOptions(instance, options.as_mut_ptr());
    param = atoi(RCL_GetDriverParameter(instance));
    s =
        RCL_GetDriverOption(instance,
                            b"perm\x00" as *const u8 as *const libc::c_char as
                                *mut libc::c_char);
    perm =
        if !s.is_null() {
            (strtol(s, 0 as *mut *mut libc::c_char, 8 as libc::c_int)) &
                0o777 as libc::c_int as libc::c_long
        } else { 0o600 as libc::c_int as libc::c_long } as libc::c_int;
    id =
        shmget(0x4e545030 as libc::c_int + param,
               ::std::mem::size_of::<shmTime>() as libc::c_ulong,
               0o1000 as libc::c_int | perm);
    if id == -(1 as libc::c_int) {
        LOG_Message(LOGS_FATAL,
                    b"shmget() failed : %s\x00" as *const u8 as
                        *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    shm =
        shmat(id, 0 as *const libc::c_void, 0 as libc::c_int) as *mut shmTime;
    if shm as libc::c_long == -(1 as libc::c_int) as libc::c_long {
        LOG_Message(LOGS_FATAL,
                    b"shmat() failed : %s\x00" as *const u8 as
                        *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    }
    RCL_SetDriverData(instance, shm as *mut libc::c_void);
    return 1 as libc::c_int;
}
unsafe extern "C" fn shm_finalise(mut instance: RCL_Instance) {
    shmdt(RCL_GetDriverData(instance));
}
unsafe extern "C" fn shm_poll(mut instance: RCL_Instance) -> libc::c_int {
    let mut receive_ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut clock_ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut t: shmTime =
        shmTime{mode: 0,
                count: 0,
                clockTimeStampSec: 0,
                clockTimeStampUSec: 0,
                receiveTimeStampSec: 0,
                receiveTimeStampUSec: 0,
                leap: 0,
                precision: 0,
                nsamples: 0,
                valid: 0,
                clockTimeStampNSec: 0,
                receiveTimeStampNSec: 0,
                dummy: [0; 8],};
    let mut shm: *mut shmTime = 0 as *mut shmTime;
    let mut offset: libc::c_double = 0.;
    shm = RCL_GetDriverData(instance) as *mut shmTime;
    t = *shm;
    if t.mode == 1 as libc::c_int && t.count != (*shm).count ||
           !(t.mode == 0 as libc::c_int || t.mode == 1 as libc::c_int) ||
           t.valid == 0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"SHM sample ignored mode=%d count=%d valid=%d\x00" as
                            *const u8 as *const libc::c_char, t.mode, t.count,
                        t.valid);
        }
        return 0 as libc::c_int
    }
    ::std::ptr::write_volatile(&mut (*shm).valid as *mut libc::c_int,
                               0 as libc::c_int);
    receive_ts.tv_sec = t.receiveTimeStampSec;
    clock_ts.tv_sec = t.clockTimeStampSec;
    if t.clockTimeStampNSec / 1000 as libc::c_int == t.clockTimeStampUSec &&
           t.receiveTimeStampNSec / 1000 as libc::c_int ==
               t.receiveTimeStampUSec {
        receive_ts.tv_nsec = t.receiveTimeStampNSec as __syscall_slong_t;
        clock_ts.tv_nsec = t.clockTimeStampNSec as __syscall_slong_t
    } else {
        receive_ts.tv_nsec =
            (1000 as libc::c_int * t.receiveTimeStampUSec) as
                __syscall_slong_t;
        clock_ts.tv_nsec =
            (1000 as libc::c_int * t.clockTimeStampUSec) as __syscall_slong_t
    }
    UTI_NormaliseTimespec(&mut clock_ts);
    UTI_NormaliseTimespec(&mut receive_ts);
    offset = UTI_DiffTimespecsToDouble(&mut clock_ts, &mut receive_ts);
    return RCL_AddSample(instance, &mut receive_ts, offset, t.leap);
}
#[no_mangle]
pub static mut RCL_SHM_driver: RefclockDriver =
    unsafe {
        {
            let mut init =
                RefclockDriver{init:
                                   Some(shm_initialise as
                                            unsafe extern "C" fn(_:
                                                                     RCL_Instance)
                                                -> libc::c_int),
                               fini:
                                   Some(shm_finalise as
                                            unsafe extern "C" fn(_:
                                                                     RCL_Instance)
                                                -> ()),
                               poll:
                                   Some(shm_poll as
                                            unsafe extern "C" fn(_:
                                                                     RCL_Instance)
                                                -> libc::c_int),};
            init
        }
    };
