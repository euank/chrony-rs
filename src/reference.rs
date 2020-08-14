#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn exp(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn mktime(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    #[no_mangle]
    fn gmtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn tzset();
    #[no_mangle]
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    #[no_mangle]
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn Malloc2(nmemb: size_t, size: size_t) -> *mut libc::c_void;
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
    #[no_mangle]
    fn UTI_ZeroTimespec(ts: *mut timespec);
    /* Check if a timespec is zero */
    #[no_mangle]
    fn UTI_IsZeroTimespec(ts: *mut timespec) -> libc::c_int;
    /* Calculate result = a - b and return as a double */
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec) -> libc::c_double;
    /* Add a double increment to a timespec to get a new one. 'start' is
    the starting time, 'end' is the result that we return.  This is
    safe to use if start and end are the same */
    #[no_mangle]
    fn UTI_AddDoubleToTimespec(start: *mut timespec, increment: libc::c_double, end: *mut timespec);
    /* Convert ref_id into a temporary string, for diagnostics */
    #[no_mangle]
    fn UTI_RefidToString(ref_id: uint32_t) -> *mut libc::c_char;
    /* Convert an IP address to string, for diagnostics */
    #[no_mangle]
    fn UTI_IPToString(ip: *mut IPAddr) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_TimeToLogForm(t: time_t) -> *mut libc::c_char;
    /* Adjust time following a frequency/offset change */
    #[no_mangle]
    fn UTI_AdjustTimespec(
        old_ts: *mut timespec,
        when: *mut timespec,
        new_ts: *mut timespec,
        delta_time: *mut libc::c_double,
        dfreq: libc::c_double,
        doffset: libc::c_double,
    );
    #[no_mangle]
    fn UTI_OpenFile(
        basedir: *const libc::c_char,
        name: *const libc::c_char,
        suffix: *const libc::c_char,
        mode_0: libc::c_char,
        perm: mode_t,
    ) -> *mut FILE;
    #[no_mangle]
    fn UTI_RenameTempFile(
        basedir: *const libc::c_char,
        name: *const libc::c_char,
        old_suffix: *const libc::c_char,
        new_suffix: *const libc::c_char,
    ) -> libc::c_int;
    /* Fill buffer with random bytes from /dev/urandom or a faster source if it's
    available (e.g. arc4random()), which may not necessarily be suitable for
    generating long-term keys */
    #[no_mangle]
    fn UTI_GetRandomBytes(buf: *mut libc::c_void, len: libc::c_uint);
    #[no_mangle]
    fn CNF_GetDriftFile() -> *mut libc::c_char;
    #[no_mangle]
    fn CNF_GetLogTracking() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetMakeStep(limit: *mut libc::c_int, threshold: *mut libc::c_double);
    #[no_mangle]
    fn CNF_GetMaxChange(
        delay: *mut libc::c_int,
        ignore: *mut libc::c_int,
        offset: *mut libc::c_double,
    );
    #[no_mangle]
    fn CNF_GetLogChange() -> libc::c_double;
    #[no_mangle]
    fn CNF_GetMailOnChange(
        enabled: *mut libc::c_int,
        threshold: *mut libc::c_double,
        user: *mut *mut libc::c_char,
    );
    #[no_mangle]
    fn CNF_GetFallbackDrifts(min: *mut libc::c_int, max: *mut libc::c_int);
    #[no_mangle]
    fn CNF_GetLeapSecMode() -> REF_LeapMode;
    #[no_mangle]
    fn CNF_GetLeapSecTimezone() -> *mut libc::c_char;
    /* Value returned in ppm, as read from file */
    #[no_mangle]
    fn CNF_GetMaxUpdateSkew() -> libc::c_double;
    #[no_mangle]
    fn CNF_GetCorrectionTimeRatio() -> libc::c_double;
    #[no_mangle]
    fn CNF_AllowLocalReference(
        stratum: *mut libc::c_int,
        orphan: *mut libc::c_int,
        distance: *mut libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn CNF_GetInitStepThreshold() -> libc::c_double;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LOG_FileOpen(name: *const libc::c_char, banner: *const libc::c_char) -> LOG_FileID;
    #[no_mangle]
    fn LOG_FileWrite(id: LOG_FileID, format: *const libc::c_char, _: ...);
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

     This module provides an interface to the system time, and
     insulates the rest of the program from the different way
     that interface has to be done on various operating systems.
     */
    /* Read the system clock */
    #[no_mangle]
    fn LCL_ReadRawTime(ts: *mut timespec);
    /* Read the current offset between the system clock and true time
    (i.e. 'cooked' - 'raw') (in seconds). */
    #[no_mangle]
    fn LCL_GetOffsetCorrection(
        raw: *mut timespec,
        correction: *mut libc::c_double,
        err: *mut libc::c_double,
    );
    /* Add a handler.  Then handler MUST NOT deregister itself!!! */
    #[no_mangle]
    fn LCL_AddParameterChangeHandler(
        handler: LCL_ParameterChangeHandler,
        anything: *mut libc::c_void,
    );
    /* Read the absolute system frequency, relative to the uncompensated
    system.  Returned in units of parts per million.  Thus the result of
    this is how many seconds fast the uncompensated system would be after
    its own time has reached 1 million seconds from the start of the
    measurement.  */
    #[no_mangle]
    fn LCL_ReadAbsoluteFrequency() -> libc::c_double;
    /* Routine to set the absolute frequency.  Only expected to be used
    when either (i) reading the drift from a file at the start of a
    run, or (ii) responsing to a user parameter 'poke'.  This is
    defined in ppm, as for the absolute frequency reading routine. */
    #[no_mangle]
    fn LCL_SetAbsoluteFrequency(afreq: libc::c_double);
    /* Routine to apply an offset (in seconds) to the local clock.  The
    argument should be positive to move the clock backwards (i.e. the
    local clock is currently fast of true time), or negative to move it
    forwards (i.e. it is currently slow of true time).  Provided is also
    a suggested correction rate (correction time * offset). */
    #[no_mangle]
    fn LCL_AccumulateOffset(offset: libc::c_double, corr_rate: libc::c_double);
    /* Routine to apply an immediate offset by doing a sudden step if
    possible. (Intended for use after an initial estimate of offset has
    been obtained, so that we don't end up using adjtime to achieve a
    slew of an hour or something like that). A positive argument means
    the system clock is fast on true time, i.e. it needs to be stepped
    backwards. (Same convention as for AccumulateOffset routine). */
    #[no_mangle]
    fn LCL_ApplyStepOffset(offset: libc::c_double) -> libc::c_int;
    /* Routine to invoke notify handlers on leap second when the system clock
    doesn't correct itself */
    #[no_mangle]
    fn LCL_NotifyLeap(leap: libc::c_int);
    /* Perform the combination of modifying the frequency and applying
    a slew, in one easy step */
    #[no_mangle]
    fn LCL_AccumulateFrequencyAndOffset(
        dfreq: libc::c_double,
        doffset: libc::c_double,
        corr_rate: libc::c_double,
    );
    /* Routine to read the maximum frequency error of the local clock.  This
    is a frequency stability, not an absolute error. */
    #[no_mangle]
    fn LCL_GetMaxClockError() -> libc::c_double;
    /* Check if the system driver supports leap seconds, i.e. LCL_SetSystemLeap
    does something */
    #[no_mangle]
    fn LCL_CanSystemLeap() -> libc::c_int;
    /* Routine to set the system clock to correct itself for a leap second and also
    set its TAI-UTC offset.  If supported, leap second will be inserted at the
    end of the day if the argument is positive, deleted if negative, and zero
    resets the setting. */
    #[no_mangle]
    fn LCL_SetSystemLeap(leap: libc::c_int, tai_offset: libc::c_int);
    /* Routine to update the synchronisation status in the kernel to allow other
    applications to know if the system clock is synchronised and error bounds */
    #[no_mangle]
    fn LCL_SetSyncStatus(
        synchronised: libc::c_int,
        est_error: libc::c_double,
        max_error: libc::c_double,
    );
    /* Get the time stamp taken after a file descriptor became ready or a timeout expired */
    #[no_mangle]
    fn SCH_GetLastEventTime(cooked: *mut timespec, err: *mut libc::c_double, raw: *mut timespec);
    /* This queues a timeout to elapse at a given (raw) local time */
    #[no_mangle]
    fn SCH_AddTimeout(
        ts: *mut timespec,
        handler: SCH_TimeoutHandler,
        arg: SCH_ArbitraryArgument,
    ) -> SCH_TimeoutID;
    /* This queues a timeout to elapse at a given delta time relative to the current (raw) time */
    #[no_mangle]
    fn SCH_AddTimeoutByDelay(
        delay: libc::c_double,
        _: SCH_TimeoutHandler,
        _: SCH_ArbitraryArgument,
    ) -> SCH_TimeoutID;
    /* The next one probably ought to return a status code */
    #[no_mangle]
    fn SCH_RemoveTimeout(_: SCH_TimeoutID);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type time_t = __time_t;
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
/* The UDP port number used by NTP */
/* The NTP protocol version that we support */
/* Maximum stratum number (infinity) */
/* The minimum valid length of an extension field */
/* The maximum assumed length of all extension fields in received
packets (RFC 5905 doesn't specify a limit on length or number of
extension fields in one packet) */
/* The minimum and maximum supported length of MAC */
/* The maximum length of MAC in NTPv4 packets which allows deterministic
parsing of extension fields (RFC 7822) */
/* Type definition for leap bits */
pub type NTP_Leap = libc::c_uint;
pub const LEAP_Unsynchronised: NTP_Leap = 3;
pub const LEAP_DeleteSecond: NTP_Leap = 2;
pub const LEAP_InsertSecond: NTP_Leap = 1;
pub const LEAP_Normal: NTP_Leap = 0;
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

 Types used for addressing sources etc
 */
/* This type is used to represent an IPv4 address or IPv6 address.
All parts are in HOST order, NOT network order. */
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
pub struct RPT_TrackingReport {
    pub ref_id: uint32_t,
    pub ip_addr: IPAddr,
    pub stratum: libc::c_int,
    pub leap_status: NTP_Leap,
    pub ref_time: timespec,
    pub current_correction: libc::c_double,
    pub last_offset: libc::c_double,
    pub rms_offset: libc::c_double,
    pub freq_ppm: libc::c_double,
    pub resid_freq_ppm: libc::c_double,
    pub skew_ppm: libc::c_double,
    pub root_delay: libc::c_double,
    pub root_dispersion: libc::c_double,
    pub last_update_interval: libc::c_double,
}
pub type REF_LeapMode = libc::c_uint;
pub const REF_LeapModeIgnore: REF_LeapMode = 3;
pub const REF_LeapModeStep: REF_LeapMode = 2;
pub const REF_LeapModeSlew: REF_LeapMode = 1;
pub const REF_LeapModeSystem: REF_LeapMode = 0;
/* File logging functions */
pub type LOG_FileID = libc::c_int;
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

 Exported header file for sched.c
 */
/* Type for timeout IDs, valid IDs are always greater than zero */
pub type SCH_TimeoutID = libc::c_uint;
pub type REF_ModeEndHandler = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
/* ================================================== */
/* Exponential moving averages of absolute clock frequencies
used as a fallback when synchronisation is lost. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fb_drift {
    pub freq: libc::c_double,
    pub secs: libc::c_double,
}
pub type SCH_ArbitraryArgument = *mut libc::c_void;
pub type SCH_TimeoutHandler = Option<unsafe extern "C" fn(_: SCH_ArbitraryArgument) -> ()>;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
pub const REF_ModeIgnore: REF_Mode = 4;
pub type REF_Mode = libc::c_uint;
pub const REF_ModePrintOnce: REF_Mode = 3;
pub const REF_ModeUpdateOnce: REF_Mode = 2;
pub const REF_ModeInitStepSlew: REF_Mode = 1;
pub const REF_ModeNormal: REF_Mode = 0;
pub type LCL_ChangeType = libc::c_uint;
pub const LCL_ChangeUnknownStep: LCL_ChangeType = 2;
pub const LCL_ChangeStep: LCL_ChangeType = 1;
pub const LCL_ChangeAdjust: LCL_ChangeType = 0;
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
static mut are_we_synchronised: libc::c_int = 0;
static mut enable_local_stratum: libc::c_int = 0;
static mut local_stratum: libc::c_int = 0;
static mut local_orphan: libc::c_int = 0;
static mut local_distance: libc::c_double = 0.;
static mut local_ref_time: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
static mut our_leap_status: NTP_Leap = LEAP_Normal;
static mut our_leap_sec: libc::c_int = 0;
static mut our_tai_offset: libc::c_int = 0;
static mut our_stratum: libc::c_int = 0;
static mut our_ref_id: uint32_t = 0;
static mut our_ref_ip: IPAddr = IPAddr {
    addr: C2RustUnnamed { in4: 0 },
    family: 0,
    _pad: 0,
};
static mut our_ref_time: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
static mut our_skew: libc::c_double = 0.;
static mut our_residual_freq: libc::c_double = 0.;
static mut our_root_delay: libc::c_double = 0.;
static mut our_root_dispersion: libc::c_double = 0.;
static mut max_update_skew: libc::c_double = 0.;
static mut last_offset: libc::c_double = 0.;
static mut avg2_offset: libc::c_double = 0.;
static mut avg2_moving: libc::c_int = 0;
static mut correction_time_ratio: libc::c_double = 0.;
/* Flag indicating that we are initialised */
static mut initialised: libc::c_int = 0 as libc::c_int;
/* Current operating mode */
static mut mode: REF_Mode = REF_ModeNormal;
/* Threshold and update limit for stepping clock */
static mut make_step_limit: libc::c_int = 0;
static mut make_step_threshold: libc::c_double = 0.;
/* Number of updates before offset checking, number of ignored updates
before exiting and the maximum allowed offset */
static mut max_offset_delay: libc::c_int = 0;
static mut max_offset_ignore: libc::c_int = 0;
static mut max_offset: libc::c_double = 0.;
/* Threshold for logging clock changes to syslog */
static mut log_change_threshold: libc::c_double = 0.;
/* Flag, threshold and user for sending mail notification on large clock changes */
static mut do_mail_change: libc::c_int = 0;
static mut mail_change_threshold: libc::c_double = 0.;
static mut mail_change_user: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* Handler for mode ending */
static mut mode_end_handler: REF_ModeEndHandler = None;
/* Filename of the drift file. */
static mut drift_file: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut drift_file_age: libc::c_double = 0.;
/* Leap second handling mode */
static mut leap_mode: REF_LeapMode = REF_LeapModeSystem;
/* Flag indicating the clock was recently corrected for leap second and it may
not have correct time yet (missing 23:59:60 in the UTC time scale) */
static mut leap_in_progress: libc::c_int = 0;
/* Timer for the leap second handler */
static mut leap_timeout_id: SCH_TimeoutID = 0;
/* Name of a system timezone containing leap seconds occuring at midnight */
static mut leap_tzname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* ================================================== */
static mut logfileid: LOG_FileID = 0;
static mut fb_drift_min: libc::c_int = 0;
static mut fb_drift_max: libc::c_int = 0;
static mut fb_drifts: *mut fb_drift = 0 as *const fb_drift as *mut fb_drift;
static mut next_fb_drift: libc::c_int = 0;
static mut fb_drift_timeout_id: SCH_TimeoutID = 0;
/* Timestamp of last reference update */
static mut last_ref_update: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
static mut last_ref_update_interval: libc::c_double = 0.;
/* ================================================== */
unsafe extern "C" fn handle_slew(
    mut raw: *mut timespec,
    mut cooked: *mut timespec,
    mut dfreq: libc::c_double,
    mut doffset: libc::c_double,
    mut change_type: LCL_ChangeType,
    mut anything: *mut libc::c_void,
) {
    let mut delta: libc::c_double = 0.;
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if UTI_IsZeroTimespec(&mut our_ref_time) == 0 {
        UTI_AdjustTimespec(
            &mut our_ref_time,
            cooked,
            &mut our_ref_time,
            &mut delta,
            dfreq,
            doffset,
        );
    }
    if change_type as libc::c_uint == LCL_ChangeUnknownStep as libc::c_int as libc::c_uint {
        UTI_ZeroTimespec(&mut last_ref_update);
    } else if last_ref_update.tv_sec != 0 {
        UTI_AdjustTimespec(
            &mut last_ref_update,
            cooked,
            &mut last_ref_update,
            &mut delta,
            dfreq,
            doffset,
        );
    }
    /* When the clock was stepped, check if that doesn't change our leap status
    and also reset the leap timeout to undo the shift in the scheduler */
    if change_type as libc::c_uint != LCL_ChangeAdjust as libc::c_int as libc::c_uint
        && our_leap_sec != 0
        && leap_in_progress == 0
    {
        LCL_ReadRawTime(&mut now);
        update_leap_status(our_leap_status, now.tv_sec, 1 as libc::c_int);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_Initialise() {
    let mut in_0: *mut FILE = 0 as *mut FILE; /* i.e. rather bad */
    let mut file_freq_ppm: libc::c_double = 0.;
    let mut file_skew_ppm: libc::c_double = 0.;
    let mut our_frequency_ppm: libc::c_double = 0.;
    let mut tai_offset: libc::c_int = 0;
    mode = REF_ModeNormal;
    are_we_synchronised = 0 as libc::c_int;
    our_leap_status = LEAP_Unsynchronised;
    our_leap_sec = 0 as libc::c_int;
    our_tai_offset = 0 as libc::c_int;
    initialised = 1 as libc::c_int;
    our_root_dispersion = 1.0f64;
    our_root_delay = 1.0f64;
    our_frequency_ppm = 0.0f64;
    our_skew = 1.0f64;
    our_residual_freq = 0.0f64;
    drift_file_age = 0.0f64;
    /* Now see if we can get the drift file opened */
    drift_file = CNF_GetDriftFile();
    if !drift_file.is_null() {
        in_0 = UTI_OpenFile(
            0 as *const libc::c_char,
            drift_file,
            0 as *const libc::c_char,
            'r' as i32 as libc::c_char,
            0 as libc::c_int as mode_t,
        );
        if !in_0.is_null() {
            if fscanf(
                in_0,
                b"%lf%lf\x00" as *const u8 as *const libc::c_char,
                &mut file_freq_ppm as *mut libc::c_double,
                &mut file_skew_ppm as *mut libc::c_double,
            ) == 2 as libc::c_int
            {
                /* We have read valid data */
                our_frequency_ppm = file_freq_ppm;
                our_skew = 1.0e-6f64 * file_skew_ppm;
                if our_skew < 1.0e-12f64 {
                    our_skew = 1.0e-12f64
                }
                LOG_Message(
                    LOGS_INFO,
                    b"Frequency %.3f +/- %.3f ppm read from %s\x00" as *const u8
                        as *const libc::c_char,
                    file_freq_ppm,
                    file_skew_ppm,
                    drift_file,
                );
                LCL_SetAbsoluteFrequency(our_frequency_ppm);
            } else {
                LOG_Message(
                    LOGS_WARN,
                    b"Could not read valid frequency and skew from driftfile %s\x00" as *const u8
                        as *const libc::c_char,
                    drift_file,
                );
            }
            fclose(in_0);
        }
    }
    if our_frequency_ppm == 0.0f64 {
        our_frequency_ppm = LCL_ReadAbsoluteFrequency();
        if our_frequency_ppm != 0.0f64 {
            LOG_Message(
                LOGS_INFO,
                b"Initial frequency %.3f ppm\x00" as *const u8 as *const libc::c_char,
                our_frequency_ppm,
            );
        }
    }
    logfileid = if CNF_GetLogTracking() != 0 {
        LOG_FileOpen(b"tracking\x00" as *const u8 as *const libc::c_char,
                         b"   Date (UTC) Time     IP Address   St   Freq ppm   Skew ppm     Offset L Co  Offset sd Rem. corr. Root delay Root disp. Max. error\x00"
                             as *const u8 as *const libc::c_char)
    } else {
        -(1 as libc::c_int)
    };
    max_update_skew = fabs(CNF_GetMaxUpdateSkew()) * 1.0e-6f64;
    correction_time_ratio = CNF_GetCorrectionTimeRatio();
    enable_local_stratum =
        CNF_AllowLocalReference(&mut local_stratum, &mut local_orphan, &mut local_distance);
    UTI_ZeroTimespec(&mut local_ref_time);
    leap_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    leap_in_progress = 0 as libc::c_int;
    leap_mode = CNF_GetLeapSecMode();
    /* Switch to step mode if the system driver doesn't support leap */
    if leap_mode as libc::c_uint == REF_LeapModeSystem as libc::c_int as libc::c_uint
        && LCL_CanSystemLeap() == 0
    {
        leap_mode = REF_LeapModeStep
    }
    leap_tzname = CNF_GetLeapSecTimezone();
    if !leap_tzname.is_null() {
        /* Check that the timezone has good data for Jun 30 2012 and Dec 31 2012 */
        if get_tz_leap(1341014400 as libc::c_int as time_t, &mut tai_offset) as libc::c_uint
            == LEAP_InsertSecond as libc::c_int as libc::c_uint
            && tai_offset == 34 as libc::c_int
            && get_tz_leap(1356912000 as libc::c_int as time_t, &mut tai_offset) as libc::c_uint
                == LEAP_Normal as libc::c_int as libc::c_uint
            && tai_offset == 35 as libc::c_int
        {
            LOG_Message(
                LOGS_INFO,
                b"Using %s timezone to obtain leap second data\x00" as *const u8
                    as *const libc::c_char,
                leap_tzname,
            );
        } else {
            LOG_Message(
                LOGS_WARN,
                b"Timezone %s failed leap second check, ignoring\x00" as *const u8
                    as *const libc::c_char,
                leap_tzname,
            );
            leap_tzname = 0 as *mut libc::c_char
        }
    }
    CNF_GetMakeStep(&mut make_step_limit, &mut make_step_threshold);
    CNF_GetMaxChange(
        &mut max_offset_delay,
        &mut max_offset_ignore,
        &mut max_offset,
    );
    CNF_GetMailOnChange(
        &mut do_mail_change,
        &mut mail_change_threshold,
        &mut mail_change_user,
    );
    log_change_threshold = CNF_GetLogChange();
    CNF_GetFallbackDrifts(&mut fb_drift_min, &mut fb_drift_max);
    if fb_drift_max >= fb_drift_min && fb_drift_min > 0 as libc::c_int {
        fb_drifts = Malloc2(
            (fb_drift_max - fb_drift_min + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<fb_drift>() as libc::c_ulong,
        ) as *mut fb_drift;
        memset(
            fb_drifts as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<fb_drift>() as libc::c_ulong)
                .wrapping_mul((fb_drift_max - fb_drift_min + 1 as libc::c_int) as libc::c_ulong),
        );
        next_fb_drift = 0 as libc::c_int;
        fb_drift_timeout_id = 0 as libc::c_int as SCH_TimeoutID
    }
    UTI_ZeroTimespec(&mut our_ref_time);
    UTI_ZeroTimespec(&mut last_ref_update);
    last_ref_update_interval = 0.0f64;
    LCL_AddParameterChangeHandler(
        Some(
            handle_slew
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
    /* Make first entry in tracking log */
    REF_SetUnsynchronised();
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_Finalise() {
    update_leap_status(
        LEAP_Unsynchronised,
        0 as libc::c_int as time_t,
        0 as libc::c_int,
    );
    if !drift_file.is_null() {
        update_drift_file(LCL_ReadAbsoluteFrequency(), our_skew);
    }
    free(fb_drifts as *mut libc::c_void);
    initialised = 0 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_SetMode(mut new_mode: REF_Mode) {
    mode = new_mode;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_GetMode() -> REF_Mode {
    return mode;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_SetModeEndHandler(mut handler: REF_ModeEndHandler) {
    mode_end_handler = handler;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_GetLeapMode() -> REF_LeapMode {
    return leap_mode;
}
/* ================================================== */
/* Update the drift coefficients to the file. */
unsafe extern "C" fn update_drift_file(mut freq_ppm: libc::c_double, mut skew: libc::c_double) {
    let mut out: *mut FILE = 0 as *mut FILE;
    /* Create a temporary file with a '.tmp' extension. */
    out = UTI_OpenFile(
        0 as *const libc::c_char,
        drift_file,
        b".tmp\x00" as *const u8 as *const libc::c_char,
        'w' as i32 as libc::c_char,
        0o644 as libc::c_int as mode_t,
    );
    if out.is_null() {
        return;
    }
    /* Write the frequency and skew parameters in ppm */
    fprintf(
        out,
        b"%20.6f %20.6f\n\x00" as *const u8 as *const libc::c_char,
        freq_ppm,
        1.0e6f64 * skew,
    );
    fclose(out);
    /* Rename the temporary file to the correct location */
    (UTI_RenameTempFile(
        0 as *const libc::c_char,
        drift_file,
        b".tmp\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    )) == 0;
}
/* ================================================== */
unsafe extern "C" fn update_fb_drifts(
    mut freq_ppm: libc::c_double,
    mut update_interval: libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut secs: libc::c_int = 0;
    if are_we_synchronised != 0 {
    } else {
        __assert_fail(
            b"are_we_synchronised\x00" as *const u8 as *const libc::c_char,
            b"reference.c\x00" as *const u8 as *const libc::c_char,
            362 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void update_fb_drifts(double, double)\x00",
            ))
            .as_ptr(),
        );
    }
    if next_fb_drift > 0 as libc::c_int {
        next_fb_drift = 0 as libc::c_int
    }
    SCH_RemoveTimeout(fb_drift_timeout_id);
    fb_drift_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    if update_interval < 1.0f64 || update_interval > last_ref_update_interval * 4.0f64 {
        return;
    }
    i = 0 as libc::c_int;
    while i < fb_drift_max - fb_drift_min + 1 as libc::c_int {
        secs = (1 as libc::c_int) << i + fb_drift_min;
        if (*fb_drifts.offset(i as isize)).secs < secs as libc::c_double {
            /* Calculate average over 2 * secs interval before switching to
            exponential updating */
            (*fb_drifts.offset(i as isize)).freq = ((*fb_drifts.offset(i as isize)).freq
                * (*fb_drifts.offset(i as isize)).secs
                + update_interval * 0.5f64 * freq_ppm)
                / (update_interval * 0.5f64 + (*fb_drifts.offset(i as isize)).secs);
            (*fb_drifts.offset(i as isize)).secs += update_interval * 0.5f64
        } else {
            /* Update exponential moving average. The smoothing factor for update
            interval equal to secs is about 0.63, for half interval about 0.39,
            for double interval about 0.86. */
            (*fb_drifts.offset(i as isize)).freq += (1 as libc::c_int as libc::c_double
                - 1.0f64 / exp(update_interval / secs as libc::c_double))
                * (freq_ppm - (*fb_drifts.offset(i as isize)).freq)
        }
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Fallback drift %d updated: %f ppm %f seconds\x00" as *const u8
                    as *const libc::c_char,
                i + fb_drift_min,
                (*fb_drifts.offset(i as isize)).freq,
                (*fb_drifts.offset(i as isize)).secs,
            );
        }
        i += 1
    }
}
/* ================================================== */
unsafe extern "C" fn fb_drift_timeout(mut arg: *mut libc::c_void) {
    if next_fb_drift >= fb_drift_min && next_fb_drift <= fb_drift_max {
    } else {
        __assert_fail(
            b"next_fb_drift >= fb_drift_min && next_fb_drift <= fb_drift_max\x00" as *const u8
                as *const libc::c_char,
            b"reference.c\x00" as *const u8 as *const libc::c_char,
            405 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"void fb_drift_timeout(void *)\x00",
            ))
            .as_ptr(),
        );
    }
    fb_drift_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"Fallback drift %d active: %f ppm\x00" as *const u8 as *const libc::c_char,
            next_fb_drift,
            (*fb_drifts.offset((next_fb_drift - fb_drift_min) as isize)).freq,
        );
    }
    LCL_SetAbsoluteFrequency((*fb_drifts.offset((next_fb_drift - fb_drift_min) as isize)).freq);
    REF_SetUnsynchronised();
}
/* ================================================== */
unsafe extern "C" fn schedule_fb_drift(mut now: *mut timespec) {
    let mut i: libc::c_int = 0; /* already scheduled */
    let mut c: libc::c_int = 0;
    let mut secs: libc::c_int = 0;
    let mut unsynchronised: libc::c_double = 0.;
    let mut when: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if fb_drift_timeout_id != 0 {
        return;
    }
    unsynchronised = UTI_DiffTimespecsToDouble(now, &mut last_ref_update);
    secs = 0 as libc::c_int;
    c = secs;
    i = fb_drift_min;
    while i <= fb_drift_max {
        secs = (1 as libc::c_int) << i;
        if !((*fb_drifts.offset((i - fb_drift_min) as isize)).secs < secs as libc::c_double) {
            if unsynchronised < secs as libc::c_double && i > next_fb_drift {
                break;
            }
            c = i
        }
        i += 1
    }
    if c > next_fb_drift {
        LCL_SetAbsoluteFrequency((*fb_drifts.offset((c - fb_drift_min) as isize)).freq);
        next_fb_drift = c;
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Fallback drift %d set\x00" as *const u8 as *const libc::c_char,
                c,
            );
        }
    }
    if i <= fb_drift_max {
        next_fb_drift = i;
        UTI_AddDoubleToTimespec(now, secs as libc::c_double - unsynchronised, &mut when);
        fb_drift_timeout_id = SCH_AddTimeout(
            &mut when,
            Some(fb_drift_timeout as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
            0 as *mut libc::c_void,
        );
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Fallback drift %d scheduled\x00" as *const u8 as *const libc::c_char,
                i,
            );
        }
    };
}
/* ================================================== */
unsafe extern "C" fn end_ref_mode(mut result: libc::c_int) {
    mode = REF_ModeIgnore;
    /* Dispatch the handler */
    if mode_end_handler.is_some() {
        mode_end_handler.expect("non-null function pointer")(result);
    };
}
/* ================================================== */
unsafe extern "C" fn maybe_log_offset(mut offset: libc::c_double, mut now: time_t) {
    let mut abs_offset: libc::c_double = 0.;
    let mut p: *mut FILE = 0 as *mut FILE;
    let mut buffer: [libc::c_char; 255] = [0; 255];
    let mut host: [libc::c_char; 255] = [0; 255];
    let mut tm: *mut tm = 0 as *mut tm;
    abs_offset = fabs(offset);
    if abs_offset > log_change_threshold {
        LOG_Message(
            LOGS_WARN,
            b"System clock wrong by %.6f seconds, adjustment started\x00" as *const u8
                as *const libc::c_char,
            -offset,
        );
    }
    if do_mail_change != 0 && abs_offset > mail_change_threshold {
        snprintf(
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
            b"%s -t\x00" as *const u8 as *const libc::c_char,
            b"/usr/lib/sendmail\x00" as *const u8 as *const libc::c_char,
        );
        p = popen(
            buffer.as_mut_ptr(),
            b"w\x00" as *const u8 as *const libc::c_char,
        );
        if !p.is_null() {
            if gethostname(
                host.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
            ) < 0 as libc::c_int
            {
                strcpy(
                    host.as_mut_ptr(),
                    b"<UNKNOWN>\x00" as *const u8 as *const libc::c_char,
                );
            }
            host[(::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] =
                '\u{0}' as i32 as libc::c_char;
            fprintf(
                p,
                b"To: %s\n\x00" as *const u8 as *const libc::c_char,
                mail_change_user,
            );
            fprintf(
                p,
                b"Subject: chronyd reports change to system clock on node [%s]\n\x00" as *const u8
                    as *const libc::c_char,
                host.as_mut_ptr(),
            );
            fputs(b"\n\x00" as *const u8 as *const libc::c_char, p);
            tm = localtime(&mut now);
            if !tm.is_null() {
                strftime(
                    buffer.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
                    b"On %A, %d %B %Y\n  with the system clock reading %H:%M:%S (%Z)\x00"
                        as *const u8 as *const libc::c_char,
                    tm,
                );
                fputs(buffer.as_mut_ptr(), p);
            }
            /* If offset < 0 the local clock is slow, so we are applying a
            positive change to it to bring it into line, hence the
            negation of 'offset' in the next statement (and earlier) */
            fprintf(p,
                    b"\n\nchronyd started to apply an adjustment of %.3f seconds to it,\n  which exceeded the reporting threshold of %.3f seconds\n\n\x00"
                        as *const u8 as *const libc::c_char, -offset,
                    mail_change_threshold);
            pclose(p);
        } else {
            LOG_Message(
                LOGS_ERR,
                b"Could not send mail notification to user %s\n\x00" as *const u8
                    as *const libc::c_char,
                mail_change_user,
            );
        }
    };
}
/* ================================================== */
unsafe extern "C" fn is_step_limit_reached(
    mut offset: libc::c_double,
    mut offset_correction: libc::c_double,
) -> libc::c_int {
    if make_step_limit == 0 as libc::c_int {
        return 0 as libc::c_int;
    } else {
        if make_step_limit > 0 as libc::c_int {
            make_step_limit -= 1
        }
    }
    return (fabs(offset - offset_correction) > make_step_threshold) as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn is_offset_ok(mut offset: libc::c_double) -> libc::c_int {
    if max_offset_delay < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if max_offset_delay > 0 as libc::c_int {
        max_offset_delay -= 1;
        return 1 as libc::c_int;
    }
    offset = fabs(offset);
    if offset > max_offset {
        LOG_Message(
            LOGS_WARN,
            b"Adjustment of %.3f seconds exceeds the allowed maximum of %.3f seconds (%s) \x00"
                as *const u8 as *const libc::c_char,
            -offset,
            max_offset,
            if max_offset_ignore == 0 {
                b"exiting\x00" as *const u8 as *const libc::c_char
            } else {
                b"ignored\x00" as *const u8 as *const libc::c_char
            },
        );
        if max_offset_ignore == 0 {
            end_ref_mode(0 as libc::c_int);
        } else if max_offset_ignore > 0 as libc::c_int {
            max_offset_ignore -= 1
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn is_leap_second_day(mut when: time_t) -> libc::c_int {
    let mut stm: *mut tm = 0 as *mut tm;
    stm = gmtime(&mut when);
    if stm.is_null() {
        return 0 as libc::c_int;
    }
    /* Allow leap second only on the last day of June and December */
    return ((*stm).tm_mon == 5 as libc::c_int && (*stm).tm_mday == 30 as libc::c_int
        || (*stm).tm_mon == 11 as libc::c_int && (*stm).tm_mday == 31 as libc::c_int)
        as libc::c_int;
}
/* ================================================== */
/* ================================================== */
unsafe extern "C" fn get_tz_leap(mut when: time_t, mut tai_offset: *mut libc::c_int) -> NTP_Leap {
    static mut last_tz_leap_check: time_t = 0;
    static mut tz_leap: NTP_Leap = LEAP_Normal;
    static mut tz_tai_offset: libc::c_int = 0;
    let mut stm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut tm: *mut tm = 0 as *mut tm;
    let mut t: time_t = 0;
    let mut tz_env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tz_orig: [libc::c_char; 128] = [0; 128];
    *tai_offset = tz_tai_offset;
    /* Do this check at most twice a day */
    when = when / (12 as libc::c_int * 3600 as libc::c_int) as libc::c_long
        * (12 as libc::c_int * 3600 as libc::c_int) as libc::c_long;
    if last_tz_leap_check == when {
        return tz_leap;
    }
    last_tz_leap_check = when;
    tz_leap = LEAP_Normal;
    tz_tai_offset = 0 as libc::c_int;
    tm = gmtime(&mut when);
    if tm.is_null() {
        return tz_leap;
    }
    stm = *tm;
    /* Temporarily switch to the timezone containing leap seconds */
    tz_env = getenv(b"TZ\x00" as *const u8 as *const libc::c_char);
    if !tz_env.is_null() {
        if strlen(tz_env) >= ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong {
            return tz_leap;
        }
        strcpy(tz_orig.as_mut_ptr(), tz_env);
    }
    setenv(
        b"TZ\x00" as *const u8 as *const libc::c_char,
        leap_tzname,
        1 as libc::c_int,
    );
    tzset();
    /* Get the TAI-UTC offset, which started at the epoch at 10 seconds */
    t = mktime(&mut stm);
    if t != -(1 as libc::c_int) as libc::c_long {
        tz_tai_offset = (t - when + 10 as libc::c_int as libc::c_long) as libc::c_int
    }
    /* Set the time to 23:59:60 and see how it overflows in mktime() */
    stm.tm_sec = 60 as libc::c_int;
    stm.tm_min = 59 as libc::c_int;
    stm.tm_hour = 23 as libc::c_int;
    t = mktime(&mut stm);
    if !tz_env.is_null() {
        setenv(
            b"TZ\x00" as *const u8 as *const libc::c_char,
            tz_orig.as_mut_ptr(),
            1 as libc::c_int,
        );
    } else {
        unsetenv(b"TZ\x00" as *const u8 as *const libc::c_char);
    }
    tzset();
    if t == -(1 as libc::c_int) as libc::c_long {
        return tz_leap;
    }
    if stm.tm_sec == 60 as libc::c_int {
        tz_leap = LEAP_InsertSecond
    } else if stm.tm_sec == 1 as libc::c_int {
        tz_leap = LEAP_DeleteSecond
    }
    *tai_offset = tz_tai_offset;
    return tz_leap;
}
/* ================================================== */
unsafe extern "C" fn leap_end_timeout(mut arg: *mut libc::c_void) {
    leap_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    leap_in_progress = 0 as libc::c_int;
    if our_tai_offset != 0 {
        our_tai_offset += our_leap_sec
    }
    our_leap_sec = 0 as libc::c_int;
    if leap_mode as libc::c_uint == REF_LeapModeSystem as libc::c_int as libc::c_uint {
        LCL_SetSystemLeap(our_leap_sec, our_tai_offset);
    }
    if our_leap_status as libc::c_uint == LEAP_InsertSecond as libc::c_int as libc::c_uint
        || our_leap_status as libc::c_uint == LEAP_DeleteSecond as libc::c_int as libc::c_uint
    {
        our_leap_status = LEAP_Normal
    };
}
/* ================================================== */
unsafe extern "C" fn leap_start_timeout(mut arg: *mut libc::c_void) {
    leap_in_progress = 1 as libc::c_int;
    match leap_mode as libc::c_uint {
        0 => {
            if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
            {
                LOG_Message(
                    LOGS_DEBUG,
                    b"Waiting for system clock leap second correction\x00" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        1 => {
            LCL_NotifyLeap(our_leap_sec);
            LCL_AccumulateOffset(our_leap_sec as libc::c_double, 0.0f64);
            LOG_Message(
                LOGS_WARN,
                b"Adjusting system clock for leap second\x00" as *const u8 as *const libc::c_char,
            );
        }
        2 => {
            LCL_NotifyLeap(our_leap_sec);
            LCL_ApplyStepOffset(our_leap_sec as libc::c_double);
            LOG_Message(
                LOGS_WARN,
                b"System clock was stepped for leap second\x00" as *const u8 as *const libc::c_char,
            );
        }
        3 => {
            LOG_Message(
                LOGS_WARN,
                b"Ignoring leap second\x00" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    }
    /* Wait until the leap second is over with some extra room to be safe */
    leap_timeout_id = SCH_AddTimeoutByDelay(
        2.0f64,
        Some(leap_end_timeout as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
    );
}
/* ================================================== */
unsafe extern "C" fn set_leap_timeout(mut now: time_t) {
    let mut when: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    /* Stop old timer if there is one */
    SCH_RemoveTimeout(leap_timeout_id);
    leap_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    leap_in_progress = 0 as libc::c_int;
    if our_leap_sec == 0 {
        return;
    }
    /* Insert leap second at 0:00:00 UTC, delete at 23:59:59 UTC.  If the clock
    will be corrected by the system, timeout slightly sooner to be sure it
    will happen before the system correction. */
    when.tv_sec = (now / (24 as libc::c_int * 3600 as libc::c_int) as libc::c_long
        + 1 as libc::c_int as libc::c_long)
        * (24 as libc::c_int * 3600 as libc::c_int) as libc::c_long;
    when.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    if our_leap_sec < 0 as libc::c_int {
        when.tv_sec -= 1
    }
    if leap_mode as libc::c_uint == REF_LeapModeSystem as libc::c_int as libc::c_uint {
        when.tv_sec -= 1;
        when.tv_nsec = 500000000 as libc::c_int as __syscall_slong_t
    }
    leap_timeout_id = SCH_AddTimeout(
        &mut when,
        Some(leap_start_timeout as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
    );
}
/* ================================================== */
unsafe extern "C" fn update_leap_status(
    mut leap: NTP_Leap,
    mut now: time_t,
    mut reset: libc::c_int,
) {
    let mut tz_leap: NTP_Leap = LEAP_Normal;
    let mut leap_sec: libc::c_int = 0;
    let mut tai_offset: libc::c_int = 0;
    leap_sec = 0 as libc::c_int;
    tai_offset = 0 as libc::c_int;
    if !leap_tzname.is_null() && now != 0 {
        tz_leap = get_tz_leap(now, &mut tai_offset);
        if leap as libc::c_uint == LEAP_Normal as libc::c_int as libc::c_uint {
            leap = tz_leap
        }
    }
    if leap as libc::c_uint == LEAP_InsertSecond as libc::c_int as libc::c_uint
        || leap as libc::c_uint == LEAP_DeleteSecond as libc::c_int as libc::c_uint
    {
        /* Check that leap second is allowed today */
        if is_leap_second_day(now) != 0 {
            if leap as libc::c_uint == LEAP_InsertSecond as libc::c_int as libc::c_uint {
                leap_sec = 1 as libc::c_int
            } else {
                leap_sec = -(1 as libc::c_int)
            }
        } else {
            leap = LEAP_Normal
        }
    }
    if (leap_sec != our_leap_sec || tai_offset != our_tai_offset) && REF_IsLeapSecondClose() == 0 {
        our_leap_sec = leap_sec;
        our_tai_offset = tai_offset;
        let mut current_block_22: u64;
        match leap_mode as libc::c_uint {
            0 => {
                LCL_SetSystemLeap(our_leap_sec, our_tai_offset);
                current_block_22 = 3989310813288763114;
            }
            1 | 2 | 3 => {
                current_block_22 = 3989310813288763114;
            }
            _ => {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"reference.c\x00" as *const u8 as *const libc::c_char,
                    779 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                        b"void update_leap_status(NTP_Leap, time_t, int)\x00",
                    ))
                    .as_ptr(),
                );
                current_block_22 = 7056779235015430508;
            }
        }
        match current_block_22 {
            3989310813288763114 =>
            /* Fall through */
            {
                set_leap_timeout(now);
            }
            _ => {}
        }
    } else if reset != 0 {
        set_leap_timeout(now);
    }
    our_leap_status = leap;
}
/* ================================================== */
unsafe extern "C" fn get_root_dispersion(mut ts: *mut timespec) -> libc::c_double {
    if UTI_IsZeroTimespec(&mut our_ref_time) != 0 {
        return 1.0f64;
    }
    return our_root_dispersion
        + fabs(UTI_DiffTimespecsToDouble(ts, &mut our_ref_time))
            * (our_skew + fabs(our_residual_freq) + LCL_GetMaxClockError());
}
/* ================================================== */
unsafe extern "C" fn write_log(
    mut now: *mut timespec,
    mut combined_sources: libc::c_int,
    mut freq: libc::c_double,
    mut offset: libc::c_double,
    mut offset_sd: libc::c_double,
    mut uncorrected_offset: libc::c_double,
    mut orig_root_distance: libc::c_double,
) {
    let leap_codes: [libc::c_char; 4] = [
        'N' as i32 as libc::c_char,
        '+' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
    ];
    let mut root_dispersion: libc::c_double = 0.;
    let mut max_error: libc::c_double = 0.;
    static mut last_sys_offset: libc::c_double = 0.0f64;
    if logfileid == -(1 as libc::c_int) {
        return;
    }
    max_error = orig_root_distance + fabs(last_sys_offset);
    root_dispersion = get_root_dispersion(now);
    last_sys_offset = offset - uncorrected_offset;
    LOG_FileWrite(
        logfileid,
        b"%s %-15s %2d %10.3f %10.3f %10.3e %1c %2d %10.3e %10.3e %10.3e %10.3e %10.3e\x00"
            as *const u8 as *const libc::c_char,
        UTI_TimeToLogForm((*now).tv_sec),
        if our_ref_ip.family as libc::c_int != 0 as libc::c_int {
            UTI_IPToString(&mut our_ref_ip)
        } else {
            UTI_RefidToString(our_ref_id)
        },
        our_stratum,
        freq,
        1.0e6f64 * our_skew,
        offset,
        leap_codes[our_leap_status as usize] as libc::c_int,
        combined_sources,
        offset_sd,
        uncorrected_offset,
        our_root_delay,
        root_dispersion,
        max_error,
    );
}
/* ================================================== */
unsafe extern "C" fn special_mode_sync(mut valid: libc::c_int, mut offset: libc::c_double) {
    let mut step: libc::c_int = 0;
    match mode as libc::c_uint {
        1 => {
            if valid == 0 {
                LOG_Message(
                    LOGS_WARN,
                    b"No suitable source for initstepslew\x00" as *const u8 as *const libc::c_char,
                );
                end_ref_mode(0 as libc::c_int);
            } else {
                step = (fabs(offset) >= CNF_GetInitStepThreshold()) as libc::c_int;
                LOG_Message(
                    LOGS_INFO,
                    b"System\'s initial offset : %.6f seconds %s of true (%s)\x00" as *const u8
                        as *const libc::c_char,
                    fabs(offset),
                    if offset >= 0 as libc::c_int as libc::c_double {
                        b"fast\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"slow\x00" as *const u8 as *const libc::c_char
                    },
                    if step != 0 {
                        b"step\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"slew\x00" as *const u8 as *const libc::c_char
                    },
                );
                if step != 0 {
                    LCL_ApplyStepOffset(offset);
                } else {
                    LCL_AccumulateOffset(offset, 0.0f64);
                }
                end_ref_mode(1 as libc::c_int);
            }
        }
        2 | 3 => {
            if valid == 0 {
                LOG_Message(
                    LOGS_WARN,
                    b"No suitable source for synchronisation\x00" as *const u8
                        as *const libc::c_char,
                );
                end_ref_mode(0 as libc::c_int);
            } else {
                step = (mode as libc::c_uint == REF_ModeUpdateOnce as libc::c_int as libc::c_uint)
                    as libc::c_int;
                LOG_Message(
                    LOGS_INFO,
                    b"System clock wrong by %.6f seconds (%s)\x00" as *const u8
                        as *const libc::c_char,
                    -offset,
                    if step != 0 {
                        b"step\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"ignored\x00" as *const u8 as *const libc::c_char
                    },
                );
                if step != 0 {
                    LCL_ApplyStepOffset(offset);
                }
                end_ref_mode(1 as libc::c_int);
            }
        }
        4 => {}
        _ => {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"reference.c\x00" as *const u8 as *const libc::c_char,
                881 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                    b"void special_mode_sync(int, double)\x00",
                ))
                .as_ptr(),
            );
        }
    };
}
/* ================================================== */
unsafe extern "C" fn get_clock_estimates(
    mut manual: libc::c_int,
    mut measured_freq: libc::c_double,
    mut measured_skew: libc::c_double,
    mut estimated_freq: *mut libc::c_double,
    mut estimated_skew: *mut libc::c_double,
    mut residual_freq: *mut libc::c_double,
) {
    let mut gain: libc::c_double = 0.;
    let mut expected_freq: libc::c_double = 0.;
    let mut expected_skew: libc::c_double = 0.;
    let mut extra_skew: libc::c_double = 0.;
    /* We assume that the local clock is running according to our previously
    determined value */
    expected_freq = 0.0f64;
    expected_skew = our_skew;
    /* Set new frequency based on weighted average of the expected and measured
    skew.  Disable updates that are based on totally unreliable frequency
    information unless it is a manual reference. */
    if manual != 0 {
        gain = 1.0f64
    } else if fabs(measured_skew) > max_update_skew {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Skew %f too large to track\x00" as *const u8 as *const libc::c_char,
                measured_skew,
            );
        }
        gain = 0.0f64
    } else {
        gain = 3.0f64 * (expected_skew * expected_skew)
            / (3.0f64 * (expected_skew * expected_skew) + measured_skew * measured_skew)
    }
    gain = if 0.0f64 > (if gain < 1.0f64 { gain } else { 1.0f64 }) {
        0.0f64
    } else if gain < 1.0f64 {
        gain
    } else {
        1.0f64
    };
    *estimated_freq = expected_freq + gain * (measured_freq - expected_freq);
    *residual_freq = measured_freq - *estimated_freq;
    extra_skew = sqrt(
        (expected_freq - *estimated_freq) * (expected_freq - *estimated_freq) * (1.0f64 - gain)
            + (measured_freq - *estimated_freq) * (measured_freq - *estimated_freq) * gain,
    );
    *estimated_skew = expected_skew + gain * (measured_skew - expected_skew) + extra_skew;
}
/* ================================================== */
unsafe extern "C" fn fuzz_ref_time(mut ts: *mut timespec) {
    let mut rnd: uint32_t = 0;
    /* Add a random value from interval [-1.0, 0.0] */
    UTI_GetRandomBytes(
        &mut rnd as *mut uint32_t as *mut libc::c_void,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong as libc::c_uint,
    );
    UTI_AddDoubleToTimespec(
        ts,
        -(rnd as libc::c_double) / -(1 as libc::c_int) as uint32_t as libc::c_double,
        ts,
    );
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_SetReference(
    mut stratum: libc::c_int,
    mut leap: NTP_Leap,
    mut combined_sources: libc::c_int,
    mut ref_id: uint32_t,
    mut ref_ip: *mut IPAddr,
    mut ref_time: *mut timespec,
    mut offset: libc::c_double,
    mut offset_sd: libc::c_double,
    mut frequency: libc::c_double,
    mut frequency_sd: libc::c_double,
    mut skew: libc::c_double,
    mut root_delay: libc::c_double,
    mut root_dispersion: libc::c_double,
) {
    let mut uncorrected_offset: libc::c_double = 0.;
    let mut accumulate_offset: libc::c_double = 0.;
    let mut step_offset: libc::c_double = 0.;
    let mut residual_frequency: libc::c_double = 0.;
    let mut local_abs_frequency: libc::c_double = 0.;
    let mut elapsed: libc::c_double = 0.;
    let mut update_interval: libc::c_double = 0.;
    let mut correction_rate: libc::c_double = 0.;
    let mut orig_root_distance: libc::c_double = 0.;
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut raw_now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut manual: libc::c_int = 0;
    if initialised != 0 {
    } else {
        __assert_fail(b"initialised\x00" as *const u8 as *const libc::c_char,
                      b"reference.c\x00" as *const u8 as *const libc::c_char,
                      951 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 137],
                                                &[libc::c_char; 137]>(b"void REF_SetReference(int, NTP_Leap, int, uint32_t, IPAddr *, struct timespec *, double, double, double, double, double, double, double)\x00")).as_ptr());
    }
    /* Special modes are implemented elsewhere */
    if mode as libc::c_uint != REF_ModeNormal as libc::c_int as libc::c_uint {
        special_mode_sync(1 as libc::c_int, offset);
        return;
    }
    manual =
        (leap as libc::c_uint == LEAP_Unsynchronised as libc::c_int as libc::c_uint) as libc::c_int;
    LCL_ReadRawTime(&mut raw_now);
    LCL_GetOffsetCorrection(
        &mut raw_now,
        &mut uncorrected_offset,
        0 as *mut libc::c_double,
    );
    UTI_AddDoubleToTimespec(&mut raw_now, uncorrected_offset, &mut now);
    elapsed = UTI_DiffTimespecsToDouble(&mut now, ref_time);
    offset += elapsed * frequency;
    offset_sd += elapsed * frequency_sd;
    if last_ref_update.tv_sec != 0 {
        update_interval = UTI_DiffTimespecsToDouble(&mut now, &mut last_ref_update);
        update_interval = if update_interval > 0.0f64 {
            update_interval
        } else {
            0.0f64
        }
    } else {
        update_interval = 0.0f64
    }
    /* Get new estimates of the frequency and skew including the new data */
    get_clock_estimates(
        manual,
        frequency,
        skew,
        &mut frequency,
        &mut skew,
        &mut residual_frequency,
    );
    if is_offset_ok(offset) == 0 {
        return;
    }
    orig_root_distance = our_root_delay / 2.0f64 + get_root_dispersion(&mut now);
    are_we_synchronised =
        (leap as libc::c_uint != LEAP_Unsynchronised as libc::c_int as libc::c_uint) as libc::c_int;
    our_stratum = stratum + 1 as libc::c_int;
    our_ref_id = ref_id;
    if !ref_ip.is_null() {
        our_ref_ip = *ref_ip
    } else {
        our_ref_ip.family = 0 as libc::c_int as uint16_t
    }
    our_ref_time = *ref_time;
    our_skew = skew;
    our_residual_freq = residual_frequency;
    our_root_delay = root_delay;
    our_root_dispersion = root_dispersion;
    last_ref_update = now;
    last_ref_update_interval = update_interval;
    last_offset = offset;
    /* We want to correct the offset quickly, but we also want to keep the
    frequency error caused by the correction itself low.

    Define correction rate as the area of the region bounded by the graph of
    offset corrected in time. Set the rate so that the time needed to correct
    an offset equal to the current sourcestats stddev will be equal to the
    update interval multiplied by the correction time ratio (assuming linear
    adjustment). The offset and the time needed to make the correction are
    inversely proportional.

    This is only a suggestion and it's up to the system driver how the
    adjustment will be executed. */
    correction_rate = correction_time_ratio * 0.5f64 * offset_sd * update_interval;
    /* Check if the clock should be stepped */
    if is_step_limit_reached(offset, uncorrected_offset) != 0 {
        /* Cancel the uncorrected offset and correct the total offset by step */
        accumulate_offset = uncorrected_offset;
        step_offset = offset - uncorrected_offset
    } else {
        accumulate_offset = offset;
        step_offset = 0.0f64
    }
    /* Adjust the clock */
    LCL_AccumulateFrequencyAndOffset(frequency, accumulate_offset, correction_rate);
    update_leap_status(leap, raw_now.tv_sec, 0 as libc::c_int);
    maybe_log_offset(offset, raw_now.tv_sec);
    if step_offset != 0.0f64 {
        if LCL_ApplyStepOffset(step_offset) != 0 {
            LOG_Message(
                LOGS_WARN,
                b"System clock was stepped by %.6f seconds\x00" as *const u8 as *const libc::c_char,
                -step_offset,
            );
        }
    }
    LCL_SetSyncStatus(
        are_we_synchronised,
        offset_sd,
        root_delay / 2.0f64 + get_root_dispersion(&mut now),
    );
    /* Add a random error of up to one second to the reference time to make it
    less useful when disclosed to NTP and cmdmon clients for estimating
    receive timestamps in the interleaved symmetric NTP mode */
    fuzz_ref_time(&mut our_ref_time);
    local_abs_frequency = LCL_ReadAbsoluteFrequency();
    write_log(
        &mut now,
        combined_sources,
        local_abs_frequency,
        offset,
        offset_sd,
        uncorrected_offset,
        orig_root_distance,
    );
    if !drift_file.is_null() {
        /* Update drift file at most once per hour */
        drift_file_age += update_interval;
        if drift_file_age < 0.0f64 || drift_file_age > 3600.0f64 {
            update_drift_file(local_abs_frequency, our_skew);
            drift_file_age = 0.0f64
        }
    }
    /* Update fallback drifts */
    if !fb_drifts.is_null() && are_we_synchronised != 0 {
        update_fb_drifts(local_abs_frequency, update_interval);
        schedule_fb_drift(&mut now);
    }
    /* Update the moving average of squares of offset, quickly on start */
    if avg2_moving != 0 {
        avg2_offset += 0.1f64 * (offset * offset - avg2_offset)
    } else {
        if avg2_offset > 0.0f64 && avg2_offset < offset * offset {
            avg2_moving = 1 as libc::c_int
        }
        avg2_offset = offset * offset
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_SetManualReference(
    mut ref_time: *mut timespec,
    mut offset: libc::c_double,
    mut frequency: libc::c_double,
    mut skew: libc::c_double,
) {
    /* We are not synchronised to an external source, as such.  This is
    only supposed to be used with the local source option, really.
    Log as MANU in the tracking log, packets will have NTP_REFID_LOCAL. */
    REF_SetReference(
        0 as libc::c_int,
        LEAP_Unsynchronised,
        1 as libc::c_int,
        0x4d414e55 as libc::c_ulong as uint32_t,
        0 as *mut IPAddr,
        ref_time,
        offset,
        0.0f64,
        frequency,
        skew,
        skew,
        0.0f64,
        0.0f64,
    );
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_SetUnsynchronised() {
    /* Variables required for logging to statistics log */
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut now_raw: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut uncorrected_offset: libc::c_double = 0.;
    if initialised != 0 {
    } else {
        __assert_fail(
            b"initialised\x00" as *const u8 as *const libc::c_char,
            b"reference.c\x00" as *const u8 as *const libc::c_char,
            1102 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"void REF_SetUnsynchronised(void)\x00",
            ))
            .as_ptr(),
        );
    }
    /* Special modes are implemented elsewhere */
    if mode as libc::c_uint != REF_ModeNormal as libc::c_int as libc::c_uint {
        special_mode_sync(0 as libc::c_int, 0.0f64);
        return;
    }
    LCL_ReadRawTime(&mut now_raw);
    LCL_GetOffsetCorrection(
        &mut now_raw,
        &mut uncorrected_offset,
        0 as *mut libc::c_double,
    );
    UTI_AddDoubleToTimespec(&mut now_raw, uncorrected_offset, &mut now);
    if !fb_drifts.is_null() {
        schedule_fb_drift(&mut now);
    }
    update_leap_status(
        LEAP_Unsynchronised,
        0 as libc::c_int as time_t,
        0 as libc::c_int,
    );
    our_ref_ip.family = 1 as libc::c_int as uint16_t;
    our_ref_ip.addr.in4 = 0 as libc::c_int as uint32_t;
    our_stratum = 0 as libc::c_int;
    are_we_synchronised = 0 as libc::c_int;
    LCL_SetSyncStatus(0 as libc::c_int, 0.0f64, 0.0f64);
    write_log(
        &mut now,
        0 as libc::c_int,
        LCL_ReadAbsoluteFrequency(),
        0.0f64,
        0.0f64,
        uncorrected_offset,
        our_root_delay / 2.0f64 + get_root_dispersion(&mut now),
    );
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_GetReferenceParams(
    mut local_time: *mut timespec,
    mut is_synchronised: *mut libc::c_int,
    mut leap_status: *mut NTP_Leap,
    mut stratum: *mut libc::c_int,
    mut ref_id: *mut uint32_t,
    mut ref_time: *mut timespec,
    mut root_delay: *mut libc::c_double,
    mut root_dispersion: *mut libc::c_double,
) {
    let mut dispersion: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    if initialised != 0 {
    } else {
        __assert_fail(b"initialised\x00" as *const u8 as *const libc::c_char,
                      b"reference.c\x00" as *const u8 as *const libc::c_char,
                      1147 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 124],
                                                &[libc::c_char; 124]>(b"void REF_GetReferenceParams(struct timespec *, int *, NTP_Leap *, int *, uint32_t *, struct timespec *, double *, double *)\x00")).as_ptr());
    }
    if are_we_synchronised != 0 {
        dispersion = get_root_dispersion(local_time)
    } else {
        dispersion = 0.0f64
    }
    /* Local reference is active when enabled and the clock is not synchronised
    or the root distance exceeds the threshold */
    if are_we_synchronised != 0
        && !(enable_local_stratum != 0
            && our_root_delay / 2 as libc::c_int as libc::c_double + dispersion > local_distance)
    {
        *is_synchronised = 1 as libc::c_int;
        *stratum = our_stratum;
        *leap_status = if leap_in_progress == 0 {
            our_leap_status as libc::c_uint
        } else {
            LEAP_Unsynchronised as libc::c_int as libc::c_uint
        } as NTP_Leap;
        *ref_id = our_ref_id;
        *ref_time = our_ref_time;
        *root_delay = our_root_delay;
        *root_dispersion = dispersion
    } else if enable_local_stratum != 0 {
        *is_synchronised = 0 as libc::c_int;
        *stratum = local_stratum;
        *ref_id = 0x7f7f0101 as libc::c_ulong as uint32_t;
        /* Keep the reference timestamp up to date.  Adjust the timestamp to make
        sure that the transmit timestamp cannot come before this (which might
        fail a test of an NTP client). */
        delta = UTI_DiffTimespecsToDouble(local_time, &mut local_ref_time);
        if delta > 64.0f64 || delta < 1.0f64 {
            UTI_AddDoubleToTimespec(local_time, -1.0f64, &mut local_ref_time);
            fuzz_ref_time(&mut local_ref_time);
        }
        *ref_time = local_ref_time;
        /* Not much else we can do for leap second bits - maybe need to
        have a way for the administrator to feed leap bits in */
        *leap_status = LEAP_Normal;
        *root_delay = 0.0f64;
        *root_dispersion = 0.0f64
    } else {
        *is_synchronised = 0 as libc::c_int;
        *leap_status = LEAP_Unsynchronised;
        *stratum = 16 as libc::c_int;
        *ref_id = 0 as libc::c_ulong as uint32_t;
        UTI_ZeroTimespec(ref_time);
        /* These values seem to be standard for a client, and
        any peer or client of ours will ignore them anyway because
        we don't claim to be synchronised */
        *root_dispersion = 1.0f64;
        *root_delay = 1.0f64
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_GetOurStratum() -> libc::c_int {
    let mut now_cooked: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut ref_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut synchronised: libc::c_int = 0;
    let mut stratum: libc::c_int = 0;
    let mut leap_status: NTP_Leap = LEAP_Normal;
    let mut ref_id: uint32_t = 0;
    let mut root_delay: libc::c_double = 0.;
    let mut root_dispersion: libc::c_double = 0.;
    SCH_GetLastEventTime(
        &mut now_cooked,
        0 as *mut libc::c_double,
        0 as *mut timespec,
    );
    REF_GetReferenceParams(
        &mut now_cooked,
        &mut synchronised,
        &mut leap_status,
        &mut stratum,
        &mut ref_id,
        &mut ref_time,
        &mut root_delay,
        &mut root_dispersion,
    );
    return stratum;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_GetOrphanStratum() -> libc::c_int {
    if enable_local_stratum == 0
        || local_orphan == 0
        || mode as libc::c_uint != REF_ModeNormal as libc::c_int as libc::c_uint
    {
        return 16 as libc::c_int;
    }
    return local_stratum;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_GetSkew() -> libc::c_double {
    return our_skew;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_ModifyMaxupdateskew(mut new_max_update_skew: libc::c_double) {
    max_update_skew = new_max_update_skew * 1.0e-6f64;
}
/* Init function */
/* Fini function */
/* Set reference update mode */
/* Get reference update mode */
/* Function type for handlers to be called back when mode ends */
/* Set the handler for being notified of mode ending */
/* Get leap second handling mode */
/* Function which takes a local cooked time and returns the estimated
time of the reference.  It also returns the other parameters
required for forming the outgoing NTP packet.

local_time is the cooked local time returned by the LCL module

is_synchronised indicates whether we are synchronised to anything
at the moment.

leap indicates the current leap status

stratum is the stratum of this machine, when considered to be sync'd to the
reference

ref_id is the reference_id of the source

ref_time is the time at which the we last set the reference source up

root_delay is the root delay of the sample we are using

root_dispersion is the root dispersion of the sample we are using, with all the
skew etc added on.

*/
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
/* Mark the local clock as unsynchronised */
/* Return the current stratum of this host or 16 if the host is not
synchronised */
/* Return stratum of the local reference if orphan mode is enabled */
/* Return the current skew */
/* Modify the setting for the maximum skew we are prepared to allow updates on (in ppm). */
/* Modify makestep settings */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_ModifyMakestep(mut limit: libc::c_int, mut threshold: libc::c_double) {
    make_step_limit = limit;
    make_step_threshold = threshold;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_EnableLocal(
    mut stratum: libc::c_int,
    mut distance: libc::c_double,
    mut orphan: libc::c_int,
) {
    enable_local_stratum = 1 as libc::c_int;
    local_stratum = if 1 as libc::c_int
        > (if stratum < 16 as libc::c_int - 1 as libc::c_int {
            stratum
        } else {
            (16 as libc::c_int) - 1 as libc::c_int
        }) {
        1 as libc::c_int
    } else if stratum < 16 as libc::c_int - 1 as libc::c_int {
        stratum
    } else {
        (16 as libc::c_int) - 1 as libc::c_int
    };
    local_distance = distance;
    local_orphan = (orphan != 0) as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_DisableLocal() {
    enable_local_stratum = 0 as libc::c_int;
}
/* Check if current raw or cooked time is close to a leap second
and is better to discard any measurements */
#[no_mangle]
pub unsafe extern "C" fn REF_IsLeapSecondClose() -> libc::c_int {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut now_raw: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut t: time_t = 0;
    if our_leap_sec == 0 {
        return 0 as libc::c_int;
    }
    SCH_GetLastEventTime(&mut now, 0 as *mut libc::c_double, &mut now_raw);
    t = if now.tv_sec > 0 as libc::c_int as libc::c_long {
        now.tv_sec
    } else {
        -now.tv_sec
    };
    if ((t + 5 as libc::c_int as libc::c_long)
        % (24 as libc::c_int * 3600 as libc::c_int) as libc::c_long)
        < (2 as libc::c_int * 5 as libc::c_int) as libc::c_long
    {
        return 1 as libc::c_int;
    }
    t = if now_raw.tv_sec > 0 as libc::c_int as libc::c_long {
        now_raw.tv_sec
    } else {
        -now_raw.tv_sec
    };
    if ((t + 5 as libc::c_int as libc::c_long)
        % (24 as libc::c_int * 3600 as libc::c_int) as libc::c_long)
        < (2 as libc::c_int * 5 as libc::c_int) as libc::c_long
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* Return TAI-UTC offset corresponding to a time in UTC if available */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_GetTaiOffset(mut ts: *mut timespec) -> libc::c_int {
    let mut tai_offset: libc::c_int = 0;
    get_tz_leap((*ts).tv_sec, &mut tai_offset);
    return tai_offset;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn REF_GetTrackingReport(mut rep: *mut RPT_TrackingReport) {
    let mut now_raw: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut now_cooked: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut correction: libc::c_double = 0.;
    let mut synchronised: libc::c_int = 0;
    LCL_ReadRawTime(&mut now_raw);
    LCL_GetOffsetCorrection(&mut now_raw, &mut correction, 0 as *mut libc::c_double);
    UTI_AddDoubleToTimespec(&mut now_raw, correction, &mut now_cooked);
    REF_GetReferenceParams(
        &mut now_cooked,
        &mut synchronised,
        &mut (*rep).leap_status,
        &mut (*rep).stratum,
        &mut (*rep).ref_id,
        &mut (*rep).ref_time,
        &mut (*rep).root_delay,
        &mut (*rep).root_dispersion,
    );
    if (*rep).stratum == 16 as libc::c_int && synchronised == 0 {
        (*rep).stratum = 0 as libc::c_int
    }
    (*rep).ip_addr.family = 0 as libc::c_int as uint16_t;
    (*rep).current_correction = correction;
    (*rep).freq_ppm = LCL_ReadAbsoluteFrequency();
    (*rep).resid_freq_ppm = 0.0f64;
    (*rep).skew_ppm = 0.0f64;
    (*rep).last_update_interval = last_ref_update_interval;
    (*rep).last_offset = last_offset;
    (*rep).rms_offset = sqrt(avg2_offset);
    if synchronised != 0 {
        (*rep).ip_addr = our_ref_ip;
        (*rep).resid_freq_ppm = 1.0e6f64 * our_residual_freq;
        (*rep).skew_ppm = 1.0e6f64 * our_skew
    };
}
