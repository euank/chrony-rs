#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           register_tool)]
extern "C" {
    pub type ADF_AuthTableInst;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    #[no_mangle]
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    #[no_mangle]
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    #[no_mangle]
    fn htons(__hostshort: uint16_t) -> uint16_t;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* Register a handler for when select goes true on a file descriptor */
    #[no_mangle]
    fn SCH_AddFileHandler(
        fd: libc::c_int,
        events: libc::c_int,
        handler: SCH_FileHandler,
        arg: SCH_ArbitraryArgument,
    );
    #[no_mangle]
    fn SCH_RemoveFileHandler(fd: libc::c_int);
    /* Get the time stamp taken after a file descriptor became ready or a timeout expired */
    #[no_mangle]
    fn SCH_GetLastEventTime(cooked: *mut timespec, err: *mut libc::c_double, raw: *mut timespec);
    #[no_mangle]
    fn SCH_QuitProgram();
    #[no_mangle]
    fn UTI_IPNetworkToHost(src: *mut IPAddr, dest: *mut IPAddr);
    #[no_mangle]
    fn UTI_IPHostToNetwork(src: *mut IPAddr, dest: *mut IPAddr);
    #[no_mangle]
    fn UTI_IPSockAddrToString(sa: *mut IPSockAddr) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_CompareIPs(a: *mut IPAddr, b: *mut IPAddr, mask: *mut IPAddr) -> libc::c_int;
    #[no_mangle]
    fn UTI_FloatNetworkToHost(x: Float) -> libc::c_double;
    #[no_mangle]
    fn UTI_TimespecNetworkToHost(src: *mut Timespec, dest: *mut timespec);
    #[no_mangle]
    fn UTI_TimespecHostToNetwork(src: *mut timespec, dest: *mut Timespec);
    #[no_mangle]
    fn UTI_FloatHostToNetwork(x: libc::c_double) -> Float;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LOG_CycleLogFiles();
    #[no_mangle]
    fn KEY_Reload();
    /* Procedure to add a new server, peer source, or pool of servers specified by
    name instead of address.  The name is resolved in exponentially increasing
    intervals until it succeeds or fails with a non-temporary error.  If the
    name is an address, it is equivalent to NSR_AddSource(). */
    #[no_mangle]
    fn NSR_AddSourceByName(
        name: *mut libc::c_char,
        port: libc::c_int,
        pool: libc::c_int,
        type_0: NTP_Source_Type,
        params: *mut SourceParameters,
    ) -> NSR_Status;
    /* Procedure to start resolving unresolved sources */
    #[no_mangle]
    fn NSR_ResolveSources();
    /* Procedure to remove a source */
    #[no_mangle]
    fn NSR_RemoveSource(remote_addr: *mut NTP_Remote_Address) -> NSR_Status;
    /* Procedure to resolve all names again */
    #[no_mangle]
    fn NSR_RefreshAddresses();
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

     This is the header for the module that manages the collection of all
     sources that we are making measurements from.  This include all NTP
     servers & peers, locally connected reference sources, eye/wristwatch
     drivers etc */
    /* Size of the source reachability register */
    /* This datatype is used to hold information about sources.  The
    instance must be passed when calling many of the interface
    functions */
    /* Initialisation function */
    /* Finalisation function */
    /* NTP client/peer */
    /* Rerefence clock */
    /* Function to create a new instance.  This would be called by one of
    the individual source-type instance creation routines. */
    /* Function to get rid of a source when it is being unconfigured.
    This may cause the current reference source to be reselected, if this
    was the reference source or contributed significantly to a
    falseticker decision. */
    /* Function to reset a source */
    /* Function to change the sources's reference ID and IP address */
    /* Function to get access to the sourcestats instance */
    /* This function is called by one of the source drivers when it has
    a new sample that is to be accumulated */
    /* This routine sets the source as receiving reachability updates */
    /* This routine sets the source as not receiving reachability updates */
    /* This routine updates the reachability register */
    /* This routine marks the source unreachable */
    /* This routine is used to select the best source from amongst those
    we currently have valid data on, and use it as the tracking base
    for the local time.  Updates are made to the local reference only
    when the selected source was updated (set as updated_inst) since
    the last reference update.  This avoids updating the frequency
    tracking for every sample from other sources - only the ones from
    the selected reference make a difference. */
    /* Force reselecting the best source */
    /* Set reselect distance */
    #[no_mangle]
    fn SRC_ReportSource(
        index: libc::c_int,
        report: *mut RPT_SourceReport,
        now: *mut timespec,
    ) -> libc::c_int;
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
    #[no_mangle]
    fn NSR_GetNTPReport(report: *mut RPT_NTPReport) -> libc::c_int;
    #[no_mangle]
    fn NSR_ModifyMaxdelaydevratio(
        address: *mut IPAddr,
        new_max_delay_ratio: libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn NSR_ModifyMaxdelay(address: *mut IPAddr, new_max_delay: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn NSR_ModifyMaxpoll(address: *mut IPAddr, new_maxpoll: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn NSR_ModifyMinpoll(address: *mut IPAddr, new_minpoll: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn NSR_InitiateSampleBurst(
        n_good_samples: libc::c_int,
        n_total_samples: libc::c_int,
        mask: *mut IPAddr,
        address: *mut IPAddr,
    ) -> libc::c_int;
    #[no_mangle]
    fn SRC_DumpSources();
    #[no_mangle]
    fn SRC_ReadNumberOfSources() -> libc::c_int;
    #[no_mangle]
    fn NSR_ModifyPolltarget(address: *mut IPAddr, new_poll_target: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn NSR_ReportSource(report: *mut RPT_SourceReport, now: *mut timespec);
    #[no_mangle]
    fn NCR_AddAccessRestriction(
        ip_addr: *mut IPAddr,
        subnet_bits: libc::c_int,
        allow: libc::c_int,
        all: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn NCR_CheckAccessRestriction(ip_addr: *mut IPAddr) -> libc::c_int;
    #[no_mangle]
    fn NSR_GetName(address: *mut IPAddr) -> *mut libc::c_char;
    #[no_mangle]
    fn NSR_ModifyMinstratum(address: *mut IPAddr, new_min_stratum: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SRC_ReselectSource();
    #[no_mangle]
    fn SRC_SetReselectDistance(distance: libc::c_double);
    #[no_mangle]
    fn NSR_SetConnectivity(
        mask: *mut IPAddr,
        address: *mut IPAddr,
        connectivity: SRC_Connectivity,
    ) -> libc::c_int;
    #[no_mangle]
    fn NSR_ModifyMaxdelayratio(
        address: *mut IPAddr,
        new_max_delay_ratio: libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn NSR_GetActivityReport(report: *mut RPT_ActivityReport);
    #[no_mangle]
    fn SRC_ReportSourcestats(
        index: libc::c_int,
        report: *mut RPT_SourcestatsReport,
        now: *mut timespec,
    ) -> libc::c_int;
    #[no_mangle]
    fn SRC_GetType(index: libc::c_int) -> SRC_Type;
    #[no_mangle]
    fn SMT_IsEnabled() -> libc::c_int;
    #[no_mangle]
    fn SMT_Activate(now: *mut timespec);
    #[no_mangle]
    fn SMT_Reset(now: *mut timespec);
    #[no_mangle]
    fn SMT_GetSmoothingReport(report: *mut RPT_SmoothingReport, now: *mut timespec) -> libc::c_int;
    /* Check if support for the IP family was enabled in the build */
    #[no_mangle]
    fn SCK_IsFamilySupported(family: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SCK_GetLoopbackIPAddress(family: libc::c_int, local_addr: *mut IPAddr);
    /* Open socket */
    #[no_mangle]
    fn SCK_OpenUdpSocket(
        remote_addr: *mut IPSockAddr,
        local_addr: *mut IPSockAddr,
        flags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn SCK_OpenUnixDatagramSocket(
        remote_addr: *const libc::c_char,
        local_addr: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
    /* Receive a single message or multiple messages.  The functions return the
    number of received messages, or 0 on error.  The returned data point to
    static buffers, which are valid until another call of these functions.  */
    #[no_mangle]
    fn SCK_ReceiveMessage(
        sock_fd: libc::c_int,
        message: *mut SCK_Message,
        flags: libc::c_int,
    ) -> libc::c_int;
    /* Send a message */
    #[no_mangle]
    fn SCK_SendMessage(
        sock_fd: libc::c_int,
        message: *mut SCK_Message,
        flags: libc::c_int,
    ) -> libc::c_int;
    /* Remove bound Unix socket */
    #[no_mangle]
    fn SCK_RemoveSocket(sock_fd: libc::c_int) -> libc::c_int;
    /* Close the socket */
    #[no_mangle]
    fn SCK_CloseSocket(sock_fd: libc::c_int);
    /* Modify the setting for the maximum skew we are prepared to allow updates on (in ppm). */
    #[no_mangle]
    fn REF_ModifyMaxupdateskew(new_max_update_skew: libc::c_double);
    /* Modify makestep settings */
    #[no_mangle]
    fn REF_ModifyMakestep(limit: libc::c_int, threshold: libc::c_double);
    #[no_mangle]
    fn REF_EnableLocal(stratum: libc::c_int, distance: libc::c_double, orphan: libc::c_int);
    #[no_mangle]
    fn REF_DisableLocal();
    #[no_mangle]
    fn REF_GetTrackingReport(rep: *mut RPT_TrackingReport);
    #[no_mangle]
    fn MNL_AcceptTimestamp(
        ts: *mut timespec,
        reg_offset: *mut libc::c_double,
        dfreq_ppm: *mut libc::c_double,
        new_afreq_ppm: *mut libc::c_double,
    ) -> libc::c_int;
    #[no_mangle]
    fn MNL_Enable();
    #[no_mangle]
    fn MNL_Disable();
    #[no_mangle]
    fn MNL_Reset();
    #[no_mangle]
    fn MNL_IsEnabled() -> libc::c_int;
    #[no_mangle]
    fn MNL_ReportSamples(
        report: *mut RPT_ManualSamplesReport,
        max: libc::c_int,
        n: *mut libc::c_int,
    );
    #[no_mangle]
    fn MNL_DeleteSample(index: libc::c_int) -> libc::c_int;
    /* Routine to apply a change of frequency to the local clock.  The
    argument is the estimated gain (positive) or loss (negative) of the
    local clock relative to true time, per unit time of the PREVIOUS
    frequency setting of the local clock.  This is assumed to be based
    on a regression of y=offset v x=cooked local time. */
    #[no_mangle]
    fn LCL_AccumulateDeltaFrequency(dfreq: libc::c_double);
    /* Routine to apply an offset (in seconds) to the local clock.  The
    argument should be positive to move the clock backwards (i.e. the
    local clock is currently fast of true time), or negative to move it
    forwards (i.e. it is currently slow of true time).  Provided is also
    a suggested correction rate (correction time * offset). */
    #[no_mangle]
    fn LCL_AccumulateOffset(offset: libc::c_double, corr_rate: libc::c_double);
    /* Routine to convert the outstanding system clock error to a step and
    apply it, e.g. if the system clock has ended up an hour wrong due
    to a timezone problem. */
    #[no_mangle]
    fn LCL_MakeStep() -> libc::c_int;
    /* Create a new table.  The default rule is deny for everything */
    #[no_mangle]
    fn ADF_CreateTable() -> ADF_AuthTable;
    /* Allow anything in the supplied subnet, EXCEPT for any more specific
    subnets that are already defined */
    #[no_mangle]
    fn ADF_Allow(table: ADF_AuthTable, ip: *mut IPAddr, subnet_bits: libc::c_int) -> ADF_Status;
    /* Allow anything in the supplied subnet, overwriting existing
    definitions for any more specific subnets */
    #[no_mangle]
    fn ADF_AllowAll(table: ADF_AuthTable, ip: *mut IPAddr, subnet_bits: libc::c_int) -> ADF_Status;
    /* Deny anything in the supplied subnet, EXCEPT for any more specific
    subnets that are already defined */
    #[no_mangle]
    fn ADF_Deny(table: ADF_AuthTable, ip: *mut IPAddr, subnet_bits: libc::c_int) -> ADF_Status;
    /* Deny anything in the supplied subnet, overwriting existing
    definitions for any more specific subnets */
    #[no_mangle]
    fn ADF_DenyAll(table: ADF_AuthTable, ip: *mut IPAddr, subnet_bits: libc::c_int) -> ADF_Status;
    /* Clear up the table */
    #[no_mangle]
    fn ADF_DestroyTable(table: ADF_AuthTable);
    /* Check whether a given IP address is allowed by the rules in
    the table */
    #[no_mangle]
    fn ADF_IsAllowed(table: ADF_AuthTable, ip: *mut IPAddr) -> libc::c_int;
    #[no_mangle]
    fn CNF_GetCommandPort() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetBindCommandAddress(family: libc::c_int, addr: *mut IPAddr);
    #[no_mangle]
    fn CNF_GetBindCommandPath() -> *mut libc::c_char;
    #[no_mangle]
    fn RTC_GetReport(report: *mut RPT_RTC_Report) -> libc::c_int;
    #[no_mangle]
    fn RTC_WriteParameters() -> libc::c_int;
    #[no_mangle]
    fn RTC_Trim() -> libc::c_int;
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
    #[no_mangle]
    fn PKL_CommandLength(r: *mut CMD_Request) -> libc::c_int;
    #[no_mangle]
    fn PKL_CommandPaddingLength(r: *mut CMD_Request) -> libc::c_int;
    #[no_mangle]
    fn PKL_ReplyLength(r: *mut CMD_Reply) -> libc::c_int;
    #[no_mangle]
    fn CLG_LogCommandAccess(client: *mut IPAddr, now: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn CLG_LimitCommandResponseRate(index: libc::c_int) -> libc::c_int;
    /* And some reporting functions, for use by chronyc. */
    #[no_mangle]
    fn CLG_GetNumberOfIndices() -> libc::c_int;
    #[no_mangle]
    fn CLG_GetClientAccessReportByIndex(
        index: libc::c_int,
        report: *mut RPT_ClientAccessByIndex_Report,
        now: *mut timespec,
    ) -> libc::c_int;
    #[no_mangle]
    fn CLG_GetServerStatsReport(report: *mut RPT_ServerStatsReport);
    #[no_mangle]
    fn RCL_ReportSource(report: *mut RPT_SourceReport, now: *mut timespec);
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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

 Module for providing an authorisation filter on IP addresses
 */
pub type ADF_AuthTable = *mut ADF_AuthTableInst;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCK_Message {
    pub data: *mut libc::c_void,
    pub length: libc::c_uint,
    pub addr_type: SCK_AddressType,
    pub if_index: libc::c_int,
    pub remote_addr: C2RustUnnamed_2,
    pub local_addr: C2RustUnnamed_1,
    pub timestamp: C2RustUnnamed_0,
    pub descriptor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub kernel: timespec,
    pub hw: timespec,
    pub if_index: libc::c_int,
    pub l2_length: libc::c_int,
    pub tx_flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ip: IPAddr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ip: IPSockAddr,
    pub path: *const libc::c_char,
}
pub type SCK_AddressType = libc::c_uint;
pub const SCK_ADDR_UNIX: SCK_AddressType = 2;
pub const SCK_ADDR_IP: SCK_AddressType = 1;
pub const SCK_ADDR_UNSPEC: SCK_AddressType = 0;
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
    pub data: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
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
    pub data: C2RustUnnamed_4,
    pub padding: [uint8_t; 396],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
pub type SRC_Connectivity = libc::c_uint;
pub const SRC_MAYBE_ONLINE: SRC_Connectivity = 2;
pub const SRC_ONLINE: SRC_Connectivity = 1;
pub const SRC_OFFLINE: SRC_Connectivity = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_ServerStatsReport {
    pub ntp_hits: uint32_t,
    pub cmd_hits: uint32_t,
    pub ntp_drops: uint32_t,
    pub cmd_drops: uint32_t,
    pub log_drops: uint32_t,
}
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
pub struct RPT_ManualSamplesReport {
    pub when: timespec,
    pub slewed_offset: libc::c_double,
    pub orig_offset: libc::c_double,
    pub residual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_ClientAccessByIndex_Report {
    pub ip_addr: IPAddr,
    pub ntp_hits: uint32_t,
    pub cmd_hits: uint32_t,
    pub ntp_drops: uint16_t,
    pub cmd_drops: uint16_t,
    pub ntp_interval: int8_t,
    pub cmd_interval: int8_t,
    pub ntp_timeout_interval: int8_t,
    pub last_ntp_hit_ago: uint32_t,
    pub last_cmd_hit_ago: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_RTC_Report {
    pub ref_time: timespec,
    pub n_samples: libc::c_ushort,
    pub n_runs: libc::c_ushort,
    pub span_seconds: libc::c_ulong,
    pub rtc_seconds_fast: libc::c_double,
    pub rtc_gain_rate_ppm: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_SourcestatsReport {
    pub ref_id: uint32_t,
    pub ip_addr: IPAddr,
    pub n_samples: libc::c_ulong,
    pub n_runs: libc::c_ulong,
    pub span_seconds: libc::c_ulong,
    pub resid_freq_ppm: libc::c_double,
    pub skew_ppm: libc::c_double,
    pub sd: libc::c_double,
    pub est_offset: libc::c_double,
    pub est_offset_err: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_SmoothingReport {
    pub active: libc::c_int,
    pub leap_only: libc::c_int,
    pub offset: libc::c_double,
    pub freq_ppm: libc::c_double,
    pub wander_ppm: libc::c_double,
    pub last_update_ago: libc::c_double,
    pub remaining_time: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_TrackingReport {
    pub ref_id: uint32_t,
    pub ip_addr: IPAddr,
    pub stratum: libc::c_int,
    pub leap_status: NTP_Leap,
    pub ref_time: timespec,
    pub current_correction: libc::c_double,
    pub last_offset: libc::c_double,
    pub rms_offset: libc::c_double,
    pub freq_ppm: libc::c_double,
    pub resid_freq_ppm: libc::c_double,
    pub skew_ppm: libc::c_double,
    pub root_delay: libc::c_double,
    pub root_dispersion: libc::c_double,
    pub last_update_interval: libc::c_double,
}
pub type NTP_Leap = libc::c_uint;
pub const LEAP_Unsynchronised: NTP_Leap = 3;
pub const LEAP_DeleteSecond: NTP_Leap = 2;
pub const LEAP_InsertSecond: NTP_Leap = 1;
pub const LEAP_Normal: NTP_Leap = 0;
pub const NSR_UnresolvedName: NSR_Status = 6;
pub const NSR_InvalidName: NSR_Status = 5;
pub const NSR_InvalidAF: NSR_Status = 4;
pub const NSR_AlreadyInUse: NSR_Status = 2;
pub const NSR_TooManySources: NSR_Status = 3;
pub const NSR_NoSuchSource: NSR_Status = 1;
pub const NSR_Success: NSR_Status = 0;
pub type NSR_Status = libc::c_uint;
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
pub const ADF_SUCCESS: ADF_Status = 0;
pub type ADF_Status = libc::c_uint;
pub const ADF_BADSUBNET: ADF_Status = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPT_SourceReport {
    pub ip_addr: IPAddr,
    pub stratum: libc::c_int,
    pub poll: libc::c_int,
    pub mode: C2RustUnnamed_6,
    pub state: C2RustUnnamed_5,
    pub sel_options: libc::c_int,
    pub reachability: libc::c_int,
    pub latest_meas_ago: libc::c_ulong,
    pub orig_latest_meas: libc::c_double,
    pub latest_meas: libc::c_double,
    pub latest_meas_err: libc::c_double,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const RPT_OUTLIER: C2RustUnnamed_5 = 5;
pub const RPT_CANDIDATE: C2RustUnnamed_5 = 4;
pub const RPT_JITTERY: C2RustUnnamed_5 = 3;
pub const RPT_FALSETICKER: C2RustUnnamed_5 = 2;
pub const RPT_UNREACH: C2RustUnnamed_5 = 1;
pub const RPT_SYNC: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const RPT_LOCAL_REFERENCE: C2RustUnnamed_6 = 2;
pub const RPT_NTP_PEER: C2RustUnnamed_6 = 1;
pub const RPT_NTP_CLIENT: C2RustUnnamed_6 = 0;
pub const SRC_REFCLOCK: SRC_Type = 1;
pub const SRC_NTP: SRC_Type = 0;
pub type SRC_Type = libc::c_uint;
pub type SCH_ArbitraryArgument = *mut libc::c_void;
pub type SCH_FileHandler =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: SCH_ArbitraryArgument) -> ()>;
/* File descriptors for command and monitoring sockets */
static mut sock_fdu: libc::c_int = 0;
static mut sock_fd4: libc::c_int = 0;
static mut sock_fd6: libc::c_int = 0;
/* Flag indicating whether this module has been initialised or not */
static mut initialised: libc::c_int = 0 as libc::c_int;
/* ================================================== */
/* Array of permission levels for command types */
static mut permissions: [libc::c_char; 66] = [
    0 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
/* ================================================== */
/* This authorisation table is used for checking whether particular
machines are allowed to make command and monitoring requests. */
static mut access_auth_table: ADF_AuthTable =
    0 as *const ADF_AuthTableInst as *mut ADF_AuthTableInst;
/* ================================================== */
unsafe extern "C" fn open_socket(mut family: libc::c_int) -> libc::c_int {
    let mut local_addr: IPSockAddr = IPSockAddr {
        ip_addr: IPAddr {
            addr: C2RustUnnamed { in4: 0 },
            family: 0,
            _pad: 0,
        },
        port: 0,
    };
    let mut local_path: *const libc::c_char = 0 as *const libc::c_char;
    let mut sock_fd: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    match family {
        1 | 2 => {
            port = CNF_GetCommandPort();
            if port == 0 as libc::c_int || SCK_IsFamilySupported(family) == 0 {
                return -(5 as libc::c_int);
            }
            CNF_GetBindCommandAddress(family, &mut local_addr.ip_addr);
            if local_addr.ip_addr.family as libc::c_int != family {
                SCK_GetLoopbackIPAddress(family, &mut local_addr.ip_addr);
            }
            local_addr.port = port as uint16_t;
            sock_fd = SCK_OpenUdpSocket(0 as *mut IPSockAddr, &mut local_addr, 4 as libc::c_int);
            if sock_fd < 0 as libc::c_int {
                LOG_Message(
                    LOGS_ERR,
                    b"Could not open command socket on %s\x00" as *const u8 as *const libc::c_char,
                    UTI_IPSockAddrToString(&mut local_addr),
                );
                return -(5 as libc::c_int);
            }
        }
        0 => {
            local_path = CNF_GetBindCommandPath();
            sock_fd =
                SCK_OpenUnixDatagramSocket(0 as *const libc::c_char, local_path, 0 as libc::c_int);
            if sock_fd < 0 as libc::c_int {
                LOG_Message(
                    LOGS_ERR,
                    b"Could not open command socket on %s\x00" as *const u8 as *const libc::c_char,
                    local_path,
                );
                return -(5 as libc::c_int);
            }
        }
        _ => {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
                189 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"int open_socket(int)\x00",
                ))
                .as_ptr(),
            );
        }
    }
    /* Register handler for read events on the socket */
    SCH_AddFileHandler(
        sock_fd,
        1 as libc::c_int,
        Some(
            read_from_cmd_socket
                as unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_void,
    );
    return sock_fd;
}
/* ================================================== */
unsafe extern "C" fn do_size_checks() {
    let mut i: libc::c_int = 0;
    let mut request_length: libc::c_int = 0;
    let mut padding_length: libc::c_int = 0;
    let mut reply_length: libc::c_int = 0;
    let mut request: CMD_Request = CMD_Request {
        version: 0,
        pkt_type: 0,
        res1: 0,
        res2: 0,
        command: 0,
        attempt: 0,
        sequence: 0,
        pad1: 0,
        pad2: 0,
        data: C2RustUnnamed_4 {
            null: REQ_Null { EOR: 0 },
        },
        padding: [0; 396],
    };
    let mut reply: CMD_Reply = CMD_Reply {
        version: 0,
        pkt_type: 0,
        res1: 0,
        res2: 0,
        command: 0,
        reply: 0,
        status: 0,
        pad1: 0,
        pad2: 0,
        pad3: 0,
        sequence: 0,
        pad4: 0,
        pad5: 0,
        data: C2RustUnnamed_3 {
            null: RPY_Null { EOR: 0 },
        },
    };
    if 20 as libc::c_ulong == 20 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(
            b"offsetof(CMD_Request, data) == 20\x00" as *const u8 as *const libc::c_char,
            b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
            207 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"void do_size_checks(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if 28 as libc::c_ulong == 28 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(
            b"offsetof(CMD_Reply, data) == 28\x00" as *const u8 as *const libc::c_char,
            b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
            208 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"void do_size_checks(void)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < 66 as libc::c_int {
        request.version = 6 as libc::c_int as uint8_t;
        request.command = htons(i as uint16_t);
        request_length = PKL_CommandLength(&mut request);
        padding_length = PKL_CommandPaddingLength(&mut request);
        if padding_length > 396 as libc::c_int
            || padding_length > request_length
            || request_length as libc::c_ulong
                > ::std::mem::size_of::<CMD_Request>() as libc::c_ulong
            || request_length != 0 && (request_length as libc::c_ulong) < 20 as libc::c_ulong
        {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
                218 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"void do_size_checks(void)\x00",
                ))
                .as_ptr(),
            );
        }
        i += 1
    }
    i = 1 as libc::c_int;
    while i < 20 as libc::c_int {
        reply.reply = htons(i as uint16_t);
        reply.status = 0 as libc::c_int as uint16_t;
        reply_length = PKL_ReplyLength(&mut reply);
        if reply_length != 0 && (reply_length as libc::c_ulong) < 28 as libc::c_ulong
            || reply_length as libc::c_ulong > ::std::mem::size_of::<CMD_Reply>() as libc::c_ulong
        {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
                227 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"void do_size_checks(void)\x00",
                ))
                .as_ptr(),
            );
        }
        i += 1
    }
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

 Header file for the control and monitoring module in the software
 */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CAM_Initialise(mut family: libc::c_int) {
    if initialised == 0 {
    } else {
        __assert_fail(
            b"!initialised\x00" as *const u8 as *const libc::c_char,
            b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
            236 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"void CAM_Initialise(int)\x00",
            ))
            .as_ptr(),
        );
    }
    if (::std::mem::size_of::<[libc::c_char; 66]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        == 66 as libc::c_int as libc::c_ulong
    {
    } else {
        __assert_fail(
            b"sizeof (permissions) / sizeof (permissions[0]) == N_REQUEST_TYPES\x00" as *const u8
                as *const libc::c_char,
            b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
            237 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"void CAM_Initialise(int)\x00",
            ))
            .as_ptr(),
        );
    }
    do_size_checks();
    initialised = 1 as libc::c_int;
    sock_fdu = -(5 as libc::c_int);
    sock_fd4 = -(5 as libc::c_int);
    sock_fd6 = -(5 as libc::c_int);
    if family == 0 as libc::c_int || family == 1 as libc::c_int {
        sock_fd4 = open_socket(1 as libc::c_int)
    }
    if family == 0 as libc::c_int || family == 2 as libc::c_int {
        sock_fd6 = open_socket(2 as libc::c_int)
    }
    access_auth_table = ADF_CreateTable();
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CAM_Finalise() {
    if sock_fdu != -(5 as libc::c_int) {
        SCH_RemoveFileHandler(sock_fdu);
        SCK_RemoveSocket(sock_fdu);
        SCK_CloseSocket(sock_fdu);
        sock_fdu = -(5 as libc::c_int)
    }
    if sock_fd4 != -(5 as libc::c_int) {
        SCH_RemoveFileHandler(sock_fd4);
        SCK_CloseSocket(sock_fd4);
        sock_fd4 = -(5 as libc::c_int)
    }
    if sock_fd6 != -(5 as libc::c_int) {
        SCH_RemoveFileHandler(sock_fd6);
        SCK_CloseSocket(sock_fd6);
        sock_fd6 = -(5 as libc::c_int)
    }
    ADF_DestroyTable(access_auth_table);
    initialised = 0 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CAM_OpenUnixSocket() {
    /* This is separated from CAM_Initialise() as it needs to be called when
    the process has already dropped the root privileges */
    if *CNF_GetBindCommandPath().offset(0 as libc::c_int as isize) != 0 {
        sock_fdu = open_socket(0 as libc::c_int)
    };
}
/* ================================================== */
unsafe extern "C" fn transmit_reply(mut sock_fd: libc::c_int, mut message: *mut SCK_Message) {
    (*message).length = PKL_ReplyLength((*message).data as *mut CMD_Reply) as libc::c_uint;
    if SCK_SendMessage(sock_fd, message, 0 as libc::c_int) == 0 {
        return;
    };
}
/* ================================================== */
unsafe extern "C" fn handle_dump(mut rx_message: *mut CMD_Request, mut tx_message: *mut CMD_Reply) {
    SRC_DumpSources();
}
/* ================================================== */
unsafe extern "C" fn handle_online(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut address: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    let mut mask: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    UTI_IPNetworkToHost(&mut (*rx_message).data.online.mask, &mut mask);
    UTI_IPNetworkToHost(&mut (*rx_message).data.online.address, &mut address);
    if NSR_SetConnectivity(&mut mask, &mut address, SRC_ONLINE) == 0 {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_offline(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut address: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    let mut mask: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    UTI_IPNetworkToHost(&mut (*rx_message).data.offline.mask, &mut mask);
    UTI_IPNetworkToHost(&mut (*rx_message).data.offline.address, &mut address);
    if NSR_SetConnectivity(&mut mask, &mut address, SRC_OFFLINE) == 0 {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_onoffline(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut address: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    let mut mask: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    mask.family = 0 as libc::c_int as uint16_t;
    address.family = mask.family;
    (NSR_SetConnectivity(&mut mask, &mut address, SRC_MAYBE_ONLINE)) == 0;
}
/* ================================================== */
unsafe extern "C" fn handle_burst(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut address: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    let mut mask: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    UTI_IPNetworkToHost(&mut (*rx_message).data.burst.mask, &mut mask);
    UTI_IPNetworkToHost(&mut (*rx_message).data.burst.address, &mut address);
    if NSR_InitiateSampleBurst(
        ntohl((*rx_message).data.burst.n_good_samples as uint32_t) as libc::c_int,
        ntohl((*rx_message).data.burst.n_total_samples as uint32_t) as libc::c_int,
        &mut mask,
        &mut address,
    ) == 0
    {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_modify_minpoll(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut address: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    UTI_IPNetworkToHost(&mut (*rx_message).data.modify_minpoll.address, &mut address);
    if NSR_ModifyMinpoll(
        &mut address,
        ntohl((*rx_message).data.modify_minpoll.new_minpoll as uint32_t) as libc::c_int,
    ) == 0
    {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_modify_maxpoll(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut address: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    UTI_IPNetworkToHost(&mut (*rx_message).data.modify_minpoll.address, &mut address);
    if NSR_ModifyMaxpoll(
        &mut address,
        ntohl((*rx_message).data.modify_minpoll.new_minpoll as uint32_t) as libc::c_int,
    ) == 0
    {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_modify_maxdelay(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut address: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    UTI_IPNetworkToHost(
        &mut (*rx_message).data.modify_maxdelay.address,
        &mut address,
    );
    if NSR_ModifyMaxdelay(
        &mut address,
        UTI_FloatNetworkToHost((*rx_message).data.modify_maxdelay.new_max_delay),
    ) == 0
    {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_modify_maxdelayratio(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut address: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    UTI_IPNetworkToHost(
        &mut (*rx_message).data.modify_maxdelayratio.address,
        &mut address,
    );
    if NSR_ModifyMaxdelayratio(
        &mut address,
        UTI_FloatNetworkToHost((*rx_message).data.modify_maxdelayratio.new_max_delay_ratio),
    ) == 0
    {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_modify_maxdelaydevratio(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut address: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    UTI_IPNetworkToHost(
        &mut (*rx_message).data.modify_maxdelaydevratio.address,
        &mut address,
    );
    if NSR_ModifyMaxdelaydevratio(
        &mut address,
        UTI_FloatNetworkToHost(
            (*rx_message)
                .data
                .modify_maxdelaydevratio
                .new_max_delay_dev_ratio,
        ),
    ) == 0
    {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_modify_minstratum(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut address: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    UTI_IPNetworkToHost(&mut (*rx_message).data.modify_minpoll.address, &mut address);
    if NSR_ModifyMinstratum(
        &mut address,
        ntohl((*rx_message).data.modify_minstratum.new_min_stratum as uint32_t) as libc::c_int,
    ) == 0
    {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_modify_polltarget(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut address: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    UTI_IPNetworkToHost(
        &mut (*rx_message).data.modify_polltarget.address,
        &mut address,
    );
    if NSR_ModifyPolltarget(
        &mut address,
        ntohl((*rx_message).data.modify_polltarget.new_poll_target as uint32_t) as libc::c_int,
    ) == 0
    {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_modify_maxupdateskew(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    REF_ModifyMaxupdateskew(UTI_FloatNetworkToHost(
        (*rx_message).data.modify_maxupdateskew.new_max_update_skew,
    ));
}
/* ================================================== */
unsafe extern "C" fn handle_modify_makestep(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    REF_ModifyMakestep(
        ntohl((*rx_message).data.modify_makestep.limit as uint32_t) as libc::c_int,
        UTI_FloatNetworkToHost((*rx_message).data.modify_makestep.threshold),
    );
}
/* ================================================== */
unsafe extern "C" fn handle_settime(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut offset: libc::c_double = 0.;
    let mut dfreq_ppm: libc::c_double = 0.;
    let mut new_afreq_ppm: libc::c_double = 0.;
    UTI_TimespecNetworkToHost(&mut (*rx_message).data.settime.ts, &mut ts);
    if MNL_IsEnabled() == 0 {
        (*tx_message).status = htons(6 as libc::c_int as uint16_t)
    } else if MNL_AcceptTimestamp(&mut ts, &mut offset, &mut dfreq_ppm, &mut new_afreq_ppm) != 0 {
        (*tx_message).reply = htons(17 as libc::c_int as uint16_t);
        (*tx_message).data.manual_timestamp.offset = UTI_FloatHostToNetwork(offset);
        (*tx_message).data.manual_timestamp.dfreq_ppm = UTI_FloatHostToNetwork(dfreq_ppm);
        (*tx_message).data.manual_timestamp.new_afreq_ppm = UTI_FloatHostToNetwork(new_afreq_ppm)
    } else {
        (*tx_message).status = htons(1 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_local(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    if ntohl((*rx_message).data.local.on_off as uint32_t) != 0 {
        REF_EnableLocal(
            ntohl((*rx_message).data.local.stratum as uint32_t) as libc::c_int,
            UTI_FloatNetworkToHost((*rx_message).data.local.distance),
            ntohl((*rx_message).data.local.orphan as uint32_t) as libc::c_int,
        );
    } else {
        REF_DisableLocal();
    };
}
/* ================================================== */
unsafe extern "C" fn handle_manual(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut option: libc::c_int = 0;
    option = ntohl((*rx_message).data.manual.option as uint32_t) as libc::c_int;
    match option {
        0 => {
            MNL_Disable();
        }
        1 => {
            MNL_Enable();
        }
        2 => {
            MNL_Reset();
        }
        _ => (*tx_message).status = htons(3 as libc::c_int as uint16_t),
    };
}
/* ================================================== */
unsafe extern "C" fn handle_n_sources(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut n_sources: libc::c_int = 0;
    n_sources = SRC_ReadNumberOfSources();
    (*tx_message).reply = htons(2 as libc::c_int as uint16_t);
    (*tx_message).data.n_sources.n_sources = htonl(n_sources as uint32_t);
}
/* ================================================== */
unsafe extern "C" fn handle_source_data(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut report: RPT_SourceReport = RPT_SourceReport {
        ip_addr: IPAddr {
            addr: C2RustUnnamed { in4: 0 },
            family: 0,
            _pad: 0,
        },
        stratum: 0,
        poll: 0,
        mode: RPT_NTP_CLIENT,
        state: RPT_SYNC,
        sel_options: 0,
        reachability: 0,
        latest_meas_ago: 0,
        orig_latest_meas: 0.,
        latest_meas: 0.,
        latest_meas_err: 0.,
    };
    let mut now_corr: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    /* Get data */
    SCH_GetLastEventTime(&mut now_corr, 0 as *mut libc::c_double, 0 as *mut timespec);
    if SRC_ReportSource(
        ntohl((*rx_message).data.source_data.index as uint32_t) as libc::c_int,
        &mut report,
        &mut now_corr,
    ) != 0
    {
        match SRC_GetType(ntohl((*rx_message).data.source_data.index as uint32_t) as libc::c_int)
            as libc::c_uint
        {
            0 => {
                NSR_ReportSource(&mut report, &mut now_corr);
            }
            1 => {
                RCL_ReportSource(&mut report, &mut now_corr);
            }
            _ => {}
        }
        (*tx_message).reply = htons(3 as libc::c_int as uint16_t);
        UTI_IPHostToNetwork(
            &mut report.ip_addr,
            &mut (*tx_message).data.source_data.ip_addr,
        );
        (*tx_message).data.source_data.stratum = htons(report.stratum as uint16_t);
        (*tx_message).data.source_data.poll = htons(report.poll as uint16_t) as int16_t;
        match report.state as libc::c_uint {
            0 => (*tx_message).data.source_data.state = htons(0 as libc::c_int as uint16_t),
            1 => (*tx_message).data.source_data.state = htons(1 as libc::c_int as uint16_t),
            2 => (*tx_message).data.source_data.state = htons(2 as libc::c_int as uint16_t),
            3 => (*tx_message).data.source_data.state = htons(3 as libc::c_int as uint16_t),
            4 => (*tx_message).data.source_data.state = htons(4 as libc::c_int as uint16_t),
            5 => (*tx_message).data.source_data.state = htons(5 as libc::c_int as uint16_t),
            _ => {}
        }
        match report.mode as libc::c_uint {
            0 => (*tx_message).data.source_data.mode = htons(0 as libc::c_int as uint16_t),
            1 => (*tx_message).data.source_data.mode = htons(1 as libc::c_int as uint16_t),
            2 => (*tx_message).data.source_data.mode = htons(2 as libc::c_int as uint16_t),
            _ => {}
        }
        (*tx_message).data.source_data.flags = htons(
            ((if report.sel_options & 0x2 as libc::c_int != 0 {
                0x2 as libc::c_int
            } else {
                0 as libc::c_int
            }) | (if report.sel_options & 0x1 as libc::c_int != 0 {
                0x1 as libc::c_int
            } else {
                0 as libc::c_int
            }) | (if report.sel_options & 0x4 as libc::c_int != 0 {
                0x4 as libc::c_int
            } else {
                0 as libc::c_int
            }) | (if report.sel_options & 0x8 as libc::c_int != 0 {
                0x8 as libc::c_int
            } else {
                0 as libc::c_int
            })) as uint16_t,
        );
        (*tx_message).data.source_data.reachability = htons(report.reachability as uint16_t);
        (*tx_message).data.source_data.since_sample = htonl(report.latest_meas_ago as uint32_t);
        (*tx_message).data.source_data.orig_latest_meas =
            UTI_FloatHostToNetwork(report.orig_latest_meas);
        (*tx_message).data.source_data.latest_meas = UTI_FloatHostToNetwork(report.latest_meas);
        (*tx_message).data.source_data.latest_meas_err =
            UTI_FloatHostToNetwork(report.latest_meas_err)
    } else {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_rekey(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    KEY_Reload();
}
/* ================================================== */
unsafe extern "C" fn handle_allowdeny(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
    mut allow: libc::c_int,
    mut all: libc::c_int,
) {
    let mut ip: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    let mut subnet_bits: libc::c_int = 0;
    UTI_IPNetworkToHost(&mut (*rx_message).data.allow_deny.ip, &mut ip);
    subnet_bits = ntohl((*rx_message).data.allow_deny.subnet_bits as uint32_t) as libc::c_int;
    if NCR_AddAccessRestriction(&mut ip, subnet_bits, allow, all) == 0 {
        (*tx_message).status = htons(7 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_cmdallowdeny(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
    mut allow: libc::c_int,
    mut all: libc::c_int,
) {
    let mut ip: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    let mut subnet_bits: libc::c_int = 0;
    UTI_IPNetworkToHost(&mut (*rx_message).data.allow_deny.ip, &mut ip);
    subnet_bits = ntohl((*rx_message).data.allow_deny.subnet_bits as uint32_t) as libc::c_int;
    if CAM_AddAccessRestriction(&mut ip, subnet_bits, allow, all) == 0 {
        (*tx_message).status = htons(7 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_accheck(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut ip: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    UTI_IPNetworkToHost(&mut (*rx_message).data.ac_check.ip, &mut ip);
    if NCR_CheckAccessRestriction(&mut ip) != 0 {
        (*tx_message).status = htons(8 as libc::c_int as uint16_t)
    } else {
        (*tx_message).status = htons(9 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_cmdaccheck(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut ip: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    UTI_IPNetworkToHost(&mut (*rx_message).data.ac_check.ip, &mut ip);
    if CAM_CheckAccessRestriction(&mut ip) != 0 {
        (*tx_message).status = htons(8 as libc::c_int as uint16_t)
    } else {
        (*tx_message).status = htons(9 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_add_source(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut type_0: NTP_Source_Type = NTP_SERVER;
    let mut params: SourceParameters = SourceParameters {
        minpoll: 0,
        maxpoll: 0,
        connectivity: SRC_OFFLINE,
        auto_offline: 0,
        presend_minpoll: 0,
        burst: 0,
        iburst: 0,
        min_stratum: 0,
        poll_target: 0,
        version: 0,
        max_sources: 0,
        min_samples: 0,
        max_samples: 0,
        filter_length: 0,
        interleaved: 0,
        sel_options: 0,
        authkey: 0,
        max_delay: 0.,
        max_delay_ratio: 0.,
        max_delay_dev_ratio: 0.,
        min_delay: 0.,
        asymmetry: 0.,
        offset: 0.,
    };
    let mut status: NSR_Status = NSR_Success;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pool: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    match ntohl((*rx_message).data.ntp_source.type_0) {
        1 => {
            type_0 = NTP_SERVER;
            pool = 0 as libc::c_int
        }
        2 => {
            type_0 = NTP_PEER;
            pool = 0 as libc::c_int
        }
        3 => {
            type_0 = NTP_SERVER;
            pool = 1 as libc::c_int
        }
        _ => {
            (*tx_message).status = htons(3 as libc::c_int as uint16_t);
            return;
        }
    }
    name = (*rx_message).data.ntp_source.name.as_mut_ptr() as *mut libc::c_char;
    /* Make sure the name is terminated */
    if *name.offset(
        (::std::mem::size_of::<[int8_t; 256]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
    ) as libc::c_int
        != '\u{0}' as i32
    {
        (*tx_message).status = htons(21 as libc::c_int as uint16_t);
        return;
    }
    port = ntohl((*rx_message).data.ntp_source.port) as libc::c_ushort as libc::c_int;
    params.minpoll = ntohl((*rx_message).data.ntp_source.minpoll as uint32_t) as libc::c_int;
    params.maxpoll = ntohl((*rx_message).data.ntp_source.maxpoll as uint32_t) as libc::c_int;
    params.presend_minpoll =
        ntohl((*rx_message).data.ntp_source.presend_minpoll as uint32_t) as libc::c_int;
    params.min_stratum = ntohl((*rx_message).data.ntp_source.min_stratum) as libc::c_int;
    params.poll_target = ntohl((*rx_message).data.ntp_source.poll_target) as libc::c_int;
    params.version = ntohl((*rx_message).data.ntp_source.version) as libc::c_int;
    params.max_sources = ntohl((*rx_message).data.ntp_source.max_sources) as libc::c_int;
    params.min_samples =
        ntohl((*rx_message).data.ntp_source.min_samples as uint32_t) as libc::c_int;
    params.max_samples =
        ntohl((*rx_message).data.ntp_source.max_samples as uint32_t) as libc::c_int;
    params.filter_length =
        ntohl((*rx_message).data.ntp_source.filter_length as uint32_t) as libc::c_int;
    params.authkey = ntohl((*rx_message).data.ntp_source.authkey);
    params.max_delay = UTI_FloatNetworkToHost((*rx_message).data.ntp_source.max_delay);
    params.max_delay_ratio = UTI_FloatNetworkToHost((*rx_message).data.ntp_source.max_delay_ratio);
    params.max_delay_dev_ratio =
        UTI_FloatNetworkToHost((*rx_message).data.ntp_source.max_delay_dev_ratio);
    params.min_delay = UTI_FloatNetworkToHost((*rx_message).data.ntp_source.min_delay);
    params.asymmetry = UTI_FloatNetworkToHost((*rx_message).data.ntp_source.asymmetry);
    params.offset = UTI_FloatNetworkToHost((*rx_message).data.ntp_source.offset);
    params.connectivity =
        if ntohl((*rx_message).data.ntp_source.flags) & 0x1 as libc::c_int as libc::c_uint != 0 {
            SRC_ONLINE as libc::c_int
        } else {
            SRC_OFFLINE as libc::c_int
        } as SRC_Connectivity;
    params.auto_offline =
        if ntohl((*rx_message).data.ntp_source.flags) & 0x2 as libc::c_int as libc::c_uint != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    params.iburst =
        if ntohl((*rx_message).data.ntp_source.flags) & 0x4 as libc::c_int as libc::c_uint != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    params.interleaved =
        if ntohl((*rx_message).data.ntp_source.flags) & 0x80 as libc::c_int as libc::c_uint != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    params.burst =
        if ntohl((*rx_message).data.ntp_source.flags) & 0x100 as libc::c_int as libc::c_uint != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    params.sel_options =
        (if ntohl((*rx_message).data.ntp_source.flags) & 0x8 as libc::c_int as libc::c_uint != 0 {
            0x2 as libc::c_int
        } else {
            0 as libc::c_int
        }) | (if ntohl((*rx_message).data.ntp_source.flags) & 0x10 as libc::c_int as libc::c_uint
            != 0
        {
            0x1 as libc::c_int
        } else {
            0 as libc::c_int
        }) | (if ntohl((*rx_message).data.ntp_source.flags) & 0x20 as libc::c_int as libc::c_uint
            != 0
        {
            0x4 as libc::c_int
        } else {
            0 as libc::c_int
        }) | (if ntohl((*rx_message).data.ntp_source.flags) & 0x40 as libc::c_int as libc::c_uint
            != 0
        {
            0x8 as libc::c_int
        } else {
            0 as libc::c_int
        });
    status = NSR_AddSourceByName(name, port, pool, type_0, &mut params);
    match status as libc::c_uint {
        6 => {
            /* Try to resolve the name now */
            NSR_ResolveSources();
        }
        2 => (*tx_message).status = htons(11 as libc::c_int as uint16_t),
        3 => (*tx_message).status = htons(12 as libc::c_int as uint16_t),
        5 => (*tx_message).status = htons(21 as libc::c_int as uint16_t),
        4 | 1 => {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
                766 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                    b"void handle_add_source(CMD_Request *, CMD_Reply *)\x00",
                ))
                .as_ptr(),
            );
        }
        0 | _ => {}
    };
}
/* ================================================== */
unsafe extern "C" fn handle_del_source(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut rem_addr: NTP_Remote_Address = IPSockAddr {
        ip_addr: IPAddr {
            addr: C2RustUnnamed { in4: 0 },
            family: 0,
            _pad: 0,
        },
        port: 0,
    };
    let mut status: NSR_Status = NSR_Success;
    UTI_IPNetworkToHost(
        &mut (*rx_message).data.del_source.ip_addr,
        &mut rem_addr.ip_addr,
    );
    rem_addr.port = 0 as libc::c_int as uint16_t;
    status = NSR_RemoveSource(&mut rem_addr);
    match status as libc::c_uint {
        1 => (*tx_message).status = htons(4 as libc::c_int as uint16_t),
        3 | 2 | 4 | 5 | 6 => {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
                794 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                    b"void handle_del_source(CMD_Request *, CMD_Reply *)\x00",
                ))
                .as_ptr(),
            );
        }
        0 | _ => {}
    };
}
/* ================================================== */
unsafe extern "C" fn handle_writertc(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    match RTC_WriteParameters() {
        1 => (*tx_message).status = htons(13 as libc::c_int as uint16_t),
        2 => (*tx_message).status = htons(14 as libc::c_int as uint16_t),
        0 | _ => {}
    };
}
/* ================================================== */
unsafe extern "C" fn handle_dfreq(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut dfreq: libc::c_double = 0.;
    dfreq = UTI_FloatNetworkToHost((*rx_message).data.dfreq.dfreq);
    LCL_AccumulateDeltaFrequency(dfreq * 1.0e-6f64);
    LOG_Message(
        LOGS_INFO,
        b"Accumulated delta freq of %.3fppm\x00" as *const u8 as *const libc::c_char,
        dfreq,
    );
}
/* ================================================== */
unsafe extern "C" fn handle_doffset(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut sec: libc::c_long = 0;
    let mut usec: libc::c_long = 0;
    let mut doffset: libc::c_double = 0.;
    sec = ntohl((*rx_message).data.doffset.sec as uint32_t) as int32_t as libc::c_long;
    usec = ntohl((*rx_message).data.doffset.usec as uint32_t) as int32_t as libc::c_long;
    doffset = sec as libc::c_double + 1.0e-6f64 * usec as libc::c_double;
    LOG_Message(
        LOGS_INFO,
        b"Accumulated delta offset of %.6f seconds\x00" as *const u8 as *const libc::c_char,
        doffset,
    );
    LCL_AccumulateOffset(doffset, 0.0f64);
}
/* ================================================== */
unsafe extern "C" fn handle_tracking(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut rpt: RPT_TrackingReport = RPT_TrackingReport {
        ref_id: 0,
        ip_addr: IPAddr {
            addr: C2RustUnnamed { in4: 0 },
            family: 0,
            _pad: 0,
        },
        stratum: 0,
        leap_status: LEAP_Normal,
        ref_time: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        current_correction: 0.,
        last_offset: 0.,
        rms_offset: 0.,
        freq_ppm: 0.,
        resid_freq_ppm: 0.,
        skew_ppm: 0.,
        root_delay: 0.,
        root_dispersion: 0.,
        last_update_interval: 0.,
    };
    REF_GetTrackingReport(&mut rpt);
    (*tx_message).reply = htons(5 as libc::c_int as uint16_t);
    (*tx_message).data.tracking.ref_id = htonl(rpt.ref_id);
    UTI_IPHostToNetwork(&mut rpt.ip_addr, &mut (*tx_message).data.tracking.ip_addr);
    (*tx_message).data.tracking.stratum = htons(rpt.stratum as uint16_t);
    (*tx_message).data.tracking.leap_status = htons(rpt.leap_status as uint16_t);
    UTI_TimespecHostToNetwork(&mut rpt.ref_time, &mut (*tx_message).data.tracking.ref_time);
    (*tx_message).data.tracking.current_correction = UTI_FloatHostToNetwork(rpt.current_correction);
    (*tx_message).data.tracking.last_offset = UTI_FloatHostToNetwork(rpt.last_offset);
    (*tx_message).data.tracking.rms_offset = UTI_FloatHostToNetwork(rpt.rms_offset);
    (*tx_message).data.tracking.freq_ppm = UTI_FloatHostToNetwork(rpt.freq_ppm);
    (*tx_message).data.tracking.resid_freq_ppm = UTI_FloatHostToNetwork(rpt.resid_freq_ppm);
    (*tx_message).data.tracking.skew_ppm = UTI_FloatHostToNetwork(rpt.skew_ppm);
    (*tx_message).data.tracking.root_delay = UTI_FloatHostToNetwork(rpt.root_delay);
    (*tx_message).data.tracking.root_dispersion = UTI_FloatHostToNetwork(rpt.root_dispersion);
    (*tx_message).data.tracking.last_update_interval =
        UTI_FloatHostToNetwork(rpt.last_update_interval);
}
/* ================================================== */
unsafe extern "C" fn handle_smoothing(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut report: RPT_SmoothingReport = RPT_SmoothingReport {
        active: 0,
        leap_only: 0,
        offset: 0.,
        freq_ppm: 0.,
        wander_ppm: 0.,
        last_update_ago: 0.,
        remaining_time: 0.,
    };
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    SCH_GetLastEventTime(&mut now, 0 as *mut libc::c_double, 0 as *mut timespec);
    if SMT_GetSmoothingReport(&mut report, &mut now) == 0 {
        (*tx_message).status = htons(6 as libc::c_int as uint16_t);
        return;
    }
    (*tx_message).reply = htons(13 as libc::c_int as uint16_t);
    (*tx_message).data.smoothing.flags = htonl(
        ((if report.active != 0 {
            0x1 as libc::c_int
        } else {
            0 as libc::c_int
        }) | (if report.leap_only != 0 {
            0x2 as libc::c_int
        } else {
            0 as libc::c_int
        })) as uint32_t,
    );
    (*tx_message).data.smoothing.offset = UTI_FloatHostToNetwork(report.offset);
    (*tx_message).data.smoothing.freq_ppm = UTI_FloatHostToNetwork(report.freq_ppm);
    (*tx_message).data.smoothing.wander_ppm = UTI_FloatHostToNetwork(report.wander_ppm);
    (*tx_message).data.smoothing.last_update_ago = UTI_FloatHostToNetwork(report.last_update_ago);
    (*tx_message).data.smoothing.remaining_time = UTI_FloatHostToNetwork(report.remaining_time);
}
/* ================================================== */
unsafe extern "C" fn handle_smoothtime(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut option: libc::c_int = 0;
    if SMT_IsEnabled() == 0 {
        (*tx_message).status = htons(6 as libc::c_int as uint16_t);
        return;
    }
    option = ntohl((*rx_message).data.smoothtime.option as uint32_t) as libc::c_int;
    SCH_GetLastEventTime(&mut now, 0 as *mut libc::c_double, 0 as *mut timespec);
    match option {
        0 => {
            SMT_Reset(&mut now);
        }
        1 => {
            SMT_Activate(&mut now);
        }
        _ => (*tx_message).status = htons(3 as libc::c_int as uint16_t),
    };
}
/* ================================================== */
unsafe extern "C" fn handle_sourcestats(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut status: libc::c_int = 0;
    let mut report: RPT_SourcestatsReport = RPT_SourcestatsReport {
        ref_id: 0,
        ip_addr: IPAddr {
            addr: C2RustUnnamed { in4: 0 },
            family: 0,
            _pad: 0,
        },
        n_samples: 0,
        n_runs: 0,
        span_seconds: 0,
        resid_freq_ppm: 0.,
        skew_ppm: 0.,
        sd: 0.,
        est_offset: 0.,
        est_offset_err: 0.,
    };
    let mut now_corr: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    SCH_GetLastEventTime(&mut now_corr, 0 as *mut libc::c_double, 0 as *mut timespec);
    status = SRC_ReportSourcestats(
        ntohl((*rx_message).data.sourcestats.index) as libc::c_int,
        &mut report,
        &mut now_corr,
    );
    if status != 0 {
        (*tx_message).reply = htons(6 as libc::c_int as uint16_t);
        (*tx_message).data.sourcestats.ref_id = htonl(report.ref_id);
        UTI_IPHostToNetwork(
            &mut report.ip_addr,
            &mut (*tx_message).data.sourcestats.ip_addr,
        );
        (*tx_message).data.sourcestats.n_samples = htonl(report.n_samples as uint32_t);
        (*tx_message).data.sourcestats.n_runs = htonl(report.n_runs as uint32_t);
        (*tx_message).data.sourcestats.span_seconds = htonl(report.span_seconds as uint32_t);
        (*tx_message).data.sourcestats.resid_freq_ppm =
            UTI_FloatHostToNetwork(report.resid_freq_ppm);
        (*tx_message).data.sourcestats.skew_ppm = UTI_FloatHostToNetwork(report.skew_ppm);
        (*tx_message).data.sourcestats.sd = UTI_FloatHostToNetwork(report.sd);
        (*tx_message).data.sourcestats.est_offset = UTI_FloatHostToNetwork(report.est_offset);
        (*tx_message).data.sourcestats.est_offset_err =
            UTI_FloatHostToNetwork(report.est_offset_err)
    } else {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_rtcreport(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut status: libc::c_int = 0;
    let mut report: RPT_RTC_Report = RPT_RTC_Report {
        ref_time: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        n_samples: 0,
        n_runs: 0,
        span_seconds: 0,
        rtc_seconds_fast: 0.,
        rtc_gain_rate_ppm: 0.,
    };
    status = RTC_GetReport(&mut report);
    if status != 0 {
        (*tx_message).reply = htons(7 as libc::c_int as uint16_t);
        UTI_TimespecHostToNetwork(&mut report.ref_time, &mut (*tx_message).data.rtc.ref_time);
        (*tx_message).data.rtc.n_samples = htons(report.n_samples);
        (*tx_message).data.rtc.n_runs = htons(report.n_runs);
        (*tx_message).data.rtc.span_seconds = htonl(report.span_seconds as uint32_t);
        (*tx_message).data.rtc.rtc_seconds_fast = UTI_FloatHostToNetwork(report.rtc_seconds_fast);
        (*tx_message).data.rtc.rtc_gain_rate_ppm = UTI_FloatHostToNetwork(report.rtc_gain_rate_ppm)
    } else {
        (*tx_message).status = htons(13 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_trimrtc(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    if RTC_Trim() == 0 {
        (*tx_message).status = htons(13 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_cyclelogs(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    LOG_CycleLogFiles();
}
/* ================================================== */
unsafe extern "C" fn handle_client_accesses_by_index(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut report: RPT_ClientAccessByIndex_Report = RPT_ClientAccessByIndex_Report {
        ip_addr: IPAddr {
            addr: C2RustUnnamed { in4: 0 },
            family: 0,
            _pad: 0,
        },
        ntp_hits: 0,
        cmd_hits: 0,
        ntp_drops: 0,
        cmd_drops: 0,
        ntp_interval: 0,
        cmd_interval: 0,
        ntp_timeout_interval: 0,
        last_ntp_hit_ago: 0,
        last_cmd_hit_ago: 0,
    };
    let mut client: *mut RPY_ClientAccesses_Client = 0 as *mut RPY_ClientAccesses_Client;
    let mut n_indices: libc::c_int = 0;
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut req_first_index: uint32_t = 0;
    let mut req_n_clients: uint32_t = 0;
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    SCH_GetLastEventTime(&mut now, 0 as *mut libc::c_double, 0 as *mut timespec);
    req_first_index = ntohl((*rx_message).data.client_accesses_by_index.first_index);
    req_n_clients = ntohl((*rx_message).data.client_accesses_by_index.n_clients);
    if req_n_clients > 8 as libc::c_int as libc::c_uint {
        req_n_clients = 8 as libc::c_int as uint32_t
    }
    n_indices = CLG_GetNumberOfIndices();
    if n_indices < 0 as libc::c_int {
        (*tx_message).status = htons(15 as libc::c_int as uint16_t);
        return;
    }
    (*tx_message).reply = htons(15 as libc::c_int as uint16_t);
    (*tx_message).data.client_accesses_by_index.n_indices = htonl(n_indices as uint32_t);
    i = req_first_index;
    j = 0 as libc::c_int as uint32_t;
    while i < n_indices as uint32_t && j < req_n_clients {
        if !(CLG_GetClientAccessReportByIndex(i as libc::c_int, &mut report, &mut now) == 0) {
            let fresh0 = j;
            j = j.wrapping_add(1);
            client = &mut *(*tx_message)
                .data
                .client_accesses_by_index
                .clients
                .as_mut_ptr()
                .offset(fresh0 as isize) as *mut RPY_ClientAccesses_Client;
            UTI_IPHostToNetwork(&mut report.ip_addr, &mut (*client).ip);
            (*client).ntp_hits = htonl(report.ntp_hits);
            (*client).cmd_hits = htonl(report.cmd_hits);
            (*client).ntp_drops = htonl(report.ntp_drops as uint32_t);
            (*client).cmd_drops = htonl(report.cmd_drops as uint32_t);
            (*client).ntp_interval = report.ntp_interval;
            (*client).cmd_interval = report.cmd_interval;
            (*client).ntp_timeout_interval = report.ntp_timeout_interval;
            (*client).last_ntp_hit_ago = htonl(report.last_ntp_hit_ago);
            (*client).last_cmd_hit_ago = htonl(report.last_cmd_hit_ago)
        }
        i = i.wrapping_add(1)
    }
    (*tx_message).data.client_accesses_by_index.next_index = htonl(i);
    (*tx_message).data.client_accesses_by_index.n_clients = htonl(j);
}
/* ================================================== */
unsafe extern "C" fn handle_manual_list(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut n_samples: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut sample: *mut RPY_ManualListSample = 0 as *mut RPY_ManualListSample;
    let mut report: [RPT_ManualSamplesReport; 16] = [RPT_ManualSamplesReport {
        when: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        slewed_offset: 0.,
        orig_offset: 0.,
        residual: 0.,
    }; 16];
    (*tx_message).reply = htons(18 as libc::c_int as uint16_t);
    MNL_ReportSamples(report.as_mut_ptr(), 16 as libc::c_int, &mut n_samples);
    (*tx_message).data.manual_list.n_samples = htonl(n_samples as uint32_t);
    i = 0 as libc::c_int;
    while i < n_samples {
        sample = &mut *(*tx_message)
            .data
            .manual_list
            .samples
            .as_mut_ptr()
            .offset(i as isize) as *mut RPY_ManualListSample;
        UTI_TimespecHostToNetwork(
            &mut (*report.as_mut_ptr().offset(i as isize)).when,
            &mut (*sample).when,
        );
        (*sample).slewed_offset = UTI_FloatHostToNetwork(report[i as usize].slewed_offset);
        (*sample).orig_offset = UTI_FloatHostToNetwork(report[i as usize].orig_offset);
        (*sample).residual = UTI_FloatHostToNetwork(report[i as usize].residual);
        i += 1
    }
}
/* ================================================== */
unsafe extern "C" fn handle_manual_delete(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut index: libc::c_int = 0;
    index = ntohl((*rx_message).data.manual_delete.index as uint32_t) as libc::c_int;
    if MNL_DeleteSample(index) == 0 {
        (*tx_message).status = htons(16 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_make_step(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    if LCL_MakeStep() == 0 {
        (*tx_message).status = htons(1 as libc::c_int as uint16_t)
    };
}
/* ================================================== */
unsafe extern "C" fn handle_activity(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut report: RPT_ActivityReport = RPT_ActivityReport {
        online: 0,
        offline: 0,
        burst_online: 0,
        burst_offline: 0,
        unresolved: 0,
    };
    NSR_GetActivityReport(&mut report);
    (*tx_message).data.activity.online = htonl(report.online as uint32_t) as int32_t;
    (*tx_message).data.activity.offline = htonl(report.offline as uint32_t) as int32_t;
    (*tx_message).data.activity.burst_online = htonl(report.burst_online as uint32_t) as int32_t;
    (*tx_message).data.activity.burst_offline = htonl(report.burst_offline as uint32_t) as int32_t;
    (*tx_message).data.activity.unresolved = htonl(report.unresolved as uint32_t) as int32_t;
    (*tx_message).reply = htons(12 as libc::c_int as uint16_t);
}
/* ================================================== */
unsafe extern "C" fn handle_reselect_distance(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut dist: libc::c_double = 0.;
    dist = UTI_FloatNetworkToHost((*rx_message).data.reselect_distance.distance);
    SRC_SetReselectDistance(dist);
}
/* ================================================== */
unsafe extern "C" fn handle_reselect(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    SRC_ReselectSource();
}
/* ================================================== */
unsafe extern "C" fn handle_refresh(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    NSR_RefreshAddresses();
}
/* ================================================== */
unsafe extern "C" fn handle_server_stats(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut report: RPT_ServerStatsReport = RPT_ServerStatsReport {
        ntp_hits: 0,
        cmd_hits: 0,
        ntp_drops: 0,
        cmd_drops: 0,
        log_drops: 0,
    };
    CLG_GetServerStatsReport(&mut report);
    (*tx_message).reply = htons(14 as libc::c_int as uint16_t);
    (*tx_message).data.server_stats.ntp_hits = htonl(report.ntp_hits);
    (*tx_message).data.server_stats.cmd_hits = htonl(report.cmd_hits);
    (*tx_message).data.server_stats.ntp_drops = htonl(report.ntp_drops);
    (*tx_message).data.server_stats.cmd_drops = htonl(report.cmd_drops);
    (*tx_message).data.server_stats.log_drops = htonl(report.log_drops);
}
/* ================================================== */
unsafe extern "C" fn handle_ntp_data(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut report: RPT_NTPReport = RPT_NTPReport {
        remote_addr: IPAddr {
            addr: C2RustUnnamed { in4: 0 },
            family: 0,
            _pad: 0,
        },
        local_addr: IPAddr {
            addr: C2RustUnnamed { in4: 0 },
            family: 0,
            _pad: 0,
        },
        remote_port: 0,
        leap: 0,
        version: 0,
        mode: 0,
        stratum: 0,
        poll: 0,
        precision: 0,
        root_delay: 0.,
        root_dispersion: 0.,
        ref_id: 0,
        ref_time: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        offset: 0.,
        peer_delay: 0.,
        peer_dispersion: 0.,
        response_time: 0.,
        jitter_asymmetry: 0.,
        tests: 0,
        interleaved: 0,
        authenticated: 0,
        tx_tss_char: 0,
        rx_tss_char: 0,
        total_tx_count: 0,
        total_rx_count: 0,
        total_valid_count: 0,
    };
    UTI_IPNetworkToHost(
        &mut (*rx_message).data.ntp_data.ip_addr,
        &mut report.remote_addr,
    );
    if NSR_GetNTPReport(&mut report) == 0 {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t);
        return;
    }
    (*tx_message).reply = htons(16 as libc::c_int as uint16_t);
    UTI_IPHostToNetwork(
        &mut report.remote_addr,
        &mut (*tx_message).data.ntp_data.remote_addr,
    );
    UTI_IPHostToNetwork(
        &mut report.local_addr,
        &mut (*tx_message).data.ntp_data.local_addr,
    );
    (*tx_message).data.ntp_data.remote_port = htons(report.remote_port);
    (*tx_message).data.ntp_data.leap = report.leap;
    (*tx_message).data.ntp_data.version = report.version;
    (*tx_message).data.ntp_data.mode = report.mode;
    (*tx_message).data.ntp_data.stratum = report.stratum;
    (*tx_message).data.ntp_data.poll = report.poll;
    (*tx_message).data.ntp_data.precision = report.precision;
    (*tx_message).data.ntp_data.root_delay = UTI_FloatHostToNetwork(report.root_delay);
    (*tx_message).data.ntp_data.root_dispersion = UTI_FloatHostToNetwork(report.root_dispersion);
    (*tx_message).data.ntp_data.ref_id = htonl(report.ref_id);
    UTI_TimespecHostToNetwork(
        &mut report.ref_time,
        &mut (*tx_message).data.ntp_data.ref_time,
    );
    (*tx_message).data.ntp_data.offset = UTI_FloatHostToNetwork(report.offset);
    (*tx_message).data.ntp_data.peer_delay = UTI_FloatHostToNetwork(report.peer_delay);
    (*tx_message).data.ntp_data.peer_dispersion = UTI_FloatHostToNetwork(report.peer_dispersion);
    (*tx_message).data.ntp_data.response_time = UTI_FloatHostToNetwork(report.response_time);
    (*tx_message).data.ntp_data.jitter_asymmetry = UTI_FloatHostToNetwork(report.jitter_asymmetry);
    (*tx_message).data.ntp_data.flags = htons(
        (report.tests as libc::c_int & 0x3ff as libc::c_int
            | (if report.interleaved != 0 {
                0x4000 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if report.authenticated != 0 {
                0x8000 as libc::c_int
            } else {
                0 as libc::c_int
            })) as uint16_t,
    );
    (*tx_message).data.ntp_data.tx_tss_char = report.tx_tss_char as uint8_t;
    (*tx_message).data.ntp_data.rx_tss_char = report.rx_tss_char as uint8_t;
    (*tx_message).data.ntp_data.total_tx_count = htonl(report.total_tx_count);
    (*tx_message).data.ntp_data.total_rx_count = htonl(report.total_rx_count);
    (*tx_message).data.ntp_data.total_valid_count = htonl(report.total_valid_count);
    memset(
        (*tx_message).data.ntp_data.reserved.as_mut_ptr() as *mut libc::c_void,
        0xff as libc::c_int,
        ::std::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong,
    );
}
/* ================================================== */
unsafe extern "C" fn handle_shutdown(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    LOG_Message(
        LOGS_INFO,
        b"Received shutdown command\x00" as *const u8 as *const libc::c_char,
    );
    SCH_QuitProgram();
}
/* ================================================== */
unsafe extern "C" fn handle_ntp_source_name(
    mut rx_message: *mut CMD_Request,
    mut tx_message: *mut CMD_Reply,
) {
    let mut addr: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    UTI_IPNetworkToHost(&mut (*rx_message).data.ntp_data.ip_addr, &mut addr);
    name = NSR_GetName(&mut addr);
    if name.is_null() {
        (*tx_message).status = htons(4 as libc::c_int as uint16_t);
        return;
    }
    (*tx_message).reply = htons(19 as libc::c_int as uint16_t);
    /* Avoid compiler warning */
    if strlen(name) >= ::std::mem::size_of::<[int8_t; 256]>() as libc::c_ulong {
        memcpy(
            (*tx_message).data.ntp_source_name.name.as_mut_ptr() as *mut libc::c_void,
            name as *const libc::c_void,
            ::std::mem::size_of::<[int8_t; 256]>() as libc::c_ulong,
        );
    } else {
        strncpy(
            (*tx_message).data.ntp_source_name.name.as_mut_ptr() as *mut libc::c_char,
            name,
            ::std::mem::size_of::<[int8_t; 256]>() as libc::c_ulong,
        );
    };
}
/* ================================================== */
/* Forward prototypes */
/* ================================================== */
/* Read a packet and process it */
unsafe extern "C" fn read_from_cmd_socket(
    mut sock_fd: libc::c_int,
    mut event: libc::c_int,
    mut anything: *mut libc::c_void,
) {
    let mut sck_message: SCK_Message = SCK_Message {
        data: 0 as *mut libc::c_void,
        length: 0,
        addr_type: SCK_ADDR_UNSPEC,
        if_index: 0,
        remote_addr: C2RustUnnamed_2 {
            ip: IPSockAddr {
                ip_addr: IPAddr {
                    addr: C2RustUnnamed { in4: 0 },
                    family: 0,
                    _pad: 0,
                },
                port: 0,
            },
        },
        local_addr: C2RustUnnamed_1 {
            ip: IPAddr {
                addr: C2RustUnnamed { in4: 0 },
                family: 0,
                _pad: 0,
            },
        },
        timestamp: C2RustUnnamed_0 {
            kernel: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            hw: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            if_index: 0,
            l2_length: 0,
            tx_flags: 0,
        },
        descriptor: 0,
    };
    let mut rx_message: CMD_Request = CMD_Request {
        version: 0,
        pkt_type: 0,
        res1: 0,
        res2: 0,
        command: 0,
        attempt: 0,
        sequence: 0,
        pad1: 0,
        pad2: 0,
        data: C2RustUnnamed_4 {
            null: REQ_Null { EOR: 0 },
        },
        padding: [0; 396],
    };
    let mut tx_message: CMD_Reply = CMD_Reply {
        version: 0,
        pkt_type: 0,
        res1: 0,
        res2: 0,
        command: 0,
        reply: 0,
        status: 0,
        pad1: 0,
        pad2: 0,
        pad3: 0,
        sequence: 0,
        pad4: 0,
        pad5: 0,
        data: C2RustUnnamed_3 {
            null: RPY_Null { EOR: 0 },
        },
    };
    let mut loopback_addr: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    let mut remote_ip: IPAddr = IPAddr {
        addr: C2RustUnnamed { in4: 0 },
        family: 0,
        _pad: 0,
    };
    let mut read_length: libc::c_int = 0;
    let mut expected_length: libc::c_int = 0;
    let mut localhost: libc::c_int = 0;
    let mut allowed: libc::c_int = 0;
    let mut log_index: libc::c_int = 0;
    let mut rx_command: libc::c_ushort = 0;
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut cooked_now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if SCK_ReceiveMessage(sock_fd, &mut sck_message, 0 as libc::c_int) == 0 {
        return;
    }
    read_length = sck_message.length as libc::c_int;
    /* Get current time cheaply */
    SCH_GetLastEventTime(&mut cooked_now, 0 as *mut libc::c_double, &mut now);
    /* Check if it's from localhost (127.0.0.1, ::1, or Unix domain),
    or an authorised address */
    match sck_message.addr_type as libc::c_uint {
        1 => {
            if sock_fd == sock_fd4 || sock_fd == sock_fd6 {
            } else {
                __assert_fail(
                    b"sock_fd == sock_fd4 || sock_fd == sock_fd6\x00" as *const u8
                        as *const libc::c_char,
                    b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
                    1246 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"void read_from_cmd_socket(int, int, void *)\x00",
                    ))
                    .as_ptr(),
                );
            }
            remote_ip = sck_message.remote_addr.ip.ip_addr;
            SCK_GetLoopbackIPAddress(remote_ip.family as libc::c_int, &mut loopback_addr);
            localhost = (UTI_CompareIPs(&mut remote_ip, &mut loopback_addr, 0 as *mut IPAddr)
                == 0 as libc::c_int) as libc::c_int;
            if localhost == 0 && ADF_IsAllowed(access_auth_table, &mut remote_ip) == 0 {
                if 0 as libc::c_int != 0
                    && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
                {
                    LOG_Message(
                        LOGS_DEBUG,
                        b"Unauthorised host %s\x00" as *const u8 as *const libc::c_char,
                        UTI_IPSockAddrToString(&mut sck_message.remote_addr.ip),
                    );
                }
                return;
            }
            if remote_ip.family as libc::c_int != 0 as libc::c_int {
            } else {
                __assert_fail(
                    b"remote_ip.family != IPADDR_UNSPEC\x00" as *const u8 as *const libc::c_char,
                    b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
                    1257 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"void read_from_cmd_socket(int, int, void *)\x00",
                    ))
                    .as_ptr(),
                );
            }
        }
        2 => {
            if sock_fd == sock_fdu {
            } else {
                __assert_fail(
                    b"sock_fd == sock_fdu\x00" as *const u8 as *const libc::c_char,
                    b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
                    1261 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"void read_from_cmd_socket(int, int, void *)\x00",
                    ))
                    .as_ptr(),
                );
            }
            remote_ip.family = 0 as libc::c_int as uint16_t;
            localhost = 1 as libc::c_int
        }
        _ => {
            if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
            {
                LOG_Message(
                    LOGS_DEBUG,
                    b"Unexpected address type\x00" as *const u8 as *const libc::c_char,
                );
            }
            return;
        }
    }
    if (read_length as libc::c_ulong) < 20 as libc::c_ulong
        || (read_length as libc::c_ulong) < 28 as libc::c_ulong
        || read_length as libc::c_ulong > ::std::mem::size_of::<CMD_Request>() as libc::c_ulong
    {
        /* We don't know how to process anything like this or an error reply
        would be larger than the request */
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Unexpected length\x00" as *const u8 as *const libc::c_char,
            );
        }
        return;
    }
    memcpy(
        &mut rx_message as *mut CMD_Request as *mut libc::c_void,
        sck_message.data,
        read_length as libc::c_ulong,
    );
    if rx_message.pkt_type as libc::c_int != 1 as libc::c_int
        || rx_message.res1 as libc::c_int != 0 as libc::c_int
        || rx_message.res2 as libc::c_int != 0 as libc::c_int
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Command packet dropped\x00" as *const u8 as *const libc::c_char,
            );
        }
        return;
    }
    log_index = CLG_LogCommandAccess(&mut remote_ip, &mut cooked_now);
    /* Don't reply to all requests from hosts other than localhost if the rate
    is excessive */
    if localhost == 0
        && log_index >= 0 as libc::c_int
        && CLG_LimitCommandResponseRate(log_index) != 0
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Command packet discarded to limit response rate\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        return;
    }
    expected_length = PKL_CommandLength(&mut rx_message);
    rx_command = ntohs(rx_message.command);
    memset(
        &mut tx_message as *mut CMD_Reply as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<CMD_Reply>() as libc::c_ulong,
    );
    sck_message.data = &mut tx_message as *mut CMD_Reply as *mut libc::c_void;
    sck_message.length = 0 as libc::c_int as libc::c_uint;
    tx_message.version = 6 as libc::c_int as uint8_t;
    tx_message.pkt_type = 2 as libc::c_int as uint8_t;
    tx_message.command = rx_message.command;
    tx_message.reply = htons(1 as libc::c_int as uint16_t);
    tx_message.status = htons(0 as libc::c_int as uint16_t);
    tx_message.sequence = rx_message.sequence;
    if rx_message.version as libc::c_int != 6 as libc::c_int {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Command packet has invalid version (%d != %d)\x00" as *const u8
                    as *const libc::c_char,
                rx_message.version as libc::c_int,
                6 as libc::c_int,
            );
        }
        if rx_message.version as libc::c_int >= 5 as libc::c_int {
            tx_message.status = htons(18 as libc::c_int as uint16_t);
            transmit_reply(sock_fd, &mut sck_message);
        }
        return;
    }
    if rx_command as libc::c_int >= 66 as libc::c_int
        || expected_length < 20 as libc::c_ulong as libc::c_int
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Command packet has invalid command %d\x00" as *const u8 as *const libc::c_char,
                rx_command as libc::c_int,
            );
        }
        tx_message.status = htons(3 as libc::c_int as uint16_t);
        transmit_reply(sock_fd, &mut sck_message);
        return;
    }
    if read_length < expected_length {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"Command packet is too short (%d < %d)\x00" as *const u8 as *const libc::c_char,
                read_length,
                expected_length,
            );
        }
        tx_message.status = htons(19 as libc::c_int as uint16_t);
        transmit_reply(sock_fd, &mut sck_message);
        return;
    }
    /* OK, we have a valid message.  Now dispatch on message type and process it. */
    if rx_command as libc::c_int >= 66 as libc::c_int {
        /* This should be already handled */
        __assert_fail(
            b"0\x00" as *const u8 as *const libc::c_char,
            b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
            1344 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"void read_from_cmd_socket(int, int, void *)\x00",
            ))
            .as_ptr(),
        );
    } else {
        /* Check level of authority required to issue the command.  All commands
        from the Unix domain socket (which is accessible only by the root and
        chrony user/group) are allowed. */
        if remote_ip.family as libc::c_int == 0 as libc::c_int {
            if sock_fd == sock_fdu {
            } else {
                __assert_fail(
                    b"sock_fd == sock_fdu\x00" as *const u8 as *const libc::c_char,
                    b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
                    1350 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"void read_from_cmd_socket(int, int, void *)\x00",
                    ))
                    .as_ptr(),
                );
            }
            allowed = 1 as libc::c_int
        } else {
            match permissions[rx_command as usize] as libc::c_int {
                2 => allowed = 0 as libc::c_int,
                1 => allowed = localhost,
                0 => allowed = 1 as libc::c_int,
                _ => {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"cmdmon.c\x00" as *const u8 as *const libc::c_char,
                        1364 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                            b"void read_from_cmd_socket(int, int, void *)\x00",
                        ))
                        .as_ptr(),
                    );
                    allowed = 0 as libc::c_int
                }
            }
        }
        if allowed != 0 {
            match rx_command as libc::c_int {
                0 => {}
                6 => {
                    handle_dump(&mut rx_message, &mut tx_message);
                }
                1 => {
                    handle_online(&mut rx_message, &mut tx_message);
                }
                2 => {
                    handle_offline(&mut rx_message, &mut tx_message);
                }
                3 => {
                    handle_burst(&mut rx_message, &mut tx_message);
                }
                4 => {
                    handle_modify_minpoll(&mut rx_message, &mut tx_message);
                }
                5 => {
                    handle_modify_maxpoll(&mut rx_message, &mut tx_message);
                }
                7 => {
                    handle_modify_maxdelay(&mut rx_message, &mut tx_message);
                }
                8 => {
                    handle_modify_maxdelayratio(&mut rx_message, &mut tx_message);
                }
                47 => {
                    handle_modify_maxdelaydevratio(&mut rx_message, &mut tx_message);
                }
                9 => {
                    handle_modify_maxupdateskew(&mut rx_message, &mut tx_message);
                }
                50 => {
                    handle_modify_makestep(&mut rx_message, &mut tx_message);
                }
                10 => {
                    /* Authentication is no longer supported, log-on always fails */
                    tx_message.status = htons(1 as libc::c_int as uint16_t)
                }
                11 => {
                    handle_settime(&mut rx_message, &mut tx_message);
                }
                56 => {
                    handle_local(&mut rx_message, &mut tx_message);
                }
                13 => {
                    handle_manual(&mut rx_message, &mut tx_message);
                }
                14 => {
                    handle_n_sources(&mut rx_message, &mut tx_message);
                }
                15 => {
                    handle_source_data(&mut rx_message, &mut tx_message);
                }
                16 => {
                    handle_rekey(&mut rx_message, &mut tx_message);
                }
                17 => {
                    handle_allowdeny(
                        &mut rx_message,
                        &mut tx_message,
                        1 as libc::c_int,
                        0 as libc::c_int,
                    );
                }
                18 => {
                    handle_allowdeny(
                        &mut rx_message,
                        &mut tx_message,
                        1 as libc::c_int,
                        1 as libc::c_int,
                    );
                }
                19 => {
                    handle_allowdeny(
                        &mut rx_message,
                        &mut tx_message,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                }
                20 => {
                    handle_allowdeny(
                        &mut rx_message,
                        &mut tx_message,
                        0 as libc::c_int,
                        1 as libc::c_int,
                    );
                }
                21 => {
                    handle_cmdallowdeny(
                        &mut rx_message,
                        &mut tx_message,
                        1 as libc::c_int,
                        0 as libc::c_int,
                    );
                }
                22 => {
                    handle_cmdallowdeny(
                        &mut rx_message,
                        &mut tx_message,
                        1 as libc::c_int,
                        1 as libc::c_int,
                    );
                }
                23 => {
                    handle_cmdallowdeny(
                        &mut rx_message,
                        &mut tx_message,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                }
                24 => {
                    handle_cmdallowdeny(
                        &mut rx_message,
                        &mut tx_message,
                        0 as libc::c_int,
                        1 as libc::c_int,
                    );
                }
                25 => {
                    handle_accheck(&mut rx_message, &mut tx_message);
                }
                26 => {
                    handle_cmdaccheck(&mut rx_message, &mut tx_message);
                }
                64 => {
                    handle_add_source(&mut rx_message, &mut tx_message);
                }
                29 => {
                    handle_del_source(&mut rx_message, &mut tx_message);
                }
                30 => {
                    handle_writertc(&mut rx_message, &mut tx_message);
                }
                31 => {
                    handle_dfreq(&mut rx_message, &mut tx_message);
                }
                32 => {
                    handle_doffset(&mut rx_message, &mut tx_message);
                }
                33 => {
                    handle_tracking(&mut rx_message, &mut tx_message);
                }
                51 => {
                    handle_smoothing(&mut rx_message, &mut tx_message);
                }
                52 => {
                    handle_smoothtime(&mut rx_message, &mut tx_message);
                }
                34 => {
                    handle_sourcestats(&mut rx_message, &mut tx_message);
                }
                35 => {
                    handle_rtcreport(&mut rx_message, &mut tx_message);
                }
                36 => {
                    handle_trimrtc(&mut rx_message, &mut tx_message);
                }
                37 => {
                    handle_cyclelogs(&mut rx_message, &mut tx_message);
                }
                55 => {
                    handle_client_accesses_by_index(&mut rx_message, &mut tx_message);
                }
                41 => {
                    handle_manual_list(&mut rx_message, &mut tx_message);
                }
                42 => {
                    handle_manual_delete(&mut rx_message, &mut tx_message);
                }
                43 => {
                    handle_make_step(&mut rx_message, &mut tx_message);
                }
                44 => {
                    handle_activity(&mut rx_message, &mut tx_message);
                }
                49 => {
                    handle_reselect_distance(&mut rx_message, &mut tx_message);
                }
                48 => {
                    handle_reselect(&mut rx_message, &mut tx_message);
                }
                45 => {
                    handle_modify_minstratum(&mut rx_message, &mut tx_message);
                }
                46 => {
                    handle_modify_polltarget(&mut rx_message, &mut tx_message);
                }
                53 => {
                    handle_refresh(&mut rx_message, &mut tx_message);
                }
                54 => {
                    handle_server_stats(&mut rx_message, &mut tx_message);
                }
                57 => {
                    handle_ntp_data(&mut rx_message, &mut tx_message);
                }
                62 => {
                    handle_shutdown(&mut rx_message, &mut tx_message);
                }
                63 => {
                    handle_onoffline(&mut rx_message, &mut tx_message);
                }
                65 => {
                    handle_ntp_source_name(&mut rx_message, &mut tx_message);
                }
                _ => {
                    if 0 as libc::c_int != 0
                        && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
                    {
                        LOG_Message(
                            LOGS_DEBUG,
                            b"Unhandled command %d\x00" as *const u8 as *const libc::c_char,
                            rx_command as libc::c_int,
                        );
                    }
                    tx_message.status = htons(1 as libc::c_int as uint16_t)
                }
            }
        } else {
            tx_message.status = htons(2 as libc::c_int as uint16_t)
        }
    }
    /* Transmit the response */
    /* Include a simple way to lose one message in three to test resend */
    static mut do_it: libc::c_int = 1 as libc::c_int;
    if do_it != 0 {
        transmit_reply(sock_fd, &mut sck_message);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CAM_AddAccessRestriction(
    mut ip_addr: *mut IPAddr,
    mut subnet_bits: libc::c_int,
    mut allow: libc::c_int,
    mut all: libc::c_int,
) -> libc::c_int {
    let mut status: ADF_Status = ADF_SUCCESS;
    if allow != 0 {
        if all != 0 {
            status = ADF_AllowAll(access_auth_table, ip_addr, subnet_bits)
        } else {
            status = ADF_Allow(access_auth_table, ip_addr, subnet_bits)
        }
    } else if all != 0 {
        status = ADF_DenyAll(access_auth_table, ip_addr, subnet_bits)
    } else {
        status = ADF_Deny(access_auth_table, ip_addr, subnet_bits)
    }
    if status as libc::c_uint == ADF_BADSUBNET as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    } else if status as libc::c_uint == ADF_SUCCESS as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CAM_CheckAccessRestriction(mut ip_addr: *mut IPAddr) -> libc::c_int {
    return ADF_IsAllowed(access_auth_table, ip_addr);
}
/* ================================================== */
/* ================================================== */
