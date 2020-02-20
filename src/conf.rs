#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           register_tool)]
extern "C" {
    pub type dirent;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ARR_Instance_Record;
    #[no_mangle]
    fn glob(__pattern: *const libc::c_char, __flags: libc::c_int,
            __errfunc:
                Option<unsafe extern "C" fn(_: *const libc::c_char,
                                            _: libc::c_int) -> libc::c_int>,
            __pglob: *mut glob_t) -> libc::c_int;
    #[no_mangle]
    fn globfree(__pglob: *mut glob_t);
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    /* Create a new array with given element size */
    #[no_mangle]
    fn ARR_CreateInstance(elem_size: libc::c_uint) -> ARR_Instance;
    /* Destroy the array */
    #[no_mangle]
    fn ARR_DestroyInstance(array: ARR_Instance);
    /* Return pointer to a new element added to the end of the array */
    #[no_mangle]
    fn ARR_GetNewElement(array: ARR_Instance) -> *mut libc::c_void;
    /* Return element with given index */
    #[no_mangle]
    fn ARR_GetElement(array: ARR_Instance, index: libc::c_uint)
     -> *mut libc::c_void;
    /* Add a new element to the end of the array */
    #[no_mangle]
    fn ARR_AppendElement(array: ARR_Instance, element: *mut libc::c_void);
    /* Set the size of the array */
    #[no_mangle]
    fn ARR_SetSize(array: ARR_Instance, size: libc::c_uint);
    /* Return current size of the array */
    #[no_mangle]
    fn ARR_GetSize(array: ARR_Instance) -> libc::c_uint;
    /* Procedure to add a new server or peer source. */
    #[no_mangle]
    fn NSR_AddSource(remote_addr: *mut NTP_Remote_Address,
                     type_0: NTP_Source_Type, params: *mut SourceParameters)
     -> NSR_Status;
    /* Procedure to add a new server, peer source, or pool of servers specified by
   name instead of address.  The name is resolved in exponentially increasing
   intervals until it succeeds or fails with a non-temporary error.  If the
   name is an address, it is equivalent to NSR_AddSource(). */
    #[no_mangle]
    fn NSR_AddSourceByName(name: *mut libc::c_char, port: libc::c_int,
                           pool: libc::c_int, type_0: NTP_Source_Type,
                           params: *mut SourceParameters) -> NSR_Status;
    #[no_mangle]
    fn NCR_AddAccessRestriction(ip_addr: *mut IPAddr,
                                subnet_bits: libc::c_int, allow: libc::c_int,
                                all: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn NCR_AddBroadcastDestination(addr: *mut IPAddr, port: libc::c_ushort,
                                   interval: libc::c_int);
    #[no_mangle]
    fn RCL_AddRefclock(params: *mut RefclockParameters) -> libc::c_int;
    #[no_mangle]
    fn CAM_AddAccessRestriction(ip_addr: *mut IPAddr,
                                subnet_bits: libc::c_int, allow: libc::c_int,
                                all: libc::c_int) -> libc::c_int;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
    /* Maximum number of addresses returned by DNS_Name2IPAddress */
    #[no_mangle]
    fn DNS_Name2IPAddress(name: *const libc::c_char, ip_addrs: *mut IPAddr,
                          max_addrs: libc::c_int) -> DNS_Status;
    #[no_mangle]
    fn Strdup(s: *const libc::c_char) -> *mut libc::c_char;
    /* Parse a command to add an NTP server or peer */
    #[no_mangle]
    fn CPS_ParseNTPSourceAdd(line: *mut libc::c_char,
                             src: *mut CPS_NTP_Source) -> libc::c_int;
    /* Parse a command to enable local reference */
    #[no_mangle]
    fn CPS_ParseLocal(line: *mut libc::c_char, stratum: *mut libc::c_int,
                      orphan: *mut libc::c_int, distance: *mut libc::c_double)
     -> libc::c_int;
    /* Remove extra white-space and comments */
    #[no_mangle]
    fn CPS_NormalizeLine(line: *mut libc::c_char);
    /* Terminate first word and return pointer to the next word */
    #[no_mangle]
    fn CPS_SplitWord(line: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_IPToString(ip: *mut IPAddr) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_StringToIP(addr: *const libc::c_char, ip: *mut IPAddr)
     -> libc::c_int;
    /* Set FD_CLOEXEC on descriptor */
    /* Get directory (as an allocated string) for a path */
    #[no_mangle]
    fn UTI_PathToDir(path: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_CreateDirAndParents(path: *const libc::c_char, mode: mode_t,
                               uid: uid_t, gid: gid_t) -> libc::c_int;
    #[no_mangle]
    fn UTI_CheckDirPermissions(path: *const libc::c_char, perm: mode_t,
                               uid: uid_t, gid: gid_t) -> libc::c_int;
    #[no_mangle]
    fn UTI_OpenFile(basedir: *const libc::c_char, name: *const libc::c_char,
                    suffix: *const libc::c_char, mode: libc::c_char,
                    perm: mode_t) -> *mut FILE;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type __size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub gl_readdir: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                               -> *mut dirent>,
    pub gl_opendir: Option<unsafe extern "C" fn(_: *const libc::c_char)
                               -> *mut libc::c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *mut stat) -> libc::c_int>,
    pub gl_stat: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                             _: *mut stat) -> libc::c_int>,
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uid_t = __uid_t;
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2014
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

  Header file for array functions.
  */
pub type ARR_Instance = *mut ARR_Instance_Record;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPAddr {
    pub addr: C2RustUnnamed,
    pub family: uint16_t,
    pub _pad: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub in4: uint32_t,
    pub in6: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPSockAddr {
    pub ip_addr: IPAddr,
    pub port: uint16_t,
}
pub type NTP_Remote_Address = IPSockAddr;
pub type REF_LeapMode = libc::c_uint;
pub const REF_LeapModeIgnore: REF_LeapMode = 3;
pub const REF_LeapModeStep: REF_LeapMode = 2;
pub const REF_LeapModeSlew: REF_LeapMode = 1;
pub const REF_LeapModeSystem: REF_LeapMode = 0;
pub type AllowDeny = _AllowDeny;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _AllowDeny {
    pub ip: IPAddr,
    pub subnet_bits: libc::c_int,
    pub all: libc::c_int,
    pub allow: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_Broadcast_Destination {
    pub addr: IPAddr,
    pub port: libc::c_ushort,
    pub interval: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefclockParameters {
    pub driver_name: *mut libc::c_char,
    pub driver_parameter: *mut libc::c_char,
    pub driver_poll: libc::c_int,
    pub poll: libc::c_int,
    pub filter_length: libc::c_int,
    pub pps_forced: libc::c_int,
    pub pps_rate: libc::c_int,
    pub min_samples: libc::c_int,
    pub max_samples: libc::c_int,
    pub sel_options: libc::c_int,
    pub max_lock_age: libc::c_int,
    pub stratum: libc::c_int,
    pub tai: libc::c_int,
    pub ref_id: uint32_t,
    pub lock_ref_id: uint32_t,
    pub offset: libc::c_double,
    pub delay: libc::c_double,
    pub precision: libc::c_double,
    pub max_dispersion: libc::c_double,
    pub pulse_width: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_Source {
    pub type_0: NTP_Source_Type,
    pub pool: libc::c_int,
    pub params: CPS_NTP_Source,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPS_NTP_Source {
    pub name: *mut libc::c_char,
    pub port: libc::c_ushort,
    pub params: SourceParameters,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SourceParameters {
    pub minpoll: libc::c_int,
    pub maxpoll: libc::c_int,
    pub connectivity: SRC_Connectivity,
    pub auto_offline: libc::c_int,
    pub presend_minpoll: libc::c_int,
    pub burst: libc::c_int,
    pub iburst: libc::c_int,
    pub min_stratum: libc::c_int,
    pub poll_target: libc::c_int,
    pub version: libc::c_int,
    pub max_sources: libc::c_int,
    pub min_samples: libc::c_int,
    pub max_samples: libc::c_int,
    pub filter_length: libc::c_int,
    pub interleaved: libc::c_int,
    pub sel_options: libc::c_int,
    pub authkey: uint32_t,
    pub max_delay: libc::c_double,
    pub max_delay_ratio: libc::c_double,
    pub max_delay_dev_ratio: libc::c_double,
    pub min_delay: libc::c_double,
    pub asymmetry: libc::c_double,
    pub offset: libc::c_double,
}
pub type SRC_Connectivity = libc::c_uint;
pub const SRC_MAYBE_ONLINE: SRC_Connectivity = 2;
pub const SRC_ONLINE: SRC_Connectivity = 1;
pub const SRC_OFFLINE: SRC_Connectivity = 0;
pub type NTP_Source_Type = libc::c_uint;
pub const NTP_PEER: NTP_Source_Type = 1;
pub const NTP_SERVER: NTP_Source_Type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CNF_HwTsInterface {
    pub name: *mut libc::c_char,
    pub minpoll: libc::c_int,
    pub min_samples: libc::c_int,
    pub max_samples: libc::c_int,
    pub nocrossts: libc::c_int,
    pub rxfilter: CNF_HwTs_RxFilter,
    pub precision: libc::c_double,
    pub tx_comp: libc::c_double,
    pub rx_comp: libc::c_double,
}
pub type CNF_HwTs_RxFilter = libc::c_uint;
pub const CNF_HWTS_RXFILTER_ALL: CNF_HwTs_RxFilter = 3;
pub const CNF_HWTS_RXFILTER_NTP: CNF_HwTs_RxFilter = 2;
pub const CNF_HWTS_RXFILTER_NONE: CNF_HwTs_RxFilter = 1;
pub const CNF_HWTS_RXFILTER_ANY: CNF_HwTs_RxFilter = 0;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
pub const DNS_Success: DNS_Status = 0;
pub type DNS_Status = libc::c_uint;
pub const DNS_Failure: DNS_Status = 2;
pub const DNS_TryAgain: DNS_Status = 1;
pub type NSR_Status = libc::c_uint;
pub const NSR_UnresolvedName: NSR_Status = 6;
pub const NSR_InvalidName: NSR_Status = 5;
pub const NSR_InvalidAF: NSR_Status = 4;
pub const NSR_TooManySources: NSR_Status = 3;
pub const NSR_AlreadyInUse: NSR_Status = 2;
pub const NSR_NoSuchSource: NSR_Status = 1;
pub const NSR_Success: NSR_Status = 0;
/* 1 to override existing more specific defns */
/* 0 for deny, 1 for allow */
/* ================================================== */
/* Configuration variables */
static mut restarted: libc::c_int = 0 as libc::c_int;
static mut rtc_device: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut acquisition_port: libc::c_int = -(1 as libc::c_int);
static mut ntp_port: libc::c_int = 123 as libc::c_int;
static mut keys_file: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut drift_file: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut rtc_file: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut max_update_skew: libc::c_double = 1000.0f64;
static mut correction_time_ratio: libc::c_double = 3.0f64;
static mut max_clock_error: libc::c_double = 1.0f64;
/* in ppm */
static mut max_drift: libc::c_double = 500000.0f64;
/* in ppm */
static mut max_slew_rate: libc::c_double = 1e6f64 / 12.0f64;
/* in ppm */
static mut max_distance: libc::c_double = 3.0f64;
static mut max_jitter: libc::c_double = 1.0f64;
static mut reselect_distance: libc::c_double = 1e-4f64;
static mut stratum_weight: libc::c_double = 1e-3f64;
static mut combine_limit: libc::c_double = 3.0f64;
static mut cmd_port: libc::c_int = 323 as libc::c_int;
static mut raw_measurements: libc::c_int = 0 as libc::c_int;
static mut do_log_measurements: libc::c_int = 0 as libc::c_int;
static mut do_log_statistics: libc::c_int = 0 as libc::c_int;
static mut do_log_tracking: libc::c_int = 0 as libc::c_int;
static mut do_log_rtc: libc::c_int = 0 as libc::c_int;
static mut do_log_refclocks: libc::c_int = 0 as libc::c_int;
static mut do_log_tempcomp: libc::c_int = 0 as libc::c_int;
static mut log_banner: libc::c_int = 32 as libc::c_int;
static mut logdir: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut dumpdir: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut enable_local: libc::c_int = 0 as libc::c_int;
static mut local_stratum: libc::c_int = 0;
static mut local_orphan: libc::c_int = 0;
static mut local_distance: libc::c_double = 0.;
/* Threshold (in seconds) - if absolute value of initial error is less
   than this, slew instead of stepping */
static mut init_slew_threshold: libc::c_double = 0.;
/* Array of IPAddr */
static mut init_sources: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
static mut enable_manual: libc::c_int = 0 as libc::c_int;
/* Flag set if the RTC runs UTC (default is it runs local time
   incl. daylight saving). */
static mut rtc_on_utc: libc::c_int = 0 as libc::c_int;
/* Filename used to read the hwclock(8) LOCAL/UTC setting */
static mut hwclock_file: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* Flag set if the RTC should be automatically synchronised by kernel */
static mut rtc_sync: libc::c_int = 0 as libc::c_int;
/* Limit and threshold for clock stepping */
static mut make_step_limit: libc::c_int = 0 as libc::c_int;
static mut make_step_threshold: libc::c_double = 0.0f64;
/* Threshold for automatic RTC trimming */
static mut rtc_autotrim_threshold: libc::c_double = 0.0f64;
/* Minimum number of selectables sources required to update the clock */
static mut min_sources: libc::c_int = 1 as libc::c_int;
/* Number of updates before offset checking, number of ignored updates
   before exiting and the maximum allowed offset */
static mut max_offset_delay: libc::c_int = -(1 as libc::c_int);
static mut max_offset_ignore: libc::c_int = 0;
static mut max_offset: libc::c_double = 0.;
/* Maximum and minimum number of samples per source */
static mut max_samples: libc::c_int = 0 as libc::c_int;
/* no limit */
static mut min_samples: libc::c_int = 6 as libc::c_int;
/* Threshold for a time adjustment to be logged to syslog */
static mut log_change_threshold: libc::c_double = 1.0f64;
static mut mail_user_on_change: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut mail_change_threshold: libc::c_double = 0.0f64;
/* Flag indicating that we don't want to log clients, e.g. to save
   memory */
static mut no_client_log: libc::c_int = 0 as libc::c_int;
/* Limit memory allocated for the clients log */
static mut client_log_limit: libc::c_ulong =
    524288 as libc::c_int as libc::c_ulong;
/* Minimum and maximum fallback drift intervals */
static mut fb_drift_min: libc::c_int = 0 as libc::c_int;
static mut fb_drift_max: libc::c_int = 0 as libc::c_int;
/* IP addresses for binding the NTP server sockets to.  UNSPEC family means
   INADDR_ANY will be used */
static mut bind_address4: IPAddr =
    IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
static mut bind_address6: IPAddr =
    IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
/* IP addresses for binding the NTP client sockets to.  UNSPEC family means
   INADDR_ANY will be used */
static mut bind_acq_address4: IPAddr =
    IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
static mut bind_acq_address6: IPAddr =
    IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
/* IP addresses for binding the command socket to.  UNSPEC family means
   the loopback address will be used */
static mut bind_cmd_address4: IPAddr =
    IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
static mut bind_cmd_address6: IPAddr =
    IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
/* Path to the Unix domain command socket. */
static mut bind_cmd_path: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* Path to Samba (ntp_signd) socket. */
static mut ntp_signd_socket: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* Rate limiting parameters */
static mut ntp_ratelimit_enabled: libc::c_int = 0 as libc::c_int;
static mut ntp_ratelimit_interval: libc::c_int = 3 as libc::c_int;
static mut ntp_ratelimit_burst: libc::c_int = 8 as libc::c_int;
static mut ntp_ratelimit_leak: libc::c_int = 2 as libc::c_int;
static mut cmd_ratelimit_enabled: libc::c_int = 0 as libc::c_int;
static mut cmd_ratelimit_interval: libc::c_int = -(4 as libc::c_int);
static mut cmd_ratelimit_burst: libc::c_int = 8 as libc::c_int;
static mut cmd_ratelimit_leak: libc::c_int = 2 as libc::c_int;
/* Smoothing constants */
static mut smooth_max_freq: libc::c_double = 0.0f64;
/* in ppm */
static mut smooth_max_wander: libc::c_double = 0.0f64;
/* in ppm/s */
static mut smooth_leap_only: libc::c_int = 0 as libc::c_int;
/* Temperature sensor, update interval and compensation coefficients */
static mut tempcomp_sensor_file: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut tempcomp_point_file: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut tempcomp_interval: libc::c_double = 0.;
static mut tempcomp_T0: libc::c_double = 0.;
static mut tempcomp_k0: libc::c_double = 0.;
static mut tempcomp_k1: libc::c_double = 0.;
static mut tempcomp_k2: libc::c_double = 0.;
static mut sched_priority: libc::c_int = 0 as libc::c_int;
static mut lock_memory: libc::c_int = 0 as libc::c_int;
/* Leap second handling mode */
static mut leapsec_mode: REF_LeapMode = REF_LeapModeSystem;
/* Name of a system timezone containing leap seconds occuring at midnight */
static mut leapsec_tz: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* Name of the user to which will be dropped root privileges. */
static mut user: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* Array of CNF_HwTsInterface */
static mut hwts_interfaces: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
/* Array of NTP_Source */
static mut ntp_sources: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
/* Array of RefclockParameters */
static mut refclock_sources: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
/* Arrays of AllowDeny */
static mut ntp_restrictions: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
static mut cmd_restrictions: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
/* Array of NTP_Broadcast_Destination */
static mut broadcasts: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
/* ================================================== */
/* The line number in the configuration file being processed */
static mut line_number: libc::c_int = 0;
static mut processed_file: *const libc::c_char = 0 as *const libc::c_char;
static mut processed_command: *const libc::c_char = 0 as *const libc::c_char;
/* ================================================== */
unsafe extern "C" fn command_parse_error() {
    LOG_Message(LOGS_FATAL,
                b"Could not parse %s directive at line %d%s%s\x00" as
                    *const u8 as *const libc::c_char, processed_command,
                line_number,
                if !processed_file.is_null() {
                    b" in file \x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                if !processed_file.is_null() {
                    processed_file
                } else { b"\x00" as *const u8 as *const libc::c_char });
    exit(1 as libc::c_int);
}
/* ================================================== */
unsafe extern "C" fn other_parse_error(mut message: *const libc::c_char) {
    LOG_Message(LOGS_FATAL,
                b"%s at line %d%s%s\x00" as *const u8 as *const libc::c_char,
                message, line_number,
                if !processed_file.is_null() {
                    b" in file \x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                if !processed_file.is_null() {
                    processed_file
                } else { b"\x00" as *const u8 as *const libc::c_char });
    exit(1 as libc::c_int);
}
/* ================================================== */
unsafe extern "C" fn get_number_of_args(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut num: libc::c_int = 0 as libc::c_int;
    /* The line is normalized, between arguments is just one space */
    if *line as libc::c_int == ' ' as i32 { line = line.offset(1) }
    if *line != 0 { num += 1 }
    while *line != 0 {
        if *line as libc::c_int == ' ' as i32 { num += 1 }
        line = line.offset(1)
    }
    return num;
}
/* ================================================== */
unsafe extern "C" fn check_number_of_args(mut line: *mut libc::c_char,
                                          mut num: libc::c_int) {
    num -= get_number_of_args(line);
    if num != 0 {
        LOG_Message(LOGS_FATAL,
                    b"%s arguments for %s directive at line %d%s%s\x00" as
                        *const u8 as *const libc::c_char,
                    if num > 0 as libc::c_int {
                        b"Missing\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"Too many\x00" as *const u8 as *const libc::c_char
                    }, processed_command, line_number,
                    if !processed_file.is_null() {
                        b" in file \x00" as *const u8 as *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char },
                    if !processed_file.is_null() {
                        processed_file
                    } else { b"\x00" as *const u8 as *const libc::c_char });
        exit(1 as libc::c_int);
    };
}
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2013-2014
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

  Header file for configuration module
  */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_Initialise(mut r: libc::c_int,
                                        mut client_only: libc::c_int) {
    restarted = r;
    hwts_interfaces =
        ARR_CreateInstance(::std::mem::size_of::<CNF_HwTsInterface>() as
                               libc::c_ulong as libc::c_uint);
    init_sources =
        ARR_CreateInstance(::std::mem::size_of::<IPAddr>() as libc::c_ulong as
                               libc::c_uint);
    ntp_sources =
        ARR_CreateInstance(::std::mem::size_of::<NTP_Source>() as
                               libc::c_ulong as libc::c_uint);
    refclock_sources =
        ARR_CreateInstance(::std::mem::size_of::<RefclockParameters>() as
                               libc::c_ulong as libc::c_uint);
    broadcasts =
        ARR_CreateInstance(::std::mem::size_of::<NTP_Broadcast_Destination>()
                               as libc::c_ulong as libc::c_uint);
    ntp_restrictions =
        ARR_CreateInstance(::std::mem::size_of::<AllowDeny>() as libc::c_ulong
                               as libc::c_uint);
    cmd_restrictions =
        ARR_CreateInstance(::std::mem::size_of::<AllowDeny>() as libc::c_ulong
                               as libc::c_uint);
    dumpdir = Strdup(b"\x00" as *const u8 as *const libc::c_char);
    logdir = Strdup(b"\x00" as *const u8 as *const libc::c_char);
    rtc_device = Strdup(b"/dev/rtc\x00" as *const u8 as *const libc::c_char);
    hwclock_file = Strdup(b"\x00" as *const u8 as *const libc::c_char);
    user = Strdup(b"root\x00" as *const u8 as *const libc::c_char);
    if client_only != 0 {
        ntp_port = 0 as libc::c_int;
        cmd_port = ntp_port;
        bind_cmd_path = Strdup(b"\x00" as *const u8 as *const libc::c_char);
    } else {
        bind_cmd_path =
            Strdup(b"/var/run/chrony/chronyd.sock\x00" as *const u8 as
                       *const libc::c_char);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_Finalise() {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(hwts_interfaces) {
        free((*(ARR_GetElement(hwts_interfaces, i) as
                    *mut CNF_HwTsInterface)).name as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    ARR_DestroyInstance(hwts_interfaces);
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(ntp_sources) {
        free((*(ARR_GetElement(ntp_sources, i) as
                    *mut NTP_Source)).params.name as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    ARR_DestroyInstance(init_sources);
    ARR_DestroyInstance(ntp_sources);
    ARR_DestroyInstance(refclock_sources);
    ARR_DestroyInstance(broadcasts);
    ARR_DestroyInstance(ntp_restrictions);
    ARR_DestroyInstance(cmd_restrictions);
    free(drift_file as *mut libc::c_void);
    free(dumpdir as *mut libc::c_void);
    free(hwclock_file as *mut libc::c_void);
    free(keys_file as *mut libc::c_void);
    free(leapsec_tz as *mut libc::c_void);
    free(logdir as *mut libc::c_void);
    free(bind_cmd_path as *mut libc::c_void);
    free(ntp_signd_socket as *mut libc::c_void);
    free(rtc_device as *mut libc::c_void);
    free(rtc_file as *mut libc::c_void);
    free(user as *mut libc::c_void);
    free(mail_user_on_change as *mut libc::c_void);
    free(tempcomp_sensor_file as *mut libc::c_void);
    free(tempcomp_point_file as *mut libc::c_void);
}
/* ================================================== */
/* Read the configuration file */
#[no_mangle]
pub unsafe extern "C" fn CNF_ReadFile(mut filename: *const libc::c_char) {
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut line: [libc::c_char; 2048] = [0; 2048];
    let mut i: libc::c_int = 0;
    in_0 =
        UTI_OpenFile(0 as *const libc::c_char, filename,
                     0 as *const libc::c_char, 'R' as i32 as libc::c_char,
                     0 as libc::c_int as mode_t);
    i = 1 as libc::c_int;
    while !fgets(line.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 2048]>() as
                     libc::c_ulong as libc::c_int, in_0).is_null() {
        CNF_ParseLine(filename, i, line.as_mut_ptr());
        i += 1
    }
    fclose(in_0);
}
/* ================================================== */
/* Parse one configuration line */
#[no_mangle]
pub unsafe extern "C" fn CNF_ParseLine(mut filename: *const libc::c_char,
                                       mut number: libc::c_int,
                                       mut line: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Set global variables used in error messages */
    processed_file = filename;
    line_number = number;
    /* Remove extra white-space and comments */
    CPS_NormalizeLine(line);
    /* Skip blank lines */
    if *line == 0 { return }
    /* We have a real line, now try to match commands */
    command = line;
    processed_command = command;
    p = CPS_SplitWord(line);
    if strcasecmp(command,
                  b"acquisitionport\x00" as *const u8 as *const libc::c_char)
           == 0 {
        parse_int(p, &mut acquisition_port);
    } else if strcasecmp(command,
                         b"allow\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        parse_allow_deny(p, ntp_restrictions, 1 as libc::c_int);
    } else if strcasecmp(command,
                         b"bindacqaddress\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_bindacqaddress(p);
    } else if strcasecmp(command,
                         b"bindaddress\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_bindaddress(p);
    } else if strcasecmp(command,
                         b"bindcmdaddress\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_bindcmdaddress(p);
    } else if strcasecmp(command,
                         b"broadcast\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        parse_broadcast(p);
    } else if strcasecmp(command,
                         b"clientloglimit\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_clientloglimit(p);
    } else if strcasecmp(command,
                         b"cmdallow\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        parse_allow_deny(p, cmd_restrictions, 1 as libc::c_int);
    } else if strcasecmp(command,
                         b"cmddeny\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        parse_allow_deny(p, cmd_restrictions, 0 as libc::c_int);
    } else if strcasecmp(command,
                         b"cmdport\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        parse_int(p, &mut cmd_port);
    } else if strcasecmp(command,
                         b"cmdratelimit\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_ratelimit(p, &mut cmd_ratelimit_enabled,
                        &mut cmd_ratelimit_interval, &mut cmd_ratelimit_burst,
                        &mut cmd_ratelimit_leak);
    } else if strcasecmp(command,
                         b"combinelimit\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_double(p, &mut combine_limit);
    } else if strcasecmp(command,
                         b"corrtimeratio\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_double(p, &mut correction_time_ratio);
    } else if strcasecmp(command,
                         b"deny\x00" as *const u8 as *const libc::c_char) == 0
     {
        parse_allow_deny(p, ntp_restrictions, 0 as libc::c_int);
    } else if strcasecmp(command,
                         b"driftfile\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        parse_string(p, &mut drift_file);
    } else if strcasecmp(command,
                         b"dumpdir\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        parse_string(p, &mut dumpdir);
    } else if !(strcasecmp(command,
                           b"dumponexit\x00" as *const u8 as
                               *const libc::c_char) == 0) {
        if strcasecmp(command,
                      b"fallbackdrift\x00" as *const u8 as
                          *const libc::c_char) == 0 {
            parse_fallbackdrift(p);
        } else if strcasecmp(command,
                             b"hwclockfile\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut hwclock_file);
        } else if strcasecmp(command,
                             b"hwtimestamp\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_hwtimestamp(p);
        } else if strcasecmp(command,
                             b"include\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_include(p);
        } else if strcasecmp(command,
                             b"initstepslew\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_initstepslew(p);
        } else if strcasecmp(command,
                             b"keyfile\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut keys_file);
        } else if strcasecmp(command,
                             b"leapsecmode\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_leapsecmode(p);
        } else if strcasecmp(command,
                             b"leapsectz\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut leapsec_tz);
        } else if strcasecmp(command,
                             b"local\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            parse_local(p);
        } else if strcasecmp(command,
                             b"lock_all\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            lock_memory = parse_null(p)
        } else if strcasecmp(command,
                             b"log\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            parse_log(p);
        } else if strcasecmp(command,
                             b"logbanner\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_int(p, &mut log_banner);
        } else if strcasecmp(command,
                             b"logchange\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut log_change_threshold);
        } else if strcasecmp(command,
                             b"logdir\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut logdir);
        } else if strcasecmp(command,
                             b"mailonchange\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_mailonchange(p);
        } else if strcasecmp(command,
                             b"makestep\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_makestep(p);
        } else if strcasecmp(command,
                             b"manual\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            enable_manual = parse_null(p)
        } else if strcasecmp(command,
                             b"maxchange\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_maxchange(p);
        } else if strcasecmp(command,
                             b"maxclockerror\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut max_clock_error);
        } else if strcasecmp(command,
                             b"maxdistance\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut max_distance);
        } else if strcasecmp(command,
                             b"maxdrift\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut max_drift);
        } else if strcasecmp(command,
                             b"maxjitter\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut max_jitter);
        } else if strcasecmp(command,
                             b"maxsamples\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_int(p, &mut max_samples);
        } else if strcasecmp(command,
                             b"maxslewrate\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut max_slew_rate);
        } else if strcasecmp(command,
                             b"maxupdateskew\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut max_update_skew);
        } else if strcasecmp(command,
                             b"minsamples\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_int(p, &mut min_samples);
        } else if strcasecmp(command,
                             b"minsources\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_int(p, &mut min_sources);
        } else if strcasecmp(command,
                             b"noclientlog\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            no_client_log = parse_null(p)
        } else if strcasecmp(command,
                             b"ntpsigndsocket\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut ntp_signd_socket);
        } else if strcasecmp(command,
                             b"peer\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            parse_source(p, NTP_PEER, 0 as libc::c_int);
        } else if strcasecmp(command,
                             b"pool\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            parse_source(p, NTP_SERVER, 1 as libc::c_int);
        } else if strcasecmp(command,
                             b"port\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            parse_int(p, &mut ntp_port);
        } else if strcasecmp(command,
                             b"ratelimit\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_ratelimit(p, &mut ntp_ratelimit_enabled,
                            &mut ntp_ratelimit_interval,
                            &mut ntp_ratelimit_burst,
                            &mut ntp_ratelimit_leak);
        } else if strcasecmp(command,
                             b"refclock\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_refclock(p);
        } else if strcasecmp(command,
                             b"reselectdist\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut reselect_distance);
        } else if strcasecmp(command,
                             b"rtcautotrim\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut rtc_autotrim_threshold);
        } else if strcasecmp(command,
                             b"rtcdevice\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut rtc_device);
        } else if strcasecmp(command,
                             b"rtcfile\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut rtc_file);
        } else if strcasecmp(command,
                             b"rtconutc\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            rtc_on_utc = parse_null(p)
        } else if strcasecmp(command,
                             b"rtcsync\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            rtc_sync = parse_null(p)
        } else if strcasecmp(command,
                             b"sched_priority\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_int(p, &mut sched_priority);
        } else if strcasecmp(command,
                             b"server\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_source(p, NTP_SERVER, 0 as libc::c_int);
        } else if strcasecmp(command,
                             b"smoothtime\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_smoothtime(p);
        } else if strcasecmp(command,
                             b"stratumweight\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut stratum_weight);
        } else if strcasecmp(command,
                             b"tempcomp\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_tempcomp(p);
        } else if strcasecmp(command,
                             b"user\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            parse_string(p, &mut user);
        } else if strcasecmp(command,
                             b"commandkey\x00" as *const u8 as
                                 *const libc::c_char) == 0 ||
                      strcasecmp(command,
                                 b"generatecommandkey\x00" as *const u8 as
                                     *const libc::c_char) == 0 ||
                      strcasecmp(command,
                                 b"linux_freq_scale\x00" as *const u8 as
                                     *const libc::c_char) == 0 ||
                      strcasecmp(command,
                                 b"linux_hz\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
            LOG_Message(LOGS_WARN,
                        b"%s directive is no longer supported\x00" as
                            *const u8 as *const libc::c_char, command);
        } else {
            other_parse_error(b"Invalid command\x00" as *const u8 as
                                  *const libc::c_char);
        }
    };
}
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2009-2017
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

  Module that reads and processes the configuration file.
  */
/* ================================================== */
/* Forward prototypes */
/* ================================================== */
unsafe extern "C" fn parse_string(mut line: *mut libc::c_char,
                                  mut result: *mut *mut libc::c_char)
 -> libc::c_int {
    check_number_of_args(line, 1 as libc::c_int);
    free(*result as *mut libc::c_void);
    *result = Strdup(line);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn parse_int(mut line: *mut libc::c_char,
                               mut result: *mut libc::c_int) -> libc::c_int {
    check_number_of_args(line, 1 as libc::c_int);
    if sscanf(line, b"%d\x00" as *const u8 as *const libc::c_char, result) !=
           1 as libc::c_int {
        command_parse_error();
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn parse_double(mut line: *mut libc::c_char,
                                  mut result: *mut libc::c_double)
 -> libc::c_int {
    check_number_of_args(line, 1 as libc::c_int);
    if sscanf(line, b"%lf\x00" as *const u8 as *const libc::c_char, result) !=
           1 as libc::c_int {
        command_parse_error();
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn parse_null(mut line: *mut libc::c_char) -> libc::c_int {
    check_number_of_args(line, 0 as libc::c_int);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn parse_source(mut line: *mut libc::c_char,
                                  mut type_0: NTP_Source_Type,
                                  mut pool: libc::c_int) {
    let mut source: NTP_Source =
        NTP_Source{type_0: NTP_SERVER,
                   pool: 0,
                   params:
                       CPS_NTP_Source{name: 0 as *mut libc::c_char,
                                      port: 0,
                                      params:
                                          SourceParameters{minpoll: 0,
                                                           maxpoll: 0,
                                                           connectivity:
                                                               SRC_OFFLINE,
                                                           auto_offline: 0,
                                                           presend_minpoll: 0,
                                                           burst: 0,
                                                           iburst: 0,
                                                           min_stratum: 0,
                                                           poll_target: 0,
                                                           version: 0,
                                                           max_sources: 0,
                                                           min_samples: 0,
                                                           max_samples: 0,
                                                           filter_length: 0,
                                                           interleaved: 0,
                                                           sel_options: 0,
                                                           authkey: 0,
                                                           max_delay: 0.,
                                                           max_delay_ratio:
                                                               0.,
                                                           max_delay_dev_ratio:
                                                               0.,
                                                           min_delay: 0.,
                                                           asymmetry: 0.,
                                                           offset: 0.,},},};
    source.type_0 = type_0;
    source.pool = pool;
    if CPS_ParseNTPSourceAdd(line, &mut source.params) == 0 {
        command_parse_error();
        return
    }
    source.params.name = Strdup(source.params.name);
    ARR_AppendElement(ntp_sources,
                      &mut source as *mut NTP_Source as *mut libc::c_void);
}
/* ================================================== */
unsafe extern "C" fn parse_ratelimit(mut line: *mut libc::c_char,
                                     mut enabled: *mut libc::c_int,
                                     mut interval: *mut libc::c_int,
                                     mut burst: *mut libc::c_int,
                                     mut leak: *mut libc::c_int) {
    let mut n: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut opt: *mut libc::c_char = 0 as *mut libc::c_char;
    *enabled = 1 as libc::c_int;
    while *line != 0 {
        opt = line;
        line = CPS_SplitWord(line);
        if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                  &mut val as *mut libc::c_int, &mut n as *mut libc::c_int) !=
               1 as libc::c_int {
            command_parse_error();
            return
        }
        line = line.offset(n as isize);
        if strcasecmp(opt,
                      b"interval\x00" as *const u8 as *const libc::c_char) ==
               0 {
            *interval = val
        } else if strcasecmp(opt,
                             b"burst\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            *burst = val
        } else if strcasecmp(opt,
                             b"leak\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            *leak = val
        } else { command_parse_error(); }
    };
}
/* ================================================== */
unsafe extern "C" fn parse_refclock(mut line: *mut libc::c_char) {
    let mut n: libc::c_int = 0;
    let mut poll: libc::c_int = 0;
    let mut dpoll: libc::c_int = 0;
    let mut filter_length: libc::c_int = 0;
    let mut pps_rate: libc::c_int = 0;
    let mut min_samples_0: libc::c_int = 0;
    let mut max_samples_0: libc::c_int = 0;
    let mut sel_options: libc::c_int = 0;
    let mut max_lock_age: libc::c_int = 0;
    let mut pps_forced: libc::c_int = 0;
    let mut stratum: libc::c_int = 0;
    let mut tai: libc::c_int = 0;
    let mut ref_id: uint32_t = 0;
    let mut lock_ref_id: uint32_t = 0;
    let mut offset: libc::c_double = 0.;
    let mut delay: libc::c_double = 0.;
    let mut precision: libc::c_double = 0.;
    let mut max_dispersion: libc::c_double = 0.;
    let mut pulse_width: libc::c_double = 0.;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut param: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ref_0: [libc::c_uchar; 5] = [0; 5];
    let mut refclock: *mut RefclockParameters = 0 as *mut RefclockParameters;
    poll = 4 as libc::c_int;
    dpoll = 0 as libc::c_int;
    filter_length = 64 as libc::c_int;
    pps_forced = 0 as libc::c_int;
    pps_rate = 0 as libc::c_int;
    min_samples_0 = -(1 as libc::c_int);
    max_samples_0 = -(1 as libc::c_int);
    sel_options = 0 as libc::c_int;
    offset = 0.0f64;
    delay = 1e-9f64;
    precision = 0.0f64;
    max_dispersion = 0.0f64;
    pulse_width = 0.0f64;
    ref_id = 0 as libc::c_int as uint32_t;
    max_lock_age = 2 as libc::c_int;
    lock_ref_id = 0 as libc::c_int as uint32_t;
    stratum = 0 as libc::c_int;
    tai = 0 as libc::c_int;
    if *line == 0 { command_parse_error(); return }
    p = line;
    line = CPS_SplitWord(line);
    if *line == 0 { command_parse_error(); return }
    name = Strdup(p);
    p = line;
    line = CPS_SplitWord(line);
    param = Strdup(p);
    cmd = line;
    while *cmd != 0 {
        line = CPS_SplitWord(line);
        if strcasecmp(cmd, b"refid\x00" as *const u8 as *const libc::c_char)
               == 0 {
            if sscanf(line, b"%4s%n\x00" as *const u8 as *const libc::c_char,
                      ref_0.as_mut_ptr() as *mut libc::c_char,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
            ref_id =
                (ref_0[0 as libc::c_int as usize] as uint32_t) <<
                    24 as libc::c_int |
                    ((ref_0[1 as libc::c_int as usize] as libc::c_int) <<
                         16 as libc::c_int) as libc::c_uint |
                    ((ref_0[2 as libc::c_int as usize] as libc::c_int) <<
                         8 as libc::c_int) as libc::c_uint |
                    ref_0[3 as libc::c_int as usize] as libc::c_uint
        } else if strcasecmp(cmd,
                             b"lock\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%4s%n\x00" as *const u8 as *const libc::c_char,
                      ref_0.as_mut_ptr() as *mut libc::c_char,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
            lock_ref_id =
                (ref_0[0 as libc::c_int as usize] as uint32_t) <<
                    24 as libc::c_int |
                    ((ref_0[1 as libc::c_int as usize] as libc::c_int) <<
                         16 as libc::c_int) as libc::c_uint |
                    ((ref_0[2 as libc::c_int as usize] as libc::c_int) <<
                         8 as libc::c_int) as libc::c_uint |
                    ref_0[3 as libc::c_int as usize] as libc::c_uint
        } else if strcasecmp(cmd,
                             b"poll\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut poll as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"dpoll\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut dpoll as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"filter\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut filter_length as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"rate\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut pps_rate as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"minsamples\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut min_samples_0 as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"maxlockage\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut max_lock_age as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"maxsamples\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut max_samples_0 as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"offset\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut offset as *mut libc::c_double,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"delay\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut delay as *mut libc::c_double,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"pps\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            n = 0 as libc::c_int;
            pps_forced = 1 as libc::c_int
        } else if strcasecmp(cmd,
                             b"precision\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut precision as *mut libc::c_double,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"maxdispersion\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut max_dispersion as *mut libc::c_double,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"stratum\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut stratum as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int ||
                   stratum >= 16 as libc::c_int || stratum < 0 as libc::c_int
               {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"tai\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            n = 0 as libc::c_int;
            tai = 1 as libc::c_int
        } else if strcasecmp(cmd,
                             b"width\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut pulse_width as *mut libc::c_double,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"noselect\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            n = 0 as libc::c_int;
            sel_options |= 0x1 as libc::c_int
        } else if strcasecmp(cmd,
                             b"prefer\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            n = 0 as libc::c_int;
            sel_options |= 0x2 as libc::c_int
        } else if strcasecmp(cmd,
                             b"trust\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            n = 0 as libc::c_int;
            sel_options |= 0x4 as libc::c_int
        } else if strcasecmp(cmd,
                             b"require\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            n = 0 as libc::c_int;
            sel_options |= 0x8 as libc::c_int
        } else {
            other_parse_error(b"Invalid refclock option\x00" as *const u8 as
                                  *const libc::c_char);
            return
        }
        line = line.offset(n as isize);
        cmd = line
    }
    if *cmd != 0 { command_parse_error(); return }
    refclock = ARR_GetNewElement(refclock_sources) as *mut RefclockParameters;
    (*refclock).driver_name = name;
    (*refclock).driver_parameter = param;
    (*refclock).driver_poll = dpoll;
    (*refclock).poll = poll;
    (*refclock).filter_length = filter_length;
    (*refclock).pps_forced = pps_forced;
    (*refclock).pps_rate = pps_rate;
    (*refclock).min_samples = min_samples_0;
    (*refclock).max_samples = max_samples_0;
    (*refclock).sel_options = sel_options;
    (*refclock).stratum = stratum;
    (*refclock).tai = tai;
    (*refclock).offset = offset;
    (*refclock).delay = delay;
    (*refclock).precision = precision;
    (*refclock).max_dispersion = max_dispersion;
    (*refclock).pulse_width = pulse_width;
    (*refclock).ref_id = ref_id;
    (*refclock).max_lock_age = max_lock_age;
    (*refclock).lock_ref_id = lock_ref_id;
}
/* ================================================== */
unsafe extern "C" fn parse_log(mut line: *mut libc::c_char) {
    let mut log_name: *mut libc::c_char = 0 as *mut libc::c_char;
    loop  {
        log_name = line;
        line = CPS_SplitWord(line);
        if !(*log_name != 0) { break ; }
        if strcmp(log_name,
                  b"rawmeasurements\x00" as *const u8 as *const libc::c_char)
               == 0 {
            do_log_measurements = 1 as libc::c_int;
            raw_measurements = 1 as libc::c_int
        } else if strcmp(log_name,
                         b"measurements\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            do_log_measurements = 1 as libc::c_int
        } else if strcmp(log_name,
                         b"statistics\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            do_log_statistics = 1 as libc::c_int
        } else if strcmp(log_name,
                         b"tracking\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            do_log_tracking = 1 as libc::c_int
        } else if strcmp(log_name,
                         b"rtc\x00" as *const u8 as *const libc::c_char) == 0
         {
            do_log_rtc = 1 as libc::c_int
        } else if strcmp(log_name,
                         b"refclocks\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            do_log_refclocks = 1 as libc::c_int
        } else if strcmp(log_name,
                         b"tempcomp\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            do_log_tempcomp = 1 as libc::c_int
        } else {
            other_parse_error(b"Invalid log parameter\x00" as *const u8 as
                                  *const libc::c_char);
            break ;
        }
    };
}
/* ================================================== */
unsafe extern "C" fn parse_local(mut line: *mut libc::c_char) {
    if CPS_ParseLocal(line, &mut local_stratum, &mut local_orphan,
                      &mut local_distance) == 0 {
        command_parse_error();
    }
    enable_local = 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn parse_initstepslew(mut line: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip_addr: IPAddr =
        IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
    /* Ignore the line if chronyd was started with -R. */
    if restarted != 0 { return }
    ARR_SetSize(init_sources, 0 as libc::c_int as libc::c_uint);
    p = CPS_SplitWord(line);
    if sscanf(line, b"%lf\x00" as *const u8 as *const libc::c_char,
              &mut init_slew_threshold as *mut libc::c_double) !=
           1 as libc::c_int {
        command_parse_error();
        return
    }
    while *p != 0 {
        hostname = p;
        p = CPS_SplitWord(p);
        if *hostname != 0 {
            if DNS_Name2IPAddress(hostname, &mut ip_addr, 1 as libc::c_int) as
                   libc::c_uint == DNS_Success as libc::c_int as libc::c_uint
               {
                ARR_AppendElement(init_sources,
                                  &mut ip_addr as *mut IPAddr as
                                      *mut libc::c_void);
            } else {
                LOG_Message(LOGS_WARN,
                            b"Could not resolve address of initstepslew server %s\x00"
                                as *const u8 as *const libc::c_char,
                            hostname);
            }
        }
    };
}
/* ================================================== */
unsafe extern "C" fn parse_leapsecmode(mut line: *mut libc::c_char) {
    if strcasecmp(line, b"system\x00" as *const u8 as *const libc::c_char) ==
           0 {
        leapsec_mode = REF_LeapModeSystem
    } else if strcasecmp(line,
                         b"slew\x00" as *const u8 as *const libc::c_char) == 0
     {
        leapsec_mode = REF_LeapModeSlew
    } else if strcasecmp(line,
                         b"step\x00" as *const u8 as *const libc::c_char) == 0
     {
        leapsec_mode = REF_LeapModeStep
    } else if strcasecmp(line,
                         b"ignore\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        leapsec_mode = REF_LeapModeIgnore
    } else { command_parse_error(); };
}
/* ================================================== */
unsafe extern "C" fn parse_clientloglimit(mut line: *mut libc::c_char) {
    check_number_of_args(line, 1 as libc::c_int);
    if sscanf(line, b"%lu\x00" as *const u8 as *const libc::c_char,
              &mut client_log_limit as *mut libc::c_ulong) != 1 as libc::c_int
       {
        command_parse_error();
    };
}
/* ================================================== */
unsafe extern "C" fn parse_fallbackdrift(mut line: *mut libc::c_char) {
    check_number_of_args(line, 2 as libc::c_int);
    if sscanf(line, b"%d %d\x00" as *const u8 as *const libc::c_char,
              &mut fb_drift_min as *mut libc::c_int,
              &mut fb_drift_max as *mut libc::c_int) != 2 as libc::c_int {
        command_parse_error();
    };
}
/* ================================================== */
unsafe extern "C" fn parse_makestep(mut line: *mut libc::c_char) {
    check_number_of_args(line, 2 as libc::c_int);
    if sscanf(line, b"%lf %d\x00" as *const u8 as *const libc::c_char,
              &mut make_step_threshold as *mut libc::c_double,
              &mut make_step_limit as *mut libc::c_int) != 2 as libc::c_int {
        make_step_limit = 0 as libc::c_int;
        command_parse_error();
    }
    /* Disable limited makestep if chronyd was started with -R. */
    if restarted != 0 && make_step_limit > 0 as libc::c_int {
        make_step_limit = 0 as libc::c_int
    };
}
/* ================================================== */
unsafe extern "C" fn parse_maxchange(mut line: *mut libc::c_char) {
    check_number_of_args(line, 3 as libc::c_int);
    if sscanf(line, b"%lf %d %d\x00" as *const u8 as *const libc::c_char,
              &mut max_offset as *mut libc::c_double,
              &mut max_offset_delay as *mut libc::c_int,
              &mut max_offset_ignore as *mut libc::c_int) != 3 as libc::c_int
       {
        max_offset_delay = -(1 as libc::c_int);
        command_parse_error();
    };
}
/* ================================================== */
unsafe extern "C" fn parse_mailonchange(mut line: *mut libc::c_char) {
    let mut address: *mut libc::c_char = 0 as *mut libc::c_char;
    check_number_of_args(line, 2 as libc::c_int);
    address = line;
    line = CPS_SplitWord(line);
    free(mail_user_on_change as *mut libc::c_void);
    if sscanf(line, b"%lf\x00" as *const u8 as *const libc::c_char,
              &mut mail_change_threshold as *mut libc::c_double) ==
           1 as libc::c_int {
        mail_user_on_change = Strdup(address)
    } else {
        mail_user_on_change = 0 as *mut libc::c_char;
        command_parse_error();
    };
}
/* ================================================== */
unsafe extern "C" fn parse_allow_deny(mut line: *mut libc::c_char,
                                      mut restrictions: ARR_Instance,
                                      mut allow: libc::c_int) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a: libc::c_ulong = 0;
    let mut b: libc::c_ulong = 0;
    let mut c: libc::c_ulong = 0;
    let mut d: libc::c_ulong = 0;
    let mut n: libc::c_ulong = 0;
    let mut all: libc::c_int = 0 as libc::c_int;
    let mut new_node: *mut AllowDeny = 0 as *mut AllowDeny;
    let mut ip_addr: IPAddr =
        IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
    p = line;
    if strncmp(p, b"all\x00" as *const u8 as *const libc::c_char,
               3 as libc::c_int as libc::c_ulong) == 0 {
        all = 1 as libc::c_int;
        p = CPS_SplitWord(line)
    }
    if *p == 0 {
        /* Empty line applies to all addresses */
        new_node = ARR_GetNewElement(restrictions) as *mut AllowDeny;
        (*new_node).allow = allow;
        (*new_node).all = all;
        (*new_node).ip.family = 0 as libc::c_int as uint16_t;
        (*new_node).subnet_bits = 0 as libc::c_int
    } else {
        let mut slashpos: *mut libc::c_char = 0 as *mut libc::c_char;
        slashpos = strchr(p, '/' as i32);
        if !slashpos.is_null() {
            *slashpos = 0 as libc::c_int as libc::c_char
        }
        check_number_of_args(p, 1 as libc::c_int);
        n = 0 as libc::c_int as libc::c_ulong;
        if UTI_StringToIP(p, &mut ip_addr) != 0 ||
               {
                   n =
                       sscanf(p,
                              b"%lu.%lu.%lu.%lu\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut a as *mut libc::c_ulong,
                              &mut b as *mut libc::c_ulong,
                              &mut c as *mut libc::c_ulong,
                              &mut d as *mut libc::c_ulong) as libc::c_ulong;
                   (n) >= 1 as libc::c_int as libc::c_ulong
               } {
            new_node = ARR_GetNewElement(restrictions) as *mut AllowDeny;
            (*new_node).allow = allow;
            (*new_node).all = all;
            if n == 0 as libc::c_int as libc::c_ulong {
                (*new_node).ip = ip_addr;
                if ip_addr.family as libc::c_int == 2 as libc::c_int {
                    (*new_node).subnet_bits = 128 as libc::c_int
                } else { (*new_node).subnet_bits = 32 as libc::c_int }
            } else {
                (*new_node).ip.family = 1 as libc::c_int as uint16_t;
                a &= 0xff as libc::c_int as libc::c_ulong;
                b &= 0xff as libc::c_int as libc::c_ulong;
                c &= 0xff as libc::c_int as libc::c_ulong;
                d &= 0xff as libc::c_int as libc::c_ulong;
                match n {
                    1 => {
                        (*new_node).ip.addr.in4 =
                            (a << 24 as libc::c_int) as uint32_t;
                        (*new_node).subnet_bits = 8 as libc::c_int
                    }
                    2 => {
                        (*new_node).ip.addr.in4 =
                            (a << 24 as libc::c_int | b << 16 as libc::c_int)
                                as uint32_t;
                        (*new_node).subnet_bits = 16 as libc::c_int
                    }
                    3 => {
                        (*new_node).ip.addr.in4 =
                            (a << 24 as libc::c_int | b << 16 as libc::c_int |
                                 c << 8 as libc::c_int) as uint32_t;
                        (*new_node).subnet_bits = 24 as libc::c_int
                    }
                    4 => {
                        (*new_node).ip.addr.in4 =
                            (a << 24 as libc::c_int | b << 16 as libc::c_int |
                                 c << 8 as libc::c_int | d) as uint32_t;
                        (*new_node).subnet_bits = 32 as libc::c_int
                    }
                    _ => {
                        __assert_fail(b"0\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"conf.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1063 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 49],
                                                                &[libc::c_char; 49]>(b"void parse_allow_deny(char *, ARR_Instance, int)\x00")).as_ptr());
                    }
                }
            }
            if !slashpos.is_null() {
                let mut specified_subnet_bits: libc::c_int = 0;
                let mut n_0: libc::c_int = 0;
                n_0 =
                    sscanf(slashpos.offset(1 as libc::c_int as isize),
                           b"%d\x00" as *const u8 as *const libc::c_char,
                           &mut specified_subnet_bits as *mut libc::c_int);
                if n_0 == 1 as libc::c_int {
                    (*new_node).subnet_bits = specified_subnet_bits
                } else { command_parse_error(); }
            }
        } else if slashpos.is_null() &&
                      DNS_Name2IPAddress(p, &mut ip_addr, 1 as libc::c_int) as
                          libc::c_uint ==
                          DNS_Success as libc::c_int as libc::c_uint {
            new_node = ARR_GetNewElement(restrictions) as *mut AllowDeny;
            (*new_node).allow = allow;
            (*new_node).all = all;
            (*new_node).ip = ip_addr;
            if ip_addr.family as libc::c_int == 2 as libc::c_int {
                (*new_node).subnet_bits = 128 as libc::c_int
            } else { (*new_node).subnet_bits = 32 as libc::c_int }
        } else { command_parse_error(); }
    };
}
/* ================================================== */
unsafe extern "C" fn parse_bindacqaddress(mut line: *mut libc::c_char) {
    let mut ip: IPAddr =
        IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
    check_number_of_args(line, 1 as libc::c_int);
    if UTI_StringToIP(line, &mut ip) != 0 {
        if ip.family as libc::c_int == 1 as libc::c_int {
            bind_acq_address4 = ip
        } else if ip.family as libc::c_int == 2 as libc::c_int {
            bind_acq_address6 = ip
        }
    } else { command_parse_error(); };
}
/* ================================================== */
unsafe extern "C" fn parse_bindaddress(mut line: *mut libc::c_char) {
    let mut ip: IPAddr =
        IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
    check_number_of_args(line, 1 as libc::c_int);
    if UTI_StringToIP(line, &mut ip) != 0 {
        if ip.family as libc::c_int == 1 as libc::c_int {
            bind_address4 = ip
        } else if ip.family as libc::c_int == 2 as libc::c_int {
            bind_address6 = ip
        }
    } else { command_parse_error(); };
}
/* ================================================== */
unsafe extern "C" fn parse_bindcmdaddress(mut line: *mut libc::c_char) {
    let mut ip: IPAddr =
        IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
    check_number_of_args(line, 1 as libc::c_int);
    /* Address starting with / is for the Unix domain socket */
    if *line.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        parse_string(line, &mut bind_cmd_path);
        /* / disables the socket */
        if strcmp(bind_cmd_path, b"/\x00" as *const u8 as *const libc::c_char)
               == 0 {
            *bind_cmd_path.offset(0 as libc::c_int as isize) =
                '\u{0}' as i32 as libc::c_char
        }
    } else if UTI_StringToIP(line, &mut ip) != 0 {
        if ip.family as libc::c_int == 1 as libc::c_int {
            bind_cmd_address4 = ip
        } else if ip.family as libc::c_int == 2 as libc::c_int {
            bind_cmd_address6 = ip
        }
    } else { command_parse_error(); };
}
/* ================================================== */
unsafe extern "C" fn parse_broadcast(mut line: *mut libc::c_char) {
    /* Syntax : broadcast <interval> <broadcast-IP-addr> [<port>] */
    let mut destination: *mut NTP_Broadcast_Destination =
        0 as *mut NTP_Broadcast_Destination;
    let mut port: libc::c_int = 0;
    let mut interval: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip: IPAddr =
        IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
    p = line;
    line = CPS_SplitWord(line);
    if sscanf(p, b"%d\x00" as *const u8 as *const libc::c_char,
              &mut interval as *mut libc::c_int) != 1 as libc::c_int {
        command_parse_error();
        return
    }
    p = line;
    line = CPS_SplitWord(line);
    if UTI_StringToIP(p, &mut ip) == 0 { command_parse_error(); return }
    p = line;
    line = CPS_SplitWord(line);
    if *p != 0 {
        if sscanf(p, b"%d\x00" as *const u8 as *const libc::c_char,
                  &mut port as *mut libc::c_int) != 1 as libc::c_int ||
               *line as libc::c_int != 0 {
            command_parse_error();
            return
        }
    } else {
        /* default port */
        port = 123 as libc::c_int
    }
    destination =
        ARR_GetNewElement(broadcasts) as *mut NTP_Broadcast_Destination;
    (*destination).addr = ip;
    (*destination).port = port as libc::c_ushort;
    (*destination).interval = interval;
}
/* ================================================== */
unsafe extern "C" fn parse_smoothtime(mut line: *mut libc::c_char) {
    if get_number_of_args(line) != 3 as libc::c_int {
        check_number_of_args(line, 2 as libc::c_int);
    }
    if sscanf(line, b"%lf %lf\x00" as *const u8 as *const libc::c_char,
              &mut smooth_max_freq as *mut libc::c_double,
              &mut smooth_max_wander as *mut libc::c_double) !=
           2 as libc::c_int {
        smooth_max_freq = 0.0f64;
        command_parse_error();
    }
    line = CPS_SplitWord(CPS_SplitWord(line));
    smooth_leap_only = 0 as libc::c_int;
    if *line != 0 {
        if strcasecmp(line,
                      b"leaponly\x00" as *const u8 as *const libc::c_char) ==
               0 {
            smooth_leap_only = 1 as libc::c_int
        } else { command_parse_error(); }
    };
}
/* ================================================== */
unsafe extern "C" fn parse_tempcomp(mut line: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut point_form: libc::c_int = 0;
    point_form =
        (get_number_of_args(line) == 3 as libc::c_int) as libc::c_int;
    if point_form == 0 { check_number_of_args(line, 6 as libc::c_int); }
    p = line;
    line = CPS_SplitWord(line);
    if *p == 0 { command_parse_error(); return }
    free(tempcomp_point_file as *mut libc::c_void);
    if point_form != 0 {
        if sscanf(line, b"%lf\x00" as *const u8 as *const libc::c_char,
                  &mut tempcomp_interval as *mut libc::c_double) !=
               1 as libc::c_int {
            command_parse_error();
            return
        }
        tempcomp_point_file = Strdup(CPS_SplitWord(line))
    } else {
        if sscanf(line,
                  b"%lf %lf %lf %lf %lf\x00" as *const u8 as
                      *const libc::c_char,
                  &mut tempcomp_interval as *mut libc::c_double,
                  &mut tempcomp_T0 as *mut libc::c_double,
                  &mut tempcomp_k0 as *mut libc::c_double,
                  &mut tempcomp_k1 as *mut libc::c_double,
                  &mut tempcomp_k2 as *mut libc::c_double) != 5 as libc::c_int
           {
            command_parse_error();
            return
        }
        tempcomp_point_file = 0 as *mut libc::c_char
    }
    free(tempcomp_sensor_file as *mut libc::c_void);
    tempcomp_sensor_file = Strdup(p);
}
/* ================================================== */
unsafe extern "C" fn parse_hwtimestamp(mut line: *mut libc::c_char) {
    let mut iface: *mut CNF_HwTsInterface = 0 as *mut CNF_HwTsInterface;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filter: [libc::c_char; 5] = [0; 5];
    let mut n: libc::c_int = 0;
    if *line == 0 { command_parse_error(); return }
    p = line;
    line = CPS_SplitWord(line);
    iface = ARR_GetNewElement(hwts_interfaces) as *mut CNF_HwTsInterface;
    (*iface).name = Strdup(p);
    (*iface).minpoll = 0 as libc::c_int;
    (*iface).min_samples = 2 as libc::c_int;
    (*iface).max_samples = 16 as libc::c_int;
    (*iface).nocrossts = 0 as libc::c_int;
    (*iface).rxfilter = CNF_HWTS_RXFILTER_ANY;
    (*iface).precision = 100.0e-9f64;
    (*iface).tx_comp = 0.0f64;
    (*iface).rx_comp = 0.0f64;
    p = line;
    while *p != 0 {
        line = CPS_SplitWord(line);
        if strcasecmp(p,
                      b"maxsamples\x00" as *const u8 as *const libc::c_char)
               == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*iface).max_samples as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(p,
                             b"minpoll\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*iface).minpoll as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(p,
                             b"minsamples\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*iface).min_samples as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(p,
                             b"precision\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*iface).precision as *mut libc::c_double,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(p,
                             b"rxcomp\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*iface).rx_comp as *mut libc::c_double,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(p,
                             b"txcomp\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*iface).tx_comp as *mut libc::c_double,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                break ;
            }
        } else if strcasecmp(p,
                             b"rxfilter\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%4s%n\x00" as *const u8 as *const libc::c_char,
                      filter.as_mut_ptr(), &mut n as *mut libc::c_int) !=
                   1 as libc::c_int {
                break ;
            }
            if strcasecmp(filter.as_mut_ptr(),
                          b"none\x00" as *const u8 as *const libc::c_char) ==
                   0 {
                (*iface).rxfilter = CNF_HWTS_RXFILTER_NONE
            } else if strcasecmp(filter.as_mut_ptr(),
                                 b"ntp\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                (*iface).rxfilter = CNF_HWTS_RXFILTER_NTP
            } else {
                if !(strcasecmp(filter.as_mut_ptr(),
                                b"all\x00" as *const u8 as
                                    *const libc::c_char) == 0) {
                    break ;
                }
                (*iface).rxfilter = CNF_HWTS_RXFILTER_ALL
            }
        } else {
            if !(strcasecmp(p,
                            b"nocrossts\x00" as *const u8 as
                                *const libc::c_char) == 0) {
                break ;
            }
            n = 0 as libc::c_int;
            (*iface).nocrossts = 1 as libc::c_int
        }
        line = line.offset(n as isize);
        p = line
    }
    if *p != 0 { command_parse_error(); };
}
/* ================================================== */
unsafe extern "C" fn parse_include(mut line: *mut libc::c_char) {
    let mut gl: glob_t =
        glob_t{gl_pathc: 0,
               gl_pathv: 0 as *mut *mut libc::c_char,
               gl_offs: 0,
               gl_flags: 0,
               gl_closedir: None,
               gl_readdir: None,
               gl_opendir: None,
               gl_lstat: None,
               gl_stat: None,};
    let mut i: size_t = 0;
    let mut r: libc::c_int = 0;
    check_number_of_args(line, 1 as libc::c_int);
    r =
        glob(line,
             (1 as libc::c_int) << 11 as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int, None, &mut gl);
    if r != 0 as libc::c_int {
        if r != 3 as libc::c_int {
            LOG_Message(LOGS_FATAL,
                        b"Could not search for files matching %s\x00" as
                            *const u8 as *const libc::c_char, line);
            exit(1 as libc::c_int);
        }
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"glob of %s failed\x00" as *const u8 as
                            *const libc::c_char, line);
        }
        return
    }
    i = 0 as libc::c_int as size_t;
    while i < gl.gl_pathc {
        CNF_ReadFile(*gl.gl_pathv.offset(i as isize));
        i = i.wrapping_add(1)
    }
    globfree(&mut gl);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_CreateDirs(mut uid: uid_t, mut gid: gid_t) {
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Create a directory for the Unix domain command socket */
    if *bind_cmd_path.offset(0 as libc::c_int as isize) != 0 {
        dir = UTI_PathToDir(bind_cmd_path);
        UTI_CreateDirAndParents(dir, 0o770 as libc::c_int as mode_t, uid,
                                gid);
        /* Check the permissions and owner/group in case the directory already
       existed.  It MUST NOT be accessible by others as permissions on Unix
       domain sockets are ignored on some systems (e.g. Solaris). */
        if UTI_CheckDirPermissions(dir, 0o770 as libc::c_int as mode_t, uid,
                                   gid) == 0 {
            LOG_Message(LOGS_WARN,
                        b"Disabled command socket %s\x00" as *const u8 as
                            *const libc::c_char, bind_cmd_path);
            *bind_cmd_path.offset(0 as libc::c_int as isize) =
                '\u{0}' as i32 as libc::c_char
        }
        free(dir as *mut libc::c_void);
    }
    if *logdir.offset(0 as libc::c_int as isize) != 0 {
        UTI_CreateDirAndParents(logdir, 0o755 as libc::c_int as mode_t, uid,
                                gid);
    }
    if *dumpdir.offset(0 as libc::c_int as isize) != 0 {
        UTI_CreateDirAndParents(dumpdir, 0o755 as libc::c_int as mode_t, uid,
                                gid);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_AddInitSources() {
    let mut cps_source: CPS_NTP_Source =
        CPS_NTP_Source{name: 0 as *mut libc::c_char,
                       port: 0,
                       params:
                           SourceParameters{minpoll: 0,
                                            maxpoll: 0,
                                            connectivity: SRC_OFFLINE,
                                            auto_offline: 0,
                                            presend_minpoll: 0,
                                            burst: 0,
                                            iburst: 0,
                                            min_stratum: 0,
                                            poll_target: 0,
                                            version: 0,
                                            max_sources: 0,
                                            min_samples: 0,
                                            max_samples: 0,
                                            filter_length: 0,
                                            interleaved: 0,
                                            sel_options: 0,
                                            authkey: 0,
                                            max_delay: 0.,
                                            max_delay_ratio: 0.,
                                            max_delay_dev_ratio: 0.,
                                            min_delay: 0.,
                                            asymmetry: 0.,
                                            offset: 0.,},};
    let mut ntp_addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    let mut dummy_hostname: [libc::c_char; 2] =
        *::std::mem::transmute::<&[u8; 2], &mut [libc::c_char; 2]>(b"H\x00");
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(init_sources) {
        /* Get the default NTP params */
        CPS_ParseNTPSourceAdd(dummy_hostname.as_mut_ptr(), &mut cps_source);
        /* Add the address as an offline iburst server */
        ntp_addr.ip_addr = *(ARR_GetElement(init_sources, i) as *mut IPAddr);
        ntp_addr.port = cps_source.port;
        cps_source.params.iburst = 1 as libc::c_int;
        cps_source.params.connectivity = SRC_OFFLINE;
        NSR_AddSource(&mut ntp_addr, NTP_SERVER, &mut cps_source.params);
        i = i.wrapping_add(1)
    }
    ARR_SetSize(init_sources, 0 as libc::c_int as libc::c_uint);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_AddSources() {
    let mut source: *mut NTP_Source = 0 as *mut NTP_Source;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(ntp_sources) {
        source = ARR_GetElement(ntp_sources, i) as *mut NTP_Source;
        NSR_AddSourceByName((*source).params.name,
                            (*source).params.port as libc::c_int,
                            (*source).pool, (*source).type_0,
                            &mut (*source).params.params);
        free((*source).params.name as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    ARR_SetSize(ntp_sources, 0 as libc::c_int as libc::c_uint);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_AddRefclocks() {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(refclock_sources) {
        RCL_AddRefclock(ARR_GetElement(refclock_sources, i) as
                            *mut RefclockParameters);
        i = i.wrapping_add(1)
    }
    ARR_SetSize(refclock_sources, 0 as libc::c_int as libc::c_uint);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_AddBroadcasts() {
    let mut i: libc::c_uint = 0;
    let mut destination: *mut NTP_Broadcast_Destination =
        0 as *mut NTP_Broadcast_Destination;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(broadcasts) {
        destination =
            ARR_GetElement(broadcasts, i) as *mut NTP_Broadcast_Destination;
        NCR_AddBroadcastDestination(&mut (*destination).addr,
                                    (*destination).port,
                                    (*destination).interval);
        i = i.wrapping_add(1)
    }
    ARR_SetSize(broadcasts, 0 as libc::c_int as libc::c_uint);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetNTPPort() -> libc::c_int { return ntp_port; }
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetAcquisitionPort() -> libc::c_int {
    return acquisition_port;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetDriftFile() -> *mut libc::c_char {
    return drift_file;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogBanner() -> libc::c_int {
    return log_banner;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogDir() -> *mut libc::c_char {
    return logdir;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetDumpDir() -> *mut libc::c_char {
    return dumpdir;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogMeasurements(mut raw: *mut libc::c_int)
 -> libc::c_int {
    *raw = raw_measurements;
    return do_log_measurements;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogStatistics() -> libc::c_int {
    return do_log_statistics;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogTracking() -> libc::c_int {
    return do_log_tracking;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogRtc() -> libc::c_int { return do_log_rtc; }
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogRefclocks() -> libc::c_int {
    return do_log_refclocks;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogTempComp() -> libc::c_int {
    return do_log_tempcomp;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetKeysFile() -> *mut libc::c_char {
    return keys_file;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetRtcAutotrim() -> libc::c_double {
    return rtc_autotrim_threshold;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetRtcFile() -> *mut libc::c_char {
    return rtc_file;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetRtcDevice() -> *mut libc::c_char {
    return rtc_device;
}
/* Value returned in ppm, as read from file */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxUpdateSkew() -> libc::c_double {
    return max_update_skew;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxDrift() -> libc::c_double {
    return max_drift;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxClockError() -> libc::c_double {
    return max_clock_error;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetCorrectionTimeRatio() -> libc::c_double {
    return correction_time_ratio;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxSlewRate() -> libc::c_double {
    return max_slew_rate;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxDistance() -> libc::c_double {
    return max_distance;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxJitter() -> libc::c_double {
    return max_jitter;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetReselectDistance() -> libc::c_double {
    return reselect_distance;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetStratumWeight() -> libc::c_double {
    return stratum_weight;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetCombineLimit() -> libc::c_double {
    return combine_limit;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetManualEnabled() -> libc::c_int {
    return enable_manual;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetCommandPort() -> libc::c_int {
    return cmd_port;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_AllowLocalReference(mut stratum:
                                                     *mut libc::c_int,
                                                 mut orphan: *mut libc::c_int,
                                                 mut distance:
                                                     *mut libc::c_double)
 -> libc::c_int {
    if enable_local != 0 {
        *stratum = local_stratum;
        *orphan = local_orphan;
        *distance = local_distance;
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetRtcOnUtc() -> libc::c_int {
    return rtc_on_utc;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetRtcSync() -> libc::c_int { return rtc_sync; }
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMakeStep(mut limit: *mut libc::c_int,
                                         mut threshold: *mut libc::c_double) {
    *limit = make_step_limit;
    *threshold = make_step_threshold;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxChange(mut delay: *mut libc::c_int,
                                          mut ignore: *mut libc::c_int,
                                          mut offset: *mut libc::c_double) {
    *delay = max_offset_delay;
    *ignore = max_offset_ignore;
    *offset = max_offset;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogChange() -> libc::c_double {
    return log_change_threshold;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMailOnChange(mut enabled: *mut libc::c_int,
                                             mut threshold:
                                                 *mut libc::c_double,
                                             mut user_0:
                                                 *mut *mut libc::c_char) {
    if !mail_user_on_change.is_null() {
        *enabled = 1 as libc::c_int;
        *threshold = mail_change_threshold;
        *user_0 = mail_user_on_change
    } else {
        *enabled = 0 as libc::c_int;
        *threshold = 0.0f64;
        *user_0 = 0 as *mut libc::c_char
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_SetupAccessRestrictions() {
    let mut node: *mut AllowDeny = 0 as *mut AllowDeny;
    let mut status: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(ntp_restrictions) {
        node = ARR_GetElement(ntp_restrictions, i) as *mut AllowDeny;
        status =
            NCR_AddAccessRestriction(&mut (*node).ip, (*node).subnet_bits,
                                     (*node).allow, (*node).all);
        if status == 0 {
            LOG_Message(LOGS_FATAL,
                        b"Bad subnet in %s/%d\x00" as *const u8 as
                            *const libc::c_char,
                        UTI_IPToString(&mut (*node).ip), (*node).subnet_bits);
            exit(1 as libc::c_int);
        }
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(cmd_restrictions) {
        node = ARR_GetElement(cmd_restrictions, i) as *mut AllowDeny;
        status =
            CAM_AddAccessRestriction(&mut (*node).ip, (*node).subnet_bits,
                                     (*node).allow, (*node).all);
        if status == 0 {
            LOG_Message(LOGS_FATAL,
                        b"Bad subnet in %s/%d\x00" as *const u8 as
                            *const libc::c_char,
                        UTI_IPToString(&mut (*node).ip), (*node).subnet_bits);
            exit(1 as libc::c_int);
        }
        i = i.wrapping_add(1)
    }
    ARR_SetSize(ntp_restrictions, 0 as libc::c_int as libc::c_uint);
    ARR_SetSize(cmd_restrictions, 0 as libc::c_int as libc::c_uint);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetNoClientLog() -> libc::c_int {
    return no_client_log;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetClientLogLimit() -> libc::c_ulong {
    return client_log_limit;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetFallbackDrifts(mut min: *mut libc::c_int,
                                               mut max: *mut libc::c_int) {
    *min = fb_drift_min;
    *max = fb_drift_max;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetBindAddress(mut family: libc::c_int,
                                            mut addr: *mut IPAddr) {
    if family == 1 as libc::c_int {
        *addr = bind_address4
    } else if family == 2 as libc::c_int {
        *addr = bind_address6
    } else { (*addr).family = 0 as libc::c_int as uint16_t };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetBindAcquisitionAddress(mut family:
                                                           libc::c_int,
                                                       mut addr:
                                                           *mut IPAddr) {
    if family == 1 as libc::c_int {
        *addr = bind_acq_address4
    } else if family == 2 as libc::c_int {
        *addr = bind_acq_address6
    } else { (*addr).family = 0 as libc::c_int as uint16_t };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetBindCommandPath() -> *mut libc::c_char {
    return bind_cmd_path;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetBindCommandAddress(mut family: libc::c_int,
                                                   mut addr: *mut IPAddr) {
    if family == 1 as libc::c_int {
        *addr = bind_cmd_address4
    } else if family == 2 as libc::c_int {
        *addr = bind_cmd_address6
    } else { (*addr).family = 0 as libc::c_int as uint16_t };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetNtpSigndSocket() -> *mut libc::c_char {
    return ntp_signd_socket;
}
/* ================================================== */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLeapSecMode() -> REF_LeapMode {
    return leapsec_mode;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLeapSecTimezone() -> *mut libc::c_char {
    return leapsec_tz;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetSchedPriority() -> libc::c_int {
    return sched_priority;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLockMemory() -> libc::c_int {
    return lock_memory;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetNTPRateLimit(mut interval: *mut libc::c_int,
                                             mut burst: *mut libc::c_int,
                                             mut leak: *mut libc::c_int)
 -> libc::c_int {
    *interval = ntp_ratelimit_interval;
    *burst = ntp_ratelimit_burst;
    *leak = ntp_ratelimit_leak;
    return ntp_ratelimit_enabled;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetCommandRateLimit(mut interval:
                                                     *mut libc::c_int,
                                                 mut burst: *mut libc::c_int,
                                                 mut leak: *mut libc::c_int)
 -> libc::c_int {
    *interval = cmd_ratelimit_interval;
    *burst = cmd_ratelimit_burst;
    *leak = cmd_ratelimit_leak;
    return cmd_ratelimit_enabled;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetSmooth(mut max_freq: *mut libc::c_double,
                                       mut max_wander: *mut libc::c_double,
                                       mut leap_only: *mut libc::c_int) {
    *max_freq = smooth_max_freq;
    *max_wander = smooth_max_wander;
    *leap_only = smooth_leap_only;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetTempComp(mut file: *mut *mut libc::c_char,
                                         mut interval: *mut libc::c_double,
                                         mut point_file:
                                             *mut *mut libc::c_char,
                                         mut T0: *mut libc::c_double,
                                         mut k0: *mut libc::c_double,
                                         mut k1: *mut libc::c_double,
                                         mut k2: *mut libc::c_double) {
    *file = tempcomp_sensor_file;
    *point_file = tempcomp_point_file;
    *interval = tempcomp_interval;
    *T0 = tempcomp_T0;
    *k0 = tempcomp_k0;
    *k1 = tempcomp_k1;
    *k2 = tempcomp_k2;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetUser() -> *mut libc::c_char { return user; }
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxSamples() -> libc::c_int {
    return max_samples;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMinSamples() -> libc::c_int {
    return min_samples;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMinSources() -> libc::c_int {
    return min_sources;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetHwclockFile() -> *mut libc::c_char {
    return hwclock_file;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetInitSources() -> libc::c_int {
    return ARR_GetSize(init_sources) as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetInitStepThreshold() -> libc::c_double {
    return init_slew_threshold;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetHwTsInterface(mut index: libc::c_uint,
                                              mut iface:
                                                  *mut *mut CNF_HwTsInterface)
 -> libc::c_int {
    if index >= ARR_GetSize(hwts_interfaces) { return 0 as libc::c_int }
    *iface = ARR_GetElement(hwts_interfaces, index) as *mut CNF_HwTsInterface;
    return 1 as libc::c_int;
}
