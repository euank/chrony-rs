#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn settimeofday(__tv: *const timeval, __tz: *const timezone)
     -> libc::c_int;
    #[no_mangle]
    fn lcl_InvokeDispersionNotifyHandlers(dispersion: libc::c_double);
    #[no_mangle]
    fn lcl_RegisterSystemDrivers(read_freq: lcl_ReadFrequencyDriver,
                                 set_freq: lcl_SetFrequencyDriver,
                                 accrue_offset_0: lcl_AccrueOffsetDriver,
                                 apply_step_offset_0:
                                     lcl_ApplyStepOffsetDriver,
                                 offset_convert_0: lcl_OffsetCorrectionDriver,
                                 set_leap: lcl_SetLeapDriver,
                                 set_sync_status_0: lcl_SetSyncStatusDriver);
    #[no_mangle]
    fn CNF_GetMaxSlewRate() -> libc::c_double;
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
    /* Add a handler.  Then handler MUST NOT deregister itself!!! */
    #[no_mangle]
    fn LCL_AddParameterChangeHandler(handler: LCL_ParameterChangeHandler,
                                     anything: *mut libc::c_void);
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
    /* This queues a timeout to elapse at a given (raw) local time */
    #[no_mangle]
    fn SCH_AddTimeout(ts: *mut timespec, handler: SCH_TimeoutHandler,
                      arg: SCH_ArbitraryArgument) -> SCH_TimeoutID;
    /* The next one probably ought to return a status code */
    #[no_mangle]
    fn SCH_RemoveTimeout(_: SCH_TimeoutID);
    #[no_mangle]
    fn UTI_AddDoubleToTimespec(start: *mut timespec,
                               increment: libc::c_double, end: *mut timespec);
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec)
     -> libc::c_double;
    #[no_mangle]
    fn UTI_TimespecToTimeval(ts: *mut timespec, tv: *mut timeval);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
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
pub type lcl_ReadFrequencyDriver
    =
    Option<unsafe extern "C" fn() -> libc::c_double>;
pub type lcl_SetFrequencyDriver
    =
    Option<unsafe extern "C" fn(_: libc::c_double) -> libc::c_double>;
pub type lcl_AccrueOffsetDriver
    =
    Option<unsafe extern "C" fn(_: libc::c_double, _: libc::c_double) -> ()>;
/* System driver to set the current local frequency, in ppm relative
   to nominal.  A positive value indicates that the local clock runs
   fast when uncompensated.  Return actual frequency (may be different
   from the requested frequency due to clamping or rounding). */
/* System driver to accrue an offset. A positive argument means slew
   the clock forwards.  The suggested correction rate of time to correct the
   offset is given in 'corr_rate'. */
/* System driver to apply a step offset. A positive argument means step
   the clock forwards. */
pub type lcl_ApplyStepOffsetDriver
    =
    Option<unsafe extern "C" fn(_: libc::c_double) -> libc::c_int>;
/* System driver to convert a raw time to an adjusted (cooked) time.
   The number of seconds returned in 'corr' have to be added to the
   raw time to get the corrected time */
pub type lcl_OffsetCorrectionDriver
    =
    Option<unsafe extern "C" fn(_: *mut timespec, _: *mut libc::c_double,
                                _: *mut libc::c_double) -> ()>;
/* System driver to schedule leap seconds and set TAI-UTC offset */
pub type lcl_SetLeapDriver
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> ()>;
/* System driver to set the synchronisation status */
pub type lcl_SetSyncStatusDriver
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_double,
                                _: libc::c_double) -> ()>;
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
pub type SCH_TimeoutHandler
    =
    Option<unsafe extern "C" fn(_: SCH_ArbitraryArgument) -> ()>;
pub type LCL_ParameterChangeHandler
    =
    Option<unsafe extern "C" fn(_: *mut timespec, _: *mut timespec,
                                _: libc::c_double, _: libc::c_double,
                                _: LCL_ChangeType, _: *mut libc::c_void)
               -> ()>;
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2014-2015
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

  Generic driver functions to complete system-specific drivers
  */
/* ================================================== */
/* System clock drivers */
static mut drv_read_freq: lcl_ReadFrequencyDriver = None;
static mut drv_set_freq: lcl_SetFrequencyDriver = None;
static mut drv_set_sync_status: lcl_SetSyncStatusDriver = None;
static mut drv_accrue_offset: lcl_AccrueOffsetDriver = None;
static mut drv_get_offset_correction: lcl_OffsetCorrectionDriver = None;
/* Current frequency as requested by the local module (in ppm) */
static mut base_freq: libc::c_double = 0.;
/* Maximum frequency that can be set by drv_set_freq (in ppm) */
static mut max_freq: libc::c_double = 0.;
/* Maximum expected delay in the actual frequency change (e.g. kernel ticks)
   in local time */
static mut max_freq_change_delay: libc::c_double = 0.;
/* Maximum allowed frequency offset relative to the base frequency */
static mut max_corr_freq: libc::c_double = 0.;
/* Amount of outstanding offset to process */
static mut offset_register: libc::c_double = 0.;
/* Current frequency offset between base_freq and the real clock frequency
   as set by drv_set_freq (not in ppm) */
static mut slew_freq: libc::c_double = 0.;
/* Time (raw) of last update of slewing frequency and offset */
static mut slew_start: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
/* Scheduler timeout ID for ending of the currently running slew */
static mut slew_timeout_id: SCH_TimeoutID = 0;
/* Suggested offset correction rate (correction time * offset) */
static mut correction_rate: libc::c_double = 0.;
/* Maximum expected offset correction error caused by delayed change in the
   real frequency of the clock */
static mut slew_error: libc::c_double = 0.;
/* Minimum offset that the system driver can slew faster than the maximum
   frequency offset that it allows to be set directly */
static mut fastslew_min_offset: libc::c_double = 0.;
/* Maximum slew rate of the system driver */
static mut fastslew_max_rate: libc::c_double = 0.;
/* Flag indicating that the system driver is currently slewing */
static mut fastslew_active: libc::c_int = 0;
/* ================================================== */
/* Adjust slew_start on clock step */
unsafe extern "C" fn handle_step(mut raw: *mut timespec,
                                 mut cooked: *mut timespec,
                                 mut dfreq: libc::c_double,
                                 mut doffset: libc::c_double,
                                 mut change_type: LCL_ChangeType,
                                 mut anything: *mut libc::c_void) {
    if change_type as libc::c_uint ==
           LCL_ChangeUnknownStep as libc::c_int as libc::c_uint {
        /* Reset offset and slewing */
        slew_start = *raw;
        offset_register = 0.0f64;
        update_slew();
    } else if change_type as libc::c_uint ==
                  LCL_ChangeStep as libc::c_int as libc::c_uint {
        UTI_AddDoubleToTimespec(&mut slew_start, -doffset, &mut slew_start);
    };
}
/* ================================================== */
unsafe extern "C" fn start_fastslew() {
    if drv_accrue_offset.is_none() { return }
    drv_accrue_offset.expect("non-null function pointer")(offset_register,
                                                          0.0f64);
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"fastslew offset=%e\x00" as *const u8 as
                        *const libc::c_char, offset_register);
    }
    offset_register = 0.0f64;
    fastslew_active = 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn stop_fastslew(mut now: *mut timespec) {
    let mut corr: libc::c_double = 0.;
    if drv_get_offset_correction.is_none() || fastslew_active == 0 { return }
    /* Cancel the remaining offset */
    drv_get_offset_correction.expect("non-null function pointer")(now,
                                                                  &mut corr,
                                                                  0 as
                                                                      *mut libc::c_double);
    drv_accrue_offset.expect("non-null function pointer")(corr, 0.0f64);
    offset_register -= corr;
}
/* ================================================== */
unsafe extern "C" fn clamp_freq(mut freq: libc::c_double) -> libc::c_double {
    if freq > max_freq { return max_freq }
    if freq < -max_freq { return -max_freq }
    return freq;
}
/* ================================================== */
/* End currently running slew and start a new one */
unsafe extern "C" fn update_slew() {
    let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut end_of_slew: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut old_slew_freq: libc::c_double = 0.;
    let mut total_freq: libc::c_double = 0.;
    let mut corr_freq: libc::c_double = 0.;
    let mut duration: libc::c_double = 0.;
    /* Remove currently running timeout */
    SCH_RemoveTimeout(slew_timeout_id);
    LCL_ReadRawTime(&mut now);
    /* Adjust the offset register by achieved slew */
    duration = UTI_DiffTimespecsToDouble(&mut now, &mut slew_start);
    offset_register -= slew_freq * duration;
    stop_fastslew(&mut now);
    /* Estimate how long should the next slew take */
    if fabs(offset_register) < 1.0e-9f64 {
        duration = 1.0e4f64
    } else {
        duration = correction_rate / fabs(offset_register);
        if duration < 1.0f64 { duration = 1.0f64 }
    }
    /* Get frequency offset needed to slew the offset in the duration
     and clamp it to the allowed maximum */
    corr_freq = offset_register / duration;
    if corr_freq < -max_corr_freq {
        corr_freq = -max_corr_freq
    } else if corr_freq > max_corr_freq { corr_freq = max_corr_freq }
    /* Let the system driver perform the slew if the requested frequency
     offset is too large for the frequency driver */
    if drv_accrue_offset.is_some() && fabs(corr_freq) >= fastslew_max_rate &&
           fabs(offset_register) > fastslew_min_offset {
        start_fastslew();
        corr_freq = 0.0f64
    }
    /* Get the new real frequency and clamp it */
    total_freq = clamp_freq(base_freq + corr_freq * (1.0e6f64 - base_freq));
    /* Set the new frequency (the actual frequency returned by the call may be
     slightly different from the requested frequency due to rounding) */
    total_freq =
        Some(drv_set_freq.expect("non-null function pointer")).expect("non-null function pointer")(total_freq);
    /* Compute the new slewing frequency, it's relative to the real frequency to
     make the calculation in offset_convert() cheaper */
    old_slew_freq = slew_freq;
    slew_freq = (total_freq - base_freq) / (1.0e6f64 - total_freq);
    /* Compute the dispersion introduced by changing frequency and add it
     to all statistics held at higher levels in the system */
    slew_error = fabs((old_slew_freq - slew_freq) * max_freq_change_delay);
    if slew_error >= 1.0e-9f64 {
        lcl_InvokeDispersionNotifyHandlers(slew_error);
    }
    /* Compute the duration of the slew and clamp it.  If the slewing frequency
     is zero or has wrong sign (e.g. due to rounding in the frequency driver or
     when base_freq is larger than max_freq, or fast slew is active), use the
     maximum timeout and try again on the next update. */
    if fabs(offset_register) < 1.0e-9f64 ||
           offset_register * slew_freq <= 0.0f64 {
        duration = 1.0e4f64
    } else {
        duration = offset_register / slew_freq;
        if duration < 1.0f64 {
            duration = 1.0f64
        } else if duration > 1.0e4f64 { duration = 1.0e4f64 }
    }
    /* Restart timer for the next update */
    UTI_AddDoubleToTimespec(&mut now, duration, &mut end_of_slew);
    slew_timeout_id =
        SCH_AddTimeout(&mut end_of_slew,
                       Some(handle_end_of_slew as
                                unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> ()), 0 as *mut libc::c_void);
    slew_start = now;
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"slew offset=%e corr_rate=%e base_freq=%f total_freq=%f slew_freq=%e duration=%f slew_error=%e\x00"
                        as *const u8 as *const libc::c_char, offset_register,
                    correction_rate, base_freq, total_freq, slew_freq,
                    duration, slew_error);
    };
}
/* ================================================== */
/* ================================================== */
unsafe extern "C" fn handle_end_of_slew(mut anything: *mut libc::c_void) {
    slew_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    update_slew();
}
/* ================================================== */
unsafe extern "C" fn read_frequency() -> libc::c_double { return base_freq; }
/* ================================================== */
unsafe extern "C" fn set_frequency(mut freq_ppm: libc::c_double)
 -> libc::c_double {
    base_freq = freq_ppm;
    update_slew();
    return base_freq;
}
/* ================================================== */
unsafe extern "C" fn accrue_offset(mut offset: libc::c_double,
                                   mut corr_rate: libc::c_double) {
    offset_register += offset;
    correction_rate = corr_rate;
    update_slew();
}
/* ================================================== */
/* Determine the correction to generate the cooked time for given raw time */
unsafe extern "C" fn offset_convert(mut raw: *mut timespec,
                                    mut corr: *mut libc::c_double,
                                    mut err: *mut libc::c_double) {
    let mut duration: libc::c_double = 0.;
    let mut fastslew_corr: libc::c_double = 0.;
    let mut fastslew_err: libc::c_double = 0.;
    duration = UTI_DiffTimespecsToDouble(raw, &mut slew_start);
    if drv_get_offset_correction.is_some() && fastslew_active != 0 {
        drv_get_offset_correction.expect("non-null function pointer")(raw,
                                                                      &mut fastslew_corr,
                                                                      &mut fastslew_err);
        if fastslew_corr == 0.0f64 && fastslew_err == 0.0f64 {
            fastslew_active = 0 as libc::c_int
        }
    } else { fastslew_err = 0.0f64; fastslew_corr = fastslew_err }
    *corr = slew_freq * duration + fastslew_corr - offset_register;
    if !err.is_null() {
        *err = fastslew_err;
        if fabs(duration) <= max_freq_change_delay { *err += slew_error }
    };
}
/* ================================================== */
/* Positive means currently fast of true time, i.e. jump backwards */
unsafe extern "C" fn apply_step_offset(mut offset: libc::c_double)
 -> libc::c_int {
    let mut old_time: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut new_time: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut new_time_tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut err: libc::c_double = 0.;
    LCL_ReadRawTime(&mut old_time);
    UTI_AddDoubleToTimespec(&mut old_time, -offset, &mut new_time);
    UTI_TimespecToTimeval(&mut new_time, &mut new_time_tv);
    if settimeofday(&mut new_time_tv, 0 as *const timezone) < 0 as libc::c_int
       {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"settimeofday() failed\x00" as *const u8 as
                            *const libc::c_char);
        }
        return 0 as libc::c_int
    }
    LCL_ReadRawTime(&mut old_time);
    err = UTI_DiffTimespecsToDouble(&mut old_time, &mut new_time);
    lcl_InvokeDispersionNotifyHandlers(fabs(err));
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn set_sync_status(mut synchronised: libc::c_int,
                                     mut est_error: libc::c_double,
                                     mut max_error: libc::c_double) {
    let mut offset: libc::c_double = 0.;
    offset = fabs(offset_register);
    if est_error < offset { est_error = offset }
    max_error += offset;
    if drv_set_sync_status.is_some() {
        drv_set_sync_status.expect("non-null function pointer")(synchronised,
                                                                est_error,
                                                                max_error);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Generic_CompleteFreqDriver(mut max_set_freq_ppm:
                                                            libc::c_double,
                                                        mut max_set_freq_delay:
                                                            libc::c_double,
                                                        mut sys_read_freq:
                                                            lcl_ReadFrequencyDriver,
                                                        mut sys_set_freq:
                                                            lcl_SetFrequencyDriver,
                                                        mut sys_apply_step_offset:
                                                            lcl_ApplyStepOffsetDriver,
                                                        mut min_fastslew_offset:
                                                            libc::c_double,
                                                        mut max_fastslew_rate:
                                                            libc::c_double,
                                                        mut sys_accrue_offset:
                                                            lcl_AccrueOffsetDriver,
                                                        mut sys_get_offset_correction:
                                                            lcl_OffsetCorrectionDriver,
                                                        mut sys_set_leap:
                                                            lcl_SetLeapDriver,
                                                        mut sys_set_sync_status:
                                                            lcl_SetSyncStatusDriver) {
    max_freq = max_set_freq_ppm;
    max_freq_change_delay =
        max_set_freq_delay * (1.0f64 + max_freq / 1.0e6f64);
    drv_read_freq = sys_read_freq;
    drv_set_freq = sys_set_freq;
    drv_accrue_offset = sys_accrue_offset;
    drv_get_offset_correction = sys_get_offset_correction;
    drv_set_sync_status = sys_set_sync_status;
    base_freq =
        Some(drv_read_freq.expect("non-null function pointer")).expect("non-null function pointer")();
    slew_freq = 0.0f64;
    offset_register = 0.0f64;
    max_corr_freq = CNF_GetMaxSlewRate() / 1.0e6f64;
    fastslew_min_offset = min_fastslew_offset;
    fastslew_max_rate = max_fastslew_rate / 1.0e6f64;
    fastslew_active = 0 as libc::c_int;
    lcl_RegisterSystemDrivers(Some(read_frequency as
                                       unsafe extern "C" fn()
                                           -> libc::c_double),
                              Some(set_frequency as
                                       unsafe extern "C" fn(_: libc::c_double)
                                           -> libc::c_double),
                              Some(accrue_offset as
                                       unsafe extern "C" fn(_: libc::c_double,
                                                            _: libc::c_double)
                                           -> ()),
                              if sys_apply_step_offset.is_some() {
                                  sys_apply_step_offset
                              } else {
                                  Some(apply_step_offset as
                                           unsafe extern "C" fn(_:
                                                                    libc::c_double)
                                               -> libc::c_int)
                              },
                              Some(offset_convert as
                                       unsafe extern "C" fn(_: *mut timespec,
                                                            _:
                                                                *mut libc::c_double,
                                                            _:
                                                                *mut libc::c_double)
                                           -> ()), sys_set_leap,
                              Some(set_sync_status as
                                       unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_double,
                                                            _: libc::c_double)
                                           -> ()));
    LCL_AddParameterChangeHandler(Some(handle_step as
                                           unsafe extern "C" fn(_:
                                                                    *mut timespec,
                                                                _:
                                                                    *mut timespec,
                                                                _:
                                                                    libc::c_double,
                                                                _:
                                                                    libc::c_double,
                                                                _:
                                                                    LCL_ChangeType,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> ()),
                                  0 as *mut libc::c_void);
}
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
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Generic_Finalise() {
    let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    /* Must *NOT* leave a slew running - clock could drift way off
     if the daemon is not restarted */
    SCH_RemoveTimeout(slew_timeout_id);
    slew_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    Some(drv_set_freq.expect("non-null function pointer")).expect("non-null function pointer")(clamp_freq(base_freq));
    LCL_ReadRawTime(&mut now);
    stop_fastslew(&mut now);
}
/* ================================================== */
