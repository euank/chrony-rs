#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn ntohs(__netshort: uint16_t) -> uint16_t;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
    pub data: C2RustUnnamed_0,
    pub padding: [uint8_t; 396],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
    pub data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Richard P. Curnow  1997-2002
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

 Routines to compute the expected length of a command or reply packet.
 These operate on the RAW NETWORK packets, from the point of view of
 integer endianness within the structures.

 */
/* ================================================== */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct request_length {
    pub command: uint16_t,
    pub padding: uint16_t,
}
// Initialized in run_static_initializers
static mut request_lengths: [request_length; 66] = [request_length {
    command: 0,
    padding: 0,
}; 66];
static mut reply_lengths: [uint16_t; 20] = [
    0 as libc::c_int as uint16_t,
    28 as libc::c_ulong as uint16_t,
    32 as libc::c_ulong as uint16_t,
    76 as libc::c_ulong as uint16_t,
    0 as libc::c_int as uint16_t,
    104 as libc::c_ulong as uint16_t,
    84 as libc::c_ulong as uint16_t,
    56 as libc::c_ulong as uint16_t,
    0 as libc::c_int as uint16_t,
    0 as libc::c_int as uint16_t,
    0 as libc::c_int as uint16_t,
    0 as libc::c_int as uint16_t,
    48 as libc::c_ulong as uint16_t,
    52 as libc::c_ulong as uint16_t,
    48 as libc::c_ulong as uint16_t,
    424 as libc::c_ulong as uint16_t,
    152 as libc::c_ulong as uint16_t,
    40 as libc::c_ulong as uint16_t,
    416 as libc::c_ulong as uint16_t,
    284 as libc::c_ulong as uint16_t,
];
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
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn PKL_CommandLength(mut r: *mut CMD_Request) -> libc::c_int {
    let mut type_0: uint32_t = 0;
    let mut command_length: libc::c_int = 0;
    if (::std::mem::size_of::<[request_length; 66]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<request_length>() as libc::c_ulong)
        == 66 as libc::c_int as libc::c_ulong
    {
    } else {
        __assert_fail(
            b"sizeof (request_lengths) / sizeof (request_lengths[0]) == N_REQUEST_TYPES\x00"
                as *const u8 as *const libc::c_char,
            b"pktlength.c\x00" as *const u8 as *const libc::c_char,
            159 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"int PKL_CommandLength(CMD_Request *)\x00",
            ))
            .as_ptr(),
        );
    }
    type_0 = ntohs((*r).command) as uint32_t;
    if type_0 >= 66 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    command_length = request_lengths[type_0 as usize].command as libc::c_int;
    if command_length == 0 {
        return 0 as libc::c_int;
    }
    return command_length + PKL_CommandPaddingLength(r);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn PKL_CommandPaddingLength(mut r: *mut CMD_Request) -> libc::c_int {
    let mut type_0: uint32_t = 0;
    if ((*r).version as libc::c_int) < 6 as libc::c_int {
        return 0 as libc::c_int;
    }
    type_0 = ntohs((*r).command) as uint32_t;
    if type_0 >= 66 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    return request_lengths[ntohs((*r).command) as usize].padding as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn PKL_ReplyLength(mut r: *mut CMD_Reply) -> libc::c_int {
    let mut type_0: uint32_t = 0;
    if (::std::mem::size_of::<[uint16_t; 20]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
        == 20 as libc::c_int as libc::c_ulong
    {
    } else {
        __assert_fail(
            b"sizeof (reply_lengths) / sizeof (reply_lengths[0]) == N_REPLY_TYPES\x00" as *const u8
                as *const libc::c_char,
            b"pktlength.c\x00" as *const u8 as *const libc::c_char,
            197 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"int PKL_ReplyLength(CMD_Reply *)\x00",
            ))
            .as_ptr(),
        );
    }
    type_0 = ntohs((*r).reply) as uint32_t;
    /* Note that reply type codes start from 1, not 0 */
    if type_0 < 1 as libc::c_int as libc::c_uint || type_0 >= 20 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    return reply_lengths[type_0 as usize] as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    request_lengths = [
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 60 as libc::c_ulong as uint16_t,
                padding: if (60 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(60 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 60 as libc::c_ulong as uint16_t,
                padding: if (60 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(60 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 68 as libc::c_ulong as uint16_t,
                padding: if (68 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(68 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 24 as libc::c_ulong as uint16_t,
                padding: if (24 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(24 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 24 as libc::c_ulong as uint16_t,
                padding: if (24 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(24 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 32 as libc::c_ulong as uint16_t,
                padding: if (32 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(32 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 32 as libc::c_ulong as uint16_t,
                padding: if (32 as libc::c_ulong) < 40 as libc::c_ulong {
                    (40 as libc::c_ulong).wrapping_sub(32 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 0 as libc::c_int as uint16_t,
                padding: 0 as libc::c_int as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 24 as libc::c_ulong as uint16_t,
                padding: if (24 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(24 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 32 as libc::c_ulong {
                    (32 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 24 as libc::c_ulong as uint16_t,
                padding: if (24 as libc::c_ulong) < 76 as libc::c_ulong {
                    (76 as libc::c_ulong).wrapping_sub(24 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 40 as libc::c_ulong as uint16_t,
                padding: if (40 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(40 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 40 as libc::c_ulong as uint16_t,
                padding: if (40 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(40 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 0 as libc::c_int as uint16_t,
                padding: 0 as libc::c_int as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 0 as libc::c_int as uint16_t,
                padding: 0 as libc::c_int as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 40 as libc::c_ulong as uint16_t,
                padding: if (40 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(40 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 24 as libc::c_ulong as uint16_t,
                padding: if (24 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(24 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 28 as libc::c_ulong as uint16_t,
                padding: if (28 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(28 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 104 as libc::c_ulong {
                    (104 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 24 as libc::c_ulong as uint16_t,
                padding: if (24 as libc::c_ulong) < 84 as libc::c_ulong {
                    (84 as libc::c_ulong).wrapping_sub(24 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 56 as libc::c_ulong {
                    (56 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 0 as libc::c_int as uint16_t,
                padding: 0 as libc::c_int as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 0 as libc::c_int as uint16_t,
                padding: 0 as libc::c_int as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 0 as libc::c_int as uint16_t,
                padding: 0 as libc::c_int as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 416 as libc::c_ulong {
                    (416 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 24 as libc::c_ulong as uint16_t,
                padding: if (24 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(24 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 48 as libc::c_ulong {
                    (48 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 44 as libc::c_ulong as uint16_t,
                padding: if (44 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(44 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 24 as libc::c_ulong as uint16_t,
                padding: if (24 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(24 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 28 as libc::c_ulong as uint16_t,
                padding: if (28 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(28 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 52 as libc::c_ulong {
                    (52 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 24 as libc::c_ulong as uint16_t,
                padding: if (24 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(24 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 48 as libc::c_ulong {
                    (48 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 28 as libc::c_ulong as uint16_t,
                padding: if (28 as libc::c_ulong) < 424 as libc::c_ulong {
                    (424 as libc::c_ulong).wrapping_sub(28 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 36 as libc::c_ulong as uint16_t,
                padding: if (36 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(36 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 40 as libc::c_ulong as uint16_t,
                padding: if (40 as libc::c_ulong) < 152 as libc::c_ulong {
                    (152 as libc::c_ulong).wrapping_sub(40 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 0 as libc::c_int as uint16_t,
                padding: 0 as libc::c_int as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 0 as libc::c_int as uint16_t,
                padding: 0 as libc::c_int as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 0 as libc::c_int as uint16_t,
                padding: 0 as libc::c_int as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 0 as libc::c_int as uint16_t,
                padding: 0 as libc::c_int as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 20 as libc::c_ulong as uint16_t,
                padding: if (20 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(20 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 368 as libc::c_ulong as uint16_t,
                padding: if (368 as libc::c_ulong) < 28 as libc::c_ulong {
                    (28 as libc::c_ulong).wrapping_sub(368 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
        {
            let mut init = request_length {
                command: 40 as libc::c_ulong as uint16_t,
                padding: if (40 as libc::c_ulong) < 284 as libc::c_ulong {
                    (284 as libc::c_ulong).wrapping_sub(40 as libc::c_ulong)
                } else {
                    0 as libc::c_int as libc::c_ulong
                } as uint16_t,
            };
            init
        },
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
/* ================================================== */
