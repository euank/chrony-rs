#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type ARR_Instance_Record;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    /* Create a new array with given element size */
    #[no_mangle]
    fn ARR_CreateInstance(elem_size: libc::c_uint) -> ARR_Instance;
    /* Destroy the array */
    #[no_mangle]
    fn ARR_DestroyInstance(array: ARR_Instance);
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
    #[no_mangle]
    fn CNF_GetClientLogLimit() -> libc::c_ulong;
    #[no_mangle]
    fn CNF_GetNoClientLog() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetCommandRateLimit(interval: *mut libc::c_int,
                               burst: *mut libc::c_int,
                               leak: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn CNF_GetNTPRateLimit(interval: *mut libc::c_int,
                           burst: *mut libc::c_int, leak: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn UTI_ZeroNtp64(ts: *mut NTP_int64);
    #[no_mangle]
    fn UTI_IPToHash(ip: *mut IPAddr) -> uint32_t;
    #[no_mangle]
    fn UTI_CompareIPs(a: *mut IPAddr, b: *mut IPAddr, mask: *mut IPAddr)
     -> libc::c_int;
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
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
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
pub type int32_t = __int32_t;
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
pub struct NTP_int64 {
    pub hi: uint32_t,
    pub lo: uint32_t,
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
pub struct RPT_ServerStatsReport {
    pub ntp_hits: uint32_t,
    pub cmd_hits: uint32_t,
    pub ntp_drops: uint32_t,
    pub cmd_drops: uint32_t,
    pub log_drops: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Record {
    pub ip_addr: IPAddr,
    pub last_ntp_hit: uint32_t,
    pub last_cmd_hit: uint32_t,
    pub ntp_hits: uint32_t,
    pub cmd_hits: uint32_t,
    pub ntp_drops: uint16_t,
    pub cmd_drops: uint16_t,
    pub ntp_tokens: uint16_t,
    pub cmd_tokens: uint16_t,
    pub ntp_rate: int8_t,
    pub cmd_rate: int8_t,
    pub ntp_timeout_rate: int8_t,
    pub flags: uint8_t,
    pub ntp_rx_ts: NTP_int64,
    pub ntp_tx_ts: NTP_int64,
}
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
/* Hash table of records, there is a fixed number of records per slot */
static mut records: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
/* Minimum number of slots */
/* Maximum number of slots, this is a hard limit */
/* Number of slots in the hash table */
static mut slots: libc::c_uint = 0;
/* Maximum number of slots given memory allocation limit */
static mut max_slots: libc::c_uint = 0;
/* Static offset included in conversion to the fixed-point timestamps to
   randomise their alignment */
static mut ts_offset: uint32_t = 0;
static mut max_ntp_tokens: uint16_t = 0;
static mut max_cmd_tokens: uint16_t = 0;
static mut ntp_tokens_per_packet: uint16_t = 0;
static mut cmd_tokens_per_packet: uint16_t = 0;
/* Reduction of token rates to avoid overflow of 16-bit counters.  Negative
   shift is used for coarse limiting with intervals shorter than -TS_FRAC. */
static mut ntp_token_shift: libc::c_int = 0;
static mut cmd_token_shift: libc::c_int = 0;
/* Rates at which responses are randomly allowed (in log2) when the
   buckets don't have enough tokens.  This is necessary in order to
   prevent an attacker sending requests with spoofed source address
   from blocking responses to the address completely. */
static mut ntp_leak_rate: libc::c_int = 0;
static mut cmd_leak_rate: libc::c_int = 0;
/* NTP limit interval in log2 */
static mut ntp_limit_interval: libc::c_int = 0;
/* Flag indicating whether facility is turned on or not */
static mut active: libc::c_int = 0;
/* Global statistics */
static mut total_ntp_hits: uint32_t = 0;
static mut total_cmd_hits: uint32_t = 0;
static mut total_ntp_drops: uint32_t = 0;
static mut total_cmd_drops: uint32_t = 0;
static mut total_record_drops: uint32_t = 0;
/* ================================================== */
unsafe extern "C" fn compare_ts(mut x: uint32_t, mut y: uint32_t)
 -> libc::c_int {
    if x == y { return 0 as libc::c_int }
    if y == 0 as libc::c_int as libc::c_uint { return 1 as libc::c_int }
    return if x.wrapping_sub(y) as int32_t > 0 as libc::c_int {
               1 as libc::c_int
           } else { -(1 as libc::c_int) };
}
/* ================================================== */
unsafe extern "C" fn get_record(mut ip: *mut IPAddr) -> *mut Record {
    let mut first: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut last_hit: time_t = 0;
    let mut oldest_hit: time_t = 0 as libc::c_int as time_t;
    let mut record: *mut Record = 0 as *mut Record;
    let mut oldest_record: *mut Record = 0 as *mut Record;
    if active == 0 ||
           (*ip).family as libc::c_int != 1 as libc::c_int &&
               (*ip).family as libc::c_int != 2 as libc::c_int {
        return 0 as *mut Record
    }
    loop  {
        /* Get index of the first record in the slot */
        first =
            UTI_IPToHash(ip).wrapping_rem(slots).wrapping_mul((1 as
                                                                   libc::c_uint)
                                                                  <<
                                                                  4 as
                                                                      libc::c_int);
        i = 0 as libc::c_int as libc::c_uint;
        oldest_record = 0 as *mut Record;
        while i < (1 as libc::c_uint) << 4 as libc::c_int {
            record =
                ARR_GetElement(records, first.wrapping_add(i)) as *mut Record;
            if UTI_CompareIPs(ip, &mut (*record).ip_addr, 0 as *mut IPAddr) ==
                   0 {
                return record
            }
            if (*record).ip_addr.family as libc::c_int == 0 as libc::c_int {
                break ;
            }
            last_hit =
                if compare_ts((*record).last_ntp_hit, (*record).last_cmd_hit)
                       > 0 as libc::c_int {
                    (*record).last_ntp_hit
                } else { (*record).last_cmd_hit } as time_t;
            if oldest_record.is_null() ||
                   compare_ts(oldest_hit as uint32_t, last_hit as uint32_t) >
                       0 as libc::c_int ||
                   oldest_hit == last_hit &&
                       (*record).ntp_hits.wrapping_add((*record).cmd_hits) <
                           (*oldest_record).ntp_hits.wrapping_add((*oldest_record).cmd_hits)
               {
                oldest_record = record;
                oldest_hit = last_hit
            }
            i = i.wrapping_add(1)
        }
        /* If the slot still has an empty record, use it */
        if (*record).ip_addr.family as libc::c_int == 0 as libc::c_int {
            break ;
        }
        /* Resize the table if possible and try again as the new slot may
       have some empty records */
        if expand_hashtable() != 0 { continue ; }
        /* There is no other option, replace the oldest record */
        record = oldest_record;
        total_record_drops = total_record_drops.wrapping_add(1);
        break ;
    }
    (*record).ip_addr = *ip;
    (*record).last_cmd_hit = 0 as libc::c_int as uint32_t;
    (*record).last_ntp_hit = (*record).last_cmd_hit;
    (*record).cmd_hits = 0 as libc::c_int as uint32_t;
    (*record).ntp_hits = (*record).cmd_hits;
    (*record).cmd_drops = 0 as libc::c_int as uint16_t;
    (*record).ntp_drops = (*record).cmd_drops;
    (*record).ntp_tokens = max_ntp_tokens;
    (*record).cmd_tokens = max_cmd_tokens;
    (*record).cmd_rate = -(128 as libc::c_int) as int8_t;
    (*record).ntp_rate = (*record).cmd_rate;
    (*record).ntp_timeout_rate = -(128 as libc::c_int) as int8_t;
    (*record).flags = 0 as libc::c_int as uint8_t;
    UTI_ZeroNtp64(&mut (*record).ntp_rx_ts);
    UTI_ZeroNtp64(&mut (*record).ntp_tx_ts);
    return record;
}
/* ================================================== */
/* ================================================== */
unsafe extern "C" fn expand_hashtable() -> libc::c_int {
    let mut old_records: ARR_Instance = 0 as *mut ARR_Instance_Record;
    let mut old_record: *mut Record = 0 as *mut Record;
    let mut new_record: *mut Record = 0 as *mut Record;
    let mut i: libc::c_uint = 0;
    old_records = records;
    if (2 as libc::c_int as libc::c_uint).wrapping_mul(slots) > max_slots {
        return 0 as libc::c_int
    }
    records =
        ARR_CreateInstance(::std::mem::size_of::<Record>() as libc::c_ulong as
                               libc::c_uint);
    slots =
        if 1 as libc::c_int as libc::c_uint >
               (2 as libc::c_int as libc::c_uint).wrapping_mul(slots) {
            1 as libc::c_int as libc::c_uint
        } else { (2 as libc::c_int as libc::c_uint).wrapping_mul(slots) };
    if slots <= max_slots {
    } else {
        __assert_fail(b"slots <= max_slots\x00" as *const u8 as
                          *const libc::c_char,
                      b"clientlog.c\x00" as *const u8 as *const libc::c_char,
                      245 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"int expand_hashtable(void)\x00")).as_ptr());
    }
    ARR_SetSize(records,
                slots.wrapping_mul((1 as libc::c_uint) << 4 as libc::c_int));
    /* Mark all new records as empty */
    i = 0 as libc::c_int as libc::c_uint;
    while i < slots.wrapping_mul((1 as libc::c_uint) << 4 as libc::c_int) {
        new_record = ARR_GetElement(records, i) as *mut Record;
        (*new_record).ip_addr.family = 0 as libc::c_int as uint16_t;
        i = i.wrapping_add(1)
    }
    if old_records.is_null() { return 1 as libc::c_int }
    /* Copy old records to the new hash table */
    i = 0 as libc::c_int as libc::c_uint;
    while i < ARR_GetSize(old_records) {
        old_record = ARR_GetElement(old_records, i) as *mut Record;
        if !((*old_record).ip_addr.family as libc::c_int == 0 as libc::c_int)
           {
            new_record = get_record(&mut (*old_record).ip_addr);
            if !new_record.is_null() {
            } else {
                __assert_fail(b"new_record\x00" as *const u8 as
                                  *const libc::c_char,
                              b"clientlog.c\x00" as *const u8 as
                                  *const libc::c_char,
                              266 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 27],
                                                        &[libc::c_char; 27]>(b"int expand_hashtable(void)\x00")).as_ptr());
            }
            *new_record = *old_record
        }
        i = i.wrapping_add(1)
    }
    ARR_DestroyInstance(old_records);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn set_bucket_params(mut interval: libc::c_int,
                                       mut burst: libc::c_int,
                                       mut max_tokens: *mut uint16_t,
                                       mut tokens_per_packet: *mut uint16_t,
                                       mut token_shift: *mut libc::c_int) {
    interval =
        if -(15 as libc::c_int) - 4 as libc::c_int >
               (if interval < 12 as libc::c_int {
                    interval
                } else { 12 as libc::c_int }) {
            (-(15 as libc::c_int)) - 4 as libc::c_int
        } else if interval < 12 as libc::c_int {
            interval
        } else { 12 as libc::c_int };
    burst =
        if 1 as libc::c_int >
               (if burst < 255 as libc::c_int {
                    burst
                } else { 255 as libc::c_int }) {
            1 as libc::c_int
        } else if burst < 255 as libc::c_int {
            burst
        } else { 255 as libc::c_int };
    if interval >= -(4 as libc::c_int) {
        /* Find the smallest shift with which the maximum number fits in 16 bits */
        *token_shift = 0 as libc::c_int;
        while *token_shift < interval + 4 as libc::c_int {
            if ((burst << 4 as libc::c_int + interval - *token_shift) as
                    libc::c_uint) < (1 as libc::c_uint) << 16 as libc::c_int {
                break ;
            }
            *token_shift += 1
        }
    } else {
        /* Coarse rate limiting */
        *token_shift = interval + 4 as libc::c_int;
        *tokens_per_packet = 1 as libc::c_int as uint16_t;
        burst =
            if (1 as libc::c_uint) << -*token_shift > burst as libc::c_uint {
                ((1 as libc::c_uint)) << -*token_shift
            } else { burst as libc::c_uint } as libc::c_int
    }
    *tokens_per_packet =
        ((1 as libc::c_uint) << 4 as libc::c_int + interval - *token_shift) as
            uint16_t;
    *max_tokens = (*tokens_per_packet as libc::c_int * burst) as uint16_t;
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"Tokens max %d packet %d shift %d\x00" as *const u8 as
                        *const libc::c_char, *max_tokens as libc::c_int,
                    *tokens_per_packet as libc::c_int, *token_shift);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CLG_Initialise() {
    let mut interval: libc::c_int = 0;
    let mut burst: libc::c_int = 0;
    let mut leak_rate: libc::c_int = 0;
    max_cmd_tokens = 0 as libc::c_int as uint16_t;
    max_ntp_tokens = max_cmd_tokens;
    cmd_tokens_per_packet = 0 as libc::c_int as uint16_t;
    ntp_tokens_per_packet = cmd_tokens_per_packet;
    cmd_token_shift = 0 as libc::c_int;
    ntp_token_shift = cmd_token_shift;
    cmd_leak_rate = 0 as libc::c_int;
    ntp_leak_rate = cmd_leak_rate;
    ntp_limit_interval = -(15 as libc::c_int) - 4 as libc::c_int;
    if CNF_GetNTPRateLimit(&mut interval, &mut burst, &mut leak_rate) != 0 {
        set_bucket_params(interval, burst, &mut max_ntp_tokens,
                          &mut ntp_tokens_per_packet, &mut ntp_token_shift);
        ntp_leak_rate =
            if 1 as libc::c_int >
                   (if leak_rate < 4 as libc::c_int {
                        leak_rate
                    } else { 4 as libc::c_int }) {
                1 as libc::c_int
            } else if leak_rate < 4 as libc::c_int {
                leak_rate
            } else { 4 as libc::c_int };
        ntp_limit_interval =
            if -(15 as libc::c_int) - 4 as libc::c_int >
                   (if interval < 12 as libc::c_int {
                        interval
                    } else { 12 as libc::c_int }) {
                (-(15 as libc::c_int)) - 4 as libc::c_int
            } else if interval < 12 as libc::c_int {
                interval
            } else { 12 as libc::c_int }
    }
    if CNF_GetCommandRateLimit(&mut interval, &mut burst, &mut leak_rate) != 0
       {
        set_bucket_params(interval, burst, &mut max_cmd_tokens,
                          &mut cmd_tokens_per_packet, &mut cmd_token_shift);
        cmd_leak_rate =
            if 1 as libc::c_int >
                   (if leak_rate < 4 as libc::c_int {
                        leak_rate
                    } else { 4 as libc::c_int }) {
                1 as libc::c_int
            } else if leak_rate < 4 as libc::c_int {
                leak_rate
            } else { 4 as libc::c_int }
    }
    active = (CNF_GetNoClientLog() == 0) as libc::c_int;
    if active == 0 {
        if ntp_leak_rate != 0 || cmd_leak_rate != 0 {
            LOG_Message(LOGS_FATAL,
                        b"ratelimit cannot be used with noclientlog\x00" as
                            *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        return
    }
    /* Calculate the maximum number of slots that can be allocated in the
     configured memory limit.  Take into account expanding of the hash
     table where two copies exist at the same time. */
    max_slots =
        CNF_GetClientLogLimit().wrapping_div((::std::mem::size_of::<Record>()
                                                  as
                                                  libc::c_ulong).wrapping_mul(((1
                                                                                    as
                                                                                    libc::c_uint)
                                                                                   <<
                                                                                   4
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  libc::c_ulong).wrapping_mul(3
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_ulong).wrapping_div(2
                                                                                                                                                  as
                                                                                                                                                  libc::c_int
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong))
            as libc::c_uint;
    max_slots =
        if 1 as libc::c_int as libc::c_uint >
               (if max_slots <
                       (1 as libc::c_uint) <<
                           24 as libc::c_int - 4 as libc::c_int {
                    max_slots
                } else {
                    ((1 as libc::c_uint)) <<
                        24 as libc::c_int - 4 as libc::c_int
                }) {
            1 as libc::c_int as libc::c_uint
        } else if max_slots <
                      (1 as libc::c_uint) <<
                          24 as libc::c_int - 4 as libc::c_int {
            max_slots
        } else {
            ((1 as libc::c_uint)) << 24 as libc::c_int - 4 as libc::c_int
        };
    slots = 0 as libc::c_int as libc::c_uint;
    records = 0 as ARR_Instance;
    expand_hashtable();
    UTI_GetRandomBytes(&mut ts_offset as *mut uint32_t as *mut libc::c_void,
                       ::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                           libc::c_uint);
    ts_offset =
        (ts_offset as
             libc::c_uint).wrapping_rem((1000000000 as
                                             libc::c_uint).wrapping_div((1 as
                                                                             libc::c_uint)
                                                                            <<
                                                                            4
                                                                                as
                                                                                libc::c_int))
            as uint32_t as uint32_t;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CLG_Finalise() {
    if active == 0 { return }
    ARR_DestroyInstance(records);
}
/* ================================================== */
unsafe extern "C" fn get_ts_from_timespec(mut ts: *mut timespec) -> uint32_t {
    let mut sec: uint32_t = (*ts).tv_sec as uint32_t;
    let mut nsec: uint32_t = (*ts).tv_nsec as uint32_t;
    nsec =
        (nsec as libc::c_uint).wrapping_add(ts_offset) as uint32_t as
            uint32_t;
    if nsec >= 1000000000 as libc::c_uint {
        nsec =
            (nsec as libc::c_uint).wrapping_sub(1000000000 as libc::c_uint) as
                uint32_t as uint32_t;
        sec = sec.wrapping_add(1)
    }
    /* This is fast and accurate enough */
    return sec << 4 as libc::c_int |
               (140740 as
                    libc::c_uint).wrapping_mul(nsec >> 15 as libc::c_int) >>
                   32 as libc::c_int - 4 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn update_record(mut now: *mut timespec,
                                   mut last_hit: *mut uint32_t,
                                   mut hits: *mut uint32_t,
                                   mut tokens: *mut uint16_t,
                                   mut max_tokens: uint32_t,
                                   mut token_shift: libc::c_int,
                                   mut rate: *mut int8_t) {
    let mut interval: uint32_t = 0;
    let mut now_ts: uint32_t = 0;
    let mut prev_hit: uint32_t = 0;
    let mut new_tokens: uint32_t = 0;
    let mut interval2: libc::c_int = 0;
    now_ts = get_ts_from_timespec(now);
    prev_hit = *last_hit;
    *last_hit = now_ts;
    *hits = (*hits).wrapping_add(1);
    interval = now_ts.wrapping_sub(prev_hit);
    if prev_hit == 0 as libc::c_int as libc::c_uint ||
           (interval as int32_t) < 0 as libc::c_int {
        return
    }
    if token_shift >= 0 as libc::c_int {
        new_tokens =
            (now_ts >> token_shift).wrapping_sub(prev_hit >> token_shift)
    } else if now_ts.wrapping_sub(prev_hit) > max_tokens {
        new_tokens = max_tokens
    } else { new_tokens = now_ts.wrapping_sub(prev_hit) << -token_shift }
    *tokens =
        if (*tokens as libc::c_uint).wrapping_add(new_tokens) < max_tokens {
            (*tokens as libc::c_uint).wrapping_add(new_tokens)
        } else { max_tokens } as uint16_t;
    /* Convert the interval to scaled and rounded log2 */
    if interval != 0 {
        interval =
            (interval as
                 libc::c_uint).wrapping_add(interval >> 1 as libc::c_int) as
                uint32_t as uint32_t;
        interval2 = -(4 as libc::c_int) * 4 as libc::c_int;
        while interval2 < -(-(14 as libc::c_int) * 4 as libc::c_int) {
            if interval <= 1 as libc::c_int as libc::c_uint { break ; }
            interval >>= 1 as libc::c_int;
            interval2 += 4 as libc::c_int
        }
    } else {
        interval2 =
            -(4 as libc::c_int) * (4 as libc::c_int + 1 as libc::c_int)
    }
    /* Update the rate in a rough approximation of exponential moving average */
    if *rate as libc::c_int == -(128 as libc::c_int) {
        *rate = -interval2 as int8_t
    } else if (*rate as libc::c_int) < -interval2 {
        *rate += 1
    } else if *rate as libc::c_int > -interval2 {
        if *rate as libc::c_int >
               4 as libc::c_int * 5 as libc::c_int / 2 as libc::c_int -
                   interval2 {
            *rate =
                (4 as libc::c_int * 5 as libc::c_int / 2 as libc::c_int -
                     interval2) as int8_t
        } else {
            *rate =
                ((*rate as libc::c_int - interval2 - 1 as libc::c_int) /
                     2 as libc::c_int) as int8_t
        }
    };
}
/* ================================================== */
unsafe extern "C" fn get_index(mut record: *mut Record) -> libc::c_int {
    return record.wrapping_offset_from(ARR_GetElements(records) as
                                           *mut Record) as libc::c_long as
               libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CLG_GetClientIndex(mut client: *mut IPAddr)
 -> libc::c_int {
    let mut record: *mut Record = 0 as *mut Record;
    record = get_record(client);
    if record.is_null() { return -(1 as libc::c_int) }
    return get_index(record);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CLG_LogNTPAccess(mut client: *mut IPAddr,
                                          mut now: *mut timespec)
 -> libc::c_int {
    let mut record: *mut Record = 0 as *mut Record;
    total_ntp_hits = total_ntp_hits.wrapping_add(1);
    record = get_record(client);
    if record.is_null() { return -(1 as libc::c_int) }
    /* Update one of the two rates depending on whether the previous request
     of the client had a reply or it timed out */
    update_record(now, &mut (*record).last_ntp_hit, &mut (*record).ntp_hits,
                  &mut (*record).ntp_tokens, max_ntp_tokens as uint32_t,
                  ntp_token_shift,
                  if (*record).flags as libc::c_int & 0x1 as libc::c_int != 0
                     {
                      &mut (*record).ntp_timeout_rate
                  } else { &mut (*record).ntp_rate });
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"NTP hits %u rate %d trate %d tokens %d\x00" as *const u8
                        as *const libc::c_char, (*record).ntp_hits,
                    (*record).ntp_rate as libc::c_int,
                    (*record).ntp_timeout_rate as libc::c_int,
                    (*record).ntp_tokens as libc::c_int);
    }
    return get_index(record);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CLG_LogCommandAccess(mut client: *mut IPAddr,
                                              mut now: *mut timespec)
 -> libc::c_int {
    let mut record: *mut Record = 0 as *mut Record;
    total_cmd_hits = total_cmd_hits.wrapping_add(1);
    record = get_record(client);
    if record.is_null() { return -(1 as libc::c_int) }
    update_record(now, &mut (*record).last_cmd_hit, &mut (*record).cmd_hits,
                  &mut (*record).cmd_tokens, max_cmd_tokens as uint32_t,
                  cmd_token_shift, &mut (*record).cmd_rate);
    if 0 as libc::c_int != 0 &&
           log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(LOGS_DEBUG,
                    b"Cmd hits %u rate %d tokens %d\x00" as *const u8 as
                        *const libc::c_char, (*record).cmd_hits,
                    (*record).cmd_rate as libc::c_int,
                    (*record).cmd_tokens as libc::c_int);
    }
    return get_index(record);
}
/* ================================================== */
unsafe extern "C" fn limit_response_random(mut leak_rate: libc::c_int)
 -> libc::c_int {
    static mut rnd: uint32_t = 0;
    static mut bits_left: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0;
    if bits_left < leak_rate {
        UTI_GetRandomBytes(&mut rnd as *mut uint32_t as *mut libc::c_void,
                           ::std::mem::size_of::<uint32_t>() as libc::c_ulong
                               as libc::c_uint);
        bits_left =
            (8 as libc::c_int as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint32_t>()
                                                 as libc::c_ulong) as
                libc::c_int
    }
    /* Return zero on average once per 2^leak_rate */
    r =
        if rnd.wrapping_rem((1 as libc::c_uint) << leak_rate) != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    rnd >>= leak_rate;
    bits_left -= leak_rate;
    return r;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CLG_LimitNTPResponseRate(mut index: libc::c_int)
 -> libc::c_int {
    let mut record: *mut Record = 0 as *mut Record;
    let mut drop_0: libc::c_int = 0;
    if ntp_tokens_per_packet == 0 { return 0 as libc::c_int }
    record = ARR_GetElement(records, index as libc::c_uint) as *mut Record;
    (*record).flags =
        ((*record).flags as libc::c_int & !(0x1 as libc::c_int)) as uint8_t;
    if (*record).ntp_tokens as libc::c_int >=
           ntp_tokens_per_packet as libc::c_int {
        (*record).ntp_tokens =
            ((*record).ntp_tokens as libc::c_int -
                 ntp_tokens_per_packet as libc::c_int) as uint16_t;
        return 0 as libc::c_int
    }
    drop_0 = limit_response_random(ntp_leak_rate);
    /* Poorly implemented clients may send new requests at even a higher rate
     when they are not getting replies.  If the request rate seems to be more
     than twice as much as when replies are sent, give up on rate limiting to
     reduce the amount of traffic.  Invert the sense of the leak to respond to
     most of the requests, but still keep the estimated rate updated. */
    if (*record).ntp_timeout_rate as libc::c_int != -(128 as libc::c_int) &&
           (*record).ntp_timeout_rate as libc::c_int >
               (*record).ntp_rate as libc::c_int + 4 as libc::c_int {
        drop_0 = (drop_0 == 0) as libc::c_int
    }
    if drop_0 == 0 {
        (*record).ntp_tokens = 0 as libc::c_int as uint16_t;
        return 0 as libc::c_int
    }
    (*record).flags =
        ((*record).flags as libc::c_int | 0x1 as libc::c_int) as uint8_t;
    (*record).ntp_drops = (*record).ntp_drops.wrapping_add(1);
    total_ntp_drops = total_ntp_drops.wrapping_add(1);
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CLG_LimitCommandResponseRate(mut index: libc::c_int)
 -> libc::c_int {
    let mut record: *mut Record = 0 as *mut Record;
    if cmd_tokens_per_packet == 0 { return 0 as libc::c_int }
    record = ARR_GetElement(records, index as libc::c_uint) as *mut Record;
    if (*record).cmd_tokens as libc::c_int >=
           cmd_tokens_per_packet as libc::c_int {
        (*record).cmd_tokens =
            ((*record).cmd_tokens as libc::c_int -
                 cmd_tokens_per_packet as libc::c_int) as uint16_t;
        return 0 as libc::c_int
    }
    if limit_response_random(cmd_leak_rate) == 0 {
        (*record).cmd_tokens = 0 as libc::c_int as uint16_t;
        return 0 as libc::c_int
    }
    (*record).cmd_drops = (*record).cmd_drops.wrapping_add(1);
    total_cmd_drops = total_cmd_drops.wrapping_add(1);
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CLG_GetNtpTimestamps(mut index: libc::c_int,
                                              mut rx_ts: *mut *mut NTP_int64,
                                              mut tx_ts:
                                                  *mut *mut NTP_int64) {
    let mut record: *mut Record = 0 as *mut Record;
    record = ARR_GetElement(records, index as libc::c_uint) as *mut Record;
    *rx_ts = &mut (*record).ntp_rx_ts;
    *tx_ts = &mut (*record).ntp_tx_ts;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CLG_GetNtpMinPoll() -> libc::c_int {
    return ntp_limit_interval;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CLG_GetNumberOfIndices() -> libc::c_int {
    if active == 0 { return -(1 as libc::c_int) }
    return ARR_GetSize(records) as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn get_interval(mut rate: libc::c_int) -> libc::c_int {
    if rate == -(128 as libc::c_int) { return 127 as libc::c_int }
    rate +=
        if rate > 0 as libc::c_int {
            (4 as libc::c_int) / 2 as libc::c_int
        } else { (-(4 as libc::c_int)) / 2 as libc::c_int };
    return rate / -(4 as libc::c_int);
}
/* ================================================== */
unsafe extern "C" fn get_last_ago(mut x: uint32_t, mut y: uint32_t)
 -> uint32_t {
    if y == 0 as libc::c_int as libc::c_uint ||
           (x.wrapping_sub(y) as int32_t) < 0 as libc::c_int {
        return -(1 as libc::c_int) as uint32_t
    }
    return x.wrapping_sub(y) >> 4 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CLG_GetClientAccessReportByIndex(mut index:
                                                              libc::c_int,
                                                          mut report:
                                                              *mut RPT_ClientAccessByIndex_Report,
                                                          mut now:
                                                              *mut timespec)
 -> libc::c_int {
    let mut record: *mut Record = 0 as *mut Record;
    let mut now_ts: uint32_t = 0;
    if active == 0 || index < 0 as libc::c_int ||
           index as libc::c_uint >= ARR_GetSize(records) {
        return 0 as libc::c_int
    }
    record = ARR_GetElement(records, index as libc::c_uint) as *mut Record;
    if (*record).ip_addr.family as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    now_ts = get_ts_from_timespec(now);
    (*report).ip_addr = (*record).ip_addr;
    (*report).ntp_hits = (*record).ntp_hits;
    (*report).cmd_hits = (*record).cmd_hits;
    (*report).ntp_drops = (*record).ntp_drops;
    (*report).cmd_drops = (*record).cmd_drops;
    (*report).ntp_interval =
        get_interval((*record).ntp_rate as libc::c_int) as int8_t;
    (*report).cmd_interval =
        get_interval((*record).cmd_rate as libc::c_int) as int8_t;
    (*report).ntp_timeout_interval =
        get_interval((*record).ntp_timeout_rate as libc::c_int) as int8_t;
    (*report).last_ntp_hit_ago = get_last_ago(now_ts, (*record).last_ntp_hit);
    (*report).last_cmd_hit_ago = get_last_ago(now_ts, (*record).last_cmd_hit);
    return 1 as libc::c_int;
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

  This module contains facilities for logging access by clients.

  */
/* And some reporting functions, for use by chronyc. */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CLG_GetServerStatsReport(mut report:
                                                      *mut RPT_ServerStatsReport) {
    (*report).ntp_hits = total_ntp_hits;
    (*report).cmd_hits = total_cmd_hits;
    (*report).ntp_drops = total_ntp_drops;
    (*report).cmd_drops = total_cmd_drops;
    (*report).log_drops = total_record_drops;
}
