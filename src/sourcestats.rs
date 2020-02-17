#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn round(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
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
    /* Return the weighting to apply to the standard deviation to get a
   given size of confidence interval assuming a T distribution */
    #[no_mangle]
    fn RGR_GetTCoef(dof: libc::c_int) -> libc::c_double;
    /* Minimum number of samples for regression */
    /* Return a status indicating whether there were enough points to
   carry out the regression */
    #[no_mangle]
    fn RGR_FindBestRegression(x: *mut libc::c_double, y: *mut libc::c_double,
                              w: *mut libc::c_double, n: libc::c_int,
                              m: libc::c_int, min_samples: libc::c_int,
                              b0: *mut libc::c_double,
                              b1: *mut libc::c_double,
                              s2: *mut libc::c_double,
                              sb0: *mut libc::c_double,
                              sb1: *mut libc::c_double,
                              new_start: *mut libc::c_int,
                              n_runs: *mut libc::c_int, dof: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn RGR_MultipleRegress(x1: *mut libc::c_double, x2: *mut libc::c_double,
                           y: *mut libc::c_double, n: libc::c_int,
                           b2: *mut libc::c_double) -> libc::c_int;
    /* Return the median value from an array */
    #[no_mangle]
    fn RGR_FindMedian(x: *mut libc::c_double, n: libc::c_int)
     -> libc::c_double;
    #[no_mangle]
    fn UTI_ZeroTimespec(ts: *mut timespec);
    #[no_mangle]
    fn UTI_CompareTimespecs(a: *mut timespec, b: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec)
     -> libc::c_double;
    #[no_mangle]
    fn UTI_NormaliseTimespec(ts: *mut timespec);
    #[no_mangle]
    fn UTI_TimespecToString(ts: *mut timespec) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_AdjustTimespec(old_ts: *mut timespec, when: *mut timespec,
                          new_ts: *mut timespec,
                          delta_time: *mut libc::c_double,
                          dfreq: libc::c_double, doffset: libc::c_double);
    #[no_mangle]
    fn UTI_TimeToLogForm(t: time_t) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_IPToString(ip: *mut IPAddr) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_RefidToString(ref_id: uint32_t) -> *mut libc::c_char;
    #[no_mangle]
    fn CNF_GetLogStatistics() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetMaxJitter() -> libc::c_double;
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
    /* Routine to read the system precision in terms of the actual time step */
    #[no_mangle]
    fn LCL_GetSysPrecisionAsQuantum() -> libc::c_double;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type time_t = __time_t;
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
pub type NTP_Leap = libc::c_uint;
pub const LEAP_Unsynchronised: NTP_Leap = 3;
pub const LEAP_DeleteSecond: NTP_Leap = 2;
pub const LEAP_InsertSecond: NTP_Leap = 1;
pub const LEAP_Normal: NTP_Leap = 0;
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
/* ================================================== */
/* This data structure is used to hold the history of data from the
   source */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SST_Stats_Record {
    pub refid: uint32_t,
    pub ip_addr: *mut IPAddr,
    pub min_samples: libc::c_int,
    pub max_samples: libc::c_int,
    pub fixed_min_delay: libc::c_double,
    pub fixed_asymmetry: libc::c_double,
    pub n_samples: libc::c_int,
    pub runs_samples: libc::c_int,
    pub last_sample: libc::c_int,
    pub regression_ok: libc::c_int,
    pub best_single_sample: libc::c_int,
    pub min_delay_sample: libc::c_int,
    pub estimated_offset: libc::c_double,
    pub estimated_offset_sd: libc::c_double,
    pub offset_time: timespec,
    pub nruns: libc::c_int,
    pub asymmetry_run: libc::c_int,
    pub asymmetry: libc::c_double,
    pub estimated_frequency: libc::c_double,
    pub estimated_frequency_sd: libc::c_double,
    pub skew: libc::c_double,
    pub std_dev: libc::c_double,
    pub sample_times: [timespec; 128],
    pub offsets: [libc::c_double; 128],
    pub orig_offsets: [libc::c_double; 64],
    pub peer_delays: [libc::c_double; 128],
    pub peer_dispersions: [libc::c_double; 64],
    pub root_delays: [libc::c_double; 64],
    pub root_dispersions: [libc::c_double; 64],
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

  Header file for module that deals with the measurements and statistics of
  each of the sources. */
pub type SST_Stats = *mut SST_Stats_Record;
/* File logging functions */
pub type LOG_FileID = libc::c_int;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
/* ================================================== */
static mut logfileid: LOG_FileID = 0;
/* Init and fini functions */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_Initialise() {
    logfileid =
        if CNF_GetLogStatistics() != 0 {
            LOG_FileOpen(b"statistics\x00" as *const u8 as
                             *const libc::c_char,
                         b"   Date (UTC) Time     IP Address    Std dev\'n Est offset  Offset sd  Diff freq   Est skew  Stress  Ns  Bs  Nr  Asym\x00"
                             as *const u8 as *const libc::c_char)
        } else { -(1 as libc::c_int) };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_Finalise() { }
/* ================================================== */
/* This function creates a new instance of the statistics handler */
#[no_mangle]
pub unsafe extern "C" fn SST_CreateInstance(mut refid: uint32_t,
                                            mut addr: *mut IPAddr,
                                            mut min_samples: libc::c_int,
                                            mut max_samples: libc::c_int,
                                            mut min_delay: libc::c_double,
                                            mut asymmetry: libc::c_double)
 -> SST_Stats {
    let mut inst: SST_Stats = 0 as *mut SST_Stats_Record;
    inst =
        Malloc(::std::mem::size_of::<SST_Stats_Record>() as libc::c_ulong) as
            *mut SST_Stats_Record;
    (*inst).min_samples = min_samples;
    (*inst).max_samples = max_samples;
    (*inst).fixed_min_delay = min_delay;
    (*inst).fixed_asymmetry = asymmetry;
    SST_SetRefid(inst, refid, addr);
    SST_ResetInstance(inst);
    return inst;
}
/* ================================================== */
/* This function deletes an instance of the statistics handler. */
#[no_mangle]
pub unsafe extern "C" fn SST_DeleteInstance(mut inst: SST_Stats) {
    free(inst as *mut libc::c_void);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_ResetInstance(mut inst: SST_Stats) {
    (*inst).n_samples = 0 as libc::c_int;
    (*inst).runs_samples = 0 as libc::c_int;
    (*inst).last_sample = 0 as libc::c_int;
    (*inst).regression_ok = 0 as libc::c_int;
    (*inst).best_single_sample = 0 as libc::c_int;
    (*inst).min_delay_sample = 0 as libc::c_int;
    (*inst).estimated_frequency = 0 as libc::c_int as libc::c_double;
    (*inst).estimated_frequency_sd = 2000.0f64 / 1.0e6f64;
    (*inst).skew = 2000.0f64 / 1.0e6f64;
    (*inst).estimated_offset = 0.0f64;
    (*inst).estimated_offset_sd = 4.0f64;
    UTI_ZeroTimespec(&mut (*inst).offset_time);
    (*inst).std_dev = 4.0f64;
    (*inst).nruns = 0 as libc::c_int;
    (*inst).asymmetry_run = 0 as libc::c_int;
    (*inst).asymmetry = 0.0f64;
    (*inst).leap = LEAP_Unsynchronised;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_SetRefid(mut inst: SST_Stats,
                                      mut refid: uint32_t,
                                      mut addr: *mut IPAddr) {
    (*inst).refid = refid;
    (*inst).ip_addr = addr;
}
/* ================================================== */
/* This function is called to prune the register down when it is full.
   For now, just discard the oldest sample.  */
unsafe extern "C" fn prune_register(mut inst: SST_Stats,
                                    mut new_oldest: libc::c_int) {
    if new_oldest == 0 { return }
    if (*inst).n_samples >= new_oldest {
    } else {
        __assert_fail(b"inst->n_samples >= new_oldest\x00" as *const u8 as
                          *const libc::c_char,
                      b"sourcestats.c\x00" as *const u8 as
                          *const libc::c_char,
                      283 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"void prune_register(SST_Stats, int)\x00")).as_ptr());
    }
    (*inst).n_samples -= new_oldest;
    (*inst).runs_samples += new_oldest;
    if (*inst).runs_samples >
           (*inst).n_samples * (2 as libc::c_int - 1 as libc::c_int) {
        (*inst).runs_samples =
            (*inst).n_samples * (2 as libc::c_int - 1 as libc::c_int)
    }
    if (*inst).n_samples + (*inst).runs_samples <=
           64 as libc::c_int * 2 as libc::c_int {
    } else {
        __assert_fail(b"inst->n_samples + inst->runs_samples <= MAX_SAMPLES * REGRESS_RUNS_RATIO\x00"
                          as *const u8 as *const libc::c_char,
                      b"sourcestats.c\x00" as *const u8 as
                          *const libc::c_char,
                      289 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"void prune_register(SST_Stats, int)\x00")).as_ptr());
    }
    find_min_delay_sample(inst);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_AccumulateSample(mut inst: SST_Stats,
                                              mut sample: *mut NTP_Sample) {
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    /* Make room for the new sample */
    if (*inst).n_samples > 0 as libc::c_int &&
           ((*inst).n_samples == 64 as libc::c_int ||
                (*inst).n_samples == (*inst).max_samples) {
        prune_register(inst, 1 as libc::c_int);
    }
    /* Make sure it's newer than the last sample */
    if (*inst).n_samples != 0 &&
           UTI_CompareTimespecs(&mut *(*inst).sample_times.as_mut_ptr().offset((*inst).last_sample
                                                                                   as
                                                                                   isize),
                                &mut (*sample).time) >= 0 as libc::c_int {
        LOG_Message(LOGS_WARN,
                    b"Out of order sample detected, discarding history for %s\x00"
                        as *const u8 as *const libc::c_char,
                    if !(*inst).ip_addr.is_null() {
                        UTI_IPToString((*inst).ip_addr)
                    } else { UTI_RefidToString((*inst).refid) });
        SST_ResetInstance(inst);
    }
    (*inst).last_sample =
        ((*inst).last_sample + 1 as libc::c_int) %
            (64 as libc::c_int * 2 as libc::c_int);
    n = (*inst).last_sample;
    m = n % 64 as libc::c_int;
    /* WE HAVE TO NEGATE OFFSET IN THIS CALL, IT IS HERE THAT THE SENSE OF OFFSET
     IS FLIPPED */
    (*inst).sample_times[n as usize] = (*sample).time;
    (*inst).offsets[n as usize] = -(*sample).offset;
    (*inst).orig_offsets[m as usize] = -(*sample).offset;
    (*inst).peer_delays[n as usize] = (*sample).peer_delay;
    (*inst).peer_dispersions[m as usize] = (*sample).peer_dispersion;
    (*inst).root_delays[m as usize] = (*sample).root_delay;
    (*inst).root_dispersions[m as usize] = (*sample).root_dispersion;
    (*inst).stratum = (*sample).stratum;
    (*inst).leap = (*sample).leap;
    if (*inst).peer_delays[n as usize] < (*inst).fixed_min_delay {
        (*inst).peer_delays[n as usize] =
            2.0f64 * (*inst).fixed_min_delay - (*inst).peer_delays[n as usize]
    }
    if (*inst).n_samples == 0 ||
           (*inst).peer_delays[n as usize] <
               (*inst).peer_delays[(*inst).min_delay_sample as usize] {
        (*inst).min_delay_sample = n
    }
    (*inst).n_samples += 1;
}
/* ================================================== */
/* Return index of the i-th sample in the sample_times and offset buffers,
   i can be negative down to -runs_samples */
unsafe extern "C" fn get_runsbuf_index(mut inst: SST_Stats,
                                       mut i: libc::c_int) -> libc::c_int {
    return (((*inst).last_sample +
                 2 as libc::c_int * 64 as libc::c_int * 2 as libc::c_int -
                 (*inst).n_samples + i + 1 as libc::c_int) as
                libc::c_uint).wrapping_rem((64 as libc::c_int *
                                                2 as libc::c_int) as
                                               libc::c_uint) as libc::c_int;
}
/* ================================================== */
/* Return index of the i-th sample in the other buffers */
unsafe extern "C" fn get_buf_index(mut inst: SST_Stats, mut i: libc::c_int)
 -> libc::c_int {
    return (((*inst).last_sample + 64 as libc::c_int * 2 as libc::c_int -
                 (*inst).n_samples + i + 1 as libc::c_int) as
                libc::c_uint).wrapping_rem(64 as libc::c_int as libc::c_uint)
               as libc::c_int;
}
/* ================================================== */
/* This function is used by both the regression routines to find the
   time interval between each historical sample and the most recent
   one */
unsafe extern "C" fn convert_to_intervals(mut inst: SST_Stats,
                                          mut times_back:
                                              *mut libc::c_double) {
    let mut ts: *mut timespec = 0 as *mut timespec;
    let mut i: libc::c_int = 0;
    ts =
        &mut *(*inst).sample_times.as_mut_ptr().offset((*inst).last_sample as
                                                           isize) as
            *mut timespec;
    i = -(*inst).runs_samples;
    while i < (*inst).n_samples {
        /* The entries in times_back[] should end up negative */
        *times_back.offset(i as isize) =
            UTI_DiffTimespecsToDouble(&mut *(*inst).sample_times.as_mut_ptr().offset((get_runsbuf_index
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   SST_Stats,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)(inst,
                                                                                                               i)
                                                                                         as
                                                                                         isize),
                                      ts);
        i += 1
    };
}
/* ================================================== */
unsafe extern "C" fn find_best_sample_index(mut inst: SST_Stats,
                                            mut times_back:
                                                *mut libc::c_double) {
    /* With the value of skew that has been computed, see which of the
     samples offers the tightest bound on root distance */
    let mut root_distance: libc::c_double = 0.;
    let mut best_root_distance: libc::c_double = 0.;
    let mut elapsed: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut best_index: libc::c_int = 0;
    if (*inst).n_samples == 0 { return }
    best_index = -(1 as libc::c_int);
    best_root_distance = 1.7976931348623157e+308f64;
    i = 0 as libc::c_int;
    while i < (*inst).n_samples {
        j = get_buf_index(inst, i);
        elapsed = -*times_back.offset(i as isize);
        if elapsed >= 0.0f64 {
        } else {
            __assert_fail(b"elapsed >= 0.0\x00" as *const u8 as
                              *const libc::c_char,
                          b"sourcestats.c\x00" as *const u8 as
                              *const libc::c_char,
                          401 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 49],
                                                    &[libc::c_char; 49]>(b"void find_best_sample_index(SST_Stats, double *)\x00")).as_ptr());
        }
        root_distance =
            (*inst).root_dispersions[j as usize] + elapsed * (*inst).skew +
                0.5f64 * (*inst).root_delays[j as usize];
        if root_distance < best_root_distance {
            best_root_distance = root_distance;
            best_index = i
        }
        i += 1
    }
    if best_index >= 0 as libc::c_int {
    } else {
        __assert_fail(b"best_index >= 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"sourcestats.c\x00" as *const u8 as
                          *const libc::c_char,
                      410 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"void find_best_sample_index(SST_Stats, double *)\x00")).as_ptr());
    }
    (*inst).best_single_sample = best_index;
}
/* ================================================== */
/* ================================================== */
unsafe extern "C" fn find_min_delay_sample(mut inst: SST_Stats) {
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    (*inst).min_delay_sample = get_runsbuf_index(inst, -(*inst).runs_samples);
    i = -(*inst).runs_samples + 1 as libc::c_int;
    while i < (*inst).n_samples {
        index = get_runsbuf_index(inst, i);
        if (*inst).peer_delays[index as usize] <
               (*inst).peer_delays[(*inst).min_delay_sample as usize] {
            (*inst).min_delay_sample = index
        }
        i += 1
    };
}
/* ================================================== */
/* This function estimates asymmetry of network jitter on the path to the
   source as a slope of offset against network delay in multiple linear
   regression.  If the asymmetry is significant and its sign doesn't change
   frequently, the measured offsets (which are used later to estimate the
   offset and frequency of the clock) are corrected to correspond to the
   minimum network delay.  This can significantly improve the accuracy and
   stability of the estimated offset and frequency. */
unsafe extern "C" fn estimate_asymmetry(mut times_back: *mut libc::c_double,
                                        mut offsets: *mut libc::c_double,
                                        mut delays: *mut libc::c_double,
                                        mut n: libc::c_int,
                                        mut asymmetry: *mut libc::c_double,
                                        mut asymmetry_run: *mut libc::c_int)
 -> libc::c_int {
    let mut a: libc::c_double = 0.;
    /* Reset the counter when the regression fails or the sign changes */
    if RGR_MultipleRegress(times_back, delays, offsets, n, &mut a) == 0 ||
           (a * *asymmetry_run as libc::c_double) < 0.0f64 {
        *asymmetry = 0 as libc::c_int as libc::c_double;
        *asymmetry_run = 0.0f64 as libc::c_int;
        return 0 as libc::c_int
    }
    if a <= -0.45f64 && *asymmetry_run > -(1000 as libc::c_int) {
        *asymmetry_run -= 1
    } else if a >= 0.45f64 && *asymmetry_run < 1000 as libc::c_int {
        *asymmetry_run += 1
    }
    if abs(*asymmetry_run) < 10 as libc::c_int { return 0 as libc::c_int }
    *asymmetry =
        if -0.5f64 > (if a < 0.5f64 { a } else { 0.5f64 }) {
            -0.5f64
        } else if a < 0.5f64 { a } else { 0.5f64 };
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn correct_asymmetry(mut inst: SST_Stats,
                                       mut times_back: *mut libc::c_double,
                                       mut offsets: *mut libc::c_double) {
    let mut min_delay: libc::c_double = 0.;
    let mut delays: [libc::c_double; 128] = [0.; 128];
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    /* Check if the asymmetry was not specified to be zero */
    if (*inst).fixed_asymmetry == 0.0f64 { return }
    min_delay = SST_MinRoundTripDelay(inst);
    n = (*inst).runs_samples + (*inst).n_samples;
    i = 0 as libc::c_int;
    while i < n {
        delays[i as usize] =
            (*inst).peer_delays[get_runsbuf_index(inst,
                                                  i - (*inst).runs_samples) as
                                    usize] - min_delay;
        i += 1
    }
    if fabs((*inst).fixed_asymmetry) <= 0.5f64 {
        (*inst).asymmetry = (*inst).fixed_asymmetry
    } else if estimate_asymmetry(times_back, offsets, delays.as_mut_ptr(), n,
                                 &mut (*inst).asymmetry,
                                 &mut (*inst).asymmetry_run) == 0 {
        return
    }
    /* Correct the offsets */
    i = 0 as libc::c_int;
    while i < n {
        *offsets.offset(i as isize) -= (*inst).asymmetry * delays[i as usize];
        i += 1
    };
}
/* ================================================== */
/* This function runs the linear regression operation on the data.  It
   finds the set of most recent samples that give the tightest
   confidence interval for the frequency, and truncates the register
   down to that number of samples */
#[no_mangle]
pub unsafe extern "C" fn SST_DoNewRegression(mut inst: SST_Stats) {
    let mut times_back: [libc::c_double; 128] = [0.; 128];
    let mut offsets: [libc::c_double; 128] = [0.; 128];
    let mut peer_distances: [libc::c_double; 64] = [0.; 64];
    let mut weights: [libc::c_double; 64] = [0.; 64];
    let mut degrees_of_freedom: libc::c_int = 0;
    let mut best_start: libc::c_int = 0;
    let mut times_back_start: libc::c_int = 0;
    let mut est_intercept: libc::c_double = 0.;
    let mut est_slope: libc::c_double = 0.;
    let mut est_var: libc::c_double = 0.;
    let mut est_intercept_sd: libc::c_double = 0.;
    let mut est_slope_sd: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nruns: libc::c_int = 0;
    let mut min_distance: libc::c_double = 0.;
    let mut median_distance: libc::c_double = 0.;
    let mut sd_weight: libc::c_double = 0.;
    let mut sd: libc::c_double = 0.;
    let mut old_skew: libc::c_double = 0.;
    let mut old_freq: libc::c_double = 0.;
    let mut stress: libc::c_double = 0.;
    let mut precision: libc::c_double = 0.;
    convert_to_intervals(inst,
                         times_back.as_mut_ptr().offset((*inst).runs_samples
                                                            as isize));
    if (*inst).n_samples > 0 as libc::c_int {
        i = -(*inst).runs_samples;
        while i < (*inst).n_samples {
            offsets[(i + (*inst).runs_samples) as usize] =
                (*inst).offsets[get_runsbuf_index(inst, i) as usize];
            i += 1
        }
        i = 0 as libc::c_int;
        min_distance = 1.7976931348623157e+308f64;
        while i < (*inst).n_samples {
            j = get_buf_index(inst, i);
            peer_distances[i as usize] =
                0.5f64 *
                    (*inst).peer_delays[get_runsbuf_index(inst, i) as usize] +
                    (*inst).peer_dispersions[j as usize];
            if peer_distances[i as usize] < min_distance {
                min_distance = peer_distances[i as usize]
            }
            i += 1
        }
        /* And now, work out the weight vector */
        precision = LCL_GetSysPrecisionAsQuantum();
        median_distance =
            RGR_FindMedian(peer_distances.as_mut_ptr(), (*inst).n_samples);
        sd = (median_distance - min_distance) / 0.7f64;
        sd =
            if precision > (if sd < min_distance { sd } else { min_distance })
               {
                precision
            } else if sd < min_distance { sd } else { min_distance };
        min_distance += precision;
        i = 0 as libc::c_int;
        while i < (*inst).n_samples {
            sd_weight = 1.0f64;
            if peer_distances[i as usize] > min_distance {
                sd_weight += (peer_distances[i as usize] - min_distance) / sd
            }
            weights[i as usize] = sd_weight * sd_weight;
            i += 1
        }
    }
    correct_asymmetry(inst, times_back.as_mut_ptr(), offsets.as_mut_ptr());
    (*inst).regression_ok =
        RGR_FindBestRegression(times_back.as_mut_ptr().offset((*inst).runs_samples
                                                                  as isize),
                               offsets.as_mut_ptr().offset((*inst).runs_samples
                                                               as isize),
                               weights.as_mut_ptr(), (*inst).n_samples,
                               (*inst).runs_samples, (*inst).min_samples,
                               &mut est_intercept, &mut est_slope,
                               &mut est_var, &mut est_intercept_sd,
                               &mut est_slope_sd, &mut best_start, &mut nruns,
                               &mut degrees_of_freedom);
    if (*inst).regression_ok != 0 {
        old_skew = (*inst).skew;
        old_freq = (*inst).estimated_frequency;
        (*inst).estimated_frequency = est_slope;
        (*inst).estimated_frequency_sd =
            if 1.0e-12f64 >
                   (if est_slope_sd < 1.0e+02f64 {
                        est_slope_sd
                    } else { 1.0e+02f64 }) {
                1.0e-12f64
            } else if est_slope_sd < 1.0e+02f64 {
                est_slope_sd
            } else { 1.0e+02f64 };
        (*inst).skew = est_slope_sd * RGR_GetTCoef(degrees_of_freedom);
        (*inst).estimated_offset = est_intercept;
        (*inst).offset_time =
            (*inst).sample_times[(*inst).last_sample as usize];
        (*inst).estimated_offset_sd = est_intercept_sd;
        (*inst).std_dev =
            if 1.0e-9f64 > sqrt(est_var) { 1.0e-9f64 } else { sqrt(est_var) };
        (*inst).nruns = nruns;
        (*inst).skew =
            if 1.0e-12f64 >
                   (if (*inst).skew < 1.0e+02f64 {
                        (*inst).skew
                    } else { 1.0e+02f64 }) {
                1.0e-12f64
            } else if (*inst).skew < 1.0e+02f64 {
                (*inst).skew
            } else { 1.0e+02f64 };
        stress = fabs(old_freq - (*inst).estimated_frequency) / old_skew;
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"off=%e freq=%e skew=%e n=%d bs=%d runs=%d asym=%f arun=%d\x00"
                            as *const u8 as *const libc::c_char,
                        (*inst).estimated_offset, (*inst).estimated_frequency,
                        (*inst).skew, (*inst).n_samples, best_start,
                        (*inst).nruns, (*inst).asymmetry,
                        (*inst).asymmetry_run);
        }
        if logfileid != -(1 as libc::c_int) {
            LOG_FileWrite(logfileid,
                          b"%s %-15s %10.3e %10.3e %10.3e %10.3e %10.3e %7.1e %3d %3d %3d %5.2f\x00"
                              as *const u8 as *const libc::c_char,
                          UTI_TimeToLogForm((*inst).offset_time.tv_sec),
                          if !(*inst).ip_addr.is_null() {
                              UTI_IPToString((*inst).ip_addr)
                          } else { UTI_RefidToString((*inst).refid) },
                          (*inst).std_dev, (*inst).estimated_offset,
                          (*inst).estimated_offset_sd,
                          (*inst).estimated_frequency, (*inst).skew, stress,
                          (*inst).n_samples, best_start, (*inst).nruns,
                          (*inst).asymmetry);
        }
        times_back_start = (*inst).runs_samples + best_start;
        prune_register(inst, best_start);
    } else {
        (*inst).estimated_frequency_sd = 2000.0f64 / 1.0e6f64;
        (*inst).skew = 2000.0f64 / 1.0e6f64;
        (*inst).estimated_offset_sd = 4.0f64;
        (*inst).std_dev = 4.0f64;
        (*inst).nruns = 0 as libc::c_int;
        if (*inst).n_samples > 0 as libc::c_int {
            (*inst).estimated_offset =
                (*inst).offsets[(*inst).last_sample as usize];
            (*inst).offset_time =
                (*inst).sample_times[(*inst).last_sample as usize]
        } else {
            (*inst).estimated_offset = 0.0f64;
            UTI_ZeroTimespec(&mut (*inst).offset_time);
        }
        times_back_start = 0 as libc::c_int
    }
    find_best_sample_index(inst,
                           times_back.as_mut_ptr().offset(times_back_start as
                                                              isize));
}
/* ================================================== */
/* Return the assumed worst case range of values that this source's
   frequency lies within.  Frequency is defined as the amount of time
   the local clock gains relative to the source per unit local clock
   time. */
#[no_mangle]
pub unsafe extern "C" fn SST_GetFrequencyRange(mut inst: SST_Stats,
                                               mut lo: *mut libc::c_double,
                                               mut hi: *mut libc::c_double) {
    let mut freq: libc::c_double = 0.;
    let mut skew: libc::c_double = 0.;
    freq = (*inst).estimated_frequency;
    skew = (*inst).skew;
    *lo = freq - skew;
    *hi = freq + skew;
    /* This function is currently used only to determine the values of delta
     and epsilon in the ntp_core module. Limit the skew to a reasonable maximum
     to avoid failing the dispersion test too easily. */
    if skew > 2000.0f64 / 1.0e6f64 {
        *lo = -(2000.0f64 / 1.0e6f64);
        *hi = 2000.0f64 / 1.0e6f64
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_GetSelectionData(mut inst: SST_Stats,
                                              mut now: *mut timespec,
                                              mut stratum: *mut libc::c_int,
                                              mut leap: *mut NTP_Leap,
                                              mut offset_lo_limit:
                                                  *mut libc::c_double,
                                              mut offset_hi_limit:
                                                  *mut libc::c_double,
                                              mut root_distance:
                                                  *mut libc::c_double,
                                              mut std_dev:
                                                  *mut libc::c_double,
                                              mut first_sample_ago:
                                                  *mut libc::c_double,
                                              mut last_sample_ago:
                                                  *mut libc::c_double,
                                              mut select_ok:
                                                  *mut libc::c_int) {
    let mut offset: libc::c_double = 0.;
    let mut sample_elapsed: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*inst).n_samples == 0 { *select_ok = 0 as libc::c_int; return }
    i = get_runsbuf_index(inst, (*inst).best_single_sample);
    j = get_buf_index(inst, (*inst).best_single_sample);
    *stratum = (*inst).stratum;
    *leap = (*inst).leap;
    *std_dev = (*inst).std_dev;
    sample_elapsed =
        fabs(UTI_DiffTimespecsToDouble(now,
                                       &mut *(*inst).sample_times.as_mut_ptr().offset(i
                                                                                          as
                                                                                          isize)));
    offset =
        (*inst).offsets[i as usize] +
            sample_elapsed * (*inst).estimated_frequency;
    *root_distance =
        0.5f64 * (*inst).root_delays[j as usize] +
            (*inst).root_dispersions[j as usize] +
            sample_elapsed * (*inst).skew;
    *offset_lo_limit = offset - *root_distance;
    *offset_hi_limit = offset + *root_distance;
    i = get_runsbuf_index(inst, 0 as libc::c_int);
    *first_sample_ago =
        UTI_DiffTimespecsToDouble(now,
                                  &mut *(*inst).sample_times.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize));
    i = get_runsbuf_index(inst, (*inst).n_samples - 1 as libc::c_int);
    *last_sample_ago =
        UTI_DiffTimespecsToDouble(now,
                                  &mut *(*inst).sample_times.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize));
    *select_ok = (*inst).regression_ok;
    /* If maxsamples is too small to have a successful regression, enable the
     selection as a special case for a fast update/print-once reference mode */
    if *select_ok == 0 && (*inst).n_samples < 3 as libc::c_int &&
           (*inst).n_samples == (*inst).max_samples {
        *std_dev = CNF_GetMaxJitter();
        *select_ok = 1 as libc::c_int
    }
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"n=%d off=%f dist=%f sd=%f first_ago=%f last_ago=%f selok=%d\x00"
                        as *const u8 as *const libc::c_char,
                    (*inst).n_samples, offset, *root_distance, *std_dev,
                    *first_sample_ago, *last_sample_ago, *select_ok);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_GetTrackingData(mut inst: SST_Stats,
                                             mut ref_time: *mut timespec,
                                             mut average_offset:
                                                 *mut libc::c_double,
                                             mut offset_sd:
                                                 *mut libc::c_double,
                                             mut frequency:
                                                 *mut libc::c_double,
                                             mut frequency_sd:
                                                 *mut libc::c_double,
                                             mut skew: *mut libc::c_double,
                                             mut root_delay:
                                                 *mut libc::c_double,
                                             mut root_dispersion:
                                                 *mut libc::c_double) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut elapsed_sample: libc::c_double = 0.;
    if (*inst).n_samples > 0 as libc::c_int {
    } else {
        __assert_fail(b"inst->n_samples > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"sourcestats.c\x00" as *const u8 as
                          *const libc::c_char,
                      734 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 125],
                                                &[libc::c_char; 125]>(b"void SST_GetTrackingData(SST_Stats, struct timespec *, double *, double *, double *, double *, double *, double *, double *)\x00")).as_ptr());
    }
    i = get_runsbuf_index(inst, (*inst).best_single_sample);
    j = get_buf_index(inst, (*inst).best_single_sample);
    *ref_time = (*inst).offset_time;
    *average_offset = (*inst).estimated_offset;
    *offset_sd = (*inst).estimated_offset_sd;
    *frequency = (*inst).estimated_frequency;
    *frequency_sd = (*inst).estimated_frequency_sd;
    *skew = (*inst).skew;
    *root_delay = (*inst).root_delays[j as usize];
    elapsed_sample =
        UTI_DiffTimespecsToDouble(&mut (*inst).offset_time,
                                  &mut *(*inst).sample_times.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize));
    *root_dispersion =
        (*inst).root_dispersions[j as usize] + (*inst).skew * elapsed_sample +
            *offset_sd;
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"n=%d off=%f offsd=%f freq=%e freqsd=%e skew=%e delay=%f disp=%f\x00"
                        as *const u8 as *const libc::c_char,
                    (*inst).n_samples, *average_offset, *offset_sd,
                    *frequency, *frequency_sd, *skew, *root_delay,
                    *root_dispersion);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_SlewSamples(mut inst: SST_Stats,
                                         mut when: *mut timespec,
                                         mut dfreq: libc::c_double,
                                         mut doffset: libc::c_double) {
    let mut m: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut delta_time: libc::c_double = 0.;
    let mut sample: *mut timespec = 0 as *mut timespec;
    let mut prev: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut prev_offset: libc::c_double = 0.;
    let mut prev_freq: libc::c_double = 0.;
    if (*inst).n_samples == 0 { return }
    m = -(*inst).runs_samples;
    while m < (*inst).n_samples {
        i = get_runsbuf_index(inst, m);
        sample =
            &mut *(*inst).sample_times.as_mut_ptr().offset(i as isize) as
                *mut timespec;
        prev = *sample;
        UTI_AdjustTimespec(sample, when, sample, &mut delta_time, dfreq,
                           doffset);
        (*inst).offsets[i as usize] += delta_time;
        m += 1
    }
    /* Update the regression estimates */
    prev = (*inst).offset_time;
    prev_offset = (*inst).estimated_offset;
    prev_freq = (*inst).estimated_frequency;
    UTI_AdjustTimespec(&mut (*inst).offset_time, when,
                       &mut (*inst).offset_time, &mut delta_time, dfreq,
                       doffset);
    (*inst).estimated_offset += delta_time;
    (*inst).estimated_frequency =
        ((*inst).estimated_frequency - dfreq) / (1.0f64 - dfreq);
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"n=%d m=%d old_off_time=%s new=%s old_off=%f new_off=%f old_freq=%.3f new_freq=%.3f\x00"
                        as *const u8 as *const libc::c_char,
                    (*inst).n_samples, (*inst).runs_samples,
                    UTI_TimespecToString(&mut prev),
                    UTI_TimespecToString(&mut (*inst).offset_time),
                    prev_offset, (*inst).estimated_offset,
                    1.0e6f64 * prev_freq,
                    1.0e6f64 * (*inst).estimated_frequency);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_AddDispersion(mut inst: SST_Stats,
                                           mut dispersion: libc::c_double) {
    let mut m: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    m = 0 as libc::c_int;
    while m < (*inst).n_samples {
        i = get_buf_index(inst, m);
        (*inst).root_dispersions[i as usize] += dispersion;
        (*inst).peer_dispersions[i as usize] += dispersion;
        m += 1
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_PredictOffset(mut inst: SST_Stats,
                                           mut when: *mut timespec)
 -> libc::c_double {
    let mut elapsed: libc::c_double = 0.;
    if (*inst).n_samples < 3 as libc::c_int {
        /* We don't have any useful statistics, and presumably the poll
       interval is minimal.  We can't do any useful prediction other
       than use the latest sample or zero if we don't have any samples */
        if (*inst).n_samples > 0 as libc::c_int {
            return (*inst).offsets[(*inst).last_sample as usize]
        } else { return 0.0f64 }
    } else {
        elapsed = UTI_DiffTimespecsToDouble(when, &mut (*inst).offset_time);
        return (*inst).estimated_offset +
                   elapsed * (*inst).estimated_frequency
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_MinRoundTripDelay(mut inst: SST_Stats)
 -> libc::c_double {
    if (*inst).fixed_min_delay > 0.0f64 { return (*inst).fixed_min_delay }
    if (*inst).n_samples == 0 { return 1.7976931348623157e+308f64 }
    return (*inst).peer_delays[(*inst).min_delay_sample as usize];
}
/* This function creates a new instance of the statistics handler */
/* This function deletes an instance of the statistics handler. */
/* This function resets an instance */
/* This function changes the reference ID and IP address */
/* This function accumulates a single sample into the statistics handler */
/* This function runs the linear regression operation on the data.  It
   finds the set of most recent samples that give the tightest
   confidence interval for the frequency, and truncates the register
   down to that number of samples. */
/* Return the assumed worst case range of values that this source's
   frequency lies within.  Frequency is defined as the amount of time
   the local clock gains relative to the source per unit local clock
   time. */
/* Get data needed for selection */
/* Get data needed when setting up tracking on this source */
/* This routine is called when the local machine clock parameters are
   changed.  It adjusts all existing samples that we are holding for
   each peer so that it looks like they were made under the new clock
   regime rather than the old one.

   when = cooked local time when the change occurs

   dfreq = delta frequency. positive means the clock has been adjusted
   because it was previously gaining time relative to the external
   reference(s).

   doffset = offset slewed onto local clock.  positive => local clock
   has been made fast by that amount.

*/
/* This routine is called when an indeterminate offset is introduced
   into the local time. */
/* Predict the offset of the local clock relative to a given source at
   a given local cooked time. Positive indicates local clock is FAST
   relative to reference. */
/* Find the minimum round trip delay in the register */
/* Get data needed for testing NTP delay */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_GetDelayTestData(mut inst: SST_Stats,
                                              mut sample_time: *mut timespec,
                                              mut last_sample_ago:
                                                  *mut libc::c_double,
                                              mut predicted_offset:
                                                  *mut libc::c_double,
                                              mut min_delay:
                                                  *mut libc::c_double,
                                              mut skew: *mut libc::c_double,
                                              mut std_dev:
                                                  *mut libc::c_double)
 -> libc::c_int {
    if (*inst).n_samples < 6 as libc::c_int { return 0 as libc::c_int }
    *last_sample_ago =
        UTI_DiffTimespecsToDouble(sample_time, &mut (*inst).offset_time);
    *predicted_offset =
        (*inst).estimated_offset +
            *last_sample_ago * (*inst).estimated_frequency;
    *min_delay = SST_MinRoundTripDelay(inst);
    *skew = (*inst).skew;
    *std_dev = (*inst).std_dev;
    return 1 as libc::c_int;
}
/* ================================================== */
/* This is used to save the register to a file, so that we can reload
   it after restarting the daemon */
#[no_mangle]
pub unsafe extern "C" fn SST_SaveToFile(mut inst: SST_Stats,
                                        mut out: *mut FILE) {
    let mut m: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    fprintf(out, b"%d\n\x00" as *const u8 as *const libc::c_char,
            (*inst).n_samples);
    m = 0 as libc::c_int;
    while m < (*inst).n_samples {
        i = get_runsbuf_index(inst, m);
        j = get_buf_index(inst, m);
        fprintf(out,
                b"%08lx %08lx %.6e %.6e %.6e %.6e %.6e %.6e %.6e %d\n\x00" as
                    *const u8 as *const libc::c_char,
                (*inst).sample_times[i as usize].tv_sec as uint64_t,
                ((*inst).sample_times[i as usize].tv_nsec as
                     libc::c_ulong).wrapping_div(1000 as libc::c_int as
                                                     libc::c_ulong),
                (*inst).offsets[i as usize], (*inst).orig_offsets[j as usize],
                (*inst).peer_delays[i as usize],
                (*inst).peer_dispersions[j as usize],
                (*inst).root_delays[j as usize],
                (*inst).root_dispersions[j as usize], 1.0f64,
                (*inst).stratum);
        m += 1
    }
    fprintf(out, b"%d\n\x00" as *const u8 as *const libc::c_char,
            (*inst).asymmetry_run);
}
/* ================================================== */
/* This is used to reload samples from a file */
#[no_mangle]
pub unsafe extern "C" fn SST_LoadFromFile(mut inst: SST_Stats,
                                          mut in_0: *mut FILE)
 -> libc::c_int {
    let mut sec: uint64_t = 0;
    let mut usec: libc::c_ulong = 0;
    let mut i: libc::c_int = 0;
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut weight: libc::c_double = 0.;
    SST_ResetInstance(inst);
    if !fgets(line.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                  as libc::c_int, in_0).is_null() &&
           sscanf(line.as_mut_ptr(),
                  b"%d\x00" as *const u8 as *const libc::c_char,
                  &mut (*inst).n_samples as *mut libc::c_int) ==
               1 as libc::c_int && (*inst).n_samples >= 0 as libc::c_int &&
           (*inst).n_samples <= 64 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*inst).n_samples {
            if fgets(line.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong as libc::c_int, in_0).is_null() ||
                   sscanf(line.as_mut_ptr(),
                          b"%lx%lx%lf%lf%lf%lf%lf%lf%lf%d\n\x00" as *const u8
                              as *const libc::c_char,
                          &mut sec as *mut uint64_t,
                          &mut usec as *mut libc::c_ulong,
                          &mut *(*inst).offsets.as_mut_ptr().offset(i as
                                                                        isize)
                              as *mut libc::c_double,
                          &mut *(*inst).orig_offsets.as_mut_ptr().offset(i as
                                                                             isize)
                              as *mut libc::c_double,
                          &mut *(*inst).peer_delays.as_mut_ptr().offset(i as
                                                                            isize)
                              as *mut libc::c_double,
                          &mut *(*inst).peer_dispersions.as_mut_ptr().offset(i
                                                                                 as
                                                                                 isize)
                              as *mut libc::c_double,
                          &mut *(*inst).root_delays.as_mut_ptr().offset(i as
                                                                            isize)
                              as *mut libc::c_double,
                          &mut *(*inst).root_dispersions.as_mut_ptr().offset(i
                                                                                 as
                                                                                 isize)
                              as *mut libc::c_double,
                          &mut weight as *mut libc::c_double,
                          &mut (*inst).stratum as *mut libc::c_int) !=
                       10 as libc::c_int {
                /* This is the branch taken if the read FAILED */
                (*inst).n_samples =
                    0 as
                        libc::c_int; /* Load abandoned if any sign of corruption */
                return 0 as libc::c_int
            } else {
                /* This is the branch taken if the read is SUCCESSFUL */
                (*inst).sample_times[i as usize].tv_sec = sec as __time_t;
                (*inst).sample_times[i as usize].tv_nsec =
                    (1000 as libc::c_int as libc::c_ulong).wrapping_mul(usec)
                        as __syscall_slong_t;
                UTI_NormaliseTimespec(&mut *(*inst).sample_times.as_mut_ptr().offset(i
                                                                                         as
                                                                                         isize));
            }
            i += 1
        }
        /* This field was not saved in older versions */
        if fgets(line.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong as libc::c_int, in_0).is_null() ||
               sscanf(line.as_mut_ptr(),
                      b"%d\n\x00" as *const u8 as *const libc::c_char,
                      &mut (*inst).asymmetry_run as *mut libc::c_int) !=
                   1 as libc::c_int {
            (*inst).asymmetry_run = 0 as libc::c_int
        }
    } else {
        (*inst).n_samples =
            0 as libc::c_int; /* Load abandoned if any sign of corruption */
        return 0 as libc::c_int
    }
    if (*inst).n_samples == 0 { return 1 as libc::c_int }
    (*inst).last_sample = (*inst).n_samples - 1 as libc::c_int;
    find_min_delay_sample(inst);
    SST_DoNewRegression(inst);
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_DoSourceReport(mut inst: SST_Stats,
                                            mut report: *mut RPT_SourceReport,
                                            mut now: *mut timespec) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut last_sample_time: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    if (*inst).n_samples > 0 as libc::c_int {
        i = get_runsbuf_index(inst, (*inst).n_samples - 1 as libc::c_int);
        j = get_buf_index(inst, (*inst).n_samples - 1 as libc::c_int);
        (*report).orig_latest_meas = (*inst).orig_offsets[j as usize];
        (*report).latest_meas = (*inst).offsets[i as usize];
        (*report).latest_meas_err =
            0.5f64 * (*inst).root_delays[j as usize] +
                (*inst).root_dispersions[j as usize];
        (*report).stratum = (*inst).stratum;
        /* Align the sample time to reduce the leak of the receive timestamp */
        last_sample_time = (*inst).sample_times[i as usize];
        last_sample_time.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        (*report).latest_meas_ago =
            UTI_DiffTimespecsToDouble(now, &mut last_sample_time) as
                libc::c_ulong
    } else {
        (*report).latest_meas_ago =
            -(1 as libc::c_int) as uint32_t as libc::c_ulong;
        (*report).orig_latest_meas = 0 as libc::c_int as libc::c_double;
        (*report).latest_meas = 0 as libc::c_int as libc::c_double;
        (*report).latest_meas_err = 0 as libc::c_int as libc::c_double;
        (*report).stratum = 0 as libc::c_int
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_Samples(mut inst: SST_Stats) -> libc::c_int {
    return (*inst).n_samples;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_DoSourcestatsReport(mut inst: SST_Stats,
                                                 mut report:
                                                     *mut RPT_SourcestatsReport,
                                                 mut now: *mut timespec) {
    let mut dspan: libc::c_double = 0.;
    let mut elapsed: libc::c_double = 0.;
    let mut sample_elapsed: libc::c_double = 0.;
    let mut bi: libc::c_int = 0;
    let mut bj: libc::c_int = 0;
    (*report).n_samples = (*inst).n_samples as libc::c_ulong;
    (*report).n_runs = (*inst).nruns as libc::c_ulong;
    if (*inst).n_samples > 0 as libc::c_int {
        bi = get_runsbuf_index(inst, (*inst).best_single_sample);
        bj = get_buf_index(inst, (*inst).best_single_sample);
        dspan =
            UTI_DiffTimespecsToDouble(&mut *(*inst).sample_times.as_mut_ptr().offset((*inst).last_sample
                                                                                         as
                                                                                         isize),
                                      &mut *(*inst).sample_times.as_mut_ptr().offset((get_runsbuf_index
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   SST_Stats,
                                                                                                               _:
                                                                                                                   libc::c_int)
                                                                                              ->
                                                                                                  libc::c_int)(inst,
                                                                                                               0
                                                                                                                   as
                                                                                                                   libc::c_int)
                                                                                         as
                                                                                         isize));
        elapsed = UTI_DiffTimespecsToDouble(now, &mut (*inst).offset_time);
        sample_elapsed =
            UTI_DiffTimespecsToDouble(now,
                                      &mut *(*inst).sample_times.as_mut_ptr().offset(bi
                                                                                         as
                                                                                         isize));
        (*report).span_seconds = round(dspan) as libc::c_ulong;
        (*report).est_offset =
            (*inst).estimated_offset + elapsed * (*inst).estimated_frequency;
        (*report).est_offset_err =
            (*inst).estimated_offset_sd + sample_elapsed * (*inst).skew +
                (0.5f64 * (*inst).root_delays[bj as usize] +
                     (*inst).root_dispersions[bj as usize])
    } else {
        (*report).span_seconds = 0 as libc::c_int as libc::c_ulong;
        (*report).est_offset = 0 as libc::c_int as libc::c_double;
        (*report).est_offset_err = 0 as libc::c_int as libc::c_double
    }
    (*report).resid_freq_ppm = 1.0e6f64 * (*inst).estimated_frequency;
    (*report).skew_ppm = 1.0e6f64 * (*inst).skew;
    (*report).sd = (*inst).std_dev;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SST_GetJitterAsymmetry(mut inst: SST_Stats)
 -> libc::c_double {
    return (*inst).asymmetry;
}
/* ================================================== */
