#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    /* Add a handler.  Then handler MUST NOT deregister itself!!! */
    #[no_mangle]
    fn LCL_AddParameterChangeHandler(handler: LCL_ParameterChangeHandler,
                                     anything: *mut libc::c_void);
    /* Remove a handler */
    #[no_mangle]
    fn LCL_RemoveParameterChangeHandler(_: LCL_ParameterChangeHandler,
                                        anything: *mut libc::c_void);
    /* Read the absolute system frequency, relative to the uncompensated
   system.  Returned in units of parts per million.  Thus the result of
   this is how many seconds fast the uncompensated system would be after
   its own time has reached 1 million seconds from the start of the
   measurement.  */
    #[no_mangle]
    fn LCL_ReadAbsoluteFrequency() -> libc::c_double;
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
    fn Malloc2(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn RGR_FindBestRobustRegression(x: *mut libc::c_double,
                                    y: *mut libc::c_double, n: libc::c_int,
                                    tol: libc::c_double,
                                    b0: *mut libc::c_double,
                                    b1: *mut libc::c_double,
                                    n_runs: *mut libc::c_int,
                                    best_start: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec)
     -> libc::c_double;
    #[no_mangle]
    fn UTI_AdjustTimespec(old_ts: *mut timespec, when: *mut timespec,
                          new_ts: *mut timespec,
                          delta_time: *mut libc::c_double,
                          dfreq: libc::c_double, doffset: libc::c_double);
    #[no_mangle]
    fn UTI_AddDoubleToTimespec(start: *mut timespec,
                               increment: libc::c_double, end: *mut timespec);
}
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HCL_Instance_Record {
    pub hw_ref: timespec,
    pub local_ref: timespec,
    pub x_data: *mut libc::c_double,
    pub y_data: *mut libc::c_double,
    pub min_samples: libc::c_int,
    pub max_samples: libc::c_int,
    pub n_samples: libc::c_int,
    pub last_err: libc::c_double,
    pub min_separation: libc::c_double,
    pub valid_coefs: libc::c_int,
    pub offset: libc::c_double,
    pub frequency: libc::c_double,
}
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2016
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

  Header for tracking of hardware clocks */
pub type HCL_Instance = *mut HCL_Instance_Record;
pub type LCL_ChangeType = libc::c_uint;
pub const LCL_ChangeUnknownStep: LCL_ChangeType = 2;
pub const LCL_ChangeStep: LCL_ChangeType = 1;
pub const LCL_ChangeAdjust: LCL_ChangeType = 0;
pub type LCL_ParameterChangeHandler
    =
    Option<unsafe extern "C" fn(_: *mut timespec, _: *mut timespec,
                                _: libc::c_double, _: libc::c_double,
                                _: LCL_ChangeType, _: *mut libc::c_void)
               -> ()>;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
/* ================================================== */
unsafe extern "C" fn handle_slew(mut raw: *mut timespec,
                                 mut cooked: *mut timespec,
                                 mut dfreq: libc::c_double,
                                 mut doffset: libc::c_double,
                                 mut change_type: LCL_ChangeType,
                                 mut anything: *mut libc::c_void) {
    let mut clock: HCL_Instance = 0 as *mut HCL_Instance_Record;
    let mut delta: libc::c_double = 0.;
    clock = anything as HCL_Instance;
    if (*clock).n_samples != 0 {
        UTI_AdjustTimespec(&mut (*clock).local_ref, cooked,
                           &mut (*clock).local_ref, &mut delta, dfreq,
                           doffset);
    }
    if (*clock).valid_coefs != 0 { (*clock).frequency /= 1.0f64 - dfreq };
}
/* Create a new HW clock instance */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn HCL_CreateInstance(mut min_samples: libc::c_int,
                                            mut max_samples: libc::c_int,
                                            mut min_separation:
                                                libc::c_double)
 -> HCL_Instance {
    let mut clock: HCL_Instance = 0 as *mut HCL_Instance_Record;
    min_samples =
        if 2 as libc::c_int >
               (if min_samples < 64 as libc::c_int {
                    min_samples
                } else { 64 as libc::c_int }) {
            2 as libc::c_int
        } else if min_samples < 64 as libc::c_int {
            min_samples
        } else { 64 as libc::c_int };
    max_samples =
        if 2 as libc::c_int >
               (if max_samples < 64 as libc::c_int {
                    max_samples
                } else { 64 as libc::c_int }) {
            2 as libc::c_int
        } else if max_samples < 64 as libc::c_int {
            max_samples
        } else { 64 as libc::c_int };
    max_samples =
        if min_samples > max_samples { min_samples } else { max_samples };
    clock =
        Malloc(::std::mem::size_of::<HCL_Instance_Record>() as libc::c_ulong)
            as *mut HCL_Instance_Record;
    (*clock).x_data =
        Malloc2(max_samples as size_t,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong) as
            *mut libc::c_double;
    (*clock).y_data =
        Malloc2(max_samples as size_t,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong) as
            *mut libc::c_double;
    *(*clock).x_data.offset((max_samples - 1 as libc::c_int) as isize) =
        0.0f64;
    *(*clock).y_data.offset((max_samples - 1 as libc::c_int) as isize) =
        0.0f64;
    (*clock).min_samples = min_samples;
    (*clock).max_samples = max_samples;
    (*clock).n_samples = 0 as libc::c_int;
    (*clock).valid_coefs = 0 as libc::c_int;
    (*clock).min_separation = min_separation;
    LCL_AddParameterChangeHandler(Some(handle_slew as
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
                                  clock as *mut libc::c_void);
    return clock;
}
/* Destroy a HW clock instance */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn HCL_DestroyInstance(mut clock: HCL_Instance) {
    LCL_RemoveParameterChangeHandler(Some(handle_slew as
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
                                     clock as *mut libc::c_void);
    free((*clock).y_data as *mut libc::c_void);
    free((*clock).x_data as *mut libc::c_void);
    free(clock as *mut libc::c_void);
}
/* Check if a new sample should be accumulated at this time */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn HCL_NeedsNewSample(mut clock: HCL_Instance,
                                            mut now: *mut timespec)
 -> libc::c_int {
    if (*clock).n_samples == 0 ||
           fabs(UTI_DiffTimespecsToDouble(now, &mut (*clock).local_ref)) >=
               (*clock).min_separation {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* Accumulate a new sample */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn HCL_AccumulateSample(mut clock: HCL_Instance,
                                              mut hw_ts: *mut timespec,
                                              mut local_ts: *mut timespec,
                                              mut err: libc::c_double) {
    let mut hw_delta: libc::c_double = 0.;
    let mut local_delta: libc::c_double = 0.;
    let mut local_freq: libc::c_double = 0.;
    let mut raw_freq: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut n_runs: libc::c_int = 0;
    let mut best_start: libc::c_int = 0;
    local_freq = 1.0f64 - LCL_ReadAbsoluteFrequency() / 1.0e6f64;
    /* Shift old samples */
    if (*clock).n_samples != 0 {
        if (*clock).n_samples >= (*clock).max_samples {
            (*clock).n_samples -= 1
        }
        hw_delta = UTI_DiffTimespecsToDouble(hw_ts, &mut (*clock).hw_ref);
        local_delta =
            UTI_DiffTimespecsToDouble(local_ts, &mut (*clock).local_ref) /
                local_freq;
        if hw_delta <= 0.0f64 ||
               local_delta < (*clock).min_separation / 2.0f64 {
            (*clock).n_samples = 0 as libc::c_int;
            if 0 as libc::c_int != 0 &&
                   log_min_severity as libc::c_int ==
                       LOGS_DEBUG as libc::c_int {
                LOG_Message(LOGS_DEBUG,
                            b"HW clock reset interval=%f\x00" as *const u8 as
                                *const libc::c_char, local_delta);
            }
        }
        i = (*clock).max_samples - (*clock).n_samples;
        while i < (*clock).max_samples {
            *(*clock).y_data.offset((i - 1 as libc::c_int) as isize) =
                *(*clock).y_data.offset(i as isize) - hw_delta;
            *(*clock).x_data.offset((i - 1 as libc::c_int) as isize) =
                *(*clock).x_data.offset(i as isize) - local_delta;
            i += 1
        }
    }
    (*clock).n_samples += 1;
    (*clock).hw_ref = *hw_ts;
    (*clock).local_ref = *local_ts;
    (*clock).last_err = err;
    /* Get new coefficients */
    (*clock).valid_coefs =
        RGR_FindBestRobustRegression((*clock).x_data.offset((*clock).max_samples
                                                                as
                                                                isize).offset(-((*clock).n_samples
                                                                                    as
                                                                                    isize)),
                                     (*clock).y_data.offset((*clock).max_samples
                                                                as
                                                                isize).offset(-((*clock).n_samples
                                                                                    as
                                                                                    isize)),
                                     (*clock).n_samples, 1.0e-10f64,
                                     &mut (*clock).offset, &mut raw_freq,
                                     &mut n_runs, &mut best_start);
    if (*clock).valid_coefs == 0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"HW clock needs more samples\x00" as *const u8 as
                            *const libc::c_char);
        }
        return
    }
    (*clock).frequency = raw_freq / local_freq;
    /* Drop unneeded samples */
    if (*clock).n_samples > (*clock).min_samples {
        (*clock).n_samples -=
            if best_start < (*clock).n_samples - (*clock).min_samples {
                best_start
            } else { ((*clock).n_samples) - (*clock).min_samples }
    }
    /* If the fit doesn't cross the error interval of the last sample,
     or the frequency is not sane, drop all samples and start again */
    if fabs((*clock).offset) > err ||
           fabs((*clock).frequency - 1.0f64) > 2.0f64 / 3.0f64 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"HW clock reset\x00" as *const u8 as
                            *const libc::c_char);
        }
        (*clock).n_samples = 0 as libc::c_int;
        (*clock).valid_coefs = 0 as libc::c_int
    }
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"HW clock samples=%d offset=%e freq=%e raw_freq=%e err=%e ref_diff=%e\x00"
                        as *const u8 as *const libc::c_char,
                    (*clock).n_samples, (*clock).offset,
                    (*clock).frequency - 1.0f64, raw_freq - 1.0f64, err,
                    UTI_DiffTimespecsToDouble(&mut (*clock).hw_ref,
                                              &mut (*clock).local_ref));
    };
}
/* Convert raw hardware time to cooked local time */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn HCL_CookTime(mut clock: HCL_Instance,
                                      mut raw: *mut timespec,
                                      mut cooked: *mut timespec,
                                      mut err: *mut libc::c_double)
 -> libc::c_int {
    let mut offset: libc::c_double = 0.;
    let mut elapsed: libc::c_double = 0.;
    if (*clock).valid_coefs == 0 { return 0 as libc::c_int }
    elapsed = UTI_DiffTimespecsToDouble(raw, &mut (*clock).hw_ref);
    offset = elapsed / (*clock).frequency - (*clock).offset;
    UTI_AddDoubleToTimespec(&mut (*clock).local_ref, offset, cooked);
    /* Fow now, just return the error of the last sample */
    if !err.is_null() { *err = (*clock).last_err }
    return 1 as libc::c_int;
}
