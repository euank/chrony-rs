#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn pthread_self() -> pthread_t;
    #[no_mangle]
    fn pthread_setschedparam(__target_thread: pthread_t,
                             __policy: libc::c_int,
                             __param: *const sched_param) -> libc::c_int;
    #[no_mangle]
    fn sched_get_priority_max(__algorithm: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sched_get_priority_min(__algorithm: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn mlockall(__flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit)
     -> libc::c_int;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
}
pub type __rlim_t = libc::c_ulong;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: libc::c_int,
}
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
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
 * Copyright (C) John G. Hasler  2009
 * Copyright (C) Miroslav Lichvar  2009-2012, 2014-2018
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

  This module is for POSIX compliant operating systems.

  */
/* ================================================== */
/* Install SCHED_FIFO real-time scheduler with specified priority */
#[no_mangle]
pub unsafe extern "C" fn SYS_Posix_SetScheduler(mut priority: libc::c_int) {
    let mut sched: sched_param = sched_param{sched_priority: 0,};
    let mut pmax: libc::c_int = 0;
    let mut pmin: libc::c_int = 0;
    if priority < 1 as libc::c_int || priority > 99 as libc::c_int {
        LOG_Message(LOGS_FATAL,
                    b"Bad scheduler priority: %d\x00" as *const u8 as
                        *const libc::c_char, priority);
        exit(1 as libc::c_int);
    }
    sched.sched_priority = priority;
    pmax = sched_get_priority_max(1 as libc::c_int);
    pmin = sched_get_priority_min(1 as libc::c_int);
    if priority > pmax {
        sched.sched_priority = pmax
    } else if priority < pmin { sched.sched_priority = pmin }
    if pthread_setschedparam(pthread_self(), 1 as libc::c_int, &mut sched) <
           0 as libc::c_int {
        LOG_Message(LOGS_ERR,
                    b"pthread_setschedparam() failed\x00" as *const u8 as
                        *const libc::c_char);
    } else if 0 as libc::c_int != 0 &&
                  log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
     {
        LOG_Message(LOGS_DEBUG,
                    b"Enabled SCHED_FIFO with priority %d\x00" as *const u8 as
                        *const libc::c_char, sched.sched_priority);
    };
}
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) John G. Hasler  2009
 * Copyright (C) Miroslav Lichvar  2009-2012, 2014-2018
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

  The header file for shared Posix functionality
  */
/* HAVE_PTHREAD_SETSCHEDPARAM  */
/* ================================================== */
/* Lock the process into RAM so that it will never be swapped out */
#[no_mangle]
pub unsafe extern "C" fn SYS_Posix_MemLockAll() {
    let mut rlim: rlimit = rlimit{rlim_cur: 0, rlim_max: 0,};
    /* Ensure we can reserve as much as we need */
    rlim.rlim_max = -(1 as libc::c_int) as __rlim_t;
    rlim.rlim_cur = -(1 as libc::c_int) as __rlim_t;
    if setrlimit(__RLIMIT_MEMLOCK, &mut rlim) < 0 as libc::c_int {
        LOG_Message(LOGS_ERR,
                    b"setrlimit() failed\x00" as *const u8 as
                        *const libc::c_char);
        return
    }
    if mlockall(1 as libc::c_int | 2 as libc::c_int) < 0 as libc::c_int {
        LOG_Message(LOGS_ERR,
                    b"mlockall() failed\x00" as *const u8 as
                        *const libc::c_char);
    } else if 0 as libc::c_int != 0 &&
                  log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
     {
        LOG_Message(LOGS_DEBUG,
                    b"Successfully locked into RAM\x00" as *const u8 as
                        *const libc::c_char);
    };
}
/* HAVE_MLOCKALL */
