#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn REF_GetSkew() -> libc::c_double;
    #[no_mangle]
    fn CNF_GetSmooth(max_freq_0: *mut libc::c_double,
                     max_wander_0: *mut libc::c_double,
                     leap_only: *mut libc::c_int);
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
    #[no_mangle]
    fn UTI_IsZeroTimespec(ts: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn UTI_ZeroTimespec(ts: *mut timespec);
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec)
     -> libc::c_double;
    #[no_mangle]
    fn UTI_AdjustTimespec(old_ts: *mut timespec, when: *mut timespec,
                          new_ts: *mut timespec,
                          delta_time: *mut libc::c_double,
                          dfreq: libc::c_double, doffset: libc::c_double);
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
pub struct RPT_SmoothingReport {
    pub active: libc::c_int,
    pub leap_only: libc::c_int,
    pub offset: libc::c_double,
    pub freq_ppm: libc::c_double,
    pub wander_ppm: libc::c_double,
    pub last_update_ago: libc::c_double,
    pub remaining_time: libc::c_double,
}
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

  Routines implementing time smoothing.

  */
/*
  Time smoothing determines an offset that needs to be applied to the cooked
  time to make it smooth for external observers.  Observed offset and frequency
  change slowly and there are no discontinuities.  This can be used on an NTP
  server to make it easier for the clients to track the time and keep their
  clocks close together even when large offset or frequency corrections are
  applied to the server's clock (e.g. after being offline for longer time).

  Accumulated offset and frequency are smoothed out in three stages.  In the
  first stage, the frequency is changed at a constant rate (wander) up to a
  maximum, in the second stage the frequency stays at the maximum for as long
  as needed and in the third stage the frequency is brought back to zero.

              |
    max_freq  +-------/--------\-------------
              |      /|        |\
        freq  |     / |        | \
              |    /  |        |  \
              |   /   |        |   \
           0  +--/----+--------+----\--------
              | /     |        |    |    time
              |/      |        |    |

        stage     1       2      3

  Integral of this function is the smoothed out offset.  It's a continuous
  piecewise polynomial with two quadratic parts and one linear.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stage {
    pub wander: libc::c_double,
    pub length: libc::c_double,
}
static mut stages: [stage; 3] = [stage{wander: 0., length: 0.,}; 3];
/* Enabled/disabled smoothing */
static mut enabled: libc::c_int = 0;
/* Enabled/disabled mode where only leap seconds are smoothed out and normal
   offset/frequency changes are ignored */
static mut leap_only_mode: libc::c_int = 0;
static mut locked: libc::c_int = 0;
/* Maximum wander and frequency offset */
static mut max_wander: libc::c_double = 0.;
static mut max_freq: libc::c_double = 0.;
/* Frequency offset, time offset and the time of the last smoothing update */
static mut smooth_freq: libc::c_double = 0.;
static mut smooth_offset: libc::c_double = 0.;
static mut last_update: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
unsafe extern "C" fn get_smoothing(mut now: *mut timespec,
                                   mut poffset: *mut libc::c_double,
                                   mut pfreq: *mut libc::c_double,
                                   mut pwander: *mut libc::c_double) {
    let mut elapsed: libc::c_double = 0.;
    let mut length: libc::c_double = 0.;
    let mut offset: libc::c_double = 0.;
    let mut freq: libc::c_double = 0.;
    let mut wander: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    elapsed = UTI_DiffTimespecsToDouble(now, &mut last_update);
    offset = smooth_offset;
    freq = smooth_freq;
    wander = 0.0f64;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if elapsed <= 0.0f64 { break ; }
        length = stages[i as usize].length;
        if length >= elapsed { length = elapsed }
        wander = stages[i as usize].wander;
        offset -= length * (2.0f64 * freq + wander * length) / 2.0f64;
        freq += wander * length;
        elapsed -= length;
        i += 1
    }
    if elapsed > 0.0f64 { wander = 0.0f64; offset -= elapsed * freq }
    *poffset = offset;
    *pfreq = freq;
    if !pwander.is_null() { *pwander = wander };
}
unsafe extern "C" fn update_stages() {
    let mut s1: libc::c_double = 0.;
    let mut s2: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut l1: libc::c_double = 0.;
    let mut l2: libc::c_double = 0.;
    let mut l3: libc::c_double = 0.;
    let mut lc: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    let mut f2: libc::c_double = 0.;
    let mut l1t: [libc::c_double; 2] = [0.; 2];
    let mut l3t: [libc::c_double; 2] = [0.; 2];
    let mut err: [libc::c_double; 2] = [0.; 2];
    let mut i: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    /* Prepare the three stages so that the integral of the frequency offset
     is equal to the offset that should be smoothed out */
    s1 = smooth_offset / max_wander;
    s2 = smooth_freq * smooth_freq / (2.0f64 * (max_wander * max_wander));
    /* Calculate the lengths of the 1st and 3rd stage assuming there is no
     frequency limit.  The direction of the 1st stage is selected so that
     the lengths will not be negative.  With extremely small offsets both
     directions may give a negative length due to numerical errors, so select
     the one which gives a smaller error. */
    i = 0 as libc::c_int;
    dir = -(1 as libc::c_int);
    while i <= 1 as libc::c_int {
        err[i as usize] = 0.0f64;
        s = dir as libc::c_double * s1 + s2;
        if s < 0.0f64 { err[i as usize] += -s; s = 0.0f64 }
        l3t[i as usize] = sqrt(s);
        l1t[i as usize] =
            l3t[i as usize] -
                dir as libc::c_double * smooth_freq / max_wander;
        if l1t[i as usize] < 0.0f64 {
            err[i as usize] += l1t[i as usize] * l1t[i as usize];
            l1t[i as usize] = 0.0f64
        }
        i += 1;
        dir += 2 as libc::c_int
    }
    if err[0 as libc::c_int as usize] < err[1 as libc::c_int as usize] {
        l1 = l1t[0 as libc::c_int as usize];
        l3 = l3t[0 as libc::c_int as usize];
        dir = -(1 as libc::c_int)
    } else {
        l1 = l1t[1 as libc::c_int as usize];
        l3 = l3t[1 as libc::c_int as usize];
        dir = 1 as libc::c_int
    }
    l2 = 0.0f64;
    /* If the limit was reached, shorten 1st+3rd stages and set a 2nd stage */
    f = dir as libc::c_double * smooth_freq + l1 * max_wander - max_freq;
    if f > 0.0f64 {
        lc = f / max_wander;
        /* No 1st stage if the frequency is already above the maximum */
        if lc > l1 {
            lc = l1;
            f2 = dir as libc::c_double * smooth_freq
        } else { f2 = max_freq }
        l2 = lc * (2.0f64 + f / f2);
        l1 -= lc;
        l3 -= lc
    }
    stages[0 as libc::c_int as usize].wander =
        dir as libc::c_double * max_wander;
    stages[0 as libc::c_int as usize].length = l1;
    stages[1 as libc::c_int as usize].wander = 0.0f64;
    stages[1 as libc::c_int as usize].length = l2;
    stages[2 as libc::c_int as usize].wander =
        -dir as libc::c_double * max_wander;
    stages[2 as libc::c_int as usize].length = l3;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Smooth stage %d wander %e length %f\x00" as
                            *const u8 as *const libc::c_char,
                        i + 1 as libc::c_int, stages[i as usize].wander,
                        stages[i as usize].length);
        }
        i += 1
    };
}
unsafe extern "C" fn update_smoothing(mut now: *mut timespec,
                                      mut offset: libc::c_double,
                                      mut freq: libc::c_double) {
    /* Don't accept offset/frequency until the clock has stabilized */
    if locked != 0 {
        if REF_GetSkew() / max_wander < 10000 as libc::c_int as libc::c_double
               || leap_only_mode != 0 {
            SMT_Activate(now);
        }
        return
    }
    get_smoothing(now, &mut smooth_offset, &mut smooth_freq,
                  0 as *mut libc::c_double);
    smooth_offset += offset;
    smooth_freq = (smooth_freq - freq) / (1.0f64 - freq);
    last_update = *now;
    update_stages();
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"Smooth offset %e freq %e\x00" as *const u8 as
                        *const libc::c_char, smooth_offset, smooth_freq);
    };
}
unsafe extern "C" fn handle_slew(mut raw: *mut timespec,
                                 mut cooked: *mut timespec,
                                 mut dfreq: libc::c_double,
                                 mut doffset: libc::c_double,
                                 mut change_type: LCL_ChangeType,
                                 mut anything: *mut libc::c_void) {
    let mut delta: libc::c_double = 0.;
    if change_type as libc::c_uint ==
           LCL_ChangeAdjust as libc::c_int as libc::c_uint {
        if leap_only_mode != 0 {
            update_smoothing(cooked, 0.0f64, 0.0f64);
        } else { update_smoothing(cooked, doffset, dfreq); }
    }
    if UTI_IsZeroTimespec(&mut last_update) == 0 {
        UTI_AdjustTimespec(&mut last_update, cooked, &mut last_update,
                           &mut delta, dfreq, doffset);
    };
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

  This module implements time smoothing.
  */
#[no_mangle]
pub unsafe extern "C" fn SMT_Initialise() {
    CNF_GetSmooth(&mut max_freq, &mut max_wander, &mut leap_only_mode);
    if max_freq <= 0.0f64 || max_wander <= 0.0f64 {
        enabled = 0 as libc::c_int;
        return
    }
    enabled = 1 as libc::c_int;
    locked = 1 as libc::c_int;
    /* Convert from ppm */
    max_freq *= 1e-6f64;
    max_wander *= 1e-6f64;
    UTI_ZeroTimespec(&mut last_update);
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
                                  0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn SMT_Finalise() { }
#[no_mangle]
pub unsafe extern "C" fn SMT_IsEnabled() -> libc::c_int { return enabled; }
#[no_mangle]
pub unsafe extern "C" fn SMT_GetOffset(mut now: *mut timespec)
 -> libc::c_double {
    let mut offset: libc::c_double = 0.;
    let mut freq: libc::c_double = 0.;
    if enabled == 0 { return 0.0f64 }
    get_smoothing(now, &mut offset, &mut freq, 0 as *mut libc::c_double);
    return offset;
}
#[no_mangle]
pub unsafe extern "C" fn SMT_Activate(mut now: *mut timespec) {
    if enabled == 0 || locked == 0 { return }
    LOG_Message(LOGS_INFO,
                b"Time smoothing activated%s\x00" as *const u8 as
                    *const libc::c_char,
                if leap_only_mode != 0 {
                    b" (leap seconds only)\x00" as *const u8 as
                        *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char });
    locked = 0 as libc::c_int;
    last_update = *now;
}
#[no_mangle]
pub unsafe extern "C" fn SMT_Reset(mut now: *mut timespec) {
    let mut i: libc::c_int = 0;
    if enabled == 0 { return }
    smooth_offset = 0.0f64;
    smooth_freq = 0.0f64;
    last_update = *now;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        stages[i as usize].length = 0.0f64;
        stages[i as usize].wander = stages[i as usize].length;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn SMT_Leap(mut now: *mut timespec,
                                  mut leap: libc::c_int) {
    /* When the leap-only mode is disabled, the leap second will be accumulated
     in handle_slew() as a normal offset */
    if enabled == 0 || leap_only_mode == 0 { return }
    update_smoothing(now, leap as libc::c_double, 0.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn SMT_GetSmoothingReport(mut report:
                                                    *mut RPT_SmoothingReport,
                                                mut now: *mut timespec)
 -> libc::c_int {
    let mut length: libc::c_double = 0.;
    let mut elapsed: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    if enabled == 0 { return 0 as libc::c_int }
    (*report).active = (locked == 0) as libc::c_int;
    (*report).leap_only = leap_only_mode;
    get_smoothing(now, &mut (*report).offset, &mut (*report).freq_ppm,
                  &mut (*report).wander_ppm);
    /* Convert to ppm and negate (positive values mean faster/speeding up) */
    (*report).freq_ppm *= -1.0e6f64;
    (*report).wander_ppm *= -1.0e6f64;
    elapsed = UTI_DiffTimespecsToDouble(now, &mut last_update);
    if locked == 0 && elapsed >= 0.0f64 {
        i = 0 as libc::c_int;
        length = 0.0f64;
        while i < 3 as libc::c_int {
            length += stages[i as usize].length;
            i += 1
        }
        (*report).last_update_ago = elapsed;
        (*report).remaining_time =
            if elapsed < length { (length) - elapsed } else { 0.0f64 }
    } else {
        (*report).last_update_ago = 0.0f64;
        (*report).remaining_time = 0.0f64
    }
    return 1 as libc::c_int;
}
