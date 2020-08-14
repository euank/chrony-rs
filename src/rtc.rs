#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    /* Read the system clock, corrected according to all accumulated
    drifts and uncompensated offsets.

    In a kernel implementation with vernier frequency control (like
    Linux), and if we were to apply offsets by stepping the clock, this
    would be identical to raw time.  In any other case (use of
    adjtime()-like interface to correct offsets, and to adjust the
    frequency), we must correct the raw time to get this value */
    #[no_mangle]
    fn LCL_ReadCookedTime(ts: *mut timespec, err: *mut libc::c_double);
    /* Routine to apply an immediate offset by doing a sudden step if
    possible. (Intended for use after an initial estimate of offset has
    been obtained, so that we don't end up using adjtime to achieve a
    slew of an hour or something like that). A positive argument means
    the system clock is fast on true time, i.e. it needs to be stepped
    backwards. (Same convention as for AccumulateOffset routine). */
    #[no_mangle]
    fn LCL_ApplyStepOffset(offset: libc::c_double) -> libc::c_int;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CNF_GetDriftFile() -> *mut libc::c_char;
    #[no_mangle]
    fn CNF_GetRtcFile() -> *mut libc::c_char;
    #[no_mangle]
    fn CNF_GetRtcSync() -> libc::c_int;
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
    #[no_mangle]
    fn RTC_Linux_Initialise() -> libc::c_int;
    #[no_mangle]
    fn RTC_Linux_Finalise();
    #[no_mangle]
    fn RTC_Linux_TimePreInit(driftile_time: time_t) -> libc::c_int;
    #[no_mangle]
    fn RTC_Linux_TimeInit(
        after_hook: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        anything: *mut libc::c_void,
    );
    #[no_mangle]
    fn RTC_Linux_StartMeasurements();
    /* 0=success, 1=no driver, 2=can't write file */
    #[no_mangle]
    fn RTC_Linux_WriteParameters() -> libc::c_int;
    #[no_mangle]
    fn RTC_Linux_GetReport(report: *mut RPT_RTC_Report) -> libc::c_int;
    #[no_mangle]
    fn RTC_Linux_Trim() -> libc::c_int;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type time_t = __time_t;
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
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub init: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub fini: Option<unsafe extern "C" fn() -> ()>,
    pub time_pre_init: Option<unsafe extern "C" fn(_: time_t) -> libc::c_int>,
    pub time_init: Option<
        unsafe extern "C" fn(
            _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub start_measurements: Option<unsafe extern "C" fn() -> ()>,
    pub write_parameters: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub get_report: Option<unsafe extern "C" fn(_: *mut RPT_RTC_Report) -> libc::c_int>,
    pub trim: Option<unsafe extern "C" fn() -> libc::c_int>,
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

 */
/* defined LINUX */
/* ================================================== */
static mut driver_initialised: libc::c_int = 0 as libc::c_int;
static mut driver_preinit_ok: libc::c_int = 0 as libc::c_int;
static mut driver: C2RustUnnamed = unsafe {
    {
        let mut init = C2RustUnnamed {
            init: Some(RTC_Linux_Initialise as unsafe extern "C" fn() -> libc::c_int),
            fini: Some(RTC_Linux_Finalise as unsafe extern "C" fn() -> ()),
            time_pre_init: Some(
                RTC_Linux_TimePreInit as unsafe extern "C" fn(_: time_t) -> libc::c_int,
            ),
            time_init: Some(
                RTC_Linux_TimeInit
                    as unsafe extern "C" fn(
                        _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            start_measurements: Some(RTC_Linux_StartMeasurements as unsafe extern "C" fn() -> ()),
            write_parameters: Some(
                RTC_Linux_WriteParameters as unsafe extern "C" fn() -> libc::c_int,
            ),
            get_report: Some(
                RTC_Linux_GetReport as unsafe extern "C" fn(_: *mut RPT_RTC_Report) -> libc::c_int,
            ),
            trim: Some(RTC_Linux_Trim as unsafe extern "C" fn() -> libc::c_int),
        };
        init
    }
};
/* ================================================== */
/* Get the last modification time of the driftfile */
unsafe extern "C" fn get_driftfile_time() -> time_t {
    let mut buf: stat = stat {
        st_dev: 0,
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
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut drift_file: *mut libc::c_char = 0 as *mut libc::c_char;
    drift_file = CNF_GetDriftFile();
    if drift_file.is_null() {
        return 0 as libc::c_int as time_t;
    }
    if stat(drift_file, &mut buf) != 0 {
        return 0 as libc::c_int as time_t;
    }
    return buf.st_mtim.tv_sec;
}
/* ================================================== */
/* Set the system time to the driftfile time if it's in the future */
unsafe extern "C" fn apply_driftfile_time(mut t: time_t) {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    LCL_ReadCookedTime(&mut now, 0 as *mut libc::c_double);
    if now.tv_sec < t {
        if LCL_ApplyStepOffset((now.tv_sec - t) as libc::c_double) != 0 {
            LOG_Message(
                LOGS_INFO,
                b"System time restored from driftfile\x00" as *const u8 as *const libc::c_char,
            );
        }
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn RTC_Initialise(mut initial_set: libc::c_int) {
    let mut driftfile_time: time_t = 0;
    let mut file_name: *mut libc::c_char = 0 as *mut libc::c_char;
    /* If the -s option was specified, try to do an initial read of the RTC and
    set the system time to it.  Also, read the last modification time of the
    driftfile (i.e. system time when chronyd was previously stopped) and set
    the system time to it if it's in the future to bring the clock closer to
    the true time when the RTC is broken (e.g. it has no battery), is missing,
    or there is no RTC driver. */
    if initial_set != 0 {
        driftfile_time = get_driftfile_time();
        if driver.time_pre_init.is_some()
            && driver.time_pre_init.expect("non-null function pointer")(driftfile_time) != 0
        {
            driver_preinit_ok = 1 as libc::c_int
        } else {
            driver_preinit_ok = 0 as libc::c_int;
            if driftfile_time != 0 {
                apply_driftfile_time(driftfile_time);
            }
        }
    }
    driver_initialised = 0 as libc::c_int;
    /* This is how we tell whether the user wants to load the RTC
    driver, if he is on a machine where it is an option. */
    file_name = CNF_GetRtcFile();
    if !file_name.is_null() {
        if CNF_GetRtcSync() != 0 {
            LOG_Message(
                LOGS_FATAL,
                b"rtcfile directive cannot be used with rtcsync\x00" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if driver.init.is_some() {
            if driver.init.expect("non-null function pointer")() != 0 {
                driver_initialised = 1 as libc::c_int
            }
        } else {
            LOG_Message(
                LOGS_ERR,
                b"RTC not supported on this operating system\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn RTC_Finalise() {
    if driver_initialised != 0 {
        driver.fini.expect("non-null function pointer")();
    };
}
/* ================================================== */
/* Start the processing to get a single measurement from the real time
clock, and use it to trim the system time, based on knowing the
drift rate of the RTC and the error the last time we set it.  If the
TimePreInit routine has succeeded, we can be sure that the trim required
is not *too* large.

We are called with a hook to a function to be called after the
initialisation is complete.  We also call this if we cannot do the
initialisation. */
#[no_mangle]
pub unsafe extern "C" fn RTC_TimeInit(
    mut after_hook: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    mut anything: *mut libc::c_void,
) {
    if driver_initialised != 0 && driver_preinit_ok != 0 {
        driver.time_init.expect("non-null function pointer")(after_hook, anything);
    } else {
        after_hook.expect("non-null function pointer")(anything);
    };
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

 */
/* ================================================== */
/* Start the RTC measurement process */
#[no_mangle]
pub unsafe extern "C" fn RTC_StartMeasurements() {
    if driver_initialised != 0 {
        driver
            .start_measurements
            .expect("non-null function pointer")();
    };
    /* Benign if driver not present */
}
/* ================================================== */
/* Write RTC information out to RTC file.  Return 0 for success, 1 if
RTC driver not running, or 2 if the file cannot be written. */
#[no_mangle]
pub unsafe extern "C" fn RTC_WriteParameters() -> libc::c_int {
    if driver_initialised != 0 {
        return driver.write_parameters.expect("non-null function pointer")();
    } else {
        return 1 as libc::c_int;
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn RTC_GetReport(mut report: *mut RPT_RTC_Report) -> libc::c_int {
    if driver_initialised != 0 {
        return driver.get_report.expect("non-null function pointer")(report);
    } else {
        return 0 as libc::c_int;
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn RTC_Trim() -> libc::c_int {
    if driver_initialised != 0 {
        return driver.trim.expect("non-null function pointer")();
    } else {
        return 0 as libc::c_int;
    };
}
/* ================================================== */
