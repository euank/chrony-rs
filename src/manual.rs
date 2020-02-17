#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
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
    fn LCL_AddParameterChangeHandler(handler: LCL_ParameterChangeHandler,
                                     anything: *mut libc::c_void);
    /* Read the absolute system frequency, relative to the uncompensated
   system.  Returned in units of parts per million.  Thus the result of
   this is how many seconds fast the uncompensated system would be after
   its own time has reached 1 million seconds from the start of the
   measurement.  */
    #[no_mangle]
    fn LCL_ReadAbsoluteFrequency() -> libc::c_double;
    #[no_mangle]
    fn CNF_GetManualEnabled() -> libc::c_int;
    #[no_mangle]
    fn REF_SetManualReference(ref_time: *mut timespec, offset: libc::c_double,
                              frequency: libc::c_double,
                              skew: libc::c_double);
    #[no_mangle]
    fn UTI_AdjustTimespec(old_ts: *mut timespec, when: *mut timespec,
                          new_ts: *mut timespec,
                          delta_time: *mut libc::c_double,
                          dfreq: libc::c_double, doffset: libc::c_double);
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec)
     -> libc::c_double;
    #[no_mangle]
    fn UTI_IsTimeOffsetSane(ts: *mut timespec, offset: libc::c_double)
     -> libc::c_int;
    #[no_mangle]
    fn RGR_FindBestRobustRegression(x: *mut libc::c_double,
                                    y: *mut libc::c_double, n: libc::c_int,
                                    tol: libc::c_double,
                                    b0: *mut libc::c_double,
                                    b1: *mut libc::c_double,
                                    n_runs: *mut libc::c_int,
                                    best_start: *mut libc::c_int)
     -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_ManualSamplesReport {
    pub when: timespec,
    pub slewed_offset: libc::c_double,
    pub orig_offset: libc::c_double,
    pub residual: libc::c_double,
}
pub type LCL_ChangeType = libc::c_uint;
pub const LCL_ChangeUnknownStep: LCL_ChangeType = 2;
pub const LCL_ChangeStep: LCL_ChangeType = 1;
pub const LCL_ChangeAdjust: LCL_ChangeType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sample {
    pub when: timespec,
    pub orig_offset: libc::c_double,
    pub offset: libc::c_double,
    pub residual: libc::c_double,
}
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

  Routines for implementing manual input of real time.

  The daemon accepts manual time input over the control connection,
  and adjusts the system time to match.  Besides this, though, it can
  determine the average rate of time loss or gain of the local system
  and adjust the frequency accordingly.

  */
static mut enabled: libc::c_int = 0 as libc::c_int;
static mut samples: [Sample; 16] =
    [Sample{when: timespec{tv_sec: 0, tv_nsec: 0,},
            orig_offset: 0.,
            offset: 0.,
            residual: 0.,}; 16];
static mut n_samples: libc::c_int = 0;
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

  Header file for manual time input module.

  */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn MNL_Initialise() {
    if CNF_GetManualEnabled() != 0 {
        enabled = 1 as libc::c_int
    } else { enabled = 0 as libc::c_int }
    n_samples = 0 as libc::c_int;
    LCL_AddParameterChangeHandler(Some(slew_samples as
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
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn MNL_Finalise() { }
/* ================================================== */
unsafe extern "C" fn estimate_and_set_system(mut now: *mut timespec,
                                             mut offset_provided: libc::c_int,
                                             mut offset: libc::c_double,
                                             mut reg_offset:
                                                 *mut libc::c_double,
                                             mut dfreq_ppm:
                                                 *mut libc::c_double,
                                             mut new_afreq_ppm:
                                                 *mut libc::c_double) {
    let mut agos: [libc::c_double; 16] =
        [0.; 16]; /* Unused results from regression analyser */
    let mut offsets: [libc::c_double; 16] =
        [0.; 16]; /* All 9's when printed to log file */
    let mut b0: libc::c_double = 0.;
    let mut b1: libc::c_double = 0.;
    let mut n_runs: libc::c_int = 0;
    let mut best_start: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut freq: libc::c_double = 0.0f64;
    let mut skew: libc::c_double = 0.099999999f64;
    let mut found_freq: libc::c_int = 0;
    let mut slew_by: libc::c_double = 0.;
    b0 = if offset_provided != 0 { offset } else { 0.0f64 };
    freq = 0.0f64;
    b1 = freq;
    found_freq = 0 as libc::c_int;
    if n_samples > 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < n_samples {
            agos[i as usize] =
                UTI_DiffTimespecsToDouble(&mut (*samples.as_mut_ptr().offset((n_samples
                                                                                  -
                                                                                  1
                                                                                      as
                                                                                      libc::c_int)
                                                                                 as
                                                                                 isize)).when,
                                          &mut (*samples.as_mut_ptr().offset(i
                                                                                 as
                                                                                 isize)).when);
            offsets[i as usize] = samples[i as usize].offset;
            i += 1
        }
        if RGR_FindBestRobustRegression(agos.as_mut_ptr(),
                                        offsets.as_mut_ptr(), n_samples,
                                        1.0e-8f64, &mut b0, &mut b1,
                                        &mut n_runs, &mut best_start) != 0 {
            /* Ignore b0 from regression; treat offset as being the most
         recently entered value.  (If the administrator knows he's put
         an outlier in, he will rerun the settime operation.)   However,
         the frequency estimate comes from the regression. */
            freq = -b1;
            found_freq = 1 as libc::c_int
        }
    } else {
        agos[0 as libc::c_int as usize] = 0.0f64;
        offsets[0 as libc::c_int as usize] = b0
    }
    if offset_provided != 0 { slew_by = offset } else { slew_by = b0 }
    if found_freq != 0 {
        LOG_Message(LOGS_INFO,
                    b"Making a frequency change of %.3f ppm and a slew of %.6f\x00"
                        as *const u8 as *const libc::c_char, 1.0e6f64 * freq,
                    slew_by);
        REF_SetManualReference(now, slew_by, freq, skew);
    } else {
        LOG_Message(LOGS_INFO,
                    b"Making a slew of %.6f\x00" as *const u8 as
                        *const libc::c_char, slew_by);
        REF_SetManualReference(now, slew_by, 0.0f64, skew);
    }
    if !reg_offset.is_null() { *reg_offset = b0 }
    if !dfreq_ppm.is_null() { *dfreq_ppm = 1.0e6f64 * freq }
    if !new_afreq_ppm.is_null() {
        *new_afreq_ppm = LCL_ReadAbsoluteFrequency()
    }
    /* Calculate residuals to store them */
    i = 0 as libc::c_int;
    while i < n_samples {
        samples[i as usize].residual =
            offsets[i as usize] - (b0 + agos[i as usize] * b1);
        i += 1
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn MNL_AcceptTimestamp(mut ts: *mut timespec,
                                             mut reg_offset:
                                                 *mut libc::c_double,
                                             mut dfreq_ppm:
                                                 *mut libc::c_double,
                                             mut new_afreq_ppm:
                                                 *mut libc::c_double)
 -> libc::c_int {
    let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut offset: libc::c_double = 0.;
    let mut diff: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    if enabled != 0 {
        LCL_ReadCookedTime(&mut now, 0 as *mut libc::c_double);
        /* Make sure the provided timestamp is sane and the sample
       is not too close to the last one */
        if UTI_IsTimeOffsetSane(ts, 0.0f64) == 0 { return 0 as libc::c_int }
        if n_samples != 0 {
            diff =
                UTI_DiffTimespecsToDouble(&mut now,
                                          &mut (*samples.as_mut_ptr().offset((n_samples
                                                                                  -
                                                                                  1
                                                                                      as
                                                                                      libc::c_int)
                                                                                 as
                                                                                 isize)).when);
            if diff < 1.0f64 { return 0 as libc::c_int }
        }
        offset = UTI_DiffTimespecsToDouble(&mut now, ts);
        /* Check if buffer full up */
        if n_samples == 16 as libc::c_int {
            /* Shift samples down */
            i = 1 as libc::c_int;
            while i < n_samples {
                samples[(i - 1 as libc::c_int) as usize] =
                    samples[i as usize];
                i += 1
            }
            n_samples -= 1
        }
        samples[n_samples as usize].when = now;
        samples[n_samples as usize].offset = offset;
        samples[n_samples as usize].orig_offset = offset;
        n_samples += 1;
        estimate_and_set_system(&mut now, 1 as libc::c_int, offset,
                                reg_offset, dfreq_ppm, new_afreq_ppm);
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/* ================================================== */
/* ================================================== */
unsafe extern "C" fn slew_samples(mut raw: *mut timespec,
                                  mut cooked: *mut timespec,
                                  mut dfreq: libc::c_double,
                                  mut doffset: libc::c_double,
                                  mut change_type: LCL_ChangeType,
                                  mut not_used: *mut libc::c_void) {
    let mut delta_time: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    if change_type as libc::c_uint ==
           LCL_ChangeUnknownStep as libc::c_int as libc::c_uint {
        MNL_Reset();
    }
    i = 0 as libc::c_int;
    while i < n_samples {
        UTI_AdjustTimespec(&mut (*samples.as_mut_ptr().offset(i as
                                                                  isize)).when,
                           cooked,
                           &mut (*samples.as_mut_ptr().offset(i as
                                                                  isize)).when,
                           &mut delta_time, dfreq, doffset);
        samples[i as usize].offset += delta_time;
        i += 1
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn MNL_Enable() { enabled = 1 as libc::c_int; }
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn MNL_Disable() { enabled = 0 as libc::c_int; }
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn MNL_Reset() { n_samples = 0 as libc::c_int; }
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn MNL_IsEnabled() -> libc::c_int { return enabled; }
/* ================================================== */
/* Generate report data for the REQ_MANUAL_LIST command/monitoring
   protocol */
#[no_mangle]
pub unsafe extern "C" fn MNL_ReportSamples(mut report:
                                               *mut RPT_ManualSamplesReport,
                                           mut max: libc::c_int,
                                           mut n: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    if n_samples > max { *n = max } else { *n = n_samples }
    i = 0 as libc::c_int;
    while i < n_samples && i < max {
        (*report.offset(i as isize)).when = samples[i as usize].when;
        (*report.offset(i as isize)).slewed_offset =
            samples[i as usize].offset;
        (*report.offset(i as isize)).orig_offset =
            samples[i as usize].orig_offset;
        (*report.offset(i as isize)).residual = samples[i as usize].residual;
        i += 1
    };
}
/* ================================================== */
/* Delete a sample if it's within range, re-estimate the error and
   drift and apply it to the system clock. */
#[no_mangle]
pub unsafe extern "C" fn MNL_DeleteSample(mut index: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    if index < 0 as libc::c_int || index >= n_samples {
        return 0 as libc::c_int
    }
    /* Crunch the samples down onto the one being deleted */
    i = index;
    while i < n_samples - 1 as libc::c_int {
        samples[i as usize] = samples[(i + 1 as libc::c_int) as usize];
        i += 1
    }
    n_samples -= 1 as libc::c_int;
    /* Now re-estimate.  NULLs because we don't want the parameters back
     in this case. */
    LCL_ReadCookedTime(&mut now, 0 as *mut libc::c_double);
    estimate_and_set_system(&mut now, 0 as libc::c_int, 0.0f64,
                            0 as *mut libc::c_double,
                            0 as *mut libc::c_double,
                            0 as *mut libc::c_double);
    return 1 as libc::c_int;
}
/* ================================================== */
