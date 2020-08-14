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
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    #[no_mangle]
    fn mktime(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn gmtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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
    /* Register a handler for when select goes true on a file descriptor */
    #[no_mangle]
    fn SCH_AddFileHandler(
        fd_0: libc::c_int,
        events: libc::c_int,
        handler: SCH_FileHandler,
        arg: SCH_ArbitraryArgument,
    );
    #[no_mangle]
    fn SCH_RemoveFileHandler(fd_0: libc::c_int);
    /* Get the time stamp taken after a file descriptor became ready or a timeout expired */
    #[no_mangle]
    fn SCH_GetLastEventTime(cooked: *mut timespec, err: *mut libc::c_double, raw: *mut timespec);
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
    /* Read the system clock, corrected according to all accumulated
    drifts and uncompensated offsets.

    In a kernel implementation with vernier frequency control (like
    Linux), and if we were to apply offsets by stepping the clock, this
    would be identical to raw time.  In any other case (use of
    adjtime()-like interface to correct offsets, and to adjust the
    frequency), we must correct the raw time to get this value */
    #[no_mangle]
    fn LCL_ReadCookedTime(ts: *mut timespec, err: *mut libc::c_double);
    /* Add a handler.  Then handler MUST NOT deregister itself!!! */
    #[no_mangle]
    fn LCL_AddParameterChangeHandler(
        handler: LCL_ParameterChangeHandler,
        anything: *mut libc::c_void,
    );
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
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec) -> libc::c_double;
    #[no_mangle]
    fn UTI_AddDoubleToTimespec(start: *mut timespec, increment: libc::c_double, end: *mut timespec);
    #[no_mangle]
    fn UTI_TimeToLogForm(t: time_t) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_AdjustTimespec(
        old_ts: *mut timespec,
        when: *mut timespec,
        new_ts: *mut timespec,
        delta_time: *mut libc::c_double,
        dfreq: libc::c_double,
        doffset: libc::c_double,
    );
    /* Set FD_CLOEXEC on descriptor */
    #[no_mangle]
    fn UTI_FdSetCloexec(fd_0: libc::c_int) -> libc::c_int;
    /* Open a file.  The full path of the file is constructed from the basedir
    (may be NULL), '/' (if basedir is not NULL), name, and suffix (may be NULL).
    Created files have specified permissions (umasked).  Returns NULL on error.
    The following modes are supported (if the mode is an uppercase character,
    errors are fatal):
    r/R - open an existing file for reading
    w/W - open a new file for writing (remove existing file)
    a/A - open an existing file for appending (create if does not exist) */
    #[no_mangle]
    fn UTI_OpenFile(
        basedir: *const libc::c_char,
        name: *const libc::c_char,
        suffix: *const libc::c_char,
        mode: libc::c_char,
        perm: mode_t,
    ) -> *mut FILE;
    /* Rename a temporary file by changing its suffix.  The paths are constructed as
    in UTI_OpenFile().  If the renaming fails, the file will be removed. */
    #[no_mangle]
    fn UTI_RenameTempFile(
        basedir: *const libc::c_char,
        name: *const libc::c_char,
        old_suffix: *const libc::c_char,
        new_suffix: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn REF_GetOurStratum() -> libc::c_int;
    #[no_mangle]
    fn RGR_FindBestRobustRegression(
        x: *mut libc::c_double,
        y: *mut libc::c_double,
        n: libc::c_int,
        tol: libc::c_double,
        b0: *mut libc::c_double,
        b1: *mut libc::c_double,
        n_runs_0: *mut libc::c_int,
        best_start: *mut libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn CNF_GetRtcDevice() -> *mut libc::c_char;
    #[no_mangle]
    fn CNF_GetLogRtc() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetRtcFile() -> *mut libc::c_char;
    #[no_mangle]
    fn CNF_GetRtcOnUtc() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetRtcAutotrim() -> libc::c_double;
    #[no_mangle]
    fn CNF_GetHwclockFile() -> *mut libc::c_char;
    #[no_mangle]
    fn Malloc2(nmemb: size_t, size: size_t) -> *mut libc::c_void;
}
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtc_time {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
}
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
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
pub type SCH_ArbitraryArgument = *mut libc::c_void;
pub type SCH_FileHandler =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: SCH_ArbitraryArgument) -> ()>;
pub type SCH_TimeoutHandler = Option<unsafe extern "C" fn(_: SCH_ArbitraryArgument) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_RTC_Report {
    pub ref_time: timespec,
    pub n_samples: libc::c_ushort,
    pub n_runs: libc::c_ushort,
    pub span_seconds: libc::c_ulong,
    pub rtc_seconds_fast: libc::c_double,
    pub rtc_gain_rate_ppm: libc::c_double,
}
pub const OM_NORMAL: OperatingMode = 0;
pub type OperatingMode = libc::c_uint;
pub const OM_AFTERTRIM: OperatingMode = 2;
pub const OM_INITIAL: OperatingMode = 1;
static mut operating_mode: OperatingMode = OM_NORMAL;
/* ================================================== */
static mut fd: libc::c_int = -(1 as libc::c_int);
static mut measurement_period: libc::c_int = 15 as libc::c_int;
static mut timeout_id: SCH_TimeoutID = 0 as libc::c_int as SCH_TimeoutID;
static mut skip_interrupts: libc::c_int = 0;
/* Real time clock samples.  We store the seconds count as originally
measured, together with a 'trim' that compensates these values for
any steps made to the RTC to bring it back into line
occasionally.  The trim is in seconds. */
static mut rtc_sec: *mut time_t = 0 as *const time_t as *mut time_t;
static mut rtc_trim: *mut libc::c_double = 0 as *const libc::c_double as *mut libc::c_double;
/* Reference time, against which delta times on the RTC scale are measured */
static mut rtc_ref: time_t = 0;
/* System clock samples associated with the above samples. */
static mut system_times: *mut timespec = 0 as *const timespec as *mut timespec;
/* Number of samples currently stored. */
static mut n_samples: libc::c_int = 0;
/* Number of new samples since last regression */
static mut n_samples_since_regression: libc::c_int = 0;
/* Number of runs of residuals in last regression (for logging) */
static mut n_runs: libc::c_int = 0;
/* Coefficients */
/* Whether they are valid */
static mut coefs_valid: libc::c_int = 0;
/* Reference time */
static mut coef_ref_time: time_t = 0;
/* Number of seconds by which RTC was fast of the system time at coef_ref_time */
static mut coef_seconds_fast: libc::c_double = 0.;
/* Estimated number of seconds that RTC gains relative to system time
for each second of ITS OWN time */
static mut coef_gain_rate: libc::c_double = 0.;
/* Gain rate saved just before we step the RTC to correct it to the
nearest second, so that we can write a useful set of coefs to the
RTC data file once we have reacquired its offset after the step */
static mut saved_coef_gain_rate: libc::c_double = 0.;
/* Threshold for automatic RTC trimming in seconds, zero when disabled */
static mut autotrim_threshold: libc::c_double = 0.;
/* Filename supplied by config file where RTC coefficients are
stored. */
static mut coefs_file_name: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* ================================================== */
/* Coefficients read from file at start of run. */
/* Whether we have tried to load the coefficients */
static mut tried_to_load_coefs: libc::c_int = 0 as libc::c_int;
/* Whether valid coefficients were read */
static mut valid_coefs_from_file: libc::c_int = 0 as libc::c_int;
/* Coefs read in */
static mut file_ref_time: time_t = 0;
static mut file_ref_offset: libc::c_double = 0.;
static mut file_rate_ppm: libc::c_double = 0.;
/* ================================================== */
/* Flag to remember whether to assume the RTC is running on UTC */
static mut rtc_on_utc: libc::c_int = 1 as libc::c_int;
/* ================================================== */
static mut logfileid: LOG_FileID = 0;
/* ================================================== */
static mut after_init_hook: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()> = None;
static mut after_init_hook_arg: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
/* ================================================== */
unsafe extern "C" fn discard_samples(mut new_first: libc::c_int) {
    let mut n_to_save: libc::c_int = 0;
    if new_first >= 0 as libc::c_int && new_first < n_samples {
    } else {
        __assert_fail(
            b"new_first >= 0 && new_first < n_samples\x00" as *const u8 as *const libc::c_char,
            b"rtc_linux.c\x00" as *const u8 as *const libc::c_char,
            166 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"void discard_samples(int)\x00",
            ))
            .as_ptr(),
        );
    }
    n_to_save = n_samples - new_first;
    memmove(
        rtc_sec as *mut libc::c_void,
        rtc_sec.offset(new_first as isize) as *const libc::c_void,
        (n_to_save as libc::c_ulong).wrapping_mul(::std::mem::size_of::<time_t>() as libc::c_ulong),
    );
    memmove(
        rtc_trim as *mut libc::c_void,
        rtc_trim.offset(new_first as isize) as *const libc::c_void,
        (n_to_save as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memmove(
        system_times as *mut libc::c_void,
        system_times.offset(new_first as isize) as *const libc::c_void,
        (n_to_save as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<timespec>() as libc::c_ulong),
    );
    n_samples = n_to_save;
}
unsafe extern "C" fn accumulate_sample(mut rtc: time_t, mut sys: *mut timespec) {
    if n_samples == 64 as libc::c_int {
        /* Discard oldest samples */
        discard_samples(4 as libc::c_int);
    }
    /* Discard all samples if the RTC was stepped back (not our trim) */
    if n_samples > 0 as libc::c_int
        && (*rtc_sec.offset((n_samples - 1 as libc::c_int) as isize) - rtc) as libc::c_double
            >= *rtc_trim.offset((n_samples - 1 as libc::c_int) as isize)
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"RTC samples discarded\x00" as *const u8 as *const libc::c_char,
            );
        }
        n_samples = 0 as libc::c_int
    }
    /* Always use most recent sample as reference */
    /* use sample only if n_sample is not negative*/
    if n_samples >= 0 as libc::c_int {
        rtc_ref = rtc;
        *rtc_sec.offset(n_samples as isize) = rtc;
        *rtc_trim.offset(n_samples as isize) = 0.0f64;
        *system_times.offset(n_samples as isize) = *sys;
        n_samples_since_regression += 1
    }
    n_samples += 1;
}
/* ================================================== */
/* The new_sample flag is to indicate whether to adjust the
measurement period depending on the behaviour of the standard
deviation. */
unsafe extern "C" fn run_regression(
    mut new_sample: libc::c_int,
    mut valid: *mut libc::c_int,
    mut ref_0: *mut time_t,
    mut fast: *mut libc::c_double,
    mut slope: *mut libc::c_double,
) {
    let mut rtc_rel: [libc::c_double; 64] = [0.; 64]; /* Relative times on RTC axis */
    let mut offsets: [libc::c_double; 64] = [0.; 64]; /* How much the RTC is fast of the system clock */
    let mut i: libc::c_int = 0;
    let mut est_intercept: libc::c_double = 0.;
    let mut est_slope: libc::c_double = 0.;
    let mut best_new_start: libc::c_int = 0;
    if n_samples > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < n_samples {
            rtc_rel[i as usize] = *rtc_trim.offset(i as isize)
                + (*rtc_sec.offset(i as isize) - rtc_ref) as libc::c_double;
            offsets[i as usize] = (rtc_ref - (*system_times.offset(i as isize)).tv_sec)
                as libc::c_double
                - 1.0e-9f64 * (*system_times.offset(i as isize)).tv_nsec as libc::c_double
                + rtc_rel[i as usize];
            i += 1
        }
        if RGR_FindBestRobustRegression(
            rtc_rel.as_mut_ptr(),
            offsets.as_mut_ptr(),
            n_samples,
            1.0e-9f64,
            &mut est_intercept,
            &mut est_slope,
            &mut n_runs,
            &mut best_new_start,
        ) != 0
        {
            /* Calculate and store coefficients.  We don't do any error
            bounds processing on any of these. */
            *valid = 1 as libc::c_int;
            *ref_0 = rtc_ref;
            *fast = est_intercept;
            *slope = est_slope;
            if best_new_start > 0 as libc::c_int {
                discard_samples(best_new_start);
            }
        }
    };
}
/* ================================================== */
unsafe extern "C" fn slew_samples(
    mut raw: *mut timespec,
    mut cooked: *mut timespec,
    mut dfreq: libc::c_double,
    mut doffset: libc::c_double,
    mut change_type: LCL_ChangeType,
    mut anything: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    let mut delta_time: libc::c_double = 0.;
    let mut old_seconds_fast: libc::c_double = 0.;
    let mut old_gain_rate: libc::c_double = 0.;
    if change_type as libc::c_uint == LCL_ChangeUnknownStep as libc::c_int as libc::c_uint {
        /* Drop all samples. */
        n_samples = 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < n_samples {
        UTI_AdjustTimespec(
            system_times.offset(i as isize),
            cooked,
            system_times.offset(i as isize),
            &mut delta_time,
            dfreq,
            doffset,
        );
        i += 1
    }
    old_seconds_fast = coef_seconds_fast;
    old_gain_rate = coef_gain_rate;
    if coefs_valid != 0 {
        coef_seconds_fast += doffset;
        coef_gain_rate += dfreq * (1.0f64 - coef_gain_rate)
    }
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"dfreq=%.8f doffset=%.6f old_fast=%.6f old_rate=%.3f new_fast=%.6f new_rate=%.3f\x00"
                as *const u8 as *const libc::c_char,
            dfreq,
            doffset,
            old_seconds_fast,
            1.0e6f64 * old_gain_rate,
            coef_seconds_fast,
            1.0e6f64 * coef_gain_rate,
        );
    };
}
/* ================================================== */
/* Function to convert from a time_t value represenging UTC to the
corresponding real time clock 'DMY HMS' form, taking account of
whether the user runs his RTC on the local time zone or UTC */
unsafe extern "C" fn rtc_from_t(mut t: *const time_t) -> *mut tm {
    if rtc_on_utc != 0 {
        return gmtime(t);
    } else {
        return localtime(t);
    };
}
/* ================================================== */
/* Inverse function to get back from RTC 'DMY HMS' form to time_t UTC
   form.  This essentially uses mktime(), but involves some awful
   complexity to cope with timezones.  The problem is that mktime's
   behaviour with regard to the daylight saving flag in the 'struct
   tm' does not seem to be reliable across all systems, unless that
   flag is set to zero.

   tm_isdst = -1 does not seem to work with all libc's - it is treated
   as meaning there is DST, or fails completely.  (It is supposed to
   use the timezone info to work out whether summer time is active at
   the specified epoch).

   tm_isdst = 1 fails if the local timezone has no summer time defined.

   The approach taken is as follows.  Suppose the RTC is on localtime.
   We perform all mktime calls with the tm_isdst field set to zero.

   Let y be the RTC reading in 'DMY HMS' form.  Let M be the mktime
   function with tm_isdst=0 and L be the localtime function.

   We seek x such that y = L(x).  Now there will exist a value Z(t)
   such that M(L(t)) = t + Z(t) for all t, where Z(t) depends on
   whether daylight saving is active at time t.

   We want L(x) = y.  Therefore M(L(x)) = x + Z = M(y).  But
   M(L(M(y))) = M(y) + Z.  Therefore x = M(y) - Z = M(y) - (M(L(M(y)))
   - M(y)).

   The case for the RTC running on UTC is identical but without the
   potential complication that Z depends on t.
*/
unsafe extern "C" fn t_from_rtc(mut stm: *mut tm) -> time_t {
    let mut temp1: tm = tm {
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
    let mut temp2: tm = tm {
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
    let mut diff: libc::c_long = 0;
    let mut t1: time_t = 0;
    let mut t2: time_t = 0;
    temp1 = *stm;
    temp1.tm_isdst = 0 as libc::c_int;
    t1 = mktime(&mut temp1);
    tm = if rtc_on_utc != 0 {
        gmtime(&mut t1)
    } else {
        localtime(&mut t1)
    };
    if tm.is_null() {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"gmtime()/localtime() failed\x00" as *const u8 as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int) as time_t;
    }
    temp2 = *tm;
    temp2.tm_isdst = 0 as libc::c_int;
    t2 = mktime(&mut temp2);
    diff = t2 - t1;
    if t1 - diff == -(1 as libc::c_int) as libc::c_long {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Could not convert RTC time\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    return t1 - diff;
}
/* ================================================== */
unsafe extern "C" fn read_hwclock_file(mut hwclock_file: *const libc::c_char) {
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut line: [libc::c_char; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    if hwclock_file.is_null() || *hwclock_file.offset(0 as libc::c_int as isize) == 0 {
        return;
    }
    in_0 = UTI_OpenFile(
        0 as *const libc::c_char,
        hwclock_file,
        0 as *const libc::c_char,
        'r' as i32 as libc::c_char,
        0 as libc::c_int as mode_t,
    );
    if in_0.is_null() {
        return;
    }
    /* Read third line from the file. */
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if fgets(
            line.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            in_0,
        )
        .is_null()
        {
            break;
        }
        i += 1
    }
    fclose(in_0);
    if i == 3 as libc::c_int
        && strncmp(
            line.as_mut_ptr(),
            b"LOCAL\x00" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        rtc_on_utc = 0 as libc::c_int
    } else if i == 3 as libc::c_int
        && strncmp(
            line.as_mut_ptr(),
            b"UTC\x00" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        rtc_on_utc = 1 as libc::c_int
    } else {
        LOG_Message(
            LOGS_WARN,
            b"Could not read RTC LOCAL/UTC setting from %s\x00" as *const u8 as *const libc::c_char,
            hwclock_file,
        );
    };
}
/* ================================================== */
unsafe extern "C" fn setup_config() {
    if CNF_GetRtcOnUtc() != 0 {
        rtc_on_utc = 1 as libc::c_int
    } else {
        rtc_on_utc = 0 as libc::c_int
    }
    read_hwclock_file(CNF_GetHwclockFile());
    autotrim_threshold = CNF_GetRtcAutotrim();
}
/* ================================================== */
/* Read the coefficients from the file where they were saved
the last time the program was run. */
unsafe extern "C" fn read_coefs_from_file() {
    let mut in_0: *mut FILE = 0 as *mut FILE; /* only gets set true if we succeed */
    if tried_to_load_coefs == 0 {
        valid_coefs_from_file = 0 as libc::c_int;
        tried_to_load_coefs = 1 as libc::c_int;
        if !coefs_file_name.is_null() && {
            in_0 = UTI_OpenFile(
                0 as *const libc::c_char,
                coefs_file_name,
                0 as *const libc::c_char,
                'r' as i32 as libc::c_char,
                0 as libc::c_int as mode_t,
            );
            !in_0.is_null()
        } {
            if !(fscanf(
                in_0,
                b"%d%ld%lf%lf\x00" as *const u8 as *const libc::c_char,
                &mut valid_coefs_from_file as *mut libc::c_int,
                &mut file_ref_time as *mut time_t,
                &mut file_ref_offset as *mut libc::c_double,
                &mut file_rate_ppm as *mut libc::c_double,
            ) == 4 as libc::c_int)
            {
                LOG_Message(
                    LOGS_WARN,
                    b"Could not read coefficients from %s\x00" as *const u8 as *const libc::c_char,
                    coefs_file_name,
                );
            }
            fclose(in_0);
        }
    };
}
/* ================================================== */
/* Write the coefficients to the file where they will be read
the next time the program is run. */
unsafe extern "C" fn write_coefs_to_file(
    mut valid: libc::c_int,
    mut ref_time: time_t,
    mut offset: libc::c_double,
    mut rate: libc::c_double,
) -> libc::c_int {
    let mut out: *mut FILE = 0 as *mut FILE;
    /* Create a temporary file with a '.tmp' extension. */
    out = UTI_OpenFile(
        0 as *const libc::c_char,
        coefs_file_name,
        b".tmp\x00" as *const u8 as *const libc::c_char,
        'w' as i32 as libc::c_char,
        0o644 as libc::c_int as mode_t,
    );
    if out.is_null() {
        return 2 as libc::c_int;
    }
    /* Gain rate is written out in ppm */
    fprintf(
        out,
        b"%1d %ld %.6f %.3f\n\x00" as *const u8 as *const libc::c_char,
        valid,
        ref_time,
        offset,
        1.0e6f64 * rate,
    );
    fclose(out);
    /* Rename the temporary file to the correct location */
    if UTI_RenameTempFile(
        0 as *const libc::c_char,
        coefs_file_name,
        b".tmp\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ) == 0
    {
        return 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn switch_interrupts(mut on_off: libc::c_int) -> libc::c_int {
    if ioctl(
        fd,
        (if on_off != 0 {
            ((0 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
                | (('p' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((0x3 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                | ((0 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                    as libc::c_uint
        } else {
            ((0 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
                | (('p' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((0x4 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                | ((0 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                    as libc::c_uint
        }) as libc::c_ulong,
        0 as libc::c_int,
    ) < 0 as libc::c_int
    {
        LOG_Message(
            LOGS_ERR,
            b"Could not %s RTC interrupt : %s\x00" as *const u8 as *const libc::c_char,
            if on_off != 0 {
                b"enable\x00" as *const u8 as *const libc::c_char
            } else {
                b"disable\x00" as *const u8 as *const libc::c_char
            },
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    if on_off != 0 {
        skip_interrupts = 1 as libc::c_int
    }
    return 1 as libc::c_int;
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

 ======================================================================

 */
/* ================================================== */
/* file_name is the name of the file where we save the RTC params
between executions.  Return status is whether we could initialise
on this version of the system. */
#[no_mangle]
pub unsafe extern "C" fn RTC_Linux_Initialise() -> libc::c_int {
    /* Try to open the device */
    fd = open(CNF_GetRtcDevice(), 0o2 as libc::c_int);
    if fd < 0 as libc::c_int {
        LOG_Message(
            LOGS_ERR,
            b"Could not open RTC device %s : %s\x00" as *const u8 as *const libc::c_char,
            CNF_GetRtcDevice(),
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    /* Make sure the RTC supports interrupts */
    if switch_interrupts(1 as libc::c_int) == 0 || switch_interrupts(0 as libc::c_int) == 0 {
        close(fd);
        return 0 as libc::c_int;
    }
    /* Close on exec */
    UTI_FdSetCloexec(fd);
    rtc_sec = Malloc2(
        64 as libc::c_int as size_t,
        ::std::mem::size_of::<time_t>() as libc::c_ulong,
    ) as *mut time_t;
    rtc_trim = Malloc2(
        64 as libc::c_int as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    system_times = Malloc2(
        64 as libc::c_int as size_t,
        ::std::mem::size_of::<timespec>() as libc::c_ulong,
    ) as *mut timespec;
    /* Setup details depending on configuration options */
    setup_config();
    /* In case it didn't get done by pre-init */
    coefs_file_name = CNF_GetRtcFile();
    n_samples = 0 as libc::c_int;
    n_samples_since_regression = 0 as libc::c_int;
    n_runs = 0 as libc::c_int;
    coefs_valid = 0 as libc::c_int;
    measurement_period = 15 as libc::c_int;
    operating_mode = OM_NORMAL;
    /* Register file handler */
    SCH_AddFileHandler(
        fd,
        1 as libc::c_int,
        Some(
            read_from_device
                as unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_void,
    );
    /* Register slew handler */
    LCL_AddParameterChangeHandler(
        Some(
            slew_samples
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
    logfileid = if CNF_GetLogRtc() != 0 {
        LOG_FileOpen(
            b"rtc\x00" as *const u8 as *const libc::c_char,
            b"   Date (UTC) Time   RTC fast (s) Val   Est fast (s)   Slope (ppm)  Ns  Nr Meas\x00"
                as *const u8 as *const libc::c_char,
        )
    } else {
        -(1 as libc::c_int)
    };
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn RTC_Linux_Finalise() {
    SCH_RemoveTimeout(timeout_id);
    timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    /* Remove input file handler */
    if fd >= 0 as libc::c_int {
        SCH_RemoveFileHandler(fd);
        switch_interrupts(0 as libc::c_int);
        close(fd);
        /* Save the RTC data */
        RTC_Linux_WriteParameters();
    }
    free(rtc_sec as *mut libc::c_void);
    free(rtc_trim as *mut libc::c_void);
    free(system_times as *mut libc::c_void);
}
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Richard P. Curnow  1997-2003
* Copyright (C) Miroslav Lichvar  2012-2014
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

 Real-time clock driver for linux.  This interfaces the program with
 the clock that keeps time when the machine is turned off.

 */
/* ================================================== */
/* Forward prototypes */
/* ================================================== */
unsafe extern "C" fn measurement_timeout(mut any: *mut libc::c_void) {
    timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    switch_interrupts(1 as libc::c_int);
}
/* ================================================== */
unsafe extern "C" fn set_rtc(mut new_rtc_time: time_t) {
    let mut rtc_tm: tm = tm {
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
    let mut rtc_raw: rtc_time = rtc_time {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
    };
    let mut status: libc::c_int = 0;
    rtc_tm = *rtc_from_t(&mut new_rtc_time);
    rtc_raw.tm_sec = rtc_tm.tm_sec;
    rtc_raw.tm_min = rtc_tm.tm_min;
    rtc_raw.tm_hour = rtc_tm.tm_hour;
    rtc_raw.tm_mday = rtc_tm.tm_mday;
    rtc_raw.tm_mon = rtc_tm.tm_mon;
    rtc_raw.tm_year = rtc_tm.tm_year;
    rtc_raw.tm_wday = rtc_tm.tm_wday;
    rtc_raw.tm_yday = rtc_tm.tm_yday;
    rtc_raw.tm_isdst = rtc_tm.tm_isdst;
    status = ioctl(
        fd,
        ((1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('p' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0xa as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<rtc_time>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut rtc_raw as *mut rtc_time,
    );
    if status < 0 as libc::c_int {
        LOG_Message(
            LOGS_ERR,
            b"Could not set RTC time\x00" as *const u8 as *const libc::c_char,
        );
    };
}
/* ================================================== */
unsafe extern "C" fn handle_initial_trim() {
    let mut rate: libc::c_double = 0.;
    let mut delta_time: libc::c_long = 0;
    let mut rtc_error_now: libc::c_double = 0.;
    let mut sys_error_now: libc::c_double = 0.;
    /* The idea is to accumulate some number of samples at 1 second
    intervals, then do a robust regression fit to this.  This
    should give a good fix on the intercept (=system clock error
    rel to RTC) at a particular time, removing risk of any
    particular sample being an outlier.  We can then look at the
    elapsed interval since the epoch recorded in the RTC file,
    and correct the system time accordingly. */
    run_regression(
        1 as libc::c_int,
        &mut coefs_valid,
        &mut coef_ref_time,
        &mut coef_seconds_fast,
        &mut coef_gain_rate,
    );
    n_samples_since_regression = 0 as libc::c_int;
    /* Set sample number to -1 so the next sample is not used, as it will not yet be corrected for System Trim*/
    n_samples = -(1 as libc::c_int);
    read_coefs_from_file();
    if valid_coefs_from_file != 0 {
        /* Can process data */
        delta_time = coef_ref_time - file_ref_time;
        rate = 1.0e-6f64 * file_rate_ppm;
        rtc_error_now = file_ref_offset + rate * delta_time as libc::c_double;
        /* sys_error_now is positive if the system clock is fast */
        sys_error_now = rtc_error_now - coef_seconds_fast;
        LCL_AccumulateOffset(sys_error_now, 0.0f64);
        LOG_Message(
            LOGS_INFO,
            b"System clock off from RTC by %f seconds (slew)\x00" as *const u8
                as *const libc::c_char,
            sys_error_now,
        );
    } else {
        LOG_Message(
            LOGS_WARN,
            b"No valid rtcfile coefficients\x00" as *const u8 as *const libc::c_char,
        );
    }
    coefs_valid = 0 as libc::c_int;
    after_init_hook.expect("non-null function pointer")(after_init_hook_arg);
    operating_mode = OM_NORMAL;
}
/* ================================================== */
unsafe extern "C" fn handle_relock_after_trim() {
    let mut valid: libc::c_int = 0;
    let mut ref_0: time_t = 0;
    let mut fast: libc::c_double = 0.;
    let mut slope: libc::c_double = 0.;
    valid = 0 as libc::c_int;
    run_regression(
        1 as libc::c_int,
        &mut valid,
        &mut ref_0,
        &mut fast,
        &mut slope,
    );
    if valid != 0 {
        write_coefs_to_file(1 as libc::c_int, ref_0, fast, saved_coef_gain_rate);
    } else if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
    {
        LOG_Message(
            LOGS_DEBUG,
            b"Could not do regression after trim\x00" as *const u8 as *const libc::c_char,
        );
    }
    coefs_valid = 0 as libc::c_int;
    n_samples = 0 as libc::c_int;
    n_samples_since_regression = 0 as libc::c_int;
    operating_mode = OM_NORMAL;
    measurement_period = 15 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn maybe_autotrim() {
    /* Trim only when in normal mode, the coefficients are fresh, the current
    offset is above the threshold and the system clock is synchronized */
    if operating_mode as libc::c_uint != OM_NORMAL as libc::c_int as libc::c_uint
        || coefs_valid == 0
        || n_samples_since_regression != 0
    {
        return;
    }
    if autotrim_threshold <= 0.0f64 || fabs(coef_seconds_fast) < autotrim_threshold {
        return;
    }
    if REF_GetOurStratum() >= 16 as libc::c_int {
        return;
    }
    RTC_Linux_Trim();
}
/* ================================================== */
unsafe extern "C" fn process_reading(mut rtc_time: time_t, mut system_time: *mut timespec) {
    let mut rtc_fast: libc::c_double = 0.;
    accumulate_sample(rtc_time, system_time);
    match operating_mode as libc::c_uint {
        0 => {
            if n_samples_since_regression >= 1 as libc::c_int {
                run_regression(
                    1 as libc::c_int,
                    &mut coefs_valid,
                    &mut coef_ref_time,
                    &mut coef_seconds_fast,
                    &mut coef_gain_rate,
                );
                n_samples_since_regression = 0 as libc::c_int;
                maybe_autotrim();
            }
        }
        1 => {
            if n_samples_since_regression >= 8 as libc::c_int {
                handle_initial_trim();
            }
        }
        2 => {
            if n_samples_since_regression >= 8 as libc::c_int {
                handle_relock_after_trim();
            }
        }
        _ => {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"rtc_linux.c\x00" as *const u8 as *const libc::c_char,
                744 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void process_reading(time_t, struct timespec *)\x00",
                ))
                .as_ptr(),
            );
        }
    }
    if logfileid != -(1 as libc::c_int) {
        rtc_fast = (rtc_time - (*system_time).tv_sec) as libc::c_double
            - 1.0e-9f64 * (*system_time).tv_nsec as libc::c_double;
        LOG_FileWrite(
            logfileid,
            b"%s %14.6f %1d  %14.6f  %12.3f  %2d  %2d %4d\x00" as *const u8 as *const libc::c_char,
            UTI_TimeToLogForm((*system_time).tv_sec),
            rtc_fast,
            coefs_valid,
            coef_seconds_fast,
            coef_gain_rate * 1.0e6f64,
            n_samples,
            n_runs,
            measurement_period,
        );
    };
}
/* ================================================== */
unsafe extern "C" fn read_from_device(
    mut fd_: libc::c_int,
    mut event: libc::c_int,
    mut any: *mut libc::c_void,
) {
    let mut status: libc::c_int = 0;
    let mut data: libc::c_ulong = 0;
    let mut sys_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut rtc_raw: rtc_time = rtc_time {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
    };
    let mut rtc_tm: tm = tm {
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
    let mut rtc_t: time_t = 0;
    let mut error: libc::c_int = 0 as libc::c_int;
    status = read(
        fd,
        &mut data as *mut libc::c_ulong as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        /* This looks like a bad error : the file descriptor was indicating it was
         * ready to read but we couldn't read anything.  Give up. */
        LOG_Message(
            LOGS_ERR,
            b"Could not read flags %s : %s\x00" as *const u8 as *const libc::c_char,
            CNF_GetRtcDevice(),
            strerror(*__errno_location()),
        ); /* Likely to raise error too, but just to be sure... */
        SCH_RemoveFileHandler(fd);
        switch_interrupts(0 as libc::c_int);
        close(fd);
        fd = -(1 as libc::c_int);
        return;
    }
    if skip_interrupts > 0 as libc::c_int {
        /* Wait for the next interrupt, this one may be bogus */
        skip_interrupts -= 1;
        return;
    }
    if data & 0x10 as libc::c_int as libc::c_ulong == 0x10 as libc::c_int as libc::c_ulong {
        /* Update interrupt detected */
        /* Read RTC time, sandwiched between two polls of the system clock
        so we can bound any error. */
        SCH_GetLastEventTime(&mut sys_time, 0 as *mut libc::c_double, 0 as *mut timespec);
        status = ioctl(
            fd,
            ((2 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
                | (('p' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((0x9 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                as libc::c_ulong
                | (::std::mem::size_of::<rtc_time>() as libc::c_ulong)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
            &mut rtc_raw as *mut rtc_time,
        );
        if status < 0 as libc::c_int {
            LOG_Message(
                LOGS_ERR,
                b"Could not read time from %s : %s\x00" as *const u8 as *const libc::c_char,
                CNF_GetRtcDevice(),
                strerror(*__errno_location()),
            );
            error = 1 as libc::c_int
        } else {
            /* Convert RTC time into a struct timespec */
            rtc_tm.tm_sec = rtc_raw.tm_sec;
            rtc_tm.tm_min = rtc_raw.tm_min;
            rtc_tm.tm_hour = rtc_raw.tm_hour;
            rtc_tm.tm_mday = rtc_raw.tm_mday;
            rtc_tm.tm_mon = rtc_raw.tm_mon;
            rtc_tm.tm_year = rtc_raw.tm_year;
            rtc_t = t_from_rtc(&mut rtc_tm);
            if rtc_t == -(1 as libc::c_int) as time_t {
                error = 1 as libc::c_int
            } else {
                process_reading(rtc_t, &mut sys_time);
                if n_samples < 4 as libc::c_int {
                    measurement_period = 15 as libc::c_int
                } else if n_samples < 6 as libc::c_int {
                    measurement_period = (15 as libc::c_int) << 1 as libc::c_int
                } else if n_samples < 10 as libc::c_int {
                    measurement_period = (15 as libc::c_int) << 2 as libc::c_int
                } else if n_samples < 14 as libc::c_int {
                    measurement_period = (15 as libc::c_int) << 3 as libc::c_int
                } else {
                    measurement_period = (15 as libc::c_int) << 4 as libc::c_int
                }
            }
        }
    }
    match operating_mode as libc::c_uint {
        1 => {
            if error != 0 {
                if 0 as libc::c_int != 0
                    && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
                {
                    LOG_Message(
                        LOGS_DEBUG,
                        b"Could not complete initial step due to errors\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
                operating_mode = OM_NORMAL;
                after_init_hook.expect("non-null function pointer")(after_init_hook_arg);
                switch_interrupts(0 as libc::c_int);
                timeout_id = SCH_AddTimeoutByDelay(
                    measurement_period as libc::c_double,
                    Some(measurement_timeout as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                    0 as *mut libc::c_void,
                )
            }
        }
        2 => {
            if error != 0 {
                if 0 as libc::c_int != 0
                    && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
                {
                    LOG_Message(
                        LOGS_DEBUG,
                        b"Could not complete after trim relock due to errors\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
                operating_mode = OM_NORMAL;
                switch_interrupts(0 as libc::c_int);
                timeout_id = SCH_AddTimeoutByDelay(
                    measurement_period as libc::c_double,
                    Some(measurement_timeout as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                    0 as *mut libc::c_void,
                )
            }
        }
        0 => {
            switch_interrupts(0 as libc::c_int);
            timeout_id = SCH_AddTimeoutByDelay(
                measurement_period as libc::c_double,
                Some(measurement_timeout as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                0 as *mut libc::c_void,
            )
        }
        _ => {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"rtc_linux.c\x00" as *const u8 as *const libc::c_char,
                874 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                    b"void read_from_device(int, int, void *)\x00",
                ))
                .as_ptr(),
            );
        }
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn RTC_Linux_TimeInit(
    mut after_hook: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    mut anything: *mut libc::c_void,
) {
    after_init_hook = after_hook;
    after_init_hook_arg = anything;
    operating_mode = OM_INITIAL;
    timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    switch_interrupts(1 as libc::c_int);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn RTC_Linux_StartMeasurements() {
    measurement_timeout(0 as *mut libc::c_void);
}
/* 0=success, 1=no driver, 2=can't write file */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn RTC_Linux_WriteParameters() -> libc::c_int {
    let mut retval: libc::c_int = 0;
    if fd < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if coefs_valid != 0 {
        retval = write_coefs_to_file(
            1 as libc::c_int,
            coef_ref_time,
            coef_seconds_fast,
            coef_gain_rate,
        )
    } else {
        /* Don't change the existing file, it may not be 100% valid but is our
        current best guess. */
        retval = 0 as libc::c_int
        /*write_coefs_to_file(0,0,0.0,0.0); */
    }
    return retval;
}
/* ================================================== */
/* Try to set the system clock from the RTC, in the same manner as
/sbin/hwclock -s would do.  We're not as picky about OS version
etc in this case, since we have fewer requirements regarding the
RTC behaviour than we do for the rest of the module. */
#[no_mangle]
pub unsafe extern "C" fn RTC_Linux_TimePreInit(mut driftfile_time: time_t) -> libc::c_int {
    let mut fd_0: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut rtc_raw: rtc_time = rtc_time {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
    };
    let mut rtc_raw_retry: rtc_time = rtc_time {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
    };
    let mut rtc_tm: tm = tm {
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
    let mut rtc_t: time_t = 0;
    let mut accumulated_error: libc::c_double = 0.;
    let mut sys_offset: libc::c_double = 0.;
    let mut new_sys_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut old_sys_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    coefs_file_name = CNF_GetRtcFile();
    setup_config();
    read_coefs_from_file();
    fd_0 = open(CNF_GetRtcDevice(), 0 as libc::c_int);
    if fd_0 < 0 as libc::c_int {
        return 0 as libc::c_int;
        /* Can't open it, and won't be able to later */
    }
    loop
    /* Retry reading the rtc until both read attempts give the same sec value.
    This way the race condition is prevented that the RTC has updated itself
    during the first read operation. */
    {
        status = ioctl(
            fd_0,
            ((2 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
                | (('p' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((0x9 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                as libc::c_ulong
                | (::std::mem::size_of::<rtc_time>() as libc::c_ulong)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
            &mut rtc_raw as *mut rtc_time,
        );
        if status >= 0 as libc::c_int {
            status = ioctl(
                fd_0,
                ((2 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
                    | (('p' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                    | ((0x9 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                    as libc::c_ulong
                    | (::std::mem::size_of::<rtc_time>() as libc::c_ulong)
                        << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
                &mut rtc_raw_retry as *mut rtc_time,
            )
        }
        if !(status >= 0 as libc::c_int && rtc_raw.tm_sec != rtc_raw_retry.tm_sec) {
            break;
        }
    }
    /* Read system clock */
    LCL_ReadCookedTime(&mut old_sys_time, 0 as *mut libc::c_double);
    close(fd_0);
    if status >= 0 as libc::c_int {
        /* Convert to seconds since 1970 */
        rtc_tm.tm_sec = rtc_raw.tm_sec;
        rtc_tm.tm_min = rtc_raw.tm_min;
        rtc_tm.tm_hour = rtc_raw.tm_hour;
        rtc_tm.tm_mday = rtc_raw.tm_mday;
        rtc_tm.tm_mon = rtc_raw.tm_mon;
        rtc_tm.tm_year = rtc_raw.tm_year;
        rtc_t = t_from_rtc(&mut rtc_tm);
        if rtc_t != -(1 as libc::c_int) as time_t {
            /* Work out approximatation to correct time (to about the
            nearest second) */
            if valid_coefs_from_file != 0 {
                accumulated_error = file_ref_offset
                    + (rtc_t - file_ref_time) as libc::c_double * 1.0e-6f64 * file_rate_ppm
            } else {
                accumulated_error = 0.0f64
            }
            /* Correct time */
            new_sys_time.tv_sec = rtc_t;
            /* Average error in the RTC reading */
            new_sys_time.tv_nsec = 500000000 as libc::c_int as __syscall_slong_t;
            UTI_AddDoubleToTimespec(&mut new_sys_time, -accumulated_error, &mut new_sys_time);
            if new_sys_time.tv_sec < driftfile_time {
                LOG_Message(
                    LOGS_WARN,
                    b"RTC time before last driftfile modification (ignored)\x00" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            sys_offset = UTI_DiffTimespecsToDouble(&mut old_sys_time, &mut new_sys_time);
            /* Set system time only if the step is larger than 1 second */
            if fabs(sys_offset) >= 1.0f64 {
                if LCL_ApplyStepOffset(sys_offset) != 0 {
                    LOG_Message(
                        LOGS_INFO,
                        b"System time set from RTC\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
        } else {
            return 0 as libc::c_int;
        }
    } else {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn RTC_Linux_GetReport(mut report: *mut RPT_RTC_Report) -> libc::c_int {
    (*report).ref_time.tv_sec = coef_ref_time;
    (*report).ref_time.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    (*report).n_samples = n_samples as libc::c_ushort;
    (*report).n_runs = n_runs as libc::c_ushort;
    if n_samples > 1 as libc::c_int {
        (*report).span_seconds = (*rtc_sec.offset((n_samples - 1 as libc::c_int) as isize)
            - *rtc_sec.offset(0 as libc::c_int as isize)
            + (*rtc_trim.offset((n_samples - 1 as libc::c_int) as isize)
                - *rtc_trim.offset(0 as libc::c_int as isize)) as libc::c_long)
            as libc::c_ulong
    } else {
        (*report).span_seconds = 0 as libc::c_int as libc::c_ulong
    }
    (*report).rtc_seconds_fast = coef_seconds_fast;
    (*report).rtc_gain_rate_ppm = 1.0e6f64 * coef_gain_rate;
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn RTC_Linux_Trim() -> libc::c_int {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    /* Remember the slope coefficient - we won't be able to determine a
    good one in a few seconds when we determine the new offset! */
    saved_coef_gain_rate = coef_gain_rate;
    if fabs(coef_seconds_fast) > 1.0f64 {
        LOG_Message(
            LOGS_INFO,
            b"RTC wrong by %.3f seconds (step)\x00" as *const u8 as *const libc::c_char,
            coef_seconds_fast,
        );
        /* Do processing to set clock.  Let R be the value we set the
        RTC to, then in 500ms the RTC ticks (R+1) (see comments in
        arch/i386/kernel/time.c about the behaviour of the real time
        clock chip).  If S is the system time now, the error at the
        next RTC tick is given by E = (R+1) - (S+0.5).  Ideally we
        want |E| <= 0.5, which implies R <= S <= R+1, i.e. R is just
        the rounded down part of S, i.e. the seconds part. */
        LCL_ReadCookedTime(&mut now, 0 as *mut libc::c_double);
        set_rtc(now.tv_sec);
        /* All old samples will now look bogus under the new
        regime. */
        n_samples = 0 as libc::c_int;
        operating_mode = OM_AFTERTRIM;
        /* Estimate the offset in case writertc is called or chronyd
        is terminated during rapid sampling */
        coef_seconds_fast = -now.tv_nsec as libc::c_double / 1.0e9f64 + 0.5f64;
        coef_ref_time = now.tv_sec;
        /* And start rapid sampling, interrupts on now */
        SCH_RemoveTimeout(timeout_id);
        timeout_id = 0 as libc::c_int as SCH_TimeoutID;
        switch_interrupts(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
