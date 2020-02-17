#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, label_break_value, register_tool)]
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn CNF_GetMaxClockError() -> libc::c_double;
    #[no_mangle]
    fn CNF_GetMaxDrift() -> libc::c_double;
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
    fn SMT_Reset(now: *mut timespec);
    #[no_mangle]
    fn SMT_Leap(now: *mut timespec, leap: libc::c_int);
    #[no_mangle]
    fn UTI_AddDoubleToTimespec(start: *mut timespec,
                               increment: libc::c_double, end: *mut timespec);
    #[no_mangle]
    fn UTI_IsTimeOffsetSane(ts: *mut timespec, offset: libc::c_double)
     -> libc::c_int;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
}
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
/* System driver to convert a raw time to an adjusted (cooked) time.
   The number of seconds returned in 'corr' have to be added to the
   raw time to get the corrected time */
pub type lcl_OffsetCorrectionDriver
    =
    Option<unsafe extern "C" fn(_: *mut timespec, _: *mut libc::c_double,
                                _: *mut libc::c_double) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ChangeListEntry {
    pub next: *mut _ChangeListEntry,
    pub prev: *mut _ChangeListEntry,
    pub handler: LCL_ParameterChangeHandler,
    pub anything: *mut libc::c_void,
}
/* ================================================== */
/* Types and variables associated with handling the parameter change
   list */
pub type ChangeListEntry = _ChangeListEntry;
/* ================================================== */
/* Types and variables associated with handling the parameter change
   list */
pub type DispersionNotifyListEntry = _DispersionNotifyListEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _DispersionNotifyListEntry {
    pub next: *mut _DispersionNotifyListEntry,
    pub prev: *mut _DispersionNotifyListEntry,
    pub handler: LCL_DispersionNotifyHandler,
    pub anything: *mut libc::c_void,
}
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
pub type LCL_DispersionNotifyHandler
    =
    Option<unsafe extern "C" fn(_: libc::c_double, _: *mut libc::c_void)
               -> ()>;
/* System driver to accrue an offset. A positive argument means slew
   the clock forwards.  The suggested correction rate of time to correct the
   offset is given in 'corr_rate'. */
pub type lcl_AccrueOffsetDriver
    =
    Option<unsafe extern "C" fn(_: libc::c_double, _: libc::c_double) -> ()>;
/* System driver to set the current local frequency, in ppm relative
   to nominal.  A positive value indicates that the local clock runs
   fast when uncompensated.  Return actual frequency (may be different
   from the requested frequency due to clamping or rounding). */
pub type lcl_SetFrequencyDriver
    =
    Option<unsafe extern "C" fn(_: libc::c_double) -> libc::c_double>;
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
/* System driver to apply a step offset. A positive argument means step
   the clock forwards. */
pub type lcl_ApplyStepOffsetDriver
    =
    Option<unsafe extern "C" fn(_: libc::c_double) -> libc::c_int>;
/* System driver to schedule leap seconds and set TAI-UTC offset */
pub type lcl_SetLeapDriver
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> ()>;
/* System driver to set the synchronisation status */
pub type lcl_SetSyncStatusDriver
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_double,
                                _: libc::c_double) -> ()>;
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2011, 2014-2015
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

  The routines in this file present a common local (system) clock
  interface to the rest of the software.

  They interface with the system specific driver files in sys_*.c
  */
/* ================================================== */
/* Variable to store the current frequency, in ppm */
static mut current_freq_ppm: libc::c_double = 0.;
/* Maximum allowed frequency, in ppm */
static mut max_freq_ppm: libc::c_double = 0.;
/* Temperature compensation, in ppm */
static mut temp_comp_ppm: libc::c_double = 0.;
/* ================================================== */
/* Store the system dependent drivers */
static mut drv_read_freq: lcl_ReadFrequencyDriver = None;
static mut drv_set_freq: lcl_SetFrequencyDriver = None;
static mut drv_accrue_offset: lcl_AccrueOffsetDriver = None;
static mut drv_apply_step_offset: lcl_ApplyStepOffsetDriver = None;
static mut drv_offset_convert: lcl_OffsetCorrectionDriver = None;
static mut drv_set_leap: lcl_SetLeapDriver = None;
static mut drv_set_sync_status: lcl_SetSyncStatusDriver = None;
static mut change_list: ChangeListEntry =
    ChangeListEntry{next:
                        0 as *const _ChangeListEntry as *mut _ChangeListEntry,
                    prev:
                        0 as *const _ChangeListEntry as *mut _ChangeListEntry,
                    handler: None,
                    anything: 0 as *const libc::c_void as *mut libc::c_void,};
static mut dispersion_notify_list: DispersionNotifyListEntry =
    DispersionNotifyListEntry{next:
                                  0 as *const _DispersionNotifyListEntry as
                                      *mut _DispersionNotifyListEntry,
                              prev:
                                  0 as *const _DispersionNotifyListEntry as
                                      *mut _DispersionNotifyListEntry,
                              handler: None,
                              anything:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,};
/* ================================================== */
static mut precision_log: libc::c_int = 0;
static mut precision_quantum: libc::c_double = 0.;
static mut max_clock_error: libc::c_double = 0.;
unsafe extern "C" fn calculate_sys_precision() {
    let mut ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut old_ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut iters: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    LCL_ReadRawTime(&mut old_ts);
    /* Assume we must be better than a second */
    best = 1000000000 as libc::c_int;
    iters = 0 as libc::c_int;
    loop  {
        LCL_ReadRawTime(&mut ts);
        diff =
            (1000000000 as libc::c_int as libc::c_long *
                 (ts.tv_sec - old_ts.tv_sec) + (ts.tv_nsec - old_ts.tv_nsec))
                as libc::c_int;
        old_ts = ts;
        if diff > 0 as libc::c_int {
            if diff < best { best = diff }
            iters += 1
        }
        if !(iters < 100 as libc::c_int) { break ; }
    }
    if best > 0 as libc::c_int {
    } else {
        __assert_fail(b"best > 0\x00" as *const u8 as *const libc::c_char,
                      b"local.c\x00" as *const u8 as *const libc::c_char,
                      136 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void calculate_sys_precision(void)\x00")).as_ptr());
    }
    precision_quantum = 1.0e-9f64 * best as libc::c_double;
    /* Get rounded log2 value of the measured precision */
    precision_log = 0 as libc::c_int;
    while best < 707106781 as libc::c_int {
        precision_log -= 1;
        best *= 2 as libc::c_int
    }
    if precision_log >= -(30 as libc::c_int) {
    } else {
        __assert_fail(b"precision_log >= -30\x00" as *const u8 as
                          *const libc::c_char,
                      b"local.c\x00" as *const u8 as *const libc::c_char,
                      147 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void calculate_sys_precision(void)\x00")).as_ptr());
    }
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"Clock precision %.9f (%d)\x00" as *const u8 as
                        *const libc::c_char, precision_quantum,
                    precision_log);
    };
}
/* Routine to initialise the module (to be called once at program
   start-up) */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_Initialise() {
    change_list.prev = &mut change_list;
    change_list.next = change_list.prev;
    dispersion_notify_list.prev = &mut dispersion_notify_list;
    dispersion_notify_list.next = dispersion_notify_list.prev;
    /* Null out the system drivers, so that we die
     if they never get defined before use */
    drv_read_freq = None;
    drv_set_freq = None;
    drv_accrue_offset = None;
    drv_offset_convert = None;
    /* This ought to be set from the system driver layer */
    current_freq_ppm = 0.0f64;
    temp_comp_ppm = 0.0f64;
    calculate_sys_precision();
    /* This is the maximum allowed frequency offset in ppm, the time must
     never stop or run backwards */
    max_freq_ppm = CNF_GetMaxDrift();
    max_freq_ppm =
        if 0.0f64 >
               (if max_freq_ppm < 500000.0f64 {
                    max_freq_ppm
                } else { 500000.0f64 }) {
            0.0f64
        } else if max_freq_ppm < 500000.0f64 {
            max_freq_ppm
        } else { 500000.0f64 };
    max_clock_error = CNF_GetMaxClockError() * 1e-6f64;
}
/* Routine to finalise the module (to be called once at end of
   run). */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_Finalise() {
    while change_list.next != &mut change_list as *mut ChangeListEntry {
        LCL_RemoveParameterChangeHandler((*change_list.next).handler,
                                         (*change_list.next).anything);
    }
    while dispersion_notify_list.next !=
              &mut dispersion_notify_list as *mut DispersionNotifyListEntry {
        LCL_RemoveDispersionNotifyHandler((*dispersion_notify_list.next).handler,
                                          (*dispersion_notify_list.next).anything);
    };
}
/* Routine to read the system precision as a log to base 2 value. */
/* ================================================== */
/* Routine to read the system precision as a log to base 2 value. */
#[no_mangle]
pub unsafe extern "C" fn LCL_GetSysPrecisionAsLog() -> libc::c_int {
    return precision_log;
}
/* Routine to read the system precision in terms of the actual time step */
/* ================================================== */
/* Routine to read the system precision in terms of the actual time step */
#[no_mangle]
pub unsafe extern "C" fn LCL_GetSysPrecisionAsQuantum() -> libc::c_double {
    return precision_quantum;
}
/* Routine to read the maximum frequency error of the local clock.  This
   is a frequency stability, not an absolute error. */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_GetMaxClockError() -> libc::c_double {
    return max_clock_error;
}
/* Add a handler.  Then handler MUST NOT deregister itself!!! */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_AddParameterChangeHandler(mut handler:
                                                           LCL_ParameterChangeHandler,
                                                       mut anything:
                                                           *mut libc::c_void) {
    let mut ptr: *mut ChangeListEntry = 0 as *mut ChangeListEntry;
    let mut new_entry: *mut ChangeListEntry = 0 as *mut ChangeListEntry;
    /* Check that the handler is not already registered */
    ptr = change_list.next;
    while ptr != &mut change_list as *mut ChangeListEntry {
        if !((*ptr).handler != handler || (*ptr).anything != anything) {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"local.c\x00" as *const u8 as *const libc::c_char,
                          233 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 71],
                                                    &[libc::c_char; 71]>(b"void LCL_AddParameterChangeHandler(LCL_ParameterChangeHandler, void *)\x00")).as_ptr());
        }
        ptr = (*ptr).next
    }
    new_entry =
        Malloc(::std::mem::size_of::<ChangeListEntry>() as libc::c_ulong) as
            *mut ChangeListEntry;
    (*new_entry).handler = handler;
    (*new_entry).anything = anything;
    /* Chain it into the list */
    (*new_entry).next = &mut change_list;
    (*new_entry).prev = change_list.prev;
    (*change_list.prev).next = new_entry;
    change_list.prev = new_entry;
}
/* Remove a handler */
/* ================================================== */
/* Remove a handler */
#[no_mangle]
pub unsafe extern "C" fn LCL_RemoveParameterChangeHandler(mut handler:
                                                              LCL_ParameterChangeHandler,
                                                          mut anything:
                                                              *mut libc::c_void) {
    let mut ptr: *mut ChangeListEntry = 0 as *mut ChangeListEntry;
    let mut ok: libc::c_int = 0;
    ptr = 0 as *mut ChangeListEntry;
    ok = 0 as libc::c_int;
    ptr = change_list.next;
    while ptr != &mut change_list as *mut ChangeListEntry {
        if (*ptr).handler == handler && (*ptr).anything == anything {
            ok = 1 as libc::c_int;
            break ;
        } else { ptr = (*ptr).next }
    }
    if ok != 0 {
    } else {
        __assert_fail(b"ok\x00" as *const u8 as *const libc::c_char,
                      b"local.c\x00" as *const u8 as *const libc::c_char,
                      268 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 74],
                                                &[libc::c_char; 74]>(b"void LCL_RemoveParameterChangeHandler(LCL_ParameterChangeHandler, void *)\x00")).as_ptr());
    }
    /* Unlink entry from the list */
    (*(*ptr).next).prev = (*ptr).prev;
    (*(*ptr).prev).next = (*ptr).next;
    free(ptr as *mut libc::c_void);
}
/* Check if a handler is invoked first when dispatching */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_IsFirstParameterChangeHandler(mut handler:
                                                               LCL_ParameterChangeHandler)
 -> libc::c_int {
    return ((*change_list.next).handler == handler) as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn invoke_parameter_change_handlers(mut raw: *mut timespec,
                                                      mut cooked:
                                                          *mut timespec,
                                                      mut dfreq:
                                                          libc::c_double,
                                                      mut doffset:
                                                          libc::c_double,
                                                      mut change_type:
                                                          LCL_ChangeType) {
    let mut ptr: *mut ChangeListEntry = 0 as *mut ChangeListEntry;
    ptr = change_list.next;
    while ptr != &mut change_list as *mut ChangeListEntry {
        (*ptr).handler.expect("non-null function pointer")(raw, cooked, dfreq,
                                                           doffset,
                                                           change_type,
                                                           (*ptr).anything);
        ptr = (*ptr).next
    };
}
/* Register a handler for being notified of dispersion being added to
   the local clock.  The handler MUST NOT unregister itself!!! */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_AddDispersionNotifyHandler(mut handler:
                                                            LCL_DispersionNotifyHandler,
                                                        mut anything:
                                                            *mut libc::c_void) {
    let mut ptr: *mut DispersionNotifyListEntry =
        0 as *mut DispersionNotifyListEntry;
    let mut new_entry: *mut DispersionNotifyListEntry =
        0 as *mut DispersionNotifyListEntry;
    /* Check that the handler is not already registered */
    ptr = dispersion_notify_list.next;
    while ptr != &mut dispersion_notify_list as *mut DispersionNotifyListEntry
          {
        if !((*ptr).handler != handler || (*ptr).anything != anything) {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"local.c\x00" as *const u8 as *const libc::c_char,
                          309 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 73],
                                                    &[libc::c_char; 73]>(b"void LCL_AddDispersionNotifyHandler(LCL_DispersionNotifyHandler, void *)\x00")).as_ptr());
        }
        ptr = (*ptr).next
    }
    new_entry =
        Malloc(::std::mem::size_of::<DispersionNotifyListEntry>() as
                   libc::c_ulong) as *mut DispersionNotifyListEntry;
    (*new_entry).handler = handler;
    (*new_entry).anything = anything;
    /* Chain it into the list */
    (*new_entry).next = &mut dispersion_notify_list;
    (*new_entry).prev = dispersion_notify_list.prev;
    (*dispersion_notify_list.prev).next = new_entry;
    dispersion_notify_list.prev = new_entry;
}
/* Delete a handler */
/* ================================================== */
/* Remove a handler */
#[no_mangle]
pub unsafe extern "C" fn LCL_RemoveDispersionNotifyHandler(mut handler:
                                                               LCL_DispersionNotifyHandler,
                                                           mut anything:
                                                               *mut libc::c_void) {
    let mut ptr: *mut DispersionNotifyListEntry =
        0 as *mut DispersionNotifyListEntry;
    let mut ok: libc::c_int = 0;
    ptr = 0 as *mut DispersionNotifyListEntry;
    ok = 0 as libc::c_int;
    ptr = dispersion_notify_list.next;
    while ptr != &mut dispersion_notify_list as *mut DispersionNotifyListEntry
          {
        if (*ptr).handler == handler && (*ptr).anything == anything {
            ok = 1 as libc::c_int;
            break ;
        } else { ptr = (*ptr).next }
    }
    if ok != 0 {
    } else {
        __assert_fail(b"ok\x00" as *const u8 as *const libc::c_char,
                      b"local.c\x00" as *const u8 as *const libc::c_char,
                      345 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 76],
                                                &[libc::c_char; 76]>(b"void LCL_RemoveDispersionNotifyHandler(LCL_DispersionNotifyHandler, void *)\x00")).as_ptr());
    }
    /* Unlink entry from the list */
    (*(*ptr).next).prev = (*ptr).prev;
    (*(*ptr).prev).next = (*ptr).next;
    free(ptr as *mut libc::c_void);
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

  This module provides an interface to the system time, and
  insulates the rest of the program from the different way
  that interface has to be done on various operating systems.
  */
/* Read the system clock */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_ReadRawTime(mut ts: *mut timespec) {
    if clock_gettime(0 as libc::c_int, ts) < 0 as libc::c_int {
        LOG_Message(LOGS_FATAL,
                    b"clock_gettime() failed : %s\x00" as *const u8 as
                        *const libc::c_char, strerror(*__errno_location()));
        exit(1 as libc::c_int);
    };
}
/* Read the system clock, corrected according to all accumulated
   drifts and uncompensated offsets.

   In a kernel implementation with vernier frequency control (like
   Linux), and if we were to apply offsets by stepping the clock, this
   would be identical to raw time.  In any other case (use of
   adjtime()-like interface to correct offsets, and to adjust the
   frequency), we must correct the raw time to get this value */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_ReadCookedTime(mut result: *mut timespec,
                                            mut err: *mut libc::c_double) {
    let mut raw: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    LCL_ReadRawTime(&mut raw);
    LCL_CookTime(&mut raw, result, err);
}
/* Convert raw time to cooked. */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_CookTime(mut raw: *mut timespec,
                                      mut cooked: *mut timespec,
                                      mut err: *mut libc::c_double) {
    let mut correction: libc::c_double = 0.;
    LCL_GetOffsetCorrection(raw, &mut correction, err);
    UTI_AddDoubleToTimespec(raw, correction, cooked);
}
/* Read the current offset between the system clock and true time
   (i.e. 'cooked' - 'raw') (in seconds). */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_GetOffsetCorrection(mut raw: *mut timespec,
                                                 mut correction:
                                                     *mut libc::c_double,
                                                 mut err:
                                                     *mut libc::c_double) {
    /* Call system specific driver to get correction */
    Some(drv_offset_convert.expect("non-null function pointer")).expect("non-null function pointer")(raw,
                                                                                                     correction,
                                                                                                     err);
}
/* Read the absolute system frequency, relative to the uncompensated
   system.  Returned in units of parts per million.  Thus the result of
   this is how many seconds fast the uncompensated system would be after
   its own time has reached 1 million seconds from the start of the
   measurement.  */
/* ================================================== */
/* Return current frequency */
#[no_mangle]
pub unsafe extern "C" fn LCL_ReadAbsoluteFrequency() -> libc::c_double {
    let mut freq: libc::c_double = 0.;
    freq = current_freq_ppm;
    /* Undo temperature compensation */
    if temp_comp_ppm != 0.0f64 {
        freq = (freq + temp_comp_ppm) / (1.0f64 - 1.0e-6f64 * temp_comp_ppm)
    }
    return freq;
}
/* ================================================== */
unsafe extern "C" fn clamp_freq(mut freq: libc::c_double) -> libc::c_double {
    if freq <= max_freq_ppm && freq >= -max_freq_ppm { return freq }
    LOG_Message(LOGS_WARN,
                b"Frequency %.1f ppm exceeds allowed maximum\x00" as *const u8
                    as *const libc::c_char, freq);
    return if -max_freq_ppm >
                  (if freq < max_freq_ppm { freq } else { max_freq_ppm }) {
               -max_freq_ppm
           } else if freq < max_freq_ppm { freq } else { max_freq_ppm };
}
/* ================================================== */
unsafe extern "C" fn check_offset(mut now: *mut timespec,
                                  mut offset: libc::c_double) -> libc::c_int {
    /* Check if the time will be still sane with accumulated offset */
    if UTI_IsTimeOffsetSane(now, -offset) != 0 { return 1 as libc::c_int }
    LOG_Message(LOGS_WARN,
                b"Adjustment of %.1f seconds is invalid\x00" as *const u8 as
                    *const libc::c_char, -offset);
    return 0 as libc::c_int;
}
/* Routine to set the absolute frequency.  Only expected to be used
   when either (i) reading the drift from a file at the start of a
   run, or (ii) responsing to a user parameter 'poke'.  This is
   defined in ppm, as for the absolute frequency reading routine. */
/* ================================================== */
/* This involves both setting the absolute frequency with the
   system-specific driver, as well as calling all notify handlers */
#[no_mangle]
pub unsafe extern "C" fn LCL_SetAbsoluteFrequency(mut afreq_ppm:
                                                      libc::c_double) {
    let mut raw: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut cooked: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut dfreq: libc::c_double = 0.;
    afreq_ppm = clamp_freq(afreq_ppm);
    /* Apply temperature compensation */
    if temp_comp_ppm != 0.0f64 {
        afreq_ppm =
            afreq_ppm * (1.0f64 - 1.0e-6f64 * temp_comp_ppm) - temp_comp_ppm
    }
    /* Call the system-specific driver for setting the frequency */
    afreq_ppm =
        Some(drv_set_freq.expect("non-null function pointer")).expect("non-null function pointer")(afreq_ppm);
    dfreq = (afreq_ppm - current_freq_ppm) / (1.0e6f64 - current_freq_ppm);
    LCL_ReadRawTime(&mut raw);
    LCL_CookTime(&mut raw, &mut cooked, 0 as *mut libc::c_double);
    /* Dispatch to all handlers */
    invoke_parameter_change_handlers(&mut raw, &mut cooked, dfreq, 0.0f64,
                                     LCL_ChangeAdjust);
    current_freq_ppm = afreq_ppm;
}
/* Routine to apply a change of frequency to the local clock.  The
   argument is the estimated gain (positive) or loss (negative) of the
   local clock relative to true time, per unit time of the PREVIOUS
   frequency setting of the local clock.  This is assumed to be based
   on a regression of y=offset v x=cooked local time. */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_AccumulateDeltaFrequency(mut dfreq:
                                                          libc::c_double) {
    let mut raw: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut cooked: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut old_freq_ppm: libc::c_double = 0.;
    old_freq_ppm = current_freq_ppm;
    /* Work out new absolute frequency.  Note that absolute frequencies
   are handled in units of ppm, whereas the 'dfreq' argument is in
   terms of the gradient of the (offset) v (local time) function. */
    current_freq_ppm += dfreq * (1.0e6f64 - current_freq_ppm);
    current_freq_ppm = clamp_freq(current_freq_ppm);
    /* Call the system-specific driver for setting the frequency */
    current_freq_ppm =
        Some(drv_set_freq.expect("non-null function pointer")).expect("non-null function pointer")(current_freq_ppm);
    dfreq = (current_freq_ppm - old_freq_ppm) / (1.0e6f64 - old_freq_ppm);
    LCL_ReadRawTime(&mut raw);
    LCL_CookTime(&mut raw, &mut cooked, 0 as *mut libc::c_double);
    /* Dispatch to all handlers */
    invoke_parameter_change_handlers(&mut raw, &mut cooked, dfreq, 0.0f64,
                                     LCL_ChangeAdjust);
}
/* Routine to apply an offset (in seconds) to the local clock.  The
   argument should be positive to move the clock backwards (i.e. the
   local clock is currently fast of true time), or negative to move it
   forwards (i.e. it is currently slow of true time).  Provided is also
   a suggested correction rate (correction time * offset). */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_AccumulateOffset(mut offset: libc::c_double,
                                              mut corr_rate: libc::c_double) {
    let mut raw: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut cooked: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    /* In this case, the cooked time to be passed to the notify clients
     has to be the cooked time BEFORE the change was made */
    LCL_ReadRawTime(&mut raw);
    LCL_CookTime(&mut raw, &mut cooked, 0 as *mut libc::c_double);
    if check_offset(&mut cooked, offset) == 0 { return }
    Some(drv_accrue_offset.expect("non-null function pointer")).expect("non-null function pointer")(offset,
                                                                                                    corr_rate);
    /* Dispatch to all handlers */
    invoke_parameter_change_handlers(&mut raw, &mut cooked, 0.0f64, offset,
                                     LCL_ChangeAdjust);
}
/* Routine to apply an immediate offset by doing a sudden step if
   possible. (Intended for use after an initial estimate of offset has
   been obtained, so that we don't end up using adjtime to achieve a
   slew of an hour or something like that). A positive argument means
   the system clock is fast on true time, i.e. it needs to be stepped
   backwards. (Same convention as for AccumulateOffset routine). */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_ApplyStepOffset(mut offset: libc::c_double)
 -> libc::c_int {
    let mut raw: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut cooked: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    /* In this case, the cooked time to be passed to the notify clients
     has to be the cooked time BEFORE the change was made */
    LCL_ReadRawTime(&mut raw);
    LCL_CookTime(&mut raw, &mut cooked, 0 as *mut libc::c_double);
    if check_offset(&mut raw, offset) == 0 { return 0 as libc::c_int }
    if Some(drv_apply_step_offset.expect("non-null function pointer")).expect("non-null function pointer")(offset)
           == 0 {
        LOG_Message(LOGS_ERR,
                    b"Could not step system clock\x00" as *const u8 as
                        *const libc::c_char);
        return 0 as libc::c_int
    }
    /* Reset smoothing on all clock steps */
    SMT_Reset(&mut cooked);
    /* Dispatch to all handlers */
    invoke_parameter_change_handlers(&mut raw, &mut cooked, 0.0f64, offset,
                                     LCL_ChangeStep);
    return 1 as libc::c_int;
}
/* Routine to invoke notify handlers on an unexpected time jump
   in system clock */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_NotifyExternalTimeStep(mut raw: *mut timespec,
                                                    mut cooked: *mut timespec,
                                                    mut offset:
                                                        libc::c_double,
                                                    mut dispersion:
                                                        libc::c_double) {
    /* Dispatch to all handlers */
    invoke_parameter_change_handlers(raw, cooked, 0.0f64, offset,
                                     LCL_ChangeUnknownStep);
    lcl_InvokeDispersionNotifyHandlers(dispersion);
}
/* Routine to invoke notify handlers on leap second when the system clock
   doesn't correct itself */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_NotifyLeap(mut leap: libc::c_int) {
    let mut raw: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut cooked: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    LCL_ReadRawTime(&mut raw);
    LCL_CookTime(&mut raw, &mut cooked, 0 as *mut libc::c_double);
    /* Smooth the leap second out */
    SMT_Leap(&mut cooked, leap);
    /* Dispatch to all handlers as if the clock was stepped */
    invoke_parameter_change_handlers(&mut raw, &mut cooked, 0.0f64,
                                     -leap as libc::c_double, LCL_ChangeStep);
}
/* Perform the combination of modifying the frequency and applying
   a slew, in one easy step */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_AccumulateFrequencyAndOffset(mut dfreq:
                                                              libc::c_double,
                                                          mut doffset:
                                                              libc::c_double,
                                                          mut corr_rate:
                                                              libc::c_double) {
    let mut raw: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut cooked: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut old_freq_ppm: libc::c_double = 0.;
    LCL_ReadRawTime(&mut raw);
    /* Due to modifying the offset, this has to be the cooked time prior
     to the change we are about to make */
    LCL_CookTime(&mut raw, &mut cooked, 0 as *mut libc::c_double);
    if check_offset(&mut cooked, doffset) == 0 { return }
    old_freq_ppm = current_freq_ppm;
    /* Work out new absolute frequency.  Note that absolute frequencies
   are handled in units of ppm, whereas the 'dfreq' argument is in
   terms of the gradient of the (offset) v (local time) function. */
    current_freq_ppm += dfreq * (1.0e6f64 - current_freq_ppm);
    current_freq_ppm = clamp_freq(current_freq_ppm);
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"old_freq=%.3fppm new_freq=%.3fppm offset=%.6fsec\x00" as
                        *const u8 as *const libc::c_char, old_freq_ppm,
                    current_freq_ppm, doffset);
    }
    /* Call the system-specific driver for setting the frequency */
    current_freq_ppm =
        Some(drv_set_freq.expect("non-null function pointer")).expect("non-null function pointer")(current_freq_ppm);
    dfreq = (current_freq_ppm - old_freq_ppm) / (1.0e6f64 - old_freq_ppm);
    Some(drv_accrue_offset.expect("non-null function pointer")).expect("non-null function pointer")(doffset,
                                                                                                    corr_rate);
    /* Dispatch to all handlers */
    invoke_parameter_change_handlers(&mut raw, &mut cooked, dfreq, doffset,
                                     LCL_ChangeAdjust);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn lcl_InvokeDispersionNotifyHandlers(mut dispersion:
                                                                libc::c_double) {
    let mut ptr: *mut DispersionNotifyListEntry =
        0 as *mut DispersionNotifyListEntry;
    ptr = dispersion_notify_list.next;
    while ptr != &mut dispersion_notify_list as *mut DispersionNotifyListEntry
          {
        (*ptr).handler.expect("non-null function pointer")(dispersion,
                                                           (*ptr).anything);
        ptr = (*ptr).next
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn lcl_RegisterSystemDrivers(mut read_freq:
                                                       lcl_ReadFrequencyDriver,
                                                   mut set_freq:
                                                       lcl_SetFrequencyDriver,
                                                   mut accrue_offset:
                                                       lcl_AccrueOffsetDriver,
                                                   mut apply_step_offset:
                                                       lcl_ApplyStepOffsetDriver,
                                                   mut offset_convert:
                                                       lcl_OffsetCorrectionDriver,
                                                   mut set_leap:
                                                       lcl_SetLeapDriver,
                                                   mut set_sync_status:
                                                       lcl_SetSyncStatusDriver) {
    drv_read_freq = read_freq;
    drv_set_freq = set_freq;
    drv_accrue_offset = accrue_offset;
    drv_apply_step_offset = apply_step_offset;
    drv_offset_convert = offset_convert;
    drv_set_leap = set_leap;
    drv_set_sync_status = set_sync_status;
    current_freq_ppm =
        Some(drv_read_freq.expect("non-null function pointer")).expect("non-null function pointer")();
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"Local freq=%.3fppm\x00" as *const u8 as
                        *const libc::c_char, current_freq_ppm);
    };
}
/* Routine to convert the outstanding system clock error to a step and
   apply it, e.g. if the system clock has ended up an hour wrong due
   to a timezone problem. */
/* ================================================== */
/* Look at the current difference between the system time and the NTP
   time, and make a step to cancel it. */
#[no_mangle]
pub unsafe extern "C" fn LCL_MakeStep() -> libc::c_int {
    let mut raw: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut correction: libc::c_double = 0.;
    LCL_ReadRawTime(&mut raw);
    LCL_GetOffsetCorrection(&mut raw, &mut correction,
                            0 as *mut libc::c_double);
    if check_offset(&mut raw, -correction) == 0 { return 0 as libc::c_int }
    /* Cancel remaining slew and make the step */
    LCL_AccumulateOffset(correction, 0.0f64);
    if LCL_ApplyStepOffset(-correction) == 0 { return 0 as libc::c_int }
    LOG_Message(LOGS_WARN,
                b"System clock was stepped by %.6f seconds\x00" as *const u8
                    as *const libc::c_char, correction);
    return 1 as libc::c_int;
}
/* Check if the system driver supports leap seconds, i.e. LCL_SetSystemLeap
   does something */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_CanSystemLeap() -> libc::c_int {
    return if drv_set_leap.is_some() {
               1 as libc::c_int
           } else { 0 as libc::c_int };
}
/* Routine to set the system clock to correct itself for a leap second and also
   set its TAI-UTC offset.  If supported, leap second will be inserted at the
   end of the day if the argument is positive, deleted if negative, and zero
   resets the setting. */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_SetSystemLeap(mut leap: libc::c_int,
                                           mut tai_offset: libc::c_int) {
    if drv_set_leap.is_some() {
        drv_set_leap.expect("non-null function pointer")(leap, tai_offset);
    };
}
/* Routine to set a frequency correction (in ppm) that should be applied
   to local clock to compensate for temperature changes.  A positive
   argument means that the clock frequency should be increased. Return the
   actual compensation (may be different from the requested compensation
   due to clamping or rounding). */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_SetTempComp(mut comp: libc::c_double)
 -> libc::c_double {
    let mut uncomp_freq_ppm: libc::c_double = 0.;
    if temp_comp_ppm == comp { return comp }
    /* Undo previous compensation */
    current_freq_ppm =
        (current_freq_ppm + temp_comp_ppm) /
            (1.0f64 - 1.0e-6f64 * temp_comp_ppm);
    uncomp_freq_ppm = current_freq_ppm;
    /* Apply new compensation */
    current_freq_ppm = current_freq_ppm * (1.0f64 - 1.0e-6f64 * comp) - comp;
    /* Call the system-specific driver for setting the frequency */
    current_freq_ppm =
        Some(drv_set_freq.expect("non-null function pointer")).expect("non-null function pointer")(current_freq_ppm);
    temp_comp_ppm =
        (uncomp_freq_ppm - current_freq_ppm) /
            (1.0e-6f64 * uncomp_freq_ppm + 1.0f64);
    return temp_comp_ppm;
}
/* Routine to update the synchronisation status in the kernel to allow other
   applications to know if the system clock is synchronised and error bounds */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LCL_SetSyncStatus(mut synchronised: libc::c_int,
                                           mut est_error: libc::c_double,
                                           mut max_error: libc::c_double) {
    if drv_set_sync_status.is_some() {
        drv_set_sync_status.expect("non-null function pointer")(synchronised,
                                                                est_error,
                                                                max_error);
    };
}
/* ================================================== */
