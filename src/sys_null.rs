#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
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
    #[no_mangle]
    fn lcl_RegisterSystemDrivers(
        read_freq: lcl_ReadFrequencyDriver,
        set_freq: lcl_SetFrequencyDriver,
        accrue_offset_0: lcl_AccrueOffsetDriver,
        apply_step_offset_0: lcl_ApplyStepOffsetDriver,
        offset_convert_0: lcl_OffsetCorrectionDriver,
        set_leap: lcl_SetLeapDriver,
        set_sync_status: lcl_SetSyncStatusDriver,
    );
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec) -> libc::c_double;
}
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
/* System driver to set the synchronisation status */
pub type lcl_SetSyncStatusDriver =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_double, _: libc::c_double) -> ()>;
/* System driver to schedule leap seconds and set TAI-UTC offset */
pub type lcl_SetLeapDriver = Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> ()>;
/* System driver to convert a raw time to an adjusted (cooked) time.
The number of seconds returned in 'corr' have to be added to the
raw time to get the corrected time */
pub type lcl_OffsetCorrectionDriver = Option<
    unsafe extern "C" fn(_: *mut timespec, _: *mut libc::c_double, _: *mut libc::c_double) -> (),
>;
/* System driver to apply a step offset. A positive argument means step
the clock forwards. */
pub type lcl_ApplyStepOffsetDriver = Option<unsafe extern "C" fn(_: libc::c_double) -> libc::c_int>;
/* System driver to accrue an offset. A positive argument means slew
the clock forwards.  The suggested correction rate of time to correct the
offset is given in 'corr_rate'. */
pub type lcl_AccrueOffsetDriver =
    Option<unsafe extern "C" fn(_: libc::c_double, _: libc::c_double) -> ()>;
/* System driver to set the current local frequency, in ppm relative
to nominal.  A positive value indicates that the local clock runs
fast when uncompensated.  Return actual frequency (may be different
from the requested frequency due to clamping or rounding). */
pub type lcl_SetFrequencyDriver = Option<unsafe extern "C" fn(_: libc::c_double) -> libc::c_double>;
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
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Miroslav Lichvar  2017
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

 Null clock driver for operation with no clock control.
 */
/* Current frequency offset of the system clock (in ppm) */
static mut freq: libc::c_double = 0.;
/* Offset of the system clock at the last update */
static mut offset_register: libc::c_double = 0.;
/* Time of the last update */
static mut last_update: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
/* ================================================== */
unsafe extern "C" fn update_offset() {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut duration: libc::c_double = 0.;
    LCL_ReadRawTime(&mut now);
    duration = UTI_DiffTimespecsToDouble(&mut now, &mut last_update);
    offset_register += 1.0e-6f64 * freq * duration;
    last_update = now;
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"System clock offset=%e freq=%f\x00" as *const u8 as *const libc::c_char,
            offset_register,
            freq,
        );
    };
}
/* ================================================== */
unsafe extern "C" fn read_frequency() -> libc::c_double {
    return freq;
}
/* ================================================== */
unsafe extern "C" fn set_frequency(mut freq_ppm: libc::c_double) -> libc::c_double {
    update_offset();
    freq = freq_ppm;
    return freq;
}
/* ================================================== */
unsafe extern "C" fn accrue_offset(mut offset: libc::c_double, mut corr_rate: libc::c_double) {
    offset_register += offset;
}
/* ================================================== */
unsafe extern "C" fn apply_step_offset(mut offset: libc::c_double) -> libc::c_int {
    return 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn offset_convert(
    mut raw: *mut timespec,
    mut corr: *mut libc::c_double,
    mut err: *mut libc::c_double,
) {
    let mut duration: libc::c_double = 0.;
    duration = UTI_DiffTimespecsToDouble(raw, &mut last_update);
    if duration > 1000.0f64 {
        update_offset();
        duration = 0.0f64
    }
    *corr = -1.0e-6f64 * freq * duration - offset_register;
    if !err.is_null() {
        *err = 0.0f64
    };
}
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Miroslav Lichvar  2017
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

 Header file for null clock driver
 */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Null_Initialise() {
    offset_register = 0.0f64;
    LCL_ReadRawTime(&mut last_update);
    lcl_RegisterSystemDrivers(
        Some(read_frequency as unsafe extern "C" fn() -> libc::c_double),
        Some(set_frequency as unsafe extern "C" fn(_: libc::c_double) -> libc::c_double),
        Some(accrue_offset as unsafe extern "C" fn(_: libc::c_double, _: libc::c_double) -> ()),
        Some(apply_step_offset as unsafe extern "C" fn(_: libc::c_double) -> libc::c_int),
        Some(
            offset_convert
                as unsafe extern "C" fn(
                    _: *mut timespec,
                    _: *mut libc::c_double,
                    _: *mut libc::c_double,
                ) -> (),
        ),
        None,
        None,
    );
    LOG_Message(
        LOGS_INFO,
        b"Disabled control of system clock\x00" as *const u8 as *const libc::c_char,
    );
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Null_Finalise() {}
