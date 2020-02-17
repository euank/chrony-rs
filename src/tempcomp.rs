#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type ARR_Instance_Record;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    /* Create a new array with given element size */
    #[no_mangle]
    fn ARR_CreateInstance(elem_size: libc::c_uint) -> ARR_Instance;
    /* Destroy the array */
    #[no_mangle]
    fn ARR_DestroyInstance(array: ARR_Instance);
    /* Return pointer to a new element added to the end of the array */
    #[no_mangle]
    fn ARR_GetNewElement(array: ARR_Instance) -> *mut libc::c_void;
    /* Return element with given index */
    #[no_mangle]
    fn ARR_GetElement(array: ARR_Instance, index: libc::c_uint)
     -> *mut libc::c_void;
    /* Return current size of the array */
    #[no_mangle]
    fn ARR_GetSize(array: ARR_Instance) -> libc::c_uint;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn CNF_GetLogTempComp() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetTempComp(file: *mut *mut libc::c_char,
                       interval: *mut libc::c_double,
                       point_file: *mut *mut libc::c_char,
                       T0_0: *mut libc::c_double, k0_0: *mut libc::c_double,
                       k1_0: *mut libc::c_double, k2_0: *mut libc::c_double);
    /* Read the system clock, corrected according to all accumulated
   drifts and uncompensated offsets.

   In a kernel implementation with vernier frequency control (like
   Linux), and if we were to apply offsets by stepping the clock, this
   would be identical to raw time.  In any other case (use of
   adjtime()-like interface to correct offsets, and to adjust the
   frequency), we must correct the raw time to get this value */
    #[no_mangle]
    fn LCL_ReadCookedTime(ts: *mut timespec, err: *mut libc::c_double);
    /* Routine to set a frequency correction (in ppm) that should be applied
   to local clock to compensate for temperature changes.  A positive
   argument means that the clock frequency should be increased. Return the
   actual compensation (may be different from the requested compensation
   due to clamping or rounding). */
    #[no_mangle]
    fn LCL_SetTempComp(comp: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn UTI_TimeToLogForm(t: time_t) -> *mut libc::c_char;
    /* Open a file.  The full path of the file is constructed from the basedir
   (may be NULL), '/' (if basedir is not NULL), name, and suffix (may be NULL).
   Created files have specified permissions (umasked).  Returns NULL on error.
   The following modes are supported (if the mode is an uppercase character,
   errors are fatal):
   r/R - open an existing file for reading
   w/W - open a new file for writing (remove existing file)
   a/A - open an existing file for appending (create if does not exist) */
    #[no_mangle]
    fn UTI_OpenFile(basedir: *const libc::c_char, name: *const libc::c_char,
                    suffix: *const libc::c_char, mode: libc::c_char,
                    perm: mode_t) -> *mut FILE;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
    #[no_mangle]
    fn LOG_FileOpen(name: *const libc::c_char, banner: *const libc::c_char)
     -> LOG_FileID;
    #[no_mangle]
    fn LOG_FileWrite(id: LOG_FileID, format: *const libc::c_char, _: ...);
    /* This queues a timeout to elapse at a given delta time relative to the current (raw) time */
    #[no_mangle]
    fn SCH_AddTimeoutByDelay(delay: libc::c_double, _: SCH_TimeoutHandler,
                             _: SCH_ArbitraryArgument) -> SCH_TimeoutID;
    /* The next one probably ought to return a status code */
    #[no_mangle]
    fn SCH_RemoveTimeout(_: SCH_TimeoutID);
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

  Header file for array functions.
  */
pub type ARR_Instance = *mut ARR_Instance_Record;
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
pub type time_t = __time_t;
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
pub type SCH_TimeoutHandler
    =
    Option<unsafe extern "C" fn(_: SCH_ArbitraryArgument) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Point {
    pub temp: libc::c_double,
    pub comp: libc::c_double,
}
static mut timeout_id: SCH_TimeoutID = 0;
static mut logfileid: LOG_FileID = 0;
static mut filename: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut update_interval: libc::c_double = 0.;
static mut T0: libc::c_double = 0.;
static mut k0: libc::c_double = 0.;
static mut k1: libc::c_double = 0.;
static mut k2: libc::c_double = 0.;
static mut points: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
unsafe extern "C" fn get_tempcomp(mut temp: libc::c_double)
 -> libc::c_double {
    let mut i: libc::c_uint = 0;
    let mut p1: *mut Point = 0 as *mut Point;
    let mut p2: *mut Point = 0 as *mut Point;
    /* If not configured with points, calculate the compensation from the
     specified quadratic function */
    if points.is_null() {
        return k0 + (temp - T0) * k1 + (temp - T0) * (temp - T0) * k2
    }
    /* Otherwise interpolate/extrapolate between two nearest points */
    i = 1 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(points) {
        p2 = ARR_GetElement(points, i) as *mut Point;
        if (*p2).temp >= temp { break ; }
        i = i.wrapping_add(1)
    }
    p1 = p2.offset(-(1 as libc::c_int as isize));
    return (temp - (*p1).temp) / ((*p2).temp - (*p1).temp) *
               ((*p2).comp - (*p1).comp) + (*p1).comp;
}
unsafe extern "C" fn read_timeout(mut arg: *mut libc::c_void) {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut temp: libc::c_double = 0.;
    let mut comp: libc::c_double = 0.;
    f =
        UTI_OpenFile(0 as *const libc::c_char, filename,
                     0 as *const libc::c_char, 'r' as i32 as libc::c_char,
                     0 as libc::c_int as mode_t);
    if !f.is_null() &&
           fscanf(f, b"%lf\x00" as *const u8 as *const libc::c_char,
                  &mut temp as *mut libc::c_double) == 1 as libc::c_int {
        comp = get_tempcomp(temp);
        if fabs(comp) <= 10.0f64 {
            comp = LCL_SetTempComp(comp);
            if 0 as libc::c_int != 0 &&
                   log_min_severity as libc::c_int ==
                       LOGS_DEBUG as libc::c_int {
                LOG_Message(LOGS_DEBUG,
                            b"tempcomp updated to %f for %f\x00" as *const u8
                                as *const libc::c_char, comp, temp);
            }
            if logfileid != -(1 as libc::c_int) {
                let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
                LCL_ReadCookedTime(&mut now, 0 as *mut libc::c_double);
                LOG_FileWrite(logfileid,
                              b"%s %11.4e %11.4e\x00" as *const u8 as
                                  *const libc::c_char,
                              UTI_TimeToLogForm(now.tv_sec), temp, comp);
            }
        } else {
            LOG_Message(LOGS_WARN,
                        b"Temperature compensation of %.3f ppm exceeds sanity limit of %.1f\x00"
                            as *const u8 as *const libc::c_char, comp,
                        10.0f64);
        }
    } else {
        LOG_Message(LOGS_WARN,
                    b"Could not read temperature from %s\x00" as *const u8 as
                        *const libc::c_char, filename);
    }
    if !f.is_null() { fclose(f); }
    timeout_id =
        SCH_AddTimeoutByDelay(update_interval,
                              Some(read_timeout as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           -> ()), 0 as *mut libc::c_void);
}
unsafe extern "C" fn read_points(mut filename_0: *const libc::c_char) {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut line: [libc::c_char; 256] = [0; 256];
    let mut p: *mut Point = 0 as *mut Point;
    f =
        UTI_OpenFile(0 as *const libc::c_char, filename_0,
                     0 as *const libc::c_char, 'R' as i32 as libc::c_char,
                     0 as libc::c_int as mode_t);
    points =
        ARR_CreateInstance(::std::mem::size_of::<Point>() as libc::c_ulong as
                               libc::c_uint);
    while !fgets(line.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                     as libc::c_int, f).is_null() {
        p = ARR_GetNewElement(points) as *mut Point;
        if !(sscanf(line.as_mut_ptr(),
                    b"%lf %lf\x00" as *const u8 as *const libc::c_char,
                    &mut (*p).temp as *mut libc::c_double,
                    &mut (*p).comp as *mut libc::c_double) !=
                 2 as libc::c_int) {
            continue ;
        }
        LOG_Message(LOGS_FATAL,
                    b"Could not read tempcomp point from %s\x00" as *const u8
                        as *const libc::c_char, filename_0);
        exit(1 as libc::c_int);
    }
    fclose(f);
    if ARR_GetSize(points) < 2 as libc::c_int as libc::c_uint {
        LOG_Message(LOGS_FATAL,
                    b"Not enough points in %s\x00" as *const u8 as
                        *const libc::c_char, filename_0);
        exit(1 as libc::c_int);
    };
}
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2011
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

  Header file for temperature compensation.

  */
#[no_mangle]
pub unsafe extern "C" fn TMC_Initialise() {
    let mut point_file: *mut libc::c_char = 0 as *mut libc::c_char;
    CNF_GetTempComp(&mut filename, &mut update_interval, &mut point_file,
                    &mut T0, &mut k0, &mut k1, &mut k2);
    if filename.is_null() { return }
    if update_interval <= 0.0f64 { update_interval = 1.0f64 }
    if !point_file.is_null() { read_points(point_file); }
    logfileid =
        if CNF_GetLogTempComp() != 0 {
            LOG_FileOpen(b"tempcomp\x00" as *const u8 as *const libc::c_char,
                         b"   Date (UTC) Time        Temp.       Comp.\x00" as
                             *const u8 as *const libc::c_char)
        } else { -(1 as libc::c_int) };
    read_timeout(0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn TMC_Finalise() {
    if filename.is_null() { return }
    if !points.is_null() { ARR_DestroyInstance(points); }
    SCH_RemoveTimeout(timeout_id);
}
