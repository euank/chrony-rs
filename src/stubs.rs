#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type CMC_Instance_Record;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPSockAddr {
    pub ip_addr: IPAddr,
    pub port: uint16_t,
}
pub type NTP_Remote_Address = IPSockAddr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_Local_Address {
    pub ip_addr: IPAddr,
    pub if_index: libc::c_int,
    pub sock_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_int64 {
    pub hi: uint32_t,
    pub lo: uint32_t,
}
pub type NTP_int32 = uint32_t;
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
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Miroslav Lichvar  2016
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

 Header for MS-SNTP authentication via Samba (ntp_signd) */
/* Initialisation function */
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Miroslav Lichvar  2014-2016
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

 Function replacements needed when optional features are disabled.

 */
/* !FEAT_ASYNCDNS */
/* !FEAT_CMDMON */
/* !FEAT_NTP */
/* !FEAT_REFCLOCK */
#[no_mangle]
pub unsafe extern "C" fn NSD_Initialise() {}
/* Finalisation function */
#[no_mangle]
pub unsafe extern "C" fn NSD_Finalise() {}
/* Function to get an estimate of delay due to signing */
#[no_mangle]
pub unsafe extern "C" fn NSD_GetAuthDelay(mut key_id: uint32_t) -> libc::c_int {
    return 0 as libc::c_int;
}
/* Function to sign an NTP packet and send it */
#[no_mangle]
pub unsafe extern "C" fn NSD_SignAndSendPacket(
    mut key_id: uint32_t,
    mut packet: *mut NTP_Packet,
    mut remote_addr: *mut NTP_Remote_Address,
    mut local_addr: *mut NTP_Local_Address,
    mut length: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
/* !FEAT_SIGND */
#[no_mangle]
pub unsafe extern "C" fn CMC_GetKeyLength(mut cipher: *const libc::c_char) -> libc::c_uint {
    return 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn CMC_CreateInstance(
    mut cipher: *const libc::c_char,
    mut key: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> CMC_Instance {
    return 0 as CMC_Instance;
}
#[no_mangle]
pub unsafe extern "C" fn CMC_Hash(
    mut inst: CMC_Instance,
    mut in_0: *const libc::c_uchar,
    mut in_len: libc::c_uint,
    mut out: *mut libc::c_uchar,
    mut out_len: libc::c_uint,
) -> libc::c_uint {
    return 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn CMC_DestroyInstance(mut inst: CMC_Instance) {}
/* !HAVE_CMAC */
