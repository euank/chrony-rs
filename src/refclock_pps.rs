#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type RCL_Instance_Record;
}
pub type RCL_Instance = *mut RCL_Instance_Record;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefclockDriver {
    pub init: Option<unsafe extern "C" fn(_: RCL_Instance) -> libc::c_int>,
    pub fini: Option<unsafe extern "C" fn(_: RCL_Instance) -> ()>,
    pub poll: Option<unsafe extern "C" fn(_: RCL_Instance) -> libc::c_int>,
}
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Miroslav Lichvar  2009
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

 PPSAPI refclock driver.

 */
#[no_mangle]
pub static mut RCL_PPS_driver: RefclockDriver = {
    let mut init = RefclockDriver {
        init: None,
        fini: None,
        poll: None,
    };
    init
};
