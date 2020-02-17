#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type RCL_Instance_Record;
    pub type HCL_Instance_Record;
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
    fn RCL_GetDriverPoll(instance: RCL_Instance) -> libc::c_int;
    #[no_mangle]
    fn RCL_GetPrecision(instance: RCL_Instance) -> libc::c_double;
    #[no_mangle]
    fn RCL_AddCookedPulse(instance: RCL_Instance, cooked_time: *mut timespec,
                          second: libc::c_double, dispersion: libc::c_double,
                          raw_correction: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn RCL_AddSample(instance: RCL_Instance, sample_time: *mut timespec,
                     offset: libc::c_double, leap: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn RCL_GetDriverOption(instance: RCL_Instance, name: *mut libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn RCL_CheckDriverOptions(instance: RCL_Instance,
                              options: *mut *const libc::c_char);
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn RCL_SetDriverData(instance: RCL_Instance, data: *mut libc::c_void);
    #[no_mangle]
    fn RCL_GetDriverData(instance: RCL_Instance) -> *mut libc::c_void;
    #[no_mangle]
    fn RCL_GetDriverParameter(instance: RCL_Instance) -> *mut libc::c_char;
    /* Create a new HW clock instance */
    #[no_mangle]
    fn HCL_CreateInstance(min_samples: libc::c_int, max_samples: libc::c_int,
                          min_separation: libc::c_double) -> HCL_Instance;
    /* Destroy a HW clock instance */
    #[no_mangle]
    fn HCL_DestroyInstance(clock: HCL_Instance);
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
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec)
     -> libc::c_double;
    /* Get 2 raised to power of a signed integer */
    #[no_mangle]
    fn UTI_Log2ToDouble(l: libc::c_int) -> libc::c_double;
    /* Register a handler for when select goes true on a file descriptor */
    #[no_mangle]
    fn SCH_AddFileHandler(fd: libc::c_int, events: libc::c_int,
                          handler: SCH_FileHandler,
                          arg: SCH_ArbitraryArgument);
    #[no_mangle]
    fn SCH_RemoveFileHandler(fd: libc::c_int);
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
    fn SYS_Linux_SetPHCExtTimestamping(fd: libc::c_int, pin: libc::c_int,
                                       channel: libc::c_int,
                                       rising: libc::c_int,
                                       falling: libc::c_int,
                                       enable: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SYS_Linux_ReadPHCExtTimestamp(fd: libc::c_int, phc_ts: *mut timespec,
                                     channel: *mut libc::c_int)
     -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type C2RustUnnamed = libc::c_uint;
pub const LEAP_Unsynchronised: C2RustUnnamed = 3;
pub const LEAP_DeleteSecond: C2RustUnnamed = 2;
pub const LEAP_InsertSecond: C2RustUnnamed = 1;
pub const LEAP_Normal: C2RustUnnamed = 0;
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
pub type SCH_ArbitraryArgument = *mut libc::c_void;
pub type SCH_FileHandler
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                _: SCH_ArbitraryArgument) -> ()>;
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2013, 2017
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

  PTP hardware clock (PHC) refclock driver.

  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct phc_instance {
    pub fd: libc::c_int,
    pub mode: libc::c_int,
    pub nocrossts: libc::c_int,
    pub extpps: libc::c_int,
    pub pin: libc::c_int,
    pub channel: libc::c_int,
    pub clock: HCL_Instance,
}
unsafe extern "C" fn phc_initialise(mut instance: RCL_Instance)
 -> libc::c_int {
    let mut options: [*const libc::c_char; 6] =
        [b"nocrossts\x00" as *const u8 as *const libc::c_char,
         b"extpps\x00" as *const u8 as *const libc::c_char,
         b"pin\x00" as *const u8 as *const libc::c_char,
         b"channel\x00" as *const u8 as *const libc::c_char,
         b"clear\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char];
    let mut phc: *mut phc_instance = 0 as *mut phc_instance;
    let mut phc_fd: libc::c_int = 0;
    let mut rising_edge: libc::c_int = 0;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    RCL_CheckDriverOptions(instance, options.as_mut_ptr());
    path = RCL_GetDriverParameter(instance);
    phc_fd = SYS_Linux_OpenPHC(path, 0 as libc::c_int);
    if phc_fd < 0 as libc::c_int {
        LOG_Message(LOGS_FATAL,
                    b"Could not open PHC\x00" as *const u8 as
                        *const libc::c_char);
        exit(1 as libc::c_int);
    }
    phc =
        Malloc(::std::mem::size_of::<phc_instance>() as libc::c_ulong) as
            *mut phc_instance;
    (*phc).fd = phc_fd;
    (*phc).mode = 0 as libc::c_int;
    (*phc).nocrossts =
        if !RCL_GetDriverOption(instance,
                                b"nocrossts\x00" as *const u8 as
                                    *const libc::c_char as
                                    *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*phc).extpps =
        if !RCL_GetDriverOption(instance,
                                b"extpps\x00" as *const u8 as
                                    *const libc::c_char as
                                    *mut libc::c_char).is_null() {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    if (*phc).extpps != 0 {
        s =
            RCL_GetDriverOption(instance,
                                b"pin\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char);
        (*phc).pin = if !s.is_null() { atoi(s) } else { 0 as libc::c_int };
        s =
            RCL_GetDriverOption(instance,
                                b"channel\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char);
        (*phc).channel =
            if !s.is_null() { atoi(s) } else { 0 as libc::c_int };
        rising_edge =
            if !RCL_GetDriverOption(instance,
                                    b"clear\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char).is_null() {
                0 as libc::c_int
            } else { 1 as libc::c_int };
        (*phc).clock =
            HCL_CreateInstance(0 as libc::c_int, 16 as libc::c_int,
                               UTI_Log2ToDouble(RCL_GetDriverPoll(instance)));
        if SYS_Linux_SetPHCExtTimestamping((*phc).fd, (*phc).pin,
                                           (*phc).channel, rising_edge,
                                           (rising_edge == 0) as libc::c_int,
                                           1 as libc::c_int) == 0 {
            LOG_Message(LOGS_FATAL,
                        b"Could not enable external PHC timestamping\x00" as
                            *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        SCH_AddFileHandler((*phc).fd, 1 as libc::c_int,
                           Some(read_ext_pulse as
                                    unsafe extern "C" fn(_: libc::c_int,
                                                         _: libc::c_int,
                                                         _: *mut libc::c_void)
                                        -> ()),
                           instance as SCH_ArbitraryArgument);
    } else {
        (*phc).channel = 0 as libc::c_int;
        (*phc).pin = (*phc).channel;
        (*phc).clock = 0 as HCL_Instance
    }
    RCL_SetDriverData(instance, phc as *mut libc::c_void);
    return 1 as libc::c_int;
}
unsafe extern "C" fn phc_finalise(mut instance: RCL_Instance) {
    let mut phc: *mut phc_instance = 0 as *mut phc_instance;
    phc = RCL_GetDriverData(instance) as *mut phc_instance;
    if (*phc).extpps != 0 {
        SCH_RemoveFileHandler((*phc).fd);
        SYS_Linux_SetPHCExtTimestamping((*phc).fd, (*phc).pin, (*phc).channel,
                                        0 as libc::c_int, 0 as libc::c_int,
                                        0 as libc::c_int);
        HCL_DestroyInstance((*phc).clock);
    }
    close((*phc).fd);
    free(phc as *mut libc::c_void);
}
unsafe extern "C" fn read_ext_pulse(mut fd: libc::c_int,
                                    mut event: libc::c_int,
                                    mut anything: *mut libc::c_void) {
    let mut instance: RCL_Instance = 0 as *mut RCL_Instance_Record;
    let mut phc: *mut phc_instance = 0 as *mut phc_instance;
    let mut phc_ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut local_ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut local_err: libc::c_double = 0.;
    let mut channel: libc::c_int = 0;
    instance = anything as RCL_Instance;
    phc = RCL_GetDriverData(instance) as *mut phc_instance;
    if SYS_Linux_ReadPHCExtTimestamp((*phc).fd, &mut phc_ts, &mut channel) ==
           0 {
        return
    }
    if channel != (*phc).channel {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Unexpected extts channel %d\n\x00" as *const u8 as
                            *const libc::c_char, channel);
        }
        return
    }
    if HCL_CookTime((*phc).clock, &mut phc_ts, &mut local_ts, &mut local_err)
           == 0 {
        return
    }
    RCL_AddCookedPulse(instance, &mut local_ts,
                       1.0e-9f64 * local_ts.tv_nsec as libc::c_double,
                       local_err,
                       UTI_DiffTimespecsToDouble(&mut phc_ts, &mut local_ts));
}
unsafe extern "C" fn phc_poll(mut instance: RCL_Instance) -> libc::c_int {
    let mut phc: *mut phc_instance = 0 as *mut phc_instance;
    let mut phc_ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut sys_ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut local_ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut offset: libc::c_double = 0.;
    let mut phc_err: libc::c_double = 0.;
    let mut local_err: libc::c_double = 0.;
    phc = RCL_GetDriverData(instance) as *mut phc_instance;
    if SYS_Linux_GetPHCSample((*phc).fd, (*phc).nocrossts,
                              RCL_GetPrecision(instance), &mut (*phc).mode,
                              &mut phc_ts, &mut sys_ts, &mut phc_err) == 0 {
        return 0 as libc::c_int
    }
    if (*phc).extpps != 0 {
        LCL_CookTime(&mut sys_ts, &mut local_ts, &mut local_err);
        HCL_AccumulateSample((*phc).clock, &mut phc_ts, &mut local_ts,
                             phc_err + local_err);
        return 0 as libc::c_int
    }
    offset = UTI_DiffTimespecsToDouble(&mut phc_ts, &mut sys_ts);
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"PHC offset: %+.9f err: %.9f\x00" as *const u8 as
                        *const libc::c_char, offset, phc_err);
    }
    return RCL_AddSample(instance, &mut sys_ts, offset,
                         LEAP_Normal as libc::c_int);
}
#[no_mangle]
pub static mut RCL_PHC_driver: RefclockDriver =
    unsafe {
        {
            let mut init =
                RefclockDriver{init:
                                   Some(phc_initialise as
                                            unsafe extern "C" fn(_:
                                                                     RCL_Instance)
                                                -> libc::c_int),
                               fini:
                                   Some(phc_finalise as
                                            unsafe extern "C" fn(_:
                                                                     RCL_Instance)
                                                -> ()),
                               poll:
                                   Some(phc_poll as
                                            unsafe extern "C" fn(_:
                                                                     RCL_Instance)
                                                -> libc::c_int),};
            init
        }
    };
