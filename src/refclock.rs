#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           register_tool)]
extern "C" {
    pub type ARR_Instance_Record;
    pub type SRC_Instance_Record;
    pub type SPF_Instance_Record;
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
    fn ARR_GetElement(array: ARR_Instance, index: libc::c_uint) -> *mut libc::c_void;
    /* Return current size of the array */
    #[no_mangle]
    fn ARR_GetSize(array: ARR_Instance) -> libc::c_uint;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn SRC_SelectSource(updated_inst: SRC_Instance);
    #[no_mangle]
    fn SRC_UpdateReachability(inst: SRC_Instance, reachable: libc::c_int);
    #[no_mangle]
    fn SRC_SetActive(inst: SRC_Instance);
    #[no_mangle]
    fn SRC_AccumulateSample(instance: SRC_Instance, sample: *mut NTP_Sample);
    #[no_mangle]
    fn SRC_DestroyInstance(instance: SRC_Instance);
    #[no_mangle]
    fn SRC_CreateNewInstance(
        ref_id: uint32_t,
        type_0: SRC_Type,
        sel_options: libc::c_int,
        addr: *mut IPAddr,
        min_samples: libc::c_int,
        max_samples: libc::c_int,
        min_delay: libc::c_double,
        asymmetry: libc::c_double,
    ) -> SRC_Instance;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    /* Function which takes a local cooked time and returns the estimated
    time of the reference.  It also returns the other parameters
    required for forming the outgoing NTP packet.

    local_time is the cooked local time returned by the LCL module

    is_synchronised indicates whether we are synchronised to anything
    at the moment.

    leap indicates the current leap status

    stratum is the stratum of this machine, when considered to be sync'd to the
    reference

    ref_id is the reference_id of the source

    ref_time is the time at which the we last set the reference source up

    root_delay is the root delay of the sample we are using

    root_dispersion is the root dispersion of the sample we are using, with all the
    skew etc added on.

    */
    #[no_mangle]
    fn REF_GetReferenceParams(
        local_time: *mut timespec,
        is_synchronised: *mut libc::c_int,
        leap: *mut NTP_Leap,
        stratum: *mut libc::c_int,
        ref_id: *mut uint32_t,
        ref_time: *mut timespec,
        root_delay: *mut libc::c_double,
        root_dispersion: *mut libc::c_double,
    );
    /* Return TAI-UTC offset corresponding to a time in UTC if available */
    #[no_mangle]
    fn REF_GetTaiOffset(ts: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn CNF_AddRefclocks();
    #[no_mangle]
    fn CNF_GetLogRefclocks() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetLeapSecTimezone() -> *mut libc::c_char;
    /* Read the system clock, corrected according to all accumulated
    drifts and uncompensated offsets.

    In a kernel implementation with vernier frequency control (like
    Linux), and if we were to apply offsets by stepping the clock, this
    would be identical to raw time.  In any other case (use of
    adjtime()-like interface to correct offsets, and to adjust the
    frequency), we must correct the raw time to get this value */
    #[no_mangle]
    fn LCL_ReadCookedTime(ts: *mut timespec, err: *mut libc::c_double);
    /* Read the current offset between the system clock and true time
    (i.e. 'cooked' - 'raw') (in seconds). */
    #[no_mangle]
    fn LCL_GetOffsetCorrection(
        raw: *mut timespec,
        correction: *mut libc::c_double,
        err: *mut libc::c_double,
    );
    /* Add a handler.  Then handler MUST NOT deregister itself!!! */
    #[no_mangle]
    fn LCL_AddParameterChangeHandler(
        handler: LCL_ParameterChangeHandler,
        anything: *mut libc::c_void,
    );
    /* Remove a handler */
    #[no_mangle]
    fn LCL_RemoveParameterChangeHandler(_: LCL_ParameterChangeHandler, anything: *mut libc::c_void);
    /* Register a handler for being notified of dispersion being added to
    the local clock.  The handler MUST NOT unregister itself!!! */
    #[no_mangle]
    fn LCL_AddDispersionNotifyHandler(
        handler: LCL_DispersionNotifyHandler,
        anything: *mut libc::c_void,
    );
    /* Delete a handler */
    #[no_mangle]
    fn LCL_RemoveDispersionNotifyHandler(
        handler: LCL_DispersionNotifyHandler,
        anything: *mut libc::c_void,
    );
    /* Routine to read the system precision in terms of the actual time step */
    #[no_mangle]
    fn LCL_GetSysPrecisionAsQuantum() -> libc::c_double;
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
    fn UTI_IsTimeOffsetSane(ts: *mut timespec, offset: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec) -> libc::c_double;
    #[no_mangle]
    fn UTI_TimespecToString(ts: *mut timespec) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_AddDoubleToTimespec(start: *mut timespec, increment: libc::c_double, end: *mut timespec);
    #[no_mangle]
    fn UTI_TimeToLogForm(t: time_t) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_Log2ToDouble(l: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn UTI_RefidToString(ref_id: uint32_t) -> *mut libc::c_char;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LOG_FileOpen(name: *const libc::c_char, banner: *const libc::c_char) -> LOG_FileID;
    #[no_mangle]
    fn LOG_FileWrite(id: LOG_FileID, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn SPF_CreateInstance(
        min_samples: libc::c_int,
        max_samples: libc::c_int,
        max_dispersion: libc::c_double,
        combine_ratio: libc::c_double,
    ) -> SPF_Instance;
    #[no_mangle]
    fn SPF_DestroyInstance(filter: SPF_Instance);
    #[no_mangle]
    fn SPF_AccumulateSample(filter: SPF_Instance, sample: *mut NTP_Sample) -> libc::c_int;
    #[no_mangle]
    fn SPF_GetLastSample(filter: SPF_Instance, sample: *mut NTP_Sample) -> libc::c_int;
    #[no_mangle]
    fn SPF_GetAvgSampleDispersion(filter: SPF_Instance) -> libc::c_double;
    #[no_mangle]
    fn SPF_DropSamples(filter: SPF_Instance);
    #[no_mangle]
    fn SPF_GetFilteredSample(filter: SPF_Instance, sample: *mut NTP_Sample) -> libc::c_int;
    #[no_mangle]
    fn SPF_SlewSamples(
        filter: SPF_Instance,
        when: *mut timespec,
        dfreq: libc::c_double,
        doffset: libc::c_double,
    );
    #[no_mangle]
    fn SPF_AddDispersion(filter: SPF_Instance, dispersion: libc::c_double);
    /* This queues a timeout to elapse at a given delta time relative to the current (raw) time */
    #[no_mangle]
    fn SCH_AddTimeoutByDelay(
        delay: libc::c_double,
        _: SCH_TimeoutHandler,
        _: SCH_ArbitraryArgument,
    ) -> SCH_TimeoutID;
    /*
     chronyd/chronyc - Programs for keeping computer clocks accurate.

    **********************************************************************
    * Copyright (C) Miroslav Lichvar  2009-2011, 2013-2014, 2016-2019
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

     Routines implementing reference clocks.

     */
    /* list of refclock drivers */
    #[no_mangle]
    static mut RCL_SHM_driver: RefclockDriver;
    #[no_mangle]
    static mut RCL_SOCK_driver: RefclockDriver;
    #[no_mangle]
    static mut RCL_PPS_driver: RefclockDriver;
    #[no_mangle]
    static mut RCL_PHC_driver: RefclockDriver;
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
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type time_t = __time_t;
/* The UDP port number used by NTP */
/* The NTP protocol version that we support */
/* Maximum stratum number (infinity) */
/* The minimum valid length of an extension field */
/* The maximum assumed length of all extension fields in received
packets (RFC 5905 doesn't specify a limit on length or number of
extension fields in one packet) */
/* The minimum and maximum supported length of MAC */
/* The maximum length of MAC in NTPv4 packets which allows deterministic
parsing of extension fields (RFC 7822) */
/* Type definition for leap bits */
pub type NTP_Leap = libc::c_uint;
pub const LEAP_Unsynchronised: NTP_Leap = 3;
pub const LEAP_DeleteSecond: NTP_Leap = 2;
pub const LEAP_InsertSecond: NTP_Leap = 1;
pub const LEAP_Normal: NTP_Leap = 0;
/* Macros to work with the lvm field */
/* Special NTP reference IDs */
/* 127.127.1.1 */
/* 127.127.1.255 */
/* Structure used to save NTP measurements.  time is the local time at which
the sample is to be considered to have been made and offset is the offset at
the time (positive indicates that the local clock is slow relative to the
source).  root_delay/root_dispersion include peer_delay/peer_dispersion. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_Sample {
    pub time: timespec,
    pub offset: libc::c_double,
    pub peer_delay: libc::c_double,
    pub peer_dispersion: libc::c_double,
    pub root_delay: libc::c_double,
    pub root_dispersion: libc::c_double,
    pub stratum: libc::c_int,
    pub leap: NTP_Leap,
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
pub struct RPT_SourceReport {
    pub ip_addr: IPAddr,
    pub stratum: libc::c_int,
    pub poll: libc::c_int,
    pub mode: C2RustUnnamed_1,
    pub state: C2RustUnnamed_0,
    pub sel_options: libc::c_int,
    pub reachability: libc::c_int,
    pub latest_meas_ago: libc::c_ulong,
    pub orig_latest_meas: libc::c_double,
    pub latest_meas: libc::c_double,
    pub latest_meas_err: libc::c_double,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const RPT_OUTLIER: C2RustUnnamed_0 = 5;
pub const RPT_CANDIDATE: C2RustUnnamed_0 = 4;
pub const RPT_JITTERY: C2RustUnnamed_0 = 3;
pub const RPT_FALSETICKER: C2RustUnnamed_0 = 2;
pub const RPT_UNREACH: C2RustUnnamed_0 = 1;
pub const RPT_SYNC: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const RPT_LOCAL_REFERENCE: C2RustUnnamed_1 = 2;
pub const RPT_NTP_PEER: C2RustUnnamed_1 = 1;
pub const RPT_NTP_CLIENT: C2RustUnnamed_1 = 0;
pub type SRC_Instance = *mut SRC_Instance_Record;
pub type SRC_Type = libc::c_uint;
pub const SRC_REFCLOCK: SRC_Type = 1;
pub const SRC_NTP: SRC_Type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefclockParameters {
    pub driver_name: *mut libc::c_char,
    pub driver_parameter: *mut libc::c_char,
    pub driver_poll: libc::c_int,
    pub poll: libc::c_int,
    pub filter_length: libc::c_int,
    pub pps_forced: libc::c_int,
    pub pps_rate: libc::c_int,
    pub min_samples: libc::c_int,
    pub max_samples: libc::c_int,
    pub sel_options: libc::c_int,
    pub max_lock_age: libc::c_int,
    pub stratum: libc::c_int,
    pub tai: libc::c_int,
    pub ref_id: uint32_t,
    pub lock_ref_id: uint32_t,
    pub offset: libc::c_double,
    pub delay: libc::c_double,
    pub precision: libc::c_double,
    pub max_dispersion: libc::c_double,
    pub pulse_width: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RCL_Instance_Record {
    pub driver: *mut RefclockDriver,
    pub data: *mut libc::c_void,
    pub driver_parameter: *mut libc::c_char,
    pub driver_parameter_length: libc::c_int,
    pub driver_poll: libc::c_int,
    pub driver_polled: libc::c_int,
    pub poll: libc::c_int,
    pub leap_status: libc::c_int,
    pub pps_forced: libc::c_int,
    pub pps_rate: libc::c_int,
    pub pps_active: libc::c_int,
    pub max_lock_age: libc::c_int,
    pub stratum: libc::c_int,
    pub tai: libc::c_int,
    pub ref_id: uint32_t,
    pub lock_ref: uint32_t,
    pub offset: libc::c_double,
    pub delay: libc::c_double,
    pub precision: libc::c_double,
    pub pulse_width: libc::c_double,
    pub filter: SPF_Instance,
    pub timeout_id: SCH_TimeoutID,
    pub source: SRC_Instance,
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
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Miroslav Lichvar  2018
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

 Header file for sample filter.

 */
pub type SPF_Instance = *mut SPF_Instance_Record;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefclockDriver {
    pub init: Option<unsafe extern "C" fn(_: RCL_Instance) -> libc::c_int>,
    pub fini: Option<unsafe extern "C" fn(_: RCL_Instance) -> ()>,
    pub poll: Option<unsafe extern "C" fn(_: RCL_Instance) -> libc::c_int>,
}
pub type RCL_Instance = *mut RCL_Instance_Record;
/* File logging functions */
pub type LOG_FileID = libc::c_int;
/* Function type for handlers to be called back when an indeterminate
offset is introduced into the local time.  This situation occurs
when the frequency must be adjusted to effect a clock slew and
there is doubt about one of the endpoints of the interval over
which the frequency change was applied.It is expected that such
handlers will add extra dispersion to any existing samples stored
in their registers.

dispersion : The bound on how much error has been introduced in the
local clock, in seconds.

anything : passthrough from the registration routine

*/
pub type LCL_DispersionNotifyHandler =
    Option<unsafe extern "C" fn(_: libc::c_double, _: *mut libc::c_void) -> ()>;
pub type LCL_ChangeType = libc::c_uint;
pub const LCL_ChangeUnknownStep: LCL_ChangeType = 2;
pub const LCL_ChangeStep: LCL_ChangeType = 1;
pub const LCL_ChangeAdjust: LCL_ChangeType = 0;
pub type LCL_ParameterChangeHandler = Option<
    unsafe extern "C" fn(
        _: *mut timespec,
        _: *mut timespec,
        _: libc::c_double,
        _: libc::c_double,
        _: LCL_ChangeType,
        _: *mut libc::c_void,
    ) -> (),
>;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
pub type SCH_ArbitraryArgument = *mut libc::c_void;
pub type SCH_TimeoutHandler = Option<unsafe extern "C" fn(_: SCH_ArbitraryArgument) -> ()>;
/* Array of pointers to RCL_Instance_Record */
static mut refclocks: ARR_Instance = 0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
static mut logfileid: LOG_FileID = 0;
unsafe extern "C" fn get_refclock(mut index: libc::c_uint) -> RCL_Instance {
    return *(ARR_GetElement(refclocks, index) as *mut RCL_Instance);
}
#[no_mangle]
pub unsafe extern "C" fn RCL_Initialise() {
    refclocks =
        ARR_CreateInstance(::std::mem::size_of::<RCL_Instance>() as libc::c_ulong as libc::c_uint);
    CNF_AddRefclocks();
    if ARR_GetSize(refclocks) > 0 as libc::c_int as libc::c_uint {
        LCL_AddParameterChangeHandler(
            Some(
                slew_samples
                    as unsafe extern "C" fn(
                        _: *mut timespec,
                        _: *mut timespec,
                        _: libc::c_double,
                        _: libc::c_double,
                        _: LCL_ChangeType,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
        LCL_AddDispersionNotifyHandler(
            Some(
                add_dispersion
                    as unsafe extern "C" fn(_: libc::c_double, _: *mut libc::c_void) -> (),
            ),
            0 as *mut libc::c_void,
        );
    }
    logfileid = if CNF_GetLogRefclocks() != 0 {
        LOG_FileOpen(
            b"refclocks\x00" as *const u8 as *const libc::c_char,
            b"   Date (UTC) Time         Refid  DP L P  Raw offset   Cooked offset      Disp.\x00"
                as *const u8 as *const libc::c_char,
        )
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn RCL_Finalise() {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(refclocks) {
        let mut inst: RCL_Instance = get_refclock(i);
        if (*(*inst).driver).fini.is_some() {
            (*(*inst).driver).fini.expect("non-null function pointer")(inst);
        }
        SPF_DestroyInstance((*inst).filter);
        free((*inst).driver_parameter as *mut libc::c_void);
        SRC_DestroyInstance((*inst).source);
        free(inst as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    if ARR_GetSize(refclocks) > 0 as libc::c_int as libc::c_uint {
        LCL_RemoveParameterChangeHandler(
            Some(
                slew_samples
                    as unsafe extern "C" fn(
                        _: *mut timespec,
                        _: *mut timespec,
                        _: libc::c_double,
                        _: libc::c_double,
                        _: LCL_ChangeType,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
        LCL_RemoveDispersionNotifyHandler(
            Some(
                add_dispersion
                    as unsafe extern "C" fn(_: libc::c_double, _: *mut libc::c_void) -> (),
            ),
            0 as *mut libc::c_void,
        );
    }
    ARR_DestroyInstance(refclocks);
}
#[no_mangle]
pub unsafe extern "C" fn RCL_AddRefclock(mut params: *mut RefclockParameters) -> libc::c_int {
    let mut inst: RCL_Instance = 0 as *mut RCL_Instance_Record;
    inst = Malloc(::std::mem::size_of::<RCL_Instance_Record>() as libc::c_ulong)
        as *mut RCL_Instance_Record;
    let ref mut fresh0 = *(ARR_GetNewElement(refclocks) as *mut RCL_Instance);
    *fresh0 = inst;
    if strcmp(
        (*params).driver_name,
        b"SHM\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*inst).driver = &mut RCL_SHM_driver
    } else if strcmp(
        (*params).driver_name,
        b"SOCK\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*inst).driver = &mut RCL_SOCK_driver
    } else if strcmp(
        (*params).driver_name,
        b"PPS\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*inst).driver = &mut RCL_PPS_driver
    } else if strcmp(
        (*params).driver_name,
        b"PHC\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*inst).driver = &mut RCL_PHC_driver
    } else {
        LOG_Message(
            LOGS_FATAL,
            b"unknown refclock driver %s\x00" as *const u8 as *const libc::c_char,
            (*params).driver_name,
        );
        exit(1 as libc::c_int);
    }
    if (*(*inst).driver).init.is_none() && (*(*inst).driver).poll.is_none() {
        LOG_Message(
            LOGS_FATAL,
            b"refclock driver %s is not compiled in\x00" as *const u8 as *const libc::c_char,
            (*params).driver_name,
        );
        exit(1 as libc::c_int);
    }
    if (*params).tai != 0 && CNF_GetLeapSecTimezone().is_null() {
        LOG_Message(
            LOGS_FATAL,
            b"refclock tai option requires leapsectz\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    (*inst).data = 0 as *mut libc::c_void;
    (*inst).driver_parameter = (*params).driver_parameter;
    (*inst).driver_parameter_length = 0 as libc::c_int;
    (*inst).driver_poll = (*params).driver_poll;
    (*inst).poll = (*params).poll;
    (*inst).driver_polled = 0 as libc::c_int;
    (*inst).leap_status = LEAP_Normal as libc::c_int;
    (*inst).pps_forced = (*params).pps_forced;
    (*inst).pps_rate = (*params).pps_rate;
    (*inst).pps_active = 0 as libc::c_int;
    (*inst).max_lock_age = (*params).max_lock_age;
    (*inst).stratum = (*params).stratum;
    (*inst).tai = (*params).tai;
    (*inst).lock_ref = (*params).lock_ref_id;
    (*inst).offset = (*params).offset;
    (*inst).delay = (*params).delay;
    (*inst).precision = LCL_GetSysPrecisionAsQuantum();
    (*inst).precision = if (*inst).precision > (*params).precision {
        (*inst).precision
    } else {
        (*params).precision
    };
    (*inst).pulse_width = (*params).pulse_width;
    (*inst).timeout_id = -(1 as libc::c_int) as SCH_TimeoutID;
    (*inst).source = 0 as SRC_Instance;
    if !(*inst).driver_parameter.is_null() {
        let mut i: libc::c_int = 0;
        (*inst).driver_parameter_length = strlen((*inst).driver_parameter) as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*inst).driver_parameter_length {
            if *(*inst).driver_parameter.offset(i as isize) as libc::c_int == ':' as i32 {
                *(*inst).driver_parameter.offset(i as isize) = '\u{0}' as i32 as libc::c_char
            }
            i += 1
        }
    }
    if (*inst).pps_rate < 1 as libc::c_int {
        (*inst).pps_rate = 1 as libc::c_int
    }
    if (*params).ref_id != 0 {
        (*inst).ref_id = (*params).ref_id
    } else {
        let mut ref_0: [libc::c_uchar; 5] = [
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        ];
        let mut index: libc::c_uint =
            ARR_GetSize(refclocks).wrapping_sub(1 as libc::c_int as libc::c_uint);
        snprintf(
            ref_0.as_mut_ptr() as *mut libc::c_char,
            ::std::mem::size_of::<[libc::c_uchar; 5]>() as libc::c_ulong,
            b"%3.3s\x00" as *const u8 as *const libc::c_char,
            (*params).driver_name,
        );
        ref_0[3 as libc::c_int as usize] = index
            .wrapping_rem(10 as libc::c_int as libc::c_uint)
            .wrapping_add('0' as i32 as libc::c_uint)
            as libc::c_uchar;
        if index >= 10 as libc::c_int as libc::c_uint {
            ref_0[2 as libc::c_int as usize] = index
                .wrapping_div(10 as libc::c_int as libc::c_uint)
                .wrapping_rem(10 as libc::c_int as libc::c_uint)
                .wrapping_add('0' as i32 as libc::c_uint)
                as libc::c_uchar
        }
        (*inst).ref_id = (ref_0[0 as libc::c_int as usize] as uint32_t) << 24 as libc::c_int
            | ((ref_0[1 as libc::c_int as usize] as libc::c_int) << 16 as libc::c_int)
                as libc::c_uint
            | ((ref_0[2 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int)
                as libc::c_uint
            | ref_0[3 as libc::c_int as usize] as libc::c_uint
    }
    if (*(*inst).driver).poll.is_some() {
        let mut max_samples: libc::c_int = 0;
        if (*inst).driver_poll > (*inst).poll {
            (*inst).driver_poll = (*inst).poll
        }
        max_samples = (1 as libc::c_int) << (*inst).poll - (*inst).driver_poll;
        if max_samples < (*params).filter_length {
            if max_samples < 4 as libc::c_int {
                LOG_Message(
                    LOGS_WARN,
                    b"Setting filter length for %s to %d\x00" as *const u8 as *const libc::c_char,
                    UTI_RefidToString((*inst).ref_id),
                    max_samples,
                );
            }
            (*params).filter_length = max_samples
        }
    }
    if (*(*inst).driver).init.is_some()
        && (*(*inst).driver).init.expect("non-null function pointer")(inst) == 0
    {
        LOG_Message(
            LOGS_FATAL,
            b"refclock %s initialisation failed\x00" as *const u8 as *const libc::c_char,
            (*params).driver_name,
        );
        exit(1 as libc::c_int);
    }
    /* Require the filter to have at least 4 samples to produce a filtered
    sample, or be full for shorter lengths, and combine 60% of samples
    closest to the median */
    (*inst).filter = SPF_CreateInstance(
        if (*params).filter_length < 4 as libc::c_int {
            (*params).filter_length
        } else {
            4 as libc::c_int
        },
        (*params).filter_length,
        (*params).max_dispersion,
        0.6f64,
    );
    (*inst).source = SRC_CreateNewInstance(
        (*inst).ref_id,
        SRC_REFCLOCK,
        (*params).sel_options,
        0 as *mut IPAddr,
        (*params).min_samples,
        (*params).max_samples,
        0.0f64,
        0.0f64,
    );
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"refclock %s refid=%s poll=%d dpoll=%d filter=%d\x00" as *const u8
                as *const libc::c_char,
            (*params).driver_name,
            UTI_RefidToString((*inst).ref_id),
            (*inst).poll,
            (*inst).driver_poll,
            (*params).filter_length,
        );
    }
    free((*params).driver_name as *mut libc::c_void);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RCL_StartRefclocks() {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    n = ARR_GetSize(refclocks);
    i = 0 as libc::c_int as libc::c_uint;
    while i < n {
        let mut inst: RCL_Instance = get_refclock(i);
        SRC_SetActive((*inst).source);
        (*inst).timeout_id = SCH_AddTimeoutByDelay(
            0.0f64,
            Some(poll_timeout as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
            inst as *mut libc::c_void,
        );
        if (*inst).lock_ref != 0 {
            /* Replace lock refid with index to refclocks */
            j = 0 as libc::c_int as libc::c_uint;
            while j < n && (*get_refclock(j)).ref_id != (*inst).lock_ref {
                j = j.wrapping_add(1)
            }
            (*inst).lock_ref = if j < n {
                j
            } else {
                -(1 as libc::c_int) as libc::c_uint
            }
        } else {
            (*inst).lock_ref = -(1 as libc::c_int) as uint32_t
        }
        i = i.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn RCL_ReportSource(
    mut report: *mut RPT_SourceReport,
    mut now: *mut timespec,
) {
    let mut i: libc::c_uint = 0;
    let mut ref_id: uint32_t = 0;
    if (*report).ip_addr.family as libc::c_int == 1 as libc::c_int {
    } else {
        __assert_fail(
            b"report->ip_addr.family == IPADDR_INET4\x00" as *const u8 as *const libc::c_char,
            b"refclock.c\x00" as *const u8 as *const libc::c_char,
            297 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                b"void RCL_ReportSource(RPT_SourceReport *, struct timespec *)\x00",
            ))
            .as_ptr(),
        );
    }
    ref_id = (*report).ip_addr.addr.in4;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(refclocks) {
        let mut inst: RCL_Instance = get_refclock(i);
        if (*inst).ref_id == ref_id {
            (*report).poll = (*inst).poll;
            (*report).mode = RPT_LOCAL_REFERENCE;
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn RCL_SetDriverData(
    mut instance: RCL_Instance,
    mut data: *mut libc::c_void,
) {
    (*instance).data = data;
}
#[no_mangle]
pub unsafe extern "C" fn RCL_GetDriverData(mut instance: RCL_Instance) -> *mut libc::c_void {
    return (*instance).data;
}
#[no_mangle]
pub unsafe extern "C" fn RCL_GetDriverParameter(mut instance: RCL_Instance) -> *mut libc::c_char {
    return (*instance).driver_parameter;
}
unsafe extern "C" fn get_next_driver_option(
    mut instance: RCL_Instance,
    mut option: *mut libc::c_char,
) -> *mut libc::c_char {
    if option.is_null() {
        option = (*instance).driver_parameter
    }
    option = option.offset(strlen(option).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    if option
        >= (*instance)
            .driver_parameter
            .offset((*instance).driver_parameter_length as isize)
    {
        return 0 as *mut libc::c_char;
    }
    return option;
}
#[no_mangle]
pub unsafe extern "C" fn RCL_CheckDriverOptions(
    mut instance: RCL_Instance,
    mut options: *mut *const libc::c_char,
) {
    let mut option: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    option = get_next_driver_option(instance, 0 as *mut libc::c_char);
    while !option.is_null() {
        i = 0 as libc::c_int;
        while !options.is_null() && !(*options.offset(i as isize)).is_null() {
            len = strlen(*options.offset(i as isize)) as libc::c_int;
            if strncmp(*options.offset(i as isize), option, len as libc::c_ulong) == 0
                && (*option.offset(len as isize) as libc::c_int == '=' as i32
                    || *option.offset(len as isize) as libc::c_int == '\u{0}' as i32)
            {
                break;
            }
            i += 1
        }
        if options.is_null() || (*options.offset(i as isize)).is_null() {
            LOG_Message(
                LOGS_FATAL,
                b"Invalid refclock driver option %s\x00" as *const u8 as *const libc::c_char,
                option,
            );
            exit(1 as libc::c_int);
        }
        option = get_next_driver_option(instance, option)
    }
}
#[no_mangle]
pub unsafe extern "C" fn RCL_GetDriverOption(
    mut instance: RCL_Instance,
    mut name: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut option: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    len = strlen(name) as libc::c_int;
    option = get_next_driver_option(instance, 0 as *mut libc::c_char);
    while !option.is_null() {
        if strncmp(name, option, len as libc::c_ulong) == 0 {
            if *option.offset(len as isize) as libc::c_int == '=' as i32 {
                return option
                    .offset(len as isize)
                    .offset(1 as libc::c_int as isize);
            }
            if *option.offset(len as isize) as libc::c_int == '\u{0}' as i32 {
                return option.offset(len as isize);
            }
        }
        option = get_next_driver_option(instance, option)
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn convert_tai_offset(
    mut sample_time: *mut timespec,
    mut offset: *mut libc::c_double,
) -> libc::c_int {
    let mut tai_ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut utc_ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut tai_offset: libc::c_int = 0;
    /* Get approximate TAI-UTC offset for the reference time in TAI */
    UTI_AddDoubleToTimespec(sample_time, *offset, &mut tai_ts);
    tai_offset = REF_GetTaiOffset(&mut tai_ts);
    /* Get TAI-UTC offset for the reference time in UTC +/- 1 second */
    UTI_AddDoubleToTimespec(&mut tai_ts, -tai_offset as libc::c_double, &mut utc_ts);
    tai_offset = REF_GetTaiOffset(&mut utc_ts);
    if tai_offset == 0 {
        return 0 as libc::c_int;
    }
    *offset -= tai_offset as libc::c_double;
    return 1 as libc::c_int;
}
unsafe extern "C" fn accumulate_sample(
    mut instance: RCL_Instance,
    mut sample_time: *mut timespec,
    mut offset: libc::c_double,
    mut dispersion: libc::c_double,
) -> libc::c_int {
    let mut sample: NTP_Sample = NTP_Sample {
        time: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        offset: 0.,
        peer_delay: 0.,
        peer_dispersion: 0.,
        root_delay: 0.,
        root_dispersion: 0.,
        stratum: 0,
        leap: LEAP_Normal,
    };
    sample.time = *sample_time;
    sample.offset = offset;
    sample.peer_delay = (*instance).delay;
    sample.root_delay = (*instance).delay;
    sample.peer_dispersion = dispersion;
    sample.root_dispersion = dispersion;
    sample.leap = (*instance).leap_status as NTP_Leap;
    /* Handle special case when PPS is used with the local reference */
    if (*instance).pps_active != 0 && (*instance).lock_ref == -(1 as libc::c_int) as libc::c_uint {
        sample.stratum = pps_stratum(instance, &mut sample.time)
    } else {
        sample.stratum = (*instance).stratum
    }
    return SPF_AccumulateSample((*instance).filter, &mut sample);
}
#[no_mangle]
pub unsafe extern "C" fn RCL_AddSample(
    mut instance: RCL_Instance,
    mut sample_time: *mut timespec,
    mut offset: libc::c_double,
    mut leap: libc::c_int,
) -> libc::c_int {
    let mut correction: libc::c_double = 0.;
    let mut dispersion: libc::c_double = 0.;
    let mut cooked_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if (*instance).pps_forced != 0 {
        return RCL_AddPulse(instance, sample_time, -offset);
    }
    LCL_GetOffsetCorrection(sample_time, &mut correction, &mut dispersion);
    UTI_AddDoubleToTimespec(sample_time, correction, &mut cooked_time);
    dispersion += (*instance).precision;
    /* Make sure the timestamp and offset provided by the driver are sane */
    if UTI_IsTimeOffsetSane(sample_time, offset) == 0
        || valid_sample_time(instance, &mut cooked_time) == 0
    {
        return 0 as libc::c_int;
    }
    match leap {
        0 | 1 | 2 => (*instance).leap_status = leap,
        _ => {
            if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
            {
                LOG_Message(
                    LOGS_DEBUG,
                    b"refclock sample ignored bad leap %d\x00" as *const u8 as *const libc::c_char,
                    leap,
                );
            }
            return 0 as libc::c_int;
        }
    }
    if (*instance).tai != 0 && convert_tai_offset(sample_time, &mut offset) == 0 {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"refclock sample ignored unknown TAI offset\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    if accumulate_sample(
        instance,
        &mut cooked_time,
        offset - correction + (*instance).offset,
        dispersion,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    (*instance).pps_active = 0 as libc::c_int;
    log_sample(
        instance,
        &mut cooked_time,
        0 as libc::c_int,
        0 as libc::c_int,
        offset,
        offset - correction + (*instance).offset,
        dispersion,
    );
    /* for logging purposes */
    if (*(*instance).driver).poll.is_none() {
        (*instance).driver_polled += 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RCL_AddPulse(
    mut instance: RCL_Instance,
    mut pulse_time: *mut timespec,
    mut second: libc::c_double,
) -> libc::c_int {
    let mut correction: libc::c_double = 0.;
    let mut dispersion: libc::c_double = 0.;
    let mut cooked_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    LCL_GetOffsetCorrection(pulse_time, &mut correction, &mut dispersion);
    UTI_AddDoubleToTimespec(pulse_time, correction, &mut cooked_time);
    second += correction;
    if UTI_IsTimeOffsetSane(pulse_time, 0.0f64) == 0 {
        return 0 as libc::c_int;
    }
    return RCL_AddCookedPulse(instance, &mut cooked_time, second, dispersion, correction);
}
unsafe extern "C" fn check_pulse_edge(
    mut instance: RCL_Instance,
    mut offset: libc::c_double,
    mut distance: libc::c_double,
) -> libc::c_int {
    let mut max_error: libc::c_double = 0.;
    if (*instance).pulse_width <= 0.0f64 {
        return 1 as libc::c_int;
    }
    max_error = 1.0f64 / (*instance).pps_rate as libc::c_double - (*instance).pulse_width;
    max_error = if (*instance).pulse_width < max_error {
        (*instance).pulse_width
    } else {
        max_error
    };
    max_error *= 0.5f64;
    if fabs(offset) > max_error || distance > max_error {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"refclock pulse ignored offset=%.9f distance=%.9f max_error=%.9f\x00" as *const u8
                    as *const libc::c_char,
                offset,
                distance,
                max_error,
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RCL_AddCookedPulse(
    mut instance: RCL_Instance,
    mut cooked_time: *mut timespec,
    mut second: libc::c_double,
    mut dispersion: libc::c_double,
    mut raw_correction: libc::c_double,
) -> libc::c_int {
    let mut offset: libc::c_double = 0.;
    let mut rate: libc::c_int = 0;
    let mut leap: NTP_Leap = LEAP_Normal;
    if UTI_IsTimeOffsetSane(cooked_time, second) == 0
        || valid_sample_time(instance, cooked_time) == 0
    {
        return 0 as libc::c_int;
    }
    leap = LEAP_Normal;
    dispersion += (*instance).precision;
    rate = (*instance).pps_rate;
    offset = -second + (*instance).offset;
    /* Adjust the offset to [-0.5/rate, 0.5/rate) interval */
    offset -= (offset * rate as libc::c_double) as libc::c_long as libc::c_double
        / rate as libc::c_double;
    if offset < -0.5f64 / rate as libc::c_double {
        offset += 1.0f64 / rate as libc::c_double
    } else if offset >= 0.5f64 / rate as libc::c_double {
        offset -= 1.0f64 / rate as libc::c_double
    }
    if (*instance).lock_ref != -(1 as libc::c_int) as libc::c_uint {
        let mut lock_refclock: RCL_Instance = 0 as *mut RCL_Instance_Record;
        let mut ref_sample: NTP_Sample = NTP_Sample {
            time: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            offset: 0.,
            peer_delay: 0.,
            peer_dispersion: 0.,
            root_delay: 0.,
            root_dispersion: 0.,
            stratum: 0,
            leap: LEAP_Normal,
        };
        let mut sample_diff: libc::c_double = 0.;
        let mut shift: libc::c_double = 0.;
        lock_refclock = get_refclock((*instance).lock_ref);
        if SPF_GetLastSample((*lock_refclock).filter, &mut ref_sample) == 0 {
            if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
            {
                LOG_Message(
                    LOGS_DEBUG,
                    b"refclock pulse ignored no ref sample\x00" as *const u8 as *const libc::c_char,
                );
            }
            return 0 as libc::c_int;
        }
        ref_sample.root_dispersion += SPF_GetAvgSampleDispersion((*lock_refclock).filter);
        sample_diff = UTI_DiffTimespecsToDouble(cooked_time, &mut ref_sample.time);
        if fabs(sample_diff) >= (*instance).max_lock_age as libc::c_double / rate as libc::c_double
        {
            if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
            {
                LOG_Message(
                    LOGS_DEBUG,
                    b"refclock pulse ignored samplediff=%.9f\x00" as *const u8
                        as *const libc::c_char,
                    sample_diff,
                );
            }
            return 0 as libc::c_int;
        }
        /* Align the offset to the reference sample */
        if ref_sample.offset - offset >= 0.0f64 {
            shift = ((ref_sample.offset - offset) * rate as libc::c_double + 0.5f64) as libc::c_long
                as libc::c_double
                / rate as libc::c_double
        } else {
            shift = ((ref_sample.offset - offset) * rate as libc::c_double - 0.5f64) as libc::c_long
                as libc::c_double
                / rate as libc::c_double
        }
        offset += shift;
        if fabs(ref_sample.offset - offset) + ref_sample.root_dispersion + dispersion
            >= 0.2f64 / rate as libc::c_double
        {
            if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
            {
                LOG_Message(
                    LOGS_DEBUG,
                    b"refclock pulse ignored offdiff=%.9f refdisp=%.9f disp=%.9f\x00" as *const u8
                        as *const libc::c_char,
                    ref_sample.offset - offset,
                    ref_sample.root_dispersion,
                    dispersion,
                );
            }
            return 0 as libc::c_int;
        }
        if check_pulse_edge(instance, ref_sample.offset - offset, 0.0f64) == 0 {
            return 0 as libc::c_int;
        }
        leap = (*lock_refclock).leap_status as NTP_Leap;
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"refclock pulse offset=%.9f offdiff=%.9f samplediff=%.9f\x00" as *const u8
                    as *const libc::c_char,
                offset,
                ref_sample.offset - offset,
                sample_diff,
            );
        }
    } else {
        let mut ref_time: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut is_synchronised: libc::c_int = 0;
        let mut stratum: libc::c_int = 0;
        let mut root_delay: libc::c_double = 0.;
        let mut root_dispersion: libc::c_double = 0.;
        let mut distance: libc::c_double = 0.;
        let mut ref_id: uint32_t = 0;
        /* Ignore the pulse if we are not well synchronized and the local
        reference is not active */
        REF_GetReferenceParams(
            cooked_time,
            &mut is_synchronised,
            &mut leap,
            &mut stratum,
            &mut ref_id,
            &mut ref_time,
            &mut root_delay,
            &mut root_dispersion,
        );
        distance = fabs(root_delay) / 2 as libc::c_int as libc::c_double + root_dispersion;
        if leap as libc::c_uint == LEAP_Unsynchronised as libc::c_int as libc::c_uint
            || distance >= 0.5f64 / rate as libc::c_double
        {
            if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int
            {
                LOG_Message(
                    LOGS_DEBUG,
                    b"refclock pulse ignored offset=%.9f sync=%d dist=%.9f\x00" as *const u8
                        as *const libc::c_char,
                    offset,
                    (leap as libc::c_uint != LEAP_Unsynchronised as libc::c_int as libc::c_uint)
                        as libc::c_int,
                    distance,
                );
            }
            /* Drop also all stored samples */
            SPF_DropSamples((*instance).filter);
            return 0 as libc::c_int;
        }
        if check_pulse_edge(instance, offset, distance) == 0 {
            return 0 as libc::c_int;
        }
    }
    if accumulate_sample(instance, cooked_time, offset, dispersion) == 0 {
        return 0 as libc::c_int;
    }
    (*instance).leap_status = leap as libc::c_int;
    (*instance).pps_active = 1 as libc::c_int;
    log_sample(
        instance,
        cooked_time,
        0 as libc::c_int,
        1 as libc::c_int,
        offset + raw_correction - (*instance).offset,
        offset,
        dispersion,
    );
    /* for logging purposes */
    if (*(*instance).driver).poll.is_none() {
        (*instance).driver_polled += 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RCL_GetPrecision(mut instance: RCL_Instance) -> libc::c_double {
    return (*instance).precision;
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

 Header file for refclocks.

 */
/* functions used by drivers */
#[no_mangle]
pub unsafe extern "C" fn RCL_GetDriverPoll(mut instance: RCL_Instance) -> libc::c_int {
    return (*instance).driver_poll;
}
unsafe extern "C" fn valid_sample_time(
    mut instance: RCL_Instance,
    mut sample_time: *mut timespec,
) -> libc::c_int {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut diff: libc::c_double = 0.;
    LCL_ReadCookedTime(&mut now, 0 as *mut libc::c_double);
    diff = UTI_DiffTimespecsToDouble(&mut now, sample_time);
    if diff < 0.0f64 || diff > UTI_Log2ToDouble((*instance).poll + 1 as libc::c_int) {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"%s refclock sample time %s not valid age=%.6f\x00" as *const u8
                    as *const libc::c_char,
                UTI_RefidToString((*instance).ref_id),
                UTI_TimespecToString(sample_time),
                diff,
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn pps_stratum(mut instance: RCL_Instance, mut ts: *mut timespec) -> libc::c_int {
    let mut ref_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut is_synchronised: libc::c_int = 0;
    let mut stratum: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut root_delay: libc::c_double = 0.;
    let mut root_dispersion: libc::c_double = 0.;
    let mut leap: NTP_Leap = LEAP_Normal;
    let mut ref_id: uint32_t = 0;
    let mut refclock: RCL_Instance = 0 as *mut RCL_Instance_Record;
    REF_GetReferenceParams(
        ts,
        &mut is_synchronised,
        &mut leap,
        &mut stratum,
        &mut ref_id,
        &mut ref_time,
        &mut root_delay,
        &mut root_dispersion,
    );
    /* Don't change our stratum if the local reference is active
    or this is the current source */
    if ref_id == (*instance).ref_id
        || is_synchronised == 0
            && leap as libc::c_uint != LEAP_Unsynchronised as libc::c_int as libc::c_uint
    {
        return stratum - 1 as libc::c_int;
    }
    /* Or the current source is another PPS refclock */
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(refclocks) {
        refclock = get_refclock(i);
        if (*refclock).ref_id == ref_id
            && (*refclock).pps_active != 0
            && (*refclock).lock_ref == -(1 as libc::c_int) as libc::c_uint
        {
            return stratum - 1 as libc::c_int;
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn poll_timeout(mut arg: *mut libc::c_void) {
    let mut sample: NTP_Sample = NTP_Sample {
        time: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        offset: 0.,
        peer_delay: 0.,
        peer_dispersion: 0.,
        root_delay: 0.,
        root_dispersion: 0.,
        stratum: 0,
        leap: LEAP_Normal,
    };
    let mut poll: libc::c_int = 0;
    let mut inst: RCL_Instance = arg as RCL_Instance;
    poll = (*inst).poll;
    if (*(*inst).driver).poll.is_some() {
        poll = (*inst).driver_poll;
        (*(*inst).driver).poll.expect("non-null function pointer")(inst);
        (*inst).driver_polled += 1
    }
    if !((*(*inst).driver).poll.is_some()
        && (*inst).driver_polled < (1 as libc::c_int) << (*inst).poll - (*inst).driver_poll)
    {
        (*inst).driver_polled = 0 as libc::c_int;
        if SPF_GetFilteredSample((*inst).filter, &mut sample) != 0 {
            SRC_UpdateReachability((*inst).source, 1 as libc::c_int);
            SRC_AccumulateSample((*inst).source, &mut sample);
            SRC_SelectSource((*inst).source);
            log_sample(
                inst,
                &mut sample.time,
                1 as libc::c_int,
                0 as libc::c_int,
                0.0f64,
                sample.offset,
                sample.peer_dispersion,
            );
        } else {
            SRC_UpdateReachability((*inst).source, 0 as libc::c_int);
        }
    }
    (*inst).timeout_id = SCH_AddTimeoutByDelay(
        UTI_Log2ToDouble(poll),
        Some(poll_timeout as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        arg,
    );
}
unsafe extern "C" fn slew_samples(
    mut raw: *mut timespec,
    mut cooked: *mut timespec,
    mut dfreq: libc::c_double,
    mut doffset: libc::c_double,
    mut change_type: LCL_ChangeType,
    mut anything: *mut libc::c_void,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(refclocks) {
        if change_type as libc::c_uint == LCL_ChangeUnknownStep as libc::c_int as libc::c_uint {
            SPF_DropSamples((*get_refclock(i)).filter);
        } else {
            SPF_SlewSamples((*get_refclock(i)).filter, cooked, dfreq, doffset);
        }
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn add_dispersion(
    mut dispersion: libc::c_double,
    mut anything: *mut libc::c_void,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(refclocks) {
        SPF_AddDispersion((*get_refclock(i)).filter, dispersion);
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn log_sample(
    mut instance: RCL_Instance,
    mut sample_time: *mut timespec,
    mut filtered: libc::c_int,
    mut pulse: libc::c_int,
    mut raw_offset: libc::c_double,
    mut cooked_offset: libc::c_double,
    mut dispersion: libc::c_double,
) {
    let mut sync_stats: [libc::c_char; 4] = [
        'N' as i32 as libc::c_char,
        '+' as i32 as libc::c_char,
        '-' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
    ];
    if logfileid == -(1 as libc::c_int) {
        return;
    }
    if filtered == 0 {
        LOG_FileWrite(
            logfileid,
            b"%s.%06d %-5s %3d %1c %1d %13.6e %13.6e %10.3e\x00" as *const u8
                as *const libc::c_char,
            UTI_TimeToLogForm((*sample_time).tv_sec),
            (*sample_time).tv_nsec as libc::c_int / 1000 as libc::c_int,
            UTI_RefidToString((*instance).ref_id),
            (*instance).driver_polled,
            sync_stats[(*instance).leap_status as usize] as libc::c_int,
            pulse,
            raw_offset,
            cooked_offset,
            dispersion,
        );
    } else {
        LOG_FileWrite(
            logfileid,
            b"%s.%06d %-5s   - %1c -       -       %13.6e %10.3e\x00" as *const u8
                as *const libc::c_char,
            UTI_TimeToLogForm((*sample_time).tv_sec),
            (*sample_time).tv_nsec as libc::c_int / 1000 as libc::c_int,
            UTI_RefidToString((*instance).ref_id),
            sync_stats[(*instance).leap_status as usize] as libc::c_int,
            cooked_offset,
            dispersion,
        );
    };
}
