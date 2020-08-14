#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]

use c2rust_bitfields::BitfieldStruct;
use caps;
use libc;
use log::debug;

use super::util;

extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn uname(__name: *mut utsname) -> libc::c_int;
    /*
     chronyd/chronyc - Programs for keeping computer clocks accurate.

    **********************************************************************
    * Copyright (C) Miroslav Lichvar  2015
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

     Header file for a driver based on the adjtimex()/ntp_adjtime() function
     */
    /* Initialise with some driver functions replaced with special versions */
    /* Wrapper for adjtimex()/ntp_adjtime() */
    #[no_mangle]
    fn SYS_Timex_Adjust(txc: *mut timex, ignore_error: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SYS_Timex_Finalise();
    #[no_mangle]
    fn SYS_Timex_InitialiseWithFunctions(
        max_set_freq_ppm: libc::c_double,
        max_set_freq_delay: libc::c_double,
        sys_read_freq: lcl_ReadFrequencyDriver,
        sys_set_freq: lcl_SetFrequencyDriver,
        sys_apply_step_offset: lcl_ApplyStepOffsetDriver,
        min_fastslew_offset: libc::c_double,
        max_fastslew_rate: libc::c_double,
        sys_accrue_offset: lcl_AccrueOffsetDriver,
        sys_get_offset_correction: lcl_OffsetCorrectionDriver,
    );
    /* Routine to read the system precision in terms of the actual time step */
    #[no_mangle]
    fn LCL_GetSysPrecisionAsQuantum() -> libc::c_double;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn UTI_AddDoubleToTimespec(start: *mut timespec, increment: libc::c_double, end: *mut timespec);
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec) -> libc::c_double;
    #[no_mangle]
    fn UTI_FdSetCloexec(fd: libc::c_int) -> libc::c_int;
    fn CNF_GetNTPPort() -> libc::c_int;
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
pub type __u32 = libc::c_uint;
pub type __s64 = libc::c_longlong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct timex {
    pub modes: libc::c_uint,
    pub offset: __syscall_slong_t,
    pub freq: __syscall_slong_t,
    pub maxerror: __syscall_slong_t,
    pub esterror: __syscall_slong_t,
    pub status: libc::c_int,
    pub constant: __syscall_slong_t,
    pub precision: __syscall_slong_t,
    pub tolerance: __syscall_slong_t,
    pub time: timeval,
    pub tick: __syscall_slong_t,
    pub ppsfreq: __syscall_slong_t,
    pub jitter: __syscall_slong_t,
    pub shift: libc::c_int,
    pub stabil: __syscall_slong_t,
    pub jitcnt: __syscall_slong_t,
    pub calcnt: __syscall_slong_t,
    pub errcnt: __syscall_slong_t,
    pub stbcnt: __syscall_slong_t,
    pub tai: libc::c_int,
    #[bitfield(name = "c2rust_unnamed", ty = "libc::c_int", bits = "0..=31")]
    #[bitfield(name = "c2rust_unnamed_0", ty = "libc::c_int", bits = "32..=63")]
    #[bitfield(name = "c2rust_unnamed_1", ty = "libc::c_int", bits = "64..=95")]
    #[bitfield(name = "c2rust_unnamed_2", ty = "libc::c_int", bits = "96..=127")]
    #[bitfield(name = "c2rust_unnamed_3", ty = "libc::c_int", bits = "128..=159")]
    #[bitfield(name = "c2rust_unnamed_4", ty = "libc::c_int", bits = "160..=191")]
    #[bitfield(name = "c2rust_unnamed_5", ty = "libc::c_int", bits = "192..=223")]
    #[bitfield(name = "c2rust_unnamed_6", ty = "libc::c_int", bits = "224..=255")]
    #[bitfield(name = "c2rust_unnamed_7", ty = "libc::c_int", bits = "256..=287")]
    #[bitfield(name = "c2rust_unnamed_8", ty = "libc::c_int", bits = "288..=319")]
    #[bitfield(name = "c2rust_unnamed_9", ty = "libc::c_int", bits = "320..=351")]
    pub c2rust_unnamed_c2rust_unnamed_0_c2rust_unnamed_1_c2rust_unnamed_2_c2rust_unnamed_3_c2rust_unnamed_4_c2rust_unnamed_5_c2rust_unnamed_6_c2rust_unnamed_7_c2rust_unnamed_8_c2rust_unnamed_9:
        [u8; 44],
}
pub type C2RustUnnamed = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptp_clock_time {
    pub sec: __s64,
    pub nsec: __u32,
    pub reserved: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptp_clock_caps {
    pub max_adj: libc::c_int,
    pub n_alarm: libc::c_int,
    pub n_ext_ts: libc::c_int,
    pub n_per_out: libc::c_int,
    pub pps: libc::c_int,
    pub n_pins: libc::c_int,
    pub cross_timestamping: libc::c_int,
    pub rsv: [libc::c_int; 13],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptp_extts_request {
    pub index: libc::c_uint,
    pub flags: libc::c_uint,
    pub rsv: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptp_sys_offset {
    pub n_samples: libc::c_uint,
    pub rsv: [libc::c_uint; 3],
    pub ts: [ptp_clock_time; 51],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptp_sys_offset_extended {
    pub n_samples: libc::c_uint,
    pub rsv: [libc::c_uint; 3],
    pub ts: [[ptp_clock_time; 3]; 25],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptp_sys_offset_precise {
    pub device: ptp_clock_time,
    pub sys_realtime: ptp_clock_time,
    pub sys_monoraw: ptp_clock_time,
    pub rsv: [libc::c_uint; 4],
}
pub type ptp_pin_function = libc::c_uint;
pub const PTP_PF_PHYSYNC: ptp_pin_function = 3;
pub const PTP_PF_PEROUT: ptp_pin_function = 2;
pub const PTP_PF_EXTTS: ptp_pin_function = 1;
pub const PTP_PF_NONE: ptp_pin_function = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptp_pin_desc {
    pub name: [libc::c_char; 64],
    pub index: libc::c_uint,
    pub func: libc::c_uint,
    pub chan: libc::c_uint,
    pub rsv: [libc::c_uint; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptp_extts_event {
    pub t: ptp_clock_time,
    pub index: libc::c_uint,
    pub flags: libc::c_uint,
    pub rsv: [libc::c_uint; 2],
}
/* System driver to apply a step offset. A positive argument means step
the clock forwards. */
/* System driver to convert a raw time to an adjusted (cooked) time.
The number of seconds returned in 'corr' have to be added to the
raw time to get the corrected time */
pub type lcl_OffsetCorrectionDriver = Option<
    unsafe extern "C" fn(_: *mut timespec, _: *mut libc::c_double, _: *mut libc::c_double) -> (),
>;
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

 Private include file for local.c and all system dependent
 driver modules.
 */
/* System driver to read the current local frequency, in ppm relative
to nominal.  A positive value indicates that the local clock runs
fast when uncompensated. */
/* System driver to set the current local frequency, in ppm relative
to nominal.  A positive value indicates that the local clock runs
fast when uncompensated.  Return actual frequency (may be different
from the requested frequency due to clamping or rounding). */
/* System driver to accrue an offset. A positive argument means slew
the clock forwards.  The suggested correction rate of time to correct the
offset is given in 'corr_rate'. */
pub type lcl_AccrueOffsetDriver =
    Option<unsafe extern "C" fn(_: libc::c_double, _: libc::c_double) -> ()>;
pub type lcl_ApplyStepOffsetDriver = Option<unsafe extern "C" fn(_: libc::c_double) -> libc::c_int>;
pub type lcl_SetFrequencyDriver = Option<unsafe extern "C" fn(_: libc::c_double) -> libc::c_double>;
pub type lcl_ReadFrequencyDriver = Option<unsafe extern "C" fn() -> libc::c_double>;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
/* Definitions used if missed in the system headers */
/* This is the uncompensated system tick value */
static mut nominal_tick: libc::c_int = 0;
/* Current tick value */
static mut current_delta_tick: libc::c_int = 0;
/* The maximum amount by which 'tick' can be biased away from 'nominal_tick'
(sys_adjtimex() in the kernel bounds this to 10%) */
static mut max_tick_bias: libc::c_int = 0;
/* The kernel USER_HZ constant */
static mut hz: libc::c_int = 0;
static mut dhz: libc::c_double = 0.;
/* And dbl prec version of same for arithmetic */
/* Flag indicating whether adjtimex() can step the clock */
static mut have_setoffset: libc::c_int = 0;
/* The assumed rate at which the effective frequency and tick values are
updated in the kernel */
static mut tick_update_hz: libc::c_int = 0;
/* ================================================== */
#[inline]
unsafe extern "C" fn our_round(mut x: libc::c_double) -> libc::c_long {
    let mut y: libc::c_long = 0;
    if x > 0.0f64 {
        y = (x + 0.5f64) as libc::c_long
    } else {
        y = (x - 0.5f64) as libc::c_long
    }
    return y;
}
/* ================================================== */
/* Positive means currently fast of true time, i.e. jump backwards */
unsafe extern "C" fn apply_step_offset(mut offset: libc::c_double) -> libc::c_int {
    let mut txc: timex =
        timex{modes: 0,
              offset: 0,
              freq: 0,
              maxerror: 0,
              esterror: 0,
              status: 0,
              constant: 0,
              precision: 0,
              tolerance: 0,
              time: timeval{tv_sec: 0, tv_usec: 0,},
              tick: 0,
              ppsfreq: 0,
              jitter: 0,
              shift: 0,
              stabil: 0,
              jitcnt: 0,
              calcnt: 0,
              errcnt: 0,
              stbcnt: 0,
              tai: 0,
              c2rust_unnamed_c2rust_unnamed_0_c2rust_unnamed_1_c2rust_unnamed_2_c2rust_unnamed_3_c2rust_unnamed_4_c2rust_unnamed_5_c2rust_unnamed_6_c2rust_unnamed_7_c2rust_unnamed_8_c2rust_unnamed_9:
                  [0; 44],};
    txc.modes = (0x100 as libc::c_int | 0x2000 as libc::c_int) as libc::c_uint;
    txc.time.tv_sec = -offset as __time_t;
    txc.time.tv_usec = (1.0e9f64 * (-offset - txc.time.tv_sec as libc::c_double)) as __suseconds_t;
    if txc.time.tv_usec < 0 as libc::c_int as libc::c_long {
        txc.time.tv_sec -= 1;
        txc.time.tv_usec += 1000000000 as libc::c_int as libc::c_long
    }
    if SYS_Timex_Adjust(&mut txc, 1 as libc::c_int) < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
/* This call sets the Linux kernel frequency to a given value in parts
per million relative to the nominal running frequency.  Nominal is taken to
be tick=10000, freq=0 (for a USER_HZ==100 system, other values otherwise).
The convention is that this is called with a positive argument if the local
clock runs fast when uncompensated.  */
unsafe extern "C" fn set_frequency(mut freq_ppm: libc::c_double) -> libc::c_double {
    let mut txc: timex =
        timex{modes: 0,
              offset: 0,
              freq: 0,
              maxerror: 0,
              esterror: 0,
              status: 0,
              constant: 0,
              precision: 0,
              tolerance: 0,
              time: timeval{tv_sec: 0, tv_usec: 0,},
              tick: 0,
              ppsfreq: 0,
              jitter: 0,
              shift: 0,
              stabil: 0,
              jitcnt: 0,
              calcnt: 0,
              errcnt: 0,
              stbcnt: 0,
              tai: 0,
              c2rust_unnamed_c2rust_unnamed_0_c2rust_unnamed_1_c2rust_unnamed_2_c2rust_unnamed_3_c2rust_unnamed_4_c2rust_unnamed_5_c2rust_unnamed_6_c2rust_unnamed_7_c2rust_unnamed_8_c2rust_unnamed_9:
                  [0; 44],};
    let mut required_tick: libc::c_long = 0;
    let mut required_freq: libc::c_double = 0.;
    let mut required_delta_tick: libc::c_int = 0;
    required_delta_tick = our_round(freq_ppm / dhz) as libc::c_int;
    /* Older kernels (pre-2.6.18) don't apply the frequency offset exactly as
    set by adjtimex() and a scaling constant (that depends on the internal
    kernel HZ constant) would be needed to compensate for the error. Because
    chronyd is closed loop it doesn't matter much if we don't scale the
    required frequency, but we want to prevent thrashing between two states
    when the system's frequency error is close to a multiple of USER_HZ.  With
    USER_HZ <= 250, the maximum frequency adjustment of 500 ppm overlaps at
    least two ticks and we can stick to the current tick if it's next to the
    required tick. */
    if hz <= 250 as libc::c_int
        && (required_delta_tick + 1 as libc::c_int == current_delta_tick
            || required_delta_tick - 1 as libc::c_int == current_delta_tick)
    {
        required_delta_tick = current_delta_tick
    }
    required_freq = -(freq_ppm - dhz * required_delta_tick as libc::c_double);
    required_tick = (nominal_tick - required_delta_tick) as libc::c_long;
    txc.modes = (0x4000 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
    txc.freq = (required_freq * ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_double)
        as __syscall_slong_t;
    txc.tick = required_tick;
    SYS_Timex_Adjust(&mut txc, 0 as libc::c_int);
    current_delta_tick = required_delta_tick;
    return dhz * current_delta_tick as libc::c_double
        - txc.freq as libc::c_double / ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_double;
}
/* ================================================== */
/* Read the ppm frequency from the kernel */
unsafe extern "C" fn read_frequency() -> libc::c_double {
    let mut txc: timex =
        timex{modes: 0,
              offset: 0,
              freq: 0,
              maxerror: 0,
              esterror: 0,
              status: 0,
              constant: 0,
              precision: 0,
              tolerance: 0,
              time: timeval{tv_sec: 0, tv_usec: 0,},
              tick: 0,
              ppsfreq: 0,
              jitter: 0,
              shift: 0,
              stabil: 0,
              jitcnt: 0,
              calcnt: 0,
              errcnt: 0,
              stbcnt: 0,
              tai: 0,
              c2rust_unnamed_c2rust_unnamed_0_c2rust_unnamed_1_c2rust_unnamed_2_c2rust_unnamed_3_c2rust_unnamed_4_c2rust_unnamed_5_c2rust_unnamed_6_c2rust_unnamed_7_c2rust_unnamed_8_c2rust_unnamed_9:
                  [0; 44],};
    txc.modes = 0 as libc::c_int as libc::c_uint;
    SYS_Timex_Adjust(&mut txc, 0 as libc::c_int);
    current_delta_tick = (nominal_tick as libc::c_long - txc.tick) as libc::c_int;
    return dhz * current_delta_tick as libc::c_double
        - txc.freq as libc::c_double / ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_double;
}
/* ================================================== */
/* Estimate the value of USER_HZ given the value of txc.tick that chronyd finds when
 * it starts.  The only credible values are 100 (Linux/x86) or powers of 2.
 * Also, the bounds checking inside the kernel's adjtimex system call enforces
 * a +/- 10% movement of tick away from the nominal value 1e6/USER_HZ. */
unsafe extern "C" fn guess_hz() -> libc::c_int {
    let mut txc: timex =
        timex{modes: 0,
              offset: 0,
              freq: 0,
              maxerror: 0,
              esterror: 0,
              status: 0,
              constant: 0,
              precision: 0,
              tolerance: 0,
              time: timeval{tv_sec: 0, tv_usec: 0,},
              tick: 0,
              ppsfreq: 0,
              jitter: 0,
              shift: 0,
              stabil: 0,
              jitcnt: 0,
              calcnt: 0,
              errcnt: 0,
              stbcnt: 0,
              tai: 0,
              c2rust_unnamed_c2rust_unnamed_0_c2rust_unnamed_1_c2rust_unnamed_2_c2rust_unnamed_3_c2rust_unnamed_4_c2rust_unnamed_5_c2rust_unnamed_6_c2rust_unnamed_7_c2rust_unnamed_8_c2rust_unnamed_9:
                  [0; 44],};
    let mut i: libc::c_int = 0;
    let mut tick: libc::c_int = 0;
    let mut tick_lo: libc::c_int = 0;
    let mut tick_hi: libc::c_int = 0;
    let mut ihz: libc::c_int = 0;
    let mut tick_nominal: libc::c_double = 0.;
    txc.modes = 0 as libc::c_int as libc::c_uint;
    SYS_Timex_Adjust(&mut txc, 0 as libc::c_int);
    tick = txc.tick as libc::c_int;
    /* Pick off the hz=100 case first */
    if tick >= 9000 as libc::c_int && tick <= 11000 as libc::c_int {
        return 100 as libc::c_int;
    }
    i = 4 as libc::c_int;
    while i < 16 as libc::c_int {
        /* surely 16 .. 32768 is a wide enough range? */
        ihz = (1 as libc::c_int) << i;
        tick_nominal = 1.0e6f64 / ihz as libc::c_double;
        tick_lo = (0.5f64 + tick_nominal * 2.0f64 / 3.0f64) as libc::c_int;
        tick_hi = (0.5f64 + tick_nominal * 4.0f64 / 3.0f64) as libc::c_int;
        if tick_lo < tick && tick <= tick_hi {
            return ihz;
        }
        i += 1
    }
    /* oh dear.  doomed. */
    LOG_Message(
        LOGS_FATAL,
        b"Can\'t determine hz from tick %d\x00" as *const u8 as *const libc::c_char,
        tick,
    );
    exit(1 as libc::c_int);
}
/* ================================================== */
unsafe extern "C" fn get_hz() -> libc::c_int {
    let mut hz_0: libc::c_int = 0;
    hz_0 = sysconf(_SC_CLK_TCK as libc::c_int) as libc::c_int;
    if hz_0 < 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    return hz_0;
}
/* ================================================== */
unsafe extern "C" fn kernelvercmp(
    mut major1: libc::c_int,
    mut minor1: libc::c_int,
    mut patch1: libc::c_int,
    mut major2: libc::c_int,
    mut minor2: libc::c_int,
    mut patch2: libc::c_int,
) -> libc::c_int {
    if major1 != major2 {
        return major1 - major2;
    }
    if minor1 != minor2 {
        return minor1 - minor2;
    }
    return patch1 - patch2;
}
/* ================================================== */
unsafe extern "C" fn get_kernel_version(
    mut major: *mut libc::c_int,
    mut minor: *mut libc::c_int,
    mut patch: *mut libc::c_int,
) {
    let mut uts: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    if uname(&mut uts) < 0 as libc::c_int {
        LOG_Message(
            LOGS_FATAL,
            b"uname() failed\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    *patch = 0 as libc::c_int;
    if sscanf(
        uts.release.as_mut_ptr(),
        b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
        major,
        minor,
        patch,
    ) < 2 as libc::c_int
    {
        LOG_Message(
            LOGS_FATAL,
            b"Could not parse kernel version\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    };
}
/* ================================================== */
/* Compute the scaling to use on any frequency we set, according to
the vintage of the Linux kernel being used. */
unsafe extern "C" fn get_version_specific_details() {
    let mut major: libc::c_int = 0; /* Mirror declaration in kernel */
    let mut minor: libc::c_int = 0;
    let mut patch: libc::c_int = 0;
    hz = get_hz();
    if hz == 0 {
        hz = guess_hz()
    }
    dhz = hz as libc::c_double;
    nominal_tick = ((1000000 as libc::c_long + (hz / 2 as libc::c_int) as libc::c_long)
        / hz as libc::c_long) as libc::c_int;
    max_tick_bias = nominal_tick / 10 as libc::c_int;
    /* In modern kernels the frequency of the clock is updated immediately in the
    adjtimex() system call.  Assume a maximum delay of 10 microseconds. */
    tick_update_hz = 100000 as libc::c_int;
    get_kernel_version(&mut major, &mut minor, &mut patch);
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"Linux kernel major=%d minor=%d patch=%d\x00" as *const u8 as *const libc::c_char,
            major,
            minor,
            patch,
        );
    }
    if kernelvercmp(
        major,
        minor,
        patch,
        2 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    ) < 0 as libc::c_int
    {
        LOG_Message(
            LOGS_FATAL,
            b"Kernel version not supported, sorry.\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if kernelvercmp(
        major,
        minor,
        patch,
        2 as libc::c_int,
        6 as libc::c_int,
        27 as libc::c_int,
    ) >= 0 as libc::c_int
        && kernelvercmp(
            major,
            minor,
            patch,
            2 as libc::c_int,
            6 as libc::c_int,
            33 as libc::c_int,
        ) < 0 as libc::c_int
    {
        /* In tickless kernels before 2.6.33 the frequency is updated in
        a half-second interval */
        tick_update_hz = 2 as libc::c_int
    } else if kernelvercmp(
        major,
        minor,
        patch,
        4 as libc::c_int,
        19 as libc::c_int,
        0 as libc::c_int,
    ) < 0 as libc::c_int
    {
        /* In kernels before 4.19 the frequency is updated only on internal ticks
        (CONFIG_HZ).  As their rate cannot be reliably detected from the user
        space, and it may not even be constant (CONFIG_NO_HZ - aka tickless),
        assume the lowest commonly used constant rate */
        tick_update_hz = 100 as libc::c_int
    }
    /* ADJ_SETOFFSET support */
    if kernelvercmp(
        major,
        minor,
        patch,
        2 as libc::c_int,
        6 as libc::c_int,
        39 as libc::c_int,
    ) < 0 as libc::c_int
    {
        have_setoffset = 0 as libc::c_int
    } else {
        have_setoffset = 1 as libc::c_int
    }
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"hz=%d nominal_tick=%d max_tick_bias=%d tick_update_hz=%d\x00" as *const u8
                as *const libc::c_char,
            hz,
            nominal_tick,
            max_tick_bias,
            tick_update_hz,
        );
    };
}
/* ================================================== */
unsafe extern "C" fn reset_adjtime_offset() {
    let mut txc: timex =
        timex{modes: 0,
              offset: 0,
              freq: 0,
              maxerror: 0,
              esterror: 0,
              status: 0,
              constant: 0,
              precision: 0,
              tolerance: 0,
              time: timeval{tv_sec: 0, tv_usec: 0,},
              tick: 0,
              ppsfreq: 0,
              jitter: 0,
              shift: 0,
              stabil: 0,
              jitcnt: 0,
              calcnt: 0,
              errcnt: 0,
              stbcnt: 0,
              tai: 0,
              c2rust_unnamed_c2rust_unnamed_0_c2rust_unnamed_1_c2rust_unnamed_2_c2rust_unnamed_3_c2rust_unnamed_4_c2rust_unnamed_5_c2rust_unnamed_6_c2rust_unnamed_7_c2rust_unnamed_8_c2rust_unnamed_9:
                  [0; 44],};
    /* Reset adjtime() offset */
    txc.modes = 0x8001 as libc::c_int as libc::c_uint;
    txc.offset = 0 as libc::c_int as __syscall_slong_t;
    SYS_Timex_Adjust(&mut txc, 0 as libc::c_int);
}
/* ================================================== */
unsafe extern "C" fn test_step_offset() -> libc::c_int {
    let mut txc: timex =
        timex{modes: 0,
              offset: 0,
              freq: 0,
              maxerror: 0,
              esterror: 0,
              status: 0,
              constant: 0,
              precision: 0,
              tolerance: 0,
              time: timeval{tv_sec: 0, tv_usec: 0,},
              tick: 0,
              ppsfreq: 0,
              jitter: 0,
              shift: 0,
              stabil: 0,
              jitcnt: 0,
              calcnt: 0,
              errcnt: 0,
              stbcnt: 0,
              tai: 0,
              c2rust_unnamed_c2rust_unnamed_0_c2rust_unnamed_1_c2rust_unnamed_2_c2rust_unnamed_3_c2rust_unnamed_4_c2rust_unnamed_5_c2rust_unnamed_6_c2rust_unnamed_7_c2rust_unnamed_8_c2rust_unnamed_9:
                  [0; 44],};
    /* Zero maxerror and check it's reset to a maximum after ADJ_SETOFFSET.
    This seems to be the only way how to verify that the kernel really
    supports the ADJ_SETOFFSET mode as it doesn't return an error on unknown
    mode. */
    txc.modes = 0x4 as libc::c_int as libc::c_uint;
    txc.maxerror = 0 as libc::c_int as __syscall_slong_t;
    if SYS_Timex_Adjust(&mut txc, 1 as libc::c_int) < 0 as libc::c_int
        || txc.maxerror != 0 as libc::c_int as libc::c_long
    {
        return 0 as libc::c_int;
    }
    txc.modes = (0x100 as libc::c_int | 0x2000 as libc::c_int) as libc::c_uint;
    txc.time.tv_sec = 0 as libc::c_int as __time_t;
    txc.time.tv_usec = 0 as libc::c_int as __suseconds_t;
    if SYS_Timex_Adjust(&mut txc, 1 as libc::c_int) < 0 as libc::c_int
        || txc.maxerror < 100000 as libc::c_int as libc::c_long
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn report_time_adjust_blockers() {}
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

 The header file for the linux driver
 */
/* ================================================== */
/* Initialisation code for this module */
#[no_mangle]
pub unsafe extern "C" fn SYS_Linux_Initialise() {
    get_version_specific_details();
    report_time_adjust_blockers();
    reset_adjtime_offset();
    if have_setoffset != 0 && test_step_offset() == 0 {
        LOG_Message(
            LOGS_INFO,
            b"adjtimex() doesn\'t support ADJ_SETOFFSET\x00" as *const u8 as *const libc::c_char,
        );
        have_setoffset = 0 as libc::c_int
    }
    SYS_Timex_InitialiseWithFunctions(
        1.0e6f64 * max_tick_bias as libc::c_double / nominal_tick as libc::c_double,
        1.0f64 / tick_update_hz as libc::c_double,
        Some(read_frequency as unsafe extern "C" fn() -> libc::c_double),
        Some(set_frequency as unsafe extern "C" fn(_: libc::c_double) -> libc::c_double),
        if have_setoffset != 0 {
            Some(apply_step_offset as unsafe extern "C" fn(_: libc::c_double) -> libc::c_int)
        } else {
            None
        },
        0.0f64,
        0.0f64,
        None,
        None,
    );
}
/* ================================================== */
/* Finalisation code for this module */
#[no_mangle]
pub unsafe extern "C" fn SYS_Linux_Finalise() {
    SYS_Timex_Finalise();
}
/* ================================================== */
/* ================================================== */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Linux_CheckKernelVersion(
    mut req_major: libc::c_int,
    mut req_minor: libc::c_int,
) -> libc::c_int {
    let mut major: libc::c_int = 0;
    let mut minor: libc::c_int = 0;
    let mut patch: libc::c_int = 0;
    get_kernel_version(&mut major, &mut minor, &mut patch);
    return (kernelvercmp(req_major, req_minor, 0 as libc::c_int, major, minor, patch)
        <= 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn process_phc_readings(
    mut ts: *mut [timespec; 3],
    mut n: libc::c_int,
    mut precision: libc::c_double,
    mut phc_ts: *mut timespec,
    mut sys_ts: *mut timespec,
    mut err: *mut libc::c_double,
) -> libc::c_int {
    let mut min_delay: libc::c_double = 0.0f64;
    let mut delays: [libc::c_double; 25] = [0.; 25];
    let mut phc_sum: libc::c_double = 0.;
    let mut sys_sum: libc::c_double = 0.;
    let mut sys_prec: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut combined: libc::c_int = 0;
    if n > 25 as libc::c_int {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < n {
        delays[i as usize] = UTI_DiffTimespecsToDouble(
            &mut *(*ts.offset(i as isize))
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize),
            &mut *(*ts.offset(i as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
        );
        if delays[i as usize] < 0.0f64 {
            /* Step in the middle of a PHC reading? */
            if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
            {
                LOG_Message(
                    LOGS_DEBUG,
                    b"Bad PTP_SYS_OFFSET sample delay=%e\x00" as *const u8 as *const libc::c_char,
                    delays[i as usize],
                );
            }
            return 0 as libc::c_int;
        }
        if i == 0 || delays[i as usize] < min_delay {
            min_delay = delays[i as usize]
        }
        i += 1
    }
    sys_prec = LCL_GetSysPrecisionAsQuantum();
    /* Combine best readings */
    combined = 0 as libc::c_int;
    i = combined;
    sys_sum = 0.0f64;
    phc_sum = sys_sum;
    while i < n {
        if !(delays[i as usize]
            > min_delay
                + (if sys_prec > precision {
                    sys_prec
                } else {
                    precision
                }))
        {
            phc_sum += UTI_DiffTimespecsToDouble(
                &mut *(*ts.offset(i as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize),
                &mut *(*ts.offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize),
            );
            sys_sum += UTI_DiffTimespecsToDouble(
                &mut *(*ts.offset(i as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize),
                &mut *(*ts.offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize),
            ) + delays[i as usize] / 2.0f64;
            combined += 1
        }
        i += 1
    }
    if combined != 0 {
    } else {
        __assert_fail(b"combined\x00" as *const u8 as *const libc::c_char,
                      b"sys_linux.c\x00" as *const u8 as *const libc::c_char,
                      678 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 110],
                                                &[libc::c_char; 110]>(b"int process_phc_readings(struct timespec (*)[3], int, double, struct timespec *, struct timespec *, double *)\x00")).as_ptr());
    }
    UTI_AddDoubleToTimespec(
        &mut *(*ts.offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
        phc_sum / combined as libc::c_double,
        phc_ts,
    );
    UTI_AddDoubleToTimespec(
        &mut *(*ts.offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize),
        sys_sum / combined as libc::c_double,
        sys_ts,
    );
    *err = if min_delay / 2.0f64 > precision {
        (min_delay) / 2.0f64
    } else {
        precision
    };
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn get_phc_sample(
    mut phc_fd: libc::c_int,
    mut precision: libc::c_double,
    mut phc_ts: *mut timespec,
    mut sys_ts: *mut timespec,
    mut err: *mut libc::c_double,
) -> libc::c_int {
    let mut ts: [[timespec; 3]; 10] = [[timespec {
        tv_sec: 0,
        tv_nsec: 0,
    }; 3]; 10];
    let mut sys_off: ptp_sys_offset = ptp_sys_offset {
        n_samples: 0,
        rsv: [0; 3],
        ts: [ptp_clock_time {
            sec: 0,
            nsec: 0,
            reserved: 0,
        }; 51],
    };
    let mut i: libc::c_int = 0;
    /* Silence valgrind */
    memset(
        &mut sys_off as *mut ptp_sys_offset as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ptp_sys_offset>() as libc::c_ulong,
    );
    sys_off.n_samples = 10 as libc::c_int as libc::c_uint;
    if ioctl(
        phc_fd,
        ((1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('=' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((5 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<ptp_sys_offset>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut sys_off as *mut ptp_sys_offset,
    ) != 0
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"ioctl(%s) failed : %s\x00" as *const u8 as *const libc::c_char,
                b"PTP_SYS_OFFSET\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        ts[i as usize][0 as libc::c_int as usize].tv_sec =
            sys_off.ts[(i * 2 as libc::c_int) as usize].sec as __time_t;
        ts[i as usize][0 as libc::c_int as usize].tv_nsec =
            sys_off.ts[(i * 2 as libc::c_int) as usize].nsec as __syscall_slong_t;
        ts[i as usize][1 as libc::c_int as usize].tv_sec =
            sys_off.ts[(i * 2 as libc::c_int + 1 as libc::c_int) as usize].sec as __time_t;
        ts[i as usize][1 as libc::c_int as usize].tv_nsec = sys_off.ts
            [(i * 2 as libc::c_int + 1 as libc::c_int) as usize]
            .nsec as __syscall_slong_t;
        ts[i as usize][2 as libc::c_int as usize].tv_sec =
            sys_off.ts[(i * 2 as libc::c_int + 2 as libc::c_int) as usize].sec as __time_t;
        ts[i as usize][2 as libc::c_int as usize].tv_nsec = sys_off.ts
            [(i * 2 as libc::c_int + 2 as libc::c_int) as usize]
            .nsec as __syscall_slong_t;
        i += 1
    }
    return process_phc_readings(
        ts.as_mut_ptr(),
        10 as libc::c_int,
        precision,
        phc_ts,
        sys_ts,
        err,
    );
}
/* ================================================== */
unsafe extern "C" fn get_extended_phc_sample(
    mut phc_fd: libc::c_int,
    mut precision: libc::c_double,
    mut phc_ts: *mut timespec,
    mut sys_ts: *mut timespec,
    mut err: *mut libc::c_double,
) -> libc::c_int {
    let mut ts: [[timespec; 3]; 10] = [[timespec {
        tv_sec: 0,
        tv_nsec: 0,
    }; 3]; 10];
    let mut sys_off: ptp_sys_offset_extended = ptp_sys_offset_extended {
        n_samples: 0,
        rsv: [0; 3],
        ts: [[ptp_clock_time {
            sec: 0,
            nsec: 0,
            reserved: 0,
        }; 3]; 25],
    };
    let mut i: libc::c_int = 0;
    /* Silence valgrind */
    memset(
        &mut sys_off as *mut ptp_sys_offset_extended as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ptp_sys_offset_extended>() as libc::c_ulong,
    );
    sys_off.n_samples = 10 as libc::c_int as libc::c_uint;
    if ioctl(
        phc_fd,
        ((2 as libc::c_uint | 1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('=' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((9 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<ptp_sys_offset_extended>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut sys_off as *mut ptp_sys_offset_extended,
    ) != 0
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"ioctl(%s) failed : %s\x00" as *const u8 as *const libc::c_char,
                b"PTP_SYS_OFFSET_EXTENDED\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        ts[i as usize][0 as libc::c_int as usize].tv_sec =
            sys_off.ts[i as usize][0 as libc::c_int as usize].sec as __time_t;
        ts[i as usize][0 as libc::c_int as usize].tv_nsec =
            sys_off.ts[i as usize][0 as libc::c_int as usize].nsec as __syscall_slong_t;
        ts[i as usize][1 as libc::c_int as usize].tv_sec =
            sys_off.ts[i as usize][1 as libc::c_int as usize].sec as __time_t;
        ts[i as usize][1 as libc::c_int as usize].tv_nsec =
            sys_off.ts[i as usize][1 as libc::c_int as usize].nsec as __syscall_slong_t;
        ts[i as usize][2 as libc::c_int as usize].tv_sec =
            sys_off.ts[i as usize][2 as libc::c_int as usize].sec as __time_t;
        ts[i as usize][2 as libc::c_int as usize].tv_nsec =
            sys_off.ts[i as usize][2 as libc::c_int as usize].nsec as __syscall_slong_t;
        i += 1
    }
    return process_phc_readings(
        ts.as_mut_ptr(),
        10 as libc::c_int,
        precision,
        phc_ts,
        sys_ts,
        err,
    );
}
/* ================================================== */
unsafe extern "C" fn get_precise_phc_sample(
    mut phc_fd: libc::c_int,
    mut precision: libc::c_double,
    mut phc_ts: *mut timespec,
    mut sys_ts: *mut timespec,
    mut err: *mut libc::c_double,
) -> libc::c_int {
    let mut sys_off: ptp_sys_offset_precise = ptp_sys_offset_precise {
        device: ptp_clock_time {
            sec: 0,
            nsec: 0,
            reserved: 0,
        },
        sys_realtime: ptp_clock_time {
            sec: 0,
            nsec: 0,
            reserved: 0,
        },
        sys_monoraw: ptp_clock_time {
            sec: 0,
            nsec: 0,
            reserved: 0,
        },
        rsv: [0; 4],
    };
    /* Silence valgrind */
    memset(
        &mut sys_off as *mut ptp_sys_offset_precise as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ptp_sys_offset_precise>() as libc::c_ulong,
    );
    if ioctl(
        phc_fd,
        ((2 as libc::c_uint | 1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('=' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((8 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<ptp_sys_offset_precise>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut sys_off as *mut ptp_sys_offset_precise,
    ) != 0
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"ioctl(%s) failed : %s\x00" as *const u8 as *const libc::c_char,
                b"PTP_SYS_OFFSET_PRECISE\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    (*phc_ts).tv_sec = sys_off.device.sec as __time_t;
    (*phc_ts).tv_nsec = sys_off.device.nsec as __syscall_slong_t;
    (*sys_ts).tv_sec = sys_off.sys_realtime.sec as __time_t;
    (*sys_ts).tv_nsec = sys_off.sys_realtime.nsec as __syscall_slong_t;
    *err = if LCL_GetSysPrecisionAsQuantum() > precision {
        LCL_GetSysPrecisionAsQuantum()
    } else {
        precision
    };
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Linux_OpenPHC(
    mut path: *const libc::c_char,
    mut phc_index: libc::c_int,
) -> libc::c_int {
    let mut caps: ptp_clock_caps = ptp_clock_caps {
        max_adj: 0,
        n_alarm: 0,
        n_ext_ts: 0,
        n_per_out: 0,
        pps: 0,
        n_pins: 0,
        cross_timestamping: 0,
        rsv: [0; 13],
    };
    let mut phc_path: [libc::c_char; 64] = [0; 64];
    let mut phc_fd: libc::c_int = 0;
    if path.is_null() {
        if snprintf(
            phc_path.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"/dev/ptp%d\x00" as *const u8 as *const libc::c_char,
            phc_index,
        ) as libc::c_ulong
            >= ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        path = phc_path.as_mut_ptr()
    }
    phc_fd = open(path, 0 as libc::c_int);
    if phc_fd < 0 as libc::c_int {
        LOG_Message(
            LOGS_ERR,
            b"Could not open %s : %s\x00" as *const u8 as *const libc::c_char,
            path,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    /* Make sure it is a PHC */
    if ioctl(
        phc_fd,
        ((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('=' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<ptp_clock_caps>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut caps as *mut ptp_clock_caps,
    ) != 0
    {
        LOG_Message(
            LOGS_ERR,
            b"ioctl(%s) failed : %s\x00" as *const u8 as *const libc::c_char,
            b"PTP_CLOCK_GETCAPS\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(phc_fd);
        return -(1 as libc::c_int);
    }
    UTI_FdSetCloexec(phc_fd);
    return phc_fd;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Linux_GetPHCSample(
    mut fd: libc::c_int,
    mut nocrossts: libc::c_int,
    mut precision: libc::c_double,
    mut reading_mode: *mut libc::c_int,
    mut phc_ts: *mut timespec,
    mut sys_ts: *mut timespec,
    mut err: *mut libc::c_double,
) -> libc::c_int {
    if (*reading_mode == 2 as libc::c_int || *reading_mode == 0)
        && nocrossts == 0
        && get_precise_phc_sample(fd, precision, phc_ts, sys_ts, err) != 0
    {
        *reading_mode = 2 as libc::c_int;
        return 1 as libc::c_int;
    } else {
        if (*reading_mode == 3 as libc::c_int || *reading_mode == 0)
            && get_extended_phc_sample(fd, precision, phc_ts, sys_ts, err) != 0
        {
            *reading_mode = 3 as libc::c_int;
            return 1 as libc::c_int;
        } else {
            if (*reading_mode == 1 as libc::c_int || *reading_mode == 0)
                && get_phc_sample(fd, precision, phc_ts, sys_ts, err) != 0
            {
                *reading_mode = 1 as libc::c_int;
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Linux_SetPHCExtTimestamping(
    mut fd: libc::c_int,
    mut pin: libc::c_int,
    mut channel: libc::c_int,
    mut rising: libc::c_int,
    mut falling: libc::c_int,
    mut enable: libc::c_int,
) -> libc::c_int {
    let mut extts_req: ptp_extts_request = ptp_extts_request {
        index: 0,
        flags: 0,
        rsv: [0; 2],
    };
    let mut pin_desc: ptp_pin_desc = ptp_pin_desc {
        name: [0; 64],
        index: 0,
        func: 0,
        chan: 0,
        rsv: [0; 5],
    };
    memset(
        &mut pin_desc as *mut ptp_pin_desc as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ptp_pin_desc>() as libc::c_ulong,
    );
    pin_desc.index = pin as libc::c_uint;
    pin_desc.func = if enable != 0 {
        PTP_PF_EXTTS as libc::c_int
    } else {
        PTP_PF_NONE as libc::c_int
    } as libc::c_uint;
    pin_desc.chan = channel as libc::c_uint;
    if ioctl(
        fd,
        ((1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('=' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((7 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<ptp_pin_desc>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut pin_desc as *mut ptp_pin_desc,
    ) != 0
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"ioctl(%s) failed : %s\x00" as *const u8 as *const libc::c_char,
                b"PTP_PIN_SETFUNC\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    memset(
        &mut extts_req as *mut ptp_extts_request as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ptp_extts_request>() as libc::c_ulong,
    );
    extts_req.index = channel as libc::c_uint;
    extts_req.flags = ((if enable != 0 {
        (1 as libc::c_int) << 0 as libc::c_int
    } else {
        0 as libc::c_int
    }) | (if rising != 0 {
        (1 as libc::c_int) << 1 as libc::c_int
    } else {
        0 as libc::c_int
    }) | (if falling != 0 {
        (1 as libc::c_int) << 2 as libc::c_int
    } else {
        0 as libc::c_int
    })) as libc::c_uint;
    if ioctl(
        fd,
        ((1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('=' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((2 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<ptp_extts_request>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut extts_req as *mut ptp_extts_request,
    ) != 0
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"ioctl(%s) failed : %s\x00" as *const u8 as *const libc::c_char,
                b"PTP_EXTTS_REQUEST\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Linux_ReadPHCExtTimestamp(
    mut fd: libc::c_int,
    mut phc_ts: *mut timespec,
    mut channel: *mut libc::c_int,
) -> libc::c_int {
    let mut extts_event: ptp_extts_event = ptp_extts_event {
        t: ptp_clock_time {
            sec: 0,
            nsec: 0,
            reserved: 0,
        },
        index: 0,
        flags: 0,
        rsv: [0; 2],
    };
    if read(
        fd,
        &mut extts_event as *mut ptp_extts_event as *mut libc::c_void,
        ::std::mem::size_of::<ptp_extts_event>() as libc::c_ulong,
    ) as libc::c_ulong
        != ::std::mem::size_of::<ptp_extts_event>() as libc::c_ulong
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Could not read PHC extts event\x00" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    (*phc_ts).tv_sec = extts_event.t.sec as __time_t;
    (*phc_ts).tv_nsec = extts_event.t.nsec as __syscall_slong_t;
    *channel = extts_event.index as libc::c_int;
    return 1 as libc::c_int;
}

pub fn drop_root(uid: u32, gid: u32, clock_control: bool) {
    let ret = unsafe { libc::prctl(libc::PR_SET_KEEPCAPS, 1, 0, 0, 0) };
    if ret != 0 {
        panic!("prctl failed: {}", ret);
    }

    util::drop_root(uid, gid).unwrap();

    let mut caps = caps::CapsHashSet::new();
    if unsafe { CNF_GetNTPPort() != 0 } {
        caps.insert(caps::Capability::CAP_NET_BIND_SERVICE);
    }
    if clock_control {
        caps.insert(caps::Capability::CAP_SYS_TIME);
    }
    if caps.len() > 0 {
        debug!("Setting capabilities to: {:?}", caps);
        caps::set(None, caps::CapSet::Permitted, caps.clone()).unwrap();
        caps::set(None, caps::CapSet::Effective, caps).unwrap();
    }
}
