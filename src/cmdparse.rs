#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
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
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2013-2014, 2016
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
  
  Module for parsing various forms of directive and command lines that
  are common to the configuration file and to the command client.

  */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CPS_ParseNTPSourceAdd(mut line: *mut libc::c_char,
                                               mut src: *mut CPS_NTP_Source)
 -> libc::c_int {
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    (*src).port = 123 as libc::c_int as libc::c_ushort;
    (*src).params.minpoll = 6 as libc::c_int;
    (*src).params.maxpoll = 10 as libc::c_int;
    (*src).params.connectivity = SRC_ONLINE;
    (*src).params.auto_offline = 0 as libc::c_int;
    (*src).params.presend_minpoll = 100 as libc::c_int;
    (*src).params.burst = 0 as libc::c_int;
    (*src).params.iburst = 0 as libc::c_int;
    (*src).params.min_stratum = 0 as libc::c_int;
    (*src).params.poll_target = 8 as libc::c_int;
    (*src).params.version = 0 as libc::c_int;
    (*src).params.max_sources = 4 as libc::c_int;
    (*src).params.min_samples = -(1 as libc::c_int);
    (*src).params.max_samples = -(1 as libc::c_int);
    (*src).params.filter_length = 0 as libc::c_int;
    (*src).params.interleaved = 0 as libc::c_int;
    (*src).params.sel_options = 0 as libc::c_int;
    (*src).params.authkey = 0 as libc::c_int as uint32_t;
    (*src).params.max_delay = 3.0f64;
    (*src).params.max_delay_ratio = 0.0f64;
    (*src).params.max_delay_dev_ratio = 10.0f64;
    (*src).params.min_delay = 0.0f64;
    (*src).params.asymmetry = 1.0f64;
    (*src).params.offset = 0.0f64;
    hostname = line;
    line = CPS_SplitWord(line);
    if *hostname == 0 { return 0 as libc::c_int }
    (*src).name = hostname;
    /* Parse options */
    while *line != 0 {
        cmd = line;
        line = CPS_SplitWord(line);
        n = 0 as libc::c_int;
        if strcasecmp(cmd,
                      b"auto_offline\x00" as *const u8 as *const libc::c_char)
               == 0 {
            (*src).params.auto_offline = 1 as libc::c_int
        } else if strcasecmp(cmd,
                             b"burst\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            (*src).params.burst = 1 as libc::c_int
        } else if strcasecmp(cmd,
                             b"iburst\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            (*src).params.iburst = 1 as libc::c_int
        } else if strcasecmp(cmd,
                             b"offline\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            (*src).params.connectivity = SRC_OFFLINE
        } else if strcasecmp(cmd,
                             b"noselect\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            (*src).params.sel_options |= 0x1 as libc::c_int
        } else if strcasecmp(cmd,
                             b"prefer\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            (*src).params.sel_options |= 0x2 as libc::c_int
        } else if strcasecmp(cmd,
                             b"require\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            (*src).params.sel_options |= 0x8 as libc::c_int
        } else if strcasecmp(cmd,
                             b"trust\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            (*src).params.sel_options |= 0x4 as libc::c_int
        } else if strcasecmp(cmd,
                             b"key\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%u%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.authkey as *mut uint32_t,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int ||
                   (*src).params.authkey == 0 as libc::c_int as libc::c_uint {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"asymmetry\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.asymmetry as *mut libc::c_double,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"filter\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.filter_length as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"maxdelay\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.max_delay as *mut libc::c_double,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"maxdelayratio\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.max_delay_ratio as
                          *mut libc::c_double, &mut n as *mut libc::c_int) !=
                   1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"maxdelaydevratio\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.max_delay_dev_ratio as
                          *mut libc::c_double, &mut n as *mut libc::c_int) !=
                   1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"maxpoll\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.maxpoll as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"maxsamples\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.max_samples as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"maxsources\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.max_sources as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"mindelay\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.min_delay as *mut libc::c_double,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"minpoll\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.minpoll as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"minsamples\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.min_samples as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"minstratum\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.min_stratum as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"offset\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.offset as *mut libc::c_double,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"port\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%hu%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).port as *mut libc::c_ushort,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"polltarget\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.poll_target as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"presend\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.presend_minpoll as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"version\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*src).params.version as *mut libc::c_int,
                      &mut n as *mut libc::c_int) != 1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"xleave\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            (*src).params.interleaved = 1 as libc::c_int
        } else { return 0 as libc::c_int }
        line = line.offset(n as isize)
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CPS_ParseLocal(mut line: *mut libc::c_char,
                                        mut stratum: *mut libc::c_int,
                                        mut orphan: *mut libc::c_int,
                                        mut distance: *mut libc::c_double)
 -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    *stratum = 10 as libc::c_int;
    *distance = 1.0f64;
    *orphan = 0 as libc::c_int;
    while *line != 0 {
        cmd = line;
        line = CPS_SplitWord(line);
        if strcasecmp(cmd, b"stratum\x00" as *const u8 as *const libc::c_char)
               == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      stratum, &mut n as *mut libc::c_int) != 1 as libc::c_int
                   || *stratum >= 16 as libc::c_int ||
                   *stratum <= 0 as libc::c_int {
                return 0 as libc::c_int
            }
        } else if strcasecmp(cmd,
                             b"orphan\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            *orphan = 1 as libc::c_int;
            n = 0 as libc::c_int
        } else if strcasecmp(cmd,
                             b"distance\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      distance, &mut n as *mut libc::c_int) !=
                   1 as libc::c_int {
                return 0 as libc::c_int
            }
        } else { return 0 as libc::c_int }
        line = line.offset(n as isize)
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CPS_NormalizeLine(mut line: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut space: libc::c_int = 1 as libc::c_int;
    let mut first: libc::c_int = 1 as libc::c_int;
    /* Remove white-space at beginning and replace white-spaces with space char */
    q = line;
    p = q;
    while *p != 0 {
        if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as
                                          isize) as libc::c_int &
               _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            if space == 0 {
                let fresh0 = q;
                q = q.offset(1);
                *fresh0 = ' ' as i32 as libc::c_char
            }
            space = 1 as libc::c_int
        } else {
            /* Discard comment lines */
            if first != 0 &&
                   !strchr(b"!;#%\x00" as *const u8 as *const libc::c_char,
                           *p as libc::c_int).is_null() {
                break ;
            }
            let fresh1 = q;
            q = q.offset(1);
            *fresh1 = *p;
            first = 0 as libc::c_int;
            space = first
        }
        p = p.offset(1)
    }
    /* Strip trailing space */
    if q > line &&
           *q.offset(-(1 as libc::c_int) as isize) as libc::c_int ==
               ' ' as i32 {
        q = q.offset(-1)
    }
    *q = '\u{0}' as i32 as libc::c_char;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CPS_SplitWord(mut line: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut p: *mut libc::c_char = line;
    let mut q: *mut libc::c_char = line;
    /* Skip white-space before the word */
    while *q as libc::c_int != 0 &&
              *(*__ctype_b_loc()).offset(*q as libc::c_uchar as libc::c_int as
                                             isize) as libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
        q = q.offset(1)
    }
    /* Move the word to the beginning */
    while *q as libc::c_int != 0 &&
              *(*__ctype_b_loc()).offset(*q as libc::c_uchar as libc::c_int as
                                             isize) as libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int ==
                  0 {
        let fresh2 = q;
        q = q.offset(1);
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = *fresh2
    }
    /* Find the next word */
    while *q as libc::c_int != 0 &&
              *(*__ctype_b_loc()).offset(*q as libc::c_uchar as libc::c_int as
                                             isize) as libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
        q = q.offset(1)
    }
    *p = '\u{0}' as i32 as libc::c_char;
    /* Return pointer to the next word or NUL */
    return q;
}
/* Parse a command to enable local reference */
/* Remove extra white-space and comments */
/* Terminate first word and return pointer to the next word */
/* Parse a key from keyfile */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CPS_ParseKey(mut line: *mut libc::c_char,
                                      mut id: *mut uint32_t,
                                      mut type_0: *mut *const libc::c_char,
                                      mut key: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s4: *mut libc::c_char = 0 as *mut libc::c_char;
    s1 = line;
    s2 = CPS_SplitWord(s1);
    s3 = CPS_SplitWord(s2);
    s4 = CPS_SplitWord(s3);
    /* Require two or three words */
    if *s2 == 0 || *s4 as libc::c_int != 0 { return 0 as libc::c_int }
    if sscanf(s1, b"%u\x00" as *const u8 as *const libc::c_char, id) !=
           1 as libc::c_int {
        return 0 as libc::c_int
    }
    if *s3 != 0 {
        *type_0 = s2;
        *key = s3
    } else {
        *type_0 = b"MD5\x00" as *const u8 as *const libc::c_char;
        *key = s2
    }
    return 1 as libc::c_int;
}
