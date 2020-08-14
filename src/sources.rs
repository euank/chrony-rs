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
    pub type SST_Stats_Record;
    #[no_mangle]
    fn glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option<
            unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int) -> libc::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn globfree(__pglob: *mut glob_t);
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    /* This function creates a new instance of the statistics handler */
    #[no_mangle]
    fn SST_CreateInstance(
        refid: uint32_t,
        addr: *mut IPAddr,
        min_samples: libc::c_int,
        max_samples: libc::c_int,
        min_delay: libc::c_double,
        asymmetry: libc::c_double,
    ) -> SST_Stats;
    /* This function deletes an instance of the statistics handler. */
    #[no_mangle]
    fn SST_DeleteInstance(inst: SST_Stats);
    /* This function resets an instance */
    #[no_mangle]
    fn SST_ResetInstance(inst: SST_Stats);
    /* This function changes the reference ID and IP address */
    #[no_mangle]
    fn SST_SetRefid(inst: SST_Stats, refid: uint32_t, addr: *mut IPAddr);
    /* This function accumulates a single sample into the statistics handler */
    #[no_mangle]
    fn SST_AccumulateSample(inst: SST_Stats, sample: *mut NTP_Sample);
    /* This function runs the linear regression operation on the data.  It
    finds the set of most recent samples that give the tightest
    confidence interval for the frequency, and truncates the register
    down to that number of samples. */
    #[no_mangle]
    fn SST_DoNewRegression(inst: SST_Stats);
    /* Get data needed for selection */
    #[no_mangle]
    fn SST_GetSelectionData(
        inst: SST_Stats,
        now: *mut timespec,
        stratum: *mut libc::c_int,
        leap: *mut NTP_Leap,
        offset_lo_limit: *mut libc::c_double,
        offset_hi_limit: *mut libc::c_double,
        root_distance: *mut libc::c_double,
        variance: *mut libc::c_double,
        first_sample_ago: *mut libc::c_double,
        last_sample_ago: *mut libc::c_double,
        select_ok: *mut libc::c_int,
    );
    /* Get data needed when setting up tracking on this source */
    #[no_mangle]
    fn SST_GetTrackingData(
        inst: SST_Stats,
        ref_time: *mut timespec,
        average_offset: *mut libc::c_double,
        offset_sd: *mut libc::c_double,
        frequency: *mut libc::c_double,
        frequency_sd: *mut libc::c_double,
        skew: *mut libc::c_double,
        root_delay: *mut libc::c_double,
        root_dispersion: *mut libc::c_double,
    );
    /* This routine is called when the local machine clock parameters are
       changed.  It adjusts all existing samples that we are holding for
       each peer so that it looks like they were made under the new clock
       regime rather than the old one.

       when = cooked local time when the change occurs

       dfreq = delta frequency. positive means the clock has been adjusted
       because it was previously gaining time relative to the external
       reference(s).

       doffset = offset slewed onto local clock.  positive => local clock
       has been made fast by that amount.

    */
    #[no_mangle]
    fn SST_SlewSamples(
        inst: SST_Stats,
        when: *mut timespec,
        dfreq: libc::c_double,
        doffset: libc::c_double,
    );
    /* This routine is called when an indeterminate offset is introduced
    into the local time. */
    #[no_mangle]
    fn SST_AddDispersion(inst: SST_Stats, dispersion: libc::c_double);
    #[no_mangle]
    fn SST_SaveToFile(inst: SST_Stats, out: *mut FILE);
    #[no_mangle]
    fn SST_LoadFromFile(inst: SST_Stats, in_0: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn SST_DoSourceReport(inst: SST_Stats, report: *mut RPT_SourceReport, now: *mut timespec);
    #[no_mangle]
    fn SST_DoSourcestatsReport(
        inst: SST_Stats,
        report: *mut RPT_SourcestatsReport,
        now: *mut timespec,
    );
    #[no_mangle]
    fn SST_Samples(inst: SST_Stats) -> libc::c_int;
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
    fn Malloc2(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn Realloc2(ptr: *mut libc::c_void, nmemb: size_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn NSR_HandleBadSource(address: *mut IPAddr);
    #[no_mangle]
    fn NSR_GetLocalRefid(address: *mut IPAddr) -> uint32_t;
    /* Add a handler.  Then handler MUST NOT deregister itself!!! */
    #[no_mangle]
    fn LCL_AddParameterChangeHandler(
        handler: LCL_ParameterChangeHandler,
        anything: *mut libc::c_void,
    );
    /* Remove a handler */
    #[no_mangle]
    fn LCL_RemoveParameterChangeHandler(_: LCL_ParameterChangeHandler, anything: *mut libc::c_void);
    /* Register a handler for being notified of dispersion being added to
    the local clock.  The handler MUST NOT unregister itself!!! */
    #[no_mangle]
    fn LCL_AddDispersionNotifyHandler(
        handler: LCL_DispersionNotifyHandler,
        anything: *mut libc::c_void,
    );
    /* Delete a handler */
    #[no_mangle]
    fn LCL_RemoveDispersionNotifyHandler(
        handler: LCL_DispersionNotifyHandler,
        anything: *mut libc::c_void,
    );
    /* Routine to read the maximum frequency error of the local clock.  This
    is a frequency stability, not an absolute error. */
    #[no_mangle]
    fn LCL_GetMaxClockError() -> libc::c_double;
    /* Get reference update mode */
    #[no_mangle]
    fn REF_GetMode() -> REF_Mode;
    /* Function called by the clock selection process to register a new
    reference source and its parameters

    stratum is the stratum of the reference

    leap is the leap status read from the source

    ref_id is the reference id of the reference

    ref_time is the time at which the parameters are assumed to be
    correct, in terms of local time

    frequency is the amount of local clock gain relative to the
    reference per unit time interval of the local clock

    skew is the maximum estimated frequency error (so we are within
    [frequency+-skew])

    root_delay is the root delay of the sample we are using

    root_dispersion is the root dispersion of the sample we are using

    */
    #[no_mangle]
    fn REF_SetReference(
        stratum: libc::c_int,
        leap: NTP_Leap,
        combined_sources: libc::c_int,
        ref_id: uint32_t,
        ref_ip: *mut IPAddr,
        ref_time: *mut timespec,
        offset: libc::c_double,
        offset_sd: libc::c_double,
        frequency: libc::c_double,
        frequency_sd: libc::c_double,
        skew: libc::c_double,
        root_delay: libc::c_double,
        root_dispersion: libc::c_double,
    );
    /* Mark the local clock as unsynchronised */
    #[no_mangle]
    fn REF_SetUnsynchronised();
    /* Return stratum of the local reference if orphan mode is enabled */
    #[no_mangle]
    fn REF_GetOrphanStratum() -> libc::c_int;
    /* Check if current raw or cooked time is close to a leap second
    and is better to discard any measurements */
    #[no_mangle]
    fn REF_IsLeapSecondClose() -> libc::c_int;
    /* Calculate result = a - b and return as a double */
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec) -> libc::c_double;
    /* Convert a timespec into a temporary string, largely for diagnostic
    display */
    #[no_mangle]
    fn UTI_TimespecToString(ts: *mut timespec) -> *mut libc::c_char;
    /* Convert ref_id into a temporary string, for diagnostics */
    #[no_mangle]
    fn UTI_RefidToString(ref_id: uint32_t) -> *mut libc::c_char;
    /* Convert an IP address to string, for diagnostics */
    #[no_mangle]
    fn UTI_IPToString(ip: *mut IPAddr) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_StringToIP(addr: *const libc::c_char, ip: *mut IPAddr) -> libc::c_int;
    #[no_mangle]
    fn UTI_OpenFile(
        basedir: *const libc::c_char,
        name: *const libc::c_char,
        suffix: *const libc::c_char,
        mode: libc::c_char,
        perm: mode_t,
    ) -> *mut FILE;
    #[no_mangle]
    fn UTI_RemoveFile(
        basedir: *const libc::c_char,
        name: *const libc::c_char,
        suffix: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn CNF_GetDumpDir() -> *mut libc::c_char;
    #[no_mangle]
    fn CNF_GetMaxDistance() -> libc::c_double;
    #[no_mangle]
    fn CNF_GetMaxJitter() -> libc::c_double;
    #[no_mangle]
    fn CNF_GetReselectDistance() -> libc::c_double;
    #[no_mangle]
    fn CNF_GetStratumWeight() -> libc::c_double;
    #[no_mangle]
    fn CNF_GetCombineLimit() -> libc::c_double;
    #[no_mangle]
    fn CNF_GetMaxSamples() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetMinSamples() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetMinSources() -> libc::c_int;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char, _: ...);
    /* Get the time stamp taken after a file descriptor became ready or a timeout expired */
    #[no_mangle]
    fn SCH_GetLastEventTime(cooked: *mut timespec, err: *mut libc::c_double, raw: *mut timespec);
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
    pub gl_readdir: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut libc::c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int>,
    pub gl_stat: Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int>,
}
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
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
pub type NTP_Leap = libc::c_uint;
pub const LEAP_Unsynchronised: NTP_Leap = 3;
pub const LEAP_DeleteSecond: NTP_Leap = 2;
pub const LEAP_InsertSecond: NTP_Leap = 1;
pub const LEAP_Normal: NTP_Leap = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_Sample {
    pub time: timespec,
    pub offset: libc::c_double,
    pub peer_delay: libc::c_double,
    pub peer_dispersion: libc::c_double,
    pub root_delay: libc::c_double,
    pub root_dispersion: libc::c_double,
    pub stratum: libc::c_int,
    pub leap: NTP_Leap,
}
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

 Data structure definitions within the daemon for various reports that
 can be generated */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_SourceReport {
    pub ip_addr: IPAddr,
    pub stratum: libc::c_int,
    pub poll: libc::c_int,
    pub mode: C2RustUnnamed_1,
    pub state: C2RustUnnamed_0,
    pub sel_options: libc::c_int,
    pub reachability: libc::c_int,
    pub latest_meas_ago: libc::c_ulong,
    pub orig_latest_meas: libc::c_double,
    pub latest_meas: libc::c_double,
    pub latest_meas_err: libc::c_double,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const RPT_OUTLIER: C2RustUnnamed_0 = 5;
pub const RPT_CANDIDATE: C2RustUnnamed_0 = 4;
pub const RPT_JITTERY: C2RustUnnamed_0 = 3;
pub const RPT_FALSETICKER: C2RustUnnamed_0 = 2;
pub const RPT_UNREACH: C2RustUnnamed_0 = 1;
pub const RPT_SYNC: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const RPT_LOCAL_REFERENCE: C2RustUnnamed_1 = 2;
pub const RPT_NTP_PEER: C2RustUnnamed_1 = 1;
pub const RPT_NTP_CLIENT: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_SourcestatsReport {
    pub ref_id: uint32_t,
    pub ip_addr: IPAddr,
    pub n_samples: libc::c_ulong,
    pub n_runs: libc::c_ulong,
    pub span_seconds: libc::c_ulong,
    pub resid_freq_ppm: libc::c_double,
    pub skew_ppm: libc::c_double,
    pub sd: libc::c_double,
    pub est_offset: libc::c_double,
    pub est_offset_err: libc::c_double,
}
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

 Header file for module that deals with the measurements and statistics of
 each of the sources. */
pub type SST_Stats = *mut SST_Stats_Record;
/* ================================================== */
/* Define the instance structure used to hold information about each
source */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRC_Instance_Record {
    pub stats: SST_Stats,
    pub index: libc::c_int,
    pub ref_id: uint32_t,
    pub ip_addr: *mut IPAddr,
    pub active: libc::c_int,
    pub reachability: libc::c_int,
    pub reachability_size: libc::c_int,
    pub updates: libc::c_int,
    pub distant: libc::c_int,
    pub status: SRC_Status,
    pub type_0: SRC_Type,
    pub sel_options: libc::c_int,
    pub sel_score: libc::c_double,
    pub sel_info: SelectInfo,
}
/* ================================================== */
/* Structure used to hold info for selecting between sources */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SelectInfo {
    pub leap: NTP_Leap,
    pub stratum: libc::c_int,
    pub select_ok: libc::c_int,
    pub std_dev: libc::c_double,
    pub root_distance: libc::c_double,
    pub lo_limit: libc::c_double,
    pub hi_limit: libc::c_double,
    pub last_sample_ago: libc::c_double,
}
pub type SRC_Type = libc::c_uint;
pub const SRC_REFCLOCK: SRC_Type = 1;
pub const SRC_NTP: SRC_Type = 0;
pub type SRC_Status = libc::c_uint;
pub const SRC_SELECTED: SRC_Status = 16;
pub const SRC_UNSELECTED: SRC_Status = 15;
pub const SRC_OUTLIER: SRC_Status = 14;
pub const SRC_DISTANT: SRC_Status = 13;
pub const SRC_WAITS_UPDATE: SRC_Status = 12;
pub const SRC_NONPREFERRED: SRC_Status = 11;
pub const SRC_WAITS_SOURCES: SRC_Status = 10;
pub const SRC_FALSETICKER: SRC_Status = 9;
pub const SRC_UNTRUSTED: SRC_Status = 8;
pub const SRC_ORPHAN: SRC_Status = 7;
pub const SRC_STALE: SRC_Status = 6;
pub const SRC_WAITS_STATS: SRC_Status = 5;
pub const SRC_JITTERY: SRC_Status = 4;
pub const SRC_BAD_DISTANCE: SRC_Status = 3;
pub const SRC_BAD_STATS: SRC_Status = 2;
pub const SRC_UNSELECTABLE: SRC_Status = 1;
pub const SRC_OK: SRC_Status = 0;
pub type SRC_Instance = *mut SRC_Instance_Record;
/* ================================================== */
/* Structure used to build the sort list for finding falsetickers */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sort_Element {
    pub index: libc::c_int,
    pub offset: libc::c_double,
    pub tag: C2RustUnnamed_2,
}
pub type C2RustUnnamed_2 = libc::c_int;
pub const HIGH: C2RustUnnamed_2 = 1;
pub const LOW: C2RustUnnamed_2 = -1;
/* Function type for handlers to be called back when an indeterminate
offset is introduced into the local time.  This situation occurs
when the frequency must be adjusted to effect a clock slew and
there is doubt about one of the endpoints of the interval over
which the frequency change was applied.It is expected that such
handlers will add extra dispersion to any existing samples stored
in their registers.

dispersion : The bound on how much error has been introduced in the
local clock, in seconds.

anything : passthrough from the registration routine

*/
pub type LCL_DispersionNotifyHandler =
    Option<unsafe extern "C" fn(_: libc::c_double, _: *mut libc::c_void) -> ()>;
pub type LCL_ChangeType = libc::c_uint;
pub const LCL_ChangeUnknownStep: LCL_ChangeType = 2;
pub const LCL_ChangeStep: LCL_ChangeType = 1;
pub const LCL_ChangeAdjust: LCL_ChangeType = 0;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
pub const REF_ModeNormal: REF_Mode = 0;
pub type REF_Mode = libc::c_uint;
pub const REF_ModeIgnore: REF_Mode = 4;
pub const REF_ModePrintOnce: REF_Mode = 3;
pub const REF_ModeUpdateOnce: REF_Mode = 2;
pub const REF_ModeInitStepSlew: REF_Mode = 1;
pub type LCL_ParameterChangeHandler = Option<
    unsafe extern "C" fn(
        _: *mut timespec,
        _: *mut timespec,
        _: libc::c_double,
        _: libc::c_double,
        _: LCL_ChangeType,
        _: *mut libc::c_void,
    ) -> (),
>;
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Richard P. Curnow  1997-2003
* Copyright (C) Miroslav Lichvar  2011-2016, 2018
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

 The routines in this file manage the complete pool of sources that
 we might be synchronizing to.  This includes NTP sources and others
 (e.g. local reference clocks, eyeball + wristwatch etc).

 */
/* For NTP_Leap */
/* ================================================== */
/* Flag indicating that we are initialised */
static mut initialised: libc::c_int = 0 as libc::c_int;
/* ================================================== */
/* Table of sources */
static mut sources: *mut *mut SRC_Instance_Record =
    0 as *const *mut SRC_Instance_Record as *mut *mut SRC_Instance_Record;
static mut sort_list: *mut Sort_Element = 0 as *const Sort_Element as *mut Sort_Element;
static mut sel_sources: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut n_sources: libc::c_int = 0;
/* Number of sources currently in the table */
static mut max_n_sources: libc::c_int = 0;
static mut selected_source_index: libc::c_int = 0;
static mut max_distance: libc::c_double = 0.;
static mut max_jitter: libc::c_double = 0.;
static mut reselect_distance: libc::c_double = 0.;
static mut stratum_weight: libc::c_double = 0.;
static mut combine_limit: libc::c_double = 0.;
/* ================================================== */
/* Initialisation function */
#[no_mangle]
pub unsafe extern "C" fn SRC_Initialise() {
    sources = 0 as *mut *mut SRC_Instance_Record;
    sort_list = 0 as *mut Sort_Element;
    sel_sources = 0 as *mut libc::c_int;
    n_sources = 0 as libc::c_int;
    max_n_sources = 0 as libc::c_int;
    selected_source_index = -(1 as libc::c_int);
    max_distance = CNF_GetMaxDistance();
    max_jitter = CNF_GetMaxJitter();
    reselect_distance = CNF_GetReselectDistance();
    stratum_weight = CNF_GetStratumWeight();
    combine_limit = CNF_GetCombineLimit();
    initialised = 1 as libc::c_int;
    LCL_AddParameterChangeHandler(
        Some(
            slew_sources
                as unsafe extern "C" fn(
                    _: *mut timespec,
                    _: *mut timespec,
                    _: libc::c_double,
                    _: libc::c_double,
                    _: LCL_ChangeType,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
    LCL_AddDispersionNotifyHandler(
        Some(add_dispersion as unsafe extern "C" fn(_: libc::c_double, _: *mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
    );
}
/* ================================================== */
/* Finalisation function */
#[no_mangle]
pub unsafe extern "C" fn SRC_Finalise() {
    LCL_RemoveParameterChangeHandler(
        Some(
            slew_sources
                as unsafe extern "C" fn(
                    _: *mut timespec,
                    _: *mut timespec,
                    _: libc::c_double,
                    _: libc::c_double,
                    _: LCL_ChangeType,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
    LCL_RemoveDispersionNotifyHandler(
        Some(add_dispersion as unsafe extern "C" fn(_: libc::c_double, _: *mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
    );
    free(sources as *mut libc::c_void);
    free(sort_list as *mut libc::c_void);
    free(sel_sources as *mut libc::c_void);
    initialised = 0 as libc::c_int;
}
/* ================================================== */
/* Function to create a new instance.  This would be called by one of
the individual source-type instance creation routines. */
#[no_mangle]
pub unsafe extern "C" fn SRC_CreateNewInstance(
    mut ref_id: uint32_t,
    mut type_0: SRC_Type,
    mut sel_options: libc::c_int,
    mut addr: *mut IPAddr,
    mut min_samples: libc::c_int,
    mut max_samples: libc::c_int,
    mut min_delay: libc::c_double,
    mut asymmetry: libc::c_double,
) -> SRC_Instance {
    let mut result: SRC_Instance = 0 as *mut SRC_Instance_Record;
    if initialised != 0 {
    } else {
        __assert_fail(b"initialised\x00" as *const u8 as *const libc::c_char,
                      b"sources.c\x00" as *const u8 as *const libc::c_char,
                      222 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 96],
                                                &[libc::c_char; 96]>(b"SRC_Instance SRC_CreateNewInstance(uint32_t, SRC_Type, int, IPAddr *, int, int, double, double)\x00")).as_ptr());
    }
    if min_samples == -(1 as libc::c_int) {
        min_samples = CNF_GetMinSamples()
    }
    if max_samples == -(1 as libc::c_int) {
        max_samples = CNF_GetMaxSamples()
    }
    result = Malloc(::std::mem::size_of::<SRC_Instance_Record>() as libc::c_ulong)
        as *mut SRC_Instance_Record;
    (*result).stats =
        SST_CreateInstance(ref_id, addr, min_samples, max_samples, min_delay, asymmetry);
    if n_sources == max_n_sources {
        /* Reallocate memory */
        max_n_sources = if max_n_sources > 0 as libc::c_int {
            (2 as libc::c_int) * max_n_sources
        } else {
            4 as libc::c_int
        };
        if !sources.is_null() {
            sources = Realloc2(
                sources as *mut libc::c_void,
                max_n_sources as size_t,
                ::std::mem::size_of::<*mut SRC_Instance_Record>() as libc::c_ulong,
            ) as *mut *mut SRC_Instance_Record;
            sort_list = Realloc2(
                sort_list as *mut libc::c_void,
                (3 as libc::c_int * max_n_sources) as size_t,
                ::std::mem::size_of::<Sort_Element>() as libc::c_ulong,
            ) as *mut Sort_Element;
            sel_sources = Realloc2(
                sel_sources as *mut libc::c_void,
                max_n_sources as size_t,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ) as *mut libc::c_int
        } else {
            sources = Malloc2(
                max_n_sources as size_t,
                ::std::mem::size_of::<*mut SRC_Instance_Record>() as libc::c_ulong,
            ) as *mut *mut SRC_Instance_Record;
            sort_list = Malloc2(
                (3 as libc::c_int * max_n_sources) as size_t,
                ::std::mem::size_of::<Sort_Element>() as libc::c_ulong,
            ) as *mut Sort_Element;
            sel_sources = Malloc2(
                max_n_sources as size_t,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ) as *mut libc::c_int
        }
    }
    let ref mut fresh0 = *sources.offset(n_sources as isize);
    *fresh0 = result;
    (*result).index = n_sources;
    (*result).type_0 = type_0;
    (*result).sel_options = sel_options;
    SRC_SetRefid(result, ref_id, addr);
    SRC_ResetInstance(result);
    n_sources += 1;
    return result;
}
/* ================================================== */
/* Function to get rid of a source when it is being unconfigured.
This may cause the current reference source to be reselected, if this
was the reference source or contributed significantly to a
falseticker decision. */
#[no_mangle]
pub unsafe extern "C" fn SRC_DestroyInstance(mut instance: SRC_Instance) {
    let mut dead_index: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if initialised != 0 {
    } else {
        __assert_fail(
            b"initialised\x00" as *const u8 as *const libc::c_char,
            b"sources.c\x00" as *const u8 as *const libc::c_char,
            271 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void SRC_DestroyInstance(SRC_Instance)\x00",
            ))
            .as_ptr(),
        );
    }
    SST_DeleteInstance((*instance).stats);
    dead_index = (*instance).index;
    i = dead_index;
    while i < n_sources - 1 as libc::c_int {
        let ref mut fresh1 = *sources.offset(i as isize);
        *fresh1 = *sources.offset((i + 1 as libc::c_int) as isize);
        (**sources.offset(i as isize)).index = i;
        i += 1
    }
    n_sources -= 1;
    free(instance as *mut libc::c_void);
    /* If this was the previous reference source, we have to reselect! */
    if selected_source_index == dead_index {
        SRC_ReselectSource();
    } else if selected_source_index > dead_index {
        selected_source_index -= 1
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_ResetInstance(mut instance: SRC_Instance) {
    (*instance).active = 0 as libc::c_int;
    (*instance).updates = 0 as libc::c_int;
    (*instance).reachability = 0 as libc::c_int;
    (*instance).reachability_size = 0 as libc::c_int;
    (*instance).distant = 0 as libc::c_int;
    (*instance).status = SRC_BAD_STATS;
    (*instance).sel_score = 1.0f64;
    SST_ResetInstance((*instance).stats);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_SetRefid(
    mut instance: SRC_Instance,
    mut ref_id: uint32_t,
    mut addr: *mut IPAddr,
) {
    (*instance).ref_id = ref_id;
    (*instance).ip_addr = addr;
    SST_SetRefid((*instance).stats, ref_id, addr);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_GetSourcestats(mut instance: SRC_Instance) -> SST_Stats {
    if initialised != 0 {
    } else {
        __assert_fail(
            b"initialised\x00" as *const u8 as *const libc::c_char,
            b"sources.c\x00" as *const u8 as *const libc::c_char,
            320 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"SST_Stats SRC_GetSourcestats(SRC_Instance)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*instance).stats;
}
/* ================================================== */
/* This function is called by one of the source drivers when it has
a new sample that is to be accumulated.

This function causes the frequency estimation to be re-run for the
designated source, and the clock selection procedure to be re-run
afterwards.
*/
#[no_mangle]
pub unsafe extern "C" fn SRC_AccumulateSample(mut inst: SRC_Instance, mut sample: *mut NTP_Sample) {
    if initialised != 0 {
    } else {
        __assert_fail(
            b"initialised\x00" as *const u8 as *const libc::c_char,
            b"sources.c\x00" as *const u8 as *const libc::c_char,
            338 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                b"void SRC_AccumulateSample(SRC_Instance, NTP_Sample *)\x00",
            ))
            .as_ptr(),
        );
    }
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"ip=[%s] t=%s ofs=%f del=%f disp=%f str=%d\x00" as *const u8 as *const libc::c_char,
            source_to_string(inst),
            UTI_TimespecToString(&mut (*sample).time),
            -(*sample).offset,
            (*sample).root_delay,
            (*sample).root_dispersion,
            (*sample).stratum,
        );
    }
    if REF_IsLeapSecondClose() != 0 {
        LOG_Message(
            LOGS_INFO,
            b"Dropping sample around leap second\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    SST_AccumulateSample((*inst).stats, sample);
    SST_DoNewRegression((*inst).stats);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_SetActive(mut inst: SRC_Instance) {
    (*inst).active = 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_UnsetActive(mut inst: SRC_Instance) {
    (*inst).active = 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn special_mode_end() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_sources {
        /* No updates from inactive sources */
        if !((**sources.offset(i as isize)).active == 0) {
            /* Don't expect more updates than from an offline iburst NTP source */
            if !((**sources.offset(i as isize)).reachability_size
                >= 8 as libc::c_int - 1 as libc::c_int)
            {
                /* Check if the source could still have enough samples to be selectable */
                if 8 as libc::c_int
                    - 1 as libc::c_int
                    - (**sources.offset(i as isize)).reachability_size
                    + SST_Samples((**sources.offset(i as isize)).stats)
                    >= 3 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
            }
        }
        i += 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SRC_UpdateReachability(
    mut inst: SRC_Instance,
    mut reachable: libc::c_int,
) {
    (*inst).reachability <<= 1 as libc::c_int;
    (*inst).reachability |= (reachable != 0) as libc::c_int;
    (*inst).reachability = ((*inst).reachability as libc::c_uint)
        .wrapping_rem((1 as libc::c_uint) << 8 as libc::c_int)
        as libc::c_int as libc::c_int;
    if (*inst).reachability_size < 8 as libc::c_int {
        (*inst).reachability_size += 1
    }
    if reachable == 0 && (*inst).index == selected_source_index {
        /* Try to select a better source */
        SRC_SelectSource(0 as SRC_Instance);
    }
    /* Check if special reference update mode failed */
    if REF_GetMode() as libc::c_uint != REF_ModeNormal as libc::c_int as libc::c_uint
        && special_mode_end() != 0
    {
        REF_SetUnsynchronised();
    }
    /* Try to replace NTP sources that are unreachable, falsetickers, or
    have root distance or jitter larger than the allowed maximums */
    if (*inst).type_0 as libc::c_uint == SRC_NTP as libc::c_int as libc::c_uint
        && ((*inst).reachability == 0 && (*inst).reachability_size == 8 as libc::c_int
            || (*inst).status as libc::c_uint == SRC_BAD_DISTANCE as libc::c_int as libc::c_uint
            || (*inst).status as libc::c_uint == SRC_JITTERY as libc::c_int as libc::c_uint
            || (*inst).status as libc::c_uint == SRC_FALSETICKER as libc::c_int as libc::c_uint)
    {
        NSR_HandleBadSource((*inst).ip_addr);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_ResetReachability(mut inst: SRC_Instance) {
    (*inst).reachability = 0 as libc::c_int;
    (*inst).reachability_size = 0 as libc::c_int;
    SRC_UpdateReachability(inst, 0 as libc::c_int);
}
/* ================================================== */
unsafe extern "C" fn log_selection_message(
    mut format: *mut libc::c_char,
    mut arg: *mut libc::c_char,
) {
    if REF_GetMode() as libc::c_uint != REF_ModeNormal as libc::c_int as libc::c_uint {
        return;
    }
    LOG_Message(LOGS_INFO, format, arg);
}
/* ================================================== */
unsafe extern "C" fn compare_sort_elements(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut u: *const Sort_Element = a as *const Sort_Element;
    let mut v: *const Sort_Element = b as *const Sort_Element;
    if (*u).offset < (*v).offset {
        return -(1 as libc::c_int);
    } else if (*u).offset > (*v).offset {
        return 1 as libc::c_int;
    } else if ((*u).tag as libc::c_int) < (*v).tag as libc::c_int {
        return -(1 as libc::c_int);
    } else if (*u).tag as libc::c_int > (*v).tag as libc::c_int {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
/* ================================================== */
unsafe extern "C" fn source_to_string(mut inst: SRC_Instance) -> *mut libc::c_char {
    match (*inst).type_0 as libc::c_uint {
        0 => return UTI_IPToString((*inst).ip_addr),
        1 => return UTI_RefidToString((*inst).ref_id),
        _ => {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"sources.c\x00" as *const u8 as *const libc::c_char,
                476 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                    b"char *source_to_string(SRC_Instance)\x00",
                ))
                .as_ptr(),
            );
        }
    }
    return 0 as *mut libc::c_char;
}
/* ================================================== */
unsafe extern "C" fn mark_ok_sources(mut status: SRC_Status) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_sources {
        if !((**sources.offset(i as isize)).status as libc::c_uint
            != SRC_OK as libc::c_int as libc::c_uint)
        {
            (**sources.offset(i as isize)).status = status
        }
        i += 1
    }
}
/* ================================================== */
unsafe extern "C" fn combine_sources(
    mut n_sel_sources: libc::c_int,
    mut ref_time: *mut timespec,
    mut offset: *mut libc::c_double,
    mut offset_sd: *mut libc::c_double,
    mut frequency: *mut libc::c_double,
    mut frequency_sd: *mut libc::c_double,
    mut skew: *mut libc::c_double,
) -> libc::c_int {
    let mut src_ref_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut src_offset: libc::c_double = 0.;
    let mut src_offset_sd: libc::c_double = 0.;
    let mut src_frequency: libc::c_double = 0.;
    let mut src_frequency_sd: libc::c_double = 0.;
    let mut src_skew: libc::c_double = 0.;
    let mut src_root_delay: libc::c_double = 0.;
    let mut src_root_dispersion: libc::c_double = 0.;
    let mut sel_src_distance: libc::c_double = 0.;
    let mut elapsed: libc::c_double = 0.;
    let mut offset_weight: libc::c_double = 0.;
    let mut sum_offset_weight: libc::c_double = 0.;
    let mut sum_offset: libc::c_double = 0.;
    let mut sum2_offset_sd: libc::c_double = 0.;
    let mut frequency_weight: libc::c_double = 0.;
    let mut sum_frequency_weight: libc::c_double = 0.;
    let mut sum_frequency: libc::c_double = 0.;
    let mut inv_sum2_frequency_sd: libc::c_double = 0.;
    let mut inv_sum2_skew: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut combined: libc::c_int = 0;
    if n_sel_sources == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    sum2_offset_sd = 0.0f64;
    sum_offset = sum2_offset_sd;
    sum_offset_weight = sum_offset;
    inv_sum2_skew = 0.0f64;
    inv_sum2_frequency_sd = inv_sum2_skew;
    sum_frequency = inv_sum2_frequency_sd;
    sum_frequency_weight = sum_frequency;
    sel_src_distance = (**sources.offset(selected_source_index as isize))
        .sel_info
        .root_distance;
    if (**sources.offset(selected_source_index as isize)).type_0 as libc::c_uint
        == SRC_NTP as libc::c_int as libc::c_uint
    {
        sel_src_distance += reselect_distance
    }
    combined = 0 as libc::c_int;
    i = combined;
    while i < n_sel_sources {
        index = *sel_sources.offset(i as isize);
        SST_GetTrackingData(
            (**sources.offset(index as isize)).stats,
            &mut src_ref_time,
            &mut src_offset,
            &mut src_offset_sd,
            &mut src_frequency,
            &mut src_frequency_sd,
            &mut src_skew,
            &mut src_root_delay,
            &mut src_root_dispersion,
        );
        /* Don't include this source if its distance is longer than the distance of
        the selected source multiplied by the limit, their estimated frequencies
        are not close, or it was recently marked as distant */
        if index != selected_source_index
            && ((**sources.offset(index as isize)).sel_info.root_distance
                > combine_limit * sel_src_distance
                || fabs(*frequency - src_frequency)
                    > combine_limit * (*skew + src_skew + LCL_GetMaxClockError()))
        {
            /* Use a smaller penalty in first few updates */
            (**sources.offset(index as isize)).distant =
                if (**sources.offset(index as isize)).reachability_size >= 8 as libc::c_int {
                    32 as libc::c_int
                } else {
                    1 as libc::c_int
                }
        } else if (**sources.offset(index as isize)).distant != 0 {
            let ref mut fresh2 = (**sources.offset(index as isize)).distant;
            *fresh2 -= 1
        }
        if (**sources.offset(index as isize)).distant != 0 {
            (**sources.offset(index as isize)).status = SRC_DISTANT
        } else {
            if (**sources.offset(index as isize)).status as libc::c_uint
                == SRC_OK as libc::c_int as libc::c_uint
            {
                (**sources.offset(index as isize)).status = SRC_UNSELECTED
            }
            elapsed = UTI_DiffTimespecsToDouble(ref_time, &mut src_ref_time);
            src_offset += elapsed * src_frequency;
            src_offset_sd += elapsed * src_frequency_sd;
            offset_weight = 1.0f64 / (**sources.offset(index as isize)).sel_info.root_distance;
            frequency_weight = 1.0f64 / (src_frequency_sd * src_frequency_sd);
            if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
            {
                LOG_Message(LOGS_DEBUG,
                            b"combining index=%d oweight=%e offset=%e osd=%e fweight=%e freq=%e fsd=%e skew=%e\x00"
                                as *const u8 as *const libc::c_char, index,
                            offset_weight, src_offset, src_offset_sd,
                            frequency_weight, src_frequency, src_frequency_sd,
                            src_skew);
            }
            sum_offset_weight += offset_weight;
            sum_offset += offset_weight * src_offset;
            sum2_offset_sd += offset_weight
                * (src_offset_sd * src_offset_sd + (src_offset - *offset) * (src_offset - *offset));
            sum_frequency_weight += frequency_weight;
            sum_frequency += frequency_weight * src_frequency;
            inv_sum2_frequency_sd += 1.0f64 / (src_frequency_sd * src_frequency_sd);
            inv_sum2_skew += 1.0f64 / (src_skew * src_skew);
            combined += 1
        }
        i += 1
    }
    if combined != 0 {
    } else {
        __assert_fail(b"combined\x00" as *const u8 as *const libc::c_char,
                      b"sources.c\x00" as *const u8 as *const libc::c_char,
                      572 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 94],
                                                &[libc::c_char; 94]>(b"int combine_sources(int, struct timespec *, double *, double *, double *, double *, double *)\x00")).as_ptr());
    }
    *offset = sum_offset / sum_offset_weight;
    *offset_sd = sqrt(sum2_offset_sd / sum_offset_weight);
    *frequency = sum_frequency / sum_frequency_weight;
    *frequency_sd = 1.0f64 / sqrt(inv_sum2_frequency_sd);
    *skew = 1.0f64 / sqrt(inv_sum2_skew);
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"combined result offset=%e osd=%e freq=%e fsd=%e skew=%e\x00" as *const u8
                as *const libc::c_char,
            *offset,
            *offset_sd,
            *frequency,
            *frequency_sd,
            *skew,
        );
    }
    return combined;
}
/* ================================================== */
/* This function selects the current reference from amongst the pool
of sources we are holding and updates the local reference */
#[no_mangle]
pub unsafe extern "C" fn SRC_SelectSource(mut updated_inst: SRC_Instance) {
    let mut si: *mut SelectInfo = 0 as *mut SelectInfo;
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut ref_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut j1: libc::c_int = 0;
    let mut j2: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut sel_prefer: libc::c_int = 0;
    let mut n_endpoints: libc::c_int = 0;
    let mut n_sel_sources: libc::c_int = 0;
    let mut sel_req_source: libc::c_int = 0;
    let mut n_badstats_sources: libc::c_int = 0;
    let mut max_sel_reach: libc::c_int = 0;
    let mut max_sel_reach_size: libc::c_int = 0;
    let mut max_badstat_reach: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut best_depth: libc::c_int = 0;
    let mut trust_depth: libc::c_int = 0;
    let mut best_trust_depth: libc::c_int = 0;
    let mut combined: libc::c_int = 0;
    let mut stratum: libc::c_int = 0;
    let mut min_stratum: libc::c_int = 0;
    let mut max_score_index: libc::c_int = 0;
    let mut orphan_stratum: libc::c_int = 0;
    let mut orphan_source: libc::c_int = 0;
    let mut leap_votes: libc::c_int = 0;
    let mut leap_ins: libc::c_int = 0;
    let mut leap_del: libc::c_int = 0;
    let mut src_offset: libc::c_double = 0.;
    let mut src_offset_sd: libc::c_double = 0.;
    let mut src_frequency: libc::c_double = 0.;
    let mut src_frequency_sd: libc::c_double = 0.;
    let mut src_skew: libc::c_double = 0.;
    let mut src_root_delay: libc::c_double = 0.;
    let mut src_root_dispersion: libc::c_double = 0.;
    let mut best_lo: libc::c_double = 0.;
    let mut best_hi: libc::c_double = 0.;
    let mut distance: libc::c_double = 0.;
    let mut sel_src_distance: libc::c_double = 0.;
    let mut max_score: libc::c_double = 0.;
    let mut first_sample_ago: libc::c_double = 0.;
    let mut max_reach_sample_ago: libc::c_double = 0.;
    let mut leap_status: NTP_Leap = LEAP_Normal;
    if !updated_inst.is_null() {
        (*updated_inst).updates += 1
    }
    if n_sources == 0 as libc::c_int {
        /* In this case, we clearly cannot synchronise to anything */
        if selected_source_index != -(1 as libc::c_int) {
            log_selection_message(
                b"Can\'t synchronise: no sources\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_char,
            );
            selected_source_index = -(1 as libc::c_int)
        }
        return;
    }
    /* This is accurate enough and cheaper than calling LCL_ReadCookedTime */
    SCH_GetLastEventTime(&mut now, 0 as *mut libc::c_double, 0 as *mut timespec);
    /* Step 1 - build intervals about each source */
    n_endpoints = 0 as libc::c_int;
    n_sel_sources = 0 as libc::c_int;
    n_badstats_sources = 0 as libc::c_int;
    sel_req_source = 0 as libc::c_int;
    max_badstat_reach = 0 as libc::c_int;
    max_sel_reach = max_badstat_reach;
    max_sel_reach_size = 0 as libc::c_int;
    max_reach_sample_ago = 0.0f64;
    i = 0 as libc::c_int;
    while i < n_sources {
        if (**sources.offset(i as isize)).status as libc::c_uint
            != SRC_OK as libc::c_int as libc::c_uint
        {
        } else {
            __assert_fail(
                b"sources[i]->status != SRC_OK\x00" as *const u8 as *const libc::c_char,
                b"sources.c\x00" as *const u8 as *const libc::c_char,
                631 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                    b"void SRC_SelectSource(SRC_Instance)\x00",
                ))
                .as_ptr(),
            );
        }
        /* If some sources are specified with the require option, at least one
        of them will have to be selectable in order to update the clock */
        if (**sources.offset(i as isize)).sel_options & 0x8 as libc::c_int != 0 {
            sel_req_source = 1 as libc::c_int
        }
        /* Ignore sources which were added with the noselect option */
        if (**sources.offset(i as isize)).sel_options & 0x1 as libc::c_int != 0 {
            (**sources.offset(i as isize)).status = SRC_UNSELECTABLE
        } else {
            si = &mut (**sources.offset(i as isize)).sel_info;
            SST_GetSelectionData(
                (**sources.offset(i as isize)).stats,
                &mut now,
                &mut (*si).stratum,
                &mut (*si).leap,
                &mut (*si).lo_limit,
                &mut (*si).hi_limit,
                &mut (*si).root_distance,
                &mut (*si).std_dev,
                &mut first_sample_ago,
                &mut (*si).last_sample_ago,
                &mut (*si).select_ok,
            );
            if (*si).select_ok == 0 {
                n_badstats_sources += 1;
                (**sources.offset(i as isize)).status = SRC_BAD_STATS;
                if max_badstat_reach < (**sources.offset(i as isize)).reachability {
                    max_badstat_reach = (**sources.offset(i as isize)).reachability
                }
            } else {
                /* Include extra dispersion in the root distance of sources that don't
                have new samples (the last sample is older than span of all samples) */
                if first_sample_ago < 2.0f64 * (*si).last_sample_ago {
                    let mut extra_disp: libc::c_double = LCL_GetMaxClockError()
                        * (2.0f64 * (*si).last_sample_ago - first_sample_ago);
                    (*si).root_distance += extra_disp;
                    (*si).lo_limit -= extra_disp;
                    (*si).hi_limit += extra_disp
                }
                /* Require the root distance to be below the allowed maximum */
                if (*si).root_distance > max_distance {
                    (**sources.offset(i as isize)).status = SRC_BAD_DISTANCE
                } else if (*si).std_dev > max_jitter {
                    (**sources.offset(i as isize)).status = SRC_JITTERY
                } else {
                    /* And the same applies for the estimated standard deviation */
                    (**sources.offset(i as isize)).status = SRC_OK; /* For now */
                    if (**sources.offset(i as isize)).reachability != 0
                        && max_reach_sample_ago < first_sample_ago
                    {
                        max_reach_sample_ago = first_sample_ago
                    }
                    if max_sel_reach < (**sources.offset(i as isize)).reachability {
                        max_sel_reach = (**sources.offset(i as isize)).reachability
                    }
                    if max_sel_reach_size < (**sources.offset(i as isize)).reachability_size {
                        max_sel_reach_size = (**sources.offset(i as isize)).reachability_size
                    }
                }
            }
        }
        i += 1
    }
    orphan_stratum = REF_GetOrphanStratum();
    orphan_source = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < n_sources {
        if !((**sources.offset(i as isize)).status as libc::c_uint
            != SRC_OK as libc::c_int as libc::c_uint)
        {
            si = &mut (**sources.offset(i as isize)).sel_info;
            /* Reachability is not a requirement for selection.  An unreachable source
            can still be selected if its newest sample is not older than the oldest
            sample from reachable sources. */
            if (**sources.offset(i as isize)).reachability == 0
                && max_reach_sample_ago < (*si).last_sample_ago
            {
                (**sources.offset(i as isize)).status = SRC_STALE
            } else if (*si).stratum >= orphan_stratum
                && (**sources.offset(i as isize)).type_0 as libc::c_uint
                    == SRC_NTP as libc::c_int as libc::c_uint
            {
                (**sources.offset(i as isize)).status = SRC_ORPHAN;
                if (*si).stratum == orphan_stratum
                    && (**sources.offset(i as isize)).reachability != 0
                    && (orphan_source == -(1 as libc::c_int)
                        || (**sources.offset(i as isize)).ref_id
                            < (**sources.offset(orphan_source as isize)).ref_id)
                {
                    orphan_source = i
                }
            } else {
                n_sel_sources += 1
            }
        }
        i += 1
    }
    /* When the local reference is configured with the orphan option, NTP
    sources that have stratum equal to the configured local stratum are
    considered to be orphans (i.e. serving local time while not being
    synchronised with real time) and are excluded from the normal source
    selection.  Sources with stratum larger than the local stratum are
    considered to be directly on indirectly synchronised to an orphan and
    are always ignored.

    If no selectable source is available and all orphan sources have
    reference IDs larger than the local ID, no source will be selected and
    the local reference mode will be activated at some point, i.e. this host
    will become an orphan.  Otherwise, the orphan source with the smallest
    reference ID will be selected.  This ensures a group of servers polling
    each other (with the same orphan configuration) which have no external
    source can settle down to a state where only one server is serving its
    local unsychronised time and others are synchronised to it. */
    /* If no selectable source is available, consider the orphan source */
    if n_sel_sources == 0 && orphan_source != -(1 as libc::c_int) {
        let mut local_ref_id: uint32_t =
            NSR_GetLocalRefid((**sources.offset(orphan_source as isize)).ip_addr);
        if local_ref_id == 0 {
            LOG_Message(
                LOGS_ERR,
                b"Unknown local refid in orphan mode\x00" as *const u8 as *const libc::c_char,
            );
        } else if (**sources.offset(orphan_source as isize)).ref_id < local_ref_id {
            (**sources.offset(orphan_source as isize)).status = SRC_OK;
            n_sel_sources = 1 as libc::c_int;
            if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
            {
                LOG_Message(
                    LOGS_DEBUG,
                    b"selecting orphan refid=%x\x00" as *const u8 as *const libc::c_char,
                    (**sources.offset(orphan_source as isize)).ref_id,
                );
            }
        }
    }
    i = 0 as libc::c_int;
    while i < n_sources {
        if !((**sources.offset(i as isize)).status as libc::c_uint
            != SRC_OK as libc::c_int as libc::c_uint)
        {
            si = &mut (**sources.offset(i as isize)).sel_info;
            j1 = n_endpoints;
            j2 = j1 + 1 as libc::c_int;
            (*sort_list.offset(j1 as isize)).index = i;
            (*sort_list.offset(j1 as isize)).offset = (*si).lo_limit;
            (*sort_list.offset(j1 as isize)).tag = LOW;
            (*sort_list.offset(j2 as isize)).index = i;
            (*sort_list.offset(j2 as isize)).offset = (*si).hi_limit;
            (*sort_list.offset(j2 as isize)).tag = HIGH;
            n_endpoints += 2 as libc::c_int
        }
        i += 1
    }
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"badstat=%d sel=%d badstat_reach=%x sel_reach=%x size=%d max_reach_ago=%f\x00"
                as *const u8 as *const libc::c_char,
            n_badstats_sources,
            n_sel_sources,
            max_badstat_reach as libc::c_uint,
            max_sel_reach as libc::c_uint,
            max_sel_reach_size,
            max_reach_sample_ago,
        );
    }
    /* Wait for the next call if we have no source selected and there is
    a source with bad stats (has less than 3 samples) with reachability
    equal to shifted maximum reachability of sources with valid stats.
    This delays selecting source on start with servers using the same
    polling interval until they all have valid stats. */
    if n_badstats_sources != 0
        && n_sel_sources != 0
        && selected_source_index == -(1 as libc::c_int)
        && max_sel_reach_size < 8 as libc::c_int
        && max_sel_reach >> 1 as libc::c_int == max_badstat_reach
    {
        mark_ok_sources(SRC_WAITS_STATS);
        return;
    }
    if n_endpoints == 0 as libc::c_int {
        /* No sources provided valid endpoints */
        if selected_source_index != -(1 as libc::c_int) {
            log_selection_message(
                b"Can\'t synchronise: no selectable sources\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_char,
            );
            selected_source_index = -(1 as libc::c_int)
        }
        return;
    }
    /* Now sort the endpoint list */
    qsort(
        sort_list as *mut libc::c_void,
        n_endpoints as size_t,
        ::std::mem::size_of::<Sort_Element>() as libc::c_ulong,
        Some(
            compare_sort_elements
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    /* Now search for the interval which is contained in the most
    individual source intervals.  Any source which overlaps this
    will be a candidate.

    If we get a case like

    <----------------------->
        <-->
                 <-->
        <===========>

    we will build the interval as shown with '=', whereas with an extra source we get

    <----------------------->
       <------->
        <-->
                 <-->
        <==>

    The first case is just bad luck - we need extra sources to
    detect the falseticker, so just make an arbitrary choice based
    on stratum & stability etc.

    Intervals from sources specified with the trust option have higher
    priority in the search.
    */
    best_trust_depth = 0 as libc::c_int;
    trust_depth = best_trust_depth;
    best_depth = 0 as libc::c_int;
    depth = best_depth;
    best_hi = 0.0f64;
    best_lo = best_hi;
    i = 0 as libc::c_int;
    while i < n_endpoints {
        match (*sort_list.offset(i as isize)).tag as libc::c_int {
            -1 => {
                depth += 1;
                if (**sources.offset((*sort_list.offset(i as isize)).index as isize)).sel_options
                    & 0x4 as libc::c_int
                    != 0
                {
                    trust_depth += 1
                }
                if trust_depth > best_trust_depth
                    || trust_depth == best_trust_depth && depth > best_depth
                {
                    best_trust_depth = trust_depth;
                    best_depth = depth;
                    best_lo = (*sort_list.offset(i as isize)).offset
                }
            }
            1 => {
                if trust_depth == best_trust_depth && depth == best_depth {
                    best_hi = (*sort_list.offset(i as isize)).offset
                }
                if (**sources.offset((*sort_list.offset(i as isize)).index as isize)).sel_options
                    & 0x4 as libc::c_int
                    != 0
                {
                    trust_depth -= 1
                }
                depth -= 1
            }
            _ => {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"sources.c\x00" as *const u8 as *const libc::c_char,
                    852 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                        b"void SRC_SelectSource(SRC_Instance)\x00",
                    ))
                    .as_ptr(),
                );
            }
        }
        i += 1
    }
    if best_depth <= n_sel_sources / 2 as libc::c_int && best_trust_depth == 0 {
        /* Could not even get half the reachable sources to agree and there
        are no trusted sources - clearly we can't synchronise */
        if selected_source_index != -(1 as libc::c_int) {
            log_selection_message(
                b"Can\'t synchronise: no majority\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_char,
            );
            REF_SetUnsynchronised();
            selected_source_index = -(1 as libc::c_int)
        }
        /* .. and mark all sources as falsetickers (so they appear thus
        on the outputs from the command client) */
        mark_ok_sources(SRC_FALSETICKER);
        return;
    }
    /* We have our interval, now work out which source are in it,
    i.e. build list of admissible sources. */
    n_sel_sources = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n_sources {
        /* This should be the same condition to get into the endpoint
        list */
        if !((**sources.offset(i as isize)).status as libc::c_uint
            != SRC_OK as libc::c_int as libc::c_uint)
        {
            /* Check if source's interval contains the best interval, or is wholly
            contained within it.  If there are any trusted sources the first
            condition is applied only to them to not allow non-trusted sources to
            move the final offset outside the interval. */
            if (best_trust_depth == 0
                || (**sources.offset(i as isize)).sel_options & 0x4 as libc::c_int != 0)
                && (**sources.offset(i as isize)).sel_info.lo_limit <= best_lo
                && (**sources.offset(i as isize)).sel_info.hi_limit >= best_hi
                || (**sources.offset(i as isize)).sel_info.lo_limit >= best_lo
                    && (**sources.offset(i as isize)).sel_info.hi_limit <= best_hi
            {
                let fresh3 = n_sel_sources;
                n_sel_sources = n_sel_sources + 1;
                *sel_sources.offset(fresh3 as isize) = i;
                if (**sources.offset(i as isize)).sel_options & 0x8 as libc::c_int != 0 {
                    sel_req_source = 0 as libc::c_int
                }
            } else if (**sources.offset(i as isize)).sel_info.lo_limit <= best_lo
                && (**sources.offset(i as isize)).sel_info.hi_limit >= best_hi
            {
                (**sources.offset(i as isize)).status = SRC_UNTRUSTED
            } else {
                (**sources.offset(i as isize)).status = SRC_FALSETICKER
            }
        }
        i += 1
    }
    if n_sel_sources == 0 || sel_req_source != 0 || n_sel_sources < CNF_GetMinSources() {
        if selected_source_index != -(1 as libc::c_int) {
            log_selection_message(
                b"Can\'t synchronise: %s selectable sources\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                if n_sel_sources == 0 {
                    b"no\x00" as *const u8 as *const libc::c_char
                } else if sel_req_source != 0 {
                    b"no required source in\x00" as *const u8 as *const libc::c_char
                } else {
                    b"not enough\x00" as *const u8 as *const libc::c_char
                } as *mut libc::c_char,
            );
            selected_source_index = -(1 as libc::c_int)
        }
        mark_ok_sources(SRC_WAITS_SOURCES);
        return;
    }
    /* Accept leap second status if more than half of selectable (and trusted
    if there are any) sources agree */
    leap_votes = 0 as libc::c_int;
    leap_del = leap_votes;
    leap_ins = leap_del;
    i = leap_ins;
    while i < n_sel_sources {
        index = *sel_sources.offset(i as isize);
        if !(best_trust_depth != 0
            && (**sources.offset(index as isize)).sel_options & 0x4 as libc::c_int == 0)
        {
            leap_votes += 1;
            if (**sources.offset(index as isize)).sel_info.leap as libc::c_uint
                == LEAP_InsertSecond as libc::c_int as libc::c_uint
            {
                leap_ins += 1
            } else if (**sources.offset(index as isize)).sel_info.leap as libc::c_uint
                == LEAP_DeleteSecond as libc::c_int as libc::c_uint
            {
                leap_del += 1
            }
        }
        i += 1
    }
    if leap_ins > leap_votes / 2 as libc::c_int {
        leap_status = LEAP_InsertSecond
    } else if leap_del > leap_votes / 2 as libc::c_int {
        leap_status = LEAP_DeleteSecond
    } else {
        leap_status = LEAP_Normal
    }
    /* If there are any sources with prefer option, reduce the list again
    only to the preferred sources */
    i = 0 as libc::c_int;
    while i < n_sel_sources {
        if (**sources.offset(*sel_sources.offset(i as isize) as isize)).sel_options
            & 0x2 as libc::c_int
            != 0
        {
            break;
        }
        i += 1
    }
    if i < n_sel_sources {
        j = 0 as libc::c_int;
        i = j;
        while i < n_sel_sources {
            if (**sources.offset(*sel_sources.offset(i as isize) as isize)).sel_options
                & 0x2 as libc::c_int
                == 0
            {
                (**sources.offset(*sel_sources.offset(i as isize) as isize)).status =
                    SRC_NONPREFERRED
            } else {
                let fresh4 = j;
                j = j + 1;
                *sel_sources.offset(fresh4 as isize) = *sel_sources.offset(i as isize)
            }
            i += 1
        }
        if j > 0 as libc::c_int {
        } else {
            __assert_fail(
                b"j > 0\x00" as *const u8 as *const libc::c_char,
                b"sources.c\x00" as *const u8 as *const libc::c_char,
                950 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                    b"void SRC_SelectSource(SRC_Instance)\x00",
                ))
                .as_ptr(),
            );
        }
        n_sel_sources = j;
        sel_prefer = 1 as libc::c_int
    } else {
        sel_prefer = 0 as libc::c_int
    }
    /* Find minimum stratum */
    index = *sel_sources.offset(0 as libc::c_int as isize);
    min_stratum = (**sources.offset(index as isize)).sel_info.stratum;
    i = 1 as libc::c_int;
    while i < n_sel_sources {
        index = *sel_sources.offset(i as isize);
        stratum = (**sources.offset(index as isize)).sel_info.stratum;
        if stratum < min_stratum {
            min_stratum = stratum
        }
        i += 1
    }
    /* Update scores and find the source with maximum score */
    max_score_index = -(1 as libc::c_int);
    max_score = 0.0f64;
    sel_src_distance = 0.0f64;
    if selected_source_index != -(1 as libc::c_int) {
        sel_src_distance = (**sources.offset(selected_source_index as isize))
            .sel_info
            .root_distance
            + ((**sources.offset(selected_source_index as isize))
                .sel_info
                .stratum
                - min_stratum) as libc::c_double
                * stratum_weight
    }
    i = 0 as libc::c_int;
    while i < n_sources {
        /* Reset score for non-selectable sources */
        if (**sources.offset(i as isize)).status as libc::c_uint
            != SRC_OK as libc::c_int as libc::c_uint
            || sel_prefer != 0
                && (**sources.offset(i as isize)).sel_options & 0x2 as libc::c_int == 0
        {
            (**sources.offset(i as isize)).sel_score = 1.0f64;
            (**sources.offset(i as isize)).distant = 32 as libc::c_int
        } else {
            distance = (**sources.offset(i as isize)).sel_info.root_distance
                + ((**sources.offset(i as isize)).sel_info.stratum - min_stratum) as libc::c_double
                    * stratum_weight;
            if (**sources.offset(i as isize)).type_0 as libc::c_uint
                == SRC_NTP as libc::c_int as libc::c_uint
            {
                distance += reselect_distance
            }
            if selected_source_index != -(1 as libc::c_int) {
                /* Update score, but only for source pairs where one source
                has a new sample */
                if *sources.offset(i as isize) == updated_inst
                    || *sources.offset(selected_source_index as isize) == updated_inst
                {
                    (**sources.offset(i as isize)).sel_score *= sel_src_distance / distance;
                    if (**sources.offset(i as isize)).sel_score < 1.0f64 {
                        (**sources.offset(i as isize)).sel_score = 1.0f64
                    }
                }
            } else {
                /* When there is no selected source yet, assign scores so that the
                source with minimum distance will have maximum score.  The scores
                will be reset when the source is selected later in this function. */
                (**sources.offset(i as isize)).sel_score = 1.0f64 / distance
            }
            if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
            {
                LOG_Message(
                    LOGS_DEBUG,
                    b"select score=%f refid=%x match_refid=%x status=%u dist=%f\x00" as *const u8
                        as *const libc::c_char,
                    (**sources.offset(i as isize)).sel_score,
                    (**sources.offset(i as isize)).ref_id,
                    if !updated_inst.is_null() {
                        (*updated_inst).ref_id
                    } else {
                        0 as libc::c_int as libc::c_uint
                    },
                    (**sources.offset(i as isize)).status as libc::c_uint,
                    distance,
                );
            }
            if max_score < (**sources.offset(i as isize)).sel_score {
                max_score = (**sources.offset(i as isize)).sel_score;
                max_score_index = i
            }
        }
        i += 1
    }
    if max_score_index != -(1 as libc::c_int) {
    } else {
        __assert_fail(
            b"max_score_index != INVALID_SOURCE\x00" as *const u8 as *const libc::c_char,
            b"sources.c\x00" as *const u8 as *const libc::c_char,
            1021 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"void SRC_SelectSource(SRC_Instance)\x00",
            ))
            .as_ptr(),
        );
    }
    /* Is the current source still a survivor and no other source has reached
    the score limit? */
    if selected_source_index == -(1 as libc::c_int)
        || (**sources.offset(selected_source_index as isize)).status as libc::c_uint
            != SRC_OK as libc::c_int as libc::c_uint
        || max_score_index != selected_source_index && max_score > 10.0f64
    {
        /* Before selecting the new synchronisation source wait until the reference
        can be updated */
        if (**sources.offset(max_score_index as isize)).updates == 0 as libc::c_int {
            selected_source_index = -(1 as libc::c_int);
            mark_ok_sources(SRC_WAITS_UPDATE);
            if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
            {
                LOG_Message(
                    LOGS_DEBUG,
                    b"best source has no updates\x00" as *const u8 as *const libc::c_char,
                );
            }
            return;
        }
        selected_source_index = max_score_index;
        log_selection_message(
            b"Selected source %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            source_to_string(*sources.offset(selected_source_index as isize)),
        );
        /* New source has been selected, reset all scores */
        i = 0 as libc::c_int;
        while i < n_sources {
            (**sources.offset(i as isize)).sel_score = 1.0f64;
            (**sources.offset(i as isize)).distant = 0 as libc::c_int;
            i += 1
        }
    }
    (**sources.offset(selected_source_index as isize)).status = SRC_SELECTED;
    /* Don't update reference when the selected source has no new samples */
    if (**sources.offset(selected_source_index as isize)).updates == 0 as libc::c_int {
        /* Mark the remaining sources as last combine_sources() call */
        i = 0 as libc::c_int;
        while i < n_sel_sources {
            index = *sel_sources.offset(i as isize);
            if (**sources.offset(index as isize)).status as libc::c_uint
                == SRC_OK as libc::c_int as libc::c_uint
            {
                (**sources.offset(index as isize)).status =
                    if (**sources.offset(index as isize)).distant != 0 {
                        SRC_DISTANT as libc::c_int
                    } else {
                        SRC_UNSELECTED as libc::c_int
                    } as SRC_Status
            }
            i += 1
        }
        return;
    }
    i = 0 as libc::c_int;
    while i < n_sources {
        (**sources.offset(i as isize)).updates = 0 as libc::c_int;
        i += 1
    }
    /* Now just use the statistics of the selected source combined with
    the other selectable sources for trimming the local clock */
    SST_GetTrackingData(
        (**sources.offset(selected_source_index as isize)).stats,
        &mut ref_time,
        &mut src_offset,
        &mut src_offset_sd,
        &mut src_frequency,
        &mut src_frequency_sd,
        &mut src_skew,
        &mut src_root_delay,
        &mut src_root_dispersion,
    );
    combined = combine_sources(
        n_sel_sources,
        &mut ref_time,
        &mut src_offset,
        &mut src_offset_sd,
        &mut src_frequency,
        &mut src_frequency_sd,
        &mut src_skew,
    );
    REF_SetReference(
        (**sources.offset(selected_source_index as isize))
            .sel_info
            .stratum,
        leap_status,
        combined,
        (**sources.offset(selected_source_index as isize)).ref_id,
        (**sources.offset(selected_source_index as isize)).ip_addr,
        &mut ref_time,
        src_offset,
        src_offset_sd,
        src_frequency,
        src_frequency_sd,
        src_skew,
        src_root_delay,
        src_root_dispersion,
    );
}
/* ================================================== */
/* Force reselecting the best source */
#[no_mangle]
pub unsafe extern "C" fn SRC_ReselectSource() {
    selected_source_index = -(1 as libc::c_int);
    SRC_SelectSource(0 as SRC_Instance);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_SetReselectDistance(mut distance: libc::c_double) {
    if reselect_distance != distance {
        reselect_distance = distance;
        LOG_Message(
            LOGS_INFO,
            b"New reselect distance %f\x00" as *const u8 as *const libc::c_char,
            distance,
        );
    };
}
/* ================================================== */
/* Forward prototype */
/* ================================================== */
/* This routine is registered as a callback with the local clock
module, to be called whenever the local clock changes frequency or
is slewed.  It runs through all the existing source statistics, and
adjusts them to make them look as though they were sampled under
the new regime. */
unsafe extern "C" fn slew_sources(
    mut raw: *mut timespec,
    mut cooked: *mut timespec,
    mut dfreq: libc::c_double,
    mut doffset: libc::c_double,
    mut change_type: LCL_ChangeType,
    mut anything: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_sources {
        if change_type as libc::c_uint == LCL_ChangeUnknownStep as libc::c_int as libc::c_uint {
            SST_ResetInstance((**sources.offset(i as isize)).stats);
        } else {
            SST_SlewSamples((**sources.offset(i as isize)).stats, cooked, dfreq, doffset);
        }
        i += 1
    }
    if change_type as libc::c_uint == LCL_ChangeUnknownStep as libc::c_int as libc::c_uint {
        /* After resetting no source is selectable, set reference unsynchronised */
        SRC_SelectSource(0 as SRC_Instance);
    };
}
/* ================================================== */
/* This routine is called when an indeterminate offset is introduced
into the local time. */
unsafe extern "C" fn add_dispersion(
    mut dispersion: libc::c_double,
    mut anything: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_sources {
        SST_AddDispersion((**sources.offset(i as isize)).stats, dispersion);
        i += 1
    }
}
/* ================================================== */
unsafe extern "C" fn open_dumpfile(mut inst: SRC_Instance, mut mode: libc::c_char) -> *mut FILE {
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut dumpdir: *mut libc::c_char = 0 as *mut libc::c_char;
    dumpdir = CNF_GetDumpDir();
    if *dumpdir.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
        LOG_Message(
            LOGS_WARN,
            b"dumpdir not specified\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FILE;
    }
    /* Include IP address in the name for NTP sources, or reference ID in hex */
    if (*inst).type_0 as libc::c_uint == SRC_NTP as libc::c_int as libc::c_uint {
        snprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            source_to_string(inst),
        );
    } else {
        snprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"refid:%08x\x00" as *const u8 as *const libc::c_char,
            (*inst).ref_id,
        );
    }
    return UTI_OpenFile(
        dumpdir,
        filename.as_mut_ptr(),
        b".dat\x00" as *const u8 as *const libc::c_char,
        mode,
        0o644 as libc::c_int as mode_t,
    );
}
/* ================================================== */
/* This is called to dump out the source measurement registers */
#[no_mangle]
pub unsafe extern "C" fn SRC_DumpSources() {
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_sources {
        out = open_dumpfile(*sources.offset(i as isize), 'w' as i32 as libc::c_char);
        if !out.is_null() {
            SST_SaveToFile((**sources.offset(i as isize)).stats, out);
            fclose(out);
        }
        i += 1
    }
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_ReloadSources() {
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_sources {
        in_0 = open_dumpfile(*sources.offset(i as isize), 'r' as i32 as libc::c_char);
        if !in_0.is_null() {
            if SST_LoadFromFile((**sources.offset(i as isize)).stats, in_0) == 0 {
                LOG_Message(
                    LOGS_WARN,
                    b"Could not load dump file for %s\x00" as *const u8 as *const libc::c_char,
                    source_to_string(*sources.offset(i as isize)),
                );
            } else {
                LOG_Message(
                    LOGS_INFO,
                    b"Loaded dump file for %s\x00" as *const u8 as *const libc::c_char,
                    source_to_string(*sources.offset(i as isize)),
                );
            }
            fclose(in_0);
        }
        i += 1
    }
}
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Richard P. Curnow  1997-2002
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

 This is the header for the module that manages the collection of all
 sources that we are making measurements from.  This include all NTP
 servers & peers, locally connected reference sources, eye/wristwatch
 drivers etc */
/* Size of the source reachability register */
/* This datatype is used to hold information about sources.  The
instance must be passed when calling many of the interface
functions */
/* Initialisation function */
/* Finalisation function */
/* NTP client/peer */
/* Rerefence clock */
/* Function to create a new instance.  This would be called by one of
the individual source-type instance creation routines. */
/* Function to get rid of a source when it is being unconfigured.
This may cause the current reference source to be reselected, if this
was the reference source or contributed significantly to a
falseticker decision. */
/* Function to reset a source */
/* Function to change the sources's reference ID and IP address */
/* Function to get access to the sourcestats instance */
/* This function is called by one of the source drivers when it has
a new sample that is to be accumulated */
/* This routine sets the source as receiving reachability updates */
/* This routine sets the source as not receiving reachability updates */
/* This routine updates the reachability register */
/* This routine marks the source unreachable */
/* This routine is used to select the best source from amongst those
we currently have valid data on, and use it as the tracking base
for the local time.  Updates are made to the local reference only
when the selected source was updated (set as updated_inst) since
the last reference update.  This avoids updating the frequency
tracking for every sample from other sources - only the ones from
the selected reference make a difference. */
/* Force reselecting the best source */
/* Set reselect distance */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_RemoveDumpFiles() {
    let mut pattern: [libc::c_char; 4096] = [0; 4096];
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut dumpdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip_addr: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    let mut gl: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut libc::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    let mut i: size_t = 0;
    dumpdir = CNF_GetDumpDir();
    if *dumpdir.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        || snprintf(
            pattern.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            b"%s/*.dat\x00" as *const u8 as *const libc::c_char,
            dumpdir,
        ) as libc::c_ulong
            >= ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
    {
        return;
    }
    if glob(pattern.as_mut_ptr(), 0 as libc::c_int, None, &mut gl) != 0 {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < gl.gl_pathc {
        s = strrchr(*gl.gl_pathv.offset(i as isize), '/' as i32);
        if !(s.is_null()
            || snprintf(
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                s.offset(1 as libc::c_int as isize),
            ) as libc::c_ulong
                >= ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
        {
            /* Remove .dat extension */
            if !(strlen(name.as_mut_ptr()) < 4 as libc::c_int as libc::c_ulong) {
                name[strlen(name.as_mut_ptr()).wrapping_sub(4 as libc::c_int as libc::c_ulong)
                    as usize] = '\u{0}' as i32 as libc::c_char;
                /* Check if it looks like name of an actual dump file */
                if !(strncmp(
                    name.as_mut_ptr(),
                    b"refid:\x00" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) != 0
                    && UTI_StringToIP(name.as_mut_ptr(), &mut ip_addr) == 0)
                {
                    (UTI_RemoveFile(
                        0 as *const libc::c_char,
                        *gl.gl_pathv.offset(i as isize),
                        0 as *const libc::c_char,
                    )) == 0;
                }
            }
        }
        i = i.wrapping_add(1)
    }
    globfree(&mut gl);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_IsSyncPeer(mut inst: SRC_Instance) -> libc::c_int {
    if (*inst).index == selected_source_index {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_IsReachable(mut inst: SRC_Instance) -> libc::c_int {
    return ((*inst).reachability != 0 as libc::c_int) as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_ReadNumberOfSources() -> libc::c_int {
    return n_sources;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_ActiveSources() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    r = 0 as libc::c_int;
    i = r;
    while i < n_sources {
        if (**sources.offset(i as isize)).active != 0 {
            r += 1
        }
        i += 1
    }
    return r;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_ReportSource(
    mut index: libc::c_int,
    mut report: *mut RPT_SourceReport,
    mut now: *mut timespec,
) -> libc::c_int {
    let mut src: SRC_Instance = 0 as *mut SRC_Instance_Record;
    if index >= n_sources || index < 0 as libc::c_int {
        return 0 as libc::c_int;
    } else {
        src = *sources.offset(index as isize);
        if !(*src).ip_addr.is_null() {
            (*report).ip_addr = *(*src).ip_addr
        } else {
            /* Use refid as an address */
            (*report).ip_addr.addr.in4 = (*src).ref_id;
            (*report).ip_addr.family = 1 as libc::c_int as uint16_t
        }
        match (*src).status as libc::c_uint {
            9 => (*report).state = RPT_FALSETICKER,
            4 => (*report).state = RPT_JITTERY,
            8 | 10 | 11 | 12 | 13 | 14 => (*report).state = RPT_OUTLIER,
            15 => (*report).state = RPT_CANDIDATE,
            16 => (*report).state = RPT_SYNC,
            _ => (*report).state = RPT_UNREACH,
        }
        (*report).sel_options = (*src).sel_options;
        (*report).reachability = (*src).reachability;
        /* Call stats module to fill out estimates */
        SST_DoSourceReport((*src).stats, report, now);
        return 1 as libc::c_int;
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_ReportSourcestats(
    mut index: libc::c_int,
    mut report: *mut RPT_SourcestatsReport,
    mut now: *mut timespec,
) -> libc::c_int {
    let mut src: SRC_Instance = 0 as *mut SRC_Instance_Record;
    if index >= n_sources || index < 0 as libc::c_int {
        return 0 as libc::c_int;
    } else {
        src = *sources.offset(index as isize);
        (*report).ref_id = (*src).ref_id;
        if !(*src).ip_addr.is_null() {
            (*report).ip_addr = *(*src).ip_addr
        } else {
            (*report).ip_addr.family = 0 as libc::c_int as uint16_t
        }
        SST_DoSourcestatsReport((*src).stats, report, now);
        return 1 as libc::c_int;
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SRC_GetType(mut index: libc::c_int) -> SRC_Type {
    if index >= n_sources || index < 0 as libc::c_int {
        return 4294967295 as SRC_Type;
    }
    return (**sources.offset(index as isize)).type_0;
}
/* ================================================== */
