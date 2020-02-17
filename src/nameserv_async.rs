#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    /* Maximum number of addresses returned by DNS_Name2IPAddress */
    #[no_mangle]
    fn DNS_Name2IPAddress(name: *const libc::c_char, ip_addrs: *mut IPAddr,
                          max_addrs: libc::c_int) -> DNS_Status;
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
    /* Register a handler for when select goes true on a file descriptor */
    #[no_mangle]
    fn SCH_AddFileHandler(fd: libc::c_int, events: libc::c_int,
                          handler: SCH_FileHandler,
                          arg: SCH_ArbitraryArgument);
    #[no_mangle]
    fn SCH_RemoveFileHandler(fd: libc::c_int);
    /* Set FD_CLOEXEC on descriptor */
    #[no_mangle]
    fn UTI_FdSetCloexec(fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_create(__newthread: *mut pthread_t,
                      __attr: *const pthread_attr_t,
                      __start_routine:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> *mut libc::c_void>,
                      __arg: *mut libc::c_void) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type ssize_t = __ssize_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPAddr {
    pub addr: C2RustUnnamed,
    pub family: uint16_t,
    pub _pad: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub in4: uint32_t,
    pub in6: [uint8_t; 16],
}
pub type DNS_Status = libc::c_uint;
pub const DNS_Failure: DNS_Status = 2;
pub const DNS_TryAgain: DNS_Status = 1;
pub const DNS_Success: DNS_Status = 0;
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

  Header for asynchronous nameserver functions
  */
/* Function type for callback to process the result */
pub type DNS_NameResolveHandler
    =
    Option<unsafe extern "C" fn(_: DNS_Status, _: libc::c_int, _: *mut IPAddr,
                                _: *mut libc::c_void) -> ()>;
pub type SCH_ArbitraryArgument = *mut libc::c_void;
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

  Functions to asynchronously convert name to IP address

  */
/* ================================================== */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DNS_Async_Instance {
    pub name: *const libc::c_char,
    pub status: DNS_Status,
    pub addresses: [IPAddr; 16],
    pub handler: DNS_NameResolveHandler,
    pub arg: *mut libc::c_void,
    pub thread: pthread_t,
    pub pipe: [libc::c_int; 2],
}
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
pub type SCH_FileHandler
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                _: SCH_ArbitraryArgument) -> ()>;
static mut resolving_threads: libc::c_int = 0 as libc::c_int;
/* ================================================== */
unsafe extern "C" fn start_resolving(mut anything: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut inst: *mut DNS_Async_Instance =
        anything as *mut DNS_Async_Instance;
    (*inst).status =
        DNS_Name2IPAddress((*inst).name, (*inst).addresses.as_mut_ptr(),
                           16 as libc::c_int);
    /* Notify the main thread that the result is ready */
    (write((*inst).pipe[1 as libc::c_int as usize],
           b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
           1 as libc::c_int as size_t)) < 0 as libc::c_int as libc::c_long;
    return 0 as *mut libc::c_void;
}
/* ================================================== */
unsafe extern "C" fn end_resolving(mut fd: libc::c_int,
                                   mut event: libc::c_int,
                                   mut anything: *mut libc::c_void) {
    let mut inst: *mut DNS_Async_Instance =
        anything as *mut DNS_Async_Instance;
    let mut i: libc::c_int = 0;
    if pthread_join((*inst).thread, 0 as *mut *mut libc::c_void) != 0 {
        LOG_Message(LOGS_FATAL,
                    b"pthread_join() failed\x00" as *const u8 as
                        *const libc::c_char);
        exit(1 as libc::c_int);
    }
    resolving_threads -= 1;
    SCH_RemoveFileHandler((*inst).pipe[0 as libc::c_int as usize]);
    close((*inst).pipe[0 as libc::c_int as usize]);
    close((*inst).pipe[1 as libc::c_int as usize]);
    i = 0 as libc::c_int;
    while (*inst).status as libc::c_uint ==
              DNS_Success as libc::c_int as libc::c_uint &&
              i < 16 as libc::c_int &&
              (*inst).addresses[i as usize].family as libc::c_int !=
                  0 as libc::c_int {
        i += 1
    }
    (*inst).handler.expect("non-null function pointer")((*inst).status, i,
                                                        (*inst).addresses.as_mut_ptr(),
                                                        (*inst).arg);
    free(inst as *mut libc::c_void);
}
/* Request resolving of a name to IP address. The handler will be
   called when the result is available. */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn DNS_Name2IPAddressAsync(mut name:
                                                     *const libc::c_char,
                                                 mut handler:
                                                     DNS_NameResolveHandler,
                                                 mut anything:
                                                     *mut libc::c_void) {
    let mut inst: *mut DNS_Async_Instance = 0 as *mut DNS_Async_Instance;
    inst =
        Malloc(::std::mem::size_of::<DNS_Async_Instance>() as libc::c_ulong)
            as *mut DNS_Async_Instance;
    (*inst).name = name;
    (*inst).handler = handler;
    (*inst).arg = anything;
    (*inst).status = DNS_Failure;
    if pipe((*inst).pipe.as_mut_ptr()) != 0 {
        LOG_Message(LOGS_FATAL,
                    b"pipe() failed\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    UTI_FdSetCloexec((*inst).pipe[0 as libc::c_int as usize]);
    UTI_FdSetCloexec((*inst).pipe[1 as libc::c_int as usize]);
    resolving_threads += 1;
    if resolving_threads <= 1 as libc::c_int {
    } else {
        __assert_fail(b"resolving_threads <= 1\x00" as *const u8 as
                          *const libc::c_char,
                      b"nameserv_async.c\x00" as *const u8 as
                          *const libc::c_char,
                      120 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 75],
                                                &[libc::c_char; 75]>(b"void DNS_Name2IPAddressAsync(const char *, DNS_NameResolveHandler, void *)\x00")).as_ptr());
    }
    if pthread_create(&mut (*inst).thread, 0 as *const pthread_attr_t,
                      Some(start_resolving as
                               unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> *mut libc::c_void),
                      inst as *mut libc::c_void) != 0 {
        LOG_Message(LOGS_FATAL,
                    b"pthread_create() failed\x00" as *const u8 as
                        *const libc::c_char);
        exit(1 as libc::c_int);
    }
    SCH_AddFileHandler((*inst).pipe[0 as libc::c_int as usize],
                       1 as libc::c_int,
                       Some(end_resolving as
                                unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int,
                                                     _: *mut libc::c_void)
                                    -> ()), inst as SCH_ArbitraryArgument);
}
/* ================================================== */
