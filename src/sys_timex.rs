#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]

use c2rust_bitfields::BitfieldStruct;

extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn adjtimex(__ntx: *mut timex) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn CNF_GetRtcSync() -> libc::c_int;
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

     Header file for generic driver
     */
    /* Register a completed driver that implements offset functions on top of
    provided frequency functions */
    #[no_mangle]
    fn SYS_Generic_CompleteFreqDriver(
        max_set_freq_ppm: libc::c_double,
        max_set_freq_delay: libc::c_double,
        sys_read_freq: lcl_ReadFrequencyDriver,
        sys_set_freq: lcl_SetFrequencyDriver,
        sys_apply_step_offset: lcl_ApplyStepOffsetDriver,
        min_fastslew_offset: libc::c_double,
        max_fastslew_rate: libc::c_double,
        sys_accrue_offset: lcl_AccrueOffsetDriver,
        sys_get_offset_correction: lcl_OffsetCorrectionDriver,
        sys_set_leap: lcl_SetLeapDriver,
        sys_set_sync_status: lcl_SetSyncStatusDriver,
    );
    #[no_mangle]
    fn SYS_Generic_Finalise();
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char, _: ...);
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
pub type lcl_ReadFrequencyDriver = Option<unsafe extern "C" fn() -> libc::c_double>;
/* System driver to set the current local frequency, in ppm relative
to nominal.  A positive value indicates that the local clock runs
fast when uncompensated.  Return actual frequency (may be different
from the requested frequency due to clamping or rounding). */
pub type lcl_SetFrequencyDriver = Option<unsafe extern "C" fn(_: libc::c_double) -> libc::c_double>;
/* System driver to accrue an offset. A positive argument means slew
the clock forwards.  The suggested correction rate of time to correct the
offset is given in 'corr_rate'. */
pub type lcl_AccrueOffsetDriver =
    Option<unsafe extern "C" fn(_: libc::c_double, _: libc::c_double) -> ()>;
/* System driver to apply a step offset. A positive argument means step
the clock forwards. */
pub type lcl_ApplyStepOffsetDriver = Option<unsafe extern "C" fn(_: libc::c_double) -> libc::c_int>;
/* System driver to convert a raw time to an adjusted (cooked) time.
The number of seconds returned in 'corr' have to be added to the
raw time to get the corrected time */
pub type lcl_OffsetCorrectionDriver = Option<
    unsafe extern "C" fn(_: *mut timespec, _: *mut libc::c_double, _: *mut libc::c_double) -> (),
>;
/* System driver to schedule leap seconds and set TAI-UTC offset */
pub type lcl_SetLeapDriver = Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> ()>;
/* System driver to set the synchronisation status */
pub type lcl_SetSyncStatusDriver =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_double, _: libc::c_double) -> ()>;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
/* Saved timex status */
static mut sys_status: libc::c_int = 0;
/* Saved TAI-UTC offset */
static mut sys_tai_offset: libc::c_int = 0;
/* ================================================== */
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
    return txc.freq as libc::c_double
        / -(((1 as libc::c_int) << 16 as libc::c_int) as libc::c_double);
}
/* ================================================== */
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
    txc.modes = 0x2 as libc::c_int as libc::c_uint;
    txc.freq = (freq_ppm * -(((1 as libc::c_int) << 16 as libc::c_int) as libc::c_double))
        as __syscall_slong_t;
    SYS_Timex_Adjust(&mut txc, 0 as libc::c_int);
    return txc.freq as libc::c_double
        / -(((1 as libc::c_int) << 16 as libc::c_int) as libc::c_double);
}
/* ================================================== */
unsafe extern "C" fn set_leap(mut leap: libc::c_int, mut tai_offset: libc::c_int) {
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
    let mut applied: libc::c_int = 0;
    let mut prev_status: libc::c_int = 0;
    txc.modes = 0 as libc::c_int as libc::c_uint;
    applied = (SYS_Timex_Adjust(&mut txc, 0 as libc::c_int) == 4 as libc::c_int) as libc::c_int;
    prev_status = sys_status;
    sys_status &= !(0x10 as libc::c_int | 0x20 as libc::c_int);
    if leap > 0 as libc::c_int {
        sys_status |= 0x10 as libc::c_int
    } else if leap < 0 as libc::c_int {
        sys_status |= 0x20 as libc::c_int
    }
    txc.modes = 0x10 as libc::c_int as libc::c_uint;
    txc.status = sys_status;
    if tai_offset != 0 {
        txc.modes |= 0x80 as libc::c_int as libc::c_uint;
        txc.constant = tai_offset as __syscall_slong_t;
        if applied != 0 && sys_status & (0x10 as libc::c_int | 0x20 as libc::c_int) == 0 {
            sys_tai_offset += if prev_status & 0x10 as libc::c_int != 0 {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }
        }
        if sys_tai_offset != tai_offset {
            sys_tai_offset = tai_offset;
            LOG_Message(
                LOGS_INFO,
                b"System clock TAI offset set to %d seconds\x00" as *const u8
                    as *const libc::c_char,
                tai_offset,
            );
        }
    }
    SYS_Timex_Adjust(&mut txc, 0 as libc::c_int);
    if prev_status != sys_status {
        LOG_Message(
            LOGS_INFO,
            b"System clock status %s leap second\x00" as *const u8 as *const libc::c_char,
            if leap != 0 {
                if leap > 0 as libc::c_int {
                    b"set to insert\x00" as *const u8 as *const libc::c_char
                } else {
                    b"set to delete\x00" as *const u8 as *const libc::c_char
                }
            } else if applied != 0 {
                b"reset after\x00" as *const u8 as *const libc::c_char
            } else {
                b"set to not insert/delete\x00" as *const u8 as *const libc::c_char
            },
        );
    };
}
/* ================================================== */
unsafe extern "C" fn set_sync_status(
    mut synchronised: libc::c_int,
    mut est_error: libc::c_double,
    mut max_error: libc::c_double,
) {
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
    if synchronised != 0 {
        if est_error > 16.0f64 {
            est_error = 16.0f64
        }
        if max_error >= 16.0f64 {
            max_error = 16.0f64;
            synchronised = 0 as libc::c_int
        }
    } else {
        max_error = 16.0f64;
        est_error = max_error
    }
    /* On Linux clear the UNSYNC flag only if rtcsync is enabled */
    if CNF_GetRtcSync() == 0 {
        synchronised = 0 as libc::c_int
    }
    if synchronised != 0 {
        sys_status &= !(0x40 as libc::c_int)
    } else {
        sys_status |= 0x40 as libc::c_int
    }
    txc.modes = (0x10 as libc::c_int | 0x8 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint;
    txc.status = sys_status;
    txc.esterror = (est_error * 1.0e6f64) as __syscall_slong_t;
    txc.maxerror = (max_error * 1.0e6f64) as __syscall_slong_t;
    (SYS_Timex_Adjust(&mut txc, 1 as libc::c_int)) < 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn initialise_timex() {
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
    sys_status = 0x40 as libc::c_int;
    sys_tai_offset = 0 as libc::c_int;
    /* Reset PLL offset */
    txc.modes = (0x1 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint;
    txc.status = 0x1 as libc::c_int | sys_status;
    txc.offset = 0 as libc::c_int as __syscall_slong_t;
    SYS_Timex_Adjust(&mut txc, 0 as libc::c_int);
    /* Turn PLL off */
    txc.modes = 0x10 as libc::c_int as libc::c_uint;
    txc.status = sys_status;
    SYS_Timex_Adjust(&mut txc, 0 as libc::c_int);
}
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
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Timex_Initialise() {
    SYS_Timex_InitialiseWithFunctions(
        500.0f64,
        1.0f64 / 100 as libc::c_int as libc::c_double,
        None,
        None,
        None,
        0.0f64,
        0.0f64,
        None,
        None,
    );
}
/* Initialise with some driver functions replaced with special versions */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Timex_InitialiseWithFunctions(
    mut max_set_freq_ppm: libc::c_double,
    mut max_set_freq_delay: libc::c_double,
    mut sys_read_freq: lcl_ReadFrequencyDriver,
    mut sys_set_freq: lcl_SetFrequencyDriver,
    mut sys_apply_step_offset: lcl_ApplyStepOffsetDriver,
    mut min_fastslew_offset: libc::c_double,
    mut max_fastslew_rate: libc::c_double,
    mut sys_accrue_offset: lcl_AccrueOffsetDriver,
    mut sys_get_offset_correction: lcl_OffsetCorrectionDriver,
) {
    initialise_timex();
    SYS_Generic_CompleteFreqDriver(
        max_set_freq_ppm,
        max_set_freq_delay,
        if sys_read_freq.is_some() {
            sys_read_freq
        } else {
            Some(read_frequency as unsafe extern "C" fn() -> libc::c_double)
        },
        if sys_set_freq.is_some() {
            sys_set_freq
        } else {
            Some(set_frequency as unsafe extern "C" fn(_: libc::c_double) -> libc::c_double)
        },
        sys_apply_step_offset,
        min_fastslew_offset,
        max_fastslew_rate,
        sys_accrue_offset,
        sys_get_offset_correction,
        Some(set_leap as unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> ()),
        Some(
            set_sync_status
                as unsafe extern "C" fn(_: libc::c_int, _: libc::c_double, _: libc::c_double) -> (),
        ),
    );
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Timex_Finalise() {
    SYS_Generic_Finalise();
}
/* Wrapper for adjtimex()/ntp_adjtime() */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Timex_Adjust(
    mut txc: *mut timex,
    mut ignore_error: libc::c_int,
) -> libc::c_int {
    let mut state: libc::c_int = 0;
    state = adjtimex(txc);
    if state < 0 as libc::c_int {
        LOG_Message(
            if ignore_error != 0 {
                LOGS_DEBUG as libc::c_int
            } else {
                LOGS_FATAL as libc::c_int
            } as LOG_Severity,
            b"adjtimex(0x%x) failed : %s\x00" as *const u8 as *const libc::c_char,
            (*txc).modes,
            strerror(*__errno_location()),
        );
    }
    return state;
}
