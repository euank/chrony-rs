#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(asm, c_variadic, const_raw_ptr_to_usize_cast, extern_types,
           label_break_value, main, register_tool)]

mod clientlog;

use c2rust_asm_casts::AsmCastTrait;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ARR_Instance_Record;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
              __writefds: *mut fd_set, __exceptfds: *mut fd_set,
              __timeout: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    #[no_mangle]
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    #[no_mangle]
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    #[no_mangle]
    fn htons(__hostshort: uint16_t) -> uint16_t;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn vprintf(_: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
     -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn gmtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
              __shortopts: *const libc::c_char) -> libc::c_int;
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
    fn Strdup(s: *const libc::c_char) -> *mut libc::c_char;
    /* Resolve names only to selected address family */
    #[no_mangle]
    fn DNS_SetAddressFamily(family: libc::c_int);
    #[no_mangle]
    fn DNS_Name2IPAddress(name: *const libc::c_char, ip_addrs: *mut IPAddr,
                          max_addrs: libc::c_int) -> DNS_Status;
    #[no_mangle]
    fn DNS_IPAddress2Name(ip_addr: *mut IPAddr, name: *mut libc::c_char,
                          len: libc::c_int) -> libc::c_int;
    /*  Copyright (C) 1995 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software Foundation,
   Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA. */
    /* Modified from the original to add stdlib.h and string.h */
    #[no_mangle]
    fn get_date(p: *const libc::c_char, now: *const time_t) -> time_t;
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

  Header file for the command parser
  */
    /* Parse a command to add an NTP server or peer */
    /* Parse a command to enable local reference */
    /* Remove extra white-space and comments */
    /* Terminate first word and return pointer to the next word */
    #[no_mangle]
    fn CPS_SplitWord(line: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn CPS_ParseNTPSourceAdd(line: *mut libc::c_char,
                             src: *mut CPS_NTP_Source) -> libc::c_int;
    #[no_mangle]
    fn CPS_ParseLocal(line: *mut libc::c_char, stratum: *mut libc::c_int,
                      orphan: *mut libc::c_int, distance: *mut libc::c_double)
     -> libc::c_int;
    #[no_mangle]
    fn CPS_NormalizeLine(line: *mut libc::c_char);
    /*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2012
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

  Header file for crypto hashing.

  */
    /* length of hash values produced by SHA512 */
    #[no_mangle]
    fn HSH_GetHashId(name: *const libc::c_char) -> libc::c_int;
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

  Header for pktlength.c, routines for working out the expected length
  of a network command/reply packet.

  */
    #[no_mangle]
    fn PKL_CommandLength(r: *mut CMD_Request) -> libc::c_int;
    #[no_mangle]
    fn PKL_CommandPaddingLength(r: *mut CMD_Request) -> libc::c_int;
    #[no_mangle]
    fn PKL_ReplyLength(r: *mut CMD_Reply) -> libc::c_int;
    /* Initialisation function */
    #[no_mangle]
    fn SCK_Initialise();
    /* Finalisation function */
    #[no_mangle]
    fn SCK_Finalise();
    /* Open socket */
    #[no_mangle]
    fn SCK_OpenUdpSocket(remote_addr: *mut IPSockAddr,
                         local_addr: *mut IPSockAddr, flags: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn SCK_OpenUnixDatagramSocket(remote_addr: *const libc::c_char,
                                  local_addr: *const libc::c_char,
                                  flags: libc::c_int) -> libc::c_int;
    /* Receive and send data on connected sockets - recv()/send() wrappers */
    #[no_mangle]
    fn SCK_Receive(sock_fd_0: libc::c_int, buffer: *mut libc::c_void,
                   length: libc::c_uint, flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SCK_Send(sock_fd_0: libc::c_int, buffer: *const libc::c_void,
                length: libc::c_uint, flags: libc::c_int) -> libc::c_int;
    /* Remove bound Unix socket */
    #[no_mangle]
    fn SCK_RemoveSocket(sock_fd_0: libc::c_int) -> libc::c_int;
    /* Close the socket */
    #[no_mangle]
    fn SCK_CloseSocket(sock_fd_0: libc::c_int);
    /* Convert a timeval into a timespec */
    #[no_mangle]
    fn UTI_TimevalToTimespec(tv: *mut timeval, ts: *mut timespec);
    /* Convert a number of seconds expressed in floating point into a
   timeval */
    #[no_mangle]
    fn UTI_DoubleToTimeval(a: libc::c_double, b: *mut timeval);
    /* Returns -1 if a comes earlier than b, 0 if a is the same time as b,
   and +1 if a comes after b */
    #[no_mangle]
    fn UTI_CompareTimespecs(a: *mut timespec, b: *mut timespec)
     -> libc::c_int;
    /* Calculate result = a - b and return as a double */
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec)
     -> libc::c_double;
    /* Convert a timespec into a temporary string, largely for diagnostic
   display */
    #[no_mangle]
    fn UTI_TimespecToString(ts: *mut timespec) -> *mut libc::c_char;
    /* Convert ref_id into a temporary string, for diagnostics */
    #[no_mangle]
    fn UTI_RefidToString(ref_id: uint32_t) -> *mut libc::c_char;
    /* Convert an IP address to string, for diagnostics */
    #[no_mangle]
    fn UTI_IPToString(ip: *mut IPAddr) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_StringToIP(addr: *const libc::c_char, ip: *mut IPAddr)
     -> libc::c_int;
    #[no_mangle]
    fn UTI_IPToRefid(ip: *mut IPAddr) -> uint32_t;
    #[no_mangle]
    fn UTI_IPHostToNetwork(src: *mut IPAddr, dest: *mut IPAddr);
    #[no_mangle]
    fn UTI_IPNetworkToHost(src: *mut IPAddr, dest: *mut IPAddr);
    #[no_mangle]
    fn UTI_TimeToLogForm(t: time_t) -> *mut libc::c_char;
    /* Get 2 raised to power of a signed integer */
    #[no_mangle]
    fn UTI_Log2ToDouble(l: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn UTI_TimespecNetworkToHost(src: *mut Timespec, dest: *mut timespec);
    #[no_mangle]
    fn UTI_TimespecHostToNetwork(src: *mut timespec, dest: *mut Timespec);
    #[no_mangle]
    fn UTI_FloatNetworkToHost(x: Float) -> libc::c_double;
    #[no_mangle]
    fn UTI_FloatHostToNetwork(x: libc::c_double) -> Float;
    #[no_mangle]
    fn UTI_SetQuitSignalsHandler(handler:
                                     Option<unsafe extern "C" fn(_:
                                                                     libc::c_int)
                                                -> ()>,
                                 ignore_sigpipe: libc::c_int);
    /* Get directory (as an allocated string) for a path */
    #[no_mangle]
    fn UTI_PathToDir(path: *const libc::c_char) -> *mut libc::c_char;
    /* Fill buffer with random bytes from /dev/urandom */
    #[no_mangle]
    fn UTI_GetRandomBytesUrandom(buf: *mut libc::c_void, len: libc::c_uint);
    /* Fill buffer with random bytes from /dev/urandom or a faster source if it's
   available (e.g. arc4random()), which may not necessarily be suitable for
   generating long-term keys */
    #[no_mangle]
    fn UTI_GetRandomBytes(buf: *mut libc::c_void, len: libc::c_uint);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
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
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPAddr {
    pub addr: C2RustUnnamed_0,
    pub family: uint16_t,
    pub _pad: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub in4: uint32_t,
    pub in6: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPSockAddr {
    pub ip_addr: IPAddr,
    pub port: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Timespec {
    pub tv_sec_high: uint32_t,
    pub tv_sec_low: uint32_t,
    pub tv_nsec: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Float {
    pub f: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Null {
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Online {
    pub mask: IPAddr,
    pub address: IPAddr,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Offline {
    pub mask: IPAddr,
    pub address: IPAddr,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Burst {
    pub mask: IPAddr,
    pub address: IPAddr,
    pub n_good_samples: int32_t,
    pub n_total_samples: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Modify_Minpoll {
    pub address: IPAddr,
    pub new_minpoll: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Modify_Maxpoll {
    pub address: IPAddr,
    pub new_maxpoll: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Dump {
    pub pad: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Modify_Maxdelay {
    pub address: IPAddr,
    pub new_max_delay: Float,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Modify_Maxdelayratio {
    pub address: IPAddr,
    pub new_max_delay_ratio: Float,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Modify_Maxdelaydevratio {
    pub address: IPAddr,
    pub new_max_delay_dev_ratio: Float,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Modify_Minstratum {
    pub address: IPAddr,
    pub new_min_stratum: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Modify_Polltarget {
    pub address: IPAddr,
    pub new_poll_target: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Modify_Maxupdateskew {
    pub new_max_update_skew: Float,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Modify_Makestep {
    pub limit: int32_t,
    pub threshold: Float,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Logon {
    pub ts: Timespec,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Settime {
    pub ts: Timespec,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Local {
    pub on_off: int32_t,
    pub stratum: int32_t,
    pub distance: Float,
    pub orphan: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Manual {
    pub option: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Source_Data {
    pub index: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Allow_Deny {
    pub ip: IPAddr,
    pub subnet_bits: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Ac_Check {
    pub ip: IPAddr,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_NTP_Source {
    pub type_0: uint32_t,
    pub name: [int8_t; 256],
    pub port: uint32_t,
    pub minpoll: int32_t,
    pub maxpoll: int32_t,
    pub presend_minpoll: int32_t,
    pub min_stratum: uint32_t,
    pub poll_target: uint32_t,
    pub version: uint32_t,
    pub max_sources: uint32_t,
    pub min_samples: int32_t,
    pub max_samples: int32_t,
    pub authkey: uint32_t,
    pub max_delay: Float,
    pub max_delay_ratio: Float,
    pub max_delay_dev_ratio: Float,
    pub min_delay: Float,
    pub asymmetry: Float,
    pub offset: Float,
    pub flags: uint32_t,
    pub filter_length: int32_t,
    pub reserved: [uint32_t; 3],
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Del_Source {
    pub ip_addr: IPAddr,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Dfreq {
    pub dfreq: Float,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Doffset {
    pub sec: int32_t,
    pub usec: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_Sourcestats {
    pub index: uint32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_ClientAccessesByIndex {
    pub first_index: uint32_t,
    pub n_clients: uint32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_ManualDelete {
    pub index: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_ReselectDistance {
    pub distance: Float,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_SmoothTime {
    pub option: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REQ_NTPData {
    pub ip_addr: IPAddr,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMD_Request {
    pub version: uint8_t,
    pub pkt_type: uint8_t,
    pub res1: uint8_t,
    pub res2: uint8_t,
    pub command: uint16_t,
    pub attempt: uint16_t,
    pub sequence: uint32_t,
    pub pad1: uint32_t,
    pub pad2: uint32_t,
    pub data: C2RustUnnamed_1,
    pub padding: [uint8_t; 396],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub null: REQ_Null,
    pub online: REQ_Online,
    pub offline: REQ_Offline,
    pub burst: REQ_Burst,
    pub modify_minpoll: REQ_Modify_Minpoll,
    pub modify_maxpoll: REQ_Modify_Maxpoll,
    pub dump: REQ_Dump,
    pub modify_maxdelay: REQ_Modify_Maxdelay,
    pub modify_maxdelayratio: REQ_Modify_Maxdelayratio,
    pub modify_maxdelaydevratio: REQ_Modify_Maxdelaydevratio,
    pub modify_minstratum: REQ_Modify_Minstratum,
    pub modify_polltarget: REQ_Modify_Polltarget,
    pub modify_maxupdateskew: REQ_Modify_Maxupdateskew,
    pub modify_makestep: REQ_Modify_Makestep,
    pub logon: REQ_Logon,
    pub settime: REQ_Settime,
    pub local: REQ_Local,
    pub manual: REQ_Manual,
    pub source_data: REQ_Source_Data,
    pub allow_deny: REQ_Allow_Deny,
    pub ac_check: REQ_Ac_Check,
    pub ntp_source: REQ_NTP_Source,
    pub del_source: REQ_Del_Source,
    pub dfreq: REQ_Dfreq,
    pub doffset: REQ_Doffset,
    pub sourcestats: REQ_Sourcestats,
    pub client_accesses_by_index: REQ_ClientAccessesByIndex,
    pub manual_delete: REQ_ManualDelete,
    pub reselect_distance: REQ_ReselectDistance,
    pub smoothtime: REQ_SmoothTime,
    pub ntp_data: REQ_NTPData,
    pub ntp_source_name: REQ_NTPData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_Null {
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_N_Sources {
    pub n_sources: uint32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_Source_Data {
    pub ip_addr: IPAddr,
    pub poll: int16_t,
    pub stratum: uint16_t,
    pub state: uint16_t,
    pub mode: uint16_t,
    pub flags: uint16_t,
    pub reachability: uint16_t,
    pub since_sample: uint32_t,
    pub orig_latest_meas: Float,
    pub latest_meas: Float,
    pub latest_meas_err: Float,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_Tracking {
    pub ref_id: uint32_t,
    pub ip_addr: IPAddr,
    pub stratum: uint16_t,
    pub leap_status: uint16_t,
    pub ref_time: Timespec,
    pub current_correction: Float,
    pub last_offset: Float,
    pub rms_offset: Float,
    pub freq_ppm: Float,
    pub resid_freq_ppm: Float,
    pub skew_ppm: Float,
    pub root_delay: Float,
    pub root_dispersion: Float,
    pub last_update_interval: Float,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_Sourcestats {
    pub ref_id: uint32_t,
    pub ip_addr: IPAddr,
    pub n_samples: uint32_t,
    pub n_runs: uint32_t,
    pub span_seconds: uint32_t,
    pub sd: Float,
    pub resid_freq_ppm: Float,
    pub skew_ppm: Float,
    pub est_offset: Float,
    pub est_offset_err: Float,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_Rtc {
    pub ref_time: Timespec,
    pub n_samples: uint16_t,
    pub n_runs: uint16_t,
    pub span_seconds: uint32_t,
    pub rtc_seconds_fast: Float,
    pub rtc_gain_rate_ppm: Float,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_ManualTimestamp {
    pub offset: Float,
    pub dfreq_ppm: Float,
    pub new_afreq_ppm: Float,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_ClientAccesses_Client {
    pub ip: IPAddr,
    pub ntp_hits: uint32_t,
    pub cmd_hits: uint32_t,
    pub ntp_drops: uint32_t,
    pub cmd_drops: uint32_t,
    pub ntp_interval: int8_t,
    pub cmd_interval: int8_t,
    pub ntp_timeout_interval: int8_t,
    pub pad: int8_t,
    pub last_ntp_hit_ago: uint32_t,
    pub last_cmd_hit_ago: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_ClientAccessesByIndex {
    pub n_indices: uint32_t,
    pub next_index: uint32_t,
    pub n_clients: uint32_t,
    pub clients: [RPY_ClientAccesses_Client; 8],
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_ServerStats {
    pub ntp_hits: uint32_t,
    pub cmd_hits: uint32_t,
    pub ntp_drops: uint32_t,
    pub cmd_drops: uint32_t,
    pub log_drops: uint32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_ManualListSample {
    pub when: Timespec,
    pub slewed_offset: Float,
    pub orig_offset: Float,
    pub residual: Float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_ManualList {
    pub n_samples: uint32_t,
    pub samples: [RPY_ManualListSample; 16],
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_Activity {
    pub online: int32_t,
    pub offline: int32_t,
    pub burst_online: int32_t,
    pub burst_offline: int32_t,
    pub unresolved: int32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_Smoothing {
    pub flags: uint32_t,
    pub offset: Float,
    pub freq_ppm: Float,
    pub wander_ppm: Float,
    pub last_update_ago: Float,
    pub remaining_time: Float,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_NTPData {
    pub remote_addr: IPAddr,
    pub local_addr: IPAddr,
    pub remote_port: uint16_t,
    pub leap: uint8_t,
    pub version: uint8_t,
    pub mode: uint8_t,
    pub stratum: uint8_t,
    pub poll: int8_t,
    pub precision: int8_t,
    pub root_delay: Float,
    pub root_dispersion: Float,
    pub ref_id: uint32_t,
    pub ref_time: Timespec,
    pub offset: Float,
    pub peer_delay: Float,
    pub peer_dispersion: Float,
    pub response_time: Float,
    pub jitter_asymmetry: Float,
    pub flags: uint16_t,
    pub tx_tss_char: uint8_t,
    pub rx_tss_char: uint8_t,
    pub total_tx_count: uint32_t,
    pub total_rx_count: uint32_t,
    pub total_valid_count: uint32_t,
    pub reserved: [uint32_t; 4],
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_NTPSourceName {
    pub name: [int8_t; 256],
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMD_Reply {
    pub version: uint8_t,
    pub pkt_type: uint8_t,
    pub res1: uint8_t,
    pub res2: uint8_t,
    pub command: uint16_t,
    pub reply: uint16_t,
    pub status: uint16_t,
    pub pad1: uint16_t,
    pub pad2: uint16_t,
    pub pad3: uint16_t,
    pub sequence: uint32_t,
    pub pad4: uint32_t,
    pub pad5: uint32_t,
    pub data: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub null: RPY_Null,
    pub n_sources: RPY_N_Sources,
    pub source_data: RPY_Source_Data,
    pub manual_timestamp: RPY_ManualTimestamp,
    pub tracking: RPY_Tracking,
    pub sourcestats: RPY_Sourcestats,
    pub rtc: RPY_Rtc,
    pub client_accesses_by_index: RPY_ClientAccessesByIndex,
    pub server_stats: RPY_ServerStats,
    pub manual_list: RPY_ManualList,
    pub activity: RPY_Activity,
    pub smoothing: RPY_Smoothing,
    pub ntp_data: RPY_NTPData,
    pub ntp_source_name: RPY_NTPSourceName,
}
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
pub type DNS_Status = libc::c_uint;
pub const DNS_Failure: DNS_Status = 2;
pub const DNS_TryAgain: DNS_Status = 1;
pub const DNS_Success: DNS_Status = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const LEAP_Unsynchronised: C2RustUnnamed_3 = 3;
pub const LEAP_DeleteSecond: C2RustUnnamed_3 = 2;
pub const LEAP_InsertSecond: C2RustUnnamed_3 = 1;
pub const LEAP_Normal: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const MODE_BROADCAST: C2RustUnnamed_4 = 5;
pub const MODE_SERVER: C2RustUnnamed_4 = 4;
pub const MODE_CLIENT: C2RustUnnamed_4 = 3;
pub const MODE_PASSIVE: C2RustUnnamed_4 = 2;
pub const MODE_ACTIVE: C2RustUnnamed_4 = 1;
pub const MODE_UNDEFINED: C2RustUnnamed_4 = 0;
pub type SRC_Connectivity = libc::c_uint;
pub const SRC_MAYBE_ONLINE: SRC_Connectivity = 2;
pub const SRC_ONLINE: SRC_Connectivity = 1;
pub const SRC_OFFLINE: SRC_Connectivity = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SourceParameters {
    pub minpoll: libc::c_int,
    pub maxpoll: libc::c_int,
    pub connectivity: SRC_Connectivity,
    pub auto_offline: libc::c_int,
    pub presend_minpoll: libc::c_int,
    pub burst: libc::c_int,
    pub iburst: libc::c_int,
    pub min_stratum: libc::c_int,
    pub poll_target: libc::c_int,
    pub version: libc::c_int,
    pub max_sources: libc::c_int,
    pub min_samples: libc::c_int,
    pub max_samples: libc::c_int,
    pub filter_length: libc::c_int,
    pub interleaved: libc::c_int,
    pub sel_options: libc::c_int,
    pub authkey: uint32_t,
    pub max_delay: libc::c_double,
    pub max_delay_ratio: libc::c_double,
    pub max_delay_dev_ratio: libc::c_double,
    pub min_delay: libc::c_double,
    pub asymmetry: libc::c_double,
    pub offset: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPS_NTP_Source {
    pub name: *mut libc::c_char,
    pub port: libc::c_ushort,
    pub params: SourceParameters,
}
pub type SCK_AddressType = libc::c_uint;
pub const SCK_ADDR_UNIX: SCK_AddressType = 2;
pub const SCK_ADDR_IP: SCK_AddressType = 1;
pub const SCK_ADDR_UNSPEC: SCK_AddressType = 0;
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Lonnie Abelbeck  2016, 2018
 * Copyright (C) Miroslav Lichvar  2009-2018
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

  Command line client for configuring the daemon and obtaining status
  from it whilst running.
  */
/* ================================================== */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Address {
    pub type_0: SCK_AddressType,
    pub addr: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ip: IPSockAddr,
    pub path: *mut libc::c_char,
}
static mut server_addresses: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
static mut sock_fd: libc::c_int = -(1 as libc::c_int);
static mut quit: libc::c_int = 0 as libc::c_int;
static mut on_terminal: libc::c_int = 0 as libc::c_int;
static mut no_dns: libc::c_int = 0 as libc::c_int;
static mut source_names: libc::c_int = 0 as libc::c_int;
static mut csv_mode: libc::c_int = 0 as libc::c_int;
/* ================================================== */
/* Log a message. This is a minimalistic replacement of the logging.c
   implementation to avoid linking with it and other modules. */
#[no_mangle]
pub static mut log_min_severity: LOG_Severity = LOGS_INFO;
/* Line logging function */
#[no_mangle]
pub unsafe extern "C" fn LOG_Message(mut severity: LOG_Severity,
                                     mut format: *const libc::c_char,
                                     mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    if (severity as libc::c_int) < log_min_severity as libc::c_int { return }
    ap = args.clone();
    vfprintf(stderr, format, ap.as_va_list());
    putc('\n' as i32, stderr);
}
/* ================================================== */
/* Read a single line of commands from standard input */
unsafe extern "C" fn read_line() -> *mut libc::c_char {
    static mut line: [libc::c_char; 2048] = [0; 2048];
    static mut prompt: *const libc::c_char =
        b"chronyc> \x00" as *const u8 as *const libc::c_char;
    if on_terminal != 0 {
        printf(b"%s\x00" as *const u8 as *const libc::c_char, prompt);
        fflush(stdout);
    }
    if !fgets(line.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong
                  as libc::c_int, stdin).is_null() {
        return line.as_mut_ptr()
    } else { return 0 as *mut libc::c_char };
}
/* ================================================== */
unsafe extern "C" fn get_addresses(mut hostnames: *const libc::c_char,
                                   mut port: libc::c_int) -> ARR_Instance {
    let mut addr: *mut Address = 0 as *mut Address;
    let mut addrs: ARR_Instance = 0 as *mut ARR_Instance_Record;
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip_addrs: [IPAddr; 16] =
        [IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,}; 16];
    let mut i: libc::c_int = 0;
    addrs =
        ARR_CreateInstance(::std::mem::size_of::<Address>() as libc::c_ulong
                               as libc::c_uint);
    s1 = Strdup(hostnames);
    /* Parse the comma-separated list of hostnames */
    hostname = s1;
    while !hostname.is_null() && *hostname as libc::c_int != 0 {
        s2 = strchr(hostname, ',' as i32);
        if !s2.is_null() {
            let fresh0 = s2;
            s2 = s2.offset(1);
            *fresh0 = '\u{0}' as i32 as libc::c_char
        }
        /* hostname starting with / is considered a path of Unix domain socket */
        if *hostname.offset(0 as libc::c_int as isize) as libc::c_int ==
               '/' as i32 {
            addr = ARR_GetNewElement(addrs) as *mut Address;
            (*addr).type_0 = SCK_ADDR_UNIX;
            (*addr).addr.path = Strdup(hostname)
        } else if DNS_Name2IPAddress(hostname, ip_addrs.as_mut_ptr(),
                                     16 as libc::c_int) as libc::c_uint !=
                      DNS_Success as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 &&
                   log_min_severity as libc::c_int ==
                       LOGS_DEBUG as libc::c_int {
                LOG_Message(LOGS_DEBUG,
                            b"Could not get IP address for %s\x00" as
                                *const u8 as *const libc::c_char, hostname);
            }
        } else {
            i = 0 as libc::c_int;
            while i < 16 as libc::c_int &&
                      ip_addrs[i as usize].family as libc::c_int !=
                          0 as libc::c_int {
                addr = ARR_GetNewElement(addrs) as *mut Address;
                (*addr).type_0 = SCK_ADDR_IP;
                (*addr).addr.ip.ip_addr = ip_addrs[i as usize];
                (*addr).addr.ip.port = port as uint16_t;
                if 0 as libc::c_int != 0 &&
                       log_min_severity as libc::c_int ==
                           LOGS_DEBUG as libc::c_int {
                    LOG_Message(LOGS_DEBUG,
                                b"Resolved %s to %s\x00" as *const u8 as
                                    *const libc::c_char, hostname,
                                UTI_IPToString(&mut *ip_addrs.as_mut_ptr().offset(i
                                                                                      as
                                                                                      isize)));
                }
                i += 1
            }
        }
        hostname = s2
    }
    free(s1 as *mut libc::c_void);
    return addrs;
}
/* ================================================== */
unsafe extern "C" fn free_addresses(mut addresses: ARR_Instance) {
    let mut addr: *mut Address = 0 as *mut Address;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(addresses) {
        addr = ARR_GetElement(addresses, i) as *mut Address;
        if (*addr).type_0 as libc::c_uint ==
               SCK_ADDR_UNIX as libc::c_int as libc::c_uint {
            free((*addr).addr.path as *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    }
    ARR_DestroyInstance(addresses);
}
/* ================================================== */
/* Initialise the socket used to talk to the daemon */
unsafe extern "C" fn open_socket(mut addr: *mut Address) -> libc::c_int {
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut local_addr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut local_addr_len: size_t = 0;
    match (*addr).type_0 as libc::c_uint {
        1 => {
            sock_fd =
                SCK_OpenUdpSocket(&mut (*addr).addr.ip, 0 as *mut IPSockAddr,
                                  0 as libc::c_int)
        }
        2 => {
            /* Construct path of our socket.  Use the same directory as the server
         socket and include our process ID to allow multiple chronyc instances
         running at the same time. */
            dir = UTI_PathToDir((*addr).addr.path);
            local_addr_len =
                strlen(dir).wrapping_add(50 as libc::c_int as libc::c_ulong);
            local_addr = Malloc(local_addr_len) as *mut libc::c_char;
            snprintf(local_addr, local_addr_len,
                     b"%s/chronyc.%d.sock\x00" as *const u8 as
                         *const libc::c_char, dir, getpid());
            sock_fd =
                SCK_OpenUnixDatagramSocket((*addr).addr.path, local_addr,
                                           8 as libc::c_int);
            free(dir as *mut libc::c_void);
            free(local_addr as *mut libc::c_void);
        }
        _ => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"client.c\x00" as *const u8 as *const libc::c_char,
                          245 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 34],
                                                    &[libc::c_char; 34]>(b"int open_socket(struct Address *)\x00")).as_ptr());
        }
    }
    if sock_fd < 0 as libc::c_int { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn close_io() {
    if sock_fd < 0 as libc::c_int { return }
    SCK_RemoveSocket(sock_fd);
    SCK_CloseSocket(sock_fd);
    sock_fd = -(1 as libc::c_int);
}
/* ================================================== */
unsafe extern "C" fn open_io() -> libc::c_int {
    static mut address_index: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut addr: *mut Address = 0 as *mut Address;
    /* If a socket is already opened, close it and try the next address */
    if sock_fd >= 0 as libc::c_int {
        close_io();
        address_index = address_index.wrapping_add(1)
    }
    /* Find an address for which a socket can be opened and connected */
    while address_index < ARR_GetSize(server_addresses) {
        addr =
            ARR_GetElement(server_addresses, address_index) as *mut Address;
        if open_socket(addr) != 0 { return 1 as libc::c_int }
        close_io();
        address_index = address_index.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn bits_to_mask(mut bits: libc::c_int,
                                  mut family: libc::c_int,
                                  mut mask: *mut IPAddr) {
    let mut i: libc::c_int = 0;
    (*mask).family = family as uint16_t;
    match family {
        1 => {
            if bits > 32 as libc::c_int || bits < 0 as libc::c_int {
                bits = 32 as libc::c_int
            }
            if bits > 0 as libc::c_int {
                (*mask).addr.in4 = -(1 as libc::c_int) as uint32_t;
                (*mask).addr.in4 <<= 32 as libc::c_int - bits
            } else { (*mask).addr.in4 = 0 as libc::c_int as uint32_t }
        }
        2 => {
            if bits > 128 as libc::c_int || bits < 0 as libc::c_int {
                bits = 128 as libc::c_int
            }
            i = 0 as libc::c_int;
            while i < bits / 8 as libc::c_int {
                (*mask).addr.in6[i as usize] = 0xff as libc::c_int as uint8_t;
                i += 1
            }
            if i < 16 as libc::c_int {
                let fresh1 = i;
                i = i + 1;
                (*mask).addr.in6[fresh1 as usize] =
                    ((0xff as libc::c_int) <<
                         8 as libc::c_int - bits % 8 as libc::c_int &
                         0xff as libc::c_int) as uint8_t
            }
            while i < 16 as libc::c_int {
                (*mask).addr.in6[i as usize] = 0 as libc::c_int as uint8_t;
                i += 1
            }
        }
        _ => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"client.c\x00" as *const u8 as *const libc::c_char,
                          324 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"void bits_to_mask(int, int, IPAddr *)\x00")).as_ptr());
        }
    };
}
/* ================================================== */
unsafe extern "C" fn read_mask_address(mut line: *mut libc::c_char,
                                       mut mask: *mut IPAddr,
                                       mut address: *mut IPAddr)
 -> libc::c_int {
    let mut bits: libc::c_uint = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    p = line;
    if *p == 0 {
        (*address).family = 0 as libc::c_int as uint16_t;
        (*mask).family = (*address).family;
        return 1 as libc::c_int
    } else {
        q = strchr(p, '/' as i32);
        if !q.is_null() {
            let fresh2 = q;
            q = q.offset(1);
            *fresh2 = 0 as libc::c_int as libc::c_char;
            if UTI_StringToIP(p, mask) != 0 {
                p = q;
                if UTI_StringToIP(p, address) != 0 {
                    if (*address).family as libc::c_int ==
                           (*mask).family as libc::c_int {
                        return 1 as libc::c_int
                    }
                } else if sscanf(p,
                                 b"%u\x00" as *const u8 as
                                     *const libc::c_char,
                                 &mut bits as *mut libc::c_uint) ==
                              1 as libc::c_int {
                    *address = *mask;
                    bits_to_mask(bits as libc::c_int,
                                 (*address).family as libc::c_int, mask);
                    return 1 as libc::c_int
                }
            }
        } else if DNS_Name2IPAddress(p, address, 1 as libc::c_int) as
                      libc::c_uint ==
                      DNS_Success as libc::c_int as libc::c_uint {
            bits_to_mask(-(1 as libc::c_int),
                         (*address).family as libc::c_int, mask);
            return 1 as libc::c_int
        } else {
            LOG_Message(LOGS_ERR,
                        b"Could not get address for hostname\x00" as *const u8
                            as *const libc::c_char);
            return 0 as libc::c_int
        }
    }
    LOG_Message(LOGS_ERR,
                b"Invalid syntax for mask/address\x00" as *const u8 as
                    *const libc::c_char);
    return 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_offline(mut msg: *mut CMD_Request,
                                         mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut mask: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut address: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut ok: libc::c_int = 0;
    if read_mask_address(line, &mut mask, &mut address) != 0 {
        UTI_IPHostToNetwork(&mut mask, &mut (*msg).data.offline.mask);
        UTI_IPHostToNetwork(&mut address, &mut (*msg).data.offline.address);
        (*msg).command = htons(2 as libc::c_int as uint16_t);
        ok = 1 as libc::c_int
    } else { ok = 0 as libc::c_int }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_online(mut msg: *mut CMD_Request,
                                        mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut mask: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut address: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut ok: libc::c_int = 0;
    if read_mask_address(line, &mut mask, &mut address) != 0 {
        UTI_IPHostToNetwork(&mut mask, &mut (*msg).data.online.mask);
        UTI_IPHostToNetwork(&mut address, &mut (*msg).data.online.address);
        (*msg).command = htons(1 as libc::c_int as uint16_t);
        ok = 1 as libc::c_int
    } else { ok = 0 as libc::c_int }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_onoffline(mut msg: *mut CMD_Request,
                                           mut line: *mut libc::c_char) {
    (*msg).command = htons(63 as libc::c_int as uint16_t);
}
/* ================================================== */
unsafe extern "C" fn read_address_integer(mut line: *mut libc::c_char,
                                          mut address: *mut IPAddr,
                                          mut value: *mut libc::c_int)
 -> libc::c_int {
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ok: libc::c_int = 0 as libc::c_int;
    hostname = line;
    line = CPS_SplitWord(line);
    if sscanf(line, b"%d\x00" as *const u8 as *const libc::c_char, value) !=
           1 as libc::c_int {
        LOG_Message(LOGS_ERR,
                    b"Invalid syntax for address value\x00" as *const u8 as
                        *const libc::c_char);
        ok = 0 as libc::c_int
    } else if DNS_Name2IPAddress(hostname, address, 1 as libc::c_int) as
                  libc::c_uint != DNS_Success as libc::c_int as libc::c_uint {
        LOG_Message(LOGS_ERR,
                    b"Could not get address for hostname\x00" as *const u8 as
                        *const libc::c_char);
        ok = 0 as libc::c_int
    } else { ok = 1 as libc::c_int }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn read_address_double(mut line: *mut libc::c_char,
                                         mut address: *mut IPAddr,
                                         mut value: *mut libc::c_double)
 -> libc::c_int {
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ok: libc::c_int = 0 as libc::c_int;
    hostname = line;
    line = CPS_SplitWord(line);
    if sscanf(line, b"%lf\x00" as *const u8 as *const libc::c_char, value) !=
           1 as libc::c_int {
        LOG_Message(LOGS_ERR,
                    b"Invalid syntax for address value\x00" as *const u8 as
                        *const libc::c_char);
        ok = 0 as libc::c_int
    } else if DNS_Name2IPAddress(hostname, address, 1 as libc::c_int) as
                  libc::c_uint != DNS_Success as libc::c_int as libc::c_uint {
        LOG_Message(LOGS_ERR,
                    b"Could not get address for hostname\x00" as *const u8 as
                        *const libc::c_char);
        ok = 0 as libc::c_int
    } else { ok = 1 as libc::c_int }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_minpoll(mut msg: *mut CMD_Request,
                                         mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut address: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut minpoll: libc::c_int = 0;
    let mut ok: libc::c_int = 0;
    if read_address_integer(line, &mut address, &mut minpoll) != 0 {
        UTI_IPHostToNetwork(&mut address,
                            &mut (*msg).data.modify_minpoll.address);
        (*msg).data.modify_minpoll.new_minpoll =
            htonl(minpoll as uint32_t) as int32_t;
        (*msg).command = htons(4 as libc::c_int as uint16_t);
        ok = 1 as libc::c_int
    } else { ok = 0 as libc::c_int }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_maxpoll(mut msg: *mut CMD_Request,
                                         mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut address: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut maxpoll: libc::c_int = 0;
    let mut ok: libc::c_int = 0;
    if read_address_integer(line, &mut address, &mut maxpoll) != 0 {
        UTI_IPHostToNetwork(&mut address,
                            &mut (*msg).data.modify_maxpoll.address);
        (*msg).data.modify_maxpoll.new_maxpoll =
            htonl(maxpoll as uint32_t) as int32_t;
        (*msg).command = htons(5 as libc::c_int as uint16_t);
        ok = 1 as libc::c_int
    } else { ok = 0 as libc::c_int }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_maxdelay(mut msg: *mut CMD_Request,
                                          mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut address: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut max_delay: libc::c_double = 0.;
    let mut ok: libc::c_int = 0;
    if read_address_double(line, &mut address, &mut max_delay) != 0 {
        UTI_IPHostToNetwork(&mut address,
                            &mut (*msg).data.modify_maxdelay.address);
        (*msg).data.modify_maxdelay.new_max_delay =
            UTI_FloatHostToNetwork(max_delay);
        (*msg).command = htons(7 as libc::c_int as uint16_t);
        ok = 1 as libc::c_int
    } else { ok = 0 as libc::c_int }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_maxdelaydevratio(mut msg: *mut CMD_Request,
                                                  mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut address: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut max_delay_dev_ratio: libc::c_double = 0.;
    let mut ok: libc::c_int = 0;
    if read_address_double(line, &mut address, &mut max_delay_dev_ratio) != 0
       {
        UTI_IPHostToNetwork(&mut address,
                            &mut (*msg).data.modify_maxdelaydevratio.address);
        (*msg).data.modify_maxdelayratio.new_max_delay_ratio =
            UTI_FloatHostToNetwork(max_delay_dev_ratio);
        (*msg).command = htons(47 as libc::c_int as uint16_t);
        ok = 1 as libc::c_int
    } else { ok = 0 as libc::c_int }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_maxdelayratio(mut msg: *mut CMD_Request,
                                               mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut address: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut max_delay_ratio: libc::c_double = 0.;
    let mut ok: libc::c_int = 0;
    if read_address_double(line, &mut address, &mut max_delay_ratio) != 0 {
        UTI_IPHostToNetwork(&mut address,
                            &mut (*msg).data.modify_maxdelayratio.address);
        (*msg).data.modify_maxdelayratio.new_max_delay_ratio =
            UTI_FloatHostToNetwork(max_delay_ratio);
        (*msg).command = htons(8 as libc::c_int as uint16_t);
        ok = 1 as libc::c_int
    } else { ok = 0 as libc::c_int }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_minstratum(mut msg: *mut CMD_Request,
                                            mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut address: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut min_stratum: libc::c_int = 0;
    let mut ok: libc::c_int = 0;
    if read_address_integer(line, &mut address, &mut min_stratum) != 0 {
        UTI_IPHostToNetwork(&mut address,
                            &mut (*msg).data.modify_minstratum.address);
        (*msg).data.modify_minstratum.new_min_stratum =
            htonl(min_stratum as uint32_t) as int32_t;
        (*msg).command = htons(45 as libc::c_int as uint16_t);
        ok = 1 as libc::c_int
    } else { ok = 0 as libc::c_int }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_polltarget(mut msg: *mut CMD_Request,
                                            mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut address: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut poll_target: libc::c_int = 0;
    let mut ok: libc::c_int = 0;
    if read_address_integer(line, &mut address, &mut poll_target) != 0 {
        UTI_IPHostToNetwork(&mut address,
                            &mut (*msg).data.modify_polltarget.address);
        (*msg).data.modify_polltarget.new_poll_target =
            htonl(poll_target as uint32_t) as int32_t;
        (*msg).command = htons(46 as libc::c_int as uint16_t);
        ok = 1 as libc::c_int
    } else { ok = 0 as libc::c_int }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_maxupdateskew(mut msg: *mut CMD_Request,
                                               mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut ok: libc::c_int = 0;
    let mut new_max_update_skew: libc::c_double = 0.;
    if sscanf(line, b"%lf\x00" as *const u8 as *const libc::c_char,
              &mut new_max_update_skew as *mut libc::c_double) ==
           1 as libc::c_int {
        (*msg).data.modify_maxupdateskew.new_max_update_skew =
            UTI_FloatHostToNetwork(new_max_update_skew);
        (*msg).command = htons(9 as libc::c_int as uint16_t);
        ok = 1 as libc::c_int
    } else { ok = 0 as libc::c_int }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_dump(mut msg: *mut CMD_Request,
                                      mut line: *mut libc::c_char) {
    (*msg).command = htons(6 as libc::c_int as uint16_t);
    (*msg).data.dump.pad = htonl(0 as libc::c_int as uint32_t) as int32_t;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_writertc(mut msg: *mut CMD_Request,
                                          mut line: *mut libc::c_char) {
    (*msg).command = htons(30 as libc::c_int as uint16_t);
}
/* ================================================== */
unsafe extern "C" fn process_cmd_trimrtc(mut msg: *mut CMD_Request,
                                         mut line: *mut libc::c_char) {
    (*msg).command = htons(36 as libc::c_int as uint16_t);
}
/* ================================================== */
unsafe extern "C" fn process_cmd_cyclelogs(mut msg: *mut CMD_Request,
                                           mut line: *mut libc::c_char) {
    (*msg).command = htons(37 as libc::c_int as uint16_t);
}
/* ================================================== */
unsafe extern "C" fn process_cmd_burst(mut msg: *mut CMD_Request,
                                       mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut n_good_samples: libc::c_int = 0;
    let mut n_total_samples: libc::c_int = 0;
    let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut address: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut mask: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    s1 = line;
    s2 = CPS_SplitWord(s1);
    CPS_SplitWord(s2);
    if sscanf(s1, b"%d/%d\x00" as *const u8 as *const libc::c_char,
              &mut n_good_samples as *mut libc::c_int,
              &mut n_total_samples as *mut libc::c_int) != 2 as libc::c_int {
        LOG_Message(LOGS_ERR,
                    b"Invalid syntax for burst command\x00" as *const u8 as
                        *const libc::c_char);
        return 0 as libc::c_int
    }
    address.family = 0 as libc::c_int as uint16_t;
    mask.family = address.family;
    if *s2 as libc::c_int != 0 &&
           read_mask_address(s2, &mut mask, &mut address) == 0 {
        return 0 as libc::c_int
    }
    (*msg).command = htons(3 as libc::c_int as uint16_t);
    (*msg).data.burst.n_good_samples =
        ntohl(n_good_samples as uint32_t) as int32_t;
    (*msg).data.burst.n_total_samples =
        ntohl(n_total_samples as uint32_t) as int32_t;
    UTI_IPHostToNetwork(&mut mask, &mut (*msg).data.burst.mask);
    UTI_IPHostToNetwork(&mut address, &mut (*msg).data.burst.address);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_local(mut msg: *mut CMD_Request,
                                       mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut on_off: libc::c_int = 0;
    let mut stratum: libc::c_int = 0 as libc::c_int;
    let mut orphan: libc::c_int = 0 as libc::c_int;
    let mut distance: libc::c_double = 0.0f64;
    if strcmp(line, b"off\x00" as *const u8 as *const libc::c_char) == 0 {
        on_off = 0 as libc::c_int
    } else if CPS_ParseLocal(line, &mut stratum, &mut orphan, &mut distance)
                  != 0 {
        on_off = 1 as libc::c_int
    } else {
        LOG_Message(LOGS_ERR,
                    b"Invalid syntax for local command\x00" as *const u8 as
                        *const libc::c_char);
        return 0 as libc::c_int
    }
    (*msg).command = htons(56 as libc::c_int as uint16_t);
    (*msg).data.local.on_off = htonl(on_off as uint32_t) as int32_t;
    (*msg).data.local.stratum = htonl(stratum as uint32_t) as int32_t;
    (*msg).data.local.distance = UTI_FloatHostToNetwork(distance);
    (*msg).data.local.orphan = htonl(orphan as uint32_t) as int32_t;
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_manual(mut msg: *mut CMD_Request,
                                        mut line: *const libc::c_char)
 -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = line;
    if strcmp(p, b"off\x00" as *const u8 as *const libc::c_char) == 0 {
        (*msg).data.manual.option =
            htonl(0 as libc::c_int as uint32_t) as int32_t
    } else if strcmp(p, b"on\x00" as *const u8 as *const libc::c_char) == 0 {
        (*msg).data.manual.option =
            htonl(1 as libc::c_int as uint32_t) as int32_t
    } else if strcmp(p, b"reset\x00" as *const u8 as *const libc::c_char) == 0
     {
        (*msg).data.manual.option =
            htonl(2 as libc::c_int as uint32_t) as int32_t
    } else {
        LOG_Message(LOGS_ERR,
                    b"Invalid syntax for manual command\x00" as *const u8 as
                        *const libc::c_char);
        return 0 as libc::c_int
    }
    (*msg).command = htons(13 as libc::c_int as uint16_t);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn parse_allow_deny(mut msg: *mut CMD_Request,
                                      mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut a: libc::c_ulong = 0;
    let mut b: libc::c_ulong = 0;
    let mut c: libc::c_ulong = 0;
    let mut d: libc::c_ulong = 0;
    let mut n: libc::c_int = 0;
    let mut specified_subnet_bits: libc::c_int = 0;
    let mut ip: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = line;
    if *p == 0 {
        /* blank line - applies to all addresses */
        ip.family = 0 as libc::c_int as uint16_t;
        UTI_IPHostToNetwork(&mut ip, &mut (*msg).data.allow_deny.ip);
        (*msg).data.allow_deny.subnet_bits =
            htonl(0 as libc::c_int as uint32_t) as int32_t
    } else {
        let mut slashpos: *mut libc::c_char = 0 as *mut libc::c_char;
        slashpos = strchr(p, '/' as i32);
        if !slashpos.is_null() {
            *slashpos = 0 as libc::c_int as libc::c_char
        }
        n = 0 as libc::c_int;
        if UTI_StringToIP(p, &mut ip) == 0 &&
               {
                   n =
                       sscanf(p,
                              b"%lu.%lu.%lu.%lu\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut a as *mut libc::c_ulong,
                              &mut b as *mut libc::c_ulong,
                              &mut c as *mut libc::c_ulong,
                              &mut d as *mut libc::c_ulong);
                   (n) <= 0 as libc::c_int
               } {
            /* Try to parse as the name of a machine */
            if !slashpos.is_null() ||
                   DNS_Name2IPAddress(p, &mut ip, 1 as libc::c_int) as
                       libc::c_uint !=
                       DNS_Success as libc::c_int as libc::c_uint {
                LOG_Message(LOGS_ERR,
                            b"Could not read address\x00" as *const u8 as
                                *const libc::c_char);
                return 0 as libc::c_int
            } else {
                UTI_IPHostToNetwork(&mut ip, &mut (*msg).data.allow_deny.ip);
                if ip.family as libc::c_int == 2 as libc::c_int {
                    (*msg).data.allow_deny.subnet_bits =
                        htonl(128 as libc::c_int as uint32_t) as int32_t
                } else {
                    (*msg).data.allow_deny.subnet_bits =
                        htonl(32 as libc::c_int as uint32_t) as int32_t
                }
            }
        } else {
            if n == 0 as libc::c_int {
                if ip.family as libc::c_int == 2 as libc::c_int {
                    (*msg).data.allow_deny.subnet_bits =
                        htonl(128 as libc::c_int as uint32_t) as int32_t
                } else {
                    (*msg).data.allow_deny.subnet_bits =
                        htonl(32 as libc::c_int as uint32_t) as int32_t
                }
            } else {
                ip.family = 1 as libc::c_int as uint16_t;
                a &= 0xff as libc::c_int as libc::c_ulong;
                b &= 0xff as libc::c_int as libc::c_ulong;
                c &= 0xff as libc::c_int as libc::c_ulong;
                d &= 0xff as libc::c_int as libc::c_ulong;
                match n {
                    1 => {
                        ip.addr.in4 =
                            htonl((a << 24 as libc::c_int) as uint32_t);
                        (*msg).data.allow_deny.subnet_bits =
                            htonl(8 as libc::c_int as uint32_t) as int32_t
                    }
                    2 => {
                        ip.addr.in4 =
                            htonl((a << 24 as libc::c_int |
                                       b << 16 as libc::c_int) as uint32_t);
                        (*msg).data.allow_deny.subnet_bits =
                            htonl(16 as libc::c_int as uint32_t) as int32_t
                    }
                    3 => {
                        ip.addr.in4 =
                            htonl((a << 24 as libc::c_int |
                                       b << 16 as libc::c_int |
                                       c << 8 as libc::c_int) as uint32_t);
                        (*msg).data.allow_deny.subnet_bits =
                            htonl(24 as libc::c_int as uint32_t) as int32_t
                    }
                    4 => {
                        ip.addr.in4 =
                            htonl((a << 24 as libc::c_int |
                                       b << 16 as libc::c_int |
                                       c << 8 as libc::c_int | d) as
                                      uint32_t);
                        (*msg).data.allow_deny.subnet_bits =
                            htonl(32 as libc::c_int as uint32_t) as int32_t
                    }
                    _ => {
                        __assert_fail(b"0\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"client.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      836 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 44],
                                                                &[libc::c_char; 44]>(b"int parse_allow_deny(CMD_Request *, char *)\x00")).as_ptr());
                    }
                }
            }
            UTI_IPHostToNetwork(&mut ip, &mut (*msg).data.allow_deny.ip);
            if !slashpos.is_null() {
                n =
                    sscanf(slashpos.offset(1 as libc::c_int as isize),
                           b"%d\x00" as *const u8 as *const libc::c_char,
                           &mut specified_subnet_bits as *mut libc::c_int);
                if n == 1 as libc::c_int {
                    (*msg).data.allow_deny.subnet_bits =
                        htonl(specified_subnet_bits as uint32_t) as int32_t
                } else {
                    LOG_Message(LOGS_WARN,
                                b"Warning: badly formatted subnet size, using %d\x00"
                                    as *const u8 as *const libc::c_char,
                                ntohl((*msg).data.allow_deny.subnet_bits as
                                          uint32_t) as libc::c_int);
                }
            }
        }
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_allow(mut msg: *mut CMD_Request,
                                       mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut status: libc::c_int = 0;
    (*msg).command = htons(17 as libc::c_int as uint16_t);
    status = parse_allow_deny(msg, line);
    return status;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_allowall(mut msg: *mut CMD_Request,
                                          mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut status: libc::c_int = 0;
    (*msg).command = htons(18 as libc::c_int as uint16_t);
    status = parse_allow_deny(msg, line);
    return status;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_deny(mut msg: *mut CMD_Request,
                                      mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut status: libc::c_int = 0;
    (*msg).command = htons(19 as libc::c_int as uint16_t);
    status = parse_allow_deny(msg, line);
    return status;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_denyall(mut msg: *mut CMD_Request,
                                         mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut status: libc::c_int = 0;
    (*msg).command = htons(20 as libc::c_int as uint16_t);
    status = parse_allow_deny(msg, line);
    return status;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_cmdallow(mut msg: *mut CMD_Request,
                                          mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut status: libc::c_int = 0;
    (*msg).command = htons(21 as libc::c_int as uint16_t);
    status = parse_allow_deny(msg, line);
    return status;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_cmdallowall(mut msg: *mut CMD_Request,
                                             mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut status: libc::c_int = 0;
    (*msg).command = htons(22 as libc::c_int as uint16_t);
    status = parse_allow_deny(msg, line);
    return status;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_cmddeny(mut msg: *mut CMD_Request,
                                         mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut status: libc::c_int = 0;
    (*msg).command = htons(23 as libc::c_int as uint16_t);
    status = parse_allow_deny(msg, line);
    return status;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_cmddenyall(mut msg: *mut CMD_Request,
                                            mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut status: libc::c_int = 0;
    (*msg).command = htons(24 as libc::c_int as uint16_t);
    status = parse_allow_deny(msg, line);
    return status;
}
/* ================================================== */
unsafe extern "C" fn accheck_getaddr(mut line: *mut libc::c_char,
                                     mut addr: *mut IPAddr) -> libc::c_int {
    let mut a: libc::c_ulong = 0;
    let mut b: libc::c_ulong = 0;
    let mut c: libc::c_ulong = 0;
    let mut d: libc::c_ulong = 0;
    let mut ip: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = line;
    if *p == 0 {
        return 0 as libc::c_int
    } else if sscanf(p,
                     b"%lu.%lu.%lu.%lu\x00" as *const u8 as
                         *const libc::c_char, &mut a as *mut libc::c_ulong,
                     &mut b as *mut libc::c_ulong,
                     &mut c as *mut libc::c_ulong,
                     &mut d as *mut libc::c_ulong) == 4 as libc::c_int {
        (*addr).family = 1 as libc::c_int as uint16_t;
        (*addr).addr.in4 =
            (a << 24 as libc::c_int | b << 16 as libc::c_int |
                 c << 8 as libc::c_int | d) as uint32_t;
        return 1 as libc::c_int
    } else if DNS_Name2IPAddress(p, &mut ip, 1 as libc::c_int) as libc::c_uint
                  != DNS_Success as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    } else { *addr = ip; return 1 as libc::c_int };
}
/* ================================================== */
unsafe extern "C" fn process_cmd_accheck(mut msg: *mut CMD_Request,
                                         mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut ip: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    (*msg).command = htons(25 as libc::c_int as uint16_t);
    if accheck_getaddr(line, &mut ip) != 0 {
        UTI_IPHostToNetwork(&mut ip, &mut (*msg).data.ac_check.ip);
        return 1 as libc::c_int
    } else {
        LOG_Message(LOGS_ERR,
                    b"Could not read address\x00" as *const u8 as
                        *const libc::c_char);
        return 0 as libc::c_int
    };
}
/* ================================================== */
unsafe extern "C" fn process_cmd_cmdaccheck(mut msg: *mut CMD_Request,
                                            mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut ip: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    (*msg).command = htons(26 as libc::c_int as uint16_t);
    if accheck_getaddr(line, &mut ip) != 0 {
        UTI_IPHostToNetwork(&mut ip, &mut (*msg).data.ac_check.ip);
        return 1 as libc::c_int
    } else {
        LOG_Message(LOGS_ERR,
                    b"Could not read address\x00" as *const u8 as
                        *const libc::c_char);
        return 0 as libc::c_int
    };
}
/* ================================================== */
unsafe extern "C" fn process_cmd_dfreq(mut msg: *mut CMD_Request,
                                       mut line: *mut libc::c_char) {
    let mut dfreq: libc::c_double = 0.;
    (*msg).command = htons(31 as libc::c_int as uint16_t);
    if sscanf(line, b"%lf\x00" as *const u8 as *const libc::c_char,
              &mut dfreq as *mut libc::c_double) == 1 as libc::c_int {
        (*msg).data.dfreq.dfreq = UTI_FloatHostToNetwork(dfreq)
    } else { (*msg).data.dfreq.dfreq = UTI_FloatHostToNetwork(0.0f64) };
}
/* ================================================== */
unsafe extern "C" fn cvt_to_sec_usec(mut x: libc::c_double,
                                     mut sec: *mut libc::c_long,
                                     mut usec: *mut libc::c_long) {
    let mut s: libc::c_long = 0;
    let mut us: libc::c_long = 0;
    s = x as libc::c_long;
    us = (0.5f64 + 1.0e6f64 * (x - s as libc::c_double)) as libc::c_long;
    while us >= 1000000 as libc::c_int as libc::c_long {
        us -= 1000000 as libc::c_int as libc::c_long;
        s += 1 as libc::c_int as libc::c_long
    }
    while us < 0 as libc::c_int as libc::c_long {
        us += 1000000 as libc::c_int as libc::c_long;
        s -= 1 as libc::c_int as libc::c_long
    }
    *sec = s;
    *usec = us;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_doffset(mut msg: *mut CMD_Request,
                                         mut line: *mut libc::c_char) {
    let mut doffset: libc::c_double = 0.;
    let mut sec: libc::c_long = 0;
    let mut usec: libc::c_long = 0;
    (*msg).command = htons(32 as libc::c_int as uint16_t);
    if sscanf(line, b"%lf\x00" as *const u8 as *const libc::c_char,
              &mut doffset as *mut libc::c_double) == 1 as libc::c_int {
        cvt_to_sec_usec(doffset, &mut sec, &mut usec);
        (*msg).data.doffset.sec = htonl(sec as uint32_t) as int32_t;
        (*msg).data.doffset.usec = htonl(usec as uint32_t) as int32_t
    } else {
        (*msg).data.doffset.sec =
            htonl(0 as libc::c_int as uint32_t) as int32_t;
        (*msg).data.doffset.usec =
            htonl(0 as libc::c_int as uint32_t) as int32_t
    };
}
/* ================================================== */
unsafe extern "C" fn process_cmd_add_source(mut msg: *mut CMD_Request,
                                            mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut data: CPS_NTP_Source =
        CPS_NTP_Source{name: 0 as *mut libc::c_char,
                       port: 0,
                       params:
                           SourceParameters{minpoll: 0,
                                            maxpoll: 0,
                                            connectivity: SRC_OFFLINE,
                                            auto_offline: 0,
                                            presend_minpoll: 0,
                                            burst: 0,
                                            iburst: 0,
                                            min_stratum: 0,
                                            poll_target: 0,
                                            version: 0,
                                            max_sources: 0,
                                            min_samples: 0,
                                            max_samples: 0,
                                            filter_length: 0,
                                            interleaved: 0,
                                            sel_options: 0,
                                            authkey: 0,
                                            max_delay: 0.,
                                            max_delay_ratio: 0.,
                                            max_delay_dev_ratio: 0.,
                                            min_delay: 0.,
                                            asymmetry: 0.,
                                            offset: 0.,},};
    let mut ip_addr: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut status: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut opt_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut word: *const libc::c_char = 0 as *const libc::c_char;
    (*msg).command = htons(64 as libc::c_int as uint16_t);
    word = line;
    line = CPS_SplitWord(line);
    if strcmp(word, b"server\x00" as *const u8 as *const libc::c_char) == 0 {
        type_0 = 1 as libc::c_int
    } else if strcmp(word, b"peer\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        type_0 = 2 as libc::c_int
    } else if strcmp(word, b"pool\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        type_0 = 3 as libc::c_int
    } else {
        LOG_Message(LOGS_ERR,
                    b"Invalid syntax for add command\x00" as *const u8 as
                        *const libc::c_char);
        return 0 as libc::c_int
    }
    status = CPS_ParseNTPSourceAdd(line, &mut data);
    match status {
        0 => {
            LOG_Message(LOGS_ERR,
                        b"Invalid syntax for add command\x00" as *const u8 as
                            *const libc::c_char);
        }
        _ => {
            /* Verify that the address is resolvable (chronyc and chronyd are
         assumed to be running on the same host) */
            if strlen(data.name) >=
                   ::std::mem::size_of::<[int8_t; 256]>() as libc::c_ulong ||
                   DNS_Name2IPAddress(data.name, &mut ip_addr,
                                      1 as libc::c_int) as libc::c_uint !=
                       DNS_Success as libc::c_int as libc::c_uint {
                LOG_Message(LOGS_ERR,
                            b"Invalid host/IP address\x00" as *const u8 as
                                *const libc::c_char);
            } else {
                opt_name = 0 as *const libc::c_char;
                if !opt_name.is_null() {
                    LOG_Message(LOGS_ERR,
                                b"%s can\'t be set in chronyc\x00" as
                                    *const u8 as *const libc::c_char,
                                opt_name);
                } else {
                    (*msg).data.ntp_source.type_0 = htonl(type_0 as uint32_t);
                    if strlen(data.name) >=
                           ::std::mem::size_of::<[int8_t; 256]>() as
                               libc::c_ulong {
                        __assert_fail(b"0\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"client.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1103 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 50],
                                                                &[libc::c_char; 50]>(b"int process_cmd_add_source(CMD_Request *, char *)\x00")).as_ptr());
                    }
                    strncpy((*msg).data.ntp_source.name.as_mut_ptr() as
                                *mut libc::c_char, data.name,
                            ::std::mem::size_of::<[int8_t; 256]>() as
                                libc::c_ulong);
                    (*msg).data.ntp_source.port =
                        htonl(data.port as libc::c_ulong as uint32_t);
                    (*msg).data.ntp_source.minpoll =
                        htonl(data.params.minpoll as uint32_t) as int32_t;
                    (*msg).data.ntp_source.maxpoll =
                        htonl(data.params.maxpoll as uint32_t) as int32_t;
                    (*msg).data.ntp_source.presend_minpoll =
                        htonl(data.params.presend_minpoll as uint32_t) as
                            int32_t;
                    (*msg).data.ntp_source.min_stratum =
                        htonl(data.params.min_stratum as uint32_t);
                    (*msg).data.ntp_source.poll_target =
                        htonl(data.params.poll_target as uint32_t);
                    (*msg).data.ntp_source.version =
                        htonl(data.params.version as uint32_t);
                    (*msg).data.ntp_source.max_sources =
                        htonl(data.params.max_sources as uint32_t);
                    (*msg).data.ntp_source.min_samples =
                        htonl(data.params.min_samples as uint32_t) as int32_t;
                    (*msg).data.ntp_source.max_samples =
                        htonl(data.params.max_samples as uint32_t) as int32_t;
                    (*msg).data.ntp_source.authkey =
                        htonl(data.params.authkey);
                    (*msg).data.ntp_source.max_delay =
                        UTI_FloatHostToNetwork(data.params.max_delay);
                    (*msg).data.ntp_source.max_delay_ratio =
                        UTI_FloatHostToNetwork(data.params.max_delay_ratio);
                    (*msg).data.ntp_source.max_delay_dev_ratio =
                        UTI_FloatHostToNetwork(data.params.max_delay_dev_ratio);
                    (*msg).data.ntp_source.min_delay =
                        UTI_FloatHostToNetwork(data.params.min_delay);
                    (*msg).data.ntp_source.asymmetry =
                        UTI_FloatHostToNetwork(data.params.asymmetry);
                    (*msg).data.ntp_source.offset =
                        UTI_FloatHostToNetwork(data.params.offset);
                    (*msg).data.ntp_source.flags =
                        htonl(((if data.params.connectivity as libc::c_uint ==
                                       SRC_ONLINE as libc::c_int as
                                           libc::c_uint {
                                    0x1 as libc::c_int
                                } else { 0 as libc::c_int }) |
                                   (if data.params.auto_offline != 0 {
                                        0x2 as libc::c_int
                                    } else { 0 as libc::c_int }) |
                                   (if data.params.iburst != 0 {
                                        0x4 as libc::c_int
                                    } else { 0 as libc::c_int }) |
                                   (if data.params.interleaved != 0 {
                                        0x80 as libc::c_int
                                    } else { 0 as libc::c_int }) |
                                   (if data.params.burst != 0 {
                                        0x100 as libc::c_int
                                    } else { 0 as libc::c_int }) |
                                   (if data.params.sel_options &
                                           0x2 as libc::c_int != 0 {
                                        0x8 as libc::c_int
                                    } else { 0 as libc::c_int }) |
                                   (if data.params.sel_options &
                                           0x1 as libc::c_int != 0 {
                                        0x10 as libc::c_int
                                    } else { 0 as libc::c_int }) |
                                   (if data.params.sel_options &
                                           0x4 as libc::c_int != 0 {
                                        0x20 as libc::c_int
                                    } else { 0 as libc::c_int }) |
                                   (if data.params.sel_options &
                                           0x8 as libc::c_int != 0 {
                                        0x40 as libc::c_int
                                    } else { 0 as libc::c_int })) as
                                  uint32_t);
                    (*msg).data.ntp_source.filter_length =
                        htonl(data.params.filter_length as uint32_t) as
                            int32_t;
                    memset((*msg).data.ntp_source.reserved.as_mut_ptr() as
                               *mut libc::c_void, 0 as libc::c_int,
                           ::std::mem::size_of::<[uint32_t; 3]>() as
                               libc::c_ulong);
                    result = 1 as libc::c_int
                }
            }
        }
    }
    return result;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_delete(mut msg: *mut CMD_Request,
                                        mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut address: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    (*msg).command = htons(29 as libc::c_int as uint16_t);
    hostname = line;
    CPS_SplitWord(line);
    if *hostname == 0 {
        LOG_Message(LOGS_ERR,
                    b"Invalid syntax for address\x00" as *const u8 as
                        *const libc::c_char);
        ok = 0 as libc::c_int
    } else if DNS_Name2IPAddress(hostname, &mut address, 1 as libc::c_int) as
                  libc::c_uint != DNS_Success as libc::c_int as libc::c_uint {
        LOG_Message(LOGS_ERR,
                    b"Could not get address for hostname\x00" as *const u8 as
                        *const libc::c_char);
        ok = 0 as libc::c_int
    } else {
        UTI_IPHostToNetwork(&mut address,
                            &mut (*msg).data.del_source.ip_addr);
        ok = 1 as libc::c_int
    }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn give_help() {
    let mut line: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let cols: [libc::c_char; 3538] =
        *::std::mem::transmute::<&[u8; 3538],
                                 &[libc::c_char; 3538]>(b"System clock:\x00\x00tracking\x00Display system time information\x00makestep\x00Correct clock by stepping immediately\x00makestep <threshold> <updates>\x00Configure automatic clock stepping\x00maxupdateskew <skew>\x00Modify maximum valid skew to update frequency\x00waitsync [<max-tries> [<max-correction> [<max-skew> [<interval>]]]]\x00Wait until synchronised in specified limits\x00\x00\x00Time sources:\x00\x00sources [-v]\x00Display information about current sources\x00sourcestats [-v]\x00Display statistics about collected measurements\x00reselect\x00Force reselecting synchronisation source\x00reselectdist <dist>\x00Modify reselection distance\x00\x00\x00NTP sources:\x00\x00activity\x00Check how many NTP sources are online/offline\x00ntpdata [<address>]\x00Display information about last valid measurement\x00add server <name> [options]\x00Add new NTP server\x00add pool <name> [options]\x00Add new pool of NTP servers\x00add peer <name> [options]\x00Add new NTP peer\x00delete <address>\x00Remove server or peer\x00burst <n-good>/<n-max> [<mask>/<address>]\x00Start rapid set of measurements\x00maxdelay <address> <delay>\x00Modify maximum valid sample delay\x00maxdelayratio <address> <ratio>\x00Modify maximum valid delay/minimum ratio\x00maxdelaydevratio <address> <ratio>\x00Modify maximum valid delay/deviation ratio\x00minpoll <address> <poll>\x00Modify minimum polling interval\x00maxpoll <address> <poll>\x00Modify maximum polling interval\x00minstratum <address> <stratum>\x00Modify minimum stratum\x00offline [<mask>/<address>]\x00Set sources in subnet to offline status\x00online [<mask>/<address>]\x00Set sources in subnet to online status\x00onoffline\x00Set all sources to online or offline status\x00\x00according to network configuration\x00polltarget <address> <target>\x00Modify poll target\x00refresh\x00Refresh IP addresses\x00sourcename <address>\x00Display original name\x00\x00\x00Manual time input:\x00\x00manual off|on|reset\x00Disable/enable/reset settime command\x00manual list\x00Show previous settime entries\x00manual delete <index>\x00Delete previous settime entry\x00settime <time>\x00Set daemon time\x00\x00(e.g. Sep 25, 2015 16:30:05 or 16:30:05)\x00\x00\x00NTP access:\x00\x00accheck <address>\x00Check whether address is allowed\x00clients\x00Report on clients that have accessed the server\x00serverstats\x00Display statistics of the server\x00allow [<subnet>]\x00Allow access to subnet as a default\x00allow all [<subnet>]\x00Allow access to subnet and all children\x00deny [<subnet>]\x00Deny access to subnet as a default\x00deny all [<subnet>]\x00Deny access to subnet and all children\x00local [options]\x00Serve time even when not synchronised\x00local off\x00Don\'t serve time when not synchronised\x00smoothtime reset|activate\x00Reset/activate time smoothing\x00smoothing\x00Display current time smoothing state\x00\x00\x00Monitoring access:\x00\x00cmdaccheck <address>\x00Check whether address is allowed\x00cmdallow [<subnet>]\x00Allow access to subnet as a default\x00cmdallow all [<subnet>]\x00Allow access to subnet and all children\x00cmddeny [<subnet>]\x00Deny access to subnet as a default\x00cmddeny all [<subnet>]\x00Deny access to subnet and all children\x00\x00\x00Real-time clock:\x00\x00rtcdata\x00Print current RTC performance parameters\x00trimrtc\x00Correct RTC relative to system clock\x00writertc\x00Save RTC performance parameters to file\x00\x00\x00Other daemon commands:\x00\x00cyclelogs\x00Close and re-open log files\x00dump\x00Dump all measurements to save files\x00rekey\x00Re-read keys from key file\x00shutdown\x00Stop daemon\x00\x00\x00Client commands:\x00\x00dns -n|+n\x00Disable/enable resolving IP addresses to hostnames\x00dns -4|-6|-46\x00Resolve hostnames only to IPv4/IPv6/both addresses\x00timeout <milliseconds>\x00Set initial response timeout\x00retries <retries>\x00Set maximum number of retries\x00keygen [<id> [<type> [<bits>]]]\x00Generate key for key file\x00exit|quit\x00Leave the program\x00help\x00Generate this help\x00\x00\x00");
    /* Indent the second column */
    s = cols.as_ptr();
    line = 0 as libc::c_int;
    while s <
              cols.as_ptr().offset(::std::mem::size_of::<[libc::c_char; 3538]>()
                                       as libc::c_ulong as isize) {
        len = strlen(s) as libc::c_int;
        printf(if line % 2 as libc::c_int == 0 as libc::c_int {
                   if len >= 28 as libc::c_int {
                       b"%s\n%28s\x00" as *const u8 as *const libc::c_char
                   } else {
                       b"%-28s%s\x00" as *const u8 as *const libc::c_char
                   }
               } else { b"%s%s\n\x00" as *const u8 as *const libc::c_char },
               s, b"\x00" as *const u8 as *const libc::c_char);
        s = s.offset((len + 1 as libc::c_int) as isize);
        line += 1
    };
}
/* ================================================== */
/* Tab-completion when editline/readline is available */
/* ================================================== */
static mut max_retries: libc::c_int = 2 as libc::c_int;
static mut initial_timeout: libc::c_int = 1000 as libc::c_int;
static mut proto_version: libc::c_int = 6 as libc::c_int;
/* This is the core protocol module.  Complete particular fields in
   the outgoing packet, send it, wait for a response, handle retries,
   etc.  Returns a Boolean indicating whether the protocol was
   successful or not.*/
unsafe extern "C" fn submit_request(mut request: *mut CMD_Request,
                                    mut reply: *mut CMD_Reply)
 -> libc::c_int {
    let mut select_status: libc::c_int = 0;
    let mut recv_status: libc::c_int = 0;
    let mut read_length: libc::c_int = 0;
    let mut command_length: libc::c_int = 0;
    let mut padding_length: libc::c_int = 0;
    let mut ts_now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut ts_start: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut n_attempts: libc::c_int = 0;
    let mut new_attempt: libc::c_int = 0;
    let mut timeout: libc::c_double = 0.;
    let mut rdfd: fd_set = fd_set{fds_bits: [0; 16],};
    (*request).pkt_type = 1 as libc::c_int as uint8_t;
    (*request).res1 = 0 as libc::c_int as uint8_t;
    (*request).res2 = 0 as libc::c_int as uint8_t;
    (*request).pad1 = 0 as libc::c_int as uint32_t;
    (*request).pad2 = 0 as libc::c_int as uint32_t;
    n_attempts = 0 as libc::c_int;
    new_attempt = 1 as libc::c_int;
    loop  {
        if gettimeofday(&mut tv, 0 as *mut timezone) != 0 {
            return 0 as libc::c_int
        }
        if new_attempt != 0 {
            new_attempt = 0 as libc::c_int;
            if n_attempts > max_retries { return 0 as libc::c_int }
            UTI_TimevalToTimespec(&mut tv, &mut ts_start);
            UTI_GetRandomBytes(&mut (*request).sequence as *mut uint32_t as
                                   *mut libc::c_void,
                               ::std::mem::size_of::<uint32_t>() as
                                   libc::c_ulong as libc::c_uint);
            (*request).attempt = htons(n_attempts as uint16_t);
            (*request).version = proto_version as uint8_t;
            command_length = PKL_CommandLength(request);
            padding_length = PKL_CommandPaddingLength(request);
            if command_length > 0 as libc::c_int &&
                   command_length > padding_length {
            } else {
                __assert_fail(b"command_length > 0 && command_length > padding_length\x00"
                                  as *const u8 as *const libc::c_char,
                              b"client.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1410 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 47],
                                                        &[libc::c_char; 47]>(b"int submit_request(CMD_Request *, CMD_Reply *)\x00")).as_ptr());
            }
            n_attempts += 1;
            /* Zero the padding to not send any uninitialized data */
            memset((request as
                        *mut libc::c_char).offset(command_length as
                                                      isize).offset(-(padding_length
                                                                          as
                                                                          isize))
                       as *mut libc::c_void, 0 as libc::c_int,
                   padding_length as libc::c_ulong);
            if sock_fd < 0 as libc::c_int {
                if 0 as libc::c_int != 0 &&
                       log_min_severity as libc::c_int ==
                           LOGS_DEBUG as libc::c_int {
                    LOG_Message(LOGS_DEBUG,
                                b"No socket to send request\x00" as *const u8
                                    as *const libc::c_char);
                }
                return 0 as libc::c_int
            }
            if SCK_Send(sock_fd, request as *mut libc::c_void,
                        command_length as libc::c_uint, 0 as libc::c_int) <
                   0 as libc::c_int {
                return 0 as libc::c_int
            }
        }
        UTI_TimevalToTimespec(&mut tv, &mut ts_now);
        /* Check if the clock wasn't stepped back */
        if UTI_CompareTimespecs(&mut ts_now, &mut ts_start) < 0 as libc::c_int
           {
            ts_start = ts_now
        }
        timeout =
            initial_timeout as libc::c_double / 1000.0f64 *
                ((1 as libc::c_uint) << n_attempts - 1 as libc::c_int) as
                    libc::c_double -
                UTI_DiffTimespecsToDouble(&mut ts_now, &mut ts_start);
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Timeout %f seconds\x00" as *const u8 as
                            *const libc::c_char, timeout);
        }
        /* Avoid calling select() with an invalid timeout */
        if timeout <= 0.0f64 {
            new_attempt = 1 as libc::c_int
        } else {
            UTI_DoubleToTimeval(timeout, &mut tv);
            let mut __d0: libc::c_int = 0;
            let mut __d1: libc::c_int = 0;
            let fresh3 = &mut __d0;
            let fresh4;
            let fresh5 = &mut __d1;
            let fresh6;
            let fresh7 =
                (::std::mem::size_of::<fd_set>() as
                     libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>()
                                                     as libc::c_ulong);
            let fresh8 =
                &mut *rdfd.fds_bits.as_mut_ptr().offset(0 as libc::c_int as
                                                            isize) as
                    *mut __fd_mask;
            asm!("cld; rep; stosq" : "={cx}" (fresh4), "={di}" (fresh6) :
                 "{ax}" (0 as libc::c_int), "0"
                 (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh7)), "1"
                 (c2rust_asm_casts::AsmCast::cast_in(fresh5, fresh8)) :
                 "memory" : "volatile");
            c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh7, fresh4);
            c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh8, fresh6);
            rdfd.fds_bits[(sock_fd /
                               (8 as libc::c_int *
                                    ::std::mem::size_of::<__fd_mask>() as
                                        libc::c_ulong as libc::c_int)) as
                              usize] |=
                ((1 as libc::c_ulong) <<
                     sock_fd %
                         (8 as libc::c_int *
                              ::std::mem::size_of::<__fd_mask>() as
                                  libc::c_ulong as libc::c_int)) as __fd_mask;
            if quit != 0 { return 0 as libc::c_int }
            select_status =
                select(sock_fd + 1 as libc::c_int, &mut rdfd,
                       0 as *mut fd_set, 0 as *mut fd_set, &mut tv);
            if select_status < 0 as libc::c_int {
                if 0 as libc::c_int != 0 &&
                       log_min_severity as libc::c_int ==
                           LOGS_DEBUG as libc::c_int {
                    LOG_Message(LOGS_DEBUG,
                                b"select failed : %s\x00" as *const u8 as
                                    *const libc::c_char,
                                strerror(*__errno_location()));
                }
                return 0 as libc::c_int
            } else if select_status == 0 as libc::c_int {
                /* Timeout must have elapsed, try a resend? */
                new_attempt = 1 as libc::c_int
            } else {
                recv_status =
                    SCK_Receive(sock_fd, reply as *mut libc::c_void,
                                ::std::mem::size_of::<CMD_Reply>() as
                                    libc::c_ulong as libc::c_uint,
                                0 as libc::c_int);
                if recv_status < 0 as libc::c_int {
                    new_attempt = 1 as libc::c_int
                } else {
                    read_length = recv_status;
                    /* Check if the header is valid */
                    if (read_length as libc::c_ulong) < 28 as libc::c_ulong ||
                           (*reply).version as libc::c_int != proto_version &&
                               !((*reply).version as libc::c_int >=
                                     4 as libc::c_int &&
                                     ntohs((*reply).status) as libc::c_int ==
                                         18 as libc::c_int) ||
                           (*reply).pkt_type as libc::c_int !=
                               2 as libc::c_int ||
                           (*reply).res1 as libc::c_int != 0 as libc::c_int ||
                           (*reply).res2 as libc::c_int != 0 as libc::c_int ||
                           (*reply).command as libc::c_int !=
                               (*request).command as libc::c_int ||
                           (*reply).sequence != (*request).sequence {
                        if 0 as libc::c_int != 0 &&
                               log_min_severity as libc::c_int ==
                                   LOGS_DEBUG as libc::c_int {
                            LOG_Message(LOGS_DEBUG,
                                        b"Invalid reply\x00" as *const u8 as
                                            *const libc::c_char);
                        }
                    } else if proto_version == 6 as libc::c_int &&
                                  (*reply).version as libc::c_int ==
                                      6 as libc::c_int - 1 as libc::c_int {
                        proto_version = 6 as libc::c_int - 1 as libc::c_int;
                        n_attempts -= 1;
                        new_attempt = 1 as libc::c_int
                    } else if read_length < PKL_ReplyLength(reply) {
                        if 0 as libc::c_int != 0 &&
                               log_min_severity as libc::c_int ==
                                   LOGS_DEBUG as libc::c_int {
                            LOG_Message(LOGS_DEBUG,
                                        b"Reply too short\x00" as *const u8 as
                                            *const libc::c_char);
                        }
                        new_attempt = 1 as libc::c_int
                    } else {
                        /* Protocol version 5 is similar to 6 except there is no padding.
           If a version 5 reply with STT_BADPKTVERSION is received,
           switch our version and try again. */
                        /* Check that the packet contains all data it is supposed to have.
           Unknown responses will always pass this test as their expected
           length is zero. */
                        /* Good packet received, print out results */
                        if 0 as libc::c_int != 0 &&
                               log_min_severity as libc::c_int ==
                                   LOGS_DEBUG as libc::c_int {
                            LOG_Message(LOGS_DEBUG,
                                        b"Reply cmd=%d reply=%d stat=%d\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        ntohs((*reply).command) as
                                            libc::c_int,
                                        ntohs((*reply).reply) as libc::c_int,
                                        ntohs((*reply).status) as
                                            libc::c_int);
                        }
                        break ;
                    }
                }
            }
        }
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn request_reply(mut request: *mut CMD_Request,
                                   mut reply: *mut CMD_Reply,
                                   mut requested_reply: libc::c_int,
                                   mut verbose: libc::c_int) -> libc::c_int {
    let mut status: libc::c_int = 0;
    while submit_request(request, reply) == 0 {
        /* Try connecting to other addresses before giving up */
        if open_io() != 0 { continue ; }
        printf(b"506 Cannot talk to daemon\n\x00" as *const u8 as
                   *const libc::c_char);
        return 0 as libc::c_int
    }
    status = ntohs((*reply).status) as libc::c_int;
    if verbose != 0 || status != 0 as libc::c_int {
        match status {
            0 => {
                printf(b"200 OK\x00" as *const u8 as *const libc::c_char);
            }
            8 => {
                printf(b"208 Access allowed\x00" as *const u8 as
                           *const libc::c_char);
            }
            9 => {
                printf(b"209 Access denied\x00" as *const u8 as
                           *const libc::c_char);
            }
            1 => {
                printf(b"500 Failure\x00" as *const u8 as
                           *const libc::c_char);
            }
            2 => {
                printf(b"501 Not authorised\x00" as *const u8 as
                           *const libc::c_char);
            }
            3 => {
                printf(b"502 Invalid command\x00" as *const u8 as
                           *const libc::c_char);
            }
            4 => {
                printf(b"503 No such source\x00" as *const u8 as
                           *const libc::c_char);
            }
            5 => {
                printf(b"504 Duplicate or stale logon detected\x00" as
                           *const u8 as *const libc::c_char);
            }
            6 => {
                printf(b"505 Facility not enabled in daemon\x00" as *const u8
                           as *const libc::c_char);
            }
            7 => {
                printf(b"507 Bad subnet\x00" as *const u8 as
                           *const libc::c_char);
            }
            10 => {
                printf(b"510 No command access from this host\x00" as
                           *const u8 as *const libc::c_char);
            }
            11 => {
                printf(b"511 Source already present\x00" as *const u8 as
                           *const libc::c_char);
            }
            12 => {
                printf(b"512 Too many sources present\x00" as *const u8 as
                           *const libc::c_char);
            }
            13 => {
                printf(b"513 RTC driver not running\x00" as *const u8 as
                           *const libc::c_char);
            }
            14 => {
                printf(b"514 Can\'t write RTC parameters\x00" as *const u8 as
                           *const libc::c_char);
            }
            17 => {
                printf(b"515 Invalid address family\x00" as *const u8 as
                           *const libc::c_char);
            }
            16 => {
                printf(b"516 Sample index out of range\x00" as *const u8 as
                           *const libc::c_char);
            }
            18 => {
                printf(b"517 Protocol version mismatch\x00" as *const u8 as
                           *const libc::c_char);
            }
            19 => {
                printf(b"518 Packet length mismatch\x00" as *const u8 as
                           *const libc::c_char);
            }
            15 => {
                printf(b"519 Client logging is not active in the daemon\x00"
                           as *const u8 as *const libc::c_char);
            }
            21 => {
                printf(b"521 Invalid name\x00" as *const u8 as
                           *const libc::c_char);
            }
            _ => {
                printf(b"520 Got unexpected error from daemon\x00" as
                           *const u8 as *const libc::c_char);
            }
        }
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
    }
    if status != 0 as libc::c_int && status != 8 as libc::c_int &&
           status != 9 as libc::c_int {
        return 0 as libc::c_int
    }
    if ntohs((*reply).reply) as libc::c_int != requested_reply {
        printf(b"508 Bad reply from daemon\n\x00" as *const u8 as
                   *const libc::c_char);
        return 0 as libc::c_int
    }
    /* Make sure an unknown response was not requested */
    if PKL_ReplyLength(reply) != 0 {
    } else {
        __assert_fail(b"PKL_ReplyLength(reply)\x00" as *const u8 as
                          *const libc::c_char,
                      b"client.c\x00" as *const u8 as *const libc::c_char,
                      1614 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 56],
                                                &[libc::c_char; 56]>(b"int request_reply(CMD_Request *, CMD_Reply *, int, int)\x00")).as_ptr());
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn print_seconds(mut s: libc::c_ulong) {
    let mut d: libc::c_ulong = 0;
    if s == -(1 as libc::c_int) as uint32_t as libc::c_ulong {
        printf(b"   -\x00" as *const u8 as *const libc::c_char);
    } else if s < 1200 as libc::c_int as libc::c_ulong {
        printf(b"%4lu\x00" as *const u8 as *const libc::c_char, s);
    } else if s < 36000 as libc::c_int as libc::c_ulong {
        printf(b"%3lum\x00" as *const u8 as *const libc::c_char,
               s.wrapping_div(60 as libc::c_int as libc::c_ulong));
    } else if s < 345600 as libc::c_int as libc::c_ulong {
        printf(b"%3luh\x00" as *const u8 as *const libc::c_char,
               s.wrapping_div(3600 as libc::c_int as libc::c_ulong));
    } else {
        d = s.wrapping_div(86400 as libc::c_int as libc::c_ulong);
        if d > 999 as libc::c_int as libc::c_ulong {
            printf(b"%3luy\x00" as *const u8 as *const libc::c_char,
                   d.wrapping_div(365 as libc::c_int as libc::c_ulong));
        } else {
            printf(b"%3lud\x00" as *const u8 as *const libc::c_char, d);
        }
    };
}
/* ================================================== */
unsafe extern "C" fn print_nanoseconds(mut s: libc::c_double) {
    s = fabs(s);
    if s < 9999.5e-9f64 {
        printf(b"%4.0fns\x00" as *const u8 as *const libc::c_char,
               s * 1e9f64);
    } else if s < 9999.5e-6f64 {
        printf(b"%4.0fus\x00" as *const u8 as *const libc::c_char,
               s * 1e6f64);
    } else if s < 9999.5e-3f64 {
        printf(b"%4.0fms\x00" as *const u8 as *const libc::c_char,
               s * 1e3f64);
    } else if s < 999.5f64 {
        printf(b"%5.1fs\x00" as *const u8 as *const libc::c_char, s);
    } else if s < 99999.5f64 {
        printf(b"%5.0fs\x00" as *const u8 as *const libc::c_char, s);
    } else if s < 99999.5f64 * 60 as libc::c_int as libc::c_double {
        printf(b"%5.0fm\x00" as *const u8 as *const libc::c_char,
               s / 60 as libc::c_int as libc::c_double);
    } else if s < 99999.5f64 * 3600 as libc::c_int as libc::c_double {
        printf(b"%5.0fh\x00" as *const u8 as *const libc::c_char,
               s / 3600 as libc::c_int as libc::c_double);
    } else if s <
                  99999.5f64 * 3600 as libc::c_int as libc::c_double *
                      24 as libc::c_int as libc::c_double {
        printf(b"%5.0fd\x00" as *const u8 as *const libc::c_char,
               s /
                   (3600 as libc::c_int * 24 as libc::c_int) as
                       libc::c_double);
    } else {
        printf(b"%5.0fy\x00" as *const u8 as *const libc::c_char,
               s /
                   (3600 as libc::c_int * 24 as libc::c_int *
                        365 as libc::c_int) as libc::c_double);
    };
}
/* ================================================== */
unsafe extern "C" fn print_signed_nanoseconds(mut s: libc::c_double) {
    let mut x: libc::c_double = 0.;
    x = fabs(s);
    if x < 9999.5e-9f64 {
        printf(b"%+5.0fns\x00" as *const u8 as *const libc::c_char,
               s * 1e9f64);
    } else if x < 9999.5e-6f64 {
        printf(b"%+5.0fus\x00" as *const u8 as *const libc::c_char,
               s * 1e6f64);
    } else if x < 9999.5e-3f64 {
        printf(b"%+5.0fms\x00" as *const u8 as *const libc::c_char,
               s * 1e3f64);
    } else if x < 999.5f64 {
        printf(b"%+6.1fs\x00" as *const u8 as *const libc::c_char, s);
    } else if x < 99999.5f64 {
        printf(b"%+6.0fs\x00" as *const u8 as *const libc::c_char, s);
    } else if x < 99999.5f64 * 60 as libc::c_int as libc::c_double {
        printf(b"%+6.0fm\x00" as *const u8 as *const libc::c_char,
               s / 60 as libc::c_int as libc::c_double);
    } else if x < 99999.5f64 * 3600 as libc::c_int as libc::c_double {
        printf(b"%+6.0fh\x00" as *const u8 as *const libc::c_char,
               s / 3600 as libc::c_int as libc::c_double);
    } else if x <
                  99999.5f64 * 3600 as libc::c_int as libc::c_double *
                      24 as libc::c_int as libc::c_double {
        printf(b"%+6.0fd\x00" as *const u8 as *const libc::c_char,
               s /
                   (3600 as libc::c_int * 24 as libc::c_int) as
                       libc::c_double);
    } else {
        printf(b"%+6.0fy\x00" as *const u8 as *const libc::c_char,
               s /
                   (3600 as libc::c_int * 24 as libc::c_int *
                        365 as libc::c_int) as libc::c_double);
    };
}
/* ================================================== */
unsafe extern "C" fn print_freq_ppm(mut f: libc::c_double) {
    if fabs(f) < 99999.5f64 {
        printf(b"%10.3f\x00" as *const u8 as *const libc::c_char, f);
    } else { printf(b"%10.0f\x00" as *const u8 as *const libc::c_char, f); };
}
/* ================================================== */
unsafe extern "C" fn print_signed_freq_ppm(mut f: libc::c_double) {
    if fabs(f) < 99999.5f64 {
        printf(b"%+10.3f\x00" as *const u8 as *const libc::c_char, f);
    } else { printf(b"%+10.0f\x00" as *const u8 as *const libc::c_char, f); };
}
/* ================================================== */
unsafe extern "C" fn print_clientlog_interval(mut rate: libc::c_int) {
    if rate >= 127 as libc::c_int {
        printf(b" -\x00" as *const u8 as *const libc::c_char);
    } else { printf(b"%2d\x00" as *const u8 as *const libc::c_char, rate); };
}
/* ================================================== */
unsafe extern "C" fn print_header(mut header: *const libc::c_char) {
    let mut len: libc::c_int = 0;
    if csv_mode != 0 { return }
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char, header);
    len = strlen(header) as libc::c_int;
    loop  {
        let fresh9 = len;
        len = len - 1;
        if !(fresh9 != 0) { break ; }
        printf(b"=\x00" as *const u8 as *const libc::c_char);
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
/* Print a report. The syntax of the format is similar to printf(), but not all
   specifiers are supported and some are different! */
unsafe extern "C" fn print_report(mut format: *const libc::c_char,
                                  mut args: ...) {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut ap: ::std::ffi::VaListImpl;
    let mut i: libc::c_int = 0;
    let mut field: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut prec: libc::c_int = 0;
    let mut spec: libc::c_int = 0;
    let mut string: *const libc::c_char = 0 as *const libc::c_char;
    let mut long_uinteger: libc::c_ulong = 0;
    let mut uinteger: libc::c_uint = 0;
    let mut integer: libc::c_int = 0;
    let mut ts: *mut timespec = 0 as *mut timespec;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut dbl: libc::c_double = 0.;
    ap = args.clone();
    field = 0 as libc::c_int;
    loop  {
        /* Search for text between format specifiers and print it
       if not in the CSV mode */
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) <
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong &&
                  *format.offset(i as isize) as libc::c_int != '%' as i32 &&
                  *format.offset(i as isize) as libc::c_int != '\u{0}' as i32
              {
            buf[i as usize] = *format.offset(i as isize);
            i += 1
        }
        if i as libc::c_ulong >=
               ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong {
            break ;
        }
        buf[i as usize] = '\u{0}' as i32 as libc::c_char;
        if csv_mode == 0 {
            printf(b"%s\x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        }
        if *format.offset(i as isize) as libc::c_int == '\u{0}' as i32 ||
               *format.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                   == '\u{0}' as i32 {
            break ;
        }
        format = format.offset((i + 1 as libc::c_int) as isize);
        sign = 0 as libc::c_int;
        width = 0 as libc::c_int;
        prec = 5 as libc::c_int;
        if *format as libc::c_int == '+' as i32 ||
               *format as libc::c_int == '-' as i32 {
            sign = 1 as libc::c_int;
            format = format.offset(1)
        }
        if *(*__ctype_b_loc()).offset(*format as libc::c_uchar as libc::c_int
                                          as isize) as libc::c_int &
               _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            width = atoi(format);
            while *(*__ctype_b_loc()).offset(*format as libc::c_uchar as
                                                 libc::c_int as isize) as
                      libc::c_int &
                      _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
                format = format.offset(1)
            }
        }
        if *format as libc::c_int == '.' as i32 {
            format = format.offset(1);
            prec = atoi(format);
            while *(*__ctype_b_loc()).offset(*format as libc::c_uchar as
                                                 libc::c_int as isize) as
                      libc::c_int &
                      _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
                format = format.offset(1)
            }
        }
        spec = *format as libc::c_int;
        format = format.offset(1);
        /* Disable human-readable formatting in the CSV mode */
        if csv_mode != 0 {
            width = 0 as libc::c_int;
            sign = width;
            if field > 0 as libc::c_int {
                printf(b",\x00" as *const u8 as *const libc::c_char);
            }
            match spec {
                67 => { spec = 'd' as i32 }
                70 | 80 => { prec = 3 as libc::c_int; spec = 'f' as i32 }
                79 | 83 => { prec = 9 as libc::c_int; spec = 'f' as i32 }
                73 => { spec = 'U' as i32 }
                84 => { spec = 'V' as i32 }
                _ => { }
            }
        }
        match spec {
            66 => {
                /* boolean */
                integer = ap.as_va_list().arg::<libc::c_int>();
                printf(b"%s\x00" as *const u8 as *const libc::c_char,
                       if integer != 0 {
                           b"Yes\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"No\x00" as *const u8 as *const libc::c_char
                       });
            }
            67 => {
                /* clientlog interval */
                integer = ap.as_va_list().arg::<libc::c_int>();
                print_clientlog_interval(integer);
            }
            70 | 79 => {
                /* absolute frequency in ppm with fast/slow keyword */
                /* absolute offset in seconds with fast/slow keyword */
                dbl = ap.as_va_list().arg::<libc::c_double>();
                printf(b"%*.*f %s %s\x00" as *const u8 as *const libc::c_char,
                       width, prec, fabs(dbl),
                       if spec == 'O' as i32 {
                           b"seconds\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"ppm\x00" as *const u8 as *const libc::c_char
                       },
                       if (dbl > 0.0f64) as libc::c_int ^
                              (spec != 'O' as i32) as libc::c_int != 0 {
                           b"slow\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"fast\x00" as *const u8 as *const libc::c_char
                       });
            }
            73 => {
                /* interval with unit */
                long_uinteger = ap.as_va_list().arg::<libc::c_ulong>();
                print_seconds(long_uinteger);
            }
            76 => {
                /* leap status */
                integer = ap.as_va_list().arg::<libc::c_int>();
                match integer {
                    0 => {
                        string =
                            b"Normal\x00" as *const u8 as *const libc::c_char
                    }
                    1 => {
                        string =
                            b"Insert second\x00" as *const u8 as
                                *const libc::c_char
                    }
                    2 => {
                        string =
                            b"Delete second\x00" as *const u8 as
                                *const libc::c_char
                    }
                    3 => {
                        string =
                            b"Not synchronised\x00" as *const u8 as
                                *const libc::c_char
                    }
                    _ => {
                        string =
                            b"Invalid\x00" as *const u8 as *const libc::c_char
                    }
                }
                printf(b"%s\x00" as *const u8 as *const libc::c_char, string);
            }
            77 => {
                /* NTP mode */
                integer = ap.as_va_list().arg::<libc::c_int>();
                match integer {
                    1 => {
                        string =
                            b"Symmetric active\x00" as *const u8 as
                                *const libc::c_char
                    }
                    2 => {
                        string =
                            b"Symmetric passive\x00" as *const u8 as
                                *const libc::c_char
                    }
                    4 => {
                        string =
                            b"Server\x00" as *const u8 as *const libc::c_char
                    }
                    _ => {
                        string =
                            b"Invalid\x00" as *const u8 as *const libc::c_char
                    }
                }
                printf(b"%s\x00" as *const u8 as *const libc::c_char, string);
            }
            78 => {
                /* Timestamp source */
                integer = ap.as_va_list().arg::<libc::c_int>();
                match integer {
                    68 => {
                        string =
                            b"Daemon\x00" as *const u8 as *const libc::c_char
                    }
                    75 => {
                        string =
                            b"Kernel\x00" as *const u8 as *const libc::c_char
                    }
                    72 => {
                        string =
                            b"Hardware\x00" as *const u8 as
                                *const libc::c_char
                    }
                    _ => {
                        string =
                            b"Invalid\x00" as *const u8 as *const libc::c_char
                    }
                }
                printf(b"%s\x00" as *const u8 as *const libc::c_char, string);
            }
            80 => {
                /* frequency in ppm */
                dbl = ap.as_va_list().arg::<libc::c_double>();
                if sign != 0 {
                    print_signed_freq_ppm(dbl);
                } else { print_freq_ppm(dbl); }
            }
            82 => {
                /* reference ID in hexdecimal */
                long_uinteger = ap.as_va_list().arg::<libc::c_ulong>();
                printf(b"%08lX\x00" as *const u8 as *const libc::c_char,
                       long_uinteger);
            }
            83 => {
                /* offset with unit */
                dbl = ap.as_va_list().arg::<libc::c_double>();
                if sign != 0 {
                    print_signed_nanoseconds(dbl);
                } else { print_nanoseconds(dbl); }
            }
            84 => {
                /* timespec as date and time in UTC */
                ts = ap.as_va_list().arg::<*mut timespec>();
                tm = gmtime(&mut (*ts).tv_sec);
                if !tm.is_null() {
                    strftime(buf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 256]>() as
                                 libc::c_ulong,
                             b"%a %b %d %T %Y\x00" as *const u8 as
                                 *const libc::c_char, tm);
                    printf(b"%s\x00" as *const u8 as *const libc::c_char,
                           buf.as_mut_ptr());
                }
            }
            85 => {
                /* unsigned long in decimal */
                long_uinteger = ap.as_va_list().arg::<libc::c_ulong>();
                printf(b"%*lu\x00" as *const u8 as *const libc::c_char, width,
                       long_uinteger);
            }
            86 => {
                /* timespec as seconds since epoch */
                ts = ap.as_va_list().arg::<*mut timespec>();
                printf(b"%s\x00" as *const u8 as *const libc::c_char,
                       UTI_TimespecToString(ts));
            }
            98 => {
                /* unsigned int in binary */
                uinteger = ap.as_va_list().arg::<libc::c_uint>();
                i = prec - 1 as libc::c_int;
                while i >= 0 as libc::c_int {
                    printf(b"%c\x00" as *const u8 as *const libc::c_char,
                           if uinteger & (1 as libc::c_uint) << i != 0 {
                               '1' as i32
                           } else { '0' as i32 });
                    i -= 1
                }
            }
            99 => {
                /* Classic printf specifiers */
                /* character */
                integer = ap.as_va_list().arg::<libc::c_int>();
                printf(b"%c\x00" as *const u8 as *const libc::c_char,
                       integer);
            }
            100 => {
                /* signed int in decimal */
                integer = ap.as_va_list().arg::<libc::c_int>();
                printf(b"%*d\x00" as *const u8 as *const libc::c_char, width,
                       integer);
            }
            102 => {
                /* double */
                dbl = ap.as_va_list().arg::<libc::c_double>();
                printf(if sign != 0 {
                           b"%+*.*f\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"%*.*f\x00" as *const u8 as *const libc::c_char
                       }, width, prec, dbl);
            }
            111 => {
                /* unsigned int in octal */
                uinteger = ap.as_va_list().arg::<libc::c_uint>();
                printf(b"%*o\x00" as *const u8 as *const libc::c_char, width,
                       uinteger);
            }
            115 => {
                /* string */
                string = ap.as_va_list().arg::<*const libc::c_char>();
                if sign != 0 {
                    printf(b"%-*s\x00" as *const u8 as *const libc::c_char,
                           width, string);
                } else {
                    printf(b"%*s\x00" as *const u8 as *const libc::c_char,
                           width, string);
                }
            }
            117 => {
                /* unsigned int in decimal */
                uinteger = ap.as_va_list().arg::<libc::c_uint>();
                printf(b"%*u\x00" as *const u8 as *const libc::c_char, width,
                       uinteger);
            }
            _ => { }
        }
        field += 1
    }
    /* Require terminating argument to catch bad type conversions */
    if ap.as_va_list().arg::<libc::c_int>() != 0x1234 as libc::c_int {
        __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                      b"client.c\x00" as *const u8 as *const libc::c_char,
                      2003 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"void print_report(const char *, ...)\x00")).as_ptr());
    }
    if csv_mode != 0 {
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
    };
}
/* ================================================== */
unsafe extern "C" fn print_info_field(mut format: *const libc::c_char,
                                      mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    if csv_mode != 0 { return }
    ap = args.clone();
    vprintf(format, ap.as_va_list());
}
/* ================================================== */
unsafe extern "C" fn get_source_name(mut ip_addr: *mut IPAddr,
                                     mut buf: *mut libc::c_char,
                                     mut size: libc::c_int) -> libc::c_int {
    let mut request: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut reply: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    let mut i: libc::c_int = 0;
    request.command = htons(65 as libc::c_int as uint16_t);
    UTI_IPHostToNetwork(ip_addr, &mut request.data.ntp_source_name.ip_addr);
    if request_reply(&mut request, &mut reply, 19 as libc::c_int,
                     0 as libc::c_int) == 0 ||
           reply.data.ntp_source_name.name[(::std::mem::size_of::<[int8_t; 256]>()
                                                as
                                                libc::c_ulong).wrapping_sub(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                               as usize] as libc::c_int !=
               '\u{0}' as i32 ||
           snprintf(buf, size as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    reply.data.ntp_source_name.name.as_mut_ptr()) >= size {
        return 0 as libc::c_int
    }
    /* Make sure the name is printable */
    i = 0 as libc::c_int;
    while i < size && *buf.offset(i as isize) as libc::c_int != '\u{0}' as i32
          {
        if *(*__ctype_b_loc()).offset(*buf.offset(i as isize) as libc::c_int
                                          as isize) as libc::c_int &
               _ISgraph as libc::c_int as libc::c_ushort as libc::c_int == 0 {
            return 0 as libc::c_int
        }
        i += 1
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn format_name(mut buf: *mut libc::c_char,
                                 mut size: libc::c_int,
                                 mut trunc_dns: libc::c_int,
                                 mut ref_0: libc::c_int, mut ref_id: uint32_t,
                                 mut source: libc::c_int,
                                 mut ip_addr: *mut IPAddr) {
    if ref_0 != 0 {
        snprintf(buf, size as libc::c_ulong,
                 b"%s\x00" as *const u8 as *const libc::c_char,
                 UTI_RefidToString(ref_id));
    } else if source != 0 && source_names != 0 {
        if get_source_name(ip_addr, buf, size) == 0 {
            snprintf(buf, size as libc::c_ulong,
                     b"?\x00" as *const u8 as *const libc::c_char);
        }
    } else if no_dns != 0 || csv_mode != 0 {
        snprintf(buf, size as libc::c_ulong,
                 b"%s\x00" as *const u8 as *const libc::c_char,
                 UTI_IPToString(ip_addr));
    } else {
        DNS_IPAddress2Name(ip_addr, buf, size);
        if trunc_dns > 0 as libc::c_int &&
               strlen(buf) > trunc_dns as libc::c_ulong {
            *buf.offset((trunc_dns - 1 as libc::c_int) as isize) =
                '>' as i32 as libc::c_char;
            *buf.offset(trunc_dns as isize) = '\u{0}' as i32 as libc::c_char
        }
    };
}
/* ================================================== */
unsafe extern "C" fn check_for_verbose_flag(mut line: *mut libc::c_char)
 -> libc::c_int {
    if csv_mode == 0 &&
           strcmp(line, b"-v\x00" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_sourcename(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut ip_addr: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut name: [libc::c_char; 256] = [0; 256];
    if UTI_StringToIP(line, &mut ip_addr) == 0 {
        LOG_Message(LOGS_ERR,
                    b"Could not read address\x00" as *const u8 as
                        *const libc::c_char);
        return 0 as libc::c_int
    }
    if get_source_name(&mut ip_addr, name.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 256]>() as
                           libc::c_ulong as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    print_report(b"%s\n\x00" as *const u8 as *const libc::c_char,
                 name.as_mut_ptr(), 0x1234 as libc::c_int);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_sources(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut request: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut reply: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    let mut ip_addr: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut i: uint32_t = 0;
    let mut mode: uint32_t = 0;
    let mut n_sources: uint32_t = 0;
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut mode_ch: libc::c_char = 0;
    let mut state_ch: libc::c_char = 0;
    let mut verbose: libc::c_int = 0;
    /* Check whether to output verbose headers */
    verbose = check_for_verbose_flag(line);
    request.command = htons(14 as libc::c_int as uint16_t);
    if request_reply(&mut request, &mut reply, 2 as libc::c_int,
                     0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    n_sources = ntohl(reply.data.n_sources.n_sources);
    print_info_field(b"210 Number of sources = %lu\n\x00" as *const u8 as
                         *const libc::c_char, n_sources as libc::c_ulong);
    if verbose != 0 {
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        printf(b"  .-- Source mode  \'^\' = server, \'=\' = peer, \'#\' = local clock.\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b" / .- Source state \'*\' = current synced, \'+\' = combined , \'-\' = not combined,\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"| /   \'?\' = unreachable, \'x\' = time may be in error, \'~\' = time too variable.\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"||                                                 .- xxxx [ yyyy ] +/- zzzz\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"||      Reachability register (octal) -.           |  xxxx = adjusted offset,\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"||      Log2(Polling interval) --.      |          |  yyyy = measured offset,\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"||                                \\     |          |  zzzz = estimated error.\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"||                                 |    |           \\\n\x00"
                   as *const u8 as *const libc::c_char);
    }
    print_header(b"MS Name/IP address         Stratum Poll Reach LastRx Last sample               \x00"
                     as *const u8 as *const libc::c_char);
    /*           "MS NNNNNNNNNNNNNNNNNNNNNNNNNNN  SS  PP   RRR  RRRR  SSSSSSS[SSSSSSS] +/- SSSSSS" */
    i = 0 as libc::c_int as uint32_t;
    while i < n_sources {
        request.command = htons(15 as libc::c_int as uint16_t);
        request.data.source_data.index = htonl(i) as int32_t;
        if request_reply(&mut request, &mut reply, 3 as libc::c_int,
                         0 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
        mode = ntohs(reply.data.source_data.mode) as uint32_t;
        UTI_IPNetworkToHost(&mut reply.data.source_data.ip_addr,
                            &mut ip_addr);
        format_name(name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as
                        libc::c_ulong as libc::c_int, 25 as libc::c_int,
                    (mode == 2 as libc::c_int as libc::c_uint &&
                         ip_addr.family as libc::c_int == 1 as libc::c_int) as
                        libc::c_int, ip_addr.addr.in4, 1 as libc::c_int,
                    &mut ip_addr);
        match mode {
            0 => { mode_ch = '^' as i32 as libc::c_char }
            1 => { mode_ch = '=' as i32 as libc::c_char }
            2 => { mode_ch = '#' as i32 as libc::c_char }
            _ => { mode_ch = ' ' as i32 as libc::c_char }
        }
        match ntohs(reply.data.source_data.state) as libc::c_int {
            0 => { state_ch = '*' as i32 as libc::c_char }
            1 => { state_ch = '?' as i32 as libc::c_char }
            2 => { state_ch = 'x' as i32 as libc::c_char }
            3 => { state_ch = '~' as i32 as libc::c_char }
            4 => { state_ch = '+' as i32 as libc::c_char }
            5 => { state_ch = '-' as i32 as libc::c_char }
            _ => { state_ch = ' ' as i32 as libc::c_char }
        }
        match ntohs(reply.data.source_data.flags) as libc::c_int { _ => { } }
        print_report(b"%c%c %-27s  %2d  %2d   %3o  %I  %+S[%+S] +/- %S\n\x00"
                         as *const u8 as *const libc::c_char,
                     mode_ch as libc::c_int, state_ch as libc::c_int,
                     name.as_mut_ptr(),
                     ntohs(reply.data.source_data.stratum) as libc::c_int,
                     ntohs(reply.data.source_data.poll as uint16_t) as int16_t
                         as libc::c_int,
                     ntohs(reply.data.source_data.reachability) as
                         libc::c_int,
                     ntohl(reply.data.source_data.since_sample) as
                         libc::c_ulong,
                     UTI_FloatNetworkToHost(reply.data.source_data.latest_meas),
                     UTI_FloatNetworkToHost(reply.data.source_data.orig_latest_meas),
                     UTI_FloatNetworkToHost(reply.data.source_data.latest_meas_err),
                     0x1234 as libc::c_int);
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_sourcestats(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut request: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut reply: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    let mut i: uint32_t = 0;
    let mut n_sources: uint32_t = 0;
    let mut verbose: libc::c_int = 0 as libc::c_int;
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut ip_addr: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    verbose = check_for_verbose_flag(line);
    request.command = htons(14 as libc::c_int as uint16_t);
    if request_reply(&mut request, &mut reply, 2 as libc::c_int,
                     0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    n_sources = ntohl(reply.data.n_sources.n_sources);
    print_info_field(b"210 Number of sources = %lu\n\x00" as *const u8 as
                         *const libc::c_char, n_sources as libc::c_ulong);
    if verbose != 0 {
        printf(b"                             .- Number of sample points in measurement set.\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"                            /    .- Number of residual runs with same sign.\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"                           |    /    .- Length of measurement set (time).\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"                           |   |    /      .- Est. clock freq error (ppm).\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"                           |   |   |      /           .- Est. error in freq.\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"                           |   |   |     |           /         .- Est. offset.\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"                           |   |   |     |          |          |   On the -.\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"                           |   |   |     |          |          |   samples. \\\n\x00"
                   as *const u8 as *const libc::c_char);
        printf(b"                           |   |   |     |          |          |             |\n\x00"
                   as *const u8 as *const libc::c_char);
    }
    print_header(b"Name/IP Address            NP  NR  Span  Frequency  Freq Skew  Offset  Std Dev\x00"
                     as *const u8 as *const libc::c_char);
    /*           "NNNNNNNNNNNNNNNNNNNNNNNNN  NP  NR  SSSS FFFFFFFFFF SSSSSSSSSS  SSSSSSS  SSSSSS" */
    i = 0 as libc::c_int as uint32_t;
    while i < n_sources {
        request.command = htons(34 as libc::c_int as uint16_t);
        request.data.source_data.index = htonl(i) as int32_t;
        if request_reply(&mut request, &mut reply, 6 as libc::c_int,
                         0 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
        UTI_IPNetworkToHost(&mut reply.data.sourcestats.ip_addr,
                            &mut ip_addr);
        format_name(name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as
                        libc::c_ulong as libc::c_int, 25 as libc::c_int,
                    (ip_addr.family as libc::c_int == 0 as libc::c_int) as
                        libc::c_int, ntohl(reply.data.sourcestats.ref_id),
                    1 as libc::c_int, &mut ip_addr);
        print_report(b"%-25s %3U %3U  %I %+P %P  %+S  %S\n\x00" as *const u8
                         as *const libc::c_char, name.as_mut_ptr(),
                     ntohl(reply.data.sourcestats.n_samples) as libc::c_ulong,
                     ntohl(reply.data.sourcestats.n_runs) as libc::c_ulong,
                     ntohl(reply.data.sourcestats.span_seconds) as
                         libc::c_ulong,
                     UTI_FloatNetworkToHost(reply.data.sourcestats.resid_freq_ppm),
                     UTI_FloatNetworkToHost(reply.data.sourcestats.skew_ppm),
                     UTI_FloatNetworkToHost(reply.data.sourcestats.est_offset),
                     UTI_FloatNetworkToHost(reply.data.sourcestats.sd),
                     0x1234 as libc::c_int);
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_tracking(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut request: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut reply: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    let mut ip_addr: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut ref_id: uint32_t = 0;
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut ref_time: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    request.command = htons(33 as libc::c_int as uint16_t);
    if request_reply(&mut request, &mut reply, 5 as libc::c_int,
                     0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    ref_id = ntohl(reply.data.tracking.ref_id);
    UTI_IPNetworkToHost(&mut reply.data.tracking.ip_addr, &mut ip_addr);
    format_name(name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                    as libc::c_int,
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                    as libc::c_int,
                (ip_addr.family as libc::c_int == 0 as libc::c_int) as
                    libc::c_int, ref_id, 1 as libc::c_int, &mut ip_addr);
    UTI_TimespecNetworkToHost(&mut reply.data.tracking.ref_time,
                              &mut ref_time);
    print_report(b"Reference ID    : %R (%s)\nStratum         : %u\nRef time (UTC)  : %T\nSystem time     : %.9O of NTP time\nLast offset     : %+.9f seconds\nRMS offset      : %.9f seconds\nFrequency       : %.3F\nResidual freq   : %+.3f ppm\nSkew            : %.3f ppm\nRoot delay      : %.9f seconds\nRoot dispersion : %.9f seconds\nUpdate interval : %.1f seconds\nLeap status     : %L\n\x00"
                     as *const u8 as *const libc::c_char,
                 ref_id as libc::c_ulong, name.as_mut_ptr(),
                 ntohs(reply.data.tracking.stratum) as libc::c_int,
                 &mut ref_time as *mut timespec,
                 UTI_FloatNetworkToHost(reply.data.tracking.current_correction),
                 UTI_FloatNetworkToHost(reply.data.tracking.last_offset),
                 UTI_FloatNetworkToHost(reply.data.tracking.rms_offset),
                 UTI_FloatNetworkToHost(reply.data.tracking.freq_ppm),
                 UTI_FloatNetworkToHost(reply.data.tracking.resid_freq_ppm),
                 UTI_FloatNetworkToHost(reply.data.tracking.skew_ppm),
                 UTI_FloatNetworkToHost(reply.data.tracking.root_delay),
                 UTI_FloatNetworkToHost(reply.data.tracking.root_dispersion),
                 UTI_FloatNetworkToHost(reply.data.tracking.last_update_interval),
                 ntohs(reply.data.tracking.leap_status) as libc::c_int,
                 0x1234 as libc::c_int);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_ntpdata(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut request: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut reply: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    let mut remote_addr: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut local_addr: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut ref_time: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut i: uint32_t = 0;
    let mut n_sources: uint32_t = 0;
    let mut mode: uint16_t = 0;
    let mut specified_addr: libc::c_int = 0;
    if *line != 0 {
        specified_addr = 1 as libc::c_int;
        n_sources = 1 as libc::c_int as uint32_t
    } else {
        specified_addr = 0 as libc::c_int;
        request.command = htons(14 as libc::c_int as uint16_t);
        if request_reply(&mut request, &mut reply, 2 as libc::c_int,
                         0 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
        n_sources = ntohl(reply.data.n_sources.n_sources)
    }
    let mut current_block_31: u64;
    i = 0 as libc::c_int as uint32_t;
    while i < n_sources {
        if specified_addr != 0 {
            if DNS_Name2IPAddress(line, &mut remote_addr, 1 as libc::c_int) as
                   libc::c_uint != DNS_Success as libc::c_int as libc::c_uint
               {
                LOG_Message(LOGS_ERR,
                            b"Could not get address for hostname\x00" as
                                *const u8 as *const libc::c_char);
                return 0 as libc::c_int
            }
            current_block_31 = 4068382217303356765;
        } else {
            request.command = htons(15 as libc::c_int as uint16_t);
            request.data.source_data.index = htonl(i) as int32_t;
            if request_reply(&mut request, &mut reply, 3 as libc::c_int,
                             0 as libc::c_int) == 0 {
                return 0 as libc::c_int
            }
            mode = ntohs(reply.data.source_data.mode);
            if mode as libc::c_int != 0 as libc::c_int &&
                   mode as libc::c_int != 1 as libc::c_int {
                current_block_31 = 10599921512955367680;
            } else {
                UTI_IPNetworkToHost(&mut reply.data.source_data.ip_addr,
                                    &mut remote_addr);
                current_block_31 = 4068382217303356765;
            }
        }
        match current_block_31 {
            4068382217303356765 => {
                request.command = htons(57 as libc::c_int as uint16_t);
                UTI_IPHostToNetwork(&mut remote_addr,
                                    &mut request.data.ntp_data.ip_addr);
                if request_reply(&mut request, &mut reply, 16 as libc::c_int,
                                 0 as libc::c_int) == 0 {
                    return 0 as libc::c_int
                }
                UTI_IPNetworkToHost(&mut reply.data.ntp_data.remote_addr,
                                    &mut remote_addr);
                UTI_IPNetworkToHost(&mut reply.data.ntp_data.local_addr,
                                    &mut local_addr);
                UTI_TimespecNetworkToHost(&mut reply.data.ntp_data.ref_time,
                                          &mut ref_time);
                if specified_addr == 0 && csv_mode == 0 {
                    printf(b"\n\x00" as *const u8 as *const libc::c_char);
                }
                print_report(b"Remote address  : %s (%R)\nRemote port     : %u\nLocal address   : %s (%R)\nLeap status     : %L\nVersion         : %u\nMode            : %M\nStratum         : %u\nPoll interval   : %d (%.0f seconds)\nPrecision       : %d (%.9f seconds)\nRoot delay      : %.6f seconds\nRoot dispersion : %.6f seconds\nReference ID    : %R (%s)\nReference time  : %T\nOffset          : %+.9f seconds\nPeer delay      : %.9f seconds\nPeer dispersion : %.9f seconds\nResponse time   : %.9f seconds\nJitter asymmetry: %+.2f\nNTP tests       : %.3b %.3b %.4b\nInterleaved     : %B\nAuthenticated   : %B\nTX timestamping : %N\nRX timestamping : %N\nTotal TX        : %U\nTotal RX        : %U\nTotal valid RX  : %U\n\x00"
                                 as *const u8 as *const libc::c_char,
                             UTI_IPToString(&mut remote_addr),
                             UTI_IPToRefid(&mut remote_addr) as libc::c_ulong,
                             ntohs(reply.data.ntp_data.remote_port) as
                                 libc::c_int, UTI_IPToString(&mut local_addr),
                             UTI_IPToRefid(&mut local_addr) as libc::c_ulong,
                             reply.data.ntp_data.leap as libc::c_int,
                             reply.data.ntp_data.version as libc::c_int,
                             reply.data.ntp_data.mode as libc::c_int,
                             reply.data.ntp_data.stratum as libc::c_int,
                             reply.data.ntp_data.poll as libc::c_int,
                             UTI_Log2ToDouble(reply.data.ntp_data.poll as
                                                  libc::c_int),
                             reply.data.ntp_data.precision as libc::c_int,
                             UTI_Log2ToDouble(reply.data.ntp_data.precision as
                                                  libc::c_int),
                             UTI_FloatNetworkToHost(reply.data.ntp_data.root_delay),
                             UTI_FloatNetworkToHost(reply.data.ntp_data.root_dispersion),
                             ntohl(reply.data.ntp_data.ref_id) as
                                 libc::c_ulong,
                             if reply.data.ntp_data.stratum as libc::c_int <=
                                    1 as libc::c_int {
                                 UTI_RefidToString(ntohl(reply.data.ntp_data.ref_id))
                             } else {
                                 b"\x00" as *const u8 as *const libc::c_char
                             }, &mut ref_time as *mut timespec,
                             UTI_FloatNetworkToHost(reply.data.ntp_data.offset),
                             UTI_FloatNetworkToHost(reply.data.ntp_data.peer_delay),
                             UTI_FloatNetworkToHost(reply.data.ntp_data.peer_dispersion),
                             UTI_FloatNetworkToHost(reply.data.ntp_data.response_time),
                             UTI_FloatNetworkToHost(reply.data.ntp_data.jitter_asymmetry),
                             ntohs(reply.data.ntp_data.flags) as libc::c_int
                                 >> 7 as libc::c_int,
                             ntohs(reply.data.ntp_data.flags) as libc::c_int
                                 >> 4 as libc::c_int,
                             ntohs(reply.data.ntp_data.flags) as libc::c_int,
                             ntohs(reply.data.ntp_data.flags) as libc::c_int &
                                 0x4000 as libc::c_int,
                             ntohs(reply.data.ntp_data.flags) as libc::c_int &
                                 0x8000 as libc::c_int,
                             reply.data.ntp_data.tx_tss_char as libc::c_int,
                             reply.data.ntp_data.rx_tss_char as libc::c_int,
                             ntohl(reply.data.ntp_data.total_tx_count) as
                                 libc::c_ulong,
                             ntohl(reply.data.ntp_data.total_rx_count) as
                                 libc::c_ulong,
                             ntohl(reply.data.ntp_data.total_valid_count) as
                                 libc::c_ulong, 0x1234 as libc::c_int);
            }
            _ => { }
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_serverstats(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut request: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut reply: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    request.command = htons(54 as libc::c_int as uint16_t);
    if request_reply(&mut request, &mut reply, 14 as libc::c_int,
                     0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    print_report(b"NTP packets received       : %U\nNTP packets dropped        : %U\nCommand packets received   : %U\nCommand packets dropped    : %U\nClient log records dropped : %U\n\x00"
                     as *const u8 as *const libc::c_char,
                 ntohl(reply.data.server_stats.ntp_hits) as libc::c_ulong,
                 ntohl(reply.data.server_stats.ntp_drops) as libc::c_ulong,
                 ntohl(reply.data.server_stats.cmd_hits) as libc::c_ulong,
                 ntohl(reply.data.server_stats.cmd_drops) as libc::c_ulong,
                 ntohl(reply.data.server_stats.log_drops) as libc::c_ulong,
                 0x1234 as libc::c_int);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_smoothing(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut request: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut reply: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    let mut flags: uint32_t = 0;
    request.command = htons(51 as libc::c_int as uint16_t);
    if request_reply(&mut request, &mut reply, 13 as libc::c_int,
                     0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    flags = ntohl(reply.data.smoothing.flags);
    print_report(b"Active         : %B %s\nOffset         : %+.9f seconds\nFrequency      : %+.6f ppm\nWander         : %+.6f ppm per second\nLast update    : %.1f seconds ago\nRemaining time : %.1f seconds\n\x00"
                     as *const u8 as *const libc::c_char,
                 (flags & 0x1 as libc::c_int as libc::c_uint != 0) as
                     libc::c_int,
                 if flags & 0x2 as libc::c_int as libc::c_uint != 0 {
                     b"(leap second only)\x00" as *const u8 as
                         *const libc::c_char
                 } else { b"\x00" as *const u8 as *const libc::c_char },
                 UTI_FloatNetworkToHost(reply.data.smoothing.offset),
                 UTI_FloatNetworkToHost(reply.data.smoothing.freq_ppm),
                 UTI_FloatNetworkToHost(reply.data.smoothing.wander_ppm),
                 UTI_FloatNetworkToHost(reply.data.smoothing.last_update_ago),
                 UTI_FloatNetworkToHost(reply.data.smoothing.remaining_time),
                 0x1234 as libc::c_int);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_smoothtime(mut msg: *mut CMD_Request,
                                            mut line: *const libc::c_char)
 -> libc::c_int {
    if strcmp(line, b"reset\x00" as *const u8 as *const libc::c_char) == 0 {
        (*msg).data.smoothtime.option =
            htonl(0 as libc::c_int as uint32_t) as int32_t
    } else if strcmp(line,
                     b"activate\x00" as *const u8 as *const libc::c_char) == 0
     {
        (*msg).data.smoothtime.option =
            htonl(1 as libc::c_int as uint32_t) as int32_t
    } else {
        LOG_Message(LOGS_ERR,
                    b"Invalid syntax for smoothtime command\x00" as *const u8
                        as *const libc::c_char);
        return 0 as libc::c_int
    }
    (*msg).command = htons(52 as libc::c_int as uint16_t);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_rtcreport(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut request: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut reply: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    let mut ref_time: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    request.command = htons(35 as libc::c_int as uint16_t);
    if request_reply(&mut request, &mut reply, 7 as libc::c_int,
                     0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    UTI_TimespecNetworkToHost(&mut reply.data.rtc.ref_time, &mut ref_time);
    print_report(b"RTC ref time (UTC) : %T\nNumber of samples  : %u\nNumber of runs     : %u\nSample span period : %I\nRTC is fast by     : %12.6f seconds\nRTC gains time at  : %9.3f ppm\n\x00"
                     as *const u8 as *const libc::c_char,
                 &mut ref_time as *mut timespec,
                 ntohs(reply.data.rtc.n_samples) as libc::c_int,
                 ntohs(reply.data.rtc.n_runs) as libc::c_int,
                 ntohl(reply.data.rtc.span_seconds) as libc::c_ulong,
                 UTI_FloatNetworkToHost(reply.data.rtc.rtc_seconds_fast),
                 UTI_FloatNetworkToHost(reply.data.rtc.rtc_gain_rate_ppm),
                 0x1234 as libc::c_int);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_clients(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut request: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut reply: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    let mut ip: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut i: uint32_t = 0;
    let mut n_clients: uint32_t = 0;
    let mut next_index: uint32_t = 0;
    let mut n_indices: uint32_t = 0;
    let mut client: *mut RPY_ClientAccesses_Client =
        0 as *mut RPY_ClientAccesses_Client;
    let mut name: [libc::c_char; 50] = [0; 50];
    next_index = 0 as libc::c_int as uint32_t;
    print_header(b"Hostname                      NTP   Drop Int IntL Last     Cmd   Drop Int  Last\x00"
                     as *const u8 as *const libc::c_char);
    loop  {
        request.command = htons(55 as libc::c_int as uint16_t);
        request.data.client_accesses_by_index.first_index = htonl(next_index);
        request.data.client_accesses_by_index.n_clients =
            htonl(8 as libc::c_int as uint32_t);
        if request_reply(&mut request, &mut reply, 15 as libc::c_int,
                         0 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
        n_clients = ntohl(reply.data.client_accesses_by_index.n_clients);
        n_indices = ntohl(reply.data.client_accesses_by_index.n_indices);
        i = 0 as libc::c_int as uint32_t;
        while i < n_clients && i < 8 as libc::c_int as libc::c_uint {
            client =
                &mut *reply.data.client_accesses_by_index.clients.as_mut_ptr().offset(i
                                                                                          as
                                                                                          isize)
                    as *mut RPY_ClientAccesses_Client;
            UTI_IPNetworkToHost(&mut (*client).ip, &mut ip);
            /* UNSPEC means the record could not be found in the daemon's tables.
         We shouldn't ever generate this case, but ignore it if we do. */
            if !(ip.family as libc::c_int == 0 as libc::c_int) {
                format_name(name.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 50]>() as
                                libc::c_ulong as libc::c_int,
                            25 as libc::c_int, 0 as libc::c_int,
                            0 as libc::c_int as uint32_t, 0 as libc::c_int,
                            &mut ip);
                print_report(b"%-25s  %6U  %5U  %C  %C  %I  %6U  %5U  %C  %I\n\x00"
                                 as *const u8 as *const libc::c_char,
                             name.as_mut_ptr(),
                             ntohl((*client).ntp_hits) as libc::c_ulong,
                             ntohl((*client).ntp_drops) as libc::c_ulong,
                             (*client).ntp_interval as libc::c_int,
                             (*client).ntp_timeout_interval as libc::c_int,
                             ntohl((*client).last_ntp_hit_ago) as
                                 libc::c_ulong,
                             ntohl((*client).cmd_hits) as libc::c_ulong,
                             ntohl((*client).cmd_drops) as libc::c_ulong,
                             (*client).cmd_interval as libc::c_int,
                             ntohl((*client).last_cmd_hit_ago) as
                                 libc::c_ulong, 0x1234 as libc::c_int);
            }
            i = i.wrapping_add(1)
        }
        /* Set the next index to probe based on what the server tells us */
        next_index = ntohl(reply.data.client_accesses_by_index.next_index);
        if next_index >= n_indices ||
               n_clients < 8 as libc::c_int as libc::c_uint {
            break ;
        }
    }
    return 1 as libc::c_int;
}
/* ================================================== */
/* Process the manual list command */
unsafe extern "C" fn process_cmd_manual_list(mut line: *const libc::c_char)
 -> libc::c_int {
    let mut request: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut reply: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    let mut i: uint32_t = 0;
    let mut n_samples: uint32_t = 0;
    let mut sample: *mut RPY_ManualListSample =
        0 as *mut RPY_ManualListSample;
    let mut when: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    request.command = htons(41 as libc::c_int as uint16_t);
    if request_reply(&mut request, &mut reply, 18 as libc::c_int,
                     0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    n_samples = ntohl(reply.data.manual_list.n_samples);
    print_info_field(b"210 n_samples = %lu\n\x00" as *const u8 as
                         *const libc::c_char, n_samples as libc::c_ulong);
    print_header(b"#    Date     Time(UTC)    Slewed   Original   Residual\x00"
                     as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as uint32_t;
    while i < n_samples && i < 16 as libc::c_int as libc::c_uint {
        sample =
            &mut *reply.data.manual_list.samples.as_mut_ptr().offset(i as
                                                                         isize)
                as *mut RPY_ManualListSample;
        UTI_TimespecNetworkToHost(&mut (*sample).when, &mut when);
        print_report(b"%2d %s %10.2f %10.2f %10.2f\n\x00" as *const u8 as
                         *const libc::c_char, i,
                     UTI_TimeToLogForm(when.tv_sec),
                     UTI_FloatNetworkToHost((*sample).slewed_offset),
                     UTI_FloatNetworkToHost((*sample).orig_offset),
                     UTI_FloatNetworkToHost((*sample).residual),
                     0x1234 as libc::c_int);
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_manual_delete(mut msg: *mut CMD_Request,
                                               mut line: *const libc::c_char)
 -> libc::c_int {
    let mut index: libc::c_int = 0;
    if sscanf(line, b"%d\x00" as *const u8 as *const libc::c_char,
              &mut index as *mut libc::c_int) != 1 as libc::c_int {
        LOG_Message(LOGS_ERR,
                    b"Bad syntax for manual delete command\x00" as *const u8
                        as *const libc::c_char);
        return 0 as libc::c_int
    }
    (*msg).command = htons(42 as libc::c_int as uint16_t);
    (*msg).data.manual_delete.index = htonl(index as uint32_t) as int32_t;
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_settime(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut now: time_t = 0;
    let mut new_time: time_t = 0;
    let mut request: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut reply: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    let mut dfreq_ppm: libc::c_double = 0.;
    let mut new_afreq_ppm: libc::c_double = 0.;
    let mut offset: libc::c_double = 0.;
    now = time(0 as *mut time_t);
    new_time = get_date(line, &mut now);
    if new_time == -(1 as libc::c_int) as libc::c_long {
        printf(b"510 - Could not parse date string\n\x00" as *const u8 as
                   *const libc::c_char);
    } else {
        ts.tv_sec = new_time;
        ts.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        UTI_TimespecHostToNetwork(&mut ts, &mut request.data.settime.ts);
        request.command = htons(11 as libc::c_int as uint16_t);
        if request_reply(&mut request, &mut reply, 17 as libc::c_int,
                         1 as libc::c_int) != 0 {
            offset =
                UTI_FloatNetworkToHost(reply.data.manual_timestamp.offset);
            dfreq_ppm =
                UTI_FloatNetworkToHost(reply.data.manual_timestamp.dfreq_ppm);
            new_afreq_ppm =
                UTI_FloatNetworkToHost(reply.data.manual_timestamp.new_afreq_ppm);
            printf(b"Clock was %.2f seconds fast.  Frequency change = %.2fppm, new frequency = %.2fppm\n\x00"
                       as *const u8 as *const libc::c_char, offset, dfreq_ppm,
                   new_afreq_ppm);
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_rekey(mut msg: *mut CMD_Request,
                                       mut line: *mut libc::c_char) {
    (*msg).command = htons(16 as libc::c_int as uint16_t);
}
/* ================================================== */
unsafe extern "C" fn process_cmd_makestep(mut msg: *mut CMD_Request,
                                          mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut limit: libc::c_int = 0;
    let mut threshold: libc::c_double = 0.;
    if *line != 0 {
        if sscanf(line, b"%lf %d\x00" as *const u8 as *const libc::c_char,
                  &mut threshold as *mut libc::c_double,
                  &mut limit as *mut libc::c_int) != 2 as libc::c_int {
            LOG_Message(LOGS_ERR,
                        b"Bad syntax for makestep command\x00" as *const u8 as
                            *const libc::c_char);
            return 0 as libc::c_int
        }
        (*msg).command = htons(50 as libc::c_int as uint16_t);
        (*msg).data.modify_makestep.limit =
            htonl(limit as uint32_t) as int32_t;
        (*msg).data.modify_makestep.threshold =
            UTI_FloatHostToNetwork(threshold)
    } else { (*msg).command = htons(43 as libc::c_int as uint16_t) }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_activity(mut line: *const libc::c_char)
 -> libc::c_int {
    let mut request: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut reply: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    request.command = htons(44 as libc::c_int as uint16_t);
    if request_reply(&mut request, &mut reply, 12 as libc::c_int,
                     0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    print_info_field(b"200 OK\n\x00" as *const u8 as *const libc::c_char);
    print_report(b"%U sources online\n%U sources offline\n%U sources doing burst (return to online)\n%U sources doing burst (return to offline)\n%U sources with unknown address\n\x00"
                     as *const u8 as *const libc::c_char,
                 ntohl(reply.data.activity.online as uint32_t) as
                     libc::c_ulong,
                 ntohl(reply.data.activity.offline as uint32_t) as
                     libc::c_ulong,
                 ntohl(reply.data.activity.burst_online as uint32_t) as
                     libc::c_ulong,
                 ntohl(reply.data.activity.burst_offline as uint32_t) as
                     libc::c_ulong,
                 ntohl(reply.data.activity.unresolved as uint32_t) as
                     libc::c_ulong, 0x1234 as libc::c_int);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_reselectdist(mut msg: *mut CMD_Request,
                                              mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut dist: libc::c_double = 0.;
    let mut ok: libc::c_int = 0;
    (*msg).command = htons(49 as libc::c_int as uint16_t);
    if sscanf(line, b"%lf\x00" as *const u8 as *const libc::c_char,
              &mut dist as *mut libc::c_double) == 1 as libc::c_int {
        (*msg).data.reselect_distance.distance = UTI_FloatHostToNetwork(dist);
        ok = 1 as libc::c_int
    } else { ok = 0 as libc::c_int }
    return ok;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_reselect(mut msg: *mut CMD_Request,
                                          mut line: *mut libc::c_char) {
    (*msg).command = htons(48 as libc::c_int as uint16_t);
}
/* ================================================== */
unsafe extern "C" fn process_cmd_refresh(mut msg: *mut CMD_Request,
                                         mut line: *mut libc::c_char) {
    (*msg).command = htons(53 as libc::c_int as uint16_t);
}
/* ================================================== */
unsafe extern "C" fn process_cmd_shutdown(mut msg: *mut CMD_Request,
                                          mut line: *mut libc::c_char) {
    (*msg).command = htons(62 as libc::c_int as uint16_t);
}
/* ================================================== */
unsafe extern "C" fn process_cmd_waitsync(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut request: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut reply: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    let mut ip_addr: IPAddr =
        IPAddr{addr: C2RustUnnamed_0{in4: 0,}, family: 0, _pad: 0,};
    let mut ref_id: uint32_t = 0;
    let mut correction: libc::c_double = 0.;
    let mut skew_ppm: libc::c_double = 0.;
    let mut max_correction: libc::c_double = 0.;
    let mut max_skew_ppm: libc::c_double = 0.;
    let mut interval: libc::c_double = 0.;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut max_tries: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut timeout: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    max_tries = 0 as libc::c_int;
    max_correction = 0.0f64;
    max_skew_ppm = 0.0f64;
    interval = 10.0f64;
    (sscanf(line, b"%d %lf %lf %lf\x00" as *const u8 as *const libc::c_char,
            &mut max_tries as *mut libc::c_int,
            &mut max_correction as *mut libc::c_double,
            &mut max_skew_ppm as *mut libc::c_double,
            &mut interval as *mut libc::c_double)) != 0;
    /* Don't allow shorter interval than 0.1 seconds */
    if interval < 0.1f64 { interval = 0.1f64 }
    request.command = htons(33 as libc::c_int as uint16_t);
    i = 1 as libc::c_int;
    loop  {
        if request_reply(&mut request, &mut reply, 5 as libc::c_int,
                         0 as libc::c_int) != 0 {
            ref_id = ntohl(reply.data.tracking.ref_id);
            UTI_IPNetworkToHost(&mut reply.data.tracking.ip_addr,
                                &mut ip_addr);
            correction =
                UTI_FloatNetworkToHost(reply.data.tracking.current_correction);
            correction = fabs(correction);
            skew_ppm = UTI_FloatNetworkToHost(reply.data.tracking.skew_ppm);
            print_report(b"try: %d, refid: %R, correction: %.9f, skew: %.3f\n\x00"
                             as *const u8 as *const libc::c_char, i,
                         ref_id as libc::c_ulong, correction, skew_ppm,
                         0x1234 as libc::c_int);
            if (ip_addr.family as libc::c_int != 0 as libc::c_int ||
                    ref_id != 0 as libc::c_int as libc::c_uint &&
                        ref_id as libc::c_long != 0x7f7f0101 as libc::c_long)
                   &&
                   (max_correction == 0.0f64 || correction <= max_correction)
                   && (max_skew_ppm == 0.0f64 || skew_ppm <= max_skew_ppm) {
                ret = 1 as libc::c_int
            }
        }
        if !(ret == 0 && (max_tries == 0 || i < max_tries) && quit == 0) {
            break ;
        }
        UTI_DoubleToTimeval(interval, &mut timeout);
        if select(0 as libc::c_int, 0 as *mut fd_set, 0 as *mut fd_set,
                  0 as *mut fd_set, &mut timeout) != 0 {
            break ;
        }
        i += 1
    }
    return ret;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_dns(mut line: *const libc::c_char)
 -> libc::c_int {
    if strcmp(line, b"-46\x00" as *const u8 as *const libc::c_char) == 0 {
        DNS_SetAddressFamily(0 as libc::c_int);
    } else if strcmp(line, b"-4\x00" as *const u8 as *const libc::c_char) == 0
     {
        DNS_SetAddressFamily(1 as libc::c_int);
    } else if strcmp(line, b"-6\x00" as *const u8 as *const libc::c_char) == 0
     {
        DNS_SetAddressFamily(2 as libc::c_int);
    } else if strcmp(line, b"-n\x00" as *const u8 as *const libc::c_char) == 0
     {
        no_dns = 1 as libc::c_int
    } else if strcmp(line, b"+n\x00" as *const u8 as *const libc::c_char) == 0
     {
        no_dns = 0 as libc::c_int
    } else {
        LOG_Message(LOGS_ERR,
                    b"Unrecognized dns command\x00" as *const u8 as
                        *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_timeout(mut line: *const libc::c_char)
 -> libc::c_int {
    let mut timeout: libc::c_int = 0;
    timeout = atoi(line);
    if timeout < 100 as libc::c_int {
        LOG_Message(LOGS_ERR,
                    b"Timeout %d is too short\x00" as *const u8 as
                        *const libc::c_char, timeout);
        return 0 as libc::c_int
    }
    initial_timeout = timeout;
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_retries(mut line: *const libc::c_char)
 -> libc::c_int {
    let mut retries: libc::c_int = 0;
    retries = atoi(line);
    if retries < 0 as libc::c_int || retries > 30 as libc::c_int {
        LOG_Message(LOGS_ERR,
                    b"Invalid maximum number of retries\x00" as *const u8 as
                        *const libc::c_char);
        return 0 as libc::c_int
    }
    max_retries = retries;
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_cmd_keygen(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut key: [libc::c_uchar; 512] = [0; 512];
    let mut type_0: [libc::c_char; 17] = [0; 17];
    let mut i: libc::c_uint = 0;
    let mut cmac_length: libc::c_uint = 0;
    let mut length: libc::c_uint = 0;
    let mut id: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut bits: libc::c_uint = 160 as libc::c_int as libc::c_uint;
    snprintf(type_0.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong,
             b"MD5\x00" as *const u8 as *const libc::c_char);
    (sscanf(line, b"%u %16s %u\x00" as *const u8 as *const libc::c_char,
            &mut id as *mut libc::c_uint, type_0.as_mut_ptr(),
            &mut bits as *mut libc::c_uint)) != 0;
    cmac_length = 0 as libc::c_int as libc::c_uint;
    if HSH_GetHashId(type_0.as_mut_ptr()) >= 0 as libc::c_int {
        length =
            bits.wrapping_add(7 as libc::c_int as
                                  libc::c_uint).wrapping_div(8 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
    } else if cmac_length > 0 as libc::c_int as libc::c_uint {
        length = cmac_length
    } else {
        LOG_Message(LOGS_ERR,
                    b"Unknown hash function or cipher %s\x00" as *const u8 as
                        *const libc::c_char, type_0.as_mut_ptr());
        return 0 as libc::c_int
    }
    length =
        if 10 as libc::c_int as libc::c_ulong >
               (if (length as libc::c_ulong) <
                       ::std::mem::size_of::<[libc::c_uchar; 512]>() as
                           libc::c_ulong {
                    length as libc::c_ulong
                } else {
                    ::std::mem::size_of::<[libc::c_uchar; 512]>() as
                        libc::c_ulong
                }) {
            10 as libc::c_int as libc::c_ulong
        } else if (length as libc::c_ulong) <
                      ::std::mem::size_of::<[libc::c_uchar; 512]>() as
                          libc::c_ulong {
            length as libc::c_ulong
        } else {
            ::std::mem::size_of::<[libc::c_uchar; 512]>() as libc::c_ulong
        } as libc::c_uint;
    UTI_GetRandomBytesUrandom(key.as_mut_ptr() as *mut libc::c_void, length);
    printf(b"%u %s HEX:\x00" as *const u8 as *const libc::c_char, id,
           type_0.as_mut_ptr());
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        printf(b"%02hhX\x00" as *const u8 as *const libc::c_char,
               key[i as usize] as libc::c_int);
        i = i.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_line(mut line: *mut libc::c_char)
 -> libc::c_int {
    let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut do_normal_submit: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tx_message: CMD_Request =
        CMD_Request{version: 0,
                    pkt_type: 0,
                    res1: 0,
                    res2: 0,
                    command: 0,
                    attempt: 0,
                    sequence: 0,
                    pad1: 0,
                    pad2: 0,
                    data: C2RustUnnamed_1{null: REQ_Null{EOR: 0,},},
                    padding: [0; 396],};
    let mut rx_message: CMD_Reply =
        CMD_Reply{version: 0,
                  pkt_type: 0,
                  res1: 0,
                  res2: 0,
                  command: 0,
                  reply: 0,
                  status: 0,
                  pad1: 0,
                  pad2: 0,
                  pad3: 0,
                  sequence: 0,
                  pad4: 0,
                  pad5: 0,
                  data: C2RustUnnamed_2{null: RPY_Null{EOR: 0,},},};
    ret = 0 as libc::c_int;
    do_normal_submit = 1 as libc::c_int;
    CPS_NormalizeLine(line);
    if *line == 0 { fflush(stderr); fflush(stdout); return 1 as libc::c_int }
    command = line;
    line = CPS_SplitWord(line);
    if strcmp(command, b"accheck\x00" as *const u8 as *const libc::c_char) ==
           0 {
        do_normal_submit = process_cmd_accheck(&mut tx_message, line)
    } else if strcmp(command,
                     b"activity\x00" as *const u8 as *const libc::c_char) == 0
     {
        do_normal_submit = 0 as libc::c_int;
        ret = process_cmd_activity(line)
    } else if strcmp(command, b"add\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        do_normal_submit = process_cmd_add_source(&mut tx_message, line)
    } else if strcmp(command,
                     b"allow\x00" as *const u8 as *const libc::c_char) == 0 {
        if strncmp(line, b"all\x00" as *const u8 as *const libc::c_char,
                   3 as libc::c_int as libc::c_ulong) == 0 {
            do_normal_submit =
                process_cmd_allowall(&mut tx_message, CPS_SplitWord(line))
        } else { do_normal_submit = process_cmd_allow(&mut tx_message, line) }
    } else if strcmp(command,
                     b"burst\x00" as *const u8 as *const libc::c_char) == 0 {
        do_normal_submit = process_cmd_burst(&mut tx_message, line)
    } else if strcmp(command,
                     b"clients\x00" as *const u8 as *const libc::c_char) == 0
     {
        ret = process_cmd_clients(line);
        do_normal_submit = 0 as libc::c_int
    } else if strcmp(command,
                     b"cmdaccheck\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        do_normal_submit = process_cmd_cmdaccheck(&mut tx_message, line)
    } else if strcmp(command,
                     b"cmdallow\x00" as *const u8 as *const libc::c_char) == 0
     {
        if strncmp(line, b"all\x00" as *const u8 as *const libc::c_char,
                   3 as libc::c_int as libc::c_ulong) == 0 {
            do_normal_submit =
                process_cmd_cmdallowall(&mut tx_message, CPS_SplitWord(line))
        } else {
            do_normal_submit = process_cmd_cmdallow(&mut tx_message, line)
        }
    } else if strcmp(command,
                     b"cmddeny\x00" as *const u8 as *const libc::c_char) == 0
     {
        if strncmp(line, b"all\x00" as *const u8 as *const libc::c_char,
                   3 as libc::c_int as libc::c_ulong) == 0 {
            line = CPS_SplitWord(line);
            do_normal_submit = process_cmd_cmddenyall(&mut tx_message, line)
        } else {
            do_normal_submit = process_cmd_cmddeny(&mut tx_message, line)
        }
    } else if strcmp(command,
                     b"cyclelogs\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        process_cmd_cyclelogs(&mut tx_message, line);
    } else if strcmp(command,
                     b"delete\x00" as *const u8 as *const libc::c_char) == 0 {
        do_normal_submit = process_cmd_delete(&mut tx_message, line)
    } else if strcmp(command, b"deny\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        if strncmp(line, b"all\x00" as *const u8 as *const libc::c_char,
                   3 as libc::c_int as libc::c_ulong) == 0 {
            do_normal_submit =
                process_cmd_denyall(&mut tx_message, CPS_SplitWord(line))
        } else { do_normal_submit = process_cmd_deny(&mut tx_message, line) }
    } else if strcmp(command,
                     b"dfreq\x00" as *const u8 as *const libc::c_char) == 0 {
        process_cmd_dfreq(&mut tx_message, line);
    } else if strcmp(command, b"dns\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        ret = process_cmd_dns(line);
        do_normal_submit = 0 as libc::c_int
    } else if strcmp(command,
                     b"doffset\x00" as *const u8 as *const libc::c_char) == 0
     {
        process_cmd_doffset(&mut tx_message, line);
    } else if strcmp(command, b"dump\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        process_cmd_dump(&mut tx_message, line);
    } else if strcmp(command, b"exit\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        do_normal_submit = 0 as libc::c_int;
        quit = 1 as libc::c_int;
        ret = 1 as libc::c_int
    } else if strcmp(command, b"help\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        do_normal_submit = 0 as libc::c_int;
        give_help();
        ret = 1 as libc::c_int
    } else if strcmp(command,
                     b"keygen\x00" as *const u8 as *const libc::c_char) == 0 {
        ret = process_cmd_keygen(line);
        do_normal_submit = 0 as libc::c_int
    } else if strcmp(command,
                     b"local\x00" as *const u8 as *const libc::c_char) == 0 {
        do_normal_submit = process_cmd_local(&mut tx_message, line)
    } else if strcmp(command,
                     b"makestep\x00" as *const u8 as *const libc::c_char) == 0
     {
        do_normal_submit = process_cmd_makestep(&mut tx_message, line)
    } else if strcmp(command,
                     b"manual\x00" as *const u8 as *const libc::c_char) == 0 {
        if strncmp(line, b"list\x00" as *const u8 as *const libc::c_char,
                   4 as libc::c_int as libc::c_ulong) == 0 {
            do_normal_submit = 0 as libc::c_int;
            ret = process_cmd_manual_list(CPS_SplitWord(line))
        } else if strncmp(line,
                          b"delete\x00" as *const u8 as *const libc::c_char,
                          6 as libc::c_int as libc::c_ulong) == 0 {
            do_normal_submit =
                process_cmd_manual_delete(&mut tx_message,
                                          CPS_SplitWord(line))
        } else {
            do_normal_submit = process_cmd_manual(&mut tx_message, line)
        }
    } else if strcmp(command,
                     b"maxdelay\x00" as *const u8 as *const libc::c_char) == 0
     {
        do_normal_submit = process_cmd_maxdelay(&mut tx_message, line)
    } else if strcmp(command,
                     b"maxdelaydevratio\x00" as *const u8 as
                         *const libc::c_char) == 0 {
        do_normal_submit = process_cmd_maxdelaydevratio(&mut tx_message, line)
    } else if strcmp(command,
                     b"maxdelayratio\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        do_normal_submit = process_cmd_maxdelayratio(&mut tx_message, line)
    } else if strcmp(command,
                     b"maxpoll\x00" as *const u8 as *const libc::c_char) == 0
     {
        do_normal_submit = process_cmd_maxpoll(&mut tx_message, line)
    } else if strcmp(command,
                     b"maxupdateskew\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        do_normal_submit = process_cmd_maxupdateskew(&mut tx_message, line)
    } else if strcmp(command,
                     b"minpoll\x00" as *const u8 as *const libc::c_char) == 0
     {
        do_normal_submit = process_cmd_minpoll(&mut tx_message, line)
    } else if strcmp(command,
                     b"minstratum\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        do_normal_submit = process_cmd_minstratum(&mut tx_message, line)
    } else if strcmp(command,
                     b"ntpdata\x00" as *const u8 as *const libc::c_char) == 0
     {
        do_normal_submit = 0 as libc::c_int;
        ret = process_cmd_ntpdata(line)
    } else if strcmp(command,
                     b"offline\x00" as *const u8 as *const libc::c_char) == 0
     {
        do_normal_submit = process_cmd_offline(&mut tx_message, line)
    } else if strcmp(command,
                     b"online\x00" as *const u8 as *const libc::c_char) == 0 {
        do_normal_submit = process_cmd_online(&mut tx_message, line)
    } else if strcmp(command,
                     b"onoffline\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        process_cmd_onoffline(&mut tx_message, line);
    } else if strcmp(command,
                     b"polltarget\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        do_normal_submit = process_cmd_polltarget(&mut tx_message, line)
    } else if strcmp(command, b"quit\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        do_normal_submit = 0 as libc::c_int;
        quit = 1 as libc::c_int;
        ret = 1 as libc::c_int
    } else if strcmp(command,
                     b"refresh\x00" as *const u8 as *const libc::c_char) == 0
     {
        process_cmd_refresh(&mut tx_message, line);
    } else if strcmp(command,
                     b"rekey\x00" as *const u8 as *const libc::c_char) == 0 {
        process_cmd_rekey(&mut tx_message, line);
    } else if strcmp(command,
                     b"reselect\x00" as *const u8 as *const libc::c_char) == 0
     {
        process_cmd_reselect(&mut tx_message, line);
    } else if strcmp(command,
                     b"reselectdist\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        do_normal_submit = process_cmd_reselectdist(&mut tx_message, line)
    } else if strcmp(command,
                     b"retries\x00" as *const u8 as *const libc::c_char) == 0
     {
        ret = process_cmd_retries(line);
        do_normal_submit = 0 as libc::c_int
    } else if strcmp(command,
                     b"rtcdata\x00" as *const u8 as *const libc::c_char) == 0
     {
        do_normal_submit = 0 as libc::c_int;
        ret = process_cmd_rtcreport(line)
    } else if strcmp(command,
                     b"serverstats\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        do_normal_submit = 0 as libc::c_int;
        ret = process_cmd_serverstats(line)
    } else if strcmp(command,
                     b"settime\x00" as *const u8 as *const libc::c_char) == 0
     {
        do_normal_submit = 0 as libc::c_int;
        ret = process_cmd_settime(line)
    } else if strcmp(command,
                     b"shutdown\x00" as *const u8 as *const libc::c_char) == 0
     {
        process_cmd_shutdown(&mut tx_message, line);
    } else if strcmp(command,
                     b"smoothing\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        do_normal_submit = 0 as libc::c_int;
        ret = process_cmd_smoothing(line)
    } else if strcmp(command,
                     b"smoothtime\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        do_normal_submit = process_cmd_smoothtime(&mut tx_message, line)
    } else if strcmp(command,
                     b"sourcename\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        do_normal_submit = 0 as libc::c_int;
        ret = process_cmd_sourcename(line)
    } else if strcmp(command,
                     b"sources\x00" as *const u8 as *const libc::c_char) == 0
     {
        do_normal_submit = 0 as libc::c_int;
        ret = process_cmd_sources(line)
    } else if strcmp(command,
                     b"sourcestats\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        do_normal_submit = 0 as libc::c_int;
        ret = process_cmd_sourcestats(line)
    } else if strcmp(command,
                     b"timeout\x00" as *const u8 as *const libc::c_char) == 0
     {
        ret = process_cmd_timeout(line);
        do_normal_submit = 0 as libc::c_int
    } else if strcmp(command,
                     b"tracking\x00" as *const u8 as *const libc::c_char) == 0
     {
        ret = process_cmd_tracking(line);
        do_normal_submit = 0 as libc::c_int
    } else if strcmp(command,
                     b"trimrtc\x00" as *const u8 as *const libc::c_char) == 0
     {
        process_cmd_trimrtc(&mut tx_message, line);
    } else if strcmp(command,
                     b"waitsync\x00" as *const u8 as *const libc::c_char) == 0
     {
        ret = process_cmd_waitsync(line);
        do_normal_submit = 0 as libc::c_int
    } else if strcmp(command,
                     b"writertc\x00" as *const u8 as *const libc::c_char) == 0
     {
        process_cmd_writertc(&mut tx_message, line);
    } else if strcmp(command,
                     b"authhash\x00" as *const u8 as *const libc::c_char) == 0
                  ||
                  strcmp(command,
                         b"password\x00" as *const u8 as *const libc::c_char)
                      == 0 {
        /* Warn, but don't return error to not break scripts */
        LOG_Message(LOGS_WARN,
                    b"Authentication is no longer supported\x00" as *const u8
                        as *const libc::c_char);
        do_normal_submit = 0 as libc::c_int;
        ret = 1 as libc::c_int
    } else {
        LOG_Message(LOGS_ERR,
                    b"Unrecognized command\x00" as *const u8 as
                        *const libc::c_char);
        do_normal_submit = 0 as libc::c_int
    }
    if do_normal_submit != 0 {
        ret =
            request_reply(&mut tx_message, &mut rx_message, 1 as libc::c_int,
                          1 as libc::c_int)
    }
    fflush(stderr);
    fflush(stdout);
    return ret;
}
/* ================================================== */
unsafe extern "C" fn process_args(mut argc: libc::c_int,
                                  mut argv: *mut *mut libc::c_char,
                                  mut multi: libc::c_int) -> libc::c_int {
    let mut total_length: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    total_length = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < argc {
        total_length =
            (total_length as
                 libc::c_ulong).wrapping_add(strlen(*argv.offset(i as
                                                                     isize)).wrapping_add(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong))
                as libc::c_int as libc::c_int;
        i += 1
    }
    line =
        Malloc(((2 as libc::c_int + total_length) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < argc {
        *line.offset(0 as libc::c_int as isize) =
            '\u{0}' as i32 as libc::c_char;
        if multi != 0 {
            strcat(line, *argv.offset(i as isize));
        } else {
            while i < argc {
                strcat(line, *argv.offset(i as isize));
                if (i + 1 as libc::c_int) < argc {
                    strcat(line,
                           b" \x00" as *const u8 as *const libc::c_char);
                }
                i += 1
            }
        }
        ret = process_line(line);
        if ret == 0 || quit != 0 { break ; }
        i += 1
    }
    free(line as *mut libc::c_void);
    return ret;
}
/* ================================================== */
unsafe extern "C" fn signal_handler(mut signum: libc::c_int) {
    quit = 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn display_gpl() {
    printf(b"chrony version %s\nCopyright (C) 1997-2003, 2007, 2009-2019 Richard P. Curnow and others\nchrony comes with ABSOLUTELY NO WARRANTY.  This is free software, and\nyou are welcome to redistribute it under certain conditions.  See the\nGNU General Public License version 2 for details.\n\n\x00"
               as *const u8 as *const libc::c_char,
           b"DEVELOPMENT\x00" as *const u8 as *const libc::c_char);
}
/* ================================================== */
unsafe extern "C" fn print_help(mut progname: *const libc::c_char) {
    printf(b"Usage: %s [-h HOST] [-p PORT] [-n] [-N] [-c] [-d] [-4|-6] [-m] [COMMAND]\n\x00"
               as *const u8 as *const libc::c_char, progname);
}
/* ================================================== */
unsafe extern "C" fn print_version() {
    printf(b"chronyc (chrony) version %s (%s)\n\x00" as *const u8 as
               *const libc::c_char,
           b"DEVELOPMENT\x00" as *const u8 as *const libc::c_char,
           b"-READLINE -SECHASH +IPV6 -DEBUG\x00" as *const u8 as
               *const libc::c_char);
}
/* ================================================== */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut progname: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    let mut hostnames: *const libc::c_char = 0 as *const libc::c_char;
    let mut opt: libc::c_int = 0;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut multi: libc::c_int = 0 as libc::c_int;
    let mut family: libc::c_int = 0 as libc::c_int;
    let mut port: libc::c_int = 323 as libc::c_int;
    /* Parse (undocumented) long command-line options */
    optind = 1 as libc::c_int;
    while optind < argc {
        if strcmp(b"--help\x00" as *const u8 as *const libc::c_char,
                  *argv.offset(optind as isize)) == 0 {
            print_help(progname);
            return 0 as libc::c_int
        } else {
            if strcmp(b"--version\x00" as *const u8 as *const libc::c_char,
                      *argv.offset(optind as isize)) == 0 {
                print_version();
                return 0 as libc::c_int
            }
        }
        optind += 1
    }
    optind = 1 as libc::c_int;
    loop 
         /* Parse short command-line options */
         {
        opt =
            getopt(argc, argv,
                   b"+46acdf:h:mnNp:v\x00" as *const u8 as
                       *const libc::c_char);
        if !(opt != -(1 as libc::c_int)) { break ; }
        match opt {
            52 | 54 => {
                family =
                    if opt == '4' as i32 {
                        1 as libc::c_int
                    } else { 2 as libc::c_int }
            }
            99 => { csv_mode = 1 as libc::c_int }
            97 | 102 | 100 => { }
            104 => { hostnames = optarg }
            109 => { multi = 1 as libc::c_int }
            110 => { no_dns = 1 as libc::c_int }
            78 => { source_names = 1 as libc::c_int }
            112 => { port = atoi(optarg) }
            118 => { print_version(); return 0 as libc::c_int }
            _ => { print_help(progname); return 1 as libc::c_int }
        }
    }
    if isatty(0 as libc::c_int) != 0 && isatty(1 as libc::c_int) != 0 &&
           isatty(2 as libc::c_int) != 0 {
        on_terminal = 1 as libc::c_int
    }
    if on_terminal != 0 && optind == argc { display_gpl(); }
    DNS_SetAddressFamily(family);
    if hostnames.is_null() {
        hostnames =
            b"/var/run/chrony/chronyd.sock,127.0.0.1,::1\x00" as *const u8 as
                *const libc::c_char
    }
    UTI_SetQuitSignalsHandler(Some(signal_handler as
                                       unsafe extern "C" fn(_: libc::c_int)
                                           -> ()), 0 as libc::c_int);
    SCK_Initialise();
    server_addresses = get_addresses(hostnames, port);
    if open_io() == 0 {
        LOG_Message(LOGS_FATAL,
                    b"Could not open connection to daemon\x00" as *const u8 as
                        *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if optind < argc {
        ret = process_args(argc - optind, argv.offset(optind as isize), multi)
    } else {
        loop  {
            line = read_line();
            if !line.is_null() && quit == 0 {
                ret = process_line(line)
            } else if on_terminal != 0 {
                printf(b"\n\x00" as *const u8 as *const libc::c_char);
            }
            if !(!line.is_null() && quit == 0) { break ; }
        }
    }
    close_io();
    free_addresses(server_addresses);
    SCK_Finalise();
    return (ret == 0) as libc::c_int;
}

// TODO: build the client too
// #[main]
// pub fn main() {
//     let mut args: Vec<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
//     };
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
//                                     args.as_mut_ptr() as
//                                         *mut *mut libc::c_char) as i32)
//     }
// }
/* supply the final '\n' when user exits via ^D */
