#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           register_tool)]
extern "C" {
    pub type ARR_Instance_Record;
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

  Header file for module that deals with the measurements and statistics of
  each of the sources. */
    pub type SST_Stats_Record;
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
    pub type SRC_Instance_Record;
    pub type SPF_Instance_Record;
    pub type ADF_AuthTableInst;
    #[no_mangle]
    fn log(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    #[no_mangle]
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    #[no_mangle]
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
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
    /* Return the assumed worst case range of values that this source's
   frequency lies within.  Frequency is defined as the amount of time
   the local clock gains relative to the source per unit local clock
   time. */
    #[no_mangle]
    fn SST_GetFrequencyRange(inst: SST_Stats, lo: *mut libc::c_double,
                             hi: *mut libc::c_double);
    /* Predict the offset of the local clock relative to a given source at
   a given local cooked time. Positive indicates local clock is FAST
   relative to reference. */
    #[no_mangle]
    fn SST_PredictOffset(inst: SST_Stats, when: *mut timespec)
     -> libc::c_double;
    /* Find the minimum round trip delay in the register */
    #[no_mangle]
    fn SST_MinRoundTripDelay(inst: SST_Stats) -> libc::c_double;
    /* Get data needed for testing NTP delay */
    #[no_mangle]
    fn SST_GetDelayTestData(inst: SST_Stats, sample_time: *mut timespec,
                            last_sample_ago: *mut libc::c_double,
                            predicted_offset: *mut libc::c_double,
                            min_delay: *mut libc::c_double,
                            skew: *mut libc::c_double,
                            std_dev: *mut libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn SST_Samples(inst: SST_Stats) -> libc::c_int;
    #[no_mangle]
    fn SST_GetJitterAsymmetry(inst: SST_Stats) -> libc::c_double;
    /* Function to create a new instance.  This would be called by one of
   the individual source-type instance creation routines. */
    #[no_mangle]
    fn SRC_CreateNewInstance(ref_id: uint32_t, type_0: SRC_Type,
                             sel_options: libc::c_int, addr: *mut IPAddr,
                             min_samples: libc::c_int,
                             max_samples: libc::c_int,
                             min_delay: libc::c_double,
                             asymmetry: libc::c_double) -> SRC_Instance;
    /* Function to get rid of a source when it is being unconfigured.
   This may cause the current reference source to be reselected, if this
   was the reference source or contributed significantly to a
   falseticker decision. */
    #[no_mangle]
    fn SRC_DestroyInstance(instance: SRC_Instance);
    /* Function to reset a source */
    #[no_mangle]
    fn SRC_ResetInstance(instance: SRC_Instance);
    /* Function to change the sources's reference ID and IP address */
    #[no_mangle]
    fn SRC_SetRefid(instance: SRC_Instance, ref_id: uint32_t,
                    addr: *mut IPAddr);
    /* Function to get access to the sourcestats instance */
    #[no_mangle]
    fn SRC_GetSourcestats(instance: SRC_Instance) -> SST_Stats;
    /* This function is called by one of the source drivers when it has
   a new sample that is to be accumulated */
    #[no_mangle]
    fn SRC_AccumulateSample(instance: SRC_Instance, sample: *mut NTP_Sample);
    /* This routine sets the source as receiving reachability updates */
    #[no_mangle]
    fn SRC_SetActive(inst: SRC_Instance);
    /* This routine sets the source as not receiving reachability updates */
    #[no_mangle]
    fn SRC_UnsetActive(inst: SRC_Instance);
    /* This routine updates the reachability register */
    #[no_mangle]
    fn SRC_UpdateReachability(inst: SRC_Instance, reachable: libc::c_int);
    /* This routine marks the source unreachable */
    #[no_mangle]
    fn SRC_ResetReachability(inst: SRC_Instance);
    /* This routine is used to select the best source from amongst those
   we currently have valid data on, and use it as the tracking base
   for the local time.  Updates are made to the local reference only
   when the selected source was updated (set as updated_inst) since
   the last reference update.  This avoids updating the frequency
   tracking for every sample from other sources - only the ones from
   the selected reference make a difference. */
    #[no_mangle]
    fn SRC_SelectSource(updated_inst: SRC_Instance);
    #[no_mangle]
    fn SRC_IsSyncPeer(inst: SRC_Instance) -> libc::c_int;
    #[no_mangle]
    fn SRC_IsReachable(inst: SRC_Instance) -> libc::c_int;
    /* Function to obtain a socket for sending client packets */
    #[no_mangle]
    fn NIO_OpenClientSocket(remote_addr: *mut NTP_Remote_Address)
     -> libc::c_int;
    /* Function to obtain a socket for sending server/peer packets */
    #[no_mangle]
    fn NIO_OpenServerSocket(remote_addr: *mut NTP_Remote_Address)
     -> libc::c_int;
    /* Function to close a socket returned by NIO_OpenClientSocket() */
    #[no_mangle]
    fn NIO_CloseClientSocket(sock_fd: libc::c_int);
    /* Function to close a socket returned by NIO_OpenServerSocket() */
    #[no_mangle]
    fn NIO_CloseServerSocket(sock_fd: libc::c_int);
    /* Function to check if socket is a server socket */
    #[no_mangle]
    fn NIO_IsServerSocket(sock_fd: libc::c_int) -> libc::c_int;
    /* Function to check if a server socket is currently open */
    #[no_mangle]
    fn NIO_IsServerSocketOpen() -> libc::c_int;
    /* Function to check if client packets can be sent to a server */
    #[no_mangle]
    fn NIO_IsServerConnectable(remote_addr: *mut NTP_Remote_Address)
     -> libc::c_int;
    /* Function to transmit a packet */
    #[no_mangle]
    fn NIO_SendPacket(packet: *mut NTP_Packet,
                      remote_addr: *mut NTP_Remote_Address,
                      local_addr: *mut NTP_Local_Address, length: libc::c_int,
                      process_tx: libc::c_int) -> libc::c_int;
    /* Function to get an estimate of delay due to signing */
    #[no_mangle]
    fn NSD_GetAuthDelay(key_id: uint32_t) -> libc::c_int;
    /* Function to sign an NTP packet and send it */
    #[no_mangle]
    fn NSD_SignAndSendPacket(key_id: uint32_t, packet: *mut NTP_Packet,
                             remote_addr: *mut NTP_Remote_Address,
                             local_addr: *mut NTP_Local_Address,
                             length: libc::c_int) -> libc::c_int;
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
    /* Get the time stamp taken after a file descriptor became ready or a timeout expired */
    #[no_mangle]
    fn SCH_GetLastEventTime(cooked: *mut timespec, err: *mut libc::c_double,
                            raw: *mut timespec);
    /* This queues a timeout to elapse at a given delta time relative to the current (raw) time */
    #[no_mangle]
    fn SCH_AddTimeoutByDelay(delay: libc::c_double, _: SCH_TimeoutHandler,
                             _: SCH_ArbitraryArgument) -> SCH_TimeoutID;
    /* This queues a timeout in a particular class, ensuring that the
   expiry time is at least a given separation away from any other
   timeout in the same class, given randomness is added to the delay
   and separation */
    #[no_mangle]
    fn SCH_AddTimeoutInClass(min_delay: libc::c_double,
                             separation: libc::c_double,
                             randomness: libc::c_double,
                             class: SCH_TimeoutClass,
                             handler: SCH_TimeoutHandler,
                             _: SCH_ArbitraryArgument) -> SCH_TimeoutID;
    /* The next one probably ought to return a status code */
    #[no_mangle]
    fn SCH_RemoveTimeout(_: SCH_TimeoutID);
    /* Get leap second handling mode */
    #[no_mangle]
    fn REF_GetLeapMode() -> REF_LeapMode;
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
    fn REF_GetReferenceParams(local_time: *mut timespec,
                              is_synchronised: *mut libc::c_int,
                              leap: *mut NTP_Leap, stratum: *mut libc::c_int,
                              ref_id: *mut uint32_t, ref_time: *mut timespec,
                              root_delay: *mut libc::c_double,
                              root_dispersion: *mut libc::c_double);
    /* Return the current stratum of this host or 16 if the host is not
   synchronised */
    #[no_mangle]
    fn REF_GetOurStratum() -> libc::c_int;
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

  This module provides an interface to the system time, and
  insulates the rest of the program from the different way
  that interface has to be done on various operating systems.
  */
    /* Read the system clock */
    #[no_mangle]
    fn LCL_ReadRawTime(ts: *mut timespec);
    /* Read the system clock, corrected according to all accumulated
   drifts and uncompensated offsets.

   In a kernel implementation with vernier frequency control (like
   Linux), and if we were to apply offsets by stepping the clock, this
   would be identical to raw time.  In any other case (use of
   adjtime()-like interface to correct offsets, and to adjust the
   frequency), we must correct the raw time to get this value */
    #[no_mangle]
    fn LCL_ReadCookedTime(ts: *mut timespec, err: *mut libc::c_double);
    /* Routine to read the system precision as a log to base 2 value. */
    #[no_mangle]
    fn LCL_GetSysPrecisionAsLog() -> libc::c_int;
    /* Routine to read the system precision in terms of the actual time step */
    #[no_mangle]
    fn LCL_GetSysPrecisionAsQuantum() -> libc::c_double;
    /* Routine to read the maximum frequency error of the local clock.  This
   is a frequency stability, not an absolute error. */
    #[no_mangle]
    fn LCL_GetMaxClockError() -> libc::c_double;
    #[no_mangle]
    fn SPF_CreateInstance(min_samples: libc::c_int, max_samples: libc::c_int,
                          max_dispersion: libc::c_double,
                          combine_ratio: libc::c_double) -> SPF_Instance;
    #[no_mangle]
    fn SPF_DestroyInstance(filter: SPF_Instance);
    #[no_mangle]
    fn SPF_AccumulateSample(filter: SPF_Instance, sample: *mut NTP_Sample)
     -> libc::c_int;
    #[no_mangle]
    fn SPF_GetNumberOfSamples(filter: SPF_Instance) -> libc::c_int;
    #[no_mangle]
    fn SPF_DropSamples(filter: SPF_Instance);
    #[no_mangle]
    fn SPF_GetFilteredSample(filter: SPF_Instance, sample: *mut NTP_Sample)
     -> libc::c_int;
    #[no_mangle]
    fn SPF_SlewSamples(filter: SPF_Instance, when: *mut timespec,
                       dfreq: libc::c_double, doffset: libc::c_double);
    #[no_mangle]
    fn SMT_IsEnabled() -> libc::c_int;
    #[no_mangle]
    fn SMT_GetOffset(now: *mut timespec) -> libc::c_double;
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

  Various utility functions
  */
    /* Zero a timespec */
    #[no_mangle]
    fn UTI_ZeroTimespec(ts: *mut timespec);
    /* Check if a timespec is zero */
    #[no_mangle]
    fn UTI_IsZeroTimespec(ts: *mut timespec) -> libc::c_int;
    /* Normalise a timespec, by adding or subtracting seconds to bring
   its nanosecond field into range */
    #[no_mangle]
    fn UTI_NormaliseTimespec(ts: *mut timespec);
    /* Calculate result = a - b and return as a double */
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec)
     -> libc::c_double;
    /* Add a double increment to a timespec to get a new one. 'start' is
   the starting time, 'end' is the result that we return.  This is
   safe to use if start and end are the same */
    #[no_mangle]
    fn UTI_AddDoubleToTimespec(start: *mut timespec,
                               increment: libc::c_double, end: *mut timespec);
    /* Calculate the average and difference (as a double) of two timespecs */
    #[no_mangle]
    fn UTI_AverageDiffTimespecs(earlier: *mut timespec, later: *mut timespec,
                                average: *mut timespec,
                                diff: *mut libc::c_double);
    /* Convert an NTP timestamp into a temporary string, largely for
   diagnostic display */
    #[no_mangle]
    fn UTI_Ntp64ToString(ts: *mut NTP_int64) -> *mut libc::c_char;
    /* Convert ref_id into a temporary string, for diagnostics */
    #[no_mangle]
    fn UTI_RefidToString(ref_id: uint32_t) -> *mut libc::c_char;
    /* Convert an IP address to string, for diagnostics */
    #[no_mangle]
    fn UTI_IPToString(ip: *mut IPAddr) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_IPToRefid(ip: *mut IPAddr) -> uint32_t;
    #[no_mangle]
    fn UTI_IPSockAddrToString(sa: *mut IPSockAddr) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_TimeToLogForm(t: time_t) -> *mut libc::c_char;
    /* Adjust time following a frequency/offset change */
    #[no_mangle]
    fn UTI_AdjustTimespec(old_ts: *mut timespec, when: *mut timespec,
                          new_ts: *mut timespec,
                          delta_time: *mut libc::c_double,
                          dfreq: libc::c_double, doffset: libc::c_double);
    /* Get zero NTP timestamp with random bits below precision */
    #[no_mangle]
    fn UTI_GetNtp64Fuzz(ts: *mut NTP_int64, precision: libc::c_int);
    #[no_mangle]
    fn UTI_Ntp32ToDouble(x: NTP_int32) -> libc::c_double;
    #[no_mangle]
    fn UTI_DoubleToNtp32(x: libc::c_double) -> NTP_int32;
    /* Zero an NTP timestamp */
    #[no_mangle]
    fn UTI_ZeroNtp64(ts: *mut NTP_int64);
    /* Check if an NTP timestamp is zero */
    #[no_mangle]
    fn UTI_IsZeroNtp64(ts: *mut NTP_int64) -> libc::c_int;
    /* Compare two NTP timestamps.  Returns -1 if a is before b, 0 if a is equal to
   b, and 1 if a is after b. */
    #[no_mangle]
    fn UTI_CompareNtp64(a: *mut NTP_int64, b: *mut NTP_int64) -> libc::c_int;
    /* Compare an NTP timestamp with up to three other timestamps.  Returns 0
   if a is not equal to any of b1, b2, and b3, 1 otherwise. */
    #[no_mangle]
    fn UTI_IsEqualAnyNtp64(a: *mut NTP_int64, b1: *mut NTP_int64,
                           b2: *mut NTP_int64, b3: *mut NTP_int64)
     -> libc::c_int;
    /* Convert a timespec into an NTP timestamp */
    #[no_mangle]
    fn UTI_TimespecToNtp64(src: *mut timespec, dest: *mut NTP_int64,
                           fuzz: *mut NTP_int64);
    /* Convert an NTP timestamp into a timespec */
    #[no_mangle]
    fn UTI_Ntp64ToTimespec(src: *mut NTP_int64, dest: *mut timespec);
    /* Get 2 raised to power of a signed integer */
    #[no_mangle]
    fn UTI_Log2ToDouble(l: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn CNF_GetLogMeasurements(raw: *mut libc::c_int) -> libc::c_int;
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
    #[no_mangle]
    fn LOG_FileOpen(name: *const libc::c_char, banner: *const libc::c_char)
     -> LOG_FileID;
    #[no_mangle]
    fn LOG_FileWrite(id: LOG_FileID, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn KEY_KeyKnown(key_id: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn KEY_GetAuthDelay(key_id: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn KEY_GetAuthLength(key_id: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn KEY_CheckKeyLength(key_id: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn KEY_GenerateAuth(key_id: uint32_t, data: *const libc::c_uchar,
                        data_len: libc::c_int, auth: *mut libc::c_uchar,
                        auth_len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn KEY_CheckAuth(key_id: uint32_t, data: *const libc::c_uchar,
                     data_len: libc::c_int, auth: *const libc::c_uchar,
                     auth_len: libc::c_int, trunc_len: libc::c_int)
     -> libc::c_int;
    /* Create a new table.  The default rule is deny for everything */
    #[no_mangle]
    fn ADF_CreateTable() -> ADF_AuthTable;
    /* Allow anything in the supplied subnet, EXCEPT for any more specific
   subnets that are already defined */
    #[no_mangle]
    fn ADF_Allow(table: ADF_AuthTable, ip: *mut IPAddr,
                 subnet_bits: libc::c_int) -> ADF_Status;
    /* Allow anything in the supplied subnet, overwriting existing
   definitions for any more specific subnets */
    #[no_mangle]
    fn ADF_AllowAll(table: ADF_AuthTable, ip: *mut IPAddr,
                    subnet_bits: libc::c_int) -> ADF_Status;
    /* Deny anything in the supplied subnet, EXCEPT for any more specific
   subnets that are already defined */
    #[no_mangle]
    fn ADF_Deny(table: ADF_AuthTable, ip: *mut IPAddr,
                subnet_bits: libc::c_int) -> ADF_Status;
    /* Deny anything in the supplied subnet, overwriting existing
   definitions for any more specific subnets */
    #[no_mangle]
    fn ADF_DenyAll(table: ADF_AuthTable, ip: *mut IPAddr,
                   subnet_bits: libc::c_int) -> ADF_Status;
    /* Clear up the table */
    #[no_mangle]
    fn ADF_DestroyTable(table: ADF_AuthTable);
    /* Check whether a given IP address is allowed by the rules in 
   the table */
    #[no_mangle]
    fn ADF_IsAllowed(table: ADF_AuthTable, ip: *mut IPAddr) -> libc::c_int;
    /* Check if at least one address from a given family is allowed by
   the rules in the table */
    #[no_mangle]
    fn ADF_IsAnyAllowed(table: ADF_AuthTable, family: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn CLG_GetClientIndex(client: *mut IPAddr) -> libc::c_int;
    #[no_mangle]
    fn CLG_LogNTPAccess(client: *mut IPAddr, now: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn CLG_LimitNTPResponseRate(index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn CLG_GetNtpTimestamps(index: libc::c_int, rx_ts: *mut *mut NTP_int64,
                            tx_ts: *mut *mut NTP_int64);
    #[no_mangle]
    fn CLG_GetNtpMinPoll() -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
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
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_int64 {
    pub hi: uint32_t,
    pub lo: uint32_t,
}
pub type NTP_int32 = uint32_t;
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
pub type NTP_Mode = libc::c_uint;
pub const MODE_BROADCAST: NTP_Mode = 5;
pub const MODE_SERVER: NTP_Mode = 4;
pub const MODE_CLIENT: NTP_Mode = 3;
pub const MODE_PASSIVE: NTP_Mode = 2;
pub const MODE_ACTIVE: NTP_Mode = 1;
pub const MODE_UNDEFINED: NTP_Mode = 0;
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

  Data structure definitions within the daemon for various reports that
  can be generated */
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
pub type SST_Stats = *mut SST_Stats_Record;
pub type SRC_Instance = *mut SRC_Instance_Record;
pub type SRC_Type = libc::c_uint;
/* Rerefence clock */
/* NTP client/peer */
pub const SRC_REFCLOCK: SRC_Type = 1;
pub const SRC_NTP: SRC_Type = 0;
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
/* ================================================== */
/* Structure used for holding a single peer/server's
   protocol machine */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NCR_Instance_Record {
    pub remote_addr: NTP_Remote_Address,
    pub local_addr: NTP_Local_Address,
    pub mode: NTP_Mode,
    pub interleaved: libc::c_int,
    pub opmode: OperatingMode,
    pub rx_timeout_id: SCH_TimeoutID,
    pub tx_timeout_id: SCH_TimeoutID,
    pub tx_suspended: libc::c_int,
    pub auto_burst: libc::c_int,
    pub auto_offline: libc::c_int,
    pub local_poll: libc::c_int,
    pub remote_poll: libc::c_int,
    pub remote_stratum: libc::c_int,
    pub presend_minpoll: libc::c_int,
    pub presend_done: libc::c_int,
    pub minpoll: libc::c_int,
    pub maxpoll: libc::c_int,
    pub min_stratum: libc::c_int,
    pub poll_target: libc::c_int,
    pub version: libc::c_int,
    pub poll_score: libc::c_double,
    pub max_delay: libc::c_double,
    pub max_delay_ratio: libc::c_double,
    pub max_delay_dev_ratio: libc::c_double,
    pub offset_correction: libc::c_double,
    pub auth_mode: AuthenticationMode,
    pub auth_key_id: uint32_t,
    pub tx_count: libc::c_uint,
    pub valid_rx: libc::c_int,
    pub valid_timestamps: libc::c_int,
    pub remote_ntp_rx: NTP_int64,
    pub remote_ntp_tx: NTP_int64,
    pub local_ntp_rx: NTP_int64,
    pub local_rx: NTP_Local_Timestamp,
    pub local_ntp_tx: NTP_int64,
    pub local_tx: NTP_Local_Timestamp,
    pub prev_local_tx: NTP_Local_Timestamp,
    pub prev_local_poll: libc::c_int,
    pub prev_tx_count: libc::c_uint,
    pub updated_init_timestamps: libc::c_int,
    pub init_remote_ntp_tx: NTP_int64,
    pub init_local_rx: NTP_Local_Timestamp,
    pub source: SRC_Instance,
    pub filter: SPF_Instance,
    pub burst_good_samples_to_go: libc::c_int,
    pub burst_total_samples_to_go: libc::c_int,
    pub report: RPT_NTPReport,
}
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
pub type AuthenticationMode = libc::c_uint;
pub const AUTH_MSSNTP_EXT: AuthenticationMode = 3;
pub const AUTH_MSSNTP: AuthenticationMode = 2;
pub const AUTH_SYMMETRIC: AuthenticationMode = 1;
pub const AUTH_NONE: AuthenticationMode = 0;
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
pub type OperatingMode = libc::c_uint;
pub const MD_BURST_WAS_ONLINE: OperatingMode = 3;
pub const MD_BURST_WAS_OFFLINE: OperatingMode = 2;
pub const MD_ONLINE: OperatingMode = 1;
pub const MD_OFFLINE: OperatingMode = 0;
pub type NCR_Instance = *mut NCR_Instance_Record;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BroadcastDestination {
    pub addr: NTP_Remote_Address,
    pub local_addr: NTP_Local_Address,
    pub interval: libc::c_int,
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

  Module for providing an authorisation filter on IP addresses
  */
pub type ADF_AuthTable = *mut ADF_AuthTableInst;
/* File logging functions */
pub type LOG_FileID = libc::c_int;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
pub type SCH_ArbitraryArgument = *mut libc::c_void;
pub type SCH_TimeoutHandler
    =
    Option<unsafe extern "C" fn(_: SCH_ArbitraryArgument) -> ()>;
pub const REF_LeapModeSlew: REF_LeapMode = 1;
pub type REF_LeapMode = libc::c_uint;
pub const REF_LeapModeIgnore: REF_LeapMode = 3;
pub const REF_LeapModeStep: REF_LeapMode = 2;
pub const REF_LeapModeSystem: REF_LeapMode = 0;
pub type SCH_TimeoutClass = libc::c_uint;
pub const SCH_NumberOfClasses: SCH_TimeoutClass = 4;
pub const SCH_NtpBroadcastClass: SCH_TimeoutClass = 3;
pub const SCH_NtpPeerClass: SCH_TimeoutClass = 2;
pub const SCH_NtpClientClass: SCH_TimeoutClass = 1;
pub const SCH_ReservedTimeoutValue: SCH_TimeoutClass = 0;
pub const ADF_SUCCESS: ADF_Status = 0;
pub type ADF_Status = libc::c_uint;
pub const ADF_BADSUBNET: ADF_Status = 1;
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2009-2018
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

  Core NTP protocol engine
  */
/* ================================================== */
static mut logfileid: LOG_FileID = 0;
static mut log_raw_measurements: libc::c_int = 0;
/* Array of BroadcastDestination */
static mut broadcasts: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
/* ================================================== */
/* Server IPv4/IPv6 sockets */
static mut server_sock_fd4: libc::c_int = 0;
static mut server_sock_fd6: libc::c_int = 0;
static mut access_auth_table: ADF_AuthTable =
    0 as *const ADF_AuthTableInst as *mut ADF_AuthTableInst;
/* Characters for printing synchronisation status and timestamping source */
static mut leap_chars: [libc::c_char; 4] =
    ['N' as i32 as libc::c_char, '+' as i32 as libc::c_char,
     '-' as i32 as libc::c_char, '?' as i32 as libc::c_char];
static mut tss_chars: [libc::c_char; 3] =
    ['D' as i32 as libc::c_char, 'K' as i32 as libc::c_char,
     'H' as i32 as libc::c_char];
/* ================================================== */
unsafe extern "C" fn do_size_checks() {
    /* Assertions to check the sizes of certain data types
     and the positions of certain record fields */
    /* Check that certain invariants are true */
    if ::std::mem::size_of::<NTP_int32>() as libc::c_ulong ==
           4 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"sizeof(NTP_int32) == 4\x00" as *const u8 as
                          *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      324 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_size_checks(void)\x00")).as_ptr());
    }
    if ::std::mem::size_of::<NTP_int64>() as libc::c_ulong ==
           8 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"sizeof(NTP_int64) == 8\x00" as *const u8 as
                          *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      325 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_size_checks(void)\x00")).as_ptr());
    }
    /* Check offsets of all fields in the NTP packet format */
    if 0 as libc::c_ulong == 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"offsetof(NTP_Packet, lvm) == 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      328 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_size_checks(void)\x00")).as_ptr());
    }
    if 1 as libc::c_ulong == 1 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"offsetof(NTP_Packet, stratum) == 1\x00" as *const u8
                          as *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      329 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_size_checks(void)\x00")).as_ptr());
    }
    if 2 as libc::c_ulong == 2 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"offsetof(NTP_Packet, poll) == 2\x00" as *const u8 as
                          *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      330 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_size_checks(void)\x00")).as_ptr());
    }
    if 3 as libc::c_ulong == 3 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"offsetof(NTP_Packet, precision) == 3\x00" as *const u8
                          as *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      331 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_size_checks(void)\x00")).as_ptr());
    }
    if 4 as libc::c_ulong == 4 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"offsetof(NTP_Packet, root_delay) == 4\x00" as
                          *const u8 as *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      332 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_size_checks(void)\x00")).as_ptr());
    }
    if 8 as libc::c_ulong == 8 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"offsetof(NTP_Packet, root_dispersion) == 8\x00" as
                          *const u8 as *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      333 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_size_checks(void)\x00")).as_ptr());
    }
    if 12 as libc::c_ulong == 12 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"offsetof(NTP_Packet, reference_id) == 12\x00" as
                          *const u8 as *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      334 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_size_checks(void)\x00")).as_ptr());
    }
    if 16 as libc::c_ulong == 16 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"offsetof(NTP_Packet, reference_ts) == 16\x00" as
                          *const u8 as *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      335 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_size_checks(void)\x00")).as_ptr());
    }
    if 24 as libc::c_ulong == 24 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"offsetof(NTP_Packet, originate_ts) == 24\x00" as
                          *const u8 as *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      336 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_size_checks(void)\x00")).as_ptr());
    }
    if 32 as libc::c_ulong == 32 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"offsetof(NTP_Packet, receive_ts) == 32\x00" as
                          *const u8 as *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      337 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_size_checks(void)\x00")).as_ptr());
    }
    if 40 as libc::c_ulong == 40 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"offsetof(NTP_Packet, transmit_ts) == 40\x00" as
                          *const u8 as *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      338 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_size_checks(void)\x00")).as_ptr());
    };
}
/* ================================================== */
unsafe extern "C" fn do_time_checks() {
    let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,}; /* 10 years */
    let mut warning_advance: time_t =
        (3600 as libc::c_int * 24 as libc::c_int * 365 as libc::c_int *
             10 as libc::c_int) as time_t;
    /* Check that time before NTP_ERA_SPLIT underflows correctly */
    let mut ts1: timespec =
        {
            let mut init =
                timespec{tv_sec:
                             (1581921726 as libc::c_longlong -
                                  (18250 as libc::c_int * 24 as libc::c_int *
                                       3600 as libc::c_int) as
                                      libc::c_longlong) as __time_t,
                         tv_nsec: 1 as libc::c_int as __syscall_slong_t,};
            init
        };
    let mut ts2: timespec =
        {
            let mut init =
                timespec{tv_sec:
                             (1581921726 as libc::c_longlong -
                                  (18250 as libc::c_int * 24 as libc::c_int *
                                       3600 as libc::c_int) as
                                      libc::c_longlong -
                                  1 as libc::c_int as libc::c_longlong) as
                                 __time_t,
                         tv_nsec: 1 as libc::c_int as __syscall_slong_t,};
            init
        };
    let mut nts1: NTP_int64 = NTP_int64{hi: 0, lo: 0,};
    let mut nts2: NTP_int64 = NTP_int64{hi: 0, lo: 0,};
    let mut r: libc::c_int = 0;
    UTI_TimespecToNtp64(&mut ts1, &mut nts1, 0 as *mut NTP_int64);
    UTI_TimespecToNtp64(&mut ts2, &mut nts2, 0 as *mut NTP_int64);
    UTI_Ntp64ToTimespec(&mut nts1, &mut ts1);
    UTI_Ntp64ToTimespec(&mut nts2, &mut ts2);
    r =
        (ts1.tv_sec as libc::c_longlong ==
             1581921726 as libc::c_longlong -
                 (18250 as libc::c_int * 24 as libc::c_int *
                      3600 as libc::c_int) as libc::c_longlong &&
             (ts1.tv_sec as
                  libc::c_ulonglong).wrapping_add((1 as libc::c_ulonglong) <<
                                                      32 as
                                                          libc::c_int).wrapping_sub(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulonglong)
                 == ts2.tv_sec as libc::c_ulonglong) as libc::c_int;
    if r != 0 {
    } else {
        __assert_fail(b"r\x00" as *const u8 as *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      364 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void do_time_checks(void)\x00")).as_ptr());
    }
    LCL_ReadRawTime(&mut now);
    if ts2.tv_sec - now.tv_sec < warning_advance {
        LOG_Message(LOGS_WARN,
                    b"Assumed NTP time ends at %s!\x00" as *const u8 as
                        *const libc::c_char, UTI_TimeToLogForm(ts2.tv_sec));
    };
}
/* ================================================== */
unsafe extern "C" fn zero_local_timestamp(mut ts: *mut NTP_Local_Timestamp) {
    UTI_ZeroTimespec(&mut (*ts).ts);
    (*ts).err = 0.0f64;
    (*ts).source = NTP_TS_DAEMON;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_Initialise() {
    do_size_checks();
    do_time_checks();
    logfileid =
        if CNF_GetLogMeasurements(&mut log_raw_measurements) != 0 {
            LOG_FileOpen(b"measurements\x00" as *const u8 as
                             *const libc::c_char,
                         b"   Date (UTC) Time     IP Address   L St 123 567 ABCD  LP RP Score    Offset  Peer del. Peer disp.  Root del. Root disp. Refid     MTxRx\x00"
                             as *const u8 as *const libc::c_char)
        } else { -(1 as libc::c_int) };
    access_auth_table = ADF_CreateTable();
    broadcasts =
        ARR_CreateInstance(::std::mem::size_of::<BroadcastDestination>() as
                               libc::c_ulong as libc::c_uint);
    /* Server socket will be opened when access is allowed */
    server_sock_fd4 = -(2 as libc::c_int);
    server_sock_fd6 = -(2 as libc::c_int);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_Finalise() {
    let mut i: libc::c_uint = 0;
    if server_sock_fd4 != -(2 as libc::c_int) {
        NIO_CloseServerSocket(server_sock_fd4);
    }
    if server_sock_fd6 != -(2 as libc::c_int) {
        NIO_CloseServerSocket(server_sock_fd6);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(broadcasts) {
        NIO_CloseServerSocket((*(ARR_GetElement(broadcasts, i) as
                                     *mut BroadcastDestination)).local_addr.sock_fd);
        i = i.wrapping_add(1)
    }
    ARR_DestroyInstance(broadcasts);
    ADF_DestroyTable(access_auth_table);
}
/* ================================================== */
unsafe extern "C" fn restart_timeout(mut inst: NCR_Instance,
                                     mut delay: libc::c_double) {
    /* Check if we can transmit */
    if (*inst).tx_suspended != 0 {
        if (*inst).tx_timeout_id == 0 {
        } else {
            __assert_fail(b"!inst->tx_timeout_id\x00" as *const u8 as
                              *const libc::c_char,
                          b"ntp_core.c\x00" as *const u8 as
                              *const libc::c_char,
                          432 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 43],
                                                    &[libc::c_char; 43]>(b"void restart_timeout(NCR_Instance, double)\x00")).as_ptr());
        }
        return
    }
    /* Stop both rx and tx timers if running */
    SCH_RemoveTimeout((*inst).rx_timeout_id);
    (*inst).rx_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    SCH_RemoveTimeout((*inst).tx_timeout_id);
    /* Start new timer for transmission */
    (*inst).tx_timeout_id =
        SCH_AddTimeoutInClass(delay, get_separation((*inst).local_poll),
                              0.02f64,
                              if (*inst).mode as libc::c_uint ==
                                     MODE_CLIENT as libc::c_int as
                                         libc::c_uint {
                                  SCH_NtpClientClass as libc::c_int
                              } else { SCH_NtpPeerClass as libc::c_int } as
                                  SCH_TimeoutClass,
                              Some(transmit_timeout as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           -> ()), inst as *mut libc::c_void);
}
/* ================================================== */
unsafe extern "C" fn start_initial_timeout(mut inst: NCR_Instance) {
    let mut delay: libc::c_double = 0.;
    let mut last_tx: libc::c_double = 0.;
    let mut now: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    if (*inst).tx_timeout_id == 0 {
        /* This will be the first transmission after mode change */
        /* Mark source active */
        SRC_SetActive((*inst).source);
    }
    /* In case the offline period was too short, adjust the delay to keep
     the interval between packets at least as long as the current polling
     interval */
    SCH_GetLastEventTime(&mut now, 0 as *mut libc::c_double,
                         0 as *mut timespec);
    last_tx = UTI_DiffTimespecsToDouble(&mut now, &mut (*inst).local_tx.ts);
    if last_tx < 0.0f64 { last_tx = 0.0f64 }
    delay = get_transmit_delay(inst, 0 as libc::c_int, 0.0f64) - last_tx;
    if delay < 0.2f64 { delay = 0.2f64 }
    restart_timeout(inst, delay);
}
/* ================================================== */
unsafe extern "C" fn close_client_socket(mut inst: NCR_Instance) {
    if (*inst).mode as libc::c_uint ==
           MODE_CLIENT as libc::c_int as libc::c_uint &&
           (*inst).local_addr.sock_fd != -(2 as libc::c_int) {
        NIO_CloseClientSocket((*inst).local_addr.sock_fd);
        (*inst).local_addr.sock_fd = -(2 as libc::c_int)
    }
    SCH_RemoveTimeout((*inst).rx_timeout_id);
    (*inst).rx_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
}
/* ================================================== */
unsafe extern "C" fn take_offline(mut inst: NCR_Instance) {
    (*inst).opmode = MD_OFFLINE;
    SCH_RemoveTimeout((*inst).tx_timeout_id);
    (*inst).tx_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    /* Mark source unreachable */
    SRC_ResetReachability((*inst).source);
    /* And inactive */
    SRC_UnsetActive((*inst).source);
    close_client_socket(inst);
    NCR_ResetInstance(inst);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_GetInstance(mut remote_addr:
                                             *mut NTP_Remote_Address,
                                         mut type_0: NTP_Source_Type,
                                         mut params: *mut SourceParameters)
 -> NCR_Instance {
    let mut result: NCR_Instance = 0 as *mut NCR_Instance_Record;
    result =
        Malloc(::std::mem::size_of::<NCR_Instance_Record>() as libc::c_ulong)
            as *mut NCR_Instance_Record;
    (*result).remote_addr = *remote_addr;
    (*result).local_addr.ip_addr.family = 0 as libc::c_int as uint16_t;
    (*result).local_addr.if_index = -(1 as libc::c_int);
    match type_0 as libc::c_uint {
        0 => {
            /* Client socket will be obtained when sending request */
            (*result).local_addr.sock_fd = -(2 as libc::c_int);
            (*result).mode = MODE_CLIENT
        }
        1 => {
            (*result).local_addr.sock_fd = NIO_OpenServerSocket(remote_addr);
            (*result).mode = MODE_ACTIVE
        }
        _ => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"ntp_core.c\x00" as *const u8 as
                              *const libc::c_char,
                          537 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 88],
                                                    &[libc::c_char; 88]>(b"NCR_Instance NCR_GetInstance(NTP_Remote_Address *, NTP_Source_Type, SourceParameters *)\x00")).as_ptr());
        }
    }
    (*result).interleaved = (*params).interleaved;
    (*result).minpoll = (*params).minpoll;
    if (*result).minpoll < -(6 as libc::c_int) {
        (*result).minpoll = 6 as libc::c_int
    } else if (*result).minpoll > 24 as libc::c_int {
        (*result).minpoll = 24 as libc::c_int
    }
    (*result).maxpoll = (*params).maxpoll;
    if (*result).maxpoll < -(6 as libc::c_int) {
        (*result).maxpoll = 10 as libc::c_int
    } else if (*result).maxpoll > 24 as libc::c_int {
        (*result).maxpoll = 24 as libc::c_int
    }
    if (*result).maxpoll < (*result).minpoll {
        (*result).maxpoll = (*result).minpoll
    }
    (*result).min_stratum = (*params).min_stratum;
    if (*result).min_stratum >= 16 as libc::c_int {
        (*result).min_stratum = 16 as libc::c_int - 1 as libc::c_int
    }
    /* Presend doesn't work in symmetric mode */
    (*result).presend_minpoll = (*params).presend_minpoll;
    if (*result).presend_minpoll <= 24 as libc::c_int &&
           (*result).mode as libc::c_uint !=
               MODE_CLIENT as libc::c_int as libc::c_uint {
        (*result).presend_minpoll = 24 as libc::c_int + 1 as libc::c_int
    }
    (*result).max_delay =
        if 0.0f64 >
               (if (*params).max_delay < 1.0e3f64 {
                    (*params).max_delay
                } else { 1.0e3f64 }) {
            0.0f64
        } else if (*params).max_delay < 1.0e3f64 {
            (*params).max_delay
        } else { 1.0e3f64 };
    (*result).max_delay_ratio =
        if 0.0f64 >
               (if (*params).max_delay_ratio < 1.0e6f64 {
                    (*params).max_delay_ratio
                } else { 1.0e6f64 }) {
            0.0f64
        } else if (*params).max_delay_ratio < 1.0e6f64 {
            (*params).max_delay_ratio
        } else { 1.0e6f64 };
    (*result).max_delay_dev_ratio =
        if 0.0f64 >
               (if (*params).max_delay_dev_ratio < 1.0e6f64 {
                    (*params).max_delay_dev_ratio
                } else { 1.0e6f64 }) {
            0.0f64
        } else if (*params).max_delay_dev_ratio < 1.0e6f64 {
            (*params).max_delay_dev_ratio
        } else { 1.0e6f64 };
    (*result).offset_correction = (*params).offset;
    (*result).auto_burst = (*params).burst;
    (*result).auto_offline = (*params).auto_offline;
    (*result).poll_target = (*params).poll_target;
    (*result).version = 4 as libc::c_int;
    if (*params).authkey == 0 as libc::c_int as libc::c_uint {
        (*result).auth_mode = AUTH_NONE;
        (*result).auth_key_id = 0 as libc::c_int as uint32_t
    } else {
        (*result).auth_mode = AUTH_SYMMETRIC;
        (*result).auth_key_id = (*params).authkey;
        if KEY_KeyKnown((*result).auth_key_id) == 0 {
            LOG_Message(LOGS_WARN,
                        b"Key %u used by source %s is %s\x00" as *const u8 as
                            *const libc::c_char, (*result).auth_key_id,
                        UTI_IPToString(&mut (*result).remote_addr.ip_addr),
                        b"missing\x00" as *const u8 as *const libc::c_char);
        } else if KEY_CheckKeyLength((*result).auth_key_id) == 0 {
            LOG_Message(LOGS_WARN,
                        b"Key %u used by source %s is %s\x00" as *const u8 as
                            *const libc::c_char, (*result).auth_key_id,
                        UTI_IPToString(&mut (*result).remote_addr.ip_addr),
                        b"too short\x00" as *const u8 as *const libc::c_char);
        }
        /* If the MAC in NTPv4 packets would be truncated, use version 3 by
       default for compatibility with older chronyd servers */
        if KEY_GetAuthLength((*result).auth_key_id) + 4 as libc::c_int >
               4 as libc::c_int + 20 as libc::c_int {
            (*result).version = 3 as libc::c_int
        }
    }
    if (*params).version != 0 {
        (*result).version =
            if 1 as libc::c_int >
                   (if (*params).version < 4 as libc::c_int {
                        (*params).version
                    } else { 4 as libc::c_int }) {
                1 as libc::c_int
            } else if (*params).version < 4 as libc::c_int {
                (*params).version
            } else { 4 as libc::c_int }
    }
    /* Create a source instance for this NTP source */
    (*result).source =
        SRC_CreateNewInstance(UTI_IPToRefid(&mut (*remote_addr).ip_addr),
                              SRC_NTP, (*params).sel_options,
                              &mut (*result).remote_addr.ip_addr,
                              (*params).min_samples, (*params).max_samples,
                              (*params).min_delay, (*params).asymmetry);
    if (*params).filter_length >= 1 as libc::c_int {
        (*result).filter =
            SPF_CreateInstance((*params).filter_length,
                               (*params).filter_length, 16.0f64, 0.0f64)
    } else { (*result).filter = 0 as SPF_Instance }
    (*result).rx_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    (*result).tx_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    (*result).tx_suspended = 1 as libc::c_int;
    (*result).opmode =
        if (*params).connectivity as libc::c_uint ==
               SRC_ONLINE as libc::c_int as libc::c_uint ||
               (*params).connectivity as libc::c_uint ==
                   SRC_MAYBE_ONLINE as libc::c_int as libc::c_uint &&
                   NIO_IsServerConnectable(remote_addr) != 0 {
            MD_ONLINE as libc::c_int
        } else { MD_OFFLINE as libc::c_int } as OperatingMode;
    (*result).local_poll = (*result).minpoll;
    (*result).poll_score = 0.0f64;
    zero_local_timestamp(&mut (*result).local_tx);
    (*result).burst_good_samples_to_go = 0 as libc::c_int;
    (*result).burst_total_samples_to_go = 0 as libc::c_int;
    memset(&mut (*result).report as *mut RPT_NTPReport as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<RPT_NTPReport>() as libc::c_ulong);
    NCR_ResetInstance(result);
    if (*params).iburst != 0 {
        NCR_InitiateSampleBurst(result, 4 as libc::c_int, 8 as libc::c_int);
    }
    return result;
}
/* ================================================== */
/* Destroy an instance */
#[no_mangle]
pub unsafe extern "C" fn NCR_DestroyInstance(mut instance: NCR_Instance) {
    if (*instance).opmode as libc::c_uint !=
           MD_OFFLINE as libc::c_int as libc::c_uint {
        take_offline(instance);
    }
    if (*instance).mode as libc::c_uint ==
           MODE_ACTIVE as libc::c_int as libc::c_uint {
        NIO_CloseServerSocket((*instance).local_addr.sock_fd);
    }
    if !(*instance).filter.is_null() {
        SPF_DestroyInstance((*instance).filter);
    }
    /* This will destroy the source instance inside the
     structure, which will cause reselection if this was the
     synchronising source etc. */
    SRC_DestroyInstance((*instance).source);
    /* Free the data structure */
    free(instance as *mut libc::c_void);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_StartInstance(mut instance: NCR_Instance) {
    (*instance).tx_suspended = 0 as libc::c_int;
    if (*instance).opmode as libc::c_uint !=
           MD_OFFLINE as libc::c_int as libc::c_uint {
        start_initial_timeout(instance);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_ResetInstance(mut instance: NCR_Instance) {
    (*instance).tx_count = 0 as libc::c_int as libc::c_uint;
    (*instance).presend_done = 0 as libc::c_int;
    (*instance).remote_poll = 0 as libc::c_int;
    (*instance).remote_stratum = 0 as libc::c_int;
    (*instance).valid_rx = 0 as libc::c_int;
    (*instance).valid_timestamps = 0 as libc::c_int;
    UTI_ZeroNtp64(&mut (*instance).remote_ntp_rx);
    UTI_ZeroNtp64(&mut (*instance).remote_ntp_tx);
    UTI_ZeroNtp64(&mut (*instance).local_ntp_rx);
    UTI_ZeroNtp64(&mut (*instance).local_ntp_tx);
    zero_local_timestamp(&mut (*instance).local_rx);
    zero_local_timestamp(&mut (*instance).prev_local_tx);
    (*instance).prev_local_poll = 0 as libc::c_int;
    (*instance).prev_tx_count = 0 as libc::c_int as libc::c_uint;
    (*instance).updated_init_timestamps = 0 as libc::c_int;
    UTI_ZeroNtp64(&mut (*instance).init_remote_ntp_tx);
    zero_local_timestamp(&mut (*instance).init_local_rx);
    if !(*instance).filter.is_null() { SPF_DropSamples((*instance).filter); };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_ResetPoll(mut instance: NCR_Instance) {
    if (*instance).local_poll != (*instance).minpoll {
        (*instance).local_poll = (*instance).minpoll;
        /* The timer was set with a longer poll interval, restart it */
        if (*instance).tx_timeout_id != 0 {
            restart_timeout(instance,
                            get_transmit_delay(instance, 0 as libc::c_int,
                                               0.0f64));
        }
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_ChangeRemoteAddress(mut inst: NCR_Instance,
                                                 mut remote_addr:
                                                     *mut NTP_Remote_Address) {
    memset(&mut (*inst).report as *mut RPT_NTPReport as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<RPT_NTPReport>() as libc::c_ulong);
    NCR_ResetInstance(inst);
    (*inst).remote_addr = *remote_addr;
    if (*inst).mode as libc::c_uint ==
           MODE_CLIENT as libc::c_int as libc::c_uint {
        close_client_socket(inst);
    } else {
        NIO_CloseServerSocket((*inst).local_addr.sock_fd);
        (*inst).local_addr.ip_addr.family = 0 as libc::c_int as uint16_t;
        (*inst).local_addr.if_index = -(1 as libc::c_int);
        (*inst).local_addr.sock_fd = NIO_OpenServerSocket(remote_addr)
    }
    /* Update the reference ID and reset the source/sourcestats instances */
    SRC_SetRefid((*inst).source, UTI_IPToRefid(&mut (*remote_addr).ip_addr),
                 &mut (*inst).remote_addr.ip_addr);
    SRC_ResetInstance((*inst).source);
}
/* ================================================== */
unsafe extern "C" fn adjust_poll(mut inst: NCR_Instance,
                                 mut adj: libc::c_double) {
    (*inst).poll_score += adj;
    if (*inst).poll_score >= 1.0f64 {
        (*inst).local_poll += (*inst).poll_score as libc::c_int;
        (*inst).poll_score -=
            (*inst).poll_score as libc::c_int as libc::c_double
    }
    if (*inst).poll_score < 0.0f64 {
        (*inst).local_poll += ((*inst).poll_score - 1.0f64) as libc::c_int;
        (*inst).poll_score -=
            ((*inst).poll_score - 1.0f64) as libc::c_int as libc::c_double
    }
    /* Clamp polling interval to defined range */
    if (*inst).local_poll < (*inst).minpoll {
        (*inst).local_poll = (*inst).minpoll;
        (*inst).poll_score = 0 as libc::c_int as libc::c_double
    } else if (*inst).local_poll > (*inst).maxpoll {
        (*inst).local_poll = (*inst).maxpoll;
        (*inst).poll_score = 1.0f64
    }
    /* Don't allow a sub-second polling interval if the source is not reachable
     or it is not in a local network according to the measured delay */
    if (*inst).local_poll < 0 as libc::c_int &&
           (SRC_IsReachable((*inst).source) == 0 ||
                SST_MinRoundTripDelay(SRC_GetSourcestats((*inst).source)) >
                    0.01f64) {
        (*inst).local_poll = 0 as libc::c_int
    };
}
/* ================================================== */
unsafe extern "C" fn get_poll_adj(mut inst: NCR_Instance,
                                  mut error_in_estimate: libc::c_double,
                                  mut peer_distance: libc::c_double)
 -> libc::c_double {
    let mut poll_adj: libc::c_double = 0.;
    let mut samples: libc::c_int = 0;
    if error_in_estimate > peer_distance {
        /* If the prediction is not even within +/- the peer distance of the peer,
       we are clearly not tracking the peer at all well, so we back off the
       sampling rate depending on just how bad the situation is */
        poll_adj = -log(error_in_estimate / peer_distance) / log(2.0f64)
    } else {
        samples = SST_Samples(SRC_GetSourcestats((*inst).source));
        /* Adjust polling interval so that the number of sourcestats samples
       remains close to the target value */
        poll_adj =
            (samples as libc::c_double / (*inst).poll_target as libc::c_double
                 - 1.0f64) / (*inst).poll_target as libc::c_double;
        /* Make interval shortening quicker */
        if samples < (*inst).poll_target { poll_adj *= 2.0f64 }
    }
    return poll_adj;
}
/* ================================================== */
unsafe extern "C" fn get_transmit_poll(mut inst: NCR_Instance)
 -> libc::c_int {
    let mut poll: libc::c_int = 0;
    poll = (*inst).local_poll;
    /* In symmetric mode, if the peer is responding, use shorter of the local
     and remote poll interval, but not shorter than the minimum */
    if (*inst).mode as libc::c_uint ==
           MODE_ACTIVE as libc::c_int as libc::c_uint &&
           poll > (*inst).remote_poll && SRC_IsReachable((*inst).source) != 0
       {
        poll =
            if (*inst).remote_poll > (*inst).minpoll {
                (*inst).remote_poll
            } else { (*inst).minpoll }
    }
    return poll;
}
/* ================================================== */
unsafe extern "C" fn get_transmit_delay(mut inst: NCR_Instance,
                                        mut on_tx: libc::c_int,
                                        mut last_tx: libc::c_double)
 -> libc::c_double {
    let mut poll_to_use: libc::c_int = 0;
    let mut stratum_diff: libc::c_int = 0;
    let mut delay_time: libc::c_double = 0.;
    /* If we're in burst mode, queue for immediate dispatch.

     If we're operating in client/server mode, queue the timeout for
     the poll interval hence.  The fact that a timeout has been queued
     in the transmit handler is immaterial - that is only done so that
     we at least send something, if no reply is heard.

     If we're in symmetric mode, we have to take account of the peer's
     wishes, otherwise his sampling regime will fall to pieces.  If
     we're in client/server mode, we don't care what poll interval the
     server responded with last time. */
    poll_to_use = get_transmit_poll(inst);
    delay_time = UTI_Log2ToDouble(poll_to_use);
    match (*inst).opmode as libc::c_uint {
        0 => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"ntp_core.c\x00" as *const u8 as
                              *const libc::c_char,
                          844 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 53],
                                                    &[libc::c_char; 53]>(b"double get_transmit_delay(NCR_Instance, int, double)\x00")).as_ptr());
        }
        1 => {
            match (*inst).mode as libc::c_uint {
                3 => { if (*inst).presend_done != 0 { delay_time = 2.0f64 } }
                1 => {
                    /* If the remote stratum is higher than ours, wait a bit for the next
             packet before responding in order to minimize the delay of the
             measurement and its error for the peer which has higher stratum.
             If the remote stratum is equal to ours, try to interleave packets
             evenly with the peer. */
                    stratum_diff =
                        (*inst).remote_stratum - REF_GetOurStratum();
                    if stratum_diff > 0 as libc::c_int &&
                           last_tx * 1.1f64 < delay_time ||
                           on_tx == 0 && stratum_diff == 0 &&
                               last_tx / delay_time > 1.1f64 - 0.5f64 {
                        delay_time *= 1.1f64
                    }
                    /* Substract the already spend time */
                    if last_tx > 0.0f64 { delay_time -= last_tx }
                    if delay_time < 0.0f64 { delay_time = 0.0f64 }
                }
                _ => {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ntp_core.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  873 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 53],
                                                            &[libc::c_char; 53]>(b"double get_transmit_delay(NCR_Instance, int, double)\x00")).as_ptr());
                }
            }
        }
        3 | 2 => {
            /* Burst modes */
            delay_time =
                if 2.0f64 < 0.25f64 * delay_time {
                    2.0f64
                } else { (0.25f64) * delay_time }
        }
        _ => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"ntp_core.c\x00" as *const u8 as
                              *const libc::c_char,
                          884 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 53],
                                                    &[libc::c_char; 53]>(b"double get_transmit_delay(NCR_Instance, int, double)\x00")).as_ptr());
        }
    }
    return delay_time;
}
/* ================================================== */
/* Calculate sampling separation for given polling interval */
unsafe extern "C" fn get_separation(mut poll: libc::c_int) -> libc::c_double {
    let mut separation: libc::c_double = 0.;
    if poll >= -(6 as libc::c_int) && poll <= 24 as libc::c_int {
    } else {
        __assert_fail(b"poll >= MIN_POLL && poll <= MAX_POLL\x00" as *const u8
                          as *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      899 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"double get_separation(int)\x00")).as_ptr());
    }
    /* Allow up to 8 sources using the same short interval to not be limited
     by the separation */
    separation = UTI_Log2ToDouble(poll - 3 as libc::c_int);
    return if 0.002f64 >
                  (if separation < 0.2f64 { separation } else { 0.2f64 }) {
               0.002f64
           } else if separation < 0.2f64 { separation } else { 0.2f64 };
}
/* ================================================== */
/* Timeout handler for closing the client socket when no acceptable
   reply can be received from the server */
unsafe extern "C" fn receive_timeout(mut arg: *mut libc::c_void) {
    let mut inst: NCR_Instance = arg as NCR_Instance;
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"Receive timeout for %s\x00" as *const u8 as
                        *const libc::c_char,
                    UTI_IPSockAddrToString(&mut (*inst).remote_addr));
    }
    (*inst).rx_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    close_client_socket(inst);
}
/* ================================================== */
unsafe extern "C" fn transmit_packet(mut my_mode: NTP_Mode,
                                     mut interleaved: libc::c_int,
                                     mut my_poll: libc::c_int,
                                     mut version: libc::c_int,
                                     mut auth_mode: libc::c_int,
                                     mut key_id: uint32_t,
                                     mut remote_ntp_rx: *mut NTP_int64,
                                     mut remote_ntp_tx: *mut NTP_int64,
                                     mut local_rx: *mut NTP_Local_Timestamp,
                                     mut local_tx: *mut NTP_Local_Timestamp,
                                     mut local_ntp_rx: *mut NTP_int64,
                                     mut local_ntp_tx: *mut NTP_int64,
                                     mut where_to: *mut NTP_Remote_Address,
                                     mut from: *mut NTP_Local_Address)
 -> libc::c_int 
 /* From what address to send it */
 {
    let mut message: NTP_Packet =
        NTP_Packet{lvm: 0,
                   stratum: 0,
                   poll: 0,
                   precision: 0,
                   root_delay: 0,
                   root_dispersion: 0,
                   reference_id: 0,
                   reference_ts: NTP_int64{hi: 0, lo: 0,},
                   originate_ts: NTP_int64{hi: 0, lo: 0,},
                   receive_ts: NTP_int64{hi: 0, lo: 0,},
                   transmit_ts: NTP_int64{hi: 0, lo: 0,},
                   auth_keyid: 0,
                   auth_data: [0; 64],};
    let mut auth_len: libc::c_int = 0;
    let mut max_auth_len: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut precision: libc::c_int = 0;
    let mut local_receive: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut local_transmit: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut smooth_offset: libc::c_double = 0.;
    let mut local_transmit_err: libc::c_double = 0.;
    let mut ts_fuzz: NTP_int64 = NTP_int64{hi: 0, lo: 0,};
    /* Parameters read from reference module */
    let mut are_we_synchronised: libc::c_int = 0;
    let mut our_stratum: libc::c_int = 0;
    let mut smooth_time: libc::c_int = 0;
    let mut leap_status: NTP_Leap = LEAP_Normal;
    let mut our_ref_id: uint32_t = 0;
    let mut our_ref_time: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut our_root_delay: libc::c_double = 0.;
    let mut our_root_dispersion: libc::c_double = 0.;
    /* Don't reply with version higher than ours */
    if version > 4 as libc::c_int { version = 4 as libc::c_int }
    /* Check if the packet can be formed in the interleaved mode */
    if interleaved != 0 &&
           (remote_ntp_rx.is_null() || local_tx.is_null() ||
                UTI_IsZeroTimespec(&mut (*local_tx).ts) != 0) {
        interleaved = 0 as libc::c_int
    }
    smooth_time = 0 as libc::c_int;
    smooth_offset = 0.0f64;
    if my_mode as libc::c_uint == MODE_CLIENT as libc::c_int as libc::c_uint {
        /* Don't reveal local time or state of the clock in client packets */
        precision = 32 as libc::c_int;
        our_ref_id = 0 as libc::c_int as uint32_t;
        our_stratum = our_ref_id as libc::c_int;
        leap_status = our_stratum as NTP_Leap;
        our_root_dispersion = 0.0f64;
        our_root_delay = our_root_dispersion;
        UTI_ZeroTimespec(&mut our_ref_time);
    } else {
        /* This is accurate enough and cheaper than calling LCL_ReadCookedTime.
       A more accurate timestamp will be taken later in this function. */
        SCH_GetLastEventTime(&mut local_transmit, 0 as *mut libc::c_double,
                             0 as *mut timespec);
        REF_GetReferenceParams(&mut local_transmit, &mut are_we_synchronised,
                               &mut leap_status, &mut our_stratum,
                               &mut our_ref_id, &mut our_ref_time,
                               &mut our_root_delay, &mut our_root_dispersion);
        /* Get current smoothing offset when sending packet to a client */
        if SMT_IsEnabled() != 0 &&
               (my_mode as libc::c_uint ==
                    MODE_SERVER as libc::c_int as libc::c_uint ||
                    my_mode as libc::c_uint ==
                        MODE_BROADCAST as libc::c_int as libc::c_uint) {
            smooth_offset = SMT_GetOffset(&mut local_transmit);
            smooth_time =
                (fabs(smooth_offset) > LCL_GetSysPrecisionAsQuantum()) as
                    libc::c_int;
            /* Suppress leap second when smoothing and slew mode are enabled */
            if REF_GetLeapMode() as libc::c_uint ==
                   REF_LeapModeSlew as libc::c_int as libc::c_uint &&
                   (leap_status as libc::c_uint ==
                        LEAP_InsertSecond as libc::c_int as libc::c_uint ||
                        leap_status as libc::c_uint ==
                            LEAP_DeleteSecond as libc::c_int as libc::c_uint)
               {
                leap_status = LEAP_Normal
            }
        }
        precision = LCL_GetSysPrecisionAsLog()
    }
    if smooth_time != 0 && UTI_IsZeroTimespec(&mut (*local_rx).ts) == 0 {
        our_ref_id = 0x7f7f01ff as libc::c_ulong as uint32_t;
        UTI_AddDoubleToTimespec(&mut our_ref_time, smooth_offset,
                                &mut our_ref_time);
        UTI_AddDoubleToTimespec(&mut (*local_rx).ts, smooth_offset,
                                &mut local_receive);
    } else { local_receive = (*local_rx).ts }
    /* Generate transmit packet */
    message.lvm =
        ((leap_status as libc::c_uint) << 6 as libc::c_int &
             0xc0 as libc::c_int as libc::c_uint |
             (version << 3 as libc::c_int & 0x38 as libc::c_int) as
                 libc::c_uint |
             my_mode as libc::c_uint & 0x7 as libc::c_int as libc::c_uint) as
            uint8_t;
    /* Stratum 16 and larger are invalid */
    if our_stratum < 16 as libc::c_int {
        message.stratum = our_stratum as uint8_t
    } else { message.stratum = 0 as libc::c_int as uint8_t }
    message.poll = my_poll as int8_t;
    message.precision = precision as int8_t;
    /* If we're sending a client mode packet and we aren't synchronized yet, 
     we might have to set up artificial values for some of these parameters */
    message.root_delay = UTI_DoubleToNtp32(our_root_delay);
    message.root_dispersion = UTI_DoubleToNtp32(our_root_dispersion);
    message.reference_id = htonl(our_ref_id);
    /* Now fill in timestamps */
    UTI_TimespecToNtp64(&mut our_ref_time, &mut message.reference_ts,
                        0 as *mut NTP_int64);
    /* Don't reveal timestamps which are not necessary for the protocol */
    if my_mode as libc::c_uint != MODE_CLIENT as libc::c_int as libc::c_uint
           || interleaved != 0 {
        /* Originate - this comes from the last packet the source sent us */
        message.originate_ts =
            if interleaved != 0 { *remote_ntp_rx } else { *remote_ntp_tx };
        loop  {
            /* Prepare random bits which will be added to the receive timestamp */
            UTI_GetNtp64Fuzz(&mut ts_fuzz, precision);
            /* Do not send a packet with a non-zero receive timestamp equal to the
         originate timestamp or previous receive timestamp */
            UTI_TimespecToNtp64(&mut local_receive, &mut message.receive_ts,
                                &mut ts_fuzz);
            if !(UTI_IsZeroNtp64(&mut message.receive_ts) == 0 &&
                     UTI_IsEqualAnyNtp64(&mut message.receive_ts,
                                         &mut message.originate_ts,
                                         local_ntp_rx, 0 as *mut NTP_int64) !=
                         0) {
                break ;
            }
        }
    } else {
        UTI_ZeroNtp64(&mut message.originate_ts);
        UTI_ZeroNtp64(&mut message.receive_ts);
    }
    loop  {
        /* Receive - this is when we received the last packet from the source.
         This timestamp will have been adjusted so that it will now look to
         the source like we have been running on our latest estimate of
         frequency all along */
        /* Prepare random bits which will be added to the transmit timestamp */
        UTI_GetNtp64Fuzz(&mut ts_fuzz, precision);
        /* Do not send a packet with a non-zero transmit timestamp which is
       equal to any of the following timestamps:
       - receive (to allow reliable detection of the interleaved mode)
       - originate (to prevent the packet from being its own valid response
                    in the symmetric mode)
       - previous transmit (to invalidate responses to the previous packet)
       (the precision must be at least -30 to prevent an infinite loop!) */
        LCL_ReadCookedTime(&mut local_transmit, &mut local_transmit_err);
        if smooth_time != 0 {
            UTI_AddDoubleToTimespec(&mut local_transmit, smooth_offset,
                                    &mut local_transmit);
        }
        length = 48 as libc::c_ulong as libc::c_int;
        if auth_mode == AUTH_SYMMETRIC as libc::c_int ||
               auth_mode == AUTH_MSSNTP as libc::c_int {
            /* Transmit - this our local time right now!  Also, we might need to
       store this for our own use later, next time we receive a message
       from the source we're sending to now. */
            /* Authenticate the packet */
            /* Pre-compensate the transmit time by approximately how long it will
         take to generate the authentication data */
            local_transmit.tv_nsec +=
                if auth_mode == AUTH_SYMMETRIC as libc::c_int {
                    KEY_GetAuthDelay(key_id)
                } else { NSD_GetAuthDelay(key_id) } as libc::c_long;
            UTI_NormaliseTimespec(&mut local_transmit);
            UTI_TimespecToNtp64(if interleaved != 0 {
                                    &mut (*local_tx).ts
                                } else { &mut local_transmit },
                                &mut message.transmit_ts, &mut ts_fuzz);
            if auth_mode == AUTH_SYMMETRIC as libc::c_int {
                /* Truncate long MACs in NTPv4 packets to allow deterministic parsing
           of extension fields (RFC 7822) */
                max_auth_len =
                    if version == 4 as libc::c_int {
                        (4 as libc::c_int + 20 as libc::c_int -
                             4 as libc::c_int) as libc::c_ulong
                    } else {
                        ::std::mem::size_of::<[uint8_t; 64]>() as
                            libc::c_ulong
                    } as libc::c_int;
                auth_len =
                    KEY_GenerateAuth(key_id,
                                     &mut message as *mut NTP_Packet as
                                         *mut libc::c_uchar,
                                     48 as libc::c_ulong as libc::c_int,
                                     &mut message.auth_data as
                                         *mut [uint8_t; 64] as
                                         *mut libc::c_uchar, max_auth_len);
                if auth_len == 0 {
                    if 0 as libc::c_int != 0 &&
                           log_min_severity as libc::c_int ==
                               LOGS_DEBUG as libc::c_int {
                        LOG_Message(LOGS_DEBUG,
                                    b"Could not generate auth data with key %u\x00"
                                        as *const u8 as *const libc::c_char,
                                    key_id);
                    }
                    return 0 as libc::c_int
                }
                message.auth_keyid = htonl(key_id);
                length =
                    (length as
                         libc::c_ulong).wrapping_add((::std::mem::size_of::<NTP_int32>()
                                                          as
                                                          libc::c_ulong).wrapping_add(auth_len
                                                                                          as
                                                                                          libc::c_ulong))
                        as libc::c_int as libc::c_int
            } else if auth_mode == AUTH_MSSNTP as libc::c_int {
                /* MS-SNTP packets are signed (asynchronously) by ntp_signd */
                return NSD_SignAndSendPacket(key_id, &mut message, where_to,
                                             from, length)
            }
        } else {
            UTI_TimespecToNtp64(if interleaved != 0 {
                                    &mut (*local_tx).ts
                                } else { &mut local_transmit },
                                &mut message.transmit_ts, &mut ts_fuzz);
        }
        if !(UTI_IsZeroNtp64(&mut message.transmit_ts) == 0 &&
                 UTI_IsEqualAnyNtp64(&mut message.transmit_ts,
                                     &mut message.receive_ts,
                                     &mut message.originate_ts, local_ntp_tx)
                     != 0) {
            break ;
        }
    }
    ret =
        NIO_SendPacket(&mut message, where_to, from, length,
                       (local_tx !=
                            0 as *mut libc::c_void as
                                *mut NTP_Local_Timestamp) as libc::c_int);
    if !local_tx.is_null() {
        (*local_tx).ts = local_transmit;
        (*local_tx).err = local_transmit_err;
        (*local_tx).source = NTP_TS_DAEMON
    }
    if !local_ntp_rx.is_null() { *local_ntp_rx = message.receive_ts }
    if !local_ntp_tx.is_null() { *local_ntp_tx = message.transmit_ts }
    return ret;
}
/* ================================================== */
/* Forward prototypes */
/* ================================================== */
/* Timeout handler for transmitting to a source. */
unsafe extern "C" fn transmit_timeout(mut arg: *mut libc::c_void) {
    let mut inst: NCR_Instance = arg as NCR_Instance;
    let mut local_addr: NTP_Local_Address =
        NTP_Local_Address{ip_addr:
                              IPAddr{addr: C2RustUnnamed{in4: 0,},
                                     family: 0,
                                     _pad: 0,},
                          if_index: 0,
                          sock_fd: 0,};
    let mut interleaved: libc::c_int = 0;
    let mut initial: libc::c_int = 0;
    let mut sent: libc::c_int = 0;
    (*inst).tx_timeout_id = 0 as libc::c_int as SCH_TimeoutID;
    match (*inst).opmode as libc::c_uint {
        3 => {
            /* With online burst switch to online before last packet */
            if (*inst).burst_total_samples_to_go <= 1 as libc::c_int {
                (*inst).opmode = MD_ONLINE
            }
        }
        2 => {
            if (*inst).burst_total_samples_to_go <= 0 as libc::c_int {
                take_offline(inst);
            }
        }
        1 => {
            /* Start a new burst if the burst option is enabled and the average
         polling interval including the burst will not fall below the
         minimum polling interval */
            if (*inst).auto_burst != 0 && (*inst).local_poll > (*inst).minpoll
               {
                NCR_InitiateSampleBurst(inst, 1 as libc::c_int,
                                        if (1 as libc::c_int) <<
                                               (*inst).local_poll -
                                                   (*inst).minpoll <
                                               4 as libc::c_int {
                                            ((1 as libc::c_int)) <<
                                                (*inst).local_poll -
                                                    (*inst).minpoll
                                        } else { 4 as libc::c_int });
            }
        }
        _ => { }
    }
    if (*inst).opmode as libc::c_uint ==
           MD_OFFLINE as libc::c_int as libc::c_uint {
        return
    }
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"Transmit timeout for %s\x00" as *const u8 as
                        *const libc::c_char,
                    UTI_IPSockAddrToString(&mut (*inst).remote_addr));
    }
    /* Open new client socket */
    if (*inst).mode as libc::c_uint ==
           MODE_CLIENT as libc::c_int as libc::c_uint {
        close_client_socket(inst);
        if (*inst).local_addr.sock_fd == -(2 as libc::c_int) {
        } else {
            __assert_fail(b"inst->local_addr.sock_fd == INVALID_SOCK_FD\x00"
                              as *const u8 as *const libc::c_char,
                          b"ntp_core.c\x00" as *const u8 as
                              *const libc::c_char,
                          1179 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 30],
                                                    &[libc::c_char; 30]>(b"void transmit_timeout(void *)\x00")).as_ptr());
        }
        (*inst).local_addr.sock_fd =
            NIO_OpenClientSocket(&mut (*inst).remote_addr)
    }
    /* Don't require the packet to be sent from the same address as before */
    local_addr.ip_addr.family = 0 as libc::c_int as uint16_t;
    local_addr.if_index = -(1 as libc::c_int);
    local_addr.sock_fd = (*inst).local_addr.sock_fd;
    /* In symmetric mode, don't send a packet in interleaved mode unless it
     is the first response to the last valid request received from the peer
     and there was just one response to the previous valid request.  This
     prevents the peer from matching the transmit timestamp with an older
     response if it can't detect missed responses.  In client mode, which has
     at most one response per request, check how many responses are missing to
     prevent the server from responding with a very old transmit timestamp. */
    interleaved =
        ((*inst).interleaved != 0 &&
             ((*inst).mode as libc::c_uint ==
                  MODE_CLIENT as libc::c_int as libc::c_uint &&
                  (*inst).tx_count < 4 as libc::c_int as libc::c_uint ||
                  (*inst).mode as libc::c_uint ==
                      MODE_ACTIVE as libc::c_int as libc::c_uint &&
                      (*inst).prev_tx_count ==
                          1 as libc::c_int as libc::c_uint &&
                      (*inst).tx_count == 0 as libc::c_int as libc::c_uint))
            as libc::c_int;
    /* In symmetric mode, if no valid response was received since the previous
     transmission, respond to the last received packet even if it failed some
     specific NTP tests.  This is necessary for starting and restarting the
     protocol, e.g. when a packet was lost. */
    initial =
        ((*inst).mode as libc::c_uint ==
             MODE_ACTIVE as libc::c_int as libc::c_uint &&
             (*inst).valid_rx == 0 &&
             UTI_IsZeroNtp64(&mut (*inst).init_remote_ntp_tx) == 0) as
            libc::c_int;
    /* Prepare for the response */
    (*inst).valid_rx = 0 as libc::c_int;
    (*inst).updated_init_timestamps = 0 as libc::c_int;
    if initial != 0 { (*inst).valid_timestamps = 0 as libc::c_int }
    /* Check whether we need to 'warm up' the link to the other end by
     sending an NTP exchange to ensure both ends' ARP caches are
     primed or whether we need to send two packets first to ensure a
     server in the interleaved mode has a fresh timestamp for us. */
    if (*inst).presend_minpoll <= (*inst).local_poll &&
           (*inst).presend_done == 0 && (*inst).burst_total_samples_to_go == 0
       {
        (*inst).presend_done =
            if interleaved != 0 { 2 as libc::c_int } else { 1 as libc::c_int }
    } else if (*inst).presend_done > 0 as libc::c_int {
        (*inst).presend_done -= 1
    }
    /* Send the request (which may also be a response in the symmetric mode) */
    sent =
        transmit_packet((*inst).mode, interleaved, (*inst).local_poll,
                        (*inst).version, (*inst).auth_mode as libc::c_int,
                        (*inst).auth_key_id,
                        if initial != 0 {
                            0 as *mut NTP_int64
                        } else { &mut (*inst).remote_ntp_rx },
                        if initial != 0 {
                            &mut (*inst).init_remote_ntp_tx
                        } else { &mut (*inst).remote_ntp_tx },
                        if initial != 0 {
                            &mut (*inst).init_local_rx
                        } else { &mut (*inst).local_rx },
                        &mut (*inst).local_tx, &mut (*inst).local_ntp_rx,
                        &mut (*inst).local_ntp_tx, &mut (*inst).remote_addr,
                        &mut local_addr);
    (*inst).tx_count = (*inst).tx_count.wrapping_add(1);
    if sent != 0 {
        (*inst).report.total_tx_count =
            (*inst).report.total_tx_count.wrapping_add(1)
    }
    /* If the source loses connectivity and our packets are still being sent,
     back off the sampling rate to reduce the network traffic.  If it's the
     source to which we are currently locked, back off slowly. */
    if (*inst).tx_count >= 2 as libc::c_int as libc::c_uint {
        /* Implies we have missed at least one transmission */
        if sent != 0 {
            adjust_poll(inst,
                        if SRC_IsSyncPeer((*inst).source) != 0 {
                            0.1f64
                        } else { 0.25f64 });
        }
        SRC_UpdateReachability((*inst).source, 0 as libc::c_int);
    }
    /* With auto_offline take the source offline if sending failed */
    if sent == 0 && (*inst).auto_offline != 0 {
        NCR_SetConnectivity(inst, SRC_OFFLINE);
    }
    let mut current_block_48: u64;
    match (*inst).opmode as libc::c_uint {
        3 => {
            /* When not reachable, don't stop online burst until sending succeeds */
            if sent == 0 && SRC_IsReachable((*inst).source) == 0 {
                current_block_48 = 13321564401369230990;
            } else { current_block_48 = 14029589282636423400; }
        }
        2 => { current_block_48 = 14029589282636423400; }
        0 => { return }
        _ => { current_block_48 = 13321564401369230990; }
    }
    match current_block_48 {
        14029589282636423400 =>
        /* Fall through */
        {
            (*inst).burst_total_samples_to_go -= 1
        }
        _ => { }
    }
    /* Restart timer for this message */
    restart_timeout(inst, get_transmit_delay(inst, 1 as libc::c_int, 0.0f64));
    /* If a client packet was just sent, schedule a timeout to close the socket
     at the time when all server replies would fail the delay test, so the
     socket is not open for longer than necessary */
    if (*inst).mode as libc::c_uint ==
           MODE_CLIENT as libc::c_int as libc::c_uint {
        (*inst).rx_timeout_id =
            SCH_AddTimeoutByDelay((*inst).max_delay + 4.0f64,
                                  Some(receive_timeout as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                                  inst as *mut libc::c_void)
    };
}
/* ================================================== */
unsafe extern "C" fn check_packet_format(mut message: *mut NTP_Packet,
                                         mut length: libc::c_int)
 -> libc::c_int {
    let mut version: libc::c_int = 0;
    /* Check version and length */
    version =
        (*message).lvm as libc::c_int >> 3 as libc::c_int &
            0x7 as libc::c_int;
    if version < 1 as libc::c_int || version > 4 as libc::c_int {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"NTP packet has invalid version %d\x00" as *const u8
                            as *const libc::c_char, version);
        }
        return 0 as libc::c_int
    }
    if length < 48 as libc::c_ulong as libc::c_int ||
           (length as
                libc::c_uint).wrapping_rem(4 as libc::c_int as libc::c_uint)
               != 0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"NTP packet has invalid length %d\x00" as *const u8
                            as *const libc::c_char, length);
        }
        return 0 as libc::c_int
    }
    /* We can't reliably check the packet for invalid extension fields as we
     support MACs longer than the shortest valid extension field */
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn is_zero_data(mut data: *mut libc::c_uchar,
                                  mut length: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < length {
        if *data.offset(i as isize) != 0 { return 0 as libc::c_int }
        i += 1
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn check_packet_auth(mut pkt: *mut NTP_Packet,
                                       mut length: libc::c_int,
                                       mut auth_mode: *mut AuthenticationMode,
                                       mut key_id: *mut uint32_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut remainder: libc::c_int = 0;
    let mut ext_length: libc::c_int = 0;
    let mut max_mac_length: libc::c_int = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut id: uint32_t = 0;
    /* Go through extension fields and see if there is a valid MAC */
    version =
        (*pkt).lvm as libc::c_int >> 3 as libc::c_int & 0x7 as libc::c_int;
    i = 48 as libc::c_ulong as libc::c_int;
    data = pkt as *mut libc::c_void as *mut libc::c_uchar;
    loop  {
        remainder = length - i;
        /* Check if the remaining data is a valid MAC.  There is a limit on MAC
       length in NTPv4 packets to allow deterministic parsing of extension
       fields (RFC 7822), but we need to support longer MACs to not break
       compatibility with older chrony clients.  This needs to be done before
       trying to parse the data as an extension field. */
        max_mac_length =
            if version == 4 as libc::c_int &&
                   remainder <= 4 as libc::c_int + 20 as libc::c_int {
                (4 as libc::c_int) + 20 as libc::c_int
            } else { (4 as libc::c_int) + 64 as libc::c_int };
        if remainder >= 4 as libc::c_int + 16 as libc::c_int &&
               remainder <= max_mac_length {
            id = ntohl(*(data.offset(i as isize) as *mut uint32_t));
            if KEY_CheckAuth(id,
                             pkt as *mut libc::c_void as *const libc::c_uchar,
                             i,
                             data.offset(i as
                                             isize).offset(4 as libc::c_int as
                                                               isize) as
                                 *mut libc::c_void as *const libc::c_uchar,
                             remainder - 4 as libc::c_int,
                             max_mac_length - 4 as libc::c_int) != 0 {
                *auth_mode = AUTH_SYMMETRIC;
                *key_id = id;
                /* If it's an NTPv4 packet with long MAC and no extension fields,
           rewrite the version in the packet to respond with long MAC too */
                if version == 4 as libc::c_int &&
                       48 as libc::c_ulong as libc::c_int + remainder ==
                           length &&
                       remainder > 4 as libc::c_int + 20 as libc::c_int {
                    (*pkt).lvm =
                        (((*pkt).lvm as libc::c_int >> 6 as libc::c_int &
                              0x3 as libc::c_int) << 6 as libc::c_int &
                             0xc0 as libc::c_int |
                             (3 as libc::c_int) << 3 as libc::c_int &
                                 0x38 as libc::c_int |
                             (*pkt).lvm as libc::c_int & 0x7 as libc::c_int &
                                 0x7 as libc::c_int) as uint8_t
                }
                return 1 as libc::c_int
            }
        }
        /* Check if this is a valid NTPv4 extension field and skip it.  It should
       have a 16-bit type, 16-bit length, and data padded to 32 bits. */
        if !(version == 4 as libc::c_int && remainder >= 16 as libc::c_int) {
            break ;
        }
        ext_length =
            ntohs(*(data.offset(i as isize).offset(2 as libc::c_int as isize)
                        as *mut uint16_t)) as libc::c_int;
        if !(ext_length >= 16 as libc::c_int && ext_length <= remainder &&
                 ext_length % 4 as libc::c_int == 0 as libc::c_int) {
            break ;
        }
        i += ext_length
    }
    /* This is not 100% reliable as a MAC could fail to authenticate and could
     pass as an extension field, leaving reminder smaller than the minimum MAC
     length */
    if remainder >= 4 as libc::c_int + 16 as libc::c_int {
        *auth_mode = AUTH_SYMMETRIC;
        *key_id = ntohl(*(data.offset(i as isize) as *mut uint32_t));
        /* Check if it is an MS-SNTP authenticator field or extended authenticator
       field with zeroes as digest */
        if version == 3 as libc::c_int && *key_id != 0 {
            if remainder == 20 as libc::c_int &&
                   is_zero_data(data.offset(i as
                                                isize).offset(4 as libc::c_int
                                                                  as isize),
                                remainder - 4 as libc::c_int) != 0 {
                *auth_mode = AUTH_MSSNTP
            } else if remainder == 72 as libc::c_int &&
                          is_zero_data(data.offset(i as
                                                       isize).offset(8 as
                                                                         libc::c_int
                                                                         as
                                                                         isize),
                                       remainder - 8 as libc::c_int) != 0 {
                *auth_mode = AUTH_MSSNTP_EXT
            }
        }
    } else { *auth_mode = AUTH_NONE; *key_id = 0 as libc::c_int as uint32_t }
    return 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn check_delay_ratio(mut inst: NCR_Instance,
                                       mut stats: SST_Stats,
                                       mut sample_time: *mut timespec,
                                       mut delay: libc::c_double)
 -> libc::c_int {
    let mut last_sample_ago: libc::c_double = 0.;
    let mut predicted_offset: libc::c_double = 0.;
    let mut min_delay: libc::c_double = 0.;
    let mut skew: libc::c_double = 0.;
    let mut std_dev: libc::c_double = 0.;
    let mut max_delay: libc::c_double = 0.;
    if (*inst).max_delay_ratio < 1.0f64 ||
           SST_GetDelayTestData(stats, sample_time, &mut last_sample_ago,
                                &mut predicted_offset, &mut min_delay,
                                &mut skew, &mut std_dev) == 0 {
        return 1 as libc::c_int
    }
    max_delay =
        min_delay * (*inst).max_delay_ratio +
            last_sample_ago * (skew + LCL_GetMaxClockError());
    if delay <= max_delay { return 1 as libc::c_int }
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"maxdelayratio: delay=%e max_delay=%e\x00" as *const u8
                        as *const libc::c_char, delay, max_delay);
    }
    return 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn check_delay_dev_ratio(mut inst: NCR_Instance,
                                           mut stats: SST_Stats,
                                           mut sample_time: *mut timespec,
                                           mut offset: libc::c_double,
                                           mut delay: libc::c_double)
 -> libc::c_int {
    let mut last_sample_ago: libc::c_double = 0.;
    let mut predicted_offset: libc::c_double = 0.;
    let mut min_delay: libc::c_double = 0.;
    let mut skew: libc::c_double = 0.;
    let mut std_dev: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut max_delta: libc::c_double = 0.;
    let mut error_in_estimate: libc::c_double = 0.;
    if SST_GetDelayTestData(stats, sample_time, &mut last_sample_ago,
                            &mut predicted_offset, &mut min_delay, &mut skew,
                            &mut std_dev) == 0 {
        return 1 as libc::c_int
    }
    /* Require that the ratio of the increase in delay from the minimum to the
     standard deviation is less than max_delay_dev_ratio.  In the allowed
     increase in delay include also dispersion. */
    max_delta =
        std_dev * (*inst).max_delay_dev_ratio +
            last_sample_ago * (skew + LCL_GetMaxClockError());
    delta = (delay - min_delay) / 2.0f64;
    if delta <= max_delta { return 1 as libc::c_int }
    error_in_estimate = offset + predicted_offset;
    /* Before we decide to drop the sample, make sure the difference between
     measured offset and predicted offset is not significantly larger than
     the increase in delay */
    if fabs(error_in_estimate) - delta > max_delta { return 1 as libc::c_int }
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"maxdelaydevratio: error=%e delay=%e delta=%e max_delta=%e\x00"
                        as *const u8 as *const libc::c_char,
                    error_in_estimate, delay, delta, max_delta);
    }
    return 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn check_sync_loop(mut inst: NCR_Instance,
                                     mut message: *mut NTP_Packet,
                                     mut local_addr: *mut NTP_Local_Address,
                                     mut local_ts: *mut timespec)
 -> libc::c_int {
    let mut our_root_delay: libc::c_double = 0.;
    let mut our_root_dispersion: libc::c_double = 0.;
    let mut are_we_synchronised: libc::c_int = 0;
    let mut our_stratum: libc::c_int = 0;
    let mut our_ref_time: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut leap_status: NTP_Leap = LEAP_Normal;
    let mut our_ref_id: uint32_t = 0;
    /* Check if a server socket is open, i.e. a client or peer can actually
     be synchronised to us */
    if NIO_IsServerSocketOpen() == 0 { return 1 as libc::c_int }
    /* Check if the source indicates that it is synchronised to our address
     (assuming it uses the same address as the one from which we send requests
     to the source) */
    if (*message).stratum as libc::c_int > 1 as libc::c_int &&
           (*message).reference_id ==
               htonl(UTI_IPToRefid(&mut (*local_addr).ip_addr)) {
        return 0 as libc::c_int
    }
    /* Compare our reference data with the source to make sure it is not us
     (e.g. due to a misconfiguration) */
    REF_GetReferenceParams(local_ts, &mut are_we_synchronised,
                           &mut leap_status, &mut our_stratum,
                           &mut our_ref_id, &mut our_ref_time,
                           &mut our_root_delay, &mut our_root_dispersion);
    if (*message).stratum as libc::c_int == our_stratum &&
           (*message).reference_id == htonl(our_ref_id) &&
           (*message).root_delay == UTI_DoubleToNtp32(our_root_delay) &&
           UTI_IsZeroNtp64(&mut (*message).reference_ts) == 0 {
        let mut ntp_ref_time: NTP_int64 = NTP_int64{hi: 0, lo: 0,};
        UTI_TimespecToNtp64(&mut our_ref_time, &mut ntp_ref_time,
                            0 as *mut NTP_int64);
        if UTI_CompareNtp64(&mut (*message).reference_ts, &mut ntp_ref_time)
               == 0 as libc::c_int {
            if 0 as libc::c_int != 0 &&
                   log_min_severity as libc::c_int ==
                       LOGS_DEBUG as libc::c_int {
                LOG_Message(LOGS_DEBUG,
                            b"Source %s is me\x00" as *const u8 as
                                *const libc::c_char,
                            UTI_IPToString(&mut (*inst).remote_addr.ip_addr));
            }
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_sample(mut inst: NCR_Instance,
                                    mut sample: *mut NTP_Sample) {
    let mut estimated_offset: libc::c_double = 0.;
    let mut error_in_estimate: libc::c_double = 0.;
    let mut filtered_sample_ago: libc::c_double = 0.;
    let mut filtered_sample: NTP_Sample =
        NTP_Sample{time: timespec{tv_sec: 0, tv_nsec: 0,},
                   offset: 0.,
                   peer_delay: 0.,
                   peer_dispersion: 0.,
                   root_delay: 0.,
                   root_dispersion: 0.,
                   stratum: 0,
                   leap: LEAP_Normal,};
    let mut filtered_samples: libc::c_int = 0;
    /* Accumulate the sample to the median filter if it is enabled.  When the
     filter produces a result, check if it is not too old, i.e. the filter did
     not miss too many samples due to missing responses or failing tests. */
    if !(*inst).filter.is_null() {
        SPF_AccumulateSample((*inst).filter, sample);
        filtered_samples = SPF_GetNumberOfSamples((*inst).filter);
        if SPF_GetFilteredSample((*inst).filter, &mut filtered_sample) == 0 {
            return
        }
        filtered_sample_ago =
            UTI_DiffTimespecsToDouble(&mut (*sample).time,
                                      &mut filtered_sample.time);
        if filtered_sample_ago >
               (8 as libc::c_int / 2 as libc::c_int * filtered_samples) as
                   libc::c_double * UTI_Log2ToDouble((*inst).local_poll) {
            if 0 as libc::c_int != 0 &&
                   log_min_severity as libc::c_int ==
                       LOGS_DEBUG as libc::c_int {
                LOG_Message(LOGS_DEBUG,
                            b"filtered sample dropped ago=%f poll=%d\x00" as
                                *const u8 as *const libc::c_char,
                            filtered_sample_ago, (*inst).local_poll);
            }
            return
        }
        sample = &mut filtered_sample
    }
    /* Get the estimated offset predicted from previous samples.  The
     convention here is that positive means local clock FAST of
     reference, i.e. backwards to the way that 'offset' is defined. */
    estimated_offset =
        SST_PredictOffset(SRC_GetSourcestats((*inst).source),
                          &mut (*sample).time);
    error_in_estimate = fabs(-(*sample).offset - estimated_offset);
    SRC_AccumulateSample((*inst).source, sample);
    SRC_SelectSource((*inst).source);
    adjust_poll(inst,
                get_poll_adj(inst, error_in_estimate,
                             (*sample).peer_dispersion +
                                 0.5f64 * (*sample).peer_delay));
}
/* ================================================== */
unsafe extern "C" fn receive_packet(mut inst: NCR_Instance,
                                    mut local_addr: *mut NTP_Local_Address,
                                    mut rx_ts: *mut NTP_Local_Timestamp,
                                    mut message: *mut NTP_Packet,
                                    mut length: libc::c_int) -> libc::c_int {
    let mut sample: NTP_Sample =
        NTP_Sample{time: timespec{tv_sec: 0, tv_nsec: 0,},
                   offset: 0.,
                   peer_delay: 0.,
                   peer_dispersion: 0.,
                   root_delay: 0.,
                   root_dispersion: 0.,
                   stratum: 0,
                   leap: LEAP_Normal,};
    let mut stats: SST_Stats = 0 as *mut SST_Stats_Record;
    let mut pkt_leap: libc::c_int = 0;
    let mut pkt_version: libc::c_int = 0;
    let mut pkt_refid: uint32_t = 0;
    let mut pkt_key_id: uint32_t = 0;
    let mut pkt_root_delay: libc::c_double = 0.;
    let mut pkt_root_dispersion: libc::c_double = 0.;
    let mut pkt_auth_mode: AuthenticationMode = AUTH_NONE;
    /* The skew and estimated frequency offset relative to the remote source */
    let mut skew: libc::c_double = 0.;
    let mut source_freq_lo: libc::c_double = 0.;
    let mut source_freq_hi: libc::c_double = 0.;
    /* RFC 5905 packet tests */
    let mut test1: libc::c_int = 0;
    let mut test2n: libc::c_int = 0;
    let mut test2i: libc::c_int = 0;
    let mut test2: libc::c_int = 0;
    let mut test3: libc::c_int = 0;
    let mut test5: libc::c_int = 0;
    let mut test6: libc::c_int = 0;
    let mut test7: libc::c_int = 0;
    let mut interleaved_packet: libc::c_int = 0;
    let mut valid_packet: libc::c_int = 0;
    let mut synced_packet: libc::c_int = 0;
    /* Additional tests */
    let mut testA: libc::c_int = 0;
    let mut testB: libc::c_int = 0;
    let mut testC: libc::c_int = 0;
    let mut testD: libc::c_int = 0;
    let mut good_packet: libc::c_int = 0;
    /* Kiss-o'-Death codes */
    let mut kod_rate: libc::c_int = 0;
    let mut local_receive: NTP_Local_Timestamp =
        NTP_Local_Timestamp{ts: timespec{tv_sec: 0, tv_nsec: 0,},
                            err: 0.,
                            source: NTP_TS_DAEMON,};
    let mut local_transmit: NTP_Local_Timestamp =
        NTP_Local_Timestamp{ts: timespec{tv_sec: 0, tv_nsec: 0,},
                            err: 0.,
                            source: NTP_TS_DAEMON,};
    let mut remote_interval: libc::c_double = 0.;
    let mut local_interval: libc::c_double = 0.;
    let mut response_time: libc::c_double = 0.;
    let mut delay_time: libc::c_double = 0.;
    let mut precision: libc::c_double = 0.;
    let mut updated_timestamps: libc::c_int = 0;
    /* ==================== */
    stats = SRC_GetSourcestats((*inst).source);
    (*inst).report.total_rx_count =
        (*inst).report.total_rx_count.wrapping_add(1);
    pkt_leap =
        (*message).lvm as libc::c_int >> 6 as libc::c_int &
            0x3 as libc::c_int;
    pkt_version =
        (*message).lvm as libc::c_int >> 3 as libc::c_int &
            0x7 as libc::c_int;
    pkt_refid = ntohl((*message).reference_id);
    pkt_root_delay = UTI_Ntp32ToDouble((*message).root_delay);
    pkt_root_dispersion = UTI_Ntp32ToDouble((*message).root_dispersion);
    /* Check if the packet is valid per RFC 5905, section 8.
     The test values are 1 when passed and 0 when failed. */
    /* Test 1 checks for duplicate packet */
    test1 =
        (UTI_CompareNtp64(&mut (*message).receive_ts,
                          &mut (*inst).remote_ntp_rx) != 0 ||
             UTI_CompareNtp64(&mut (*message).transmit_ts,
                              &mut (*inst).remote_ntp_tx) != 0) as
            libc::c_int;
    /* Test 2 checks for bogus packet in the basic and interleaved modes.  This
     ensures the source is responding to the latest packet we sent to it. */
    test2n =
        (UTI_CompareNtp64(&mut (*message).originate_ts,
                          &mut (*inst).local_ntp_tx) == 0) as libc::c_int;
    test2i =
        ((*inst).interleaved != 0 &&
             UTI_CompareNtp64(&mut (*message).originate_ts,
                              &mut (*inst).local_ntp_rx) == 0) as libc::c_int;
    test2 = (test2n != 0 || test2i != 0) as libc::c_int;
    interleaved_packet = (test2n == 0 && test2i != 0) as libc::c_int;
    /* Test 3 checks for invalid timestamps.  This can happen when the
     association if not properly 'up'. */
    test3 =
        (UTI_IsZeroNtp64(&mut (*message).originate_ts) == 0 &&
             UTI_IsZeroNtp64(&mut (*message).receive_ts) == 0 &&
             UTI_IsZeroNtp64(&mut (*message).transmit_ts) == 0) as
            libc::c_int;
    /* Test 4 would check for denied access.  It would always pass as this
     function is called only for known sources. */
    /* Test 5 checks for authentication failure.  If we expect authenticated info
     from this peer/server and the packet doesn't have it, the authentication
     is bad, or it's authenticated with a different key than expected, it's got
     to fail.  If we don't expect the packet to be authenticated, just ignore
     the test. */
    test5 =
        ((*inst).auth_mode as libc::c_uint ==
             AUTH_NONE as libc::c_int as libc::c_uint ||
             check_packet_auth(message, length, &mut pkt_auth_mode,
                               &mut pkt_key_id) != 0 &&
                 pkt_auth_mode as libc::c_uint ==
                     (*inst).auth_mode as libc::c_uint &&
                 pkt_key_id == (*inst).auth_key_id) as libc::c_int;
    /* Test 6 checks for unsynchronised server */
    test6 =
        (pkt_leap != LEAP_Unsynchronised as libc::c_int &&
             ((*message).stratum as libc::c_int) < 16 as libc::c_int &&
             (*message).stratum as libc::c_int != 0 as libc::c_int) as
            libc::c_int;
    /* Test 7 checks for bad data.  The root distance must be smaller than a
     defined maximum. */
    test7 =
        (pkt_root_delay / 2.0f64 + pkt_root_dispersion < 16.0f64) as
            libc::c_int;
    /* The packet is considered valid if the tests 1-5 passed.  The timestamps
     can be used for synchronisation if the tests 6 and 7 passed too. */
    valid_packet =
        (test1 != 0 && test2 != 0 && test3 != 0 && test5 != 0) as libc::c_int;
    synced_packet =
        (valid_packet != 0 && test6 != 0 && test7 != 0) as libc::c_int;
    /* Check for Kiss-o'-Death codes */
    kod_rate = 0 as libc::c_int;
    if test1 != 0 && test2 != 0 && test5 != 0 &&
           pkt_leap == LEAP_Unsynchronised as libc::c_int &&
           (*message).stratum as libc::c_int == 0 as libc::c_int {
        if pkt_refid as libc::c_ulong == 0x52415445 as libc::c_ulong {
            kod_rate = 1 as libc::c_int
        }
    }
    if synced_packet != 0 &&
           (interleaved_packet == 0 || (*inst).valid_timestamps != 0) {
        /* These are the timespec equivalents of the remote and local epochs */
        let mut remote_receive: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
        let mut remote_transmit: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
        let mut remote_request_receive: timespec =
            timespec{tv_sec: 0, tv_nsec: 0,};
        let mut local_average: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
        let mut remote_average: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
        let mut prev_remote_transmit: timespec =
            timespec{tv_sec: 0, tv_nsec: 0,};
        let mut prev_remote_poll_interval: libc::c_double = 0.;
        /* Select remote and local timestamps for the new sample */
        if interleaved_packet != 0 {
            /* Prefer previous local TX and remote RX timestamps if it will make
         the intervals significantly shorter in order to improve the accuracy
         of the measured delay */
            if UTI_IsZeroTimespec(&mut (*inst).prev_local_tx.ts) == 0 &&
                   0.1f64 *
                       UTI_DiffTimespecsToDouble(&mut (*inst).local_tx.ts,
                                                 &mut (*inst).local_rx.ts) >
                       UTI_DiffTimespecsToDouble(&mut (*inst).local_rx.ts,
                                                 &mut (*inst).prev_local_tx.ts)
               {
                UTI_Ntp64ToTimespec(&mut (*inst).remote_ntp_rx,
                                    &mut remote_receive);
                remote_request_receive = remote_receive;
                local_transmit = (*inst).prev_local_tx
            } else {
                UTI_Ntp64ToTimespec(&mut (*message).receive_ts,
                                    &mut remote_receive);
                UTI_Ntp64ToTimespec(&mut (*inst).remote_ntp_rx,
                                    &mut remote_request_receive);
                local_transmit = (*inst).local_tx
            }
            UTI_Ntp64ToTimespec(&mut (*message).transmit_ts,
                                &mut remote_transmit);
            UTI_Ntp64ToTimespec(&mut (*inst).remote_ntp_tx,
                                &mut prev_remote_transmit);
            local_receive = (*inst).local_rx
        } else {
            UTI_Ntp64ToTimespec(&mut (*message).receive_ts,
                                &mut remote_receive);
            UTI_Ntp64ToTimespec(&mut (*message).transmit_ts,
                                &mut remote_transmit);
            UTI_ZeroTimespec(&mut prev_remote_transmit);
            remote_request_receive = remote_receive;
            local_receive = *rx_ts;
            local_transmit = (*inst).local_tx
        }
        /* Calculate intervals between remote and local timestamps */
        UTI_AverageDiffTimespecs(&mut remote_receive, &mut remote_transmit,
                                 &mut remote_average, &mut remote_interval);
        UTI_AverageDiffTimespecs(&mut local_transmit.ts,
                                 &mut local_receive.ts, &mut local_average,
                                 &mut local_interval);
        response_time =
            fabs(UTI_DiffTimespecsToDouble(&mut remote_transmit,
                                           &mut remote_request_receive));
        precision =
            LCL_GetSysPrecisionAsQuantum() +
                UTI_Log2ToDouble((*message).precision as libc::c_int);
        /* Calculate delay */
        sample.peer_delay = fabs(local_interval - remote_interval);
        if sample.peer_delay < precision { sample.peer_delay = precision }
        /* Calculate offset.  Following the NTP definition, this is negative
       if we are fast of the remote source. */
        sample.offset =
            UTI_DiffTimespecsToDouble(&mut remote_average,
                                      &mut local_average);
        /* Apply configured correction */
        sample.offset += (*inst).offset_correction;
        /* We treat the time of the sample as being midway through the local
       measurement period.  An analysis assuming constant relative
       frequency and zero network delay shows this is the only possible
       choice to estimate the frequency difference correctly for every
       sample pair. */
        sample.time = local_average;
        SST_GetFrequencyRange(stats, &mut source_freq_lo,
                              &mut source_freq_hi);
        /* Calculate skew */
        skew = (source_freq_hi - source_freq_lo) / 2.0f64;
        /* and then calculate peer dispersion */
        sample.peer_dispersion =
            (if precision >
                    (if local_transmit.err > local_receive.err {
                         local_transmit.err
                     } else { local_receive.err }) {
                 precision
             } else {
                 (if local_transmit.err > local_receive.err {
                      local_transmit.err
                  } else { local_receive.err })
             }) + skew * fabs(local_interval);
        /* If the source is an active peer, this is the minimum assumed interval
       between previous two transmissions (if not constrained by minpoll) */
        prev_remote_poll_interval =
            UTI_Log2ToDouble(if (*inst).remote_poll < (*inst).prev_local_poll
                                {
                                 (*inst).remote_poll
                             } else { (*inst).prev_local_poll });
        /* Additional tests required to pass before accumulating the sample */
        /* Test A requires that the minimum estimate of the peer delay is not
       larger than the configured maximum, in both client modes that the server
       processing time is sane, and in interleaved symmetric mode that the
       measured delay and intervals between remote timestamps don't indicate
       a missed response */
        testA =
            (sample.peer_delay - sample.peer_dispersion <= (*inst).max_delay
                 && precision <= (*inst).max_delay &&
                 !((*inst).mode as libc::c_uint ==
                       MODE_CLIENT as libc::c_int as libc::c_uint &&
                       response_time > 4.0f64) &&
                 !((*inst).mode as libc::c_uint ==
                       MODE_ACTIVE as libc::c_int as libc::c_uint &&
                       interleaved_packet != 0 &&
                       (sample.peer_delay > 0.5f64 * prev_remote_poll_interval
                            ||
                            UTI_CompareNtp64(&mut (*message).receive_ts,
                                             &mut (*message).transmit_ts) <=
                                0 as libc::c_int ||
                            (*inst).remote_poll <= (*inst).prev_local_poll &&
                                UTI_DiffTimespecsToDouble(&mut remote_transmit,
                                                          &mut prev_remote_transmit)
                                    > 1.5f64 * prev_remote_poll_interval))) as
                libc::c_int;
        /* Test B requires in client mode that the ratio of the round trip delay
       to the minimum one currently in the stats data register is less than an
       administrator-defined value */
        testB =
            check_delay_ratio(inst, stats, &mut sample.time,
                              sample.peer_delay);
        /* Test C requires that the ratio of the increase in delay from the minimum
       one in the stats data register to the standard deviation of the offsets
       in the register is less than an administrator-defined value or the
       difference between measured offset and predicted offset is larger than
       the increase in delay */
        testC =
            check_delay_dev_ratio(inst, stats, &mut sample.time,
                                  sample.offset, sample.peer_delay);
        /* Test D requires that the source is not synchronised to us and is not us
       to prevent a synchronisation loop */
        testD = check_sync_loop(inst, message, local_addr, &mut (*rx_ts).ts)
    } else {
        response_time = 0.0f64;
        local_interval = response_time;
        remote_interval = local_interval;
        sample.peer_dispersion = 0.0f64;
        sample.peer_delay = sample.peer_dispersion;
        sample.offset = sample.peer_delay;
        sample.time = (*rx_ts).ts;
        local_receive = *rx_ts;
        local_transmit = (*inst).local_tx;
        testD = 0 as libc::c_int;
        testC = testD;
        testB = testC;
        testA = testB
    }
    /* The packet is considered good for synchronisation if
     the additional tests passed */
    good_packet =
        (testA != 0 && testB != 0 && testC != 0 && testD != 0) as libc::c_int;
    sample.root_delay = pkt_root_delay + sample.peer_delay;
    sample.root_dispersion = pkt_root_dispersion + sample.peer_dispersion;
    sample.stratum =
        if (*message).stratum as libc::c_int > (*inst).min_stratum {
            (*message).stratum as libc::c_int
        } else { (*inst).min_stratum };
    sample.leap = pkt_leap as NTP_Leap;
    /* Update the NTP timestamps.  If it's a valid packet from a synchronised
     source, the timestamps may be used later when processing a packet in the
     interleaved mode.  Protect the timestamps against replay attacks in client
     mode, and also in symmetric mode as long as the peers use the same polling
     interval and never start with clocks in future or very distant past.
     The authentication test (test5) is required to prevent DoS attacks using
     unauthenticated packets on authenticated symmetric associations. */
    if (*inst).mode as libc::c_uint ==
           MODE_CLIENT as libc::c_int as libc::c_uint && valid_packet != 0 &&
           (*inst).valid_rx == 0 ||
           (*inst).mode as libc::c_uint ==
               MODE_ACTIVE as libc::c_int as libc::c_uint && valid_packet != 0
               &&
               ((*inst).valid_rx == 0 ||
                    UTI_CompareNtp64(&mut (*inst).remote_ntp_tx,
                                     &mut (*message).transmit_ts) <
                        0 as libc::c_int) {
        (*inst).remote_ntp_rx = (*message).receive_ts;
        (*inst).remote_ntp_tx = (*message).transmit_ts;
        (*inst).local_rx = *rx_ts;
        (*inst).valid_timestamps = synced_packet;
        UTI_ZeroNtp64(&mut (*inst).init_remote_ntp_tx);
        zero_local_timestamp(&mut (*inst).init_local_rx);
        (*inst).updated_init_timestamps = 0 as libc::c_int;
        updated_timestamps = 2 as libc::c_int;
        /* Don't use the same set of timestamps for the next sample */
        if interleaved_packet != 0 {
            (*inst).prev_local_tx = (*inst).local_tx
        } else { zero_local_timestamp(&mut (*inst).prev_local_tx); }
    } else if (*inst).mode as libc::c_uint ==
                  MODE_ACTIVE as libc::c_int as libc::c_uint && test1 != 0 &&
                  UTI_IsZeroNtp64(&mut (*message).transmit_ts) == 0 &&
                  test5 != 0 &&
                  ((*inst).updated_init_timestamps == 0 ||
                       UTI_CompareNtp64(&mut (*inst).init_remote_ntp_tx,
                                        &mut (*message).transmit_ts) <
                           0 as libc::c_int) {
        (*inst).init_remote_ntp_tx = (*message).transmit_ts;
        (*inst).init_local_rx = *rx_ts;
        (*inst).updated_init_timestamps = 1 as libc::c_int;
        updated_timestamps = 1 as libc::c_int
    } else { updated_timestamps = 0 as libc::c_int }
    /* Accept at most one response per request.  The NTP specification recommends
     resetting local_ntp_tx to make the following packets fail test2 or test3,
     but that would not allow the code above to make multiple updates of the
     timestamps in symmetric mode.  Also, ignore presend responses. */
    if (*inst).valid_rx != 0 {
        test3 = 0 as libc::c_int;
        test2 = test3;
        good_packet = 0 as libc::c_int;
        synced_packet = good_packet;
        valid_packet = synced_packet
    } else if valid_packet != 0 {
        if (*inst).presend_done != 0 {
            testA = 0 as libc::c_int;
            good_packet = 0 as libc::c_int
        }
        (*inst).valid_rx = 1 as libc::c_int
    }
    if local_receive.source as libc::c_uint as libc::c_ulong >=
           ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong ||
           local_transmit.source as libc::c_uint as libc::c_ulong >=
               ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong {
        __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                      b"ntp_core.c\x00" as *const u8 as *const libc::c_char,
                      1841 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 96],
                                                &[libc::c_char; 96]>(b"int receive_packet(NCR_Instance, NTP_Local_Address *, NTP_Local_Timestamp *, NTP_Packet *, int)\x00")).as_ptr());
    }
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"NTP packet lvm=%o stratum=%d poll=%d prec=%d root_delay=%f root_disp=%f refid=%x [%s]\x00"
                        as *const u8 as *const libc::c_char,
                    (*message).lvm as libc::c_int,
                    (*message).stratum as libc::c_int,
                    (*message).poll as libc::c_int,
                    (*message).precision as libc::c_int, pkt_root_delay,
                    pkt_root_dispersion, pkt_refid,
                    if (*message).stratum as libc::c_int == 0 as libc::c_int {
                        UTI_RefidToString(pkt_refid)
                    } else { b"\x00" as *const u8 as *const libc::c_char });
    }
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"reference=%s origin=%s receive=%s transmit=%s\x00" as
                        *const u8 as *const libc::c_char,
                    UTI_Ntp64ToString(&mut (*message).reference_ts),
                    UTI_Ntp64ToString(&mut (*message).originate_ts),
                    UTI_Ntp64ToString(&mut (*message).receive_ts),
                    UTI_Ntp64ToString(&mut (*message).transmit_ts));
    }
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"offset=%.9f delay=%.9f dispersion=%f root_delay=%f root_dispersion=%f\x00"
                        as *const u8 as *const libc::c_char, sample.offset,
                    sample.peer_delay, sample.peer_dispersion,
                    sample.root_delay, sample.root_dispersion);
    }
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"remote_interval=%.9f local_interval=%.9f response_time=%.9f txs=%c rxs=%c\x00"
                        as *const u8 as *const libc::c_char, remote_interval,
                    local_interval, response_time,
                    tss_chars[local_transmit.source as usize] as libc::c_int,
                    tss_chars[local_receive.source as usize] as libc::c_int);
    }
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"test123=%d%d%d test567=%d%d%d testABCD=%d%d%d%d kod_rate=%d interleaved=%d presend=%d valid=%d good=%d updated=%d\x00"
                        as *const u8 as *const libc::c_char, test1, test2,
                    test3, test5, test6, test7, testA, testB, testC, testD,
                    kod_rate, interleaved_packet, (*inst).presend_done,
                    valid_packet, good_packet, updated_timestamps);
    }
    if valid_packet != 0 {
        (*inst).remote_poll = (*message).poll as libc::c_int;
        (*inst).remote_stratum =
            if (*message).stratum as libc::c_int != 0 as libc::c_int {
                (*message).stratum as libc::c_int
            } else { 16 as libc::c_int };
        (*inst).prev_local_poll = (*inst).local_poll;
        (*inst).prev_tx_count = (*inst).tx_count;
        (*inst).tx_count = 0 as libc::c_int as libc::c_uint;
        SRC_UpdateReachability((*inst).source, synced_packet);
        if good_packet != 0 {
            /* Adjust the polling interval, accumulate the sample, etc. */
            process_sample(inst, &mut sample);
            /* If we're in burst mode, check whether the burst is completed and
         revert to the previous mode */
            match (*inst).opmode as libc::c_uint {
                3 | 2 => {
                    (*inst).burst_good_samples_to_go -= 1;
                    if (*inst).burst_good_samples_to_go <= 0 as libc::c_int {
                        if (*inst).opmode as libc::c_uint ==
                               MD_BURST_WAS_ONLINE as libc::c_int as
                                   libc::c_uint {
                            (*inst).opmode = MD_ONLINE
                        } else { take_offline(inst); }
                    }
                }
                _ => { }
            }
        } else {
            /* Slowly increase the polling interval if we can't get good packet */
            adjust_poll(inst, 0.1f64);
        }
        /* If in client mode, no more packets are expected to be coming from the
       server and the socket can be closed */
        close_client_socket(inst);
        /* Update the local address and interface */
        (*inst).local_addr.ip_addr = (*local_addr).ip_addr;
        (*inst).local_addr.if_index = (*local_addr).if_index;
        /* And now, requeue the timer */
        if (*inst).opmode as libc::c_uint !=
               MD_OFFLINE as libc::c_int as libc::c_uint {
            delay_time =
                get_transmit_delay(inst, 0 as libc::c_int,
                                   UTI_DiffTimespecsToDouble(&mut (*inst).local_rx.ts,
                                                             &mut (*inst).local_tx.ts));
            if kod_rate != 0 {
                LOG_Message(LOGS_WARN,
                            b"Received KoD RATE from %s\x00" as *const u8 as
                                *const libc::c_char,
                            UTI_IPToString(&mut (*inst).remote_addr.ip_addr));
                /* Back off for a while and stop ongoing burst */
                delay_time +=
                    4 as libc::c_int as libc::c_double *
                        UTI_Log2ToDouble((*inst).local_poll);
                if (*inst).opmode as libc::c_uint ==
                       MD_BURST_WAS_OFFLINE as libc::c_int as libc::c_uint ||
                       (*inst).opmode as libc::c_uint ==
                           MD_BURST_WAS_ONLINE as libc::c_int as libc::c_uint
                   {
                    (*inst).burst_good_samples_to_go = 0 as libc::c_int
                }
            }
            /* Get rid of old timeout and start a new one */
            if (*inst).tx_timeout_id != 0 {
            } else {
                __assert_fail(b"inst->tx_timeout_id\x00" as *const u8 as
                                  *const libc::c_char,
                              b"ntp_core.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1926 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 96],
                                                        &[libc::c_char; 96]>(b"int receive_packet(NCR_Instance, NTP_Local_Address *, NTP_Local_Timestamp *, NTP_Packet *, int)\x00")).as_ptr());
            }
            restart_timeout(inst, delay_time);
        }
        /* Update the NTP report */
        (*inst).report.remote_addr = (*inst).remote_addr.ip_addr;
        (*inst).report.local_addr = (*inst).local_addr.ip_addr;
        (*inst).report.remote_port = (*inst).remote_addr.port;
        (*inst).report.leap = pkt_leap as uint8_t;
        (*inst).report.version = pkt_version as uint8_t;
        (*inst).report.mode =
            ((*message).lvm as libc::c_int & 0x7 as libc::c_int) as uint8_t;
        (*inst).report.stratum = (*message).stratum;
        (*inst).report.poll = (*message).poll;
        (*inst).report.precision = (*message).precision;
        (*inst).report.root_delay = pkt_root_delay;
        (*inst).report.root_dispersion = pkt_root_dispersion;
        (*inst).report.ref_id = pkt_refid;
        UTI_Ntp64ToTimespec(&mut (*message).reference_ts,
                            &mut (*inst).report.ref_time);
        (*inst).report.offset = sample.offset;
        (*inst).report.peer_delay = sample.peer_delay;
        (*inst).report.peer_dispersion = sample.peer_dispersion;
        (*inst).report.response_time = response_time;
        (*inst).report.jitter_asymmetry = SST_GetJitterAsymmetry(stats);
        (*inst).report.tests =
            (((((((((test1 << 1 as libc::c_int | test2) << 1 as libc::c_int |
                        test3) << 1 as libc::c_int | test5) <<
                      1 as libc::c_int | test6) << 1 as libc::c_int | test7)
                    << 1 as libc::c_int | testA) << 1 as libc::c_int | testB)
                  << 1 as libc::c_int | testC) << 1 as libc::c_int | testD) as
                uint16_t;
        (*inst).report.interleaved = interleaved_packet;
        (*inst).report.authenticated =
            ((*inst).auth_mode as libc::c_uint !=
                 AUTH_NONE as libc::c_int as libc::c_uint) as libc::c_int;
        (*inst).report.tx_tss_char =
            tss_chars[local_transmit.source as usize];
        (*inst).report.rx_tss_char = tss_chars[local_receive.source as usize];
        (*inst).report.total_valid_count =
            (*inst).report.total_valid_count.wrapping_add(1)
    }
    /* Do measurement logging */
    if logfileid != -(1 as libc::c_int) &&
           (log_raw_measurements != 0 || synced_packet != 0) {
        LOG_FileWrite(logfileid,
                      b"%s %-15s %1c %2d %1d%1d%1d %1d%1d%1d %1d%1d%1d%d  %2d %2d %4.2f %10.3e %10.3e %10.3e %10.3e %10.3e %08X %1d%1c %1c %1c\x00"
                          as *const u8 as *const libc::c_char,
                      UTI_TimeToLogForm(sample.time.tv_sec),
                      UTI_IPToString(&mut (*inst).remote_addr.ip_addr),
                      leap_chars[pkt_leap as usize] as libc::c_int,
                      (*message).stratum as libc::c_int, test1, test2, test3,
                      test5, test6, test7, testA, testB, testC, testD,
                      (*inst).local_poll, (*message).poll as libc::c_int,
                      (*inst).poll_score, sample.offset, sample.peer_delay,
                      sample.peer_dispersion, pkt_root_delay,
                      pkt_root_dispersion, pkt_refid,
                      (*message).lvm as libc::c_int & 0x7 as libc::c_int,
                      if interleaved_packet != 0 {
                          'I' as i32
                      } else { 'B' as i32 },
                      tss_chars[local_transmit.source as usize] as
                          libc::c_int,
                      tss_chars[local_receive.source as usize] as
                          libc::c_int);
    }
    return good_packet;
}
/* ================================================== */
/* From RFC 5905, the standard handling of received packets, depending
   on the mode of the packet and of the source, is :

   +------------------+---------------------------------------+
   |                  |              Packet Mode              |
   +------------------+-------+-------+-------+-------+-------+
   | Association Mode |   1   |   2   |   3   |   4   |   5   |
   +------------------+-------+-------+-------+-------+-------+
   | No Association 0 | NEWPS | DSCRD | FXMIT | MANY  | NEWBC |
   | Symm. Active   1 | PROC  | PROC  | DSCRD | DSCRD | DSCRD |
   | Symm. Passive  2 | PROC  | ERR   | DSCRD | DSCRD | DSCRD |
   | Client         3 | DSCRD | DSCRD | DSCRD | PROC  | DSCRD |
   | Server         4 | DSCRD | DSCRD | DSCRD | DSCRD | DSCRD |
   | Broadcast      5 | DSCRD | DSCRD | DSCRD | DSCRD | DSCRD |
   | Bcast Client   6 | DSCRD | DSCRD | DSCRD | DSCRD | PROC  |
   +------------------+-------+-------+-------+-------+-------+

   Association mode 0 is implemented in NCR_ProcessRxUnknown(), other modes
   in NCR_ProcessRxKnown().

   Broadcast, manycast and ephemeral symmetric passive associations are not
   supported yet.
 */
/* ================================================== */
/* This routine is called when a new packet arrives off the network,
   and it relates to a source we have an ongoing protocol exchange with */
#[no_mangle]
pub unsafe extern "C" fn NCR_ProcessRxKnown(mut inst: NCR_Instance,
                                            mut local_addr:
                                                *mut NTP_Local_Address,
                                            mut rx_ts:
                                                *mut NTP_Local_Timestamp,
                                            mut message: *mut NTP_Packet,
                                            mut length: libc::c_int)
 -> libc::c_int {
    let mut pkt_mode: libc::c_int = 0;
    let mut proc_packet: libc::c_int = 0;
    let mut proc_as_unknown: libc::c_int = 0;
    if check_packet_format(message, length) == 0 { return 0 as libc::c_int }
    pkt_mode = (*message).lvm as libc::c_int & 0x7 as libc::c_int;
    proc_packet = 0 as libc::c_int;
    proc_as_unknown = 0 as libc::c_int;
    /* Now, depending on the mode we decide what to do */
    match pkt_mode {
        1 => {
            match (*inst).mode as libc::c_uint {
                1 => {
                    /* Ordinary symmetric peering */
                    proc_packet = 1 as libc::c_int
                }
                2 => { }
                3 => {
                    /* This is where we have the remote configured as a server and he has
             us configured as a peer, process as from an unknown source */
                    proc_as_unknown = 1 as libc::c_int
                }
                _ => { }
            }
        }
        2 => {
            match (*inst).mode as libc::c_uint {
                1 => {
                    /* This would arise if we have the remote configured as a peer and
             he does not have us configured */
                    proc_packet = 1 as libc::c_int
                }
                2 => { }
                _ => { }
            }
        }
        3 => {
            /* If message is client mode, we just respond with a server mode
         packet, regardless of what we think the remote machine is
         supposed to be.  However, even though this is a configured
         peer or server, we still implement access restrictions on
         client mode operation.

         This copes with the case for an isolated network where one
         machine is set by eye and is used as the master, with the
         other machines pointed at it.  If the master goes down, we
         want to be able to reset its time at startup by relying on
         one of the secondaries to flywheel it. The behaviour coded here
         is required in the secondaries to make this possible. */
            proc_as_unknown = 1 as libc::c_int
        }
        4 => {
            match (*inst).mode as libc::c_uint {
                3 => {
                    /* Standard case where he's a server and we're the client */
                    proc_packet = 1 as libc::c_int
                }
                _ => { }
            }
        }
        5 => { }
        _ => { }
    }
    if proc_packet != 0 {
        /* Check if the reply was received by the socket that sent the request */
        if (*local_addr).sock_fd != (*inst).local_addr.sock_fd {
            if 0 as libc::c_int != 0 &&
                   log_min_severity as libc::c_int ==
                       LOGS_DEBUG as libc::c_int {
                LOG_Message(LOGS_DEBUG,
                            b"Packet received by wrong socket %d (expected %d)\x00"
                                as *const u8 as *const libc::c_char,
                            (*local_addr).sock_fd,
                            (*inst).local_addr.sock_fd);
            }
            return 0 as libc::c_int
        }
        /* Ignore packets from offline sources */
        if (*inst).opmode as libc::c_uint ==
               MD_OFFLINE as libc::c_int as libc::c_uint ||
               (*inst).tx_suspended != 0 {
            if 0 as libc::c_int != 0 &&
                   log_min_severity as libc::c_int ==
                       LOGS_DEBUG as libc::c_int {
                LOG_Message(LOGS_DEBUG,
                            b"Packet from offline source\x00" as *const u8 as
                                *const libc::c_char);
            }
            return 0 as libc::c_int
        }
        return receive_packet(inst, local_addr, rx_ts, message, length)
    } else if proc_as_unknown != 0 {
        NCR_ProcessRxUnknown(&mut (*inst).remote_addr, local_addr, rx_ts,
                             message, length);
        /* It's not a reply to our request, don't return success */
        return 0 as libc::c_int
    } else {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"NTP packet discarded pkt_mode=%d our_mode=%u\x00" as
                            *const u8 as *const libc::c_char, pkt_mode,
                        (*inst).mode as libc::c_uint);
        }
        return 0 as libc::c_int
    };
}
/* ================================================== */
/* This routine is called when a new packet arrives off the network,
   and it relates to a source we don't know (not our server or peer) */
#[no_mangle]
pub unsafe extern "C" fn NCR_ProcessRxUnknown(mut remote_addr:
                                                  *mut NTP_Remote_Address,
                                              mut local_addr:
                                                  *mut NTP_Local_Address,
                                              mut rx_ts:
                                                  *mut NTP_Local_Timestamp,
                                              mut message: *mut NTP_Packet,
                                              mut length: libc::c_int) {
    let mut pkt_mode: NTP_Mode = MODE_UNDEFINED;
    let mut my_mode: NTP_Mode = MODE_UNDEFINED;
    let mut local_ntp_rx: *mut NTP_int64 = 0 as *mut NTP_int64;
    let mut local_ntp_tx: *mut NTP_int64 = 0 as *mut NTP_int64;
    let mut local_tx: NTP_Local_Timestamp =
        NTP_Local_Timestamp{ts: timespec{tv_sec: 0, tv_nsec: 0,},
                            err: 0.,
                            source: NTP_TS_DAEMON,};
    let mut tx_ts: *mut NTP_Local_Timestamp = 0 as *mut NTP_Local_Timestamp;
    let mut pkt_version: libc::c_int = 0;
    let mut valid_auth: libc::c_int = 0;
    let mut log_index: libc::c_int = 0;
    let mut interleaved: libc::c_int = 0;
    let mut poll: libc::c_int = 0;
    let mut auth_mode: AuthenticationMode = AUTH_NONE;
    let mut key_id: uint32_t = 0;
    /* Ignore the packet if it wasn't received by server socket */
    if NIO_IsServerSocket((*local_addr).sock_fd) == 0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"NTP request packet received by client socket %d\x00"
                            as *const u8 as *const libc::c_char,
                        (*local_addr).sock_fd);
        }
        return
    }
    if check_packet_format(message, length) == 0 { return }
    if ADF_IsAllowed(access_auth_table, &mut (*remote_addr).ip_addr) == 0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"NTP packet received from unauthorised host %s\x00"
                            as *const u8 as *const libc::c_char,
                        UTI_IPToString(&mut (*remote_addr).ip_addr));
        }
        return
    }
    pkt_mode =
        ((*message).lvm as libc::c_int & 0x7 as libc::c_int) as NTP_Mode;
    pkt_version =
        (*message).lvm as libc::c_int >> 3 as libc::c_int &
            0x7 as libc::c_int;
    let mut current_block_25: u64;
    match pkt_mode as libc::c_uint {
        1 => {
            /* We are symmetric passive, even though we don't ever lock to him */
            my_mode = MODE_PASSIVE;
            current_block_25 = 5494826135382683477;
        }
        3 => {
            /* Reply with server packet */
            my_mode = MODE_SERVER;
            current_block_25 = 5494826135382683477;
        }
        0 => {
            /* Check if it is an NTPv1 client request (NTPv1 packets have a reserved
         field instead of the mode field and the actual mode is determined from
         the port numbers).  Don't ever respond with a mode 0 packet! */
            if pkt_version == 1 as libc::c_int &&
                   (*remote_addr).port as libc::c_int != 123 as libc::c_int {
                my_mode = MODE_SERVER;
                current_block_25 = 5494826135382683477;
            } else { current_block_25 = 12820755034573918988; }
        }
        _ => { current_block_25 = 12820755034573918988; }
    }
    match current_block_25 {
        5494826135382683477 => { }
        _ =>
        /* Fall through */
        /* Discard */
        {
            if 0 as libc::c_int != 0 &&
                   log_min_severity as libc::c_int ==
                       LOGS_DEBUG as libc::c_int {
                LOG_Message(LOGS_DEBUG,
                            b"NTP packet discarded pkt_mode=%u\x00" as
                                *const u8 as *const libc::c_char,
                            pkt_mode as libc::c_uint);
            }
            return
        }
    }
    log_index =
        CLG_LogNTPAccess(&mut (*remote_addr).ip_addr, &mut (*rx_ts).ts);
    /* Don't reply to all requests if the rate is excessive */
    if log_index >= 0 as libc::c_int &&
           CLG_LimitNTPResponseRate(log_index) != 0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"NTP packet discarded to limit response rate\x00" as
                            *const u8 as *const libc::c_char);
        }
        return
    }
    /* Check if the packet includes MAC that authenticates properly */
    valid_auth =
        check_packet_auth(message, length, &mut auth_mode, &mut key_id);
    /* If authentication failed, select whether and how we should respond */
    if valid_auth == 0 {
        match auth_mode as libc::c_uint {
            0 => { }
            2 => { }
            _ => {
                /* Discard packets in other modes */
                if 0 as libc::c_int != 0 &&
                       log_min_severity as libc::c_int ==
                           LOGS_DEBUG as libc::c_int {
                    LOG_Message(LOGS_DEBUG,
                                b"NTP packet discarded auth_mode=%u\x00" as
                                    *const u8 as *const libc::c_char,
                                auth_mode as libc::c_uint);
                }
                return
            }
        }
    }
    local_ntp_tx = 0 as *mut NTP_int64;
    local_ntp_rx = local_ntp_tx;
    tx_ts = 0 as *mut NTP_Local_Timestamp;
    interleaved = 0 as libc::c_int;
    /* Check if the client is using the interleaved mode.  If it is, save the
     new transmit timestamp and if the old transmit timestamp is valid, respond
     in the interleaved mode.  This means the third reply to a new client is
     the earliest one that can be interleaved.  We don't want to waste time
     on clients that are not using the interleaved mode. */
    if log_index >= 0 as libc::c_int {
        CLG_GetNtpTimestamps(log_index, &mut local_ntp_rx, &mut local_ntp_tx);
        interleaved =
            (UTI_IsZeroNtp64(local_ntp_rx) == 0 &&
                 UTI_CompareNtp64(&mut (*message).originate_ts, local_ntp_rx)
                     == 0 &&
                 UTI_CompareNtp64(&mut (*message).receive_ts,
                                  &mut (*message).transmit_ts) != 0) as
                libc::c_int;
        if interleaved != 0 {
            UTI_Ntp64ToTimespec(local_ntp_tx, &mut local_tx.ts);
            tx_ts = &mut local_tx
        } else {
            UTI_ZeroNtp64(local_ntp_tx);
            local_ntp_tx = 0 as *mut NTP_int64
        }
    }
    /* Suggest the client to increase its polling interval if it indicates
     the interval is shorter than the rate limiting interval */
    poll = CLG_GetNtpMinPoll();
    poll =
        if poll > (*message).poll as libc::c_int {
            poll
        } else { (*message).poll as libc::c_int };
    /* Send a reply */
    transmit_packet(my_mode, interleaved, poll, pkt_version,
                    auth_mode as libc::c_int, key_id,
                    &mut (*message).receive_ts, &mut (*message).transmit_ts,
                    rx_ts, tx_ts, local_ntp_rx, 0 as *mut NTP_int64,
                    remote_addr, local_addr);
    /* Save the transmit timestamp */
    if !tx_ts.is_null() {
        UTI_TimespecToNtp64(&mut (*tx_ts).ts, local_ntp_tx,
                            0 as *mut NTP_int64);
    };
}
/* ================================================== */
unsafe extern "C" fn update_tx_timestamp(mut tx_ts: *mut NTP_Local_Timestamp,
                                         mut new_tx_ts:
                                             *mut NTP_Local_Timestamp,
                                         mut local_ntp_rx: *mut NTP_int64,
                                         mut local_ntp_tx: *mut NTP_int64,
                                         mut message: *mut NTP_Packet) {
    let mut delay: libc::c_double = 0.;
    if UTI_IsZeroTimespec(&mut (*tx_ts).ts) != 0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Unexpected TX update\x00" as *const u8 as
                            *const libc::c_char);
        }
        return
    }
    /* Check if this is the last packet that was sent */
    if !local_ntp_rx.is_null() &&
           UTI_CompareNtp64(&mut (*message).receive_ts, local_ntp_rx) != 0 ||
           !local_ntp_tx.is_null() &&
               UTI_CompareNtp64(&mut (*message).transmit_ts, local_ntp_tx) !=
                   0 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"RX/TX timestamp mismatch\x00" as *const u8 as
                            *const libc::c_char);
        }
        return
    }
    delay = UTI_DiffTimespecsToDouble(&mut (*new_tx_ts).ts, &mut (*tx_ts).ts);
    if delay < 0.0f64 || delay > 1.0f64 {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Unacceptable TX delay %.9f\x00" as *const u8 as
                            *const libc::c_char, delay);
        }
        return
    }
    *tx_ts = *new_tx_ts;
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"Updated TX timestamp delay=%.9f\x00" as *const u8 as
                        *const libc::c_char, delay);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_ProcessTxKnown(mut inst: NCR_Instance,
                                            mut local_addr:
                                                *mut NTP_Local_Address,
                                            mut tx_ts:
                                                *mut NTP_Local_Timestamp,
                                            mut message: *mut NTP_Packet,
                                            mut length: libc::c_int) {
    let mut pkt_mode: NTP_Mode = MODE_UNDEFINED;
    if check_packet_format(message, length) == 0 { return }
    pkt_mode =
        ((*message).lvm as libc::c_int & 0x7 as libc::c_int) as NTP_Mode;
    /* Server and passive mode packets are responses to unknown sources */
    if pkt_mode as libc::c_uint != MODE_CLIENT as libc::c_int as libc::c_uint
           &&
           pkt_mode as libc::c_uint !=
               MODE_ACTIVE as libc::c_int as libc::c_uint {
        NCR_ProcessTxUnknown(&mut (*inst).remote_addr, local_addr, tx_ts,
                             message, length);
        return
    }
    update_tx_timestamp(&mut (*inst).local_tx, tx_ts,
                        &mut (*inst).local_ntp_rx, &mut (*inst).local_ntp_tx,
                        message);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_ProcessTxUnknown(mut remote_addr:
                                                  *mut NTP_Remote_Address,
                                              mut local_addr:
                                                  *mut NTP_Local_Address,
                                              mut tx_ts:
                                                  *mut NTP_Local_Timestamp,
                                              mut message: *mut NTP_Packet,
                                              mut length: libc::c_int) {
    let mut local_ntp_rx: *mut NTP_int64 = 0 as *mut NTP_int64;
    let mut local_ntp_tx: *mut NTP_int64 = 0 as *mut NTP_int64;
    let mut local_tx: NTP_Local_Timestamp =
        NTP_Local_Timestamp{ts: timespec{tv_sec: 0, tv_nsec: 0,},
                            err: 0.,
                            source: NTP_TS_DAEMON,};
    let mut log_index: libc::c_int = 0;
    if check_packet_format(message, length) == 0 { return }
    if (*message).lvm as libc::c_int & 0x7 as libc::c_int ==
           MODE_BROADCAST as libc::c_int {
        return
    }
    log_index = CLG_GetClientIndex(&mut (*remote_addr).ip_addr);
    if log_index < 0 as libc::c_int { return }
    if SMT_IsEnabled() != 0 &&
           (*message).lvm as libc::c_int & 0x7 as libc::c_int ==
               MODE_SERVER as libc::c_int {
        UTI_AddDoubleToTimespec(&mut (*tx_ts).ts,
                                SMT_GetOffset(&mut (*tx_ts).ts),
                                &mut (*tx_ts).ts);
    }
    CLG_GetNtpTimestamps(log_index, &mut local_ntp_rx, &mut local_ntp_tx);
    UTI_Ntp64ToTimespec(local_ntp_tx, &mut local_tx.ts);
    update_tx_timestamp(&mut local_tx, tx_ts, local_ntp_rx,
                        0 as *mut NTP_int64, message);
    UTI_TimespecToNtp64(&mut local_tx.ts, local_ntp_tx, 0 as *mut NTP_int64);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_SlewTimes(mut inst: NCR_Instance,
                                       mut when: *mut timespec,
                                       mut dfreq: libc::c_double,
                                       mut doffset: libc::c_double) {
    let mut delta: libc::c_double = 0.;
    if UTI_IsZeroTimespec(&mut (*inst).local_rx.ts) == 0 {
        UTI_AdjustTimespec(&mut (*inst).local_rx.ts, when,
                           &mut (*inst).local_rx.ts, &mut delta, dfreq,
                           doffset);
    }
    if UTI_IsZeroTimespec(&mut (*inst).local_tx.ts) == 0 {
        UTI_AdjustTimespec(&mut (*inst).local_tx.ts, when,
                           &mut (*inst).local_tx.ts, &mut delta, dfreq,
                           doffset);
    }
    if UTI_IsZeroTimespec(&mut (*inst).prev_local_tx.ts) == 0 {
        UTI_AdjustTimespec(&mut (*inst).prev_local_tx.ts, when,
                           &mut (*inst).prev_local_tx.ts, &mut delta, dfreq,
                           doffset);
    }
    if UTI_IsZeroTimespec(&mut (*inst).init_local_rx.ts) == 0 {
        UTI_AdjustTimespec(&mut (*inst).init_local_rx.ts, when,
                           &mut (*inst).init_local_rx.ts, &mut delta, dfreq,
                           doffset);
    }
    if !(*inst).filter.is_null() {
        SPF_SlewSamples((*inst).filter, when, dfreq, doffset);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_SetConnectivity(mut inst: NCR_Instance,
                                             mut connectivity:
                                                 SRC_Connectivity) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = UTI_IPToString(&mut (*inst).remote_addr.ip_addr);
    if connectivity as libc::c_uint ==
           SRC_MAYBE_ONLINE as libc::c_int as libc::c_uint {
        connectivity =
            if NIO_IsServerConnectable(&mut (*inst).remote_addr) != 0 {
                SRC_ONLINE as libc::c_int
            } else { SRC_OFFLINE as libc::c_int } as SRC_Connectivity
    }
    match connectivity as libc::c_uint {
        1 => {
            match (*inst).opmode as libc::c_uint {
                1 => { }
                0 => {
                    LOG_Message(LOGS_INFO,
                                b"Source %s online\x00" as *const u8 as
                                    *const libc::c_char, s);
                    (*inst).opmode = MD_ONLINE;
                    NCR_ResetInstance(inst);
                    start_initial_timeout(inst);
                }
                3 => { }
                2 => {
                    (*inst).opmode = MD_BURST_WAS_ONLINE;
                    LOG_Message(LOGS_INFO,
                                b"Source %s online\x00" as *const u8 as
                                    *const libc::c_char, s);
                }
                _ => {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ntp_core.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  2386 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 57],
                                                            &[libc::c_char; 57]>(b"void NCR_SetConnectivity(NCR_Instance, SRC_Connectivity)\x00")).as_ptr());
                }
            }
        }
        0 => {
            match (*inst).opmode as libc::c_uint {
                1 => {
                    LOG_Message(LOGS_INFO,
                                b"Source %s offline\x00" as *const u8 as
                                    *const libc::c_char, s);
                    take_offline(inst);
                }
                3 => {
                    (*inst).opmode = MD_BURST_WAS_OFFLINE;
                    LOG_Message(LOGS_INFO,
                                b"Source %s offline\x00" as *const u8 as
                                    *const libc::c_char, s);
                }
                0 | 2 => { }
                _ => {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ntp_core.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  2404 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 57],
                                                            &[libc::c_char; 57]>(b"void NCR_SetConnectivity(NCR_Instance, SRC_Connectivity)\x00")).as_ptr());
                }
            }
        }
        _ => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"ntp_core.c\x00" as *const u8 as
                              *const libc::c_char,
                          2408 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 57],
                                                    &[libc::c_char; 57]>(b"void NCR_SetConnectivity(NCR_Instance, SRC_Connectivity)\x00")).as_ptr());
        }
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_ModifyMinpoll(mut inst: NCR_Instance,
                                           mut new_minpoll: libc::c_int) {
    if new_minpoll < -(6 as libc::c_int) || new_minpoll > 24 as libc::c_int {
        return
    }
    (*inst).minpoll = new_minpoll;
    LOG_Message(LOGS_INFO,
                b"Source %s new minpoll %d\x00" as *const u8 as
                    *const libc::c_char,
                UTI_IPToString(&mut (*inst).remote_addr.ip_addr),
                new_minpoll);
    if (*inst).maxpoll < (*inst).minpoll {
        NCR_ModifyMaxpoll(inst, (*inst).minpoll);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_ModifyMaxpoll(mut inst: NCR_Instance,
                                           mut new_maxpoll: libc::c_int) {
    if new_maxpoll < -(6 as libc::c_int) || new_maxpoll > 24 as libc::c_int {
        return
    }
    (*inst).maxpoll = new_maxpoll;
    LOG_Message(LOGS_INFO,
                b"Source %s new maxpoll %d\x00" as *const u8 as
                    *const libc::c_char,
                UTI_IPToString(&mut (*inst).remote_addr.ip_addr),
                new_maxpoll);
    if (*inst).minpoll > (*inst).maxpoll {
        NCR_ModifyMinpoll(inst, (*inst).maxpoll);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_ModifyMaxdelay(mut inst: NCR_Instance,
                                            mut new_max_delay:
                                                libc::c_double) {
    (*inst).max_delay =
        if 0.0f64 >
               (if new_max_delay < 1.0e3f64 {
                    new_max_delay
                } else { 1.0e3f64 }) {
            0.0f64
        } else if new_max_delay < 1.0e3f64 {
            new_max_delay
        } else { 1.0e3f64 };
    LOG_Message(LOGS_INFO,
                b"Source %s new maxdelay %f\x00" as *const u8 as
                    *const libc::c_char,
                UTI_IPToString(&mut (*inst).remote_addr.ip_addr),
                (*inst).max_delay);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_ModifyMaxdelayratio(mut inst: NCR_Instance,
                                                 mut new_max_delay_ratio:
                                                     libc::c_double) {
    (*inst).max_delay_ratio =
        if 0.0f64 >
               (if new_max_delay_ratio < 1.0e6f64 {
                    new_max_delay_ratio
                } else { 1.0e6f64 }) {
            0.0f64
        } else if new_max_delay_ratio < 1.0e6f64 {
            new_max_delay_ratio
        } else { 1.0e6f64 };
    LOG_Message(LOGS_INFO,
                b"Source %s new maxdelayratio %f\x00" as *const u8 as
                    *const libc::c_char,
                UTI_IPToString(&mut (*inst).remote_addr.ip_addr),
                (*inst).max_delay_ratio);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_ModifyMaxdelaydevratio(mut inst: NCR_Instance,
                                                    mut new_max_delay_dev_ratio:
                                                        libc::c_double) {
    (*inst).max_delay_dev_ratio =
        if 0.0f64 >
               (if new_max_delay_dev_ratio < 1.0e6f64 {
                    new_max_delay_dev_ratio
                } else { 1.0e6f64 }) {
            0.0f64
        } else if new_max_delay_dev_ratio < 1.0e6f64 {
            new_max_delay_dev_ratio
        } else { 1.0e6f64 };
    LOG_Message(LOGS_INFO,
                b"Source %s new maxdelaydevratio %f\x00" as *const u8 as
                    *const libc::c_char,
                UTI_IPToString(&mut (*inst).remote_addr.ip_addr),
                (*inst).max_delay_dev_ratio);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_ModifyMinstratum(mut inst: NCR_Instance,
                                              mut new_min_stratum:
                                                  libc::c_int) {
    (*inst).min_stratum = new_min_stratum;
    LOG_Message(LOGS_INFO,
                b"Source %s new minstratum %d\x00" as *const u8 as
                    *const libc::c_char,
                UTI_IPToString(&mut (*inst).remote_addr.ip_addr),
                new_min_stratum);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_ModifyPolltarget(mut inst: NCR_Instance,
                                              mut new_poll_target:
                                                  libc::c_int) {
    (*inst).poll_target = new_poll_target;
    LOG_Message(LOGS_INFO,
                b"Source %s new polltarget %d\x00" as *const u8 as
                    *const libc::c_char,
                UTI_IPToString(&mut (*inst).remote_addr.ip_addr),
                new_poll_target);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_InitiateSampleBurst(mut inst: NCR_Instance,
                                                 mut n_good_samples:
                                                     libc::c_int,
                                                 mut n_total_samples:
                                                     libc::c_int) {
    if (*inst).mode as libc::c_uint ==
           MODE_CLIENT as libc::c_int as libc::c_uint {
        /* We want to prevent burst mode being used on symmetric active
       associations - it will play havoc with the peer's sampling
       strategy. (This obviously relies on us having the peer
       configured that way if he has us configured symmetric active -
       but there's not much else we can do.) */
        match (*inst).opmode as libc::c_uint {
            2 | 3 => { }
            1 | 0 => {
                (*inst).opmode =
                    if (*inst).opmode as libc::c_uint ==
                           MD_ONLINE as libc::c_int as libc::c_uint {
                        MD_BURST_WAS_ONLINE as libc::c_int
                    } else { MD_BURST_WAS_OFFLINE as libc::c_int } as
                        OperatingMode;
                (*inst).burst_good_samples_to_go = n_good_samples;
                (*inst).burst_total_samples_to_go = n_total_samples;
                start_initial_timeout(inst);
            }
            _ => {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"ntp_core.c\x00" as *const u8 as
                                  *const libc::c_char,
                              2517 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 53],
                                                        &[libc::c_char; 53]>(b"void NCR_InitiateSampleBurst(NCR_Instance, int, int)\x00")).as_ptr());
            }
        }
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_ReportSource(mut inst: NCR_Instance,
                                          mut report: *mut RPT_SourceReport,
                                          mut now: *mut timespec) {
    (*report).poll = get_transmit_poll(inst);
    match (*inst).mode as libc::c_uint {
        3 => { (*report).mode = RPT_NTP_CLIENT }
        1 => { (*report).mode = RPT_NTP_PEER }
        _ => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"ntp_core.c\x00" as *const u8 as
                              *const libc::c_char,
                          2539 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 75],
                                                    &[libc::c_char; 75]>(b"void NCR_ReportSource(NCR_Instance, RPT_SourceReport *, struct timespec *)\x00")).as_ptr());
        }
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_GetNTPReport(mut inst: NCR_Instance,
                                          mut report: *mut RPT_NTPReport) {
    *report = (*inst).report;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_AddAccessRestriction(mut ip_addr: *mut IPAddr,
                                                  mut subnet_bits:
                                                      libc::c_int,
                                                  mut allow: libc::c_int,
                                                  mut all: libc::c_int)
 -> libc::c_int {
    let mut status: ADF_Status = ADF_SUCCESS;
    if allow != 0 {
        if all != 0 {
            status = ADF_AllowAll(access_auth_table, ip_addr, subnet_bits)
        } else { status = ADF_Allow(access_auth_table, ip_addr, subnet_bits) }
    } else if all != 0 {
        status = ADF_DenyAll(access_auth_table, ip_addr, subnet_bits)
    } else { status = ADF_Deny(access_auth_table, ip_addr, subnet_bits) }
    if status as libc::c_uint != ADF_SUCCESS as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    /* Keep server sockets open only when an address allowed */
    if allow != 0 {
        let mut remote_addr: NTP_Remote_Address =
            NTP_Remote_Address{ip_addr:
                                   IPAddr{addr: C2RustUnnamed{in4: 0,},
                                          family: 0,
                                          _pad: 0,},
                               port: 0,};
        if server_sock_fd4 == -(2 as libc::c_int) &&
               ADF_IsAnyAllowed(access_auth_table, 1 as libc::c_int) != 0 {
            remote_addr.ip_addr.family = 1 as libc::c_int as uint16_t;
            server_sock_fd4 = NIO_OpenServerSocket(&mut remote_addr)
        }
        if server_sock_fd6 == -(2 as libc::c_int) &&
               ADF_IsAnyAllowed(access_auth_table, 2 as libc::c_int) != 0 {
            remote_addr.ip_addr.family = 2 as libc::c_int as uint16_t;
            server_sock_fd6 = NIO_OpenServerSocket(&mut remote_addr)
        }
    } else {
        if server_sock_fd4 != -(2 as libc::c_int) &&
               ADF_IsAnyAllowed(access_auth_table, 1 as libc::c_int) == 0 {
            NIO_CloseServerSocket(server_sock_fd4);
            server_sock_fd4 = -(2 as libc::c_int)
        }
        if server_sock_fd6 != -(2 as libc::c_int) &&
               ADF_IsAnyAllowed(access_auth_table, 2 as libc::c_int) == 0 {
            NIO_CloseServerSocket(server_sock_fd6);
            server_sock_fd6 = -(2 as libc::c_int)
        }
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_CheckAccessRestriction(mut ip_addr: *mut IPAddr)
 -> libc::c_int {
    return ADF_IsAllowed(access_auth_table, ip_addr);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_IncrementActivityCounters(mut inst: NCR_Instance,
                                                       mut online:
                                                           *mut libc::c_int,
                                                       mut offline:
                                                           *mut libc::c_int,
                                                       mut burst_online:
                                                           *mut libc::c_int,
                                                       mut burst_offline:
                                                           *mut libc::c_int) {
    match (*inst).opmode as libc::c_uint {
        2 => { *burst_offline += 1 }
        3 => { *burst_online += 1 }
        1 => { *online += 1 }
        0 => { *offline += 1 }
        _ => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"ntp_core.c\x00" as *const u8 as
                              *const libc::c_char,
                          2633 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 77],
                                                    &[libc::c_char; 77]>(b"void NCR_IncrementActivityCounters(NCR_Instance, int *, int *, int *, int *)\x00")).as_ptr());
        }
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_GetRemoteAddress(mut inst: NCR_Instance)
 -> *mut NTP_Remote_Address {
    return &mut (*inst).remote_addr;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_GetLocalRefid(mut inst: NCR_Instance)
 -> uint32_t {
    return UTI_IPToRefid(&mut (*inst).local_addr.ip_addr);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_IsSyncPeer(mut inst: NCR_Instance)
 -> libc::c_int {
    return SRC_IsSyncPeer((*inst).source);
}
/* ================================================== */
unsafe extern "C" fn broadcast_timeout(mut arg: *mut libc::c_void) {
    let mut destination: *mut BroadcastDestination =
        0 as *mut BroadcastDestination;
    let mut orig_ts: NTP_int64 = NTP_int64{hi: 0, lo: 0,};
    let mut recv_ts: NTP_Local_Timestamp =
        NTP_Local_Timestamp{ts: timespec{tv_sec: 0, tv_nsec: 0,},
                            err: 0.,
                            source: NTP_TS_DAEMON,};
    let mut poll: libc::c_int = 0;
    destination =
        ARR_GetElement(broadcasts, arg as libc::c_long as libc::c_uint) as
            *mut BroadcastDestination;
    poll =
        (log((*destination).interval as libc::c_double) / log(2.0f64) +
             0.5f64) as libc::c_int;
    UTI_ZeroNtp64(&mut orig_ts);
    zero_local_timestamp(&mut recv_ts);
    transmit_packet(MODE_BROADCAST, 0 as libc::c_int, poll, 4 as libc::c_int,
                    0 as libc::c_int, 0 as libc::c_int as uint32_t,
                    &mut orig_ts, &mut orig_ts, &mut recv_ts,
                    0 as *mut NTP_Local_Timestamp, 0 as *mut NTP_int64,
                    0 as *mut NTP_int64, &mut (*destination).addr,
                    &mut (*destination).local_addr);
    /* Requeue timeout.  We don't care if interval drifts gradually. */
    SCH_AddTimeoutInClass((*destination).interval as libc::c_double,
                          get_separation(poll), 0.02f64,
                          SCH_NtpBroadcastClass,
                          Some(broadcast_timeout as
                                   unsafe extern "C" fn(_: *mut libc::c_void)
                                       -> ()), arg);
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
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NCR_AddBroadcastDestination(mut addr: *mut IPAddr,
                                                     mut port: libc::c_ushort,
                                                     mut interval:
                                                         libc::c_int) {
    let mut destination: *mut BroadcastDestination =
        0 as *mut BroadcastDestination;
    destination = ARR_GetNewElement(broadcasts) as *mut BroadcastDestination;
    (*destination).addr.ip_addr = *addr;
    (*destination).addr.port = port;
    (*destination).local_addr.ip_addr.family = 0 as libc::c_int as uint16_t;
    (*destination).local_addr.if_index = -(1 as libc::c_int);
    (*destination).local_addr.sock_fd =
        NIO_OpenServerSocket(&mut (*destination).addr);
    (*destination).interval =
        if 1 as libc::c_int >
               (if interval < (1 as libc::c_int) << 24 as libc::c_int {
                    interval
                } else { ((1 as libc::c_int)) << 24 as libc::c_int }) {
            1 as libc::c_int
        } else if interval < (1 as libc::c_int) << 24 as libc::c_int {
            interval
        } else { ((1 as libc::c_int)) << 24 as libc::c_int };
    SCH_AddTimeoutInClass((*destination).interval as libc::c_double, 0.2f64,
                          0.02f64, SCH_NtpBroadcastClass,
                          Some(broadcast_timeout as
                                   unsafe extern "C" fn(_: *mut libc::c_void)
                                       -> ()),
                          ARR_GetSize(broadcasts).wrapping_sub(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                              as libc::c_long as *mut libc::c_void);
}
