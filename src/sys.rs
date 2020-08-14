#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]

use super::sys_linux;

extern "C" {
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    /*
     chronyd/chronyc - Programs for keeping computer clocks accurate.

    **********************************************************************
    * Copyright (C) Miroslav Lichvar  2017
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

     Header file for null clock driver
     */
    #[no_mangle]
    fn SYS_Null_Initialise();
    #[no_mangle]
    fn SYS_Null_Finalise();
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char, _: ...);
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

     The header file for the linux driver
     */
    #[no_mangle]
    fn SYS_Linux_Initialise();
    #[no_mangle]
    fn SYS_Linux_Finalise();
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
    #[no_mangle]
    fn SYS_Posix_MemLockAll();
    #[no_mangle]
    fn SYS_Posix_SetScheduler(priority: libc::c_int);
}
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
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

 This file contains all the conditionally compiled bits that pull
 in the various operating-system specific modules
 */
/* ================================================== */
static mut null_driver: libc::c_int = 0;
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

  This is the header for the file that links in the operating system-
  specific parts of the software

*/
/* Called at the start of the run to do initialisation */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Initialise(mut clock_control: libc::c_int) {
    null_driver = (clock_control == 0) as libc::c_int;
    if null_driver != 0 {
        SYS_Null_Initialise();
        return;
    }
    SYS_Linux_Initialise();
}
/* Called at the end of the run to do final clean-up */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_Finalise() {
    if null_driver != 0 {
        SYS_Null_Finalise();
        return;
    }
    SYS_Linux_Finalise();
}
/* Drop root privileges to the specified user and group */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_DropRoot(mut uid: uid_t, mut gid: gid_t) {
    sys_linux::drop_root(uid as u32, gid as u32, null_driver == 0);
}
/* Enable a system call filter to allow only system calls
which chronyd normally needs after initialization */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_EnableSystemCallFilter(mut level: libc::c_int) {
    LOG_Message(
        LOGS_FATAL,
        b"system call filter not supported\x00" as *const u8 as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_SetScheduler(mut SchedPriority: libc::c_int) {
    SYS_Posix_SetScheduler(SchedPriority);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SYS_LockMemory() {
    SYS_Posix_MemLockAll();
}
/* ================================================== */
