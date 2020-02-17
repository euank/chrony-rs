#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type ARR_Instance_Record;
    pub type HCL_Instance_Record;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    #[no_mangle]
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn getifaddrs(__ifap: *mut *mut ifaddrs) -> libc::c_int;
    #[no_mangle]
    fn freeifaddrs(__ifa: *mut ifaddrs);
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
    #[no_mangle]
    fn CNF_GetHwTsInterface(index: libc::c_uint,
                            iface: *mut *mut CNF_HwTsInterface)
     -> libc::c_int;
    /* Create a new HW clock instance */
    #[no_mangle]
    fn HCL_CreateInstance(min_samples: libc::c_int, max_samples: libc::c_int,
                          min_separation: libc::c_double) -> HCL_Instance;
    /* Destroy a HW clock instance */
    #[no_mangle]
    fn HCL_DestroyInstance(clock: HCL_Instance);
    /* Check if a new sample should be accumulated at this time */
    #[no_mangle]
    fn HCL_NeedsNewSample(clock: HCL_Instance, now: *mut timespec)
     -> libc::c_int;
    /* Accumulate a new sample */
    #[no_mangle]
    fn HCL_AccumulateSample(clock: HCL_Instance, hw_ts: *mut timespec,
                            local_ts: *mut timespec, err: libc::c_double);
    /* Convert raw hardware time to cooked local time */
    #[no_mangle]
    fn HCL_CookTime(clock: HCL_Instance, raw: *mut timespec,
                    cooked: *mut timespec, err: *mut libc::c_double)
     -> libc::c_int;
    /* Convert raw time to cooked. */
    #[no_mangle]
    fn LCL_CookTime(raw: *mut timespec, cooked: *mut timespec,
                    err: *mut libc::c_double);
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
    /* Function to check if socket is a server socket */
    #[no_mangle]
    fn NIO_IsServerSocket(sock_fd: libc::c_int) -> libc::c_int;
    /* Open socket */
    #[no_mangle]
    fn SCK_OpenUdpSocket(remote_addr: *mut IPSockAddr,
                         local_addr: *mut IPSockAddr, flags: libc::c_int)
     -> libc::c_int;
    /* Set and get a socket option of int size */
    #[no_mangle]
    fn SCK_SetIntOption(sock_fd: libc::c_int, level: libc::c_int,
                        name: libc::c_int, value: libc::c_int) -> libc::c_int;
    /* Close the socket */
    #[no_mangle]
    fn SCK_CloseSocket(sock_fd: libc::c_int);
    /* This routine is called by ntp_io when a packet was sent to the network and
   an accurate transmit timestamp was captured */
    #[no_mangle]
    fn NSR_ProcessTx(remote_addr: *mut NTP_Remote_Address,
                     local_addr: *mut NTP_Local_Address,
                     tx_ts: *mut NTP_Local_Timestamp,
                     message: *mut NTP_Packet, length: libc::c_int);
    #[no_mangle]
    fn SCH_SetFileHandlerEvent(fd: libc::c_int, event: libc::c_int,
                               enable: libc::c_int);
    /* This queues a timeout to elapse at a given delta time relative to the current (raw) time */
    #[no_mangle]
    fn SCH_AddTimeoutByDelay(delay: libc::c_double, _: SCH_TimeoutHandler,
                             _: SCH_ArbitraryArgument) -> SCH_TimeoutID;
    /* The next one probably ought to return a status code */
    #[no_mangle]
    fn SCH_RemoveTimeout(_: SCH_TimeoutID);
    #[no_mangle]
    fn SYS_Linux_CheckKernelVersion(req_major: libc::c_int,
                                    req_minor: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SYS_Linux_OpenPHC(path: *const libc::c_char, phc_index: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn SYS_Linux_GetPHCSample(fd: libc::c_int, nocrossts: libc::c_int,
                              precision: libc::c_double,
                              reading_mode: *mut libc::c_int,
                              phc_ts: *mut timespec, sys_ts: *mut timespec,
                              err: *mut libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn UTI_AddDoubleToTimespec(start: *mut timespec,
                               increment: libc::c_double, end: *mut timespec);
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec)
     -> libc::c_double;
    #[no_mangle]
    fn UTI_IsZeroTimespec(ts: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn UTI_IPSockAddrToString(sa: *mut IPSockAddr) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_Log2ToDouble(l: libc::c_int) -> libc::c_double;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __caddr_t = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type __u8 = libc::c_uchar;
pub type __u16 = libc::c_ushort;
pub type __s32 = libc::c_int;
pub type __u32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut libc::c_char,
    pub ifa_flags: libc::c_uint,
    pub ifa_addr: *mut sockaddr,
    pub ifa_netmask: *mut sockaddr,
    pub ifa_ifu: C2RustUnnamed,
    pub ifa_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ifu_broadaddr: *mut sockaddr,
    pub ifu_dstaddr: *mut sockaddr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ethtool_cmd {
    pub cmd: __u32,
    pub supported: __u32,
    pub advertising: __u32,
    pub speed: __u16,
    pub duplex: __u8,
    pub port: __u8,
    pub phy_address: __u8,
    pub transceiver: __u8,
    pub autoneg: __u8,
    pub mdio_support: __u8,
    pub maxtxpkt: __u32,
    pub maxrxpkt: __u32,
    pub speed_hi: __u16,
    pub eth_tp_mdix: __u8,
    pub eth_tp_mdix_ctrl: __u8,
    pub lp_advertising: __u32,
    pub reserved: [__u32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ethtool_ts_info {
    pub cmd: __u32,
    pub so_timestamping: __u32,
    pub phc_index: __s32,
    pub tx_types: __u32,
    pub tx_reserved: [__u32; 3],
    pub rx_filters: __u32,
    pub rx_reserved: [__u32; 3],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SOF_TIMESTAMPING_MASK: C2RustUnnamed_0 = 32767;
pub const SOF_TIMESTAMPING_LAST: C2RustUnnamed_0 = 16384;
pub const SOF_TIMESTAMPING_OPT_TX_SWHW: C2RustUnnamed_0 = 16384;
pub const SOF_TIMESTAMPING_OPT_PKTINFO: C2RustUnnamed_0 = 8192;
pub const SOF_TIMESTAMPING_OPT_STATS: C2RustUnnamed_0 = 4096;
pub const SOF_TIMESTAMPING_OPT_TSONLY: C2RustUnnamed_0 = 2048;
pub const SOF_TIMESTAMPING_OPT_CMSG: C2RustUnnamed_0 = 1024;
pub const SOF_TIMESTAMPING_TX_ACK: C2RustUnnamed_0 = 512;
pub const SOF_TIMESTAMPING_TX_SCHED: C2RustUnnamed_0 = 256;
pub const SOF_TIMESTAMPING_OPT_ID: C2RustUnnamed_0 = 128;
pub const SOF_TIMESTAMPING_RAW_HARDWARE: C2RustUnnamed_0 = 64;
pub const SOF_TIMESTAMPING_SYS_HARDWARE: C2RustUnnamed_0 = 32;
pub const SOF_TIMESTAMPING_SOFTWARE: C2RustUnnamed_0 = 16;
pub const SOF_TIMESTAMPING_RX_SOFTWARE: C2RustUnnamed_0 = 8;
pub const SOF_TIMESTAMPING_RX_HARDWARE: C2RustUnnamed_0 = 4;
pub const SOF_TIMESTAMPING_TX_SOFTWARE: C2RustUnnamed_0 = 2;
pub const SOF_TIMESTAMPING_TX_HARDWARE: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hwtstamp_config {
    pub flags: libc::c_int,
    pub tx_type: libc::c_int,
    pub rx_filter: libc::c_int,
}
pub type hwtstamp_tx_types = libc::c_uint;
pub const HWTSTAMP_TX_ONESTEP_SYNC: hwtstamp_tx_types = 2;
pub const HWTSTAMP_TX_ON: hwtstamp_tx_types = 1;
pub const HWTSTAMP_TX_OFF: hwtstamp_tx_types = 0;
pub type hwtstamp_rx_filters = libc::c_uint;
pub const HWTSTAMP_FILTER_NTP_ALL: hwtstamp_rx_filters = 15;
pub const HWTSTAMP_FILTER_PTP_V2_DELAY_REQ: hwtstamp_rx_filters = 14;
pub const HWTSTAMP_FILTER_PTP_V2_SYNC: hwtstamp_rx_filters = 13;
pub const HWTSTAMP_FILTER_PTP_V2_EVENT: hwtstamp_rx_filters = 12;
pub const HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ: hwtstamp_rx_filters = 11;
pub const HWTSTAMP_FILTER_PTP_V2_L2_SYNC: hwtstamp_rx_filters = 10;
pub const HWTSTAMP_FILTER_PTP_V2_L2_EVENT: hwtstamp_rx_filters = 9;
pub const HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ: hwtstamp_rx_filters = 8;
pub const HWTSTAMP_FILTER_PTP_V2_L4_SYNC: hwtstamp_rx_filters = 7;
pub const HWTSTAMP_FILTER_PTP_V2_L4_EVENT: hwtstamp_rx_filters = 6;
pub const HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ: hwtstamp_rx_filters = 5;
pub const HWTSTAMP_FILTER_PTP_V1_L4_SYNC: hwtstamp_rx_filters = 4;
pub const HWTSTAMP_FILTER_PTP_V1_L4_EVENT: hwtstamp_rx_filters = 3;
pub const HWTSTAMP_FILTER_SOME: hwtstamp_rx_filters = 2;
pub const HWTSTAMP_FILTER_ALL: hwtstamp_rx_filters = 1;
pub const HWTSTAMP_FILTER_NONE: hwtstamp_rx_filters = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifmap {
    pub mem_start: libc::c_ulong,
    pub mem_end: libc::c_ulong,
    pub base_addr: libc::c_ushort,
    pub irq: libc::c_uchar,
    pub dma: libc::c_uchar,
    pub port: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: C2RustUnnamed_2,
    pub ifr_ifru: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: libc::c_short,
    pub ifru_ivalue: libc::c_int,
    pub ifru_mtu: libc::c_int,
    pub ifru_map: ifmap,
    pub ifru_slave: [libc::c_char; 16],
    pub ifru_newname: [libc::c_char; 16],
    pub ifru_data: __caddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ifrn_name: [libc::c_char; 16],
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
    pub addr: C2RustUnnamed_3,
    pub family: uint16_t,
    pub _pad: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
pub type CNF_HwTs_RxFilter = libc::c_uint;
pub const CNF_HWTS_RXFILTER_ALL: CNF_HwTs_RxFilter = 3;
pub const CNF_HWTS_RXFILTER_NTP: CNF_HwTs_RxFilter = 2;
pub const CNF_HWTS_RXFILTER_NONE: CNF_HwTs_RxFilter = 1;
pub const CNF_HWTS_RXFILTER_ANY: CNF_HwTs_RxFilter = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CNF_HwTsInterface {
    pub name: *mut libc::c_char,
    pub minpoll: libc::c_int,
    pub min_samples: libc::c_int,
    pub max_samples: libc::c_int,
    pub nocrossts: libc::c_int,
    pub rxfilter: CNF_HwTs_RxFilter,
    pub precision: libc::c_double,
    pub tx_comp: libc::c_double,
    pub rx_comp: libc::c_double,
}
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

  Header for tracking of hardware clocks */
pub type HCL_Instance = *mut HCL_Instance_Record;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
pub type NTP_Timestamp_Source = libc::c_uint;
pub const NTP_TS_HARDWARE: NTP_Timestamp_Source = 2;
pub const NTP_TS_KERNEL: NTP_Timestamp_Source = 1;
pub const NTP_TS_DAEMON: NTP_Timestamp_Source = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_Local_Timestamp {
    pub ts: timespec,
    pub err: libc::c_double,
    pub source: NTP_Timestamp_Source,
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
    pub remote_addr: C2RustUnnamed_6,
    pub local_addr: C2RustUnnamed_5,
    pub timestamp: C2RustUnnamed_4,
    pub descriptor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub kernel: timespec,
    pub hw: timespec,
    pub if_index: libc::c_int,
    pub l2_length: libc::c_int,
    pub tx_flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ip: IPAddr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ip: IPSockAddr,
    pub path: *const libc::c_char,
}
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2016-2019
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

  Functions for NTP I/O specific to Linux
  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Interface {
    pub name: [libc::c_char; 16],
    pub if_index: libc::c_int,
    pub phc_fd: libc::c_int,
    pub phc_mode: libc::c_int,
    pub phc_nocrossts: libc::c_int,
    pub link_speed: libc::c_int,
    pub l2_udp4_ntp_start: libc::c_int,
    pub l2_udp6_ntp_start: libc::c_int,
    pub precision: libc::c_double,
    pub tx_comp: libc::c_double,
    pub rx_comp: libc::c_double,
    pub clock: HCL_Instance,
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

  Exported header file for sched.c
  */
/* Type for timeout IDs, valid IDs are always greater than zero */
pub type SCH_TimeoutID = libc::c_uint;
pub type SCH_ArbitraryArgument = *mut libc::c_void;
pub type SCH_TimeoutHandler
    =
    Option<unsafe extern "C" fn(_: SCH_ArbitraryArgument) -> ()>;
#[inline]
unsafe extern "C" fn ethtool_cmd_speed(mut ep: *const ethtool_cmd) -> __u32 {
    return (((*ep).speed_hi as libc::c_int) << 16 as libc::c_int |
                (*ep).speed as libc::c_int) as __u32;
}
/* Array of Interfaces */
static mut interfaces: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
/* RX/TX and TX-specific timestamping socket options */
static mut ts_flags: libc::c_int = 0;
static mut ts_tx_flags: libc::c_int = 0;
/* Flag indicating the socket options can't be changed in control messages */
static mut permanent_ts_options: libc::c_int = 0;
/* When sending client requests to a close and fast server, it is possible that
   a response will be received before the HW transmit timestamp of the request
   itself.  To avoid processing of the response without the HW timestamp, we
   monitor events returned by select() and suspend reading of packets from the
   receive queue for up to 200 microseconds.  As the requests are normally
   separated by at least 200 milliseconds, it is sufficient to monitor and
   suspend one socket at a time. */
static mut monitored_socket: libc::c_int = 0;
static mut suspended_socket: libc::c_int = 0;
static mut resume_timeout_id: SCH_TimeoutID = 0;
/* Unbound socket keeping the kernel RX timestamping permanently enabled
   in order to avoid a race condition between receiving a server response
   and the kernel actually starting to timestamp received packets after
   enabling the timestamping and sending a request */
static mut dummy_rxts_socket: libc::c_int = 0;
/* ================================================== */
unsafe extern "C" fn add_interface(mut conf_iface: *mut CNF_HwTsInterface)
 -> libc::c_int {
    let mut ts_info: ethtool_ts_info =
        ethtool_ts_info{cmd: 0,
                        so_timestamping: 0,
                        phc_index: 0,
                        tx_types: 0,
                        tx_reserved: [0; 3],
                        rx_filters: 0,
                        rx_reserved: [0; 3],};
    let mut ts_config: hwtstamp_config =
        hwtstamp_config{flags: 0, tx_type: 0, rx_filter: 0,};
    let mut req: ifreq =
        ifreq{ifr_ifrn: C2RustUnnamed_2{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_1{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut sock_fd: libc::c_int = 0;
    let mut if_index: libc::c_int = 0;
    let mut phc_fd: libc::c_int = 0;
    let mut req_hwts_flags: libc::c_int = 0;
    let mut rx_filter: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut iface: *mut Interface = 0 as *mut Interface;
    /* Check if the interface was not already added */
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(interfaces) {
        if strcmp((*conf_iface).name,
                  (*(ARR_GetElement(interfaces, i) as
                         *mut Interface)).name.as_mut_ptr()) == 0 {
            return 1 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    sock_fd =
        SCK_OpenUdpSocket(0 as *mut IPSockAddr, 0 as *mut IPSockAddr,
                          0 as libc::c_int);
    if sock_fd < 0 as libc::c_int { return 0 as libc::c_int }
    memset(&mut req as *mut ifreq as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<ifreq>() as libc::c_ulong);
    memset(&mut ts_info as *mut ethtool_ts_info as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<ethtool_ts_info>() as libc::c_ulong);
    if snprintf(req.ifr_ifrn.ifrn_name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                (*conf_iface).name) as libc::c_ulong >=
           ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong {
        SCK_CloseSocket(sock_fd);
        return 0 as libc::c_int
    }
    if ioctl(sock_fd, 0x8933 as libc::c_int as libc::c_ulong,
             &mut req as *mut ifreq) != 0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"ioctl(%s) failed : %s\x00" as *const u8 as
                            *const libc::c_char,
                        b"SIOCGIFINDEX\x00" as *const u8 as
                            *const libc::c_char,
                        strerror(*__errno_location()));
        }
        SCK_CloseSocket(sock_fd);
        return 0 as libc::c_int
    }
    if_index = req.ifr_ifru.ifru_ivalue;
    ts_info.cmd = 0x41 as libc::c_int as __u32;
    req.ifr_ifru.ifru_data =
        &mut ts_info as *mut ethtool_ts_info as *mut libc::c_char;
    if ioctl(sock_fd, 0x8946 as libc::c_int as libc::c_ulong,
             &mut req as *mut ifreq) != 0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"ioctl(%s) failed : %s\x00" as *const u8 as
                            *const libc::c_char,
                        b"SIOCETHTOOL\x00" as *const u8 as
                            *const libc::c_char,
                        strerror(*__errno_location()));
        }
        SCK_CloseSocket(sock_fd);
        return 0 as libc::c_int
    }
    req_hwts_flags =
        SOF_TIMESTAMPING_RX_HARDWARE as libc::c_int |
            SOF_TIMESTAMPING_TX_HARDWARE as libc::c_int |
            SOF_TIMESTAMPING_RAW_HARDWARE as libc::c_int;
    if ts_info.so_timestamping & req_hwts_flags as libc::c_uint !=
           req_hwts_flags as libc::c_uint {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"HW timestamping not supported on %s\x00" as
                            *const u8 as *const libc::c_char,
                        req.ifr_ifrn.ifrn_name.as_mut_ptr());
        }
        SCK_CloseSocket(sock_fd);
        return 0 as libc::c_int
    }
    if ts_info.phc_index < 0 as libc::c_int {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"PHC missing on %s\x00" as *const u8 as
                            *const libc::c_char,
                        req.ifr_ifrn.ifrn_name.as_mut_ptr());
        }
        SCK_CloseSocket(sock_fd);
        return 0 as libc::c_int
    }
    match (*conf_iface).rxfilter as libc::c_uint {
        0 => {
            if ts_info.rx_filters &
                   ((1 as libc::c_int) <<
                        HWTSTAMP_FILTER_NTP_ALL as libc::c_int) as
                       libc::c_uint != 0 {
                rx_filter = HWTSTAMP_FILTER_NTP_ALL as libc::c_int
            } else if ts_info.rx_filters &
                          ((1 as libc::c_int) <<
                               HWTSTAMP_FILTER_ALL as libc::c_int) as
                              libc::c_uint != 0 {
                rx_filter = HWTSTAMP_FILTER_ALL as libc::c_int
            } else { rx_filter = HWTSTAMP_FILTER_NONE as libc::c_int }
        }
        1 => { rx_filter = HWTSTAMP_FILTER_NONE as libc::c_int }
        2 => { rx_filter = HWTSTAMP_FILTER_NTP_ALL as libc::c_int }
        _ => { rx_filter = HWTSTAMP_FILTER_ALL as libc::c_int }
    }
    ts_config.flags = 0 as libc::c_int;
    ts_config.tx_type = HWTSTAMP_TX_ON as libc::c_int;
    ts_config.rx_filter = rx_filter;
    req.ifr_ifru.ifru_data =
        &mut ts_config as *mut hwtstamp_config as *mut libc::c_char;
    if ioctl(sock_fd, 0x89b0 as libc::c_int as libc::c_ulong,
             &mut req as *mut ifreq) != 0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"ioctl(%s) failed : %s\x00" as *const u8 as
                            *const libc::c_char,
                        b"SIOCSHWTSTAMP\x00" as *const u8 as
                            *const libc::c_char,
                        strerror(*__errno_location()));
        }
        /* Check the current timestamping configuration in case this interface
       allows only reading of the configuration and it was already configured
       as requested */
        req.ifr_ifru.ifru_data =
            &mut ts_config as *mut hwtstamp_config as *mut libc::c_char;
        if ioctl(sock_fd, 0x89b1 as libc::c_int as libc::c_ulong,
                 &mut req as *mut ifreq) != 0 ||
               ts_config.tx_type != HWTSTAMP_TX_ON as libc::c_int ||
               ts_config.rx_filter != rx_filter {
            SCK_CloseSocket(sock_fd);
            return 0 as libc::c_int
        }
    }
    SCK_CloseSocket(sock_fd);
    phc_fd = SYS_Linux_OpenPHC(0 as *const libc::c_char, ts_info.phc_index);
    if phc_fd < 0 as libc::c_int { return 0 as libc::c_int }
    iface = ARR_GetNewElement(interfaces) as *mut Interface;
    snprintf((*iface).name.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
             b"%s\x00" as *const u8 as *const libc::c_char,
             (*conf_iface).name);
    (*iface).if_index = if_index;
    (*iface).phc_fd = phc_fd;
    (*iface).phc_mode = 0 as libc::c_int;
    (*iface).phc_nocrossts = (*conf_iface).nocrossts;
    /* Start with 1 gbit and no VLANs or IPv4/IPv6 options */
    (*iface).link_speed = 1000 as libc::c_int;
    (*iface).l2_udp4_ntp_start = 42 as libc::c_int;
    (*iface).l2_udp6_ntp_start = 62 as libc::c_int;
    (*iface).precision = (*conf_iface).precision;
    (*iface).tx_comp = (*conf_iface).tx_comp;
    (*iface).rx_comp = (*conf_iface).rx_comp;
    (*iface).clock =
        HCL_CreateInstance((*conf_iface).min_samples,
                           (*conf_iface).max_samples,
                           UTI_Log2ToDouble(if (*conf_iface).minpoll >
                                                   -(6 as libc::c_int) {
                                                (*conf_iface).minpoll
                                            } else { -(6 as libc::c_int) }));
    LOG_Message(LOGS_INFO,
                b"Enabled HW timestamping %son %s\x00" as *const u8 as
                    *const libc::c_char,
                if ts_config.rx_filter == HWTSTAMP_FILTER_NONE as libc::c_int
                   {
                    b"(TX only) \x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                (*iface).name.as_mut_ptr());
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn add_all_interfaces(mut conf_iface_all:
                                            *mut CNF_HwTsInterface)
 -> libc::c_int {
    let mut conf_iface: CNF_HwTsInterface =
        CNF_HwTsInterface{name: 0 as *mut libc::c_char,
                          minpoll: 0,
                          min_samples: 0,
                          max_samples: 0,
                          nocrossts: 0,
                          rxfilter: CNF_HWTS_RXFILTER_ANY,
                          precision: 0.,
                          tx_comp: 0.,
                          rx_comp: 0.,};
    let mut ifaddr: *mut ifaddrs = 0 as *mut ifaddrs;
    let mut ifa: *mut ifaddrs = 0 as *mut ifaddrs;
    let mut r: libc::c_int = 0;
    conf_iface = *conf_iface_all;
    if getifaddrs(&mut ifaddr) != 0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"getifaddrs() failed : %s\x00" as *const u8 as
                            *const libc::c_char,
                        strerror(*__errno_location()));
        }
        return 0 as libc::c_int
    }
    r = 0 as libc::c_int;
    ifa = ifaddr;
    while !ifa.is_null() {
        conf_iface.name = (*ifa).ifa_name;
        if add_interface(&mut conf_iface) != 0 { r = 1 as libc::c_int }
        ifa = (*ifa).ifa_next
    }
    freeifaddrs(ifaddr);
    /* Return success if at least one interface was added */
    return r;
}
/* ================================================== */
unsafe extern "C" fn update_interface_speed(mut iface: *mut Interface) {
    let mut cmd: ethtool_cmd =
        ethtool_cmd{cmd: 0,
                    supported: 0,
                    advertising: 0,
                    speed: 0,
                    duplex: 0,
                    port: 0,
                    phy_address: 0,
                    transceiver: 0,
                    autoneg: 0,
                    mdio_support: 0,
                    maxtxpkt: 0,
                    maxrxpkt: 0,
                    speed_hi: 0,
                    eth_tp_mdix: 0,
                    eth_tp_mdix_ctrl: 0,
                    lp_advertising: 0,
                    reserved: [0; 2],};
    let mut req: ifreq =
        ifreq{ifr_ifrn: C2RustUnnamed_2{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_1{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut sock_fd: libc::c_int = 0;
    let mut link_speed: libc::c_int = 0;
    sock_fd =
        SCK_OpenUdpSocket(0 as *mut IPSockAddr, 0 as *mut IPSockAddr,
                          0 as libc::c_int);
    if sock_fd < 0 as libc::c_int { return }
    memset(&mut req as *mut ifreq as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<ifreq>() as libc::c_ulong);
    memset(&mut cmd as *mut ethtool_cmd as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<ethtool_cmd>() as libc::c_ulong);
    snprintf(req.ifr_ifrn.ifrn_name.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
             b"%s\x00" as *const u8 as *const libc::c_char,
             (*iface).name.as_mut_ptr());
    cmd.cmd = 0x1 as libc::c_int as __u32;
    req.ifr_ifru.ifru_data =
        &mut cmd as *mut ethtool_cmd as *mut libc::c_char;
    if ioctl(sock_fd, 0x8946 as libc::c_int as libc::c_ulong,
             &mut req as *mut ifreq) != 0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"ioctl(%s) failed : %s\x00" as *const u8 as
                            *const libc::c_char,
                        b"SIOCETHTOOL\x00" as *const u8 as
                            *const libc::c_char,
                        strerror(*__errno_location()));
        }
        SCK_CloseSocket(sock_fd);
        return
    }
    SCK_CloseSocket(sock_fd);
    link_speed = ethtool_cmd_speed(&mut cmd) as libc::c_int;
    if (*iface).link_speed != link_speed {
        (*iface).link_speed = link_speed;
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Updated speed of %s to %d Mb/s\x00" as *const u8 as
                            *const libc::c_char, (*iface).name.as_mut_ptr(),
                        link_speed);
        }
    };
}
/* ================================================== */
unsafe extern "C" fn check_timestamping_option(mut option: libc::c_int)
 -> libc::c_int {
    let mut sock_fd: libc::c_int = 0;
    sock_fd =
        SCK_OpenUdpSocket(0 as *mut IPSockAddr, 0 as *mut IPSockAddr,
                          0 as libc::c_int);
    if sock_fd < 0 as libc::c_int { return 0 as libc::c_int }
    if SCK_SetIntOption(sock_fd, 1 as libc::c_int, 37 as libc::c_int, option)
           == 0 {
        SCK_CloseSocket(sock_fd);
        return 0 as libc::c_int
    }
    SCK_CloseSocket(sock_fd);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn open_dummy_socket() -> libc::c_int {
    let mut sock_fd: libc::c_int = 0;
    let mut events: libc::c_int = 0 as libc::c_int;
    sock_fd =
        SCK_OpenUdpSocket(0 as *mut IPSockAddr, 0 as *mut IPSockAddr,
                          0 as libc::c_int);
    if sock_fd < 0 as libc::c_int { return -(3 as libc::c_int) }
    if NIO_Linux_SetTimestampSocketOptions(sock_fd, 1 as libc::c_int,
                                           &mut events) == 0 {
        SCK_CloseSocket(sock_fd);
        return -(3 as libc::c_int)
    }
    return sock_fd;
}
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

  This is the header file for the Linux-specific NTP socket I/O bits.
  */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_Linux_Initialise() {
    let mut conf_iface: *mut CNF_HwTsInterface = 0 as *mut CNF_HwTsInterface;
    let mut i: libc::c_uint = 0;
    let mut hwts: libc::c_int = 0;
    interfaces =
        ARR_CreateInstance(::std::mem::size_of::<Interface>() as libc::c_ulong
                               as libc::c_uint);
    /* Enable HW timestamping on specified interfaces.  If "*" was specified, try
     all interfaces.  If no interface was specified, enable SW timestamping. */
    hwts = 0 as libc::c_int;
    i = hwts as libc::c_uint;
    while CNF_GetHwTsInterface(i, &mut conf_iface) != 0 {
        if !(strcmp(b"*\x00" as *const u8 as *const libc::c_char,
                    (*conf_iface).name) == 0) {
            if add_interface(conf_iface) == 0 {
                LOG_Message(LOGS_FATAL,
                            b"Could not enable HW timestamping on %s\x00" as
                                *const u8 as *const libc::c_char,
                            (*conf_iface).name);
                exit(1 as libc::c_int);
            }
            hwts = 1 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as libc::c_uint;
    while CNF_GetHwTsInterface(i, &mut conf_iface) != 0 {
        if strcmp(b"*\x00" as *const u8 as *const libc::c_char,
                  (*conf_iface).name) != 0 {
            i = i.wrapping_add(1)
        } else {
            if add_all_interfaces(conf_iface) != 0 { hwts = 1 as libc::c_int }
            break ;
        }
    }
    ts_flags =
        SOF_TIMESTAMPING_SOFTWARE as libc::c_int |
            SOF_TIMESTAMPING_RX_SOFTWARE as libc::c_int;
    ts_tx_flags = SOF_TIMESTAMPING_TX_SOFTWARE as libc::c_int;
    if hwts != 0 {
        ts_flags |=
            SOF_TIMESTAMPING_RAW_HARDWARE as libc::c_int |
                SOF_TIMESTAMPING_RX_HARDWARE as libc::c_int;
        ts_tx_flags |= SOF_TIMESTAMPING_TX_HARDWARE as libc::c_int;
        if check_timestamping_option(SOF_TIMESTAMPING_OPT_PKTINFO as
                                         libc::c_int) != 0 {
            ts_flags |= SOF_TIMESTAMPING_OPT_PKTINFO as libc::c_int
        }
        if check_timestamping_option(SOF_TIMESTAMPING_OPT_TX_SWHW as
                                         libc::c_int) != 0 {
            ts_flags |= SOF_TIMESTAMPING_OPT_TX_SWHW as libc::c_int
        }
    }
    /* Enable IP_PKTINFO in messages looped back to the error queue */
    ts_flags |= SOF_TIMESTAMPING_OPT_CMSG as libc::c_int;
    /* Kernels before 4.7 ignore timestamping flags set in control messages */
    permanent_ts_options =
        (SYS_Linux_CheckKernelVersion(4 as libc::c_int, 7 as libc::c_int) ==
             0) as libc::c_int;
    monitored_socket = -(3 as libc::c_int);
    suspended_socket = -(3 as libc::c_int);
    dummy_rxts_socket = -(3 as libc::c_int);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_Linux_Finalise() {
    let mut iface: *mut Interface = 0 as *mut Interface;
    let mut i: libc::c_uint = 0;
    if dummy_rxts_socket != -(3 as libc::c_int) {
        SCK_CloseSocket(dummy_rxts_socket);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(interfaces) {
        iface = ARR_GetElement(interfaces, i) as *mut Interface;
        HCL_DestroyInstance((*iface).clock);
        close((*iface).phc_fd);
        i = i.wrapping_add(1)
    }
    ARR_DestroyInstance(interfaces);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_Linux_SetTimestampSocketOptions(mut sock_fd:
                                                                 libc::c_int,
                                                             mut client_only:
                                                                 libc::c_int,
                                                             mut events:
                                                                 *mut libc::c_int)
 -> libc::c_int {
    let mut val: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    if ts_flags == 0 { return 0 as libc::c_int }
    /* Enable SCM_TIMESTAMPING control messages and the socket's error queue in
     order to receive our transmitted packets with more accurate timestamps */
    val = 1 as libc::c_int;
    flags = ts_flags;
    if client_only != 0 || permanent_ts_options != 0 { flags |= ts_tx_flags }
    if SCK_SetIntOption(sock_fd, 1 as libc::c_int, 45 as libc::c_int, val) ==
           0 {
        ts_flags = 0 as libc::c_int;
        return 0 as libc::c_int
    }
    if SCK_SetIntOption(sock_fd, 1 as libc::c_int, 37 as libc::c_int, flags)
           == 0 {
        ts_flags = 0 as libc::c_int;
        return 0 as libc::c_int
    }
    *events |= 4 as libc::c_int;
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn resume_socket(mut sock_fd: libc::c_int) {
    if monitored_socket == sock_fd { monitored_socket = -(3 as libc::c_int) }
    if sock_fd == -(3 as libc::c_int) || sock_fd != suspended_socket {
        return
    }
    suspended_socket = -(3 as libc::c_int);
    SCH_SetFileHandlerEvent(sock_fd, 1 as libc::c_int, 1 as libc::c_int);
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"Resumed RX processing %s timeout fd=%d\x00" as *const u8
                        as *const libc::c_char,
                    if resume_timeout_id != 0 {
                        b"before\x00" as *const u8 as *const libc::c_char
                    } else { b"on\x00" as *const u8 as *const libc::c_char },
                    sock_fd);
    }
    if resume_timeout_id != 0 {
        SCH_RemoveTimeout(resume_timeout_id);
        resume_timeout_id = 0 as libc::c_int as SCH_TimeoutID
    };
}
/* ================================================== */
unsafe extern "C" fn resume_timeout(mut arg: *mut libc::c_void) {
    resume_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    resume_socket(suspended_socket);
}
/* ================================================== */
unsafe extern "C" fn suspend_socket(mut sock_fd: libc::c_int) {
    resume_socket(suspended_socket);
    suspended_socket = sock_fd;
    SCH_SetFileHandlerEvent(suspended_socket, 1 as libc::c_int,
                            0 as libc::c_int);
    resume_timeout_id =
        SCH_AddTimeoutByDelay(200.0e-6f64,
                              Some(resume_timeout as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           -> ()), 0 as *mut libc::c_void);
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"Suspended RX processing fd=%d\x00" as *const u8 as
                        *const libc::c_char, sock_fd);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_Linux_ProcessEvent(mut sock_fd: libc::c_int,
                                                mut event: libc::c_int)
 -> libc::c_int {
    if sock_fd != monitored_socket { return 0 as libc::c_int }
    if event == 1 as libc::c_int {
        suspend_socket(monitored_socket);
        monitored_socket = -(3 as libc::c_int);
        /* Don't process the message yet */
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn get_interface(mut if_index: libc::c_int)
 -> *mut Interface {
    let mut iface: *mut Interface = 0 as *mut Interface;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(interfaces) {
        iface = ARR_GetElement(interfaces, i) as *mut Interface;
        if (*iface).if_index != if_index {
            i = i.wrapping_add(1)
        } else { return iface }
    }
    return 0 as *mut Interface;
}
/* ================================================== */
unsafe extern "C" fn process_hw_timestamp(mut iface: *mut Interface,
                                          mut hw_ts: *mut timespec,
                                          mut local_ts:
                                              *mut NTP_Local_Timestamp,
                                          mut rx_ntp_length: libc::c_int,
                                          mut family: libc::c_int,
                                          mut l2_length: libc::c_int) {
    let mut sample_phc_ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut sample_sys_ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut sample_local_ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut rx_correction: libc::c_double = 0.;
    let mut ts_delay: libc::c_double = 0.;
    let mut phc_err: libc::c_double = 0.;
    let mut local_err: libc::c_double = 0.;
    if HCL_NeedsNewSample((*iface).clock, &mut (*local_ts).ts) != 0 {
        if SYS_Linux_GetPHCSample((*iface).phc_fd, (*iface).phc_nocrossts,
                                  (*iface).precision, &mut (*iface).phc_mode,
                                  &mut sample_phc_ts, &mut sample_sys_ts,
                                  &mut phc_err) == 0 {
            return
        }
        LCL_CookTime(&mut sample_sys_ts, &mut sample_local_ts,
                     &mut local_err);
        HCL_AccumulateSample((*iface).clock, &mut sample_phc_ts,
                             &mut sample_local_ts, phc_err + local_err);
        update_interface_speed(iface);
    }
    /* We need to transpose RX timestamps as hardware timestamps are normally
     preamble timestamps and RX timestamps in NTP are supposed to be trailer
     timestamps.  If we don't know the length of the packet at layer 2, we
     make an assumption that UDP data start at the same position as in the
     last transmitted packet which had a HW TX timestamp. */
    if rx_ntp_length != 0 && (*iface).link_speed != 0 {
        if l2_length == 0 {
            l2_length =
                (if family == 1 as libc::c_int {
                     (*iface).l2_udp4_ntp_start
                 } else { (*iface).l2_udp6_ntp_start }) + rx_ntp_length
        }
        /* Include the frame check sequence (FCS) */
        l2_length += 4 as libc::c_int;
        rx_correction =
            l2_length as libc::c_double /
                (1.0e6f64 / 8 as libc::c_int as libc::c_double *
                     (*iface).link_speed as libc::c_double);
        UTI_AddDoubleToTimespec(hw_ts, rx_correction, hw_ts);
    }
    if HCL_CookTime((*iface).clock, hw_ts, &mut ts, &mut local_err) == 0 {
        return
    }
    if rx_ntp_length == 0 && (*iface).tx_comp != 0. {
        UTI_AddDoubleToTimespec(&mut ts, (*iface).tx_comp, &mut ts);
    } else if rx_ntp_length != 0 && (*iface).rx_comp != 0. {
        UTI_AddDoubleToTimespec(&mut ts, -(*iface).rx_comp, &mut ts);
    }
    ts_delay = UTI_DiffTimespecsToDouble(&mut (*local_ts).ts, &mut ts);
    if fabs(ts_delay) > 1.0f64 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Unacceptable timestamp delay %.9f\x00" as *const u8
                            as *const libc::c_char, ts_delay);
        }
        return
    }
    (*local_ts).ts = ts;
    (*local_ts).err = local_err;
    (*local_ts).source = NTP_TS_HARDWARE;
}
/* ================================================== */
/* Extract UDP data from a layer 2 message.  Supported is Ethernet
   with optional VLAN tags. */
unsafe extern "C" fn extract_udp_data(mut msg: *mut libc::c_uchar,
                                      mut remote_addr:
                                          *mut NTP_Remote_Address,
                                      mut len: libc::c_int) -> libc::c_int {
    let mut msg_start: *mut libc::c_uchar = msg;
    (*remote_addr).ip_addr.family = 0 as libc::c_int as uint16_t;
    (*remote_addr).port = 0 as libc::c_int as uint16_t;
    /* Skip MACs */
    if len < 12 as libc::c_int { return 0 as libc::c_int }
    len -= 12 as libc::c_int;
    msg = msg.offset(12 as libc::c_int as isize);
    /* Skip VLAN tag(s) if present */
    while len >= 4 as libc::c_int &&
              *msg.offset(0 as libc::c_int as isize) as libc::c_int ==
                  0x81 as libc::c_int &&
              *msg.offset(1 as libc::c_int as isize) as libc::c_int ==
                  0 as libc::c_int {
        len -= 4 as libc::c_int;
        msg = msg.offset(4 as libc::c_int as isize)
    }
    /* Skip IPv4 or IPv6 ethertype */
    if len < 2 as libc::c_int ||
           !(*msg.offset(0 as libc::c_int as isize) as libc::c_int ==
                 0x8 as libc::c_int &&
                 *msg.offset(1 as libc::c_int as isize) as libc::c_int ==
                     0 as libc::c_int ||
                 *msg.offset(0 as libc::c_int as isize) as libc::c_int ==
                     0x86 as libc::c_int &&
                     *msg.offset(1 as libc::c_int as isize) as libc::c_int ==
                         0xdd as libc::c_int) {
        return 0 as libc::c_int
    }
    len -= 2 as libc::c_int;
    msg = msg.offset(2 as libc::c_int as isize);
    /* Parse destination address and port from IPv4/IPv6 and UDP headers */
    if len >= 20 as libc::c_int &&
           *msg.offset(0 as libc::c_int as isize) as libc::c_int >>
               4 as libc::c_int == 4 as libc::c_int {
        let mut ihl: libc::c_int =
            (*msg.offset(0 as libc::c_int as isize) as libc::c_int &
                 0xf as libc::c_int) * 4 as libc::c_int;
        let mut addr: uint32_t = 0;
        if len < ihl + 8 as libc::c_int ||
               *msg.offset(9 as libc::c_int as isize) as libc::c_int !=
                   17 as libc::c_int {
            return 0 as libc::c_int
        }
        memcpy(&mut addr as *mut uint32_t as *mut libc::c_void,
               msg.offset(16 as libc::c_int as isize) as *const libc::c_void,
               ::std::mem::size_of::<uint32_t>() as libc::c_ulong);
        (*remote_addr).ip_addr.addr.in4 = ntohl(addr);
        (*remote_addr).port =
            ntohs(*(msg.offset(ihl as isize).offset(2 as libc::c_int as isize)
                        as *mut uint16_t));
        (*remote_addr).ip_addr.family = 1 as libc::c_int as uint16_t;
        len -= ihl + 8 as libc::c_int;
        msg = msg.offset((ihl + 8 as libc::c_int) as isize)
    } else if len >= 48 as libc::c_int &&
                  *msg.offset(0 as libc::c_int as isize) as libc::c_int >>
                      4 as libc::c_int == 6 as libc::c_int {
        let mut eh_len: libc::c_int = 0;
        let mut next_header: libc::c_int =
            *msg.offset(6 as libc::c_int as isize) as libc::c_int;
        memcpy(&mut (*remote_addr).ip_addr.addr.in6 as *mut [uint8_t; 16] as
                   *mut libc::c_void,
               msg.offset(24 as libc::c_int as isize) as *const libc::c_void,
               ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong);
        len -= 40 as libc::c_int;
        msg = msg.offset(40 as libc::c_int as isize);
        /* Skip IPv6 extension headers if present */
        while next_header != 17 as libc::c_int {
            let mut current_block_25: u64;
            match next_header {
                44 => {
                    /* Fragment Header */
                    /* Process only the first fragment */
                    if ntohs(*(msg.offset(2 as libc::c_int as isize) as
                                   *mut uint16_t)) as libc::c_int >>
                           3 as libc::c_int != 0 as libc::c_int {
                        return 0 as libc::c_int
                    }
                    eh_len = 8 as libc::c_int;
                    current_block_25 = 11459959175219260272;
                }
                0 => {
                    /* Hop-by-Hop Options */
                    current_block_25 = 2555804151391322472;
                }
                43 => { current_block_25 = 2555804151391322472; }
                60 | 135 => { current_block_25 = 3274233182728301011; }
                51 => {
                    /* Authentication Header */
                    eh_len =
                        4 as libc::c_int *
                            (*msg.offset(1 as libc::c_int as isize) as
                                 libc::c_int + 2 as libc::c_int);
                    current_block_25 = 11459959175219260272;
                }
                _ => { return 0 as libc::c_int }
            }
            match current_block_25 {
                2555804151391322472 =>
                /* Routing Header */
                {
                    current_block_25 = 3274233182728301011;
                }
                _ => { }
            }
            match current_block_25 {
                3274233182728301011 =>
                /* Destination Options */
                /* Mobility Header */
                {
                    eh_len =
                        8 as libc::c_int *
                            (*msg.offset(1 as libc::c_int as isize) as
                                 libc::c_int + 1 as libc::c_int)
                }
                _ => { }
            }
            if eh_len < 8 as libc::c_int || len < eh_len + 8 as libc::c_int {
                return 0 as libc::c_int
            }
            next_header =
                *msg.offset(0 as libc::c_int as isize) as libc::c_int;
            len -= eh_len;
            msg = msg.offset(eh_len as isize)
        }
        (*remote_addr).port =
            ntohs(*(msg.offset(2 as libc::c_int as isize) as *mut uint16_t));
        (*remote_addr).ip_addr.family = 2 as libc::c_int as uint16_t;
        len -= 8 as libc::c_int;
        msg = msg.offset(8 as libc::c_int as isize)
    } else { return 0 as libc::c_int }
    /* Move the message to fix alignment of its fields */
    if len > 0 as libc::c_int {
        memmove(msg_start as *mut libc::c_void, msg as *const libc::c_void,
                len as libc::c_ulong);
    }
    return len;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_Linux_ProcessMessage(mut message:
                                                      *mut SCK_Message,
                                                  mut local_addr:
                                                      *mut NTP_Local_Address,
                                                  mut local_ts:
                                                      *mut NTP_Local_Timestamp,
                                                  mut event: libc::c_int)
 -> libc::c_int {
    let mut iface: *mut Interface = 0 as *mut Interface;
    let mut is_tx: libc::c_int = 0;
    let mut ts_if_index: libc::c_int = 0;
    let mut l2_length: libc::c_int = 0;
    is_tx = (event == 4 as libc::c_int) as libc::c_int;
    iface = 0 as *mut Interface;
    ts_if_index = (*message).timestamp.if_index;
    if ts_if_index == -(1 as libc::c_int) {
        ts_if_index = (*message).if_index
    }
    l2_length = (*message).timestamp.l2_length;
    if UTI_IsZeroTimespec(&mut (*message).timestamp.hw) == 0 {
        iface = get_interface(ts_if_index);
        if !iface.is_null() {
            process_hw_timestamp(iface, &mut (*message).timestamp.hw,
                                 local_ts,
                                 if is_tx == 0 {
                                     (*message).length
                                 } else { 0 as libc::c_int as libc::c_uint }
                                     as libc::c_int,
                                 (*message).remote_addr.ip.ip_addr.family as
                                     libc::c_int, l2_length);
        } else if 0 as libc::c_int != 0 &&
                      log_min_severity as libc::c_int ==
                          LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"HW clock not found for interface %d\x00" as
                            *const u8 as *const libc::c_char, ts_if_index);
        }
        /* If a HW transmit timestamp was received, resume processing
       of non-error messages on this socket */
        if is_tx != 0 { resume_socket((*local_addr).sock_fd); }
    }
    if (*local_ts).source as libc::c_uint ==
           NTP_TS_DAEMON as libc::c_int as libc::c_uint &&
           UTI_IsZeroTimespec(&mut (*message).timestamp.kernel) == 0 &&
           (is_tx == 0 ||
                UTI_IsZeroTimespec(&mut (*message).timestamp.hw) != 0) {
        LCL_CookTime(&mut (*message).timestamp.kernel, &mut (*local_ts).ts,
                     &mut (*local_ts).err);
        (*local_ts).source = NTP_TS_KERNEL
    }
    /* If the kernel is slow with enabling RX timestamping, open a dummy
     socket to keep the kernel RX timestamping permanently enabled */
    if is_tx == 0 &&
           (*local_ts).source as libc::c_uint ==
               NTP_TS_DAEMON as libc::c_int as libc::c_uint && ts_flags != 0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Missing kernel RX timestamp\x00" as *const u8 as
                            *const libc::c_char);
        }
        if dummy_rxts_socket == -(3 as libc::c_int) {
            dummy_rxts_socket = open_dummy_socket()
        }
    }
    /* Return the message if it's not received from the error queue */
    if is_tx == 0 { return 0 as libc::c_int }
    /* The data from the error queue includes all layers up to UDP.  We have to
     extract the UDP data and also the destination address with port as there
     currently doesn't seem to be a better way to get them both. */
    l2_length = (*message).length as libc::c_int;
    (*message).length =
        extract_udp_data((*message).data as *mut libc::c_uchar,
                         &mut (*message).remote_addr.ip,
                         (*message).length as libc::c_int) as libc::c_uint;
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"Extracted message for %s fd=%d len=%u\x00" as *const u8
                        as *const libc::c_char,
                    UTI_IPSockAddrToString(&mut (*message).remote_addr.ip),
                    (*local_addr).sock_fd, (*message).length);
    }
    /* Update assumed position of UDP data at layer 2 for next received packet */
    if !iface.is_null() && (*message).length != 0 {
        if (*message).remote_addr.ip.ip_addr.family as libc::c_int ==
               1 as libc::c_int {
            (*iface).l2_udp4_ntp_start =
                (l2_length as libc::c_uint).wrapping_sub((*message).length) as
                    libc::c_int
        } else if (*message).remote_addr.ip.ip_addr.family as libc::c_int ==
                      2 as libc::c_int {
            (*iface).l2_udp6_ntp_start =
                (l2_length as libc::c_uint).wrapping_sub((*message).length) as
                    libc::c_int
        }
    }
    /* Drop the message if it has no timestamp or its processing failed */
    if (*local_ts).source as libc::c_uint ==
           NTP_TS_DAEMON as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Missing TX timestamp\x00" as *const u8 as
                            *const libc::c_char);
        }
        return 1 as libc::c_int
    }
    if (*message).length < 48 as libc::c_ulong as libc::c_int as libc::c_uint
       {
        return 1 as libc::c_int
    }
    NSR_ProcessTx(&mut (*message).remote_addr.ip, local_addr, local_ts,
                  (*message).data as *mut NTP_Packet,
                  (*message).length as libc::c_int);
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_Linux_RequestTxTimestamp(mut message:
                                                          *mut SCK_Message,
                                                      mut sock_fd:
                                                          libc::c_int) {
    if ts_flags == 0 { return }
    /* If a HW transmit timestamp is requested on a client socket, monitor
     events on the socket in order to avoid processing of a fast response
     without the HW timestamp of the request */
    if ts_tx_flags & SOF_TIMESTAMPING_TX_HARDWARE as libc::c_int != 0 &&
           NIO_IsServerSocket(sock_fd) == 0 {
        monitored_socket = sock_fd
    }
    /* Check if TX timestamping is disabled on this socket */
    if permanent_ts_options != 0 || NIO_IsServerSocket(sock_fd) == 0 {
        return
    }
    (*message).timestamp.tx_flags = ts_tx_flags;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_Linux_NotifySocketClosing(mut sock_fd:
                                                           libc::c_int) {
    resume_socket(sock_fd);
}
