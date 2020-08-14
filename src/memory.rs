#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
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

 Header file for memory functions
 */
/* Wrappers checking for errors */
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Miroslav Lichvar  2014, 2017
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

 Utility functions for memory allocation.

 */
#[no_mangle]
pub unsafe extern "C" fn Malloc(mut size: size_t) -> *mut libc::c_void {
    let mut r: *mut libc::c_void = 0 as *mut libc::c_void;
    r = malloc(size);
    if r.is_null() && size != 0 {
        LOG_Message(
            LOGS_FATAL,
            b"Could not allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn Realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut r: *mut libc::c_void = 0 as *mut libc::c_void;
    r = realloc(ptr, size);
    if r.is_null() && size != 0 {
        LOG_Message(
            LOGS_FATAL,
            b"Could not allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return r;
}
unsafe extern "C" fn get_array_size(mut nmemb: size_t, mut size: size_t) -> size_t {
    let mut array_size: size_t = 0;
    array_size = nmemb.wrapping_mul(size);
    /* Check for overflow */
    if nmemb > 0 as libc::c_int as libc::c_ulong && array_size.wrapping_div(nmemb) != size {
        LOG_Message(
            LOGS_FATAL,
            b"Could not allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return array_size;
}
#[no_mangle]
pub unsafe extern "C" fn Malloc2(mut nmemb: size_t, mut size: size_t) -> *mut libc::c_void {
    return Malloc(get_array_size(nmemb, size));
}
#[no_mangle]
pub unsafe extern "C" fn Realloc2(
    mut ptr: *mut libc::c_void,
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return Realloc(ptr, get_array_size(nmemb, size));
}
#[no_mangle]
pub unsafe extern "C" fn Strdup(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut r: *mut libc::c_void = 0 as *mut libc::c_void;
    r = strdup(s) as *mut libc::c_void;
    if r.is_null() {
        LOG_Message(
            LOGS_FATAL,
            b"Could not allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return r as *mut libc::c_char;
}
