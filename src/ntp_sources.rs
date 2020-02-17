#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           register_tool)]
extern "C" {
    pub type ARR_Instance_Record;
    pub type NCR_Instance_Record;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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
    /* Return pointer to the internal array of elements */
    #[no_mangle]
    fn ARR_GetElements(array: ARR_Instance) -> *mut libc::c_void;
    /* Set the size of the array */
    #[no_mangle]
    fn ARR_SetSize(array: ARR_Instance, size: libc::c_uint);
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

  Header file for the main NTP protocol engine
  */
    /* This is a private data type used for storing the instance record for
   each source that we are chiming with */
    /* Init and fini functions */
    /* Get a new instance for a server or peer */
    /* Destroy an instance */
    /* Start an instance */
    /* Reset an instance */
    /* Reset polling interval of an instance */
    /* Change the remote address of an instance */
    /* This routine is called when a new packet arrives off the network,
   and it relates to a source we have an ongoing protocol exchange with */
    /* This routine is called when a new packet arrives off the network,
   and we do not recognize its source */
    /* This routine is called when a packet is sent to a source we have
   an ongoing protocol exchange with */
    /* This routine is called when a packet is sent to a destination we
   do not recognize */
    /* Slew receive and transmit times in instance records */
    /* Take a particular source online (i.e. start sampling it) or offline
   (i.e. stop sampling it) */
    #[no_mangle]
    fn NCR_IsSyncPeer(instance: NCR_Instance) -> libc::c_int;
    #[no_mangle]
    fn NCR_GetInstance(remote_addr: *mut NTP_Remote_Address,
                       type_0: NTP_Source_Type, params: *mut SourceParameters)
     -> NCR_Instance;
    #[no_mangle]
    fn NCR_DestroyInstance(instance: NCR_Instance);
    #[no_mangle]
    fn NCR_StartInstance(instance: NCR_Instance);
    #[no_mangle]
    fn NCR_ResetInstance(inst: NCR_Instance);
    #[no_mangle]
    fn NCR_ResetPoll(instance: NCR_Instance);
    #[no_mangle]
    fn NCR_ChangeRemoteAddress(inst: NCR_Instance,
                               remote_addr: *mut NTP_Remote_Address);
    #[no_mangle]
    fn NCR_ProcessRxKnown(inst: NCR_Instance,
                          local_addr: *mut NTP_Local_Address,
                          rx_ts: *mut NTP_Local_Timestamp,
                          message: *mut NTP_Packet, length: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn NCR_ProcessRxUnknown(remote_addr: *mut NTP_Remote_Address,
                            local_addr: *mut NTP_Local_Address,
                            rx_ts: *mut NTP_Local_Timestamp,
                            message: *mut NTP_Packet, length: libc::c_int);
    #[no_mangle]
    fn NCR_ProcessTxKnown(inst: NCR_Instance,
                          local_addr: *mut NTP_Local_Address,
                          tx_ts: *mut NTP_Local_Timestamp,
                          message: *mut NTP_Packet, length: libc::c_int);
    #[no_mangle]
    fn NCR_ProcessTxUnknown(remote_addr: *mut NTP_Remote_Address,
                            local_addr: *mut NTP_Local_Address,
                            tx_ts: *mut NTP_Local_Timestamp,
                            message: *mut NTP_Packet, length: libc::c_int);
    #[no_mangle]
    fn NCR_SlewTimes(inst: NCR_Instance, when: *mut timespec,
                     dfreq: libc::c_double, doffset: libc::c_double);
    #[no_mangle]
    fn NCR_SetConnectivity(inst: NCR_Instance,
                           connectivity: SRC_Connectivity);
    #[no_mangle]
    fn NCR_ModifyMinpoll(inst: NCR_Instance, new_minpoll: libc::c_int);
    #[no_mangle]
    fn NCR_ModifyMaxpoll(inst: NCR_Instance, new_maxpoll: libc::c_int);
    #[no_mangle]
    fn NCR_ModifyMaxdelay(inst: NCR_Instance, new_max_delay: libc::c_double);
    #[no_mangle]
    fn NCR_ModifyMaxdelayratio(inst: NCR_Instance,
                               new_max_delay_ratio: libc::c_double);
    #[no_mangle]
    fn NCR_ModifyMaxdelaydevratio(inst: NCR_Instance,
                                  new_max_delay_dev_ratio: libc::c_double);
    #[no_mangle]
    fn NCR_ModifyMinstratum(inst: NCR_Instance, new_min_stratum: libc::c_int);
    #[no_mangle]
    fn NCR_ModifyPolltarget(inst: NCR_Instance, new_poll_target: libc::c_int);
    #[no_mangle]
    fn NCR_InitiateSampleBurst(inst: NCR_Instance,
                               n_good_samples: libc::c_int,
                               n_total_samples: libc::c_int);
    #[no_mangle]
    fn NCR_ReportSource(inst: NCR_Instance, report: *mut RPT_SourceReport,
                        now: *mut timespec);
    #[no_mangle]
    fn NCR_GetNTPReport(inst: NCR_Instance, report: *mut RPT_NTPReport);
    #[no_mangle]
    fn NCR_IncrementActivityCounters(inst: NCR_Instance,
                                     online: *mut libc::c_int,
                                     offline: *mut libc::c_int,
                                     burst_online: *mut libc::c_int,
                                     burst_offline: *mut libc::c_int);
    #[no_mangle]
    fn NCR_GetRemoteAddress(instance: NCR_Instance)
     -> *mut NTP_Remote_Address;
    #[no_mangle]
    fn NCR_GetLocalRefid(inst: NCR_Instance) -> uint32_t;
    #[no_mangle]
    fn UTI_CompareIPs(a: *mut IPAddr, b: *mut IPAddr, mask: *mut IPAddr)
     -> libc::c_int;
    #[no_mangle]
    fn UTI_IPToHash(ip: *mut IPAddr) -> uint32_t;
    #[no_mangle]
    fn UTI_StringToIP(addr: *const libc::c_char, ip: *mut IPAddr)
     -> libc::c_int;
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec)
     -> libc::c_double;
    #[no_mangle]
    fn UTI_IPToString(ip: *mut IPAddr) -> *mut libc::c_char;
    /* Fill buffer with random bytes from /dev/urandom or a faster source if it's
   available (e.g. arc4random()), which may not necessarily be suitable for
   generating long-term keys */
    #[no_mangle]
    fn UTI_GetRandomBytes(buf: *mut libc::c_void, len: libc::c_uint);
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
    /* Add a handler.  Then handler MUST NOT deregister itself!!! */
    #[no_mangle]
    fn LCL_AddParameterChangeHandler(handler: LCL_ParameterChangeHandler,
                                     anything: *mut libc::c_void);
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
    fn Malloc2(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn Strdup(s: *const libc::c_char) -> *mut libc::c_char;
    /* Request resolving of a name to IP address. The handler will be
   called when the result is available. */
    #[no_mangle]
    fn DNS_Name2IPAddressAsync(name: *const libc::c_char,
                               handler: DNS_NameResolveHandler,
                               anything: *mut libc::c_void);
    #[no_mangle]
    fn DNS_Reload();
    /* Get the time stamp taken after a file descriptor became ready or a timeout expired */
    #[no_mangle]
    fn SCH_GetLastEventTime(cooked: *mut timespec, err: *mut libc::c_double,
                            raw: *mut timespec);
    /* This queues a timeout to elapse at a given delta time relative to the current (raw) time */
    #[no_mangle]
    fn SCH_AddTimeoutByDelay(delay: libc::c_double, _: SCH_TimeoutHandler,
                             _: SCH_ArbitraryArgument) -> SCH_TimeoutID;
    /* The next one probably ought to return a status code */
    #[no_mangle]
    fn SCH_RemoveTimeout(_: SCH_TimeoutID);
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
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
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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

  Types used for addressing sources etc
  */
/* This type is used to represent an IPv4 address or IPv6 address.
   All parts are in HOST order, NOT network order. */
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
pub type NTP_Remote_Address = IPSockAddr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_Local_Address {
    pub ip_addr: IPAddr,
    pub if_index: libc::c_int,
    pub sock_fd: libc::c_int,
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

  Data structure definitions within the daemon for various reports that
  can be generated */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_SourceReport {
    pub ip_addr: IPAddr,
    pub stratum: libc::c_int,
    pub poll: libc::c_int,
    pub mode: C2RustUnnamed_2,
    pub state: C2RustUnnamed_1,
    pub sel_options: libc::c_int,
    pub reachability: libc::c_int,
    pub latest_meas_ago: libc::c_ulong,
    pub orig_latest_meas: libc::c_double,
    pub latest_meas: libc::c_double,
    pub latest_meas_err: libc::c_double,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const RPT_OUTLIER: C2RustUnnamed_1 = 5;
pub const RPT_CANDIDATE: C2RustUnnamed_1 = 4;
pub const RPT_JITTERY: C2RustUnnamed_1 = 3;
pub const RPT_FALSETICKER: C2RustUnnamed_1 = 2;
pub const RPT_UNREACH: C2RustUnnamed_1 = 1;
pub const RPT_SYNC: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const RPT_LOCAL_REFERENCE: C2RustUnnamed_2 = 2;
pub const RPT_NTP_PEER: C2RustUnnamed_2 = 1;
pub const RPT_NTP_CLIENT: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_ActivityReport {
    pub online: libc::c_int,
    pub offline: libc::c_int,
    pub burst_online: libc::c_int,
    pub burst_offline: libc::c_int,
    pub unresolved: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_NTPReport {
    pub remote_addr: IPAddr,
    pub local_addr: IPAddr,
    pub remote_port: uint16_t,
    pub leap: uint8_t,
    pub version: uint8_t,
    pub mode: uint8_t,
    pub stratum: uint8_t,
    pub poll: int8_t,
    pub precision: int8_t,
    pub root_delay: libc::c_double,
    pub root_dispersion: libc::c_double,
    pub ref_id: uint32_t,
    pub ref_time: timespec,
    pub offset: libc::c_double,
    pub peer_delay: libc::c_double,
    pub peer_dispersion: libc::c_double,
    pub response_time: libc::c_double,
    pub jitter_asymmetry: libc::c_double,
    pub tests: uint16_t,
    pub interleaved: libc::c_int,
    pub authenticated: libc::c_int,
    pub tx_tss_char: libc::c_char,
    pub rx_tss_char: libc::c_char,
    pub total_tx_count: uint32_t,
    pub total_rx_count: uint32_t,
    pub total_valid_count: uint32_t,
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

  Header file defining parameters that can be set on a per source basis
  */
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
pub type NTP_Source_Type = libc::c_uint;
pub const NTP_PEER: NTP_Source_Type = 1;
pub const NTP_SERVER: NTP_Source_Type = 0;
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
pub type NCR_Instance = *mut NCR_Instance_Record;
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2002
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

  Header for the part of the software that deals with the set of
  current NTP servers and peers, which can resolve an IP address into
  a source record for further processing.

  */
/* Status values returned by operations that indirectly result from user
   input. */
pub type NSR_Status = libc::c_uint;
/* AddSourceByName - name will be resolved later */
/* AddSourceByName - attempt to add a source with invalid name */
pub const NSR_UnresolvedName: NSR_Status = 6;
/* AddSource - attempt to add a source with invalid address family */
pub const NSR_InvalidName: NSR_Status = 5;
/* AddSource - too many sources already present */
pub const NSR_InvalidAF: NSR_Status = 4;
/* AddSource - attempt to add a source that is already known */
pub const NSR_TooManySources: NSR_Status = 3;
/* Remove - attempt to remove a source that is not known */
pub const NSR_AlreadyInUse: NSR_Status = 2;
/* Operation successful */
pub const NSR_NoSuchSource: NSR_Status = 1;
pub const NSR_Success: NSR_Status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SourceRecord {
    pub remote_addr: *mut NTP_Remote_Address,
    pub data: NCR_Instance,
    pub name: *mut libc::c_char,
    pub pool: libc::c_int,
    pub tentative: libc::c_int,
}
/* Source with unknown address (which may be resolved later) */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnresolvedSource {
    pub name: *mut libc::c_char,
    pub port: libc::c_int,
    pub random_order: libc::c_int,
    pub replacement: libc::c_int,
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub next: *mut UnresolvedSource,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub new_source: C2RustUnnamed_4,
    pub replace_source: NTP_Remote_Address,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub type_0: NTP_Source_Type,
    pub params: SourceParameters,
    pub pool: libc::c_int,
    pub max_new_sources: libc::c_int,
}
pub type NSR_SourceResolvingEndHandler = Option<unsafe extern "C" fn() -> ()>;
/* Pool of sources with the same name */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SourcePool {
    pub sources: libc::c_int,
    pub max_sources: libc::c_int,
}
pub type DNS_Status = libc::c_uint;
pub const DNS_Failure: DNS_Status = 2;
pub const DNS_TryAgain: DNS_Status = 1;
pub const DNS_Success: DNS_Status = 0;
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

  Header for asynchronous nameserver functions
  */
/* Function type for callback to process the result */
pub type DNS_NameResolveHandler
    =
    Option<unsafe extern "C" fn(_: DNS_Status, _: libc::c_int, _: *mut IPAddr,
                                _: *mut libc::c_void) -> ()>;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
pub type LCL_ChangeType = libc::c_uint;
pub const LCL_ChangeUnknownStep: LCL_ChangeType = 2;
pub const LCL_ChangeStep: LCL_ChangeType = 1;
pub const LCL_ChangeAdjust: LCL_ChangeType = 0;
pub type LCL_ParameterChangeHandler
    =
    Option<unsafe extern "C" fn(_: *mut timespec, _: *mut timespec,
                                _: libc::c_double, _: libc::c_double,
                                _: LCL_ChangeType, _: *mut libc::c_void)
               -> ()>;
/* Hash table of SourceRecord, its size is a power of two and it's never
   more than half full */
static mut records: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
/* Number of sources in the hash table */
static mut n_sources: libc::c_int = 0;
/* Flag indicating new sources will be started automatically when added */
static mut auto_start_sources: libc::c_int = 0 as libc::c_int;
static mut unresolved_sources: *mut UnresolvedSource =
    0 as *const UnresolvedSource as *mut UnresolvedSource;
static mut resolving_interval: libc::c_int = 0 as libc::c_int;
static mut resolving_id: SCH_TimeoutID = 0;
static mut resolving_source: *mut UnresolvedSource =
    0 as *const UnresolvedSource as *mut UnresolvedSource;
static mut resolving_end_handler: NSR_SourceResolvingEndHandler = None;
/* Array of SourcePool */
static mut pools: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
/* ================================================== */
/* Flag indicating whether module is initialised */
static mut initialised: libc::c_int = 0 as libc::c_int;
/* ================================================== */
unsafe extern "C" fn get_record(mut index: libc::c_uint)
 -> *mut SourceRecord {
    return ARR_GetElement(records, index) as *mut SourceRecord;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_Initialise() {
    n_sources = 0 as libc::c_int;
    initialised = 1 as libc::c_int;
    records =
        ARR_CreateInstance(::std::mem::size_of::<SourceRecord>() as
                               libc::c_ulong as libc::c_uint);
    rehash_records();
    pools =
        ARR_CreateInstance(::std::mem::size_of::<SourcePool>() as
                               libc::c_ulong as libc::c_uint);
    LCL_AddParameterChangeHandler(Some(slew_sources as
                                           unsafe extern "C" fn(_:
                                                                    *mut timespec,
                                                                _:
                                                                    *mut timespec,
                                                                _:
                                                                    libc::c_double,
                                                                _:
                                                                    libc::c_double,
                                                                _:
                                                                    LCL_ChangeType,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> ()),
                                  0 as *mut libc::c_void);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_Finalise() {
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut us: *mut UnresolvedSource = 0 as *mut UnresolvedSource;
    let mut i: libc::c_uint = 0;
    ARR_DestroyInstance(pools);
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(records) {
        record = get_record(i);
        if !(*record).remote_addr.is_null() { clean_source_record(record); }
        i = i.wrapping_add(1)
    }
    ARR_DestroyInstance(records);
    while !unresolved_sources.is_null() {
        us = unresolved_sources;
        unresolved_sources = (*us).next;
        free((*us).name as *mut libc::c_void);
        free(us as *mut libc::c_void);
    }
    initialised = 0 as libc::c_int;
}
/* ================================================== */
/* Return slot number and whether the IP address was matched or not.
   found = 0 => Neither IP nor port matched, empty slot returned
   found = 1 => Only IP matched, port doesn't match
   found = 2 => Both IP and port matched.

   It is assumed that there can only ever be one record for a
   particular IP address.  (If a different port comes up, it probably
   means someone is running ntpdate -d or something).  Thus, if we
   match the IP address we stop the search regardless of whether the
   port number matches.

  */
unsafe extern "C" fn find_slot(mut remote_addr: *mut NTP_Remote_Address,
                               mut slot: *mut libc::c_int,
                               mut found: *mut libc::c_int) {
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut hash: uint32_t = 0;
    let mut i: libc::c_uint = 0;
    let mut size: libc::c_uint = 0;
    let mut port: libc::c_ushort = 0;
    size = ARR_GetSize(records);
    *slot = 0 as libc::c_int;
    *found = 0 as libc::c_int;
    if (*remote_addr).ip_addr.family as libc::c_int != 1 as libc::c_int &&
           (*remote_addr).ip_addr.family as libc::c_int != 2 as libc::c_int {
        return
    }
    hash = UTI_IPToHash(&mut (*remote_addr).ip_addr);
    port = (*remote_addr).port;
    i = 0 as libc::c_int as libc::c_uint;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_uint) {
        /* Use quadratic probing */
        *slot =
            hash.wrapping_add(i.wrapping_add(i.wrapping_mul(i)).wrapping_div(2
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)).wrapping_rem(size)
                as libc::c_int;
        record = get_record(*slot as libc::c_uint);
        if (*record).remote_addr.is_null() { break ; }
        if UTI_CompareIPs(&mut (*(*record).remote_addr).ip_addr,
                          &mut (*remote_addr).ip_addr, 0 as *mut IPAddr) == 0
           {
            *found =
                if (*(*record).remote_addr).port as libc::c_int ==
                       port as libc::c_int {
                    2 as libc::c_int
                } else { 1 as libc::c_int };
            return
        }
        i = i.wrapping_add(1)
    };
}
/* ================================================== */
/* Check if hash table of given size is sufficient to contain sources */
unsafe extern "C" fn check_hashtable_size(mut sources: libc::c_uint,
                                          mut size: libc::c_uint)
 -> libc::c_int {
    return (sources.wrapping_mul(2 as libc::c_int as libc::c_uint) <= size) as
               libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn rehash_records() {
    let mut temp_records: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut i: libc::c_uint = 0;
    let mut old_size: libc::c_uint = 0;
    let mut new_size: libc::c_uint = 0;
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    old_size = ARR_GetSize(records);
    temp_records =
        Malloc2(old_size as size_t,
                ::std::mem::size_of::<SourceRecord>() as libc::c_ulong) as
            *mut SourceRecord;
    memcpy(temp_records as *mut libc::c_void, ARR_GetElements(records),
           (old_size as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<SourceRecord>()
                                                as libc::c_ulong));
    /* The size of the hash table is always a power of two */
    new_size = 1 as libc::c_int as libc::c_uint;
    while check_hashtable_size(n_sources as libc::c_uint, new_size) == 0 {
        new_size = new_size.wrapping_mul(2 as libc::c_int as libc::c_uint)
    }
    ARR_SetSize(records, new_size);
    i = 0 as libc::c_int as libc::c_uint;
    while i < new_size {
        let ref mut fresh0 = (*get_record(i)).remote_addr;
        *fresh0 = 0 as *mut NTP_Remote_Address;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < old_size {
        if !(*temp_records.offset(i as isize)).remote_addr.is_null() {
            find_slot((*temp_records.offset(i as isize)).remote_addr,
                      &mut slot, &mut found);
            if found == 0 {
            } else {
                __assert_fail(b"!found\x00" as *const u8 as
                                  *const libc::c_char,
                              b"ntp_sources.c\x00" as *const u8 as
                                  *const libc::c_char,
                              273 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 26],
                                                        &[libc::c_char; 26]>(b"void rehash_records(void)\x00")).as_ptr());
            }
            *get_record(slot as libc::c_uint) =
                *temp_records.offset(i as isize)
        }
        i = i.wrapping_add(1)
    }
    free(temp_records as *mut libc::c_void);
}
/* ================================================== */
/* Procedure to add a new source */
unsafe extern "C" fn add_source(mut remote_addr: *mut NTP_Remote_Address,
                                mut name: *mut libc::c_char,
                                mut type_0: NTP_Source_Type,
                                mut params: *mut SourceParameters,
                                mut pool: libc::c_int) -> NSR_Status {
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    if initialised != 0 {
    } else {
        __assert_fail(b"initialised\x00" as *const u8 as *const libc::c_char,
                      b"ntp_sources.c\x00" as *const u8 as
                          *const libc::c_char,
                      290 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 94],
                                                &[libc::c_char; 94]>(b"NSR_Status add_source(NTP_Remote_Address *, char *, NTP_Source_Type, SourceParameters *, int)\x00")).as_ptr());
    }
    /* Find empty bin & check that we don't have the address already */
    find_slot(remote_addr, &mut slot, &mut found);
    if found != 0 {
        return NSR_AlreadyInUse
    } else if (*remote_addr).ip_addr.family as libc::c_int != 1 as libc::c_int
                  &&
                  (*remote_addr).ip_addr.family as libc::c_int !=
                      2 as libc::c_int {
        return NSR_InvalidAF
    } else {
        n_sources += 1;
        if check_hashtable_size(n_sources as libc::c_uint,
                                ARR_GetSize(records)) == 0 {
            rehash_records();
            find_slot(remote_addr, &mut slot, &mut found);
        }
        if found == 0 {
        } else {
            __assert_fail(b"!found\x00" as *const u8 as *const libc::c_char,
                          b"ntp_sources.c\x00" as *const u8 as
                              *const libc::c_char,
                          308 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 94],
                                                    &[libc::c_char; 94]>(b"NSR_Status add_source(NTP_Remote_Address *, char *, NTP_Source_Type, SourceParameters *, int)\x00")).as_ptr());
        }
        record = get_record(slot as libc::c_uint);
        (*record).data = NCR_GetInstance(remote_addr, type_0, params);
        (*record).remote_addr = NCR_GetRemoteAddress((*record).data);
        (*record).name =
            if !name.is_null() {
                Strdup(name)
            } else { 0 as *mut libc::c_char };
        (*record).pool = pool;
        (*record).tentative = 1 as libc::c_int;
        if auto_start_sources != 0 { NCR_StartInstance((*record).data); }
        return NSR_Success
    };
}
/* ================================================== */
unsafe extern "C" fn replace_source(mut old_addr: *mut NTP_Remote_Address,
                                    mut new_addr: *mut NTP_Remote_Address)
 -> NSR_Status {
    let mut slot1: libc::c_int = 0;
    let mut slot2: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut pool: *mut SourcePool = 0 as *mut SourcePool;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    find_slot(old_addr, &mut slot1, &mut found);
    if found == 0 { return NSR_NoSuchSource }
    find_slot(new_addr, &mut slot2, &mut found);
    if found != 0 { return NSR_AlreadyInUse }
    record = get_record(slot1 as libc::c_uint);
    NCR_ChangeRemoteAddress((*record).data, new_addr);
    (*record).remote_addr = NCR_GetRemoteAddress((*record).data);
    if (*record).tentative == 0 {
        (*record).tentative = 1 as libc::c_int;
        if (*record).pool != -(1 as libc::c_int) {
            pool =
                ARR_GetElement(pools, (*record).pool as libc::c_uint) as
                    *mut SourcePool;
            (*pool).sources -= 1
        }
    }
    name = (*record).name;
    /* The hash table must be rebuilt for the new address */
    rehash_records();
    LOG_Message(LOGS_INFO,
                b"Source %s replaced with %s (%s)\x00" as *const u8 as
                    *const libc::c_char,
                UTI_IPToString(&mut (*old_addr).ip_addr),
                UTI_IPToString(&mut (*new_addr).ip_addr),
                if !name.is_null() {
                    name
                } else { b"\x00" as *const u8 as *const libc::c_char });
    return NSR_Success;
}
/* ================================================== */
unsafe extern "C" fn process_resolved_name(mut us: *mut UnresolvedSource,
                                           mut ip_addrs: *mut IPAddr,
                                           mut n_addrs: libc::c_int) {
    let mut address: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    let mut i: libc::c_int = 0;
    let mut added: libc::c_int = 0;
    let mut first: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    if (*us).random_order != 0 {
        UTI_GetRandomBytes(&mut first as *mut libc::c_ushort as
                               *mut libc::c_void,
                           ::std::mem::size_of::<libc::c_ushort>() as
                               libc::c_ulong as libc::c_uint);
    }
    added = 0 as libc::c_int;
    i = added;
    while i < n_addrs {
        address.ip_addr =
            *ip_addrs.offset((i as
                                  libc::c_uint).wrapping_add(first as
                                                                 libc::c_uint).wrapping_rem(n_addrs
                                                                                                as
                                                                                                libc::c_uint)
                                 as isize);
        address.port = (*us).port as uint16_t;
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"(%d) %s\x00" as *const u8 as *const libc::c_char,
                        i + 1 as libc::c_int,
                        UTI_IPToString(&mut address.ip_addr));
        }
        if (*us).replacement != 0 {
            if replace_source(&mut (*us).c2rust_unnamed.replace_source,
                              &mut address) as libc::c_uint !=
                   NSR_AlreadyInUse as libc::c_int as libc::c_uint {
                break ;
            }
        } else {
            if add_source(&mut address, (*us).name,
                          (*us).c2rust_unnamed.new_source.type_0,
                          &mut (*us).c2rust_unnamed.new_source.params,
                          (*us).c2rust_unnamed.new_source.pool) as
                   libc::c_uint == NSR_Success as libc::c_int as libc::c_uint
               {
                added += 1
            }
            if added >= (*us).c2rust_unnamed.new_source.max_new_sources {
                break ;
            }
        }
        i += 1
    };
}
/* ================================================== */
unsafe extern "C" fn name_resolve_handler(mut status: DNS_Status,
                                          mut n_addrs: libc::c_int,
                                          mut ip_addrs: *mut IPAddr,
                                          mut anything: *mut libc::c_void) {
    let mut us: *mut UnresolvedSource = 0 as *mut UnresolvedSource;
    let mut i: *mut *mut UnresolvedSource = 0 as *mut *mut UnresolvedSource;
    let mut next: *mut UnresolvedSource = 0 as *mut UnresolvedSource;
    us = anything as *mut UnresolvedSource;
    if us == resolving_source {
    } else {
        __assert_fail(b"us == resolving_source\x00" as *const u8 as
                          *const libc::c_char,
                      b"ntp_sources.c\x00" as *const u8 as
                          *const libc::c_char,
                      407 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 61],
                                                &[libc::c_char; 61]>(b"void name_resolve_handler(DNS_Status, int, IPAddr *, void *)\x00")).as_ptr());
    }
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"%s resolved to %d addrs\x00" as *const u8 as
                        *const libc::c_char, (*us).name, n_addrs);
    }
    match status as libc::c_uint {
        1 => { }
        0 => { process_resolved_name(us, ip_addrs, n_addrs); }
        2 => {
            LOG_Message(LOGS_WARN,
                        b"Invalid host %s\x00" as *const u8 as
                            *const libc::c_char, (*us).name);
        }
        _ => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"ntp_sources.c\x00" as *const u8 as
                              *const libc::c_char,
                          421 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 61],
                                                    &[libc::c_char; 61]>(b"void name_resolve_handler(DNS_Status, int, IPAddr *, void *)\x00")).as_ptr());
        }
    }
    next = (*us).next;
    /* Remove the source from the list on success or failure, replacements
     are removed on any status */
    if (*us).replacement != 0 ||
           status as libc::c_uint !=
               DNS_TryAgain as libc::c_int as libc::c_uint {
        i = &mut unresolved_sources;
        while !(*i).is_null() {
            if *i == us {
                *i = (*us).next;
                free((*us).name as *mut libc::c_void);
                free(us as *mut libc::c_void);
                break ;
            } else { i = &mut (**i).next }
        }
    }
    resolving_source = next;
    if !next.is_null() {
        /* Continue with the next source in the list */
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"resolving %s\x00" as *const u8 as
                            *const libc::c_char, (*next).name);
        }
        DNS_Name2IPAddressAsync((*next).name,
                                Some(name_resolve_handler as
                                         unsafe extern "C" fn(_: DNS_Status,
                                                              _: libc::c_int,
                                                              _: *mut IPAddr,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> ()),
                                next as *mut libc::c_void);
    } else {
        /* This was the last source in the list. If some sources couldn't
       be resolved, try again in exponentially increasing interval. */
        if !unresolved_sources.is_null() {
            if resolving_interval < 2 as libc::c_int {
                resolving_interval = 2 as libc::c_int
            } else if resolving_interval < 9 as libc::c_int {
                resolving_interval += 1
            }
            resolving_id =
                SCH_AddTimeoutByDelay((7 as libc::c_int *
                                           ((1 as libc::c_int) <<
                                                resolving_interval)) as
                                          libc::c_double,
                                      Some(resolve_sources as
                                               unsafe extern "C" fn(_:
                                                                        *mut libc::c_void)
                                                   -> ()),
                                      0 as *mut libc::c_void)
        } else { resolving_interval = 0 as libc::c_int }
        /* This round of resolving is done */
        if resolving_end_handler.is_some() {
            resolving_end_handler.expect("non-null function pointer")();
        }
    };
}
/* ================================================== */
/* Forward prototypes */
/* ================================================== */
unsafe extern "C" fn resolve_sources(mut arg: *mut libc::c_void) {
    let mut us: *mut UnresolvedSource = 0 as *mut UnresolvedSource;
    if resolving_source.is_null() {
    } else {
        __assert_fail(b"!resolving_source\x00" as *const u8 as
                          *const libc::c_char,
                      b"ntp_sources.c\x00" as *const u8 as
                          *const libc::c_char,
                      472 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"void resolve_sources(void *)\x00")).as_ptr());
    }
    DNS_Reload();
    /* Start with the first source in the list, name_resolve_handler
     will iterate over the rest */
    us = unresolved_sources;
    resolving_source = us;
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"resolving %s\x00" as *const u8 as *const libc::c_char,
                    (*us).name);
    }
    DNS_Name2IPAddressAsync((*us).name,
                            Some(name_resolve_handler as
                                     unsafe extern "C" fn(_: DNS_Status,
                                                          _: libc::c_int,
                                                          _: *mut IPAddr,
                                                          _:
                                                              *mut libc::c_void)
                                         -> ()), us as *mut libc::c_void);
}
/* ================================================== */
unsafe extern "C" fn append_unresolved_source(mut us: *mut UnresolvedSource) {
    let mut i: *mut *mut UnresolvedSource = 0 as *mut *mut UnresolvedSource;
    i = &mut unresolved_sources;
    while !(*i).is_null() { i = &mut (**i).next }
    *i = us;
    (*us).next = 0 as *mut UnresolvedSource;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_AddSource(mut remote_addr:
                                           *mut NTP_Remote_Address,
                                       mut type_0: NTP_Source_Type,
                                       mut params: *mut SourceParameters)
 -> NSR_Status {
    return add_source(remote_addr, 0 as *mut libc::c_char, type_0, params,
                      -(1 as libc::c_int));
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_AddSourceByName(mut name: *mut libc::c_char,
                                             mut port: libc::c_int,
                                             mut pool: libc::c_int,
                                             mut type_0: NTP_Source_Type,
                                             mut params:
                                                 *mut SourceParameters)
 -> NSR_Status {
    let mut us: *mut UnresolvedSource = 0 as *mut UnresolvedSource;
    let mut sp: *mut SourcePool = 0 as *mut SourcePool;
    let mut remote_addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    let mut i: libc::c_int = 0;
    /* If the name is an IP address, don't bother with full resolving now
     or later when trying to replace the source */
    if UTI_StringToIP(name, &mut remote_addr.ip_addr) != 0 {
        remote_addr.port = port as uint16_t;
        return NSR_AddSource(&mut remote_addr, type_0, params)
    }
    /* Make sure the name is at least printable and has no spaces */
    i = 0 as libc::c_int;
    while *name.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
        if *(*__ctype_b_loc()).offset(*name.offset(i as isize) as libc::c_int
                                          as isize) as libc::c_int &
               _ISgraph as libc::c_int as libc::c_ushort as libc::c_int == 0 {
            return NSR_InvalidName
        }
        i += 1
    }
    us =
        Malloc(::std::mem::size_of::<UnresolvedSource>() as libc::c_ulong) as
            *mut UnresolvedSource;
    (*us).name = Strdup(name);
    (*us).port = port;
    (*us).random_order = 0 as libc::c_int;
    (*us).replacement = 0 as libc::c_int;
    (*us).c2rust_unnamed.new_source.type_0 = type_0;
    (*us).c2rust_unnamed.new_source.params = *params;
    if pool == 0 {
        (*us).c2rust_unnamed.new_source.pool = -(1 as libc::c_int);
        (*us).c2rust_unnamed.new_source.max_new_sources = 1 as libc::c_int
    } else {
        sp = ARR_GetNewElement(pools) as *mut SourcePool;
        (*sp).sources = 0 as libc::c_int;
        (*sp).max_sources = (*params).max_sources;
        (*us).c2rust_unnamed.new_source.pool =
            ARR_GetSize(pools).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as libc::c_int;
        (*us).c2rust_unnamed.new_source.max_new_sources = 16 as libc::c_int
    }
    append_unresolved_source(us);
    return NSR_UnresolvedName;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_SetSourceResolvingEndHandler(mut handler:
                                                              NSR_SourceResolvingEndHandler) {
    resolving_end_handler = handler;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_ResolveSources() {
    /* Try to resolve unresolved sources now */
    if !unresolved_sources.is_null() {
        /* Make sure no resolving is currently running */
        if resolving_source.is_null() {
            if resolving_interval != 0 {
                SCH_RemoveTimeout(resolving_id);
                resolving_interval -= 1
            }
            resolve_sources(0 as *mut libc::c_void);
        }
    } else if resolving_end_handler.is_some() {
        resolving_end_handler.expect("non-null function pointer")();
    };
}
/* No unresolved sources, we are done */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_StartSources() {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(records) {
        if !(*get_record(i)).remote_addr.is_null() {
            NCR_StartInstance((*get_record(i)).data);
        }
        i = i.wrapping_add(1)
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_AutoStartSources() {
    auto_start_sources = 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn clean_source_record(mut record: *mut SourceRecord) {
    if !(*record).remote_addr.is_null() {
    } else {
        __assert_fail(b"record->remote_addr\x00" as *const u8 as
                          *const libc::c_char,
                      b"ntp_sources.c\x00" as *const u8 as
                          *const libc::c_char,
                      608 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"void clean_source_record(SourceRecord *)\x00")).as_ptr());
    }
    (*record).remote_addr = 0 as *mut NTP_Remote_Address;
    NCR_DestroyInstance((*record).data);
    if !(*record).name.is_null() {
        free((*record).name as *mut libc::c_void);
    }
    n_sources -= 1;
}
/* ================================================== */
/* Procedure to remove a source.  We don't bother whether the port
   address is matched - we're only interested in removing a record for
   the right IP address.  Thus the caller can specify the port number
   as zero if it wishes. */
#[no_mangle]
pub unsafe extern "C" fn NSR_RemoveSource(mut remote_addr:
                                              *mut NTP_Remote_Address)
 -> NSR_Status {
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    if initialised != 0 {
    } else {
        __assert_fail(b"initialised\x00" as *const u8 as *const libc::c_char,
                      b"ntp_sources.c\x00" as *const u8 as
                          *const libc::c_char,
                      628 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"NSR_Status NSR_RemoveSource(NTP_Remote_Address *)\x00")).as_ptr());
    }
    find_slot(remote_addr, &mut slot, &mut found);
    if found == 0 { return NSR_NoSuchSource }
    clean_source_record(get_record(slot as libc::c_uint));
    /* Rehash the table to make sure there are no broken probe sequences.
     This is costly, but it's not expected to happen frequently. */
    rehash_records();
    return NSR_Success;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_RemoveAllSources() {
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(records) {
        record = get_record(i);
        if !(*record).remote_addr.is_null() { clean_source_record(record); }
        i = i.wrapping_add(1)
    }
    rehash_records();
}
/* ================================================== */
unsafe extern "C" fn resolve_source_replacement(mut record:
                                                    *mut SourceRecord) {
    let mut us: *mut UnresolvedSource = 0 as *mut UnresolvedSource;
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"trying to replace %s\x00" as *const u8 as
                        *const libc::c_char,
                    UTI_IPToString(&mut (*(*record).remote_addr).ip_addr));
    }
    us =
        Malloc(::std::mem::size_of::<UnresolvedSource>() as libc::c_ulong) as
            *mut UnresolvedSource;
    (*us).name = Strdup((*record).name);
    (*us).port = (*(*record).remote_addr).port as libc::c_int;
    /* If there never was a valid reply from this source (e.g. it was a bad
     replacement), ignore the order of addresses from the resolver to not get
     stuck to a pair of addresses if the order doesn't change, or a group of
     IPv4/IPv6 addresses if the resolver prefers inaccessible IP family */
    (*us).random_order = (*record).tentative;
    (*us).replacement = 1 as libc::c_int;
    (*us).c2rust_unnamed.replace_source = *(*record).remote_addr;
    append_unresolved_source(us);
    NSR_ResolveSources();
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_HandleBadSource(mut address: *mut IPAddr) {
    static mut last_replacement: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut remote_addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut diff: libc::c_double = 0.;
    remote_addr.ip_addr = *address;
    remote_addr.port = 0 as libc::c_int as uint16_t;
    find_slot(&mut remote_addr, &mut slot, &mut found);
    if found == 0 { return }
    record = get_record(slot as libc::c_uint);
    /* Only sources with a name can be replaced */
    if (*record).name.is_null() { return }
    /* Don't resolve names too frequently */
    SCH_GetLastEventTime(0 as *mut timespec, 0 as *mut libc::c_double,
                         &mut now);
    diff = UTI_DiffTimespecsToDouble(&mut now, &mut last_replacement);
    if fabs(diff) <
           (7 as libc::c_int * ((1 as libc::c_int) << 8 as libc::c_int)) as
               libc::c_double {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"replacement postponed\x00" as *const u8 as
                            *const libc::c_char);
        }
        return
    }
    last_replacement = now;
    resolve_source_replacement(record);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_RefreshAddresses() {
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(records) {
        record = get_record(i);
        if !((*record).remote_addr.is_null() || (*record).name.is_null()) {
            resolve_source_replacement(record);
        }
        i = i.wrapping_add(1)
    };
}
/* ================================================== */
unsafe extern "C" fn remove_tentative_pool_sources(mut pool: libc::c_int) {
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut i: libc::c_uint = 0;
    let mut removed: libc::c_uint = 0;
    removed = 0 as libc::c_int as libc::c_uint;
    i = removed;
    while i < ARR_GetSize(records) {
        record = get_record(i);
        if !((*record).remote_addr.is_null() || (*record).pool != pool ||
                 (*record).tentative == 0) {
            if 0 as libc::c_int != 0 &&
                   log_min_severity as libc::c_int ==
                       LOGS_DEBUG as libc::c_int {
                LOG_Message(LOGS_DEBUG,
                            b"removing tentative source %s\x00" as *const u8
                                as *const libc::c_char,
                            UTI_IPToString(&mut (*(*record).remote_addr).ip_addr));
            }
            clean_source_record(record);
            removed = removed.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    if removed != 0 { rehash_records(); };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_GetLocalRefid(mut address: *mut IPAddr)
 -> uint32_t {
    let mut remote_addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    remote_addr.ip_addr = *address;
    remote_addr.port = 0 as libc::c_int as uint16_t;
    find_slot(&mut remote_addr, &mut slot, &mut found);
    if found == 0 { return 0 as libc::c_int as uint32_t }
    return NCR_GetLocalRefid((*get_record(slot as libc::c_uint)).data);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_GetName(mut address: *mut IPAddr)
 -> *mut libc::c_char {
    let mut remote_addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    remote_addr.ip_addr = *address;
    remote_addr.port = 0 as libc::c_int as uint16_t;
    find_slot(&mut remote_addr, &mut slot, &mut found);
    if found == 0 { return 0 as *mut libc::c_char }
    record = get_record(slot as libc::c_uint);
    if !(*record).name.is_null() { return (*record).name }
    return UTI_IPToString(&mut (*(*record).remote_addr).ip_addr);
}
/* ================================================== */
/* This routine is called by ntp_io when a new packet arrives off the network,
   possibly with an authentication tail */
#[no_mangle]
pub unsafe extern "C" fn NSR_ProcessRx(mut remote_addr:
                                           *mut NTP_Remote_Address,
                                       mut local_addr: *mut NTP_Local_Address,
                                       mut rx_ts: *mut NTP_Local_Timestamp,
                                       mut message: *mut NTP_Packet,
                                       mut length: libc::c_int) {
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut pool: *mut SourcePool = 0 as *mut SourcePool;
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    if initialised != 0 {
    } else {
        __assert_fail(b"initialised\x00" as *const u8 as *const libc::c_char,
                      b"ntp_sources.c\x00" as *const u8 as
                          *const libc::c_char,
                      818 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 104],
                                                &[libc::c_char; 104]>(b"void NSR_ProcessRx(NTP_Remote_Address *, NTP_Local_Address *, NTP_Local_Timestamp *, NTP_Packet *, int)\x00")).as_ptr());
    }
    find_slot(remote_addr, &mut slot, &mut found);
    if found == 2 as libc::c_int {
        /* Must match IP address AND port number */
        record = get_record(slot as libc::c_uint);
        if NCR_ProcessRxKnown((*record).data, local_addr, rx_ts, message,
                              length) == 0 {
            return
        }
        if (*record).tentative != 0 {
            /* This was the first good reply from the source */
            (*record).tentative = 0 as libc::c_int;
            if (*record).pool != -(1 as libc::c_int) {
                pool =
                    ARR_GetElement(pools, (*record).pool as libc::c_uint) as
                        *mut SourcePool;
                (*pool).sources += 1;
                if 0 as libc::c_int != 0 &&
                       log_min_severity as libc::c_int ==
                           LOGS_DEBUG as libc::c_int {
                    LOG_Message(LOGS_DEBUG,
                                b"pool %s has %d confirmed sources\x00" as
                                    *const u8 as *const libc::c_char,
                                (*record).name, (*pool).sources);
                }
                /* If the number of sources from the pool reached the configured
           maximum, remove the remaining tentative sources */
                if (*pool).sources >= (*pool).max_sources {
                    remove_tentative_pool_sources((*record).pool);
                }
            }
        }
    } else {
        NCR_ProcessRxUnknown(remote_addr, local_addr, rx_ts, message, length);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_ProcessTx(mut remote_addr:
                                           *mut NTP_Remote_Address,
                                       mut local_addr: *mut NTP_Local_Address,
                                       mut tx_ts: *mut NTP_Local_Timestamp,
                                       mut message: *mut NTP_Packet,
                                       mut length: libc::c_int) {
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    find_slot(remote_addr, &mut slot, &mut found);
    if found == 2 as libc::c_int {
        /* Must match IP address AND port number */
        record = get_record(slot as libc::c_uint);
        NCR_ProcessTxKnown((*record).data, local_addr, tx_ts, message,
                           length);
    } else {
        NCR_ProcessTxUnknown(remote_addr, local_addr, tx_ts, message, length);
    };
}
/* ================================================== */
unsafe extern "C" fn slew_sources(mut raw: *mut timespec,
                                  mut cooked: *mut timespec,
                                  mut dfreq: libc::c_double,
                                  mut doffset: libc::c_double,
                                  mut change_type: LCL_ChangeType,
                                  mut anything: *mut libc::c_void) {
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(records) {
        record = get_record(i);
        if !(*record).remote_addr.is_null() {
            if change_type as libc::c_uint ==
                   LCL_ChangeUnknownStep as libc::c_int as libc::c_uint {
                NCR_ResetInstance((*record).data);
                NCR_ResetPoll((*record).data);
            } else { NCR_SlewTimes((*record).data, cooked, dfreq, doffset); }
        }
        i = i.wrapping_add(1)
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_SetConnectivity(mut mask: *mut IPAddr,
                                             mut address: *mut IPAddr,
                                             mut connectivity:
                                                 SRC_Connectivity)
 -> libc::c_int {
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut syncpeer: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut i: libc::c_uint = 0;
    let mut any: libc::c_uint = 0;
    if connectivity as libc::c_uint !=
           SRC_OFFLINE as libc::c_int as libc::c_uint {
        NSR_ResolveSources();
    }
    any = 0 as libc::c_int as libc::c_uint;
    syncpeer = 0 as *mut SourceRecord;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(records) {
        record = get_record(i);
        if !(*record).remote_addr.is_null() {
            if (*address).family as libc::c_int == 0 as libc::c_int ||
                   UTI_CompareIPs(&mut (*(*record).remote_addr).ip_addr,
                                  address, mask) == 0 {
                any = 1 as libc::c_int as libc::c_uint;
                if NCR_IsSyncPeer((*record).data) != 0 {
                    syncpeer = record
                } else { NCR_SetConnectivity((*record).data, connectivity); }
            }
        }
        i = i.wrapping_add(1)
    }
    /* Set the sync peer last to avoid unnecessary reference switching */
    if !syncpeer.is_null() {
        NCR_SetConnectivity((*syncpeer).data, connectivity);
    }
    if (*address).family as libc::c_int == 0 as libc::c_int {
        let mut us: *mut UnresolvedSource = 0 as *mut UnresolvedSource;
        us = unresolved_sources;
        while !us.is_null() {
            if !((*us).replacement != 0) {
                any = 1 as libc::c_int as libc::c_uint;
                (*us).c2rust_unnamed.new_source.params.connectivity =
                    connectivity
            }
            us = (*us).next
        }
    }
    return any as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_ModifyMinpoll(mut address: *mut IPAddr,
                                           mut new_minpoll: libc::c_int)
 -> libc::c_int {
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    addr.ip_addr = *address;
    addr.port = 0 as libc::c_int as uint16_t;
    find_slot(&mut addr, &mut slot, &mut found);
    if found == 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        NCR_ModifyMinpoll((*get_record(slot as libc::c_uint)).data,
                          new_minpoll);
        return 1 as libc::c_int
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_ModifyMaxpoll(mut address: *mut IPAddr,
                                           mut new_maxpoll: libc::c_int)
 -> libc::c_int {
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    addr.ip_addr = *address;
    addr.port = 0 as libc::c_int as uint16_t;
    find_slot(&mut addr, &mut slot, &mut found);
    if found == 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        NCR_ModifyMaxpoll((*get_record(slot as libc::c_uint)).data,
                          new_maxpoll);
        return 1 as libc::c_int
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_ModifyMaxdelay(mut address: *mut IPAddr,
                                            mut new_max_delay: libc::c_double)
 -> libc::c_int {
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    addr.ip_addr = *address;
    addr.port = 0 as libc::c_int as uint16_t;
    find_slot(&mut addr, &mut slot, &mut found);
    if found == 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        NCR_ModifyMaxdelay((*get_record(slot as libc::c_uint)).data,
                           new_max_delay);
        return 1 as libc::c_int
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_ModifyMaxdelayratio(mut address: *mut IPAddr,
                                                 mut new_max_delay_ratio:
                                                     libc::c_double)
 -> libc::c_int {
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    addr.ip_addr = *address;
    addr.port = 0 as libc::c_int as uint16_t;
    find_slot(&mut addr, &mut slot, &mut found);
    if found == 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        NCR_ModifyMaxdelayratio((*get_record(slot as libc::c_uint)).data,
                                new_max_delay_ratio);
        return 1 as libc::c_int
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_ModifyMaxdelaydevratio(mut address: *mut IPAddr,
                                                    mut new_max_delay_dev_ratio:
                                                        libc::c_double)
 -> libc::c_int {
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    addr.ip_addr = *address;
    addr.port = 0 as libc::c_int as uint16_t;
    find_slot(&mut addr, &mut slot, &mut found);
    if found == 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        NCR_ModifyMaxdelaydevratio((*get_record(slot as libc::c_uint)).data,
                                   new_max_delay_dev_ratio);
        return 1 as libc::c_int
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_ModifyMinstratum(mut address: *mut IPAddr,
                                              mut new_min_stratum:
                                                  libc::c_int)
 -> libc::c_int {
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    addr.ip_addr = *address;
    addr.port = 0 as libc::c_int as uint16_t;
    find_slot(&mut addr, &mut slot, &mut found);
    if found == 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        NCR_ModifyMinstratum((*get_record(slot as libc::c_uint)).data,
                             new_min_stratum);
        return 1 as libc::c_int
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_ModifyPolltarget(mut address: *mut IPAddr,
                                              mut new_poll_target:
                                                  libc::c_int)
 -> libc::c_int {
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    addr.ip_addr = *address;
    addr.port = 0 as libc::c_int as uint16_t;
    find_slot(&mut addr, &mut slot, &mut found);
    if found == 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        NCR_ModifyPolltarget((*get_record(slot as libc::c_uint)).data,
                             new_poll_target);
        return 1 as libc::c_int
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_InitiateSampleBurst(mut n_good_samples:
                                                     libc::c_int,
                                                 mut n_total_samples:
                                                     libc::c_int,
                                                 mut mask: *mut IPAddr,
                                                 mut address: *mut IPAddr)
 -> libc::c_int {
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut i: libc::c_uint = 0;
    let mut any: libc::c_int = 0;
    any = 0 as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(records) {
        record = get_record(i);
        if !(*record).remote_addr.is_null() {
            if (*address).family as libc::c_int == 0 as libc::c_int ||
                   UTI_CompareIPs(&mut (*(*record).remote_addr).ip_addr,
                                  address, mask) == 0 {
                any = 1 as libc::c_int;
                NCR_InitiateSampleBurst((*record).data, n_good_samples,
                                        n_total_samples);
            }
        }
        i = i.wrapping_add(1)
    }
    return any;
}
/* ================================================== */
/* The ip address is assumed to be completed on input, that is how we
   identify the source record. */
#[no_mangle]
pub unsafe extern "C" fn NSR_ReportSource(mut report: *mut RPT_SourceReport,
                                          mut now: *mut timespec) {
    let mut rem_addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    rem_addr.ip_addr = (*report).ip_addr;
    rem_addr.port = 0 as libc::c_int as uint16_t;
    find_slot(&mut rem_addr, &mut slot, &mut found);
    if found != 0 {
        NCR_ReportSource((*get_record(slot as libc::c_uint)).data, report,
                         now);
    } else {
        (*report).poll = 0 as libc::c_int;
        (*report).latest_meas_ago = 0 as libc::c_int as libc::c_ulong
    };
}
/* ================================================== */
/* The ip address is assumed to be completed on input, that is how we
   identify the source record. */
#[no_mangle]
pub unsafe extern "C" fn NSR_GetNTPReport(mut report: *mut RPT_NTPReport)
 -> libc::c_int {
    let mut rem_addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    let mut slot: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    rem_addr.ip_addr = (*report).remote_addr;
    rem_addr.port = 0 as libc::c_int as uint16_t;
    find_slot(&mut rem_addr, &mut slot, &mut found);
    if found == 0 { return 0 as libc::c_int }
    NCR_GetNTPReport((*get_record(slot as libc::c_uint)).data, report);
    return 1 as libc::c_int;
}
/* Procedure to add a new server or peer source. */
/* Procedure to add a new server, peer source, or pool of servers specified by
   name instead of address.  The name is resolved in exponentially increasing
   intervals until it succeeds or fails with a non-temporary error.  If the
   name is an address, it is equivalent to NSR_AddSource(). */
/* Function type for handlers to be called back when an attempt
 * (possibly unsuccessful) to resolve unresolved sources ends */
/* Set the handler, or NULL to disable the notification */
/* Procedure to start resolving unresolved sources */
/* Procedure to start all sources */
/* Start new sources automatically */
/* Procedure to remove a source */
/* Procedure to remove all sources */
/* Procedure to try to find a replacement for a bad source */
/* Procedure to resolve all names again */
/* Procedure to get local reference ID corresponding to a source */
/* Procedure to get the name of a source.  If the source doesn't have a name,
   it returns a temporary string containing formatted address. */
/* This routine is called by ntp_io when a new packet arrives off the network */
/* This routine is called by ntp_io when a packet was sent to the network and
   an accurate transmit timestamp was captured */
/* Initialisation function */
/* Finalisation function */
/* This routine is used to indicate that sources whose IP addresses
   match a particular subnet should be set online or offline.  It returns
   a flag indicating whether any hosts matched the address. */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NSR_GetActivityReport(mut report:
                                                   *mut RPT_ActivityReport) {
    let mut record: *mut SourceRecord = 0 as *mut SourceRecord;
    let mut i: libc::c_uint = 0;
    let mut us: *mut UnresolvedSource = 0 as *mut UnresolvedSource;
    (*report).online = 0 as libc::c_int;
    (*report).offline = 0 as libc::c_int;
    (*report).burst_online = 0 as libc::c_int;
    (*report).burst_offline = 0 as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(records) {
        record = get_record(i);
        if !(*record).remote_addr.is_null() {
            NCR_IncrementActivityCounters((*record).data,
                                          &mut (*report).online,
                                          &mut (*report).offline,
                                          &mut (*report).burst_online,
                                          &mut (*report).burst_offline);
        }
        i = i.wrapping_add(1)
    }
    (*report).unresolved = 0 as libc::c_int;
    us = unresolved_sources;
    while !us.is_null() { (*report).unresolved += 1; us = (*us).next };
}
/* ================================================== */
