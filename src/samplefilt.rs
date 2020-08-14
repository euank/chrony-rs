#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn LCL_GetSysPrecisionAsQuantum() -> libc::c_double;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char, _: ...);
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

     Header file for regression routine(s)

     */
    #[no_mangle]
    fn RGR_WeightedRegression(
        x: *mut libc::c_double,
        y: *mut libc::c_double,
        w: *mut libc::c_double,
        n: libc::c_int,
        b0: *mut libc::c_double,
        b1: *mut libc::c_double,
        s2: *mut libc::c_double,
        sb0: *mut libc::c_double,
        sb1: *mut libc::c_double,
    );
    /* Return the value to apply to the variance to make an upper one-sided
    test assuming a chi-square distribution. */
    #[no_mangle]
    fn RGR_GetChi2Coef(dof: libc::c_int) -> libc::c_double;
    /* Returns -1 if a comes earlier than b, 0 if a is the same time as b,
    and +1 if a comes after b */
    #[no_mangle]
    fn UTI_CompareTimespecs(a: *mut timespec, b: *mut timespec) -> libc::c_int;
    /* Calculate result = a - b */
    /* Calculate result = a - b and return as a double */
    /* Add a double increment to a timespec to get a new one. 'start' is
    the starting time, 'end' is the result that we return.  This is
    safe to use if start and end are the same */
    /* Calculate the average and difference (as a double) of two timespecs */
    /* Calculate result = a - b + c */
    /* Convert a timespec into a temporary string, largely for diagnostic
    display */
    #[no_mangle]
    fn UTI_TimespecToString(ts: *mut timespec) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_AdjustTimespec(
        old_ts: *mut timespec,
        when: *mut timespec,
        new_ts: *mut timespec,
        delta_time: *mut libc::c_double,
        dfreq: libc::c_double,
        doffset: libc::c_double,
    );
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec) -> libc::c_double;
    #[no_mangle]
    fn UTI_AddDoubleToTimespec(start: *mut timespec, increment: libc::c_double, end: *mut timespec);
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
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
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
* Copyright (C) Miroslav Lichvar  2009-2011, 2014, 2016, 2018
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

 Routines implementing a median sample filter.

 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPF_Instance_Record {
    pub min_samples: libc::c_int,
    pub max_samples: libc::c_int,
    pub index: libc::c_int,
    pub used: libc::c_int,
    pub last: libc::c_int,
    pub avg_var_n: libc::c_int,
    pub avg_var: libc::c_double,
    pub max_var: libc::c_double,
    pub combine_ratio: libc::c_double,
    pub samples: *mut NTP_Sample,
    pub selected: *mut libc::c_int,
    pub x_data: *mut libc::c_double,
    pub y_data: *mut libc::c_double,
    pub w_data: *mut libc::c_double,
}
pub type SPF_Instance = *mut SPF_Instance_Record;
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SPF_CreateInstance(
    mut min_samples: libc::c_int,
    mut max_samples: libc::c_int,
    mut max_dispersion: libc::c_double,
    mut combine_ratio: libc::c_double,
) -> SPF_Instance {
    let mut filter: SPF_Instance = 0 as *mut SPF_Instance_Record;
    filter = Malloc(::std::mem::size_of::<SPF_Instance_Record>() as libc::c_ulong)
        as *mut SPF_Instance_Record;
    min_samples = if 1 as libc::c_int
        > (if min_samples < 256 as libc::c_int {
            min_samples
        } else {
            256 as libc::c_int
        }) {
        1 as libc::c_int
    } else if min_samples < 256 as libc::c_int {
        min_samples
    } else {
        256 as libc::c_int
    };
    max_samples = if 1 as libc::c_int
        > (if max_samples < 256 as libc::c_int {
            max_samples
        } else {
            256 as libc::c_int
        }) {
        1 as libc::c_int
    } else if max_samples < 256 as libc::c_int {
        max_samples
    } else {
        256 as libc::c_int
    };
    max_samples = if min_samples > max_samples {
        min_samples
    } else {
        max_samples
    };
    combine_ratio = if 0.0f64
        > (if combine_ratio < 1.0f64 {
            combine_ratio
        } else {
            1.0f64
        }) {
        0.0f64
    } else if combine_ratio < 1.0f64 {
        combine_ratio
    } else {
        1.0f64
    };
    (*filter).min_samples = min_samples;
    (*filter).max_samples = max_samples;
    (*filter).index = -(1 as libc::c_int);
    (*filter).used = 0 as libc::c_int;
    (*filter).last = -(1 as libc::c_int);
    /* Set the first estimate to the system precision */
    (*filter).avg_var_n = 0 as libc::c_int;
    (*filter).avg_var = LCL_GetSysPrecisionAsQuantum() * LCL_GetSysPrecisionAsQuantum();
    (*filter).max_var = max_dispersion * max_dispersion;
    (*filter).combine_ratio = combine_ratio;
    (*filter).samples = Malloc2(
        (*filter).max_samples as size_t,
        ::std::mem::size_of::<NTP_Sample>() as libc::c_ulong,
    ) as *mut NTP_Sample;
    (*filter).selected = Malloc2(
        (*filter).max_samples as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    (*filter).x_data = Malloc2(
        (*filter).max_samples as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    (*filter).y_data = Malloc2(
        (*filter).max_samples as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    (*filter).w_data = Malloc2(
        (*filter).max_samples as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    return filter;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SPF_DestroyInstance(mut filter: SPF_Instance) {
    free((*filter).samples as *mut libc::c_void);
    free((*filter).selected as *mut libc::c_void);
    free((*filter).x_data as *mut libc::c_void);
    free((*filter).y_data as *mut libc::c_void);
    free((*filter).w_data as *mut libc::c_void);
    free(filter as *mut libc::c_void);
}
/* ================================================== */
/* Check that samples times are strictly increasing */
unsafe extern "C" fn check_sample(
    mut filter: SPF_Instance,
    mut sample: *mut NTP_Sample,
) -> libc::c_int {
    if (*filter).used <= 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if UTI_CompareTimespecs(
        &mut (*(*filter).samples.offset((*filter).last as isize)).time,
        &mut (*sample).time,
    ) >= 0 as libc::c_int
    {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"filter non-increasing sample time %s\x00" as *const u8 as *const libc::c_char,
                UTI_TimespecToString(&mut (*sample).time),
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SPF_AccumulateSample(
    mut filter: SPF_Instance,
    mut sample: *mut NTP_Sample,
) -> libc::c_int {
    if check_sample(filter, sample) == 0 {
        return 0 as libc::c_int;
    }
    (*filter).index += 1;
    (*filter).index %= (*filter).max_samples;
    (*filter).last = (*filter).index;
    if (*filter).used < (*filter).max_samples {
        (*filter).used += 1
    }
    *(*filter).samples.offset((*filter).index as isize) = *sample;
    if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
        LOG_Message(
            LOGS_DEBUG,
            b"filter sample %d t=%s offset=%.9f peer_disp=%.9f\x00" as *const u8
                as *const libc::c_char,
            (*filter).index,
            UTI_TimespecToString(&mut (*sample).time),
            (*sample).offset,
            (*sample).peer_dispersion,
        );
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SPF_GetLastSample(
    mut filter: SPF_Instance,
    mut sample: *mut NTP_Sample,
) -> libc::c_int {
    if (*filter).last < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    *sample = *(*filter).samples.offset((*filter).last as isize);
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SPF_GetNumberOfSamples(mut filter: SPF_Instance) -> libc::c_int {
    return (*filter).used;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SPF_GetAvgSampleDispersion(mut filter: SPF_Instance) -> libc::c_double {
    return sqrt((*filter).avg_var);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SPF_DropSamples(mut filter: SPF_Instance) {
    (*filter).index = -(1 as libc::c_int);
    (*filter).used = 0 as libc::c_int;
}
/* ================================================== */
static mut tmp_sort_samples: *const NTP_Sample = 0 as *const NTP_Sample;
unsafe extern "C" fn compare_samples(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut s1: *const NTP_Sample = 0 as *const NTP_Sample;
    let mut s2: *const NTP_Sample = 0 as *const NTP_Sample;
    s1 = &*tmp_sort_samples.offset(*(a as *mut libc::c_int) as isize) as *const NTP_Sample;
    s2 = &*tmp_sort_samples.offset(*(b as *mut libc::c_int) as isize) as *const NTP_Sample;
    if (*s1).offset < (*s2).offset {
        return -(1 as libc::c_int);
    } else {
        if (*s1).offset > (*s2).offset {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn select_samples(mut filter: SPF_Instance) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut o: libc::c_int = 0;
    let mut from: libc::c_int = 0;
    let mut to: libc::c_int = 0;
    let mut selected: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut min_dispersion: libc::c_double = 0.;
    if (*filter).used < (*filter).min_samples {
        return 0 as libc::c_int;
    }
    selected = (*filter).selected;
    /* With 4 or more samples, select those that have peer dispersion smaller
    than 1.5x of the minimum dispersion */
    if (*filter).used > 4 as libc::c_int {
        i = 1 as libc::c_int;
        min_dispersion = (*(*filter).samples.offset(0 as libc::c_int as isize)).peer_dispersion;
        while i < (*filter).used {
            if min_dispersion > (*(*filter).samples.offset(i as isize)).peer_dispersion {
                min_dispersion = (*(*filter).samples.offset(i as isize)).peer_dispersion
            }
            i += 1
        }
        j = 0 as libc::c_int;
        i = j;
        while i < (*filter).used {
            if (*(*filter).samples.offset(i as isize)).peer_dispersion <= 1.5f64 * min_dispersion {
                let fresh0 = j;
                j = j + 1;
                *selected.offset(fresh0 as isize) = i
            }
            i += 1
        }
    } else {
        j = 0 as libc::c_int
    }
    if j < 4 as libc::c_int {
        /* Select all samples */
        j = 0 as libc::c_int;
        while j < (*filter).used {
            *selected.offset(j as isize) = j;
            j += 1
        }
    }
    /* And sort their indices by offset */
    tmp_sort_samples = (*filter).samples;
    qsort(
        selected as *mut libc::c_void,
        j as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        Some(
            compare_samples
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    /* Select samples closest to the median */
    if j > 2 as libc::c_int {
        from = (j as libc::c_double * (1.0f64 - (*filter).combine_ratio) / 2.0f64) as libc::c_int;
        from = if 1 as libc::c_int
            > (if from < (j - 1 as libc::c_int) / 2 as libc::c_int {
                from
            } else {
                (j - 1 as libc::c_int) / 2 as libc::c_int
            }) {
            1 as libc::c_int
        } else if from < (j - 1 as libc::c_int) / 2 as libc::c_int {
            from
        } else {
            (j - 1 as libc::c_int) / 2 as libc::c_int
        }
    } else {
        from = 0 as libc::c_int
    }
    to = j - from;
    /* Mark unused samples and sort the rest by their time */
    o = (*filter).used - (*filter).index - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < from {
        *selected.offset(i as isize) = -(1 as libc::c_int);
        i += 1
    }
    while i < to {
        *selected.offset(i as isize) = (*selected.offset(i as isize) + o) % (*filter).used;
        i += 1
    }
    while i < (*filter).used {
        *selected.offset(i as isize) = -(1 as libc::c_int);
        i += 1
    }
    i = from;
    while i < to {
        j = *selected.offset(i as isize);
        *selected.offset(i as isize) = -(1 as libc::c_int);
        while j != -(1 as libc::c_int) && *selected.offset(j as isize) != j {
            k = *selected.offset(j as isize);
            *selected.offset(j as isize) = j;
            j = k
        }
        i += 1
    }
    j = 0 as libc::c_int;
    i = j;
    k = -(1 as libc::c_int);
    while i < (*filter).used {
        if *selected.offset(i as isize) != -(1 as libc::c_int) {
            let fresh1 = j;
            j = j + 1;
            *selected.offset(fresh1 as isize) =
                (*selected.offset(i as isize) + (*filter).used - o) % (*filter).used
        }
        i += 1
    }
    if j > 0 as libc::c_int && j <= (*filter).max_samples {
    } else {
        __assert_fail(
            b"j > 0 && j <= filter->max_samples\x00" as *const u8 as *const libc::c_char,
            b"samplefilt.c\x00" as *const u8 as *const libc::c_char,
            275 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"int select_samples(SPF_Instance)\x00",
            ))
            .as_ptr(),
        );
    }
    return j;
}
/* ================================================== */
unsafe extern "C" fn combine_selected_samples(
    mut filter: SPF_Instance,
    mut n: libc::c_int,
    mut result: *mut NTP_Sample,
) -> libc::c_int {
    let mut mean_peer_dispersion: libc::c_double = 0.;
    let mut mean_root_dispersion: libc::c_double = 0.;
    let mut mean_peer_delay: libc::c_double = 0.;
    let mut mean_root_delay: libc::c_double = 0.;
    let mut mean_x: libc::c_double = 0.;
    let mut mean_y: libc::c_double = 0.;
    let mut disp: libc::c_double = 0.;
    let mut var: libc::c_double = 0.;
    let mut prev_avg_var: libc::c_double = 0.;
    let mut sample: *mut NTP_Sample = 0 as *mut NTP_Sample;
    let mut last_sample: *mut NTP_Sample = 0 as *mut NTP_Sample;
    let mut i: libc::c_int = 0;
    let mut dof: libc::c_int = 0;
    last_sample = &mut *(*filter)
        .samples
        .offset(*(*filter).selected.offset((n - 1 as libc::c_int) as isize) as isize)
        as *mut NTP_Sample;
    /* Prepare data */
    i = 0 as libc::c_int;
    while i < n {
        sample = &mut *(*filter)
            .samples
            .offset(*(*filter).selected.offset(i as isize) as isize)
            as *mut NTP_Sample;
        *(*filter).x_data.offset(i as isize) =
            UTI_DiffTimespecsToDouble(&mut (*sample).time, &mut (*last_sample).time);
        *(*filter).y_data.offset(i as isize) = (*sample).offset;
        *(*filter).w_data.offset(i as isize) = (*sample).peer_dispersion;
        i += 1
    }
    /* Calculate mean offset and interval since the last sample */
    i = 0 as libc::c_int;
    mean_y = 0.0f64;
    mean_x = mean_y;
    while i < n {
        mean_x += *(*filter).x_data.offset(i as isize);
        mean_y += *(*filter).y_data.offset(i as isize);
        i += 1
    }
    mean_x /= n as libc::c_double;
    mean_y /= n as libc::c_double;
    if n >= 4 as libc::c_int {
        let mut b0: libc::c_double = 0.;
        let mut b1: libc::c_double = 0.;
        let mut s2: libc::c_double = 0.;
        let mut sb0: libc::c_double = 0.;
        let mut sb1: libc::c_double = 0.;
        /* Set y axis to the mean sample time */
        i = 0 as libc::c_int;
        while i < n {
            *(*filter).x_data.offset(i as isize) -= mean_x;
            i += 1
        }
        /* Make a linear fit and use the estimated standard deviation of the
        intercept as dispersion */
        RGR_WeightedRegression(
            (*filter).x_data,
            (*filter).y_data,
            (*filter).w_data,
            n,
            &mut b0,
            &mut b1,
            &mut s2,
            &mut sb0,
            &mut sb1,
        );
        var = s2;
        disp = sb0;
        dof = n - 2 as libc::c_int
    } else if n >= 2 as libc::c_int {
        i = 0 as libc::c_int;
        disp = 0.0f64;
        while i < n {
            disp += (*(*filter).y_data.offset(i as isize) - mean_y)
                * (*(*filter).y_data.offset(i as isize) - mean_y);
            i += 1
        }
        var = disp / (n - 1 as libc::c_int) as libc::c_double;
        disp = sqrt(var);
        dof = n - 1 as libc::c_int
    } else {
        var = (*filter).avg_var;
        disp = sqrt(var);
        dof = 1 as libc::c_int
    }
    /* Avoid working with zero dispersion */
    if var < 1e-20f64 {
        var = 1e-20f64;
        disp = sqrt(var)
    }
    /* Drop the sample if the variance is larger than the maximum */
    if (*filter).max_var > 0.0f64 && var > (*filter).max_var {
        if 0 as libc::c_int != 0 && log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(
                LOGS_DEBUG,
                b"filter dispersion too large disp=%.9f max=%.9f\x00" as *const u8
                    as *const libc::c_char,
                sqrt(var),
                sqrt((*filter).max_var),
            );
        }
        return 0 as libc::c_int;
    }
    prev_avg_var = (*filter).avg_var;
    /* Update the exponential moving average of the variance */
    if (*filter).avg_var_n > 50 as libc::c_int {
        (*filter).avg_var +=
            dof as libc::c_double / (dof as libc::c_double + 50.0f64) * (var - (*filter).avg_var)
    } else {
        (*filter).avg_var = ((*filter).avg_var * (*filter).avg_var_n as libc::c_double
            + var * dof as libc::c_double)
            / (dof + (*filter).avg_var_n) as libc::c_double;
        if (*filter).avg_var_n == 0 as libc::c_int {
            prev_avg_var = (*filter).avg_var
        }
        (*filter).avg_var_n += dof
    }
    /* Use the long-term average of variance instead of the estimated value
    unless it is significantly smaller in order to reduce the noise in
    sourcestats weights */
    if var * dof as libc::c_double / RGR_GetChi2Coef(dof) < prev_avg_var {
        disp = sqrt((*filter).avg_var) * disp / sqrt(var)
    }
    mean_root_delay = 0.0f64;
    mean_peer_delay = mean_root_delay;
    mean_root_dispersion = mean_peer_delay;
    mean_peer_dispersion = mean_root_dispersion;
    i = 0 as libc::c_int;
    while i < n {
        sample = &mut *(*filter)
            .samples
            .offset(*(*filter).selected.offset(i as isize) as isize)
            as *mut NTP_Sample;
        mean_peer_dispersion += (*sample).peer_dispersion;
        mean_root_dispersion += (*sample).root_dispersion;
        mean_peer_delay += (*sample).peer_delay;
        mean_root_delay += (*sample).root_delay;
        i += 1
    }
    mean_peer_dispersion /= n as libc::c_double;
    mean_root_dispersion /= n as libc::c_double;
    mean_peer_delay /= n as libc::c_double;
    mean_root_delay /= n as libc::c_double;
    UTI_AddDoubleToTimespec(&mut (*last_sample).time, mean_x, &mut (*result).time);
    (*result).offset = mean_y;
    (*result).peer_dispersion = if disp > mean_peer_dispersion {
        disp
    } else {
        mean_peer_dispersion
    };
    (*result).root_dispersion = if disp > mean_root_dispersion {
        disp
    } else {
        mean_root_dispersion
    };
    (*result).peer_delay = mean_peer_delay;
    (*result).root_delay = mean_root_delay;
    (*result).stratum = (*last_sample).stratum;
    (*result).leap = (*last_sample).leap;
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SPF_GetFilteredSample(
    mut filter: SPF_Instance,
    mut sample: *mut NTP_Sample,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = select_samples(filter);
    if n < 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if combine_selected_samples(filter, n, sample) == 0 {
        return 0 as libc::c_int;
    }
    SPF_DropSamples(filter);
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SPF_SlewSamples(
    mut filter: SPF_Instance,
    mut when: *mut timespec,
    mut dfreq: libc::c_double,
    mut doffset: libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut delta_time: libc::c_double = 0.;
    if (*filter).last < 0 as libc::c_int {
        return;
    }
    /* Always slew the last sample as it may be returned even if no new
    samples were accumulated */
    if (*filter).used > 0 as libc::c_int {
        first = 0 as libc::c_int;
        last = (*filter).used - 1 as libc::c_int
    } else {
        last = (*filter).last;
        first = last
    }
    i = first;
    while i <= last {
        UTI_AdjustTimespec(
            &mut (*(*filter).samples.offset(i as isize)).time,
            when,
            &mut (*(*filter).samples.offset(i as isize)).time,
            &mut delta_time,
            dfreq,
            doffset,
        );
        (*(*filter).samples.offset(i as isize)).offset -= delta_time;
        i += 1
    }
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
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SPF_AddDispersion(
    mut filter: SPF_Instance,
    mut dispersion: libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*filter).used {
        (*(*filter).samples.offset(i as isize)).peer_dispersion += dispersion;
        (*(*filter).samples.offset(i as isize)).root_dispersion += dispersion;
        i += 1
    }
}
