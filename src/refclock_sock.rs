#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type RCL_Instance_Record;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn recv(__fd: libc::c_int, __buf: *mut libc::c_void, __n: size_t,
            __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
    fn RCL_AddSample(instance: RCL_Instance, sample_time: *mut timespec,
                     offset: libc::c_double, leap: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn RCL_AddPulse(instance: RCL_Instance, pulse_time: *mut timespec,
                    second: libc::c_double) -> libc::c_int;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
    #[no_mangle]
    fn UTI_TimevalToTimespec(tv: *mut timeval, ts: *mut timespec);
    #[no_mangle]
    fn UTI_NormaliseTimespec(ts: *mut timespec);
    /* Register a handler for when select goes true on a file descriptor */
    #[no_mangle]
    fn SCH_AddFileHandler(fd: libc::c_int, events: libc::c_int,
                          handler: SCH_FileHandler,
                          arg: SCH_ArbitraryArgument);
    #[no_mangle]
    fn SCH_RemoveFileHandler(fd: libc::c_int);
    #[no_mangle]
    fn SCK_OpenUnixDatagramSocket(remote_addr: *const libc::c_char,
                                  local_addr: *const libc::c_char,
                                  flags: libc::c_int) -> libc::c_int;
    /* Remove bound Unix socket */
    #[no_mangle]
    fn SCK_RemoveSocket(sock_fd: libc::c_int) -> libc::c_int;
    /* Close the socket */
    #[no_mangle]
    fn SCK_CloseSocket(sock_fd: libc::c_int);
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2009
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 * 
 **********************************************************************

  =======================================================================

  Header file for refclocks.

  */
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
pub type SCH_ArbitraryArgument = *mut libc::c_void;
pub type SCH_FileHandler
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                _: SCH_ArbitraryArgument) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock_sample {
    pub tv: timeval,
    pub offset: libc::c_double,
    pub pulse: libc::c_int,
    pub leap: libc::c_int,
    pub _pad: libc::c_int,
    pub magic: libc::c_int,
}
unsafe extern "C" fn read_sample(mut sockfd: libc::c_int,
                                 mut event: libc::c_int,
                                 mut anything: *mut libc::c_void) {
    let mut sample: sock_sample =
        sock_sample{tv: timeval{tv_sec: 0, tv_usec: 0,},
                    offset: 0.,
                    pulse: 0,
                    leap: 0,
                    _pad: 0,
                    magic: 0,};
    let mut ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut instance: RCL_Instance = 0 as *mut RCL_Instance_Record;
    let mut s: libc::c_int = 0;
    instance = anything as RCL_Instance;
    s =
        recv(sockfd, &mut sample as *mut sock_sample as *mut libc::c_void,
             ::std::mem::size_of::<sock_sample>() as libc::c_ulong,
             0 as libc::c_int) as libc::c_int;
    if s < 0 as libc::c_int {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Could not read SOCK sample : %s\x00" as *const u8 as
                            *const libc::c_char,
                        strerror(*__errno_location()));
        }
        return
    }
    if s as libc::c_ulong !=
           ::std::mem::size_of::<sock_sample>() as libc::c_ulong {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Unexpected length of SOCK sample : %d != %ld\x00" as
                            *const u8 as *const libc::c_char, s,
                        ::std::mem::size_of::<sock_sample>() as libc::c_ulong
                            as libc::c_long);
        }
        return
    }
    if sample.magic != 0x534f434b as libc::c_int {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Unexpected magic number in SOCK sample : %x != %x\x00"
                            as *const u8 as *const libc::c_char,
                        sample.magic as libc::c_uint,
                        0x534f434b as libc::c_int as libc::c_uint);
        }
        return
    }
    UTI_TimevalToTimespec(&mut sample.tv, &mut ts);
    UTI_NormaliseTimespec(&mut ts);
    if sample.pulse != 0 {
        RCL_AddPulse(instance, &mut ts, sample.offset);
    } else { RCL_AddSample(instance, &mut ts, sample.offset, sample.leap); };
}
unsafe extern "C" fn sock_initialise(mut instance: RCL_Instance)
 -> libc::c_int {
    let mut sockfd: libc::c_int = 0;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    RCL_CheckDriverOptions(instance, 0 as *mut *const libc::c_char);
    path = RCL_GetDriverParameter(instance);
    sockfd =
        SCK_OpenUnixDatagramSocket(0 as *const libc::c_char, path,
                                   0 as libc::c_int);
    if sockfd < 0 as libc::c_int {
        LOG_Message(LOGS_FATAL,
                    b"Could not open socket %s\x00" as *const u8 as
                        *const libc::c_char, path);
        exit(1 as libc::c_int);
    }
    RCL_SetDriverData(instance, sockfd as libc::c_long as *mut libc::c_void);
    SCH_AddFileHandler(sockfd, 1 as libc::c_int,
                       Some(read_sample as
                                unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int,
                                                     _: *mut libc::c_void)
                                    -> ()),
                       instance as SCH_ArbitraryArgument);
    return 1 as libc::c_int;
}
unsafe extern "C" fn sock_finalise(mut instance: RCL_Instance) {
    let mut sockfd: libc::c_int = 0;
    sockfd = RCL_GetDriverData(instance) as libc::c_long as libc::c_int;
    SCH_RemoveFileHandler(sockfd);
    SCK_RemoveSocket(sockfd);
    SCK_CloseSocket(sockfd);
}
#[no_mangle]
pub static mut RCL_SOCK_driver: RefclockDriver =
    unsafe {
        {
            let mut init =
                RefclockDriver{init:
                                   Some(sock_initialise as
                                            unsafe extern "C" fn(_:
                                                                     RCL_Instance)
                                                -> libc::c_int),
                               fini:
                                   Some(sock_finalise as
                                            unsafe extern "C" fn(_:
                                                                     RCL_Instance)
                                                -> ()),
                               poll: None,};
            init
        }
    };
