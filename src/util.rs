#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           ptr_wrapping_offset_from, register_tool)]

use log::debug;
use anyhow::{Context, Result};
use rand::{
    RngCore,
    rngs::OsRng,
};

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn log(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn setgroups(__n: size_t, __groups: *const __gid_t) -> libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    #[no_mangle]
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    #[no_mangle]
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    #[no_mangle]
    fn htons(__hostshort: uint16_t) -> uint16_t;
    #[no_mangle]
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                 __oact: *mut sigaction) -> libc::c_int;
    #[no_mangle]
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn gmtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn chown(__file: *const libc::c_char, __owner: __uid_t, __group: __gid_t)
     -> libc::c_int;
    #[no_mangle]
    fn setuid(__uid: __uid_t) -> libc::c_int;
    #[no_mangle]
    fn setgid(__gid: __gid_t) -> libc::c_int;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn inet_pton(__af: libc::c_int, __cp: *const libc::c_char,
                 __buf: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn inet_ntop(__af: libc::c_int, __cp: *const libc::c_void,
                 __buf: *mut libc::c_char, __len: socklen_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn getrandom(__buffer: *mut libc::c_void, __length: size_t,
                 __flags: libc::c_uint) -> ssize_t;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
    /*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2002
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

  Header file for memory functions
  */
    /* Wrappers checking for errors */
    #[no_mangle]
    fn Malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn Strdup(s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn HSH_GetHashId(name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn HSH_Hash(id: libc::c_int, in1: *const libc::c_uchar,
                in1_len: libc::c_uint, in2: *const libc::c_uchar,
                in2_len: libc::c_uint, out: *mut libc::c_uchar,
                out_len: libc::c_uint) -> libc::c_uint;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type gid_t = __gid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uid_t = __uid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_10,
    pub _timer: C2RustUnnamed_9,
    pub _rt: C2RustUnnamed_8,
    pub _sigchld: C2RustUnnamed_7,
    pub _sigfault: C2RustUnnamed_4,
    pub _sigpoll: C2RustUnnamed_3,
    pub _sigsys: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub _addr_bnd: C2RustUnnamed_6,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_11,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut siginfo_t,
                                                  _: *mut libc::c_void)
                                 -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPAddr {
    pub addr: C2RustUnnamed_12,
    pub family: uint16_t,
    pub _pad: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub in4: uint32_t,
    pub in6: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPSockAddr {
    pub ip_addr: IPAddr,
    pub port: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_int64 {
    pub hi: uint32_t,
    pub lo: uint32_t,
}
pub type NTP_int32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Timespec {
    pub tv_sec_high: uint32_t,
    pub tv_sec_low: uint32_t,
    pub tv_nsec: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Float {
    pub f: int32_t,
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_ZeroTimespec(mut ts: *mut timespec) {
    (*ts).tv_sec = 0 as libc::c_int as __time_t;
    (*ts).tv_nsec = 0 as libc::c_int as __syscall_slong_t;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_IsZeroTimespec(mut ts: *mut timespec)
 -> libc::c_int {
    return ((*ts).tv_sec == 0 && (*ts).tv_nsec == 0) as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_TimevalToTimespec(mut tv: *mut timeval,
                                               mut ts: *mut timespec) {
    (*ts).tv_sec = (*tv).tv_sec;
    (*ts).tv_nsec = 1000 as libc::c_int as libc::c_long * (*tv).tv_usec;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_TimespecToTimeval(mut ts: *mut timespec,
                                               mut tv: *mut timeval) {
    (*tv).tv_sec = (*ts).tv_sec;
    (*tv).tv_usec = (*ts).tv_nsec / 1000 as libc::c_int as libc::c_long;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_TimespecToDouble(mut ts: *mut timespec)
 -> libc::c_double {
    return (*ts).tv_sec as libc::c_double +
               1.0e-9f64 * (*ts).tv_nsec as libc::c_double;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_DoubleToTimespec(mut d: libc::c_double,
                                              mut ts: *mut timespec) {
    (*ts).tv_sec = d as __time_t;
    (*ts).tv_nsec =
        (1.0e9f64 * (d - (*ts).tv_sec as libc::c_double)) as
            __syscall_slong_t;
    UTI_NormaliseTimespec(ts);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_NormaliseTimespec(mut ts: *mut timespec) {
    if (*ts).tv_nsec >= 1000000000 as libc::c_int as libc::c_long ||
           (*ts).tv_nsec < 0 as libc::c_int as libc::c_long {
        (*ts).tv_sec +=
            (*ts).tv_nsec / 1000000000 as libc::c_int as libc::c_long;
        (*ts).tv_nsec =
            (*ts).tv_nsec % 1000000000 as libc::c_int as libc::c_long;
        /* If seconds are negative nanoseconds would end up negative too */
        if (*ts).tv_nsec < 0 as libc::c_int as libc::c_long {
            (*ts).tv_sec -= 1;
            (*ts).tv_nsec += 1000000000 as libc::c_int as libc::c_long
        }
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_TimevalToDouble(mut tv: *mut timeval)
 -> libc::c_double {
    return (*tv).tv_sec as libc::c_double +
               1.0e-6f64 * (*tv).tv_usec as libc::c_double;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_DoubleToTimeval(mut a: libc::c_double,
                                             mut b: *mut timeval) {
    let mut frac_part: libc::c_double = 0.;
    (*b).tv_sec = a as __time_t;
    frac_part = 1.0e6f64 * (a - (*b).tv_sec as libc::c_double);
    (*b).tv_usec =
        if frac_part > 0 as libc::c_int as libc::c_double {
            (frac_part) + 0.5f64
        } else { (frac_part) - 0.5f64 } as __suseconds_t;
    UTI_NormaliseTimeval(b);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_NormaliseTimeval(mut x: *mut timeval) {
    /* Reduce tv_usec to within +-1000000 of zero. JGH */
    if (*x).tv_usec >= 1000000 as libc::c_int as libc::c_long ||
           (*x).tv_usec <= -(1000000 as libc::c_int) as libc::c_long {
        (*x).tv_sec += (*x).tv_usec / 1000000 as libc::c_int as libc::c_long;
        (*x).tv_usec = (*x).tv_usec % 1000000 as libc::c_int as libc::c_long
    }
    /* Make tv_usec positive. JGH */
    if (*x).tv_usec < 0 as libc::c_int as libc::c_long {
        (*x).tv_sec -= 1;
        (*x).tv_usec += 1000000 as libc::c_int as libc::c_long
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_CompareTimespecs(mut a: *mut timespec,
                                              mut b: *mut timespec)
 -> libc::c_int {
    if (*a).tv_sec < (*b).tv_sec { return -(1 as libc::c_int) }
    if (*a).tv_sec > (*b).tv_sec { return 1 as libc::c_int }
    if (*a).tv_nsec < (*b).tv_nsec { return -(1 as libc::c_int) }
    if (*a).tv_nsec > (*b).tv_nsec { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_DiffTimespecs(mut result: *mut timespec,
                                           mut a: *mut timespec,
                                           mut b: *mut timespec) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_nsec = (*a).tv_nsec - (*b).tv_nsec;
    UTI_NormaliseTimespec(result);
}
/* ================================================== */
/* Calculate result = a - b and return as a double */
#[no_mangle]
pub unsafe extern "C" fn UTI_DiffTimespecsToDouble(mut a: *mut timespec,
                                                   mut b: *mut timespec)
 -> libc::c_double {
    return (*a).tv_sec as libc::c_double - (*b).tv_sec as libc::c_double +
               1.0e-9f64 * ((*a).tv_nsec - (*b).tv_nsec) as libc::c_double;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_AddDoubleToTimespec(mut start: *mut timespec,
                                                 mut increment:
                                                     libc::c_double,
                                                 mut end: *mut timespec) {
    let mut int_part: time_t = 0;
    int_part = increment as time_t;
    (*end).tv_sec = (*start).tv_sec + int_part;
    (*end).tv_nsec =
        ((*start).tv_nsec as libc::c_double +
             1.0e9f64 * (increment - int_part as libc::c_double)) as
            __syscall_slong_t;
    UTI_NormaliseTimespec(end);
}
/* ================================================== */
/* Calculate the average and difference (as a double) of two timespecs */
#[no_mangle]
pub unsafe extern "C" fn UTI_AverageDiffTimespecs(mut earlier: *mut timespec,
                                                  mut later: *mut timespec,
                                                  mut average: *mut timespec,
                                                  mut diff:
                                                      *mut libc::c_double) {
    *diff = UTI_DiffTimespecsToDouble(later, earlier);
    UTI_AddDoubleToTimespec(earlier, *diff / 2.0f64, average);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_AddDiffToTimespec(mut a: *mut timespec,
                                               mut b: *mut timespec,
                                               mut c: *mut timespec,
                                               mut result: *mut timespec) {
    let mut diff: libc::c_double = 0.;
    diff = UTI_DiffTimespecsToDouble(a, b);
    UTI_AddDoubleToTimespec(c, diff, result);
}
static mut buffer_pool: [[libc::c_char; 64]; 16] = [[0; 64]; 16];
static mut pool_ptr: libc::c_int = 0 as libc::c_int;
/* ================================================== */
/* Convert a timespec into a temporary string, largely for diagnostic display */
#[no_mangle]
pub unsafe extern "C" fn UTI_TimespecToString(mut ts: *mut timespec)
 -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    pool_ptr = (pool_ptr + 1 as libc::c_int) % 16 as libc::c_int;
    result = buffer_pool[pool_ptr as usize].as_mut_ptr();
    snprintf(result, 64 as libc::c_int as libc::c_ulong,
             b"%ld.%09lu\x00" as *const u8 as *const libc::c_char,
             (*ts).tv_sec, (*ts).tv_nsec as libc::c_ulong);
    return result;
}
/* ================================================== */
/* Convert an NTP timestamp into a temporary string, largely
   for diagnostic display */
#[no_mangle]
pub unsafe extern "C" fn UTI_Ntp64ToString(mut ntp_ts: *mut NTP_int64)
 -> *mut libc::c_char {
    let mut ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    UTI_Ntp64ToTimespec(ntp_ts, &mut ts);
    return UTI_TimespecToString(&mut ts);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_RefidToString(mut ref_id: uint32_t)
 -> *mut libc::c_char {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut c: libc::c_uint = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    pool_ptr = (pool_ptr + 1 as libc::c_int) % 16 as libc::c_int;
    result = buffer_pool[pool_ptr as usize].as_mut_ptr();
    j = 0 as libc::c_int as libc::c_uint;
    i = j;
    while i < 4 as libc::c_int as libc::c_uint &&
              i < (64 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
        c =
            ref_id >>
                (24 as libc::c_int as
                     libc::c_uint).wrapping_sub(i.wrapping_mul(8 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint))
                & 0xff as libc::c_int as libc::c_uint;
        if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as
               libc::c_int &
               _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            let fresh0 = j;
            j = j.wrapping_add(1);
            *result.offset(fresh0 as isize) = c as libc::c_char
        }
        i = i.wrapping_add(1)
    }
    *result.offset(j as isize) = '\u{0}' as i32 as libc::c_char;
    return result;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_IPToString(mut addr: *mut IPAddr)
 -> *mut libc::c_char {
    let mut a: libc::c_ulong = 0;
    let mut b: libc::c_ulong = 0;
    let mut c: libc::c_ulong = 0;
    let mut d: libc::c_ulong = 0;
    let mut ip: libc::c_ulong = 0;
    let mut ip6: *mut uint8_t = 0 as *mut uint8_t;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    pool_ptr = (pool_ptr + 1 as libc::c_int) % 16 as libc::c_int;
    result = buffer_pool[pool_ptr as usize].as_mut_ptr();
    match (*addr).family as libc::c_int {
        0 => {
            snprintf(result, 64 as libc::c_int as libc::c_ulong,
                     b"[UNSPEC]\x00" as *const u8 as *const libc::c_char);
        }
        1 => {
            ip = (*addr).addr.in4 as libc::c_ulong;
            a =
                ip >> 24 as libc::c_int &
                    0xff as libc::c_int as libc::c_ulong;
            b =
                ip >> 16 as libc::c_int &
                    0xff as libc::c_int as libc::c_ulong;
            c = ip >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong;
            d = ip >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_ulong;
            snprintf(result, 64 as libc::c_int as libc::c_ulong,
                     b"%lu.%lu.%lu.%lu\x00" as *const u8 as
                         *const libc::c_char, a, b, c, d);
        }
        2 => {
            ip6 = (*addr).addr.in6.as_mut_ptr();
            inet_ntop(10 as libc::c_int, ip6 as *const libc::c_void, result,
                      64 as libc::c_int as socklen_t);
        }
        _ => {
            snprintf(result, 64 as libc::c_int as libc::c_ulong,
                     b"[UNKNOWN]\x00" as *const u8 as *const libc::c_char);
        }
    }
    return result;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_StringToIP(mut addr: *const libc::c_char,
                                        mut ip: *mut IPAddr) -> libc::c_int {
    let mut in4: in_addr = in_addr{s_addr: 0,};
    let mut in6: in6_addr =
        in6_addr{__in6_u: C2RustUnnamed_0{__u6_addr8: [0; 16],},};
    if inet_pton(2 as libc::c_int, addr,
                 &mut in4 as *mut in_addr as *mut libc::c_void) >
           0 as libc::c_int {
        (*ip).family = 1 as libc::c_int as uint16_t;
        (*ip)._pad = 0 as libc::c_int as uint16_t;
        (*ip).addr.in4 = ntohl(in4.s_addr);
        return 1 as libc::c_int
    }
    if inet_pton(10 as libc::c_int, addr,
                 &mut in6 as *mut in6_addr as *mut libc::c_void) >
           0 as libc::c_int {
        (*ip).family = 2 as libc::c_int as uint16_t;
        (*ip)._pad = 0 as libc::c_int as uint16_t;
        memcpy((*ip).addr.in6.as_mut_ptr() as *mut libc::c_void,
               in6.__in6_u.__u6_addr8.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_IPToRefid(mut ip: *mut IPAddr) -> uint32_t {
    static mut MD5_hash: libc::c_int = -(1 as libc::c_int);
    let mut buf: [libc::c_uchar; 16] = [0; 16];
    match (*ip).family as libc::c_int {
        1 => { return (*ip).addr.in4 }
        2 => {
            if MD5_hash < 0 as libc::c_int {
                MD5_hash =
                    HSH_GetHashId(b"MD5\x00" as *const u8 as
                                      *const libc::c_char)
            }
            if MD5_hash < 0 as libc::c_int ||
                   HSH_Hash(MD5_hash,
                            (*ip).addr.in6.as_mut_ptr() as
                                *const libc::c_uchar,
                            ::std::mem::size_of::<[uint8_t; 16]>() as
                                libc::c_ulong as libc::c_uint,
                            0 as *const libc::c_uchar,
                            0 as libc::c_int as libc::c_uint,
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_uchar; 16]>() as
                                libc::c_ulong as libc::c_uint) as
                       libc::c_ulong !=
                       ::std::mem::size_of::<[libc::c_uchar; 16]>() as
                           libc::c_ulong {
                LOG_Message(LOGS_FATAL,
                            b"Could not get MD5\x00" as *const u8 as
                                *const libc::c_char);
                exit(1 as libc::c_int);
            }
            return (buf[0 as libc::c_int as usize] as uint32_t) <<
                       24 as libc::c_int |
                       ((buf[1 as libc::c_int as usize] as libc::c_int) <<
                            16 as libc::c_int) as libc::c_uint |
                       ((buf[2 as libc::c_int as usize] as libc::c_int) <<
                            8 as libc::c_int) as libc::c_uint |
                       buf[3 as libc::c_int as usize] as libc::c_uint
        }
        _ => { }
    }
    return 0 as libc::c_int as uint32_t;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_IPToHash(mut ip: *mut IPAddr) -> uint32_t {
    static mut seed: uint32_t = 0 as libc::c_int as uint32_t;
    let mut addr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_uint = 0;
    let mut len: libc::c_uint = 0;
    let mut hash: uint32_t = 0;
    match (*ip).family as libc::c_int {
        1 => {
            addr = &mut (*ip).addr.in4 as *mut uint32_t as *mut libc::c_uchar;
            len =
                ::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                    libc::c_uint
        }
        2 => {
            addr = (*ip).addr.in6.as_mut_ptr();
            len =
                ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong as
                    libc::c_uint
        }
        _ => { return 0 as libc::c_int as uint32_t }
    }
    /* Include a random seed in the hash to randomize collisions
     and order of addresses in hash tables */
    while seed == 0 {
        UTI_GetRandomBytes(&mut seed as *mut uint32_t as *mut libc::c_void,
                           ::std::mem::size_of::<uint32_t>() as libc::c_ulong
                               as libc::c_uint);
    }
    i = 0 as libc::c_int as libc::c_uint;
    hash = seed;
    while i < len {
        hash =
            (71 as libc::c_int as
                 libc::c_uint).wrapping_mul(hash).wrapping_add(*addr.offset(i
                                                                                as
                                                                                isize)
                                                                   as
                                                                   libc::c_uint);
        i = i.wrapping_add(1)
    }
    return hash.wrapping_add(seed);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_IPHostToNetwork(mut src: *mut IPAddr,
                                             mut dest: *mut IPAddr) {
    /* Don't send uninitialized bytes over network */
    memset(dest as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<IPAddr>() as libc::c_ulong);
    (*dest).family = htons((*src).family);
    match (*src).family as libc::c_int {
        1 => { (*dest).addr.in4 = htonl((*src).addr.in4) }
        2 => {
            memcpy((*dest).addr.in6.as_mut_ptr() as *mut libc::c_void,
                   (*src).addr.in6.as_mut_ptr() as *const libc::c_void,
                   ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong);
        }
        _ => { (*dest).family = htons(0 as libc::c_int as uint16_t) }
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_IPNetworkToHost(mut src: *mut IPAddr,
                                             mut dest: *mut IPAddr) {
    (*dest).family = ntohs((*src).family);
    (*dest)._pad = 0 as libc::c_int as uint16_t;
    match (*dest).family as libc::c_int {
        1 => { (*dest).addr.in4 = ntohl((*src).addr.in4) }
        2 => {
            memcpy((*dest).addr.in6.as_mut_ptr() as *mut libc::c_void,
                   (*src).addr.in6.as_mut_ptr() as *const libc::c_void,
                   ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong);
        }
        _ => { (*dest).family = 0 as libc::c_int as uint16_t }
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_CompareIPs(mut a: *mut IPAddr,
                                        mut b: *mut IPAddr,
                                        mut mask: *mut IPAddr)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    if (*a).family as libc::c_int != (*b).family as libc::c_int {
        return (*a).family as libc::c_int - (*b).family as libc::c_int
    }
    if !mask.is_null() &&
           (*mask).family as libc::c_int != (*b).family as libc::c_int {
        mask = 0 as *mut IPAddr
    }
    match (*a).family as libc::c_int {
        0 => { return 0 as libc::c_int }
        1 => {
            if !mask.is_null() {
                return ((*a).addr.in4 &
                            (*mask).addr.in4).wrapping_sub((*b).addr.in4 &
                                                               (*mask).addr.in4)
                           as libc::c_int
            } else {
                return (*a).addr.in4.wrapping_sub((*b).addr.in4) as
                           libc::c_int
            }
        }
        2 => {
            i = 0 as libc::c_int;
            d = 0 as libc::c_int;
            while d == 0 && i < 16 as libc::c_int {
                if !mask.is_null() {
                    d =
                        ((*a).addr.in6[i as usize] as libc::c_int &
                             (*mask).addr.in6[i as usize] as libc::c_int) -
                            ((*b).addr.in6[i as usize] as libc::c_int &
                                 (*mask).addr.in6[i as usize] as libc::c_int)
                } else {
                    d =
                        (*a).addr.in6[i as usize] as libc::c_int -
                            (*b).addr.in6[i as usize] as libc::c_int
                }
                i += 1
            }
            return d
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_IPSockAddrToString(mut sa: *mut IPSockAddr)
 -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    pool_ptr = (pool_ptr + 1 as libc::c_int) % 16 as libc::c_int;
    result = buffer_pool[pool_ptr as usize].as_mut_ptr();
    snprintf(result, 64 as libc::c_int as libc::c_ulong,
             if (*sa).ip_addr.family as libc::c_int != 2 as libc::c_int {
                 b"%s:%hu\x00" as *const u8 as *const libc::c_char
             } else { b"[%s]:%hu\x00" as *const u8 as *const libc::c_char },
             UTI_IPToString(&mut (*sa).ip_addr), (*sa).port as libc::c_int);
    return result;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_TimeToLogForm(mut t: time_t)
 -> *mut libc::c_char {
    let mut stm: *mut tm = 0 as *mut tm;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    pool_ptr = (pool_ptr + 1 as libc::c_int) % 16 as libc::c_int;
    result = buffer_pool[pool_ptr as usize].as_mut_ptr();
    stm = gmtime(&mut t);
    if !stm.is_null() {
        strftime(result, 64 as libc::c_int as size_t,
                 b"%Y-%m-%d %H:%M:%S\x00" as *const u8 as *const libc::c_char,
                 stm);
    } else {
        snprintf(result, 64 as libc::c_int as libc::c_ulong,
                 b"INVALID    INVALID \x00" as *const u8 as
                     *const libc::c_char);
    }
    return result;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_AdjustTimespec(mut old_ts: *mut timespec,
                                            mut when: *mut timespec,
                                            mut new_ts: *mut timespec,
                                            mut delta_time:
                                                *mut libc::c_double,
                                            mut dfreq: libc::c_double,
                                            mut doffset: libc::c_double) {
    let mut elapsed: libc::c_double = 0.;
    elapsed = UTI_DiffTimespecsToDouble(when, old_ts);
    *delta_time = elapsed * dfreq - doffset;
    UTI_AddDoubleToTimespec(old_ts, *delta_time, new_ts);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_GetNtp64Fuzz(mut ts: *mut NTP_int64,
                                          mut precision: libc::c_int) {
    let mut start: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    if precision >= -(32 as libc::c_int) && precision <= 32 as libc::c_int {
    } else {
        __assert_fail(b"precision >= -32 && precision <= 32\x00" as *const u8
                          as *const libc::c_char,
                      b"util.c\x00" as *const u8 as *const libc::c_char,
                      547 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"void UTI_GetNtp64Fuzz(NTP_int64 *, int)\x00")).as_ptr());
    }
    if ::std::mem::size_of::<NTP_int64>() as libc::c_ulong ==
           8 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"sizeof (*ts) == 8\x00" as *const u8 as
                          *const libc::c_char,
                      b"util.c\x00" as *const u8 as *const libc::c_char,
                      548 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"void UTI_GetNtp64Fuzz(NTP_int64 *, int)\x00")).as_ptr());
    }
    start =
        (::std::mem::size_of::<NTP_int64>() as
             libc::c_ulong).wrapping_sub(((precision + 32 as libc::c_int +
                                               7 as libc::c_int) /
                                              8 as libc::c_int) as
                                             libc::c_ulong) as libc::c_int;
    (*ts).lo = 0 as libc::c_int as uint32_t;
    (*ts).hi = (*ts).lo;
    UTI_GetRandomBytes((ts as *mut libc::c_uchar).offset(start as isize) as
                           *mut libc::c_void,
                       (::std::mem::size_of::<NTP_int64>() as
                            libc::c_ulong).wrapping_sub(start as
                                                            libc::c_ulong) as
                           libc::c_uint);
    bits = (precision + 32 as libc::c_int) % 8 as libc::c_int;
    if bits != 0 {
        let ref mut fresh1 =
            *(ts as *mut libc::c_uchar).offset(start as isize);
        *fresh1 =
            (*fresh1 as
                 libc::c_uint).wrapping_rem((1 as libc::c_uint) << bits) as
                libc::c_uchar as libc::c_uchar
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_Ntp32ToDouble(mut x: NTP_int32)
 -> libc::c_double {
    return ntohl(x) as libc::c_double / 65536.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn UTI_DoubleToNtp32(mut x: libc::c_double)
 -> NTP_int32 {
    let mut r: NTP_int32 = 0;
    if x >= 4294967295.0f64 / 65536.0f64 {
        r = 0xffffffff as libc::c_uint
    } else if x <= 0.0f64 {
        r = 0 as libc::c_int as NTP_int32
    } else {
        x *= 65536.0f64;
        r = x as NTP_int32;
        /* Round up */
        if (r as libc::c_double) < x { r = r.wrapping_add(1) }
    }
    return htonl(r);
}

/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_ZeroNtp64(mut ts: *mut NTP_int64) {
    (*ts).lo = htonl(0 as libc::c_int as uint32_t);
    (*ts).hi = (*ts).lo;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_IsZeroNtp64(mut ts: *mut NTP_int64)
 -> libc::c_int {
    return ((*ts).hi == 0 && (*ts).lo == 0) as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_CompareNtp64(mut a: *mut NTP_int64,
                                          mut b: *mut NTP_int64)
 -> libc::c_int {
    let mut diff: int32_t = 0;
    if (*a).hi == (*b).hi && (*a).lo == (*b).lo { return 0 as libc::c_int }
    diff = ntohl((*a).hi).wrapping_sub(ntohl((*b).hi)) as int32_t;
    if diff < 0 as libc::c_int { return -(1 as libc::c_int) }
    if diff > 0 as libc::c_int { return 1 as libc::c_int }
    return if ntohl((*a).lo) < ntohl((*b).lo) {
               -(1 as libc::c_int)
           } else { 1 as libc::c_int };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_IsEqualAnyNtp64(mut a: *mut NTP_int64,
                                             mut b1: *mut NTP_int64,
                                             mut b2: *mut NTP_int64,
                                             mut b3: *mut NTP_int64)
 -> libc::c_int {
    if !b1.is_null() && (*a).lo == (*b1).lo && (*a).hi == (*b1).hi {
        return 1 as libc::c_int
    }
    if !b2.is_null() && (*a).lo == (*b2).lo && (*a).hi == (*b2).hi {
        return 1 as libc::c_int
    }
    if !b3.is_null() && (*a).lo == (*b3).lo && (*a).hi == (*b3).hi {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn UTI_TimespecToNtp64(mut src: *mut timespec,
                                             mut dest: *mut NTP_int64,
                                             mut fuzz: *mut NTP_int64) {
    let mut hi: uint32_t = 0;
    let mut lo: uint32_t = 0;
    let mut sec: uint32_t = 0;
    let mut nsec: uint32_t = 0;
    sec = (*src).tv_sec as uint32_t;
    nsec = (*src).tv_nsec as uint32_t;
    /* Recognize zero as a special case - it always signifies
     an 'unknown' value */
    if nsec == 0 && sec == 0 {
        lo = 0 as libc::c_int as uint32_t;
        hi = lo
    } else {
        hi =
            htonl((sec as
                       libc::c_ulong).wrapping_add(0x83aa7e80 as
                                                       libc::c_ulong) as
                      uint32_t);
        lo = htonl((4.294967296f64 * nsec as libc::c_double) as uint32_t);
        /* Add the fuzz */
        if !fuzz.is_null() { hi ^= (*fuzz).hi; lo ^= (*fuzz).lo }
    }
    (*dest).hi = hi;
    (*dest).lo = lo;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_Ntp64ToTimespec(mut src: *mut NTP_int64,
                                             mut dest: *mut timespec) {
    let mut ntp_sec: uint32_t = 0;
    let mut ntp_frac: uint32_t = 0;
    /* Zero is a special value */
    if UTI_IsZeroNtp64(src) != 0 { UTI_ZeroTimespec(dest); return }
    ntp_sec = ntohl((*src).hi);
    ntp_frac = ntohl((*src).lo);
    (*dest).tv_sec =
        ntp_sec.wrapping_sub(((1581921726 as libc::c_longlong -
                                   (18250 as libc::c_int * 24 as libc::c_int *
                                        3600 as libc::c_int) as
                                       libc::c_longlong) as
                                  libc::c_ulonglong).wrapping_add(0x83aa7e80
                                                                      as
                                                                      libc::c_ulong
                                                                      as
                                                                      libc::c_ulonglong)
                                 as uint32_t) as libc::c_long +
            (1581921726 as libc::c_longlong -
                 (18250 as libc::c_int * 24 as libc::c_int *
                      3600 as libc::c_int) as libc::c_longlong) as time_t;
    (*dest).tv_nsec =
        (ntp_frac as libc::c_double / 4.294967296f64) as __syscall_slong_t;
}
/* Minimum allowed distance from maximum 32-bit time_t */
#[no_mangle]
pub unsafe extern "C" fn UTI_IsTimeOffsetSane(mut ts: *mut timespec,
                                              mut offset: libc::c_double)
 -> libc::c_int {
    let mut t: libc::c_double = 0.;
    /* Handle nan correctly here */
    if !(offset > -4294967296.0f64 && offset < 4294967296.0f64) {
        return 0 as libc::c_int
    }
    t = UTI_TimespecToDouble(ts) + offset;
    /* Time before 1970 is not considered valid */
    if t < 0.0f64 { return 0 as libc::c_int }
    /* Check if it's in the interval to which NTP time is mapped */
    if t <
           (1581921726 as libc::c_longlong -
                (18250 as libc::c_int * 24 as libc::c_int *
                     3600 as libc::c_int) as libc::c_longlong) as
               libc::c_double ||
           t >
               (1581921726 as libc::c_longlong -
                    (18250 as libc::c_int * 24 as libc::c_int *
                         3600 as libc::c_int) as libc::c_longlong +
                    ((1 as libc::c_longlong) << 32 as libc::c_int)) as
                   libc::c_double {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_Log2ToDouble(mut l: libc::c_int)
 -> libc::c_double {
    if l >= 0 as libc::c_int {
        if l > 31 as libc::c_int { l = 31 as libc::c_int }
        return ((1 as libc::c_int as uint32_t) << l) as libc::c_double
    } else {
        if l < -(31 as libc::c_int) { l = -(31 as libc::c_int) }
        return 1.0f64 /
                   ((1 as libc::c_int as uint32_t) << -l) as libc::c_double
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_TimespecNetworkToHost(mut src: *mut Timespec,
                                                   mut dest: *mut timespec) {
    let mut sec_low: uint32_t = 0;
    let mut nsec: uint32_t = 0;
    let mut sec_high: uint32_t = 0;
    sec_low = ntohl((*src).tv_sec_low);
    sec_high = ntohl((*src).tv_sec_high);
    if sec_high == 0x7fffffff as libc::c_int as libc::c_uint {
        sec_high = 0 as libc::c_int as uint32_t
    }
    (*dest).tv_sec =
        ((sec_high as uint64_t) << 32 as libc::c_int |
             sec_low as libc::c_ulong) as __time_t;
    nsec = ntohl((*src).tv_nsec);
    (*dest).tv_nsec =
        if nsec < 999999999 as libc::c_uint {
            nsec
        } else { 999999999 as libc::c_uint } as __syscall_slong_t;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_TimespecHostToNetwork(mut src: *mut timespec,
                                                   mut dest: *mut Timespec) {
    (*dest).tv_nsec = htonl((*src).tv_nsec as uint32_t);
    (*dest).tv_sec_high =
        htonl(((*src).tv_sec as uint64_t >> 32 as libc::c_int) as uint32_t);
    (*dest).tv_sec_low = htonl((*src).tv_sec as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn UTI_FloatNetworkToHost(mut f: Float)
 -> libc::c_double {
    let mut exp: int32_t = 0;
    let mut coef: int32_t = 0;
    let mut x: uint32_t = 0;
    x = ntohl(f.f as uint32_t);
    exp =
        (x >>
             ::std::mem::size_of::<int32_t>() as libc::c_ulong as libc::c_int
                 * 8 as libc::c_int - 7 as libc::c_int) as int32_t;
    if exp >= (1 as libc::c_int) << 7 as libc::c_int - 1 as libc::c_int {
        exp -= (1 as libc::c_int) << 7 as libc::c_int
    }
    exp -=
        ::std::mem::size_of::<int32_t>() as libc::c_ulong as libc::c_int *
            8 as libc::c_int - 7 as libc::c_int;
    coef =
        x.wrapping_rem((1 as libc::c_uint) <<
                           ::std::mem::size_of::<int32_t>() as libc::c_ulong
                               as libc::c_int * 8 as libc::c_int -
                               7 as libc::c_int) as int32_t;
    if coef >=
           (1 as libc::c_int) <<
               ::std::mem::size_of::<int32_t>() as libc::c_ulong as
                   libc::c_int * 8 as libc::c_int - 7 as libc::c_int -
                   1 as libc::c_int {
        coef -=
            (1 as libc::c_int) <<
                ::std::mem::size_of::<int32_t>() as libc::c_ulong as
                    libc::c_int * 8 as libc::c_int - 7 as libc::c_int
    }
    return coef as libc::c_double * pow(2.0f64, exp as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn UTI_FloatHostToNetwork(mut x: libc::c_double)
 -> Float {
    let mut exp: int32_t = 0;
    let mut coef: int32_t = 0;
    let mut neg: int32_t = 0;
    let mut f: Float = Float{f: 0,};
    if x < 0.0f64 {
        x = -x;
        neg = 1 as libc::c_int
    } else if x >= 0.0f64 {
        neg = 0 as libc::c_int
    } else {
        /* Save NaN as zero */
        x = 0.0f64;
        neg = 0 as libc::c_int
    }
    if x < 1.0e-100f64 {
        coef = 0 as libc::c_int;
        exp = coef
    } else if x > 1.0e100f64 {
        exp =
            --((1 as libc::c_int) << 7 as libc::c_int - 1 as libc::c_int) -
                1 as libc::c_int;
        coef =
            --((1 as libc::c_int) <<
                   ::std::mem::size_of::<int32_t>() as libc::c_ulong as
                       libc::c_int * 8 as libc::c_int - 7 as libc::c_int -
                       1 as libc::c_int) - 1 as libc::c_int + neg
    } else {
        exp =
            (log(x) / log(2 as libc::c_int as libc::c_double) +
                 1 as libc::c_int as libc::c_double) as int32_t;
        coef =
            (x *
                 pow(2.0f64,
                     (-exp +
                          (::std::mem::size_of::<int32_t>() as libc::c_ulong
                               as libc::c_int * 8 as libc::c_int -
                               7 as libc::c_int)) as libc::c_double) + 0.5f64)
                as int32_t;
        if coef > 0 as libc::c_int {
        } else {
            __assert_fail(b"coef > 0\x00" as *const u8 as *const libc::c_char,
                          b"util.c\x00" as *const u8 as *const libc::c_char,
                          852 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 37],
                                                    &[libc::c_char; 37]>(b"Float UTI_FloatHostToNetwork(double)\x00")).as_ptr());
        }
        /* we may need to shift up to two bits down */
        while coef >
                  --((1 as libc::c_int) <<
                         ::std::mem::size_of::<int32_t>() as libc::c_ulong as
                             libc::c_int * 8 as libc::c_int - 7 as libc::c_int
                             - 1 as libc::c_int) - 1 as libc::c_int + neg {
            coef >>= 1 as libc::c_int;
            exp += 1
        }
        if exp >
               --((1 as libc::c_int) << 7 as libc::c_int - 1 as libc::c_int) -
                   1 as libc::c_int {
            /* overflow */
            exp =
                --((1 as libc::c_int) << 7 as libc::c_int - 1 as libc::c_int)
                    - 1 as libc::c_int;
            coef =
                --((1 as libc::c_int) <<
                       ::std::mem::size_of::<int32_t>() as libc::c_ulong as
                           libc::c_int * 8 as libc::c_int - 7 as libc::c_int -
                           1 as libc::c_int) - 1 as libc::c_int + neg
        } else if exp <
                      -((1 as libc::c_int) <<
                            7 as libc::c_int - 1 as libc::c_int) {
            /* underflow */
            if exp +
                   (::std::mem::size_of::<int32_t>() as libc::c_ulong as
                        libc::c_int * 8 as libc::c_int - 7 as libc::c_int) >=
                   -((1 as libc::c_int) <<
                         7 as libc::c_int - 1 as libc::c_int) {
                coef >>=
                    -((1 as libc::c_int) <<
                          7 as libc::c_int - 1 as libc::c_int) - exp;
                exp =
                    -((1 as libc::c_int) <<
                          7 as libc::c_int - 1 as libc::c_int)
            } else { coef = 0 as libc::c_int; exp = coef }
        }
    }
    /* negate back */
    if neg != 0 {
        coef =
            ((-coef as uint32_t) << 7 as libc::c_int >> 7 as libc::c_int) as
                int32_t
    }
    f.f =
        htonl((exp as uint32_t) <<
                  ::std::mem::size_of::<int32_t>() as libc::c_ulong as
                      libc::c_int * 8 as libc::c_int - 7 as libc::c_int |
                  coef as libc::c_uint) as int32_t;
    return f;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_FdSetCloexec(mut fd: libc::c_int)
 -> libc::c_int {
    let mut flags: libc::c_int = 0;
    flags = fcntl(fd, 1 as libc::c_int);
    if flags == -(1 as libc::c_int) {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"fcntl() failed : %s\x00" as *const u8 as
                            *const libc::c_char,
                        strerror(*__errno_location()));
        }
        return 0 as libc::c_int
    }
    flags |= 1 as libc::c_int;
    if fcntl(fd, 2 as libc::c_int, flags) < 0 as libc::c_int {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"fcntl() failed : %s\x00" as *const u8 as
                            *const libc::c_char,
                        strerror(*__errno_location()));
        }
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_SetQuitSignalsHandler(mut handler:
                                                       Option<unsafe extern "C" fn(_:
                                                                                       libc::c_int)
                                                                  -> ()>,
                                                   mut ignore_sigpipe:
                                                       libc::c_int) {
    let mut sa: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_11{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    sa.__sigaction_handler.sa_handler = handler;
    sa.sa_flags = 0x10000000 as libc::c_int;
    if sigemptyset(&mut sa.sa_mask) < 0 as libc::c_int {
        LOG_Message(LOGS_FATAL,
                    b"sigemptyset() failed\x00" as *const u8 as
                        *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if sigaction(2 as libc::c_int, &mut sa, 0 as *mut sigaction) <
           0 as libc::c_int {
        LOG_Message(LOGS_FATAL,
                    b"sigaction(%d) failed\x00" as *const u8 as
                        *const libc::c_char, 2 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if sigaction(15 as libc::c_int, &mut sa, 0 as *mut sigaction) <
           0 as libc::c_int {
        LOG_Message(LOGS_FATAL,
                    b"sigaction(%d) failed\x00" as *const u8 as
                        *const libc::c_char, 15 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if sigaction(3 as libc::c_int, &mut sa, 0 as *mut sigaction) <
           0 as libc::c_int {
        LOG_Message(LOGS_FATAL,
                    b"sigaction(%d) failed\x00" as *const u8 as
                        *const libc::c_char, 3 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if sigaction(1 as libc::c_int, &mut sa, 0 as *mut sigaction) <
           0 as libc::c_int {
        LOG_Message(LOGS_FATAL,
                    b"sigaction(%d) failed\x00" as *const u8 as
                        *const libc::c_char, 1 as libc::c_int);
        exit(1 as libc::c_int);
    }
    if ignore_sigpipe != 0 {
        sa.__sigaction_handler.sa_handler =
            ::std::mem::transmute::<libc::intptr_t,
                                    __sighandler_t>(1 as libc::c_int as
                                                        libc::intptr_t)
    }
    if sigaction(13 as libc::c_int, &mut sa, 0 as *mut sigaction) <
           0 as libc::c_int {
        LOG_Message(LOGS_FATAL,
                    b"sigaction(%d) failed\x00" as *const u8 as
                        *const libc::c_char, 13 as libc::c_int);
        exit(1 as libc::c_int);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_PathToDir(mut path: *const libc::c_char)
 -> *mut libc::c_char {
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    slash = strrchr(path, '/' as i32);
    if slash.is_null() {
        return Strdup(b".\x00" as *const u8 as *const libc::c_char)
    }
    if slash == path as *mut libc::c_char {
        return Strdup(b"/\x00" as *const u8 as *const libc::c_char)
    }
    dir =
        Malloc((slash.wrapping_offset_from(path) as libc::c_long +
                    1 as libc::c_int as libc::c_long) as size_t) as
            *mut libc::c_char;
    snprintf(dir,
             (slash.wrapping_offset_from(path) as libc::c_long +
                  1 as libc::c_int as libc::c_long) as libc::c_ulong,
             b"%s\x00" as *const u8 as *const libc::c_char, path);
    return dir;
}
/* ================================================== */
unsafe extern "C" fn create_dir(mut p: *mut libc::c_char, mut mode: mode_t,
                                mut uid: uid_t, mut gid: gid_t)
 -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut buf: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    /* See if directory exists */
    status = stat(p, &mut buf);
    if status < 0 as libc::c_int {
        if *__errno_location() != 2 as libc::c_int {
            LOG_Message(LOGS_ERR,
                        b"Could not access %s : %s\x00" as *const u8 as
                            *const libc::c_char, p,
                        strerror(*__errno_location()));
            return 0 as libc::c_int
        }
    } else {
        if buf.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
               0o40000 as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        }
        LOG_Message(LOGS_ERR,
                    b"%s is not directory\x00" as *const u8 as
                        *const libc::c_char, p);
        return 0 as libc::c_int
    }
    /* Create the directory */
    if mkdir(p, mode) < 0 as libc::c_int {
        LOG_Message(LOGS_ERR,
                    b"Could not create directory %s : %s\x00" as *const u8 as
                        *const libc::c_char, p,
                    strerror(*__errno_location()));
        return 0 as libc::c_int
    }
    /* Set its owner */
    if chown(p, uid, gid) < 0 as libc::c_int {
        LOG_Message(LOGS_ERR,
                    b"Could not change ownership of %s : %s\x00" as *const u8
                        as *const libc::c_char, p,
                    strerror(*__errno_location()));
        /* Don't leave it there with incorrect ownership */
        rmdir(p);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* ================================================== */
/* Return 0 if the directory couldn't be created, 1 if it could (or
   already existed) */
#[no_mangle]
pub unsafe extern "C" fn UTI_CreateDirAndParents(mut path:
                                                     *const libc::c_char,
                                                 mut mode: mode_t,
                                                 mut uid: uid_t,
                                                 mut gid: gid_t)
 -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    /* Don't try to create current directory */
    if strcmp(path, b".\x00" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int
    }
    p =
        Malloc((1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(path)))
            as *mut libc::c_char;
    k = 0 as libc::c_int;
    i = k;
    loop  {
        let fresh2 = k;
        k = k + 1;
        let fresh3 = i;
        i = i + 1;
        *p.offset(fresh3 as isize) = *path.offset(fresh2 as isize);
        if *path.offset(k as isize) as libc::c_int == '/' as i32 ||
               *path.offset(k as isize) == 0 {
            /* Check whether its end of string, a trailing / or group of / */
            last = 1 as libc::c_int;
            j = k;
            while *path.offset(j as isize) != 0 {
                if *path.offset(j as isize) as libc::c_int != '/' as i32 {
                    /* Pick up a / into p[] thru the assignment at the top of the loop */
                    k = j - 1 as libc::c_int;
                    last = 0 as libc::c_int;
                    break ;
                } else { j += 1 }
            }
            *p.offset(i as isize) = 0 as libc::c_int as libc::c_char;
            if create_dir(p,
                          if last != 0 {
                              mode
                          } else { 0o755 as libc::c_int as libc::c_uint },
                          if last != 0 {
                              uid
                          } else { 0 as libc::c_int as libc::c_uint },
                          if last != 0 {
                              gid
                          } else { 0 as libc::c_int as libc::c_uint }) == 0 {
                free(p as *mut libc::c_void);
                return 0 as libc::c_int
            }
            if last != 0 { break ; }
        }
        if *path.offset(k as isize) == 0 { break ; }
    }
    free(p as *mut libc::c_void);
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_CheckDirPermissions(mut path:
                                                     *const libc::c_char,
                                                 mut perm: mode_t,
                                                 mut uid: uid_t,
                                                 mut gid: gid_t)
 -> libc::c_int {
    let mut buf: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if stat(path, &mut buf) != 0 {
        LOG_Message(LOGS_ERR,
                    b"Could not access %s : %s\x00" as *const u8 as
                        *const libc::c_char, path,
                    strerror(*__errno_location()));
        return 0 as libc::c_int
    }
    if !(buf.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
             0o40000 as libc::c_int as libc::c_uint) {
        LOG_Message(LOGS_ERR,
                    b"%s is not directory\x00" as *const u8 as
                        *const libc::c_char, path);
        return 0 as libc::c_int
    }
    if buf.st_mode & 0o777 as libc::c_int as libc::c_uint & !perm != 0 {
        LOG_Message(LOGS_ERR,
                    b"Wrong permissions on %s\x00" as *const u8 as
                        *const libc::c_char, path);
        return 0 as libc::c_int
    }
    if buf.st_uid != uid {
        LOG_Message(LOGS_ERR,
                    b"Wrong owner of %s (%s != %u)\x00" as *const u8 as
                        *const libc::c_char, path,
                    b"UID\x00" as *const u8 as *const libc::c_char, uid);
        return 0 as libc::c_int
    }
    if buf.st_gid != gid {
        LOG_Message(LOGS_ERR,
                    b"Wrong owner of %s (%s != %u)\x00" as *const u8 as
                        *const libc::c_char, path,
                    b"GID\x00" as *const u8 as *const libc::c_char, gid);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn join_path(mut basedir: *const libc::c_char,
                               mut name: *const libc::c_char,
                               mut suffix: *const libc::c_char,
                               mut buffer: *mut libc::c_char,
                               mut length: size_t, mut severity: LOG_Severity)
 -> libc::c_int {
    let mut sep: *const libc::c_char = 0 as *const libc::c_char;
    if basedir.is_null() {
        basedir = b"\x00" as *const u8 as *const libc::c_char;
        sep = b"\x00" as *const u8 as *const libc::c_char
    } else { sep = b"/\x00" as *const u8 as *const libc::c_char }
    if suffix.is_null() {
        suffix = b"\x00" as *const u8 as *const libc::c_char
    }
    if snprintf(buffer, length,
                b"%s%s%s%s\x00" as *const u8 as *const libc::c_char, basedir,
                sep, name, suffix) as libc::c_ulong >= length {
        LOG_Message(severity,
                    b"File path %s%s%s%s too long\x00" as *const u8 as
                        *const libc::c_char, basedir, sep, name, suffix);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_OpenFile(mut basedir: *const libc::c_char,
                                      mut name: *const libc::c_char,
                                      mut suffix: *const libc::c_char,
                                      mut mode: libc::c_char,
                                      mut perm: mode_t) -> *mut FILE {
    let mut file_mode: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut severity: LOG_Severity = LOGS_INFO;
    let mut fd: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut file: *mut FILE = 0 as *mut FILE;
    severity =
        if mode as libc::c_int >= 'A' as i32 &&
               mode as libc::c_int <= 'Z' as i32 {
            LOGS_FATAL as libc::c_int
        } else { LOGS_ERR as libc::c_int } as LOG_Severity;
    if join_path(basedir, name, suffix, path.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 4096]>() as
                     libc::c_ulong, severity) == 0 {
        return 0 as *mut FILE
    }
    match mode as libc::c_int {
        114 | 82 => {
            flags = 0 as libc::c_int;
            file_mode = b"r\x00" as *const u8 as *const libc::c_char;
            if severity as libc::c_int != LOGS_FATAL as libc::c_int {
                severity = LOGS_DEBUG
            }
        }
        119 | 87 => {
            flags =
                0o1 as libc::c_int | 0o100 as libc::c_int |
                    0o200 as libc::c_int;
            file_mode = b"w\x00" as *const u8 as *const libc::c_char
        }
        97 | 65 => {
            flags =
                0o1 as libc::c_int | 0o100 as libc::c_int |
                    0o2000 as libc::c_int;
            file_mode = b"a\x00" as *const u8 as *const libc::c_char
        }
        _ => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"util.c\x00" as *const u8 as *const libc::c_char,
                          1152 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 75],
                                                    &[libc::c_char; 75]>(b"FILE *UTI_OpenFile(const char *, const char *, const char *, char, mode_t)\x00")).as_ptr());
            return 0 as *mut FILE
        }
    }
    loop  {
        fd = open(path.as_mut_ptr(), flags, perm);
        if fd < 0 as libc::c_int {
            if *__errno_location() == 17 as libc::c_int {
                if unlink(path.as_mut_ptr()) < 0 as libc::c_int {
                    LOG_Message(severity,
                                b"Could not remove %s : %s\x00" as *const u8
                                    as *const libc::c_char, path.as_mut_ptr(),
                                strerror(*__errno_location()));
                    return 0 as *mut FILE
                }
                if 0 as libc::c_int != 0 &&
                       log_min_severity as libc::c_int ==
                           LOGS_DEBUG as libc::c_int {
                    LOG_Message(LOGS_DEBUG,
                                b"Removed %s\x00" as *const u8 as
                                    *const libc::c_char, path.as_mut_ptr());
                }
            } else {
                LOG_Message(severity,
                            b"Could not open %s : %s\x00" as *const u8 as
                                *const libc::c_char, path.as_mut_ptr(),
                            strerror(*__errno_location()));
                return 0 as *mut FILE
            }
        } else {
            UTI_FdSetCloexec(fd);
            file = fdopen(fd, file_mode);
            if file.is_null() {
                LOG_Message(severity,
                            b"Could not open %s : %s\x00" as *const u8 as
                                *const libc::c_char, path.as_mut_ptr(),
                            strerror(*__errno_location()));
                close(fd);
                return 0 as *mut FILE
            }
            if 0 as libc::c_int != 0 &&
                   log_min_severity as libc::c_int ==
                       LOGS_DEBUG as libc::c_int {
                LOG_Message(LOGS_DEBUG,
                            b"Opened %s fd=%d mode=%c\x00" as *const u8 as
                                *const libc::c_char, path.as_mut_ptr(), fd,
                            mode as libc::c_int);
            }
            return file
        }
    };
}


/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_RenameTempFile(mut basedir: *const libc::c_char,
                                            mut name: *const libc::c_char,
                                            mut old_suffix:
                                                *const libc::c_char,
                                            mut new_suffix:
                                                *const libc::c_char)
 -> libc::c_int {
    let mut old_path: [libc::c_char; 4096] = [0; 4096];
    let mut new_path: [libc::c_char; 4096] = [0; 4096];
    if join_path(basedir, name, old_suffix, old_path.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 4096]>() as
                     libc::c_ulong, LOGS_ERR) == 0 {
        return 0 as libc::c_int
    }
    if !(join_path(basedir, name, new_suffix, new_path.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 4096]>() as
                       libc::c_ulong, LOGS_ERR) == 0) {
        if rename(old_path.as_mut_ptr(), new_path.as_mut_ptr()) <
               0 as libc::c_int {
            LOG_Message(LOGS_ERR,
                        b"Could not replace %s with %s : %s\x00" as *const u8
                            as *const libc::c_char, new_path.as_mut_ptr(),
                        old_path.as_mut_ptr(), strerror(*__errno_location()));
        } else {
            if 0 as libc::c_int != 0 &&
                   log_min_severity as libc::c_int ==
                       LOGS_DEBUG as libc::c_int {
                LOG_Message(LOGS_DEBUG,
                            b"Renamed %s to %s\x00" as *const u8 as
                                *const libc::c_char, old_path.as_mut_ptr(),
                            new_path.as_mut_ptr());
            }
            return 1 as libc::c_int
        }
    }
    if unlink(old_path.as_mut_ptr()) < 0 as libc::c_int {
        LOG_Message(LOGS_ERR,
                    b"Could not remove %s : %s\x00" as *const u8 as
                        *const libc::c_char, old_path.as_mut_ptr(),
                    strerror(*__errno_location()));
    }
    return 0 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_RemoveFile(mut basedir: *const libc::c_char,
                                        mut name: *const libc::c_char,
                                        mut suffix: *const libc::c_char)
 -> libc::c_int {
    let mut path: [libc::c_char; 4096] = [0; 4096];
    if join_path(basedir, name, suffix, path.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 4096]>() as
                     libc::c_ulong, LOGS_ERR) == 0 {
        return 0 as libc::c_int
    }
    if unlink(path.as_mut_ptr()) < 0 as libc::c_int {
        LOG_Message(if *__errno_location() != 2 as libc::c_int {
                        LOGS_ERR as libc::c_int
                    } else { LOGS_DEBUG as libc::c_int } as LOG_Severity,
                    b"Could not remove %s : %s\x00" as *const u8 as
                        *const libc::c_char, path.as_mut_ptr(),
                    strerror(*__errno_location()));
        return 0 as libc::c_int
    }
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"Removed %s\x00" as *const u8 as *const libc::c_char,
                    path.as_mut_ptr());
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_DropRoot(mut uid: uid_t, mut gid: gid_t) {
    if let Err(e) = drop_root(uid as u32, gid as u32) {
        LOG_Message(LOGS_FATAL, format!("{}", e).as_ptr() as *const i8);
        exit(1 as libc::c_int);
    }
}

pub fn drop_root(uid: u32, gid: u32) -> Result<(), anyhow::Error> {
    nix::unistd::setgroups(&[nix::unistd::Gid::from_raw(0)])
        .context("setgroups() failed")?;
    nix::unistd::setgid(nix::unistd::Gid::from_raw(gid))
        .with_context(|| format!("setgid({}) failed", gid))?;
    nix::unistd::setuid(nix::unistd::Uid::from_raw(uid))
        .with_context(|| format!("setuid({}) failed", uid))?;
    debug!("Dropped root privileges: UID {} GID {}", uid, gid);
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn UTI_GetRandomBytesUrandom(mut buf: *mut libc::c_void, mut len: libc::c_uint) {
    let bytes = get_random_bytes(len as usize);
    std::ptr::copy(bytes.as_ptr() as *const libc::c_void, buf as *mut libc::c_void, len as usize);
}

#[test]
fn test_get_random_bytes_urandom() {
    let mut buf = vec![0; 100];

    unsafe {
        UTI_GetRandomBytesUrandom(buf.as_mut_ptr() as *mut libc::c_void, buf.len() as libc::c_uint);
    }

    // Just test that at least one bit isn't zero. Testing randomness is hard.
    assert!(buf.iter().any(|&e| e != 0));
}

pub fn get_random_bytes(len: usize) -> Vec<u8> {
    let mut v = vec![0; len];
    OsRng.fill_bytes(&mut v);
    v
}

/* ================================================== */
unsafe extern "C" fn get_random_bytes_getrandom(mut buf: *mut libc::c_char,
                                                mut len: libc::c_uint) {
    static mut rand_buf: [libc::c_char; 256] = [0; 256];
    static mut available: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    static mut disabled: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < len {
        if available == 0 {
            if disabled != 0 { break ; }
            if getrandom(rand_buf.as_mut_ptr() as *mut libc::c_void,
                         ::std::mem::size_of::<[libc::c_char; 256]>() as
                             libc::c_ulong,
                         0x1 as libc::c_int as libc::c_uint) as libc::c_ulong
                   !=
                   ::std::mem::size_of::<[libc::c_char; 256]>() as
                       libc::c_ulong {
                disabled = 1 as libc::c_int as libc::c_uint;
                break ;
            } else {
                available =
                    ::std::mem::size_of::<[libc::c_char; 256]>() as
                        libc::c_ulong as libc::c_uint
            }
        }
        available = available.wrapping_sub(1);
        *buf.offset(i as isize) = rand_buf[available as usize];
        i = i.wrapping_add(1)
    }
    if i < len { UTI_GetRandomBytesUrandom(buf as *mut libc::c_void, len); };
}
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
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

  Various utility functions
  */
/* Zero a timespec */
/* Check if a timespec is zero */
/* Convert a timeval into a timespec */
/* Convert a timespec into a timeval */
/* Convert a timespec into a floating point number of seconds */
/* Convert a number of seconds expressed in floating point into a
   timespec */
/* Normalise a timespec, by adding or subtracting seconds to bring
   its nanosecond field into range */
/* Convert a timeval into a floating point number of seconds */
/* Convert a number of seconds expressed in floating point into a
   timeval */
/* Normalise a struct timeval, by adding or subtracting seconds to bring
   its microseconds field into range */
/* Returns -1 if a comes earlier than b, 0 if a is the same time as b,
   and +1 if a comes after b */
/* Calculate result = a - b */
/* Calculate result = a - b and return as a double */
/* Add a double increment to a timespec to get a new one. 'start' is
   the starting time, 'end' is the result that we return.  This is
   safe to use if start and end are the same */
/* Calculate the average and difference (as a double) of two timespecs */
/* Calculate result = a - b + c */
/* Convert a timespec into a temporary string, largely for diagnostic
   display */
/* Convert an NTP timestamp into a temporary string, largely for
   diagnostic display */
/* Convert ref_id into a temporary string, for diagnostics */
/* Convert an IP address to string, for diagnostics */
/* Adjust time following a frequency/offset change */
/* Get zero NTP timestamp with random bits below precision */
/* Zero an NTP timestamp */
/* Check if an NTP timestamp is zero */
/* Compare two NTP timestamps.  Returns -1 if a is before b, 0 if a is equal to
   b, and 1 if a is after b. */
/* Compare an NTP timestamp with up to three other timestamps.  Returns 0
   if a is not equal to any of b1, b2, and b3, 1 otherwise. */
/* Convert a timespec into an NTP timestamp */
/* Convert an NTP timestamp into a timespec */
/* Check if time + offset is sane */
/* Get 2 raised to power of a signed integer */
/* Set FD_CLOEXEC on descriptor */
/* Get directory (as an allocated string) for a path */
/* Create a directory with a specified mode (umasked) and set its uid/gid.
   Create also any parent directories that don't exist with mode 755 and
   default uid/gid.  Returns 1 if created or already exists (even with
   different mode/uid/gid), 0 otherwise. */
/* Check if a directory is secure.  It must not have other than the specified
   permissions and its uid/gid must match the specified values. */
/* Open a file.  The full path of the file is constructed from the basedir
   (may be NULL), '/' (if basedir is not NULL), name, and suffix (may be NULL).
   Created files have specified permissions (umasked).  Returns NULL on error.
   The following modes are supported (if the mode is an uppercase character,
   errors are fatal):
   r/R - open an existing file for reading
   w/W - open a new file for writing (remove existing file)
   a/A - open an existing file for appending (create if does not exist) */
/* Rename a temporary file by changing its suffix.  The paths are constructed as
   in UTI_OpenFile().  If the renaming fails, the file will be removed. */
/* Remove a file.  The path is constructed as in UTI_OpenFile(). */
/* Set process user/group IDs and drop supplementary groups */
/* Fill buffer with random bytes from /dev/urandom */
/* Fill buffer with random bytes from /dev/urandom or a faster source if it's
   available (e.g. arc4random()), which may not necessarily be suitable for
   generating long-term keys */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn UTI_GetRandomBytes(mut buf: *mut libc::c_void,
                                            mut len: libc::c_uint) {
    get_random_bytes_getrandom(buf as *mut libc::c_char, len);
}
