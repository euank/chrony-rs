#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ARR_Instance_Record;
    pub type CMC_Instance_Record;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn strtol(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char,
              __base: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn bsearch(__key: *const libc::c_void, __base: *const libc::c_void,
               __nmemb: size_t, __size: size_t, __compar: __compar_fn_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* Create a new array with given element size */
    #[no_mangle]
    fn ARR_CreateInstance(elem_size: libc::c_uint) -> ARR_Instance;
    /* Destroy the array */
    #[no_mangle]
    fn ARR_DestroyInstance(array: ARR_Instance);
    /* Return element with given index */
    #[no_mangle]
    fn ARR_GetElement(array: ARR_Instance, index: libc::c_uint)
     -> *mut libc::c_void;
    /* Return pointer to the internal array of elements */
    #[no_mangle]
    fn ARR_GetElements(array: ARR_Instance) -> *mut libc::c_void;
    /* Add a new element to the end of the array */
    #[no_mangle]
    fn ARR_AppendElement(array: ARR_Instance, element: *mut libc::c_void);
    /* Set the size of the array */
    #[no_mangle]
    fn ARR_SetSize(array: ARR_Instance, size: libc::c_uint);
    /* Return current size of the array */
    #[no_mangle]
    fn ARR_GetSize(array: ARR_Instance) -> libc::c_uint;
    #[no_mangle]
    fn CMC_GetKeyLength(cipher: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn CMC_CreateInstance(cipher: *const libc::c_char,
                          key: *const libc::c_uchar, length: libc::c_uint)
     -> CMC_Instance;
    #[no_mangle]
    fn CMC_Hash(inst: CMC_Instance, in_0: *const libc::c_uchar,
                in_len: libc::c_uint, out: *mut libc::c_uchar,
                out_len: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn CMC_DestroyInstance(inst: CMC_Instance);
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
    fn HSH_Hash(id: libc::c_int, in1: *const libc::c_uchar,
                in1_len: libc::c_uint, in2: *const libc::c_uchar,
                in2_len: libc::c_uint, out: *mut libc::c_uchar,
                out_len: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn HSH_GetHashId(name: *const libc::c_char) -> libc::c_int;
    /* Parse a key from keyfile */
    #[no_mangle]
    fn CPS_ParseKey(line: *mut libc::c_char, id: *mut uint32_t,
                    type_0: *mut *const libc::c_char,
                    key: *mut *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn CPS_NormalizeLine(line: *mut libc::c_char);
    #[no_mangle]
    fn CNF_GetKeysFile() -> *mut libc::c_char;
    #[no_mangle]
    fn Malloc2(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec)
     -> libc::c_double;
    #[no_mangle]
    fn UTI_OpenFile(basedir: *const libc::c_char, name: *const libc::c_char,
                    suffix: *const libc::c_char, mode: libc::c_char,
                    perm: mode_t) -> *mut FILE;
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
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
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
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
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
pub struct Key {
    pub id: uint32_t,
    pub class: KeyClass,
    pub data: C2RustUnnamed,
    pub auth_delay: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ntp_mac: C2RustUnnamed_0,
    pub cmac: CMC_Instance,
}
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2019
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

  Header file for CMAC.

  */
pub type CMC_Instance = *mut CMC_Instance_Record;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub value: *mut libc::c_uchar,
    pub length: libc::c_int,
    pub hash_id: libc::c_int,
}
pub type KeyClass = libc::c_uint;
pub const CMAC: KeyClass = 1;
pub const NTP_MAC: KeyClass = 0;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_Packet {
    pub lvm: uint8_t,
    pub stratum: uint8_t,
    pub poll: int8_t,
    pub precision: int8_t,
    pub root_delay: NTP_int32,
    pub root_dispersion: NTP_int32,
    pub reference_id: NTP_int32,
    pub reference_ts: NTP_int64,
    pub originate_ts: NTP_int64,
    pub receive_ts: NTP_int64,
    pub transmit_ts: NTP_int64,
    pub auth_keyid: NTP_int32,
    pub auth_data: [uint8_t; 64],
}
pub type NTP_int32 = uint32_t;
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

  Header file containing common NTP bits and pieces
  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_int64 {
    pub hi: uint32_t,
    pub lo: uint32_t,
}
static mut keys: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
static mut cache_valid: libc::c_int = 0;
static mut cache_key_id: uint32_t = 0;
static mut cache_key_pos: libc::c_int = 0;
/* ================================================== */
unsafe extern "C" fn free_keys() {
    let mut i: libc::c_uint = 0;
    let mut key: *mut Key = 0 as *mut Key;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(keys) {
        key = ARR_GetElement(keys, i) as *mut Key;
        match (*key).class as libc::c_uint {
            0 => { free((*key).data.ntp_mac.value as *mut libc::c_void); }
            1 => { CMC_DestroyInstance((*key).data.cmac); }
            _ => {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"keys.c\x00" as *const u8 as
                                  *const libc::c_char,
                              89 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 21],
                                                        &[libc::c_char; 21]>(b"void free_keys(void)\x00")).as_ptr());
            }
        }
        i = i.wrapping_add(1)
    }
    ARR_SetSize(keys, 0 as libc::c_int as libc::c_uint);
    cache_valid = 0 as libc::c_int;
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

  Header for key management module
  */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn KEY_Initialise() {
    keys =
        ARR_CreateInstance(::std::mem::size_of::<Key>() as libc::c_ulong as
                               libc::c_uint);
    cache_valid = 0 as libc::c_int;
    KEY_Reload();
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn KEY_Finalise() {
    free_keys();
    ARR_DestroyInstance(keys);
}
/* ================================================== */
unsafe extern "C" fn get_key(mut index: libc::c_uint) -> *mut Key {
    return (ARR_GetElements(keys) as *mut Key).offset(index as isize);
}
/* ================================================== */
unsafe extern "C" fn determine_hash_delay(mut key_id: uint32_t)
 -> libc::c_int {
    let mut pkt: NTP_Packet =
        NTP_Packet{lvm: 0,
                   stratum: 0,
                   poll: 0,
                   precision: 0,
                   root_delay: 0,
                   root_dispersion: 0,
                   reference_id: 0,
                   reference_ts: NTP_int64{hi: 0, lo: 0,},
                   originate_ts: NTP_int64{hi: 0, lo: 0,},
                   receive_ts: NTP_int64{hi: 0, lo: 0,},
                   transmit_ts: NTP_int64{hi: 0, lo: 0,},
                   auth_keyid: 0,
                   auth_data: [0; 64],};
    let mut before: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut after: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut diff: libc::c_double = 0.;
    let mut min_diff: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut nsecs: libc::c_int = 0;
    memset(&mut pkt as *mut NTP_Packet as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<NTP_Packet>() as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        LCL_ReadRawTime(&mut before);
        KEY_GenerateAuth(key_id,
                         &mut pkt as *mut NTP_Packet as *mut libc::c_uchar,
                         48 as libc::c_ulong as libc::c_int,
                         &mut pkt.auth_data as *mut [uint8_t; 64] as
                             *mut libc::c_uchar,
                         ::std::mem::size_of::<[uint8_t; 64]>() as
                             libc::c_ulong as libc::c_int);
        LCL_ReadRawTime(&mut after);
        diff = UTI_DiffTimespecsToDouble(&mut after, &mut before);
        if i == 0 as libc::c_int || min_diff > diff { min_diff = diff }
        i += 1
    }
    nsecs = (1.0e9f64 * min_diff) as libc::c_int;
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"authentication delay for key %u: %d nsecs\x00" as
                        *const u8 as *const libc::c_char, key_id, nsecs);
    }
    return nsecs;
}
/* ================================================== */
/* Decode key encoded in ASCII or HEX */
unsafe extern "C" fn decode_key(mut key: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = strlen(key) as libc::c_int;
    let mut buf: [libc::c_char; 3] = [0; 3];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if strncmp(key, b"ASCII:\x00" as *const u8 as *const libc::c_char,
               6 as libc::c_int as libc::c_ulong) == 0 {
        memmove(key as *mut libc::c_void,
                key.offset(6 as libc::c_int as isize) as *const libc::c_void,
                (len - 6 as libc::c_int) as libc::c_ulong);
        return len - 6 as libc::c_int
    } else if strncmp(key, b"HEX:\x00" as *const u8 as *const libc::c_char,
                      4 as libc::c_int as libc::c_ulong) == 0 {
        if (len - 4 as libc::c_int) % 2 as libc::c_int != 0 {
            return 0 as libc::c_int
        }
        i = 0 as libc::c_int;
        j = 4 as libc::c_int;
        while (j + 1 as libc::c_int) < len {
            buf[0 as libc::c_int as usize] = *key.offset(j as isize);
            buf[1 as libc::c_int as usize] =
                *key.offset((j + 1 as libc::c_int) as isize);
            buf[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
            *key.offset(i as isize) =
                strtol(buf.as_mut_ptr(), &mut p, 16 as libc::c_int) as
                    libc::c_char;
            if p != buf.as_mut_ptr().offset(2 as libc::c_int as isize) {
                return 0 as libc::c_int
            }
            i += 1;
            j += 2 as libc::c_int
        }
        return i
    } else {
        /* assume ASCII */
        return len
    };
}
/* ================================================== */
/* Compare two keys */
unsafe extern "C" fn compare_keys_by_id(mut a: *const libc::c_void,
                                        mut b: *const libc::c_void)
 -> libc::c_int {
    let mut c: *const Key = a as *const Key;
    let mut d: *const Key = b as *const Key;
    if (*c).id < (*d).id {
        return -(1 as libc::c_int)
    } else if (*c).id > (*d).id {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn KEY_Reload() {
    let mut i: libc::c_uint = 0;
    let mut line_number: libc::c_uint = 0;
    let mut key_length: libc::c_uint = 0;
    let mut cmac_key_length: libc::c_uint = 0;
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut line: [libc::c_char; 2048] = [0; 2048];
    let mut key_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key_value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key_type: *const libc::c_char = 0 as *const libc::c_char;
    let mut hash_id: libc::c_int = 0;
    let mut key: Key =
        Key{id: 0,
            class: NTP_MAC,
            data:
                C2RustUnnamed{ntp_mac:
                                  C2RustUnnamed_0{value:
                                                      0 as *mut libc::c_uchar,
                                                  length: 0,
                                                  hash_id: 0,},},
            auth_delay: 0,};
    free_keys();
    key_file = CNF_GetKeysFile();
    line_number = 0 as libc::c_int as libc::c_uint;
    if key_file.is_null() { return }
    in_0 =
        UTI_OpenFile(0 as *const libc::c_char, key_file,
                     0 as *const libc::c_char, 'r' as i32 as libc::c_char,
                     0 as libc::c_int as mode_t);
    if in_0.is_null() {
        LOG_Message(LOGS_WARN,
                    b"Could not open keyfile %s\x00" as *const u8 as
                        *const libc::c_char, key_file);
        return
    }
    while !fgets(line.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 2048]>() as
                     libc::c_ulong as libc::c_int, in_0).is_null() {
        line_number = line_number.wrapping_add(1);
        CPS_NormalizeLine(line.as_mut_ptr());
        if *line.as_mut_ptr() == 0 { continue ; }
        memset(&mut key as *mut Key as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<Key>() as libc::c_ulong);
        if CPS_ParseKey(line.as_mut_ptr(), &mut key.id, &mut key_type,
                        &mut key_value) == 0 {
            LOG_Message(LOGS_WARN,
                        b"Could not parse key at line %u in file %s\x00" as
                            *const u8 as *const libc::c_char, line_number,
                        key_file);
        } else {
            key_length = decode_key(key_value) as libc::c_uint;
            if key_length == 0 as libc::c_int as libc::c_uint {
                LOG_Message(LOGS_WARN,
                            b"Could not decode key %u\x00" as *const u8 as
                                *const libc::c_char, key.id);
            } else {
                hash_id = HSH_GetHashId(key_type);
                cmac_key_length = CMC_GetKeyLength(key_type);
                if hash_id >= 0 as libc::c_int {
                    key.class = NTP_MAC;
                    key.data.ntp_mac.value =
                        Malloc2(key_length as size_t,
                                ::std::mem::size_of::<libc::c_uchar>() as
                                    libc::c_ulong) as *mut libc::c_uchar;
                    memcpy(key.data.ntp_mac.value as *mut libc::c_void,
                           key_value as *const libc::c_void,
                           key_length as libc::c_ulong);
                    key.data.ntp_mac.length = key_length as libc::c_int;
                    key.data.ntp_mac.hash_id = hash_id
                } else if cmac_key_length > 0 as libc::c_int as libc::c_uint {
                    if cmac_key_length != key_length {
                        LOG_Message(LOGS_WARN,
                                    b"Invalid length of %s key %u (expected %u bits)\x00"
                                        as *const u8 as *const libc::c_char,
                                    key_type, key.id,
                                    (8 as libc::c_int as
                                         libc::c_uint).wrapping_mul(cmac_key_length));
                        continue ;
                    } else {
                        key.class = CMAC;
                        key.data.cmac =
                            CMC_CreateInstance(key_type,
                                               key_value as
                                                   *mut libc::c_uchar,
                                               key_length);
                        if !key.data.cmac.is_null() {
                        } else {
                            __assert_fail(b"key.data.cmac\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"keys.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          270 as libc::c_int as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 22],
                                                                    &[libc::c_char; 22]>(b"void KEY_Reload(void)\x00")).as_ptr());
                        }
                    }
                } else {
                    LOG_Message(LOGS_WARN,
                                b"Unknown hash function or cipher in key %u\x00"
                                    as *const u8 as *const libc::c_char,
                                key.id);
                    continue ;
                }
                ARR_AppendElement(keys,
                                  &mut key as *mut Key as *mut libc::c_void);
            }
        }
    }
    fclose(in_0);
    /* Sort keys into order.  Note, if there's a duplicate, it is
     arbitrary which one we use later - the user should have been
     more careful! */
    qsort(ARR_GetElements(keys), ARR_GetSize(keys) as size_t,
          ::std::mem::size_of::<Key>() as libc::c_ulong,
          Some(compare_keys_by_id as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    /* Check for duplicates */
    i = 1 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(keys) {
        if (*get_key(i.wrapping_sub(1 as libc::c_int as libc::c_uint))).id ==
               (*get_key(i)).id {
            LOG_Message(LOGS_WARN,
                        b"Detected duplicate key %u\x00" as *const u8 as
                            *const libc::c_char,
                        (*get_key(i.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint))).id);
        }
        i = i.wrapping_add(1)
    }
    /* Erase any passwords from stack */
    memset(line.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong);
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(keys) {
        (*get_key(i)).auth_delay = determine_hash_delay((*get_key(i)).id);
        i = i.wrapping_add(1)
    };
}
/* ================================================== */
unsafe extern "C" fn lookup_key(mut id: uint32_t) -> libc::c_int {
    let mut specimen: Key =
        Key{id: 0,
            class: NTP_MAC,
            data:
                C2RustUnnamed{ntp_mac:
                                  C2RustUnnamed_0{value:
                                                      0 as *mut libc::c_uchar,
                                                  length: 0,
                                                  hash_id: 0,},},
            auth_delay: 0,};
    let mut where_0: *mut Key = 0 as *mut Key;
    let mut keys_ptr: *mut Key = 0 as *mut Key;
    let mut pos: libc::c_int = 0;
    keys_ptr = ARR_GetElements(keys) as *mut Key;
    specimen.id = id;
    where_0 =
        bsearch(&mut specimen as *mut Key as *mut libc::c_void,
                keys_ptr as *const libc::c_void, ARR_GetSize(keys) as size_t,
                ::std::mem::size_of::<Key>() as libc::c_ulong,
                Some(compare_keys_by_id as
                         unsafe extern "C" fn(_: *const libc::c_void,
                                              _: *const libc::c_void)
                             -> libc::c_int)) as *mut Key;
    if where_0.is_null() {
        return -(1 as libc::c_int)
    } else {
        pos =
            where_0.wrapping_offset_from(keys_ptr) as libc::c_long as
                libc::c_int;
        return pos
    };
}
/* ================================================== */
unsafe extern "C" fn get_key_by_id(mut key_id: uint32_t) -> *mut Key {
    let mut position: libc::c_int = 0;
    if cache_valid != 0 && key_id == cache_key_id {
        return get_key(cache_key_pos as libc::c_uint)
    }
    position = lookup_key(key_id);
    if position >= 0 as libc::c_int {
        cache_valid = 1 as libc::c_int;
        cache_key_pos = position;
        cache_key_id = key_id;
        return get_key(position as libc::c_uint)
    }
    return 0 as *mut Key;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn KEY_KeyKnown(mut key_id: uint32_t) -> libc::c_int {
    return (get_key_by_id(key_id) != 0 as *mut libc::c_void as *mut Key) as
               libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn KEY_GetAuthDelay(mut key_id: uint32_t)
 -> libc::c_int {
    let mut key: *mut Key = 0 as *mut Key;
    key = get_key_by_id(key_id);
    if key.is_null() { return 0 as libc::c_int }
    return (*key).auth_delay;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn KEY_GetAuthLength(mut key_id: uint32_t)
 -> libc::c_int {
    let mut buf: [libc::c_uchar; 64] = [0; 64];
    let mut key: *mut Key = 0 as *mut Key;
    key = get_key_by_id(key_id);
    if key.is_null() { return 0 as libc::c_int }
    match (*key).class as libc::c_uint {
        0 => {
            return HSH_Hash((*key).data.ntp_mac.hash_id, buf.as_mut_ptr(),
                            0 as libc::c_int as libc::c_uint,
                            buf.as_mut_ptr(),
                            0 as libc::c_int as libc::c_uint,
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_uchar; 64]>() as
                                libc::c_ulong as libc::c_uint) as libc::c_int
        }
        1 => {
            return CMC_Hash((*key).data.cmac, buf.as_mut_ptr(),
                            0 as libc::c_int as libc::c_uint,
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_uchar; 64]>() as
                                libc::c_ulong as libc::c_uint) as libc::c_int
        }
        _ => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"keys.c\x00" as *const u8 as *const libc::c_char,
                          384 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 32],
                                                    &[libc::c_char; 32]>(b"int KEY_GetAuthLength(uint32_t)\x00")).as_ptr());
            return 0 as libc::c_int
        }
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn KEY_CheckKeyLength(mut key_id: uint32_t)
 -> libc::c_int {
    let mut key: *mut Key = 0 as *mut Key;
    key = get_key_by_id(key_id);
    if key.is_null() { return 0 as libc::c_int }
    match (*key).class as libc::c_uint {
        0 => {
            return ((*key).data.ntp_mac.length >= 10 as libc::c_int) as
                       libc::c_int
        }
        _ => { return 1 as libc::c_int }
    };
}
/* ================================================== */
unsafe extern "C" fn generate_auth(mut key: *mut Key,
                                   mut data: *const libc::c_uchar,
                                   mut data_len: libc::c_int,
                                   mut auth: *mut libc::c_uchar,
                                   mut auth_len: libc::c_int) -> libc::c_int {
    match (*key).class as libc::c_uint {
        0 => {
            return HSH_Hash((*key).data.ntp_mac.hash_id,
                            (*key).data.ntp_mac.value,
                            (*key).data.ntp_mac.length as libc::c_uint, data,
                            data_len as libc::c_uint, auth,
                            auth_len as libc::c_uint) as libc::c_int
        }
        1 => {
            return CMC_Hash((*key).data.cmac, data, data_len as libc::c_uint,
                            auth, auth_len as libc::c_uint) as libc::c_int
        }
        _ => { return 0 as libc::c_int }
    };
}
/* ================================================== */
unsafe extern "C" fn check_auth(mut key: *mut Key,
                                mut data: *const libc::c_uchar,
                                mut data_len: libc::c_int,
                                mut auth: *const libc::c_uchar,
                                mut auth_len: libc::c_int,
                                mut trunc_len: libc::c_int) -> libc::c_int {
    let mut buf: [libc::c_uchar; 64] = [0; 64];
    let mut hash_len: libc::c_int = 0;
    hash_len =
        generate_auth(key, data, data_len, buf.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_uchar; 64]>() as
                          libc::c_ulong as libc::c_int);
    return ((if hash_len < trunc_len { hash_len } else { trunc_len }) ==
                auth_len &&
                memcmp(buf.as_mut_ptr() as *const libc::c_void,
                       auth as *const libc::c_void, auth_len as libc::c_ulong)
                    == 0) as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn KEY_GenerateAuth(mut key_id: uint32_t,
                                          mut data: *const libc::c_uchar,
                                          mut data_len: libc::c_int,
                                          mut auth: *mut libc::c_uchar,
                                          mut auth_len: libc::c_int)
 -> libc::c_int {
    let mut key: *mut Key = 0 as *mut Key;
    key = get_key_by_id(key_id);
    if key.is_null() { return 0 as libc::c_int }
    return generate_auth(key, data, data_len, auth, auth_len);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn KEY_CheckAuth(mut key_id: uint32_t,
                                       mut data: *const libc::c_uchar,
                                       mut data_len: libc::c_int,
                                       mut auth: *const libc::c_uchar,
                                       mut auth_len: libc::c_int,
                                       mut trunc_len: libc::c_int)
 -> libc::c_int {
    let mut key: *mut Key = 0 as *mut Key;
    key = get_key_by_id(key_id);
    if key.is_null() { return 0 as libc::c_int }
    return check_auth(key, data, data_len, auth, auth_len, trunc_len);
}
