#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           register_tool)]
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type ARR_Instance_Record;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __cmsg_nxthdr(__mhdr: *mut msghdr, __cmsg: *mut cmsghdr) -> *mut cmsghdr;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn socketpair(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
        __fds: *mut libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> libc::c_int;
    #[no_mangle]
    fn getsockname(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __len: *mut socklen_t)
        -> libc::c_int;
    #[no_mangle]
    fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> libc::c_int;
    #[no_mangle]
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    #[no_mangle]
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    #[no_mangle]
    fn sendmsg(__fd: libc::c_int, __message: *const msghdr, __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn recvmmsg(
        __fd: libc::c_int,
        __vmessages: *mut mmsghdr,
        __vlen: libc::c_uint,
        __flags: libc::c_int,
        __tmo: *mut timespec,
    ) -> libc::c_int;
    #[no_mangle]
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __addr_len: *mut socklen_t)
        -> libc::c_int;
    #[no_mangle]
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static in6addr_any: in6_addr;
    #[no_mangle]
    static in6addr_loopback: in6_addr;
    #[no_mangle]
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    #[no_mangle]
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    #[no_mangle]
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    #[no_mangle]
    fn htons(__hostshort: uint16_t) -> uint16_t;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    /* Create a new array with given element size */
    #[no_mangle]
    fn ARR_CreateInstance(elem_size: libc::c_uint) -> ARR_Instance;
    /* Destroy the array */
    #[no_mangle]
    fn ARR_DestroyInstance(array: ARR_Instance);
    /* Return element with given index */
    #[no_mangle]
    fn ARR_GetElement(array: ARR_Instance, index: libc::c_uint) -> *mut libc::c_void;
    /* Return pointer to the internal array of elements */
    #[no_mangle]
    fn ARR_GetElements(array: ARR_Instance) -> *mut libc::c_void;
    /* Set the size of the array */
    #[no_mangle]
    fn ARR_SetSize(array: ARR_Instance, size: libc::c_uint);
    /* Return current size of the array */
    #[no_mangle]
    fn ARR_GetSize(array: ARR_Instance) -> libc::c_uint;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn UTI_IsZeroTimespec(ts: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn UTI_TimevalToTimespec(tv: *mut timeval, ts: *mut timespec);
    #[no_mangle]
    fn UTI_ZeroTimespec(ts: *mut timespec);
    #[no_mangle]
    fn UTI_IPSockAddrToString(sa: *mut IPSockAddr) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_CompareIPs(a: *mut IPAddr, b: *mut IPAddr, mask: *mut IPAddr) -> libc::c_int;
    #[no_mangle]
    fn UTI_IPToString(ip: *mut IPAddr) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_RemoveFile(
        basedir: *const libc::c_char,
        name: *const libc::c_char,
        suffix: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn UTI_FdSetCloexec(fd: libc::c_int) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_TRYHARD: C2RustUnnamed = 4;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: libc::c_int,
    pub cmsg_type: libc::c_int,
    pub __cmsg_data: [libc::c_uchar; 0],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SCM_CREDENTIALS: C2RustUnnamed_0 = 2;
pub const SCM_RIGHTS: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed_1 = 2;
pub const SHUT_WR: C2RustUnnamed_1 = 1;
pub const SHUT_RD: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mmsghdr {
    pub msg_hdr: msghdr,
    pub msg_len: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_pktinfo {
    pub ipi_ifindex: libc::c_int,
    pub ipi_spec_dst: in_addr,
    pub ipi_addr: in_addr,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_3 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_3 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_3 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_3 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_3 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_3 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_3 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_3 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_3 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_3 = 92;
pub const IPPROTO_AH: C2RustUnnamed_3 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_3 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_3 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_3 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_3 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_3 = 33;
pub const IPPROTO_TP: C2RustUnnamed_3 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_3 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_3 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_3 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_3 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_3 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_3 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_3 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_3 = 1;
pub const IPPROTO_IP: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_pktinfo {
    pub ipi6_addr: in6_addr,
    pub ipi6_ifindex: libc::c_uint,
}
pub type __u8 = libc::c_uchar;
pub type __u32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock_extended_err {
    pub ee_errno: __u32,
    pub ee_origin: __u8,
    pub ee_type: __u8,
    pub ee_code: __u8,
    pub ee_pad: __u8,
    pub ee_info: __u32,
    pub ee_data: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scm_timestamping {
    pub ts: [timespec; 3],
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const SCM_TSTAMP_ACK: C2RustUnnamed_4 = 2;
pub const SCM_TSTAMP_SCHED: C2RustUnnamed_4 = 1;
pub const SCM_TSTAMP_SND: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scm_ts_pktinfo {
    pub if_index: __u32,
    pub pkt_length: __u32,
    pub reserved: [__u32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPAddr {
    pub addr: C2RustUnnamed_5,
    pub family: uint16_t,
    pub _pad: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub in4: uint32_t,
    pub in6: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPSockAddr {
    pub ip_addr: IPAddr,
    pub port: uint16_t,
}
pub type SCK_AddressType = libc::c_uint;
pub const SCK_ADDR_UNIX: SCK_AddressType = 2;
pub const SCK_ADDR_IP: SCK_AddressType = 1;
pub const SCK_ADDR_UNSPEC: SCK_AddressType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCK_Message {
    pub data: *mut libc::c_void,
    pub length: libc::c_uint,
    pub addr_type: SCK_AddressType,
    pub if_index: libc::c_int,
    pub remote_addr: C2RustUnnamed_8,
    pub local_addr: C2RustUnnamed_7,
    pub timestamp: C2RustUnnamed_6,
    pub descriptor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub kernel: timespec,
    pub hw: timespec,
    pub if_index: libc::c_int,
    pub l2_length: libc::c_int,
    pub tx_flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ip: IPAddr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ip: IPSockAddr,
    pub path: *const libc::c_char,
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
pub struct Message {
    pub name: sockaddr_all,
    pub iov: iovec,
    pub msg_buf: C2RustUnnamed_9,
    pub cmsg_buf: [cmsghdr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub ntp_msg: NTP_Receive_Buffer,
    pub cmd_request: CMD_Request,
    pub cmd_reply: CMD_Reply,
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
    pub data: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_NTPSourceName {
    pub name: [int8_t; 256],
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
pub struct Float {
    pub f: int32_t,
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
pub struct RPY_ManualList {
    pub n_samples: uint32_t,
    pub samples: [RPY_ManualListSample; 16],
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
pub struct RPY_ClientAccessesByIndex {
    pub n_indices: uint32_t,
    pub next_index: uint32_t,
    pub n_clients: uint32_t,
    pub clients: [RPY_ClientAccesses_Client; 8],
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
pub struct RPY_ManualTimestamp {
    pub offset: Float,
    pub dfreq_ppm: Float,
    pub new_afreq_ppm: Float,
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
pub struct RPY_N_Sources {
    pub n_sources: uint32_t,
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPY_Null {
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
    pub data: C2RustUnnamed_11,
    pub padding: [uint8_t; 396],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
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
pub struct REQ_NTPData {
    pub ip_addr: IPAddr,
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
pub struct REQ_ReselectDistance {
    pub distance: Float,
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
pub struct REQ_ClientAccessesByIndex {
    pub first_index: uint32_t,
    pub n_clients: uint32_t,
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
pub struct REQ_Doffset {
    pub sec: int32_t,
    pub usec: int32_t,
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
pub struct REQ_Del_Source {
    pub ip_addr: IPAddr,
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
pub struct REQ_Ac_Check {
    pub ip: IPAddr,
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
pub struct REQ_Source_Data {
    pub index: int32_t,
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
pub struct REQ_Local {
    pub on_off: int32_t,
    pub stratum: int32_t,
    pub distance: Float,
    pub orphan: int32_t,
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
pub struct REQ_Logon {
    pub ts: Timespec,
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
pub struct REQ_Modify_Maxupdateskew {
    pub new_max_update_skew: Float,
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
pub struct REQ_Modify_Minstratum {
    pub address: IPAddr,
    pub new_min_stratum: int32_t,
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
pub struct REQ_Modify_Maxdelayratio {
    pub address: IPAddr,
    pub new_max_delay_ratio: Float,
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
pub struct REQ_Dump {
    pub pad: int32_t,
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
pub struct REQ_Modify_Minpoll {
    pub address: IPAddr,
    pub new_minpoll: int32_t,
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
pub struct REQ_Offline {
    pub mask: IPAddr,
    pub address: IPAddr,
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
pub struct REQ_Null {
    pub EOR: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_Receive_Buffer {
    pub ntp_pkt: NTP_Packet,
    pub extensions: [uint8_t; 1024],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_int64 {
    pub hi: uint32_t,
    pub lo: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sockaddr_all {
    pub in4: sockaddr_in,
    pub in6: sockaddr_in6,
    pub un: sockaddr_un,
    pub sa: sockaddr,
}
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
static mut initialised: libc::c_int = 0;
/* Flags supported by socket() */
static mut supported_socket_flags: libc::c_int = 0;
/* Arrays of Message and MessageHeader */
static mut recv_messages: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
static mut recv_headers: ARR_Instance = 0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
static mut received_messages: libc::c_uint = 0;
static mut priv_bind_function: Option<
    unsafe extern "C" fn(_: libc::c_int, _: *mut sockaddr, _: socklen_t) -> libc::c_int,
> = None;
/* ================================================== */
unsafe extern "C" fn prepare_buffers(mut n: libc::c_uint) {
    let mut hdr: *mut mmsghdr = 0 as *mut mmsghdr;
    let mut msg: *mut Message = 0 as *mut Message;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < n {
        msg = ARR_GetElement(recv_messages, i) as *mut Message;
        hdr = ARR_GetElement(recv_headers, i) as *mut mmsghdr;
        (*msg).iov.iov_base = &mut (*msg).msg_buf as *mut C2RustUnnamed_9 as *mut libc::c_void;
        (*msg).iov.iov_len = ::std::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong;
        (*hdr).msg_hdr.msg_name = &mut (*msg).name as *mut sockaddr_all as *mut libc::c_void;
        (*hdr).msg_hdr.msg_namelen =
            ::std::mem::size_of::<sockaddr_all>() as libc::c_ulong as socklen_t;
        (*hdr).msg_hdr.msg_iov = &mut (*msg).iov;
        (*hdr).msg_hdr.msg_iovlen = 1 as libc::c_int as size_t;
        (*hdr).msg_hdr.msg_control =
            &mut (*msg).cmsg_buf as *mut [cmsghdr; 16] as *mut libc::c_void;
        (*hdr).msg_hdr.msg_controllen = ::std::mem::size_of::<[cmsghdr; 16]>() as libc::c_ulong;
        (*hdr).msg_hdr.msg_flags = 0 as libc::c_int;
        (*hdr).msg_len = 0 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1)
    }
}
/* ================================================== */
unsafe extern "C" fn domain_to_string(mut domain: libc::c_int) -> *const libc::c_char {
    match domain {
        2 => return b"IPv4\x00" as *const u8 as *const libc::c_char,
        10 => return b"IPv6\x00" as *const u8 as *const libc::c_char,
        1 => return b"Unix\x00" as *const u8 as *const libc::c_char,
        0 => return b"UNSPEC\x00" as *const u8 as *const libc::c_char,
        _ => return b"?\x00" as *const u8 as *const libc::c_char,
    };
}
/* ================================================== */
unsafe extern "C" fn check_socket_flag(
    mut sock_flag: libc::c_int,
    mut fd_flag: libc::c_int,
    mut fs_flag: libc::c_int,
) -> libc::c_int {
    let mut sock_fd: libc::c_int = 0;
    let mut fd_flags: libc::c_int = 0;
    let mut fs_flags: libc::c_int = 0;
    sock_fd = socket(
        2 as libc::c_int,
        SOCK_DGRAM as libc::c_int | sock_flag,
        0 as libc::c_int,
    );
    if sock_fd < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    fd_flags = fcntl(sock_fd, 1 as libc::c_int);
    fs_flags = fcntl(sock_fd, 3 as libc::c_int);
    close(sock_fd);
    if fd_flags == -(1 as libc::c_int)
        || fd_flags & fd_flag != fd_flag
        || fs_flags == -(1 as libc::c_int)
        || fs_flags & fs_flag != fs_flag
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn set_socket_nonblock(mut sock_fd: libc::c_int) -> libc::c_int {
    if fcntl(sock_fd, 4 as libc::c_int, 0o4000 as libc::c_int) != 0 {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Could not set O_NONBLOCK : %s\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn get_open_flags(mut flags: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = supported_socket_flags;
    if flags & 1 as libc::c_int != 0 {
        r &= !(SOCK_NONBLOCK as libc::c_int)
    }
    return r;
}
/* ================================================== */
unsafe extern "C" fn set_socket_flags(
    mut sock_fd: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    /* Close the socket automatically on exec */
    if supported_socket_flags & SOCK_CLOEXEC as libc::c_int == 0 as libc::c_int
        && UTI_FdSetCloexec(sock_fd) == 0
    {
        return 0 as libc::c_int;
    }
    /* Enable non-blocking mode */
    if flags & 1 as libc::c_int == 0 as libc::c_int
        && supported_socket_flags & SOCK_NONBLOCK as libc::c_int == 0 as libc::c_int
        && set_socket_nonblock(sock_fd) == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn open_socket(
    mut domain: libc::c_int,
    mut type_0: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut sock_fd: libc::c_int = 0;
    sock_fd = socket(domain, type_0 | get_open_flags(flags), 0 as libc::c_int);
    if sock_fd < 0 as libc::c_int {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Could not open %s socket : %s\x00" as *const u8 as *const libc::c_char,
                domain_to_string(domain),
                strerror(*__errno_location()),
            );
        }
        return -(4 as libc::c_int);
    }
    if set_socket_flags(sock_fd, flags) == 0 {
        close(sock_fd);
        return -(4 as libc::c_int);
    }
    return sock_fd;
}
/* ================================================== */
unsafe extern "C" fn open_socket_pair(
    mut domain: libc::c_int,
    mut type_0: libc::c_int,
    mut flags: libc::c_int,
    mut other_fd: *mut libc::c_int,
) -> libc::c_int {
    let mut sock_fds: [libc::c_int; 2] = [0; 2];
    if socketpair(
        domain,
        type_0 | get_open_flags(flags),
        0 as libc::c_int,
        sock_fds.as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Could not open %s socket : %s\x00" as *const u8 as *const libc::c_char,
                domain_to_string(domain),
                strerror(*__errno_location()),
            );
        }
        return -(4 as libc::c_int);
    }
    if set_socket_flags(sock_fds[0 as libc::c_int as usize], flags) == 0
        || set_socket_flags(sock_fds[1 as libc::c_int as usize], flags) == 0
    {
        close(sock_fds[0 as libc::c_int as usize]);
        close(sock_fds[1 as libc::c_int as usize]);
        return -(4 as libc::c_int);
    }
    *other_fd = sock_fds[1 as libc::c_int as usize];
    return sock_fds[0 as libc::c_int as usize];
}
/* ================================================== */
unsafe extern "C" fn set_socket_options(
    mut sock_fd: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    /* Make the socket capable of sending broadcast packets if requested */
    (flags & 2 as libc::c_int != 0)
        && SCK_SetIntOption(
            sock_fd,
            1 as libc::c_int,
            6 as libc::c_int,
            1 as libc::c_int,
        ) == 0;
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn set_ip_options(
    mut sock_fd: libc::c_int,
    mut family: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    /* Receive only IPv6 packets on an IPv6 socket */
    if family == 2 as libc::c_int
        && SCK_SetIntOption(
            sock_fd,
            IPPROTO_IPV6 as libc::c_int,
            26 as libc::c_int,
            1 as libc::c_int,
        ) == 0
    {
        return 0 as libc::c_int;
    }
    /* Provide destination address of received packets if requested */
    if flags & 4 as libc::c_int != 0 {
        if family == 1 as libc::c_int {
            (SCK_SetIntOption(
                sock_fd,
                IPPROTO_IP as libc::c_int,
                8 as libc::c_int,
                1 as libc::c_int,
            )) == 0;
        } else if family == 2 as libc::c_int {
            (SCK_SetIntOption(
                sock_fd,
                IPPROTO_IPV6 as libc::c_int,
                49 as libc::c_int,
                1 as libc::c_int,
            )) == 0;
        }
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn is_any_address(mut addr: *mut IPAddr) -> libc::c_int {
    let mut any_addr: IPAddr = IPAddr {
        addr: C2RustUnnamed_5 { in4: 0 },
        family: 0,
        _pad: 0,
    };
    SCK_GetAnyLocalIPAddress((*addr).family as libc::c_int, &mut any_addr);
    return (UTI_CompareIPs(&mut any_addr, addr, 0 as *mut IPAddr) == 0 as libc::c_int)
        as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn bind_ip_address(
    mut sock_fd: libc::c_int,
    mut addr: *mut IPSockAddr,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut saddr: sockaddr_all = sockaddr_all {
        in4: sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
    };
    let mut saddr_len: socklen_t = 0;
    let mut s: libc::c_int = 0;
    /* Make the socket capable of re-using an old address if binding to a specific port */
    ((*addr).port as libc::c_int > 0 as libc::c_int)
        && SCK_SetIntOption(
            sock_fd,
            1 as libc::c_int,
            2 as libc::c_int,
            1 as libc::c_int,
        ) == 0;
    /* Allow binding to an address that doesn't exist yet */
    (SCK_SetIntOption(
        sock_fd,
        IPPROTO_IP as libc::c_int,
        15 as libc::c_int,
        1 as libc::c_int,
    )) == 0;
    saddr_len = SCK_IPSockAddrToSockaddr(
        addr,
        &mut saddr as *mut sockaddr_all as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_all>() as libc::c_ulong as libc::c_int,
    ) as socklen_t;
    if saddr_len == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if flags & 16 as libc::c_int != 0 && priv_bind_function.is_some() {
        s = priv_bind_function.expect("non-null function pointer")(
            sock_fd,
            &mut saddr.sa,
            saddr_len,
        )
    } else {
        s = bind(
            sock_fd,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &mut saddr.sa,
            },
            saddr_len,
        )
    }
    if s < 0 as libc::c_int {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Could not bind socket to %s : %s\x00" as *const u8 as *const libc::c_char,
                UTI_IPSockAddrToString(addr),
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn connect_ip_address(
    mut sock_fd: libc::c_int,
    mut addr: *mut IPSockAddr,
) -> libc::c_int {
    let mut saddr: sockaddr_all = sockaddr_all {
        in4: sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
    };
    let mut saddr_len: socklen_t = 0;
    saddr_len = SCK_IPSockAddrToSockaddr(
        addr,
        &mut saddr as *mut sockaddr_all as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_all>() as libc::c_ulong as libc::c_int,
    ) as socklen_t;
    if saddr_len == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if connect(
        sock_fd,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut saddr.sa,
        },
        saddr_len,
    ) < 0 as libc::c_int
        && *__errno_location() != 115 as libc::c_int
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Could not connect socket to %s : %s\x00" as *const u8 as *const libc::c_char,
                UTI_IPSockAddrToString(addr),
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn open_ip_socket(
    mut remote_addr: *mut IPSockAddr,
    mut local_addr: *mut IPSockAddr,
    mut type_0: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut domain: libc::c_int = 0;
    let mut family: libc::c_int = 0;
    let mut sock_fd: libc::c_int = 0;
    if !local_addr.is_null() {
        family = (*local_addr).ip_addr.family as libc::c_int
    } else if !remote_addr.is_null() {
        family = (*remote_addr).ip_addr.family as libc::c_int
    } else {
        family = 1 as libc::c_int
    }
    match family {
        1 => domain = 2 as libc::c_int,
        2 => domain = 10 as libc::c_int,
        _ => {
            if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
            {
                LOG_Message(
                    LOGS_DEBUG,
                    b"Unspecified family\x00" as *const u8 as *const libc::c_char,
                );
            }
            return -(4 as libc::c_int);
        }
    }
    sock_fd = open_socket(domain, type_0, flags);
    if sock_fd < 0 as libc::c_int {
        return -(4 as libc::c_int);
    }
    if !(set_socket_options(sock_fd, flags) == 0) {
        if !(set_ip_options(sock_fd, family, flags) == 0) {
            /* Bind the socket if a non-any local address/port was specified */
            if !(!local_addr.is_null()
                && (*local_addr).ip_addr.family as libc::c_int != 0 as libc::c_int
                && ((*local_addr).port as libc::c_int != 0 as libc::c_int
                    || is_any_address(&mut (*local_addr).ip_addr) == 0)
                && bind_ip_address(sock_fd, local_addr, flags) == 0)
            {
                /* Connect the socket if a remote address was specified */
                if !(!remote_addr.is_null()
                    && (*remote_addr).ip_addr.family as libc::c_int != 0 as libc::c_int
                    && connect_ip_address(sock_fd, remote_addr) == 0)
                {
                    if !remote_addr.is_null() || !local_addr.is_null() {
                        if 0 as libc::c_int != 0
                            && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
                        {
                            LOG_Message(
                                LOGS_DEBUG,
                                b"Opened %s%s socket fd=%d%s%s%s%s\x00" as *const u8
                                    as *const libc::c_char,
                                if type_0 == SOCK_DGRAM as libc::c_int {
                                    b"UDP\x00" as *const u8 as *const libc::c_char
                                } else if type_0 == SOCK_STREAM as libc::c_int {
                                    b"TCP\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"?\x00" as *const u8 as *const libc::c_char
                                },
                                if family == 1 as libc::c_int {
                                    b"v4\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"v6\x00" as *const u8 as *const libc::c_char
                                },
                                sock_fd,
                                if !remote_addr.is_null() {
                                    b" remote=\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"\x00" as *const u8 as *const libc::c_char
                                },
                                if !remote_addr.is_null() {
                                    UTI_IPSockAddrToString(remote_addr)
                                } else {
                                    b"\x00" as *const u8 as *const libc::c_char
                                },
                                if !local_addr.is_null() {
                                    b" local=\x00" as *const u8 as *const libc::c_char
                                } else {
                                    b"\x00" as *const u8 as *const libc::c_char
                                },
                                if !local_addr.is_null() {
                                    UTI_IPSockAddrToString(local_addr)
                                } else {
                                    b"\x00" as *const u8 as *const libc::c_char
                                },
                            );
                        }
                    }
                    return sock_fd;
                }
            }
        }
    }
    SCK_CloseSocket(sock_fd);
    return -(4 as libc::c_int);
}
/* ================================================== */
unsafe extern "C" fn bind_unix_address(
    mut sock_fd: libc::c_int,
    mut addr: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut saddr: sockaddr_all = sockaddr_all {
        in4: sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
    };
    if snprintf(
        saddr.un.sun_path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
        b"%s\x00" as *const u8 as *const libc::c_char,
        addr,
    ) as libc::c_ulong
        >= ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Unix socket path %s too long\x00" as *const u8 as *const libc::c_char,
                addr,
            );
        }
        return 0 as libc::c_int;
    }
    saddr.un.sun_family = 1 as libc::c_int as sa_family_t;
    (UTI_RemoveFile(0 as *const libc::c_char, addr, 0 as *const libc::c_char)) == 0;
    /* PRV_BindSocket() doesn't support Unix sockets yet */
    if bind(
        sock_fd,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut saddr.sa,
        },
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Could not bind Unix socket to %s : %s\x00" as *const u8 as *const libc::c_char,
                addr,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    /* Allow access to everyone with access to the directory if requested */
    if flags & 8 as libc::c_int != 0
        && chmod(addr, 0o666 as libc::c_int as __mode_t) < 0 as libc::c_int
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Could not change permissions of %s : %s\x00" as *const u8 as *const libc::c_char,
                addr,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn connect_unix_address(
    mut sock_fd: libc::c_int,
    mut addr: *const libc::c_char,
) -> libc::c_int {
    let mut saddr: sockaddr_all = sockaddr_all {
        in4: sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
    };
    if snprintf(
        saddr.un.sun_path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
        b"%s\x00" as *const u8 as *const libc::c_char,
        addr,
    ) as libc::c_ulong
        >= ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Unix socket path %s too long\x00" as *const u8 as *const libc::c_char,
                addr,
            );
        }
        return 0 as libc::c_int;
    }
    saddr.un.sun_family = 1 as libc::c_int as sa_family_t;
    if connect(
        sock_fd,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut saddr.sa,
        },
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Could not connect Unix socket to %s : %s\x00" as *const u8 as *const libc::c_char,
                addr,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn open_unix_socket(
    mut remote_addr: *const libc::c_char,
    mut local_addr: *const libc::c_char,
    mut type_0: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut sock_fd: libc::c_int = 0;
    sock_fd = open_socket(1 as libc::c_int, type_0, flags);
    if sock_fd < 0 as libc::c_int {
        return -(4 as libc::c_int);
    }
    if !(set_socket_options(sock_fd, flags) == 0) {
        /* Bind the socket if a local address was specified */
        if !(!local_addr.is_null() && bind_unix_address(sock_fd, local_addr, flags) == 0) {
            /* Connect the socket if a remote address was specified */
            if !(!remote_addr.is_null() && connect_unix_address(sock_fd, remote_addr) == 0) {
                if 0 as libc::c_int != 0
                    && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
                {
                    LOG_Message(
                        LOGS_DEBUG,
                        b"Opened Unix socket fd=%d%s%s%s%s\x00" as *const u8 as *const libc::c_char,
                        sock_fd,
                        if !remote_addr.is_null() {
                            b" remote=\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        },
                        if !remote_addr.is_null() {
                            remote_addr
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        },
                        if !local_addr.is_null() {
                            b" local=\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        },
                        if !local_addr.is_null() {
                            local_addr
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        },
                    );
                }
                return sock_fd;
            }
        }
    }
    SCK_RemoveSocket(sock_fd);
    SCK_CloseSocket(sock_fd);
    return -(4 as libc::c_int);
}
/* ================================================== */
unsafe extern "C" fn open_unix_socket_pair(
    mut type_0: libc::c_int,
    mut flags: libc::c_int,
    mut other_fd: *mut libc::c_int,
) -> libc::c_int {
    let mut sock_fd: libc::c_int = 0;
    sock_fd = open_socket_pair(1 as libc::c_int, type_0, flags, other_fd);
    if sock_fd < 0 as libc::c_int {
        return -(4 as libc::c_int);
    }
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"Opened Unix socket pair fd1=%d fd2=%d\x00" as *const u8 as *const libc::c_char,
            sock_fd,
            *other_fd,
        );
    }
    return sock_fd;
}
/* ================================================== */
unsafe extern "C" fn get_recv_flags(mut flags: libc::c_int) -> libc::c_int {
    let mut recv_flags: libc::c_int = 0 as libc::c_int;
    if flags & 1 as libc::c_int != 0 {
        recv_flags |= MSG_ERRQUEUE as libc::c_int
    }
    return recv_flags;
}
/* ================================================== */
unsafe extern "C" fn handle_recv_error(mut sock_fd: libc::c_int, mut flags: libc::c_int) {
    /* If reading from the error queue failed, the select() exception should
    be for a socket error.  Clear the error to avoid a busy loop. */
    if flags & 1 as libc::c_int != 0 {
        let mut error: libc::c_int = 0 as libc::c_int;
        if SCK_GetIntOption(sock_fd, 1 as libc::c_int, 4 as libc::c_int, &mut error) != 0 {
            *__errno_location() = error
        }
    }
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"Could not receive message fd=%d : %s\x00" as *const u8 as *const libc::c_char,
            sock_fd,
            strerror(*__errno_location()),
        );
    };
}
/* ================================================== */
unsafe extern "C" fn log_message(
    mut sock_fd: libc::c_int,
    mut direction: libc::c_int,
    mut message: *mut SCK_Message,
    mut prefix: *const libc::c_char,
    mut error: *const libc::c_char,
) {
    let mut local_addr: *const libc::c_char = 0 as *const libc::c_char;
    let mut remote_addr: *const libc::c_char = 0 as *const libc::c_char;
    let mut if_index: [libc::c_char; 20] = [0; 20];
    let mut tss: [libc::c_char; 10] = [0; 10];
    let mut tsif: [libc::c_char; 20] = [0; 20];
    let mut tslen: [libc::c_char; 20] = [0; 20];
    if 0 as libc::c_int <= 0 as libc::c_int
        || (log_min_severity as libc::c_int) < LOGS_DEBUG as libc::c_int
    {
        return;
    }
    remote_addr = 0 as *const libc::c_char;
    local_addr = 0 as *const libc::c_char;
    if_index[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    tss[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    tsif[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    tslen[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    match (*message).addr_type as libc::c_uint {
        1 => {
            if (*message).remote_addr.ip.ip_addr.family as libc::c_int != 0 as libc::c_int {
                remote_addr = UTI_IPSockAddrToString(&mut (*message).remote_addr.ip)
            }
            if (*message).local_addr.ip.family as libc::c_int != 0 as libc::c_int {
                local_addr = UTI_IPToString(&mut (*message).local_addr.ip)
            }
        }
        2 => remote_addr = (*message).remote_addr.path,
        _ => {}
    }
    if (*message).if_index != -(1 as libc::c_int) {
        snprintf(
            if_index.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
            b" if=%d\x00" as *const u8 as *const libc::c_char,
            (*message).if_index,
        );
    }
    if direction > 0 as libc::c_int {
        if UTI_IsZeroTimespec(&mut (*message).timestamp.kernel) == 0
            || UTI_IsZeroTimespec(&mut (*message).timestamp.hw) == 0
        {
            snprintf(
                tss.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                b" tss=%s%s\x00" as *const u8 as *const libc::c_char,
                if UTI_IsZeroTimespec(&mut (*message).timestamp.kernel) == 0 {
                    b"K\x00" as *const u8 as *const libc::c_char
                } else {
                    b"\x00" as *const u8 as *const libc::c_char
                },
                if UTI_IsZeroTimespec(&mut (*message).timestamp.hw) == 0 {
                    b"H\x00" as *const u8 as *const libc::c_char
                } else {
                    b"\x00" as *const u8 as *const libc::c_char
                },
            );
        }
        if (*message).timestamp.if_index != -(1 as libc::c_int) {
            snprintf(
                tsif.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
                b" tsif=%d\x00" as *const u8 as *const libc::c_char,
                (*message).timestamp.if_index,
            );
        }
        if (*message).timestamp.l2_length != 0 as libc::c_int {
            snprintf(
                tslen.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
                b" tslen=%d\x00" as *const u8 as *const libc::c_char,
                (*message).timestamp.l2_length,
            );
        }
    }
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"%s message%s%s%s%s fd=%d len=%u%s%s%s%s%s%s\x00" as *const u8 as *const libc::c_char,
            prefix,
            if !remote_addr.is_null() {
                if direction > 0 as libc::c_int {
                    b" from \x00" as *const u8 as *const libc::c_char
                } else {
                    b" to \x00" as *const u8 as *const libc::c_char
                }
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if !remote_addr.is_null() {
                remote_addr
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if !local_addr.is_null() {
                if direction > 0 as libc::c_int {
                    b" to \x00" as *const u8 as *const libc::c_char
                } else {
                    b" from \x00" as *const u8 as *const libc::c_char
                }
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if !local_addr.is_null() {
                local_addr
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            sock_fd,
            (*message).length,
            if_index.as_mut_ptr(),
            tss.as_mut_ptr(),
            tsif.as_mut_ptr(),
            tslen.as_mut_ptr(),
            if !error.is_null() {
                b" : \x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if !error.is_null() {
                error
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
        );
    };
}
/* ================================================== */
unsafe extern "C" fn init_message_addresses(
    mut message: *mut SCK_Message,
    mut addr_type: SCK_AddressType,
) {
    (*message).addr_type = addr_type;
    match addr_type as libc::c_uint {
        0 => {}
        1 => {
            (*message).remote_addr.ip.ip_addr.family = 0 as libc::c_int as uint16_t;
            (*message).remote_addr.ip.port = 0 as libc::c_int as uint16_t;
            (*message).local_addr.ip.family = 0 as libc::c_int as uint16_t
        }
        2 => (*message).remote_addr.path = 0 as *const libc::c_char,
        _ => {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"socket.c\x00" as *const u8 as *const libc::c_char,
                679 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                    b"void init_message_addresses(SCK_Message *, SCK_AddressType)\x00",
                ))
                .as_ptr(),
            );
        }
    };
}
/* ================================================== */
unsafe extern "C" fn init_message_nonaddress(mut message: *mut SCK_Message) {
    (*message).data = 0 as *mut libc::c_void;
    (*message).length = 0 as libc::c_int as libc::c_uint;
    (*message).if_index = -(1 as libc::c_int);
    UTI_ZeroTimespec(&mut (*message).timestamp.kernel);
    UTI_ZeroTimespec(&mut (*message).timestamp.hw);
    (*message).timestamp.if_index = -(1 as libc::c_int);
    (*message).timestamp.l2_length = 0 as libc::c_int;
    (*message).timestamp.tx_flags = 0 as libc::c_int;
    (*message).descriptor = -(4 as libc::c_int);
}
/* ================================================== */
unsafe extern "C" fn process_header(
    mut msg: *mut msghdr,
    mut msg_length: libc::c_uint,
    mut sock_fd: libc::c_int,
    mut flags: libc::c_int,
    mut message: *mut SCK_Message,
) -> libc::c_int {
    let mut cmsg: *mut cmsghdr = 0 as *mut cmsghdr;
    if (*msg).msg_iovlen != 1 as libc::c_int as libc::c_ulong {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Unexpected iovlen\x00" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    if (*msg).msg_namelen as libc::c_ulong > ::std::mem::size_of::<sockaddr_all>() as libc::c_ulong
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Truncated source address\x00" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    if (*msg).msg_namelen as libc::c_ulong > ::std::mem::size_of::<sa_family_t>() as libc::c_ulong {
        match (*((*msg).msg_name as *mut sockaddr)).sa_family as libc::c_int {
            2 | 10 => {
                init_message_addresses(message, SCK_ADDR_IP);
                SCK_SockaddrToIPSockAddr(
                    (*msg).msg_name as *mut sockaddr,
                    (*msg).msg_namelen as libc::c_int,
                    &mut (*message).remote_addr.ip,
                );
            }
            1 => {
                init_message_addresses(message, SCK_ADDR_UNIX);
                (*message).remote_addr.path = (*((*msg).msg_name as *mut sockaddr_un))
                    .sun_path
                    .as_mut_ptr()
            }
            _ => {
                if 0 as libc::c_int != 0
                    && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
                {
                    LOG_Message(
                        LOGS_DEBUG,
                        b"Unexpected address\x00" as *const u8 as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
        }
    } else {
        init_message_addresses(message, SCK_ADDR_UNSPEC);
    }
    init_message_nonaddress(message);
    (*message).data = (*(*msg).msg_iov.offset(0 as libc::c_int as isize)).iov_base;
    (*message).length = msg_length;
    if (*msg).msg_flags & MSG_TRUNC as libc::c_int != 0 {
        log_message(
            sock_fd,
            1 as libc::c_int,
            message,
            b"Truncated\x00" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if (*msg).msg_flags & MSG_CTRUNC as libc::c_int != 0 {
        log_message(
            sock_fd,
            1 as libc::c_int,
            message,
            b"Truncated cmsg in\x00" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    cmsg = if (*msg).msg_controllen >= ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
        (*msg).msg_control as *mut cmsghdr
    } else {
        0 as *mut cmsghdr
    };
    while !cmsg.is_null() {
        if (*cmsg).cmsg_level == IPPROTO_IP as libc::c_int && (*cmsg).cmsg_type == 8 as libc::c_int
        {
            let mut ipi: in_pktinfo = in_pktinfo {
                ipi_ifindex: 0,
                ipi_spec_dst: in_addr { s_addr: 0 },
                ipi_addr: in_addr { s_addr: 0 },
            };
            if (*message).addr_type as libc::c_uint != SCK_ADDR_IP as libc::c_int as libc::c_uint {
                init_message_addresses(message, SCK_ADDR_IP);
            }
            memcpy(
                &mut ipi as *mut in_pktinfo as *mut libc::c_void,
                (*cmsg).__cmsg_data.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<in_pktinfo>() as libc::c_ulong,
            );
            (*message).local_addr.ip.addr.in4 = ntohl(ipi.ipi_addr.s_addr);
            (*message).local_addr.ip.family = 1 as libc::c_int as uint16_t;
            (*message).if_index = ipi.ipi_ifindex
        }
        if (*cmsg).cmsg_level == IPPROTO_IPV6 as libc::c_int
            && (*cmsg).cmsg_type == 50 as libc::c_int
        {
            let mut ipi_0: in6_pktinfo = in6_pktinfo {
                ipi6_addr: in6_addr {
                    __in6_u: C2RustUnnamed_2 {
                        __u6_addr8: [0; 16],
                    },
                },
                ipi6_ifindex: 0,
            };
            if (*message).addr_type as libc::c_uint != SCK_ADDR_IP as libc::c_int as libc::c_uint {
                init_message_addresses(message, SCK_ADDR_IP);
            }
            memcpy(
                &mut ipi_0 as *mut in6_pktinfo as *mut libc::c_void,
                (*cmsg).__cmsg_data.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<in6_pktinfo>() as libc::c_ulong,
            );
            memcpy(
                &mut (*message).local_addr.ip.addr.in6 as *mut [uint8_t; 16] as *mut libc::c_void,
                &mut ipi_0.ipi6_addr.__in6_u.__u6_addr8 as *mut [uint8_t; 16]
                    as *const libc::c_void,
                ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
            );
            (*message).local_addr.ip.family = 2 as libc::c_int as uint16_t;
            (*message).if_index = ipi_0.ipi6_ifindex as libc::c_int
        }
        if (*cmsg).cmsg_level == 1 as libc::c_int && (*cmsg).cmsg_type == 29 as libc::c_int {
            let mut tv: timeval = timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            memcpy(
                &mut tv as *mut timeval as *mut libc::c_void,
                (*cmsg).__cmsg_data.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<timeval>() as libc::c_ulong,
            );
            UTI_TimevalToTimespec(&mut tv, &mut (*message).timestamp.kernel);
        }
        if (*cmsg).cmsg_level == 1 as libc::c_int && (*cmsg).cmsg_type == 35 as libc::c_int {
            memcpy(
                &mut (*message).timestamp.kernel as *mut timespec as *mut libc::c_void,
                (*cmsg).__cmsg_data.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<timespec>() as libc::c_ulong,
            );
        }
        if (*cmsg).cmsg_level == 1 as libc::c_int && (*cmsg).cmsg_type == 58 as libc::c_int {
            let mut ts_pktinfo: scm_ts_pktinfo = scm_ts_pktinfo {
                if_index: 0,
                pkt_length: 0,
                reserved: [0; 2],
            };
            memcpy(
                &mut ts_pktinfo as *mut scm_ts_pktinfo as *mut libc::c_void,
                (*cmsg).__cmsg_data.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<scm_ts_pktinfo>() as libc::c_ulong,
            );
            (*message).timestamp.if_index = ts_pktinfo.if_index as libc::c_int;
            (*message).timestamp.l2_length = ts_pktinfo.pkt_length as libc::c_int
        }
        if (*cmsg).cmsg_level == 1 as libc::c_int && (*cmsg).cmsg_type == 37 as libc::c_int {
            let mut ts3: scm_timestamping = scm_timestamping {
                ts: [timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                }; 3],
            };
            memcpy(
                &mut ts3 as *mut scm_timestamping as *mut libc::c_void,
                (*cmsg).__cmsg_data.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<scm_timestamping>() as libc::c_ulong,
            );
            (*message).timestamp.kernel = ts3.ts[0 as libc::c_int as usize];
            (*message).timestamp.hw = ts3.ts[2 as libc::c_int as usize]
        }
        if (*cmsg).cmsg_level == 0 as libc::c_int && (*cmsg).cmsg_type == 11 as libc::c_int
            || (*cmsg).cmsg_level == 41 as libc::c_int && (*cmsg).cmsg_type == 25 as libc::c_int
        {
            let mut err: sock_extended_err = sock_extended_err {
                ee_errno: 0,
                ee_origin: 0,
                ee_type: 0,
                ee_code: 0,
                ee_pad: 0,
                ee_info: 0,
                ee_data: 0,
            };
            memcpy(
                &mut err as *mut sock_extended_err as *mut libc::c_void,
                (*cmsg).__cmsg_data.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<sock_extended_err>() as libc::c_ulong,
            );
            if err.ee_errno != 42 as libc::c_int as libc::c_uint
                || err.ee_info != SCM_TSTAMP_SND as libc::c_int as libc::c_uint
                || err.ee_origin as libc::c_int != 4 as libc::c_int
            {
                log_message(
                    sock_fd,
                    1 as libc::c_int,
                    message,
                    b"Unexpected extended error in\x00" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
        }
        if (*cmsg).cmsg_level == 1 as libc::c_int && (*cmsg).cmsg_type == SCM_RIGHTS as libc::c_int
        {
            if flags & 2 as libc::c_int == 0
                || (*cmsg).cmsg_len
                    != ((::std::mem::size_of::<cmsghdr>() as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    .wrapping_add(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            {
                let mut i: libc::c_uint = 0;
                if 0 as libc::c_int != 0
                    && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
                {
                    LOG_Message(
                        LOGS_DEBUG,
                        b"Unexpected SCM_RIGHTS\x00" as *const u8 as *const libc::c_char,
                    );
                }
                i = 0 as libc::c_int as libc::c_uint;
                while ((::std::mem::size_of::<cmsghdr>() as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                .wrapping_add(
                    (i.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
                ) <= (*cmsg).cmsg_len
                {
                    close(
                        *((*cmsg).__cmsg_data.as_mut_ptr() as *mut libc::c_int).offset(i as isize),
                    );
                    i = i.wrapping_add(1)
                }
                return 0 as libc::c_int;
            }
            (*message).descriptor = *((*cmsg).__cmsg_data.as_mut_ptr() as *mut libc::c_int)
        }
        cmsg = __cmsg_nxthdr(msg, cmsg)
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn receive_messages(
    mut sock_fd: libc::c_int,
    mut messages: *mut SCK_Message,
    mut max_messages: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut hdr: *mut mmsghdr = 0 as *mut mmsghdr;
    let mut i: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    let mut ret: libc::c_int = 0;
    let mut recv_flags: libc::c_int = 0 as libc::c_int;
    if initialised != 0 {
    } else {
        __assert_fail(
            b"initialised\x00" as *const u8 as *const libc::c_char,
            b"socket.c\x00" as *const u8 as *const libc::c_char,
            869 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"int receive_messages(int, SCK_Message *, int, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if max_messages < 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Prepare used buffers for new messages */
    prepare_buffers(received_messages);
    received_messages = 0 as libc::c_int as libc::c_uint;
    hdr = ARR_GetElements(recv_headers) as *mut mmsghdr;
    n = ARR_GetSize(recv_headers);
    n = if n < max_messages as libc::c_uint {
        n
    } else {
        max_messages as libc::c_uint
    };
    if n >= 1 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(
            b"n >= 1\x00" as *const u8 as *const libc::c_char,
            b"socket.c\x00" as *const u8 as *const libc::c_char,
            881 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"int receive_messages(int, SCK_Message *, int, int)\x00",
            ))
            .as_ptr(),
        );
    }
    recv_flags = get_recv_flags(flags);
    ret = recvmmsg(sock_fd, hdr, n, recv_flags, 0 as *mut timespec);
    if ret >= 0 as libc::c_int {
        n = ret as libc::c_uint
    }
    if ret < 0 as libc::c_int {
        handle_recv_error(sock_fd, flags);
        return 0 as libc::c_int;
    }
    received_messages = n;
    i = 0 as libc::c_int as libc::c_uint;
    while i < n {
        hdr = ARR_GetElement(recv_headers, i) as *mut mmsghdr;
        if process_header(
            &mut (*hdr).msg_hdr,
            (*hdr).msg_len,
            sock_fd,
            flags,
            &mut *messages.offset(i as isize),
        ) == 0
        {
            return 0 as libc::c_int;
        }
        log_message(
            sock_fd,
            1 as libc::c_int,
            &mut *messages.offset(i as isize),
            if flags & 1 as libc::c_int != 0 {
                b"Received error\x00" as *const u8 as *const libc::c_char
            } else {
                b"Received\x00" as *const u8 as *const libc::c_char
            },
            0 as *const libc::c_char,
        );
        i = i.wrapping_add(1)
    }
    return n as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn add_control_message(
    mut msg: *mut msghdr,
    mut level: libc::c_int,
    mut type_0: libc::c_int,
    mut length: size_t,
    mut buf_length: size_t,
) -> *mut libc::c_void {
    let mut cmsg: *mut cmsghdr = 0 as *mut cmsghdr;
    let mut cmsg_space: size_t = 0;
    /* Avoid using CMSG_NXTHDR as the one in glibc does not support adding
    control messages: https://sourceware.org/bugzilla/show_bug.cgi?id=13500 */
    cmsg = (*msg).msg_control as *mut cmsghdr;
    cmsg_space = (length
        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
    .wrapping_add(
        (::std::mem::size_of::<cmsghdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if cmsg.is_null()
        || length > buf_length
        || (*msg).msg_controllen.wrapping_add(cmsg_space) > buf_length
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Could not add control message level=%d type=%d\x00" as *const u8
                    as *const libc::c_char,
                level,
                type_0,
            );
        }
        return 0 as *mut libc::c_void;
    }
    cmsg = (cmsg as *mut libc::c_char).offset((*msg).msg_controllen as isize) as *mut cmsghdr;
    memset(cmsg as *mut libc::c_void, 0 as libc::c_int, cmsg_space);
    (*cmsg).cmsg_level = level;
    (*cmsg).cmsg_type = type_0;
    (*cmsg).cmsg_len = ((::std::mem::size_of::<cmsghdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
    .wrapping_add(length);
    (*msg).msg_controllen =
        ((*msg).msg_controllen as libc::c_ulong).wrapping_add(cmsg_space) as size_t as size_t;
    return (*cmsg).__cmsg_data.as_mut_ptr() as *mut libc::c_void;
}
/* ================================================== */
unsafe extern "C" fn send_message(
    mut sock_fd: libc::c_int,
    mut message: *mut SCK_Message,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut cmsg_buf: [cmsghdr; 16] = [cmsghdr {
        cmsg_len: 0,
        cmsg_level: 0,
        cmsg_type: 0,
        __cmsg_data: [],
    }; 16];
    let mut saddr: sockaddr_all = sockaddr_all {
        in4: sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
    };
    let mut saddr_len: socklen_t = 0;
    let mut msg: msghdr = msghdr {
        msg_name: 0 as *mut libc::c_void,
        msg_namelen: 0,
        msg_iov: 0 as *mut iovec,
        msg_iovlen: 0,
        msg_control: 0 as *mut libc::c_void,
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut iov: iovec = iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    };
    match (*message).addr_type as libc::c_uint {
        0 => saddr_len = 0 as libc::c_int as socklen_t,
        1 => {
            saddr_len = SCK_IPSockAddrToSockaddr(
                &mut (*message).remote_addr.ip,
                &mut saddr as *mut sockaddr_all as *mut sockaddr,
                ::std::mem::size_of::<sockaddr_all>() as libc::c_ulong as libc::c_int,
            ) as socklen_t
        }
        2 => {
            memset(
                &mut saddr as *mut sockaddr_all as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr_all>() as libc::c_ulong,
            );
            if snprintf(
                saddr.un.sun_path.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                (*message).remote_addr.path,
            ) as libc::c_ulong
                >= ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong
            {
                if 0 as libc::c_int != 0
                    && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
                {
                    LOG_Message(
                        LOGS_DEBUG,
                        b"Unix socket path %s too long\x00" as *const u8 as *const libc::c_char,
                        (*message).remote_addr.path,
                    );
                }
                return 0 as libc::c_int;
            }
            saddr.un.sun_family = 1 as libc::c_int as sa_family_t;
            saddr_len = ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t
        }
        _ => {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"socket.c\x00" as *const u8 as *const libc::c_char,
                977 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"int send_message(int, SCK_Message *, int)\x00",
                ))
                .as_ptr(),
            );
        }
    }
    if saddr_len != 0 {
        msg.msg_name = &mut saddr.un as *mut sockaddr_un as *mut libc::c_void;
        msg.msg_namelen = saddr_len
    } else {
        msg.msg_name = 0 as *mut libc::c_void;
        msg.msg_namelen = 0 as libc::c_int as socklen_t
    }
    iov.iov_base = (*message).data;
    iov.iov_len = (*message).length as size_t;
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1 as libc::c_int as size_t;
    msg.msg_control = cmsg_buf.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen = 0 as libc::c_int as size_t;
    msg.msg_flags = 0 as libc::c_int;
    if (*message).addr_type as libc::c_uint == SCK_ADDR_IP as libc::c_int as libc::c_uint {
        if (*message).local_addr.ip.family as libc::c_int == 1 as libc::c_int {
            let mut ipi: *mut in_pktinfo = 0 as *mut in_pktinfo;
            ipi = add_control_message(
                &mut msg,
                IPPROTO_IP as libc::c_int,
                8 as libc::c_int,
                ::std::mem::size_of::<in_pktinfo>() as libc::c_ulong,
                ::std::mem::size_of::<[cmsghdr; 16]>() as libc::c_ulong,
            ) as *mut in_pktinfo;
            if ipi.is_null() {
                return 0 as libc::c_int;
            }
            (*ipi).ipi_spec_dst.s_addr = htonl((*message).local_addr.ip.addr.in4);
            if (*message).if_index != -(1 as libc::c_int) {
                (*ipi).ipi_ifindex = (*message).if_index
            }
        }
        if (*message).local_addr.ip.family as libc::c_int == 2 as libc::c_int {
            let mut ipi_0: *mut in6_pktinfo = 0 as *mut in6_pktinfo;
            ipi_0 = add_control_message(
                &mut msg,
                IPPROTO_IPV6 as libc::c_int,
                50 as libc::c_int,
                ::std::mem::size_of::<in6_pktinfo>() as libc::c_ulong,
                ::std::mem::size_of::<[cmsghdr; 16]>() as libc::c_ulong,
            ) as *mut in6_pktinfo;
            if ipi_0.is_null() {
                return 0 as libc::c_int;
            }
            memcpy(
                &mut (*ipi_0).ipi6_addr.__in6_u.__u6_addr8 as *mut [uint8_t; 16]
                    as *mut libc::c_void,
                &mut (*message).local_addr.ip.addr.in6 as *mut [uint8_t; 16] as *const libc::c_void,
                ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
            );
            if (*message).if_index != -(1 as libc::c_int) {
                (*ipi_0).ipi6_ifindex = (*message).if_index as libc::c_uint
            }
        }
    }
    if (*message).timestamp.tx_flags != 0 {
        let mut ts_tx_flags: *mut libc::c_int = 0 as *mut libc::c_int;
        /* Set timestamping flags for this message */
        ts_tx_flags = add_control_message(
            &mut msg,
            1 as libc::c_int,
            37 as libc::c_int,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ::std::mem::size_of::<[cmsghdr; 16]>() as libc::c_ulong,
        ) as *mut libc::c_int;
        if ts_tx_flags.is_null() {
            return 0 as libc::c_int;
        }
        *ts_tx_flags = (*message).timestamp.tx_flags
    }
    if flags & 2 as libc::c_int != 0 {
        let mut fd: *mut libc::c_int = 0 as *mut libc::c_int;
        fd = add_control_message(
            &mut msg,
            1 as libc::c_int,
            SCM_RIGHTS as libc::c_int,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ::std::mem::size_of::<[cmsghdr; 16]>() as libc::c_ulong,
        ) as *mut libc::c_int;
        if fd.is_null() {
            return 0 as libc::c_int;
        }
        *fd = (*message).descriptor
    }
    /* This is apparently required on some systems */
    if msg.msg_controllen == 0 as libc::c_int as libc::c_ulong {
        msg.msg_control = 0 as *mut libc::c_void
    }
    if sendmsg(sock_fd, &mut msg, 0 as libc::c_int) < 0 as libc::c_int as libc::c_long {
        log_message(
            sock_fd,
            -(1 as libc::c_int),
            message,
            b"Could not send\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    log_message(
        sock_fd,
        -(1 as libc::c_int),
        message,
        b"Sent\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_Initialise() {
    recv_messages =
        ARR_CreateInstance(::std::mem::size_of::<Message>() as libc::c_ulong as libc::c_uint);
    ARR_SetSize(recv_messages, 4 as libc::c_int as libc::c_uint);
    recv_headers =
        ARR_CreateInstance(::std::mem::size_of::<mmsghdr>() as libc::c_ulong as libc::c_uint);
    ARR_SetSize(recv_headers, 4 as libc::c_int as libc::c_uint);
    received_messages = 4 as libc::c_int as libc::c_uint;
    priv_bind_function = None;
    supported_socket_flags = 0 as libc::c_int;
    if check_socket_flag(
        SOCK_CLOEXEC as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ) != 0
    {
        supported_socket_flags |= SOCK_CLOEXEC as libc::c_int
    }
    if check_socket_flag(
        SOCK_NONBLOCK as libc::c_int,
        0 as libc::c_int,
        0o4000 as libc::c_int,
    ) != 0
    {
        supported_socket_flags |= SOCK_NONBLOCK as libc::c_int
    }
    initialised = 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_Finalise() {
    ARR_DestroyInstance(recv_headers);
    ARR_DestroyInstance(recv_messages);
    initialised = 0 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_IsFamilySupported(mut family: libc::c_int) -> libc::c_int {
    match family {
        1 => return 1 as libc::c_int,
        2 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_GetAnyLocalIPAddress(
    mut family: libc::c_int,
    mut local_addr: *mut IPAddr,
) {
    (*local_addr).family = family as uint16_t;
    match family {
        1 => (*local_addr).addr.in4 = 0 as libc::c_int as in_addr_t,
        2 => {
            memcpy(
                &mut (*local_addr).addr.in6 as *mut [uint8_t; 16] as *mut libc::c_void,
                &in6addr_any as *const in6_addr as *const libc::c_void,
                ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
            );
        }
        _ => {}
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_GetLoopbackIPAddress(
    mut family: libc::c_int,
    mut local_addr: *mut IPAddr,
) {
    (*local_addr).family = family as uint16_t;
    match family {
        1 => (*local_addr).addr.in4 = 0x7f000001 as libc::c_int as in_addr_t,
        2 => {
            memcpy(
                &mut (*local_addr).addr.in6 as *mut [uint8_t; 16] as *mut libc::c_void,
                &in6addr_loopback as *const in6_addr as *const libc::c_void,
                ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
            );
        }
        _ => {}
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_SetPrivBind(
    mut function: Option<
        unsafe extern "C" fn(_: libc::c_int, _: *mut sockaddr, _: socklen_t) -> libc::c_int,
    >,
) {
    priv_bind_function = function;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_OpenUdpSocket(
    mut remote_addr: *mut IPSockAddr,
    mut local_addr: *mut IPSockAddr,
    mut flags: libc::c_int,
) -> libc::c_int {
    return open_ip_socket(remote_addr, local_addr, SOCK_DGRAM as libc::c_int, flags);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_OpenTcpSocket(
    mut remote_addr: *mut IPSockAddr,
    mut local_addr: *mut IPSockAddr,
    mut flags: libc::c_int,
) -> libc::c_int {
    return open_ip_socket(remote_addr, local_addr, SOCK_STREAM as libc::c_int, flags);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_OpenUnixDatagramSocket(
    mut remote_addr: *const libc::c_char,
    mut local_addr: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    return open_unix_socket(remote_addr, local_addr, SOCK_DGRAM as libc::c_int, flags);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_OpenUnixStreamSocket(
    mut remote_addr: *const libc::c_char,
    mut local_addr: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    return open_unix_socket(remote_addr, local_addr, SOCK_STREAM as libc::c_int, flags);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_OpenUnixSocketPair(
    mut flags: libc::c_int,
    mut other_fd: *mut libc::c_int,
) -> libc::c_int {
    let mut sock_fd: libc::c_int = 0;
    /* Prefer SEQPACKET sockets over DGRAM in order to receive a zero-length
    message (end of file) when the other end is unexpectedly closed */
    sock_fd = open_unix_socket_pair(SOCK_SEQPACKET as libc::c_int, flags, other_fd);
    if sock_fd < 0 as libc::c_int && {
        sock_fd = open_unix_socket_pair(SOCK_DGRAM as libc::c_int, flags, other_fd);
        (sock_fd) < 0 as libc::c_int
    } {
        return -(4 as libc::c_int);
    }
    return sock_fd;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_SetIntOption(
    mut sock_fd: libc::c_int,
    mut level: libc::c_int,
    mut name: libc::c_int,
    mut value: libc::c_int,
) -> libc::c_int {
    if setsockopt(
        sock_fd,
        level,
        name,
        &mut value as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"setsockopt() failed fd=%d level=%d name=%d value=%d : %s\x00" as *const u8
                    as *const libc::c_char,
                sock_fd,
                level,
                name,
                value,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_GetIntOption(
    mut sock_fd: libc::c_int,
    mut level: libc::c_int,
    mut name: libc::c_int,
    mut value: *mut libc::c_int,
) -> libc::c_int {
    let mut len: socklen_t = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    if getsockopt(sock_fd, level, name, value as *mut libc::c_void, &mut len) < 0 as libc::c_int {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"getsockopt() failed fd=%d level=%d name=%d : %s\x00" as *const u8
                    as *const libc::c_char,
                sock_fd,
                level,
                name,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_EnableKernelRxTimestamping(mut sock_fd: libc::c_int) -> libc::c_int {
    if SCK_SetIntOption(
        sock_fd,
        1 as libc::c_int,
        35 as libc::c_int,
        1 as libc::c_int,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    if SCK_SetIntOption(
        sock_fd,
        1 as libc::c_int,
        29 as libc::c_int,
        1 as libc::c_int,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_ListenOnSocket(
    mut sock_fd: libc::c_int,
    mut backlog: libc::c_int,
) -> libc::c_int {
    if listen(sock_fd, backlog) < 0 as libc::c_int {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"listen() failed : %s\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_AcceptConnection(
    mut sock_fd: libc::c_int,
    mut remote_addr: *mut IPSockAddr,
) -> libc::c_int {
    let mut saddr: sockaddr_all = sockaddr_all {
        in4: sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
    };
    let mut saddr_len: socklen_t =
        ::std::mem::size_of::<sockaddr_all>() as libc::c_ulong as socklen_t;
    let mut conn_fd: libc::c_int = 0;
    conn_fd = accept(
        sock_fd,
        __SOCKADDR_ARG {
            __sockaddr__: &mut saddr.sa as *mut sockaddr,
        },
        &mut saddr_len,
    );
    if conn_fd < 0 as libc::c_int {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"accept() failed : %s\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return -(4 as libc::c_int);
    }
    if UTI_FdSetCloexec(conn_fd) == 0 || set_socket_nonblock(conn_fd) == 0 {
        close(conn_fd);
        return -(4 as libc::c_int);
    }
    SCK_SockaddrToIPSockAddr(&mut saddr.sa, saddr_len as libc::c_int, remote_addr);
    return conn_fd;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_ShutdownConnection(mut sock_fd: libc::c_int) -> libc::c_int {
    if shutdown(sock_fd, SHUT_RDWR as libc::c_int) < 0 as libc::c_int {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"shutdown() failed : %s\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_Receive(
    mut sock_fd: libc::c_int,
    mut buffer: *mut libc::c_void,
    mut length: libc::c_uint,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = recv(sock_fd, buffer, length as size_t, get_recv_flags(flags)) as libc::c_int;
    if r < 0 as libc::c_int {
        handle_recv_error(sock_fd, flags);
        return r;
    }
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"Received data fd=%d len=%d\x00" as *const u8 as *const libc::c_char,
            sock_fd,
            r,
        );
    }
    return r;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_Send(
    mut sock_fd: libc::c_int,
    mut buffer: *const libc::c_void,
    mut length: libc::c_uint,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if flags == 0 as libc::c_int {
    } else {
        __assert_fail(
            b"flags == 0\x00" as *const u8 as *const libc::c_char,
            b"socket.c\x00" as *const u8 as *const libc::c_char,
            1360 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"int SCK_Send(int, const void *, unsigned int, int)\x00",
            ))
            .as_ptr(),
        );
    }
    r = send(sock_fd, buffer, length as size_t, 0 as libc::c_int) as libc::c_int;
    if r < 0 as libc::c_int {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Could not send data fd=%d len=%u : %s\x00" as *const u8 as *const libc::c_char,
                sock_fd,
                length,
                strerror(*__errno_location()),
            );
        }
        return r;
    }
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"Sent data fd=%d len=%d\x00" as *const u8 as *const libc::c_char,
            sock_fd,
            r,
        );
    }
    return r;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_ReceiveMessage(
    mut sock_fd: libc::c_int,
    mut message: *mut SCK_Message,
    mut flags: libc::c_int,
) -> libc::c_int {
    return SCK_ReceiveMessages(sock_fd, message, 1 as libc::c_int, flags);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_ReceiveMessages(
    mut sock_fd: libc::c_int,
    mut messages: *mut SCK_Message,
    mut max_messages: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    return receive_messages(sock_fd, messages, max_messages, flags);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_InitMessage(
    mut message: *mut SCK_Message,
    mut addr_type: SCK_AddressType,
) {
    init_message_addresses(message, addr_type);
    init_message_nonaddress(message);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_SendMessage(
    mut sock_fd: libc::c_int,
    mut message: *mut SCK_Message,
    mut flags: libc::c_int,
) -> libc::c_int {
    return send_message(sock_fd, message, flags);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_RemoveSocket(mut sock_fd: libc::c_int) -> libc::c_int {
    let mut saddr: sockaddr_all = sockaddr_all {
        in4: sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
    };
    let mut saddr_len: socklen_t = 0;
    saddr_len = ::std::mem::size_of::<sockaddr_all>() as libc::c_ulong as socklen_t;
    if getsockname(
        sock_fd,
        __SOCKADDR_ARG {
            __sockaddr__: &mut saddr.sa as *mut sockaddr,
        },
        &mut saddr_len,
    ) < 0 as libc::c_int
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"getsockname() failed : %s\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return 0 as libc::c_int;
    }
    if saddr_len as libc::c_ulong > ::std::mem::size_of::<sockaddr_all>() as libc::c_ulong
        || saddr_len as libc::c_ulong <= ::std::mem::size_of::<sa_family_t>() as libc::c_ulong
        || saddr.sa.sa_family as libc::c_int != 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if UTI_RemoveFile(
        0 as *const libc::c_char,
        saddr.un.sun_path.as_mut_ptr(),
        0 as *const libc::c_char,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_CloseSocket(mut sock_fd: libc::c_int) {
    close(sock_fd);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_SockaddrToIPSockAddr(
    mut sa: *mut sockaddr,
    mut sa_length: libc::c_int,
    mut ip_sa: *mut IPSockAddr,
) {
    (*ip_sa).ip_addr.family = 0 as libc::c_int as uint16_t;
    (*ip_sa).port = 0 as libc::c_int as uint16_t;
    match (*sa).sa_family as libc::c_int {
        2 => {
            if (sa_length as libc::c_ulong) < ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
            {
                return;
            }
            (*ip_sa).ip_addr.family = 1 as libc::c_int as uint16_t;
            (*ip_sa).ip_addr.addr.in4 = ntohl((*(sa as *mut sockaddr_in)).sin_addr.s_addr);
            (*ip_sa).port = ntohs((*(sa as *mut sockaddr_in)).sin_port)
        }
        10 => {
            if (sa_length as libc::c_ulong) < ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong
            {
                return;
            }
            (*ip_sa).ip_addr.family = 2 as libc::c_int as uint16_t;
            memcpy(
                &mut (*ip_sa).ip_addr.addr.in6 as *mut [uint8_t; 16] as *mut libc::c_void,
                (*(sa as *mut sockaddr_in6))
                    .sin6_addr
                    .__in6_u
                    .__u6_addr8
                    .as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
            );
            (*ip_sa).port = ntohs((*(sa as *mut sockaddr_in6)).sin6_port)
        }
        _ => {}
    };
}
/* Initialisation function */
/* Finalisation function */
/* Check if support for the IP family was enabled in the build */
/* Get the 0.0.0.0/::0 or 127.0.0.1/::1 address */
/* Specify a bind()-like function for binding sockets to privileged ports when
running in a restricted process (e.g. after dropping root privileges) */
/* Open socket */
/* Set and get a socket option of int size */
/* Enable RX timestamping socket option */
/* Operate on a stream socket - listen()/accept()/shutdown() wrappers */
/* Receive and send data on connected sockets - recv()/send() wrappers */
/* Receive a single message or multiple messages.  The functions return the
number of received messages, or 0 on error.  The returned data point to
static buffers, which are valid until another call of these functions.  */
/* Initialise a new message (e.g. before sending) */
/* Send a message */
/* Remove bound Unix socket */
/* Close the socket */
/* Convert between IPSockAddr and sockaddr_in/in6 */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCK_IPSockAddrToSockaddr(
    mut ip_sa: *mut IPSockAddr,
    mut sa: *mut sockaddr,
    mut sa_length: libc::c_int,
) -> libc::c_int {
    match (*ip_sa).ip_addr.family as libc::c_int {
        1 => {
            if (sa_length as libc::c_ulong) < ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
            {
                return 0 as libc::c_int;
            }
            memset(
                sa as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
            );
            (*sa).sa_family = 2 as libc::c_int as sa_family_t;
            (*(sa as *mut sockaddr_in)).sin_addr.s_addr = htonl((*ip_sa).ip_addr.addr.in4);
            (*(sa as *mut sockaddr_in)).sin_port = htons((*ip_sa).port);
            return ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as libc::c_int;
        }
        2 => {
            if (sa_length as libc::c_ulong) < ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong
            {
                return 0 as libc::c_int;
            }
            memset(
                sa as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong,
            );
            (*sa).sa_family = 10 as libc::c_int as sa_family_t;
            memcpy(
                &mut (*(sa as *mut sockaddr_in6)).sin6_addr.__in6_u.__u6_addr8 as *mut [uint8_t; 16]
                    as *mut libc::c_void,
                (*ip_sa).ip_addr.addr.in6.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
            );
            (*(sa as *mut sockaddr_in6)).sin6_port = htons((*ip_sa).port);
            return ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as libc::c_int;
        }
        _ => {
            if (sa_length as libc::c_ulong) < ::std::mem::size_of::<sockaddr>() as libc::c_ulong {
                return 0 as libc::c_int;
            }
            memset(
                sa as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr>() as libc::c_ulong,
            );
            (*sa).sa_family = 0 as libc::c_int as sa_family_t;
            return 0 as libc::c_int;
        }
    };
}
