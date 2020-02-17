#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
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

  Header file for regression routine(s)

  */
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2011, 2016-2017
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

  Regression algorithms.

  */
#[no_mangle]
pub unsafe extern "C" fn RGR_WeightedRegression(mut x: *mut libc::c_double,
                                                mut y: *mut libc::c_double,
                                                mut w: *mut libc::c_double,
                                                mut n: libc::c_int,
                                                mut b0: *mut libc::c_double,
                                                mut b1: *mut libc::c_double,
                                                mut s2: *mut libc::c_double,
                                                mut sb0: *mut libc::c_double,
                                                mut sb1: *mut libc::c_double) 
 /* estimated standard deviation of
                                   slope */
 /* Could add correlation stuff later if required */
 {
    let mut P: libc::c_double = 0.;
    let mut Q: libc::c_double = 0.;
    let mut U: libc::c_double = 0.;
    let mut V: libc::c_double = 0.;
    let mut W: libc::c_double = 0.;
    let mut diff: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut ui: libc::c_double = 0.;
    let mut aa: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    if n >= 3 as libc::c_int {
    } else {
        __assert_fail(b"n >= 3\x00" as *const u8 as *const libc::c_char,
                      b"regress.c\x00" as *const u8 as *const libc::c_char,
                      67 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 113],
                                                &[libc::c_char; 113]>(b"void RGR_WeightedRegression(double *, double *, double *, int, double *, double *, double *, double *, double *)\x00")).as_ptr());
    }
    U = 0 as libc::c_int as libc::c_double;
    W = U;
    i = 0 as libc::c_int;
    while i < n {
        U += *x.offset(i as isize) / *w.offset(i as isize);
        W += 1.0f64 / *w.offset(i as isize);
        i += 1
    }
    u = U / W;
    /* Calculate statistics from data */
    V = 0.0f64;
    Q = V;
    P = Q;
    i = 0 as libc::c_int;
    while i < n {
        ui = *x.offset(i as isize) - u;
        P += *y.offset(i as isize) / *w.offset(i as isize);
        Q += *y.offset(i as isize) * ui / *w.offset(i as isize);
        V += ui * ui / *w.offset(i as isize);
        i += 1
    }
    *b1 = Q / V;
    *b0 = P / W - *b1 * u;
    *s2 = 0.0f64;
    i = 0 as libc::c_int;
    while i < n {
        diff = *y.offset(i as isize) - *b0 - *b1 * *x.offset(i as isize);
        *s2 += diff * diff / *w.offset(i as isize);
        i += 1
    }
    *s2 /= (n - 2 as libc::c_int) as libc::c_double;
    *sb1 = sqrt(*s2 / V);
    aa = u * *sb1;
    *sb0 = sqrt(*s2 / W + aa * aa);
    *s2 *= n as libc::c_double / W;
    /* Giving weighted average of variances */
}
/* Return the weighting to apply to the standard deviation to get a
   given size of confidence interval assuming a T distribution */
/* ================================================== */
/* Get the coefficient to multiply the standard deviation by, to get a
   particular size of confidence interval (assuming a t-distribution) */
#[no_mangle]
pub unsafe extern "C" fn RGR_GetTCoef(mut dof: libc::c_int)
 -> libc::c_double {
    /* Assuming now the 99.95% quantile */
    static mut coefs: [libc::c_float; 40] =
        [636.6f64 as libc::c_float, 31.6f64 as libc::c_float,
         12.92f64 as libc::c_float, 8.61f64 as libc::c_float,
         6.869f64 as libc::c_float, 5.959f64 as libc::c_float,
         5.408f64 as libc::c_float, 5.041f64 as libc::c_float,
         4.781f64 as libc::c_float, 4.587f64 as libc::c_float,
         4.437f64 as libc::c_float, 4.318f64 as libc::c_float,
         4.221f64 as libc::c_float, 4.140f64 as libc::c_float,
         4.073f64 as libc::c_float, 4.015f64 as libc::c_float,
         3.965f64 as libc::c_float, 3.922f64 as libc::c_float,
         3.883f64 as libc::c_float, 3.850f64 as libc::c_float,
         3.819f64 as libc::c_float, 3.792f64 as libc::c_float,
         3.768f64 as libc::c_float, 3.745f64 as libc::c_float,
         3.725f64 as libc::c_float, 3.707f64 as libc::c_float,
         3.690f64 as libc::c_float, 3.674f64 as libc::c_float,
         3.659f64 as libc::c_float, 3.646f64 as libc::c_float,
         3.633f64 as libc::c_float, 3.622f64 as libc::c_float,
         3.611f64 as libc::c_float, 3.601f64 as libc::c_float,
         3.591f64 as libc::c_float, 3.582f64 as libc::c_float,
         3.574f64 as libc::c_float, 3.566f64 as libc::c_float,
         3.558f64 as libc::c_float, 3.551f64 as libc::c_float];
    if dof <= 40 as libc::c_int {
        return coefs[(dof - 1 as libc::c_int) as usize] as libc::c_double
    } else {
        return 3.5f64
        /* Until I can be bothered to do something better */
    };
}
/* Return the value to apply to the variance to make an upper one-sided
   test assuming a chi-square distribution. */
/* ================================================== */
/* Get 90% quantile of chi-square distribution */
#[no_mangle]
pub unsafe extern "C" fn RGR_GetChi2Coef(mut dof: libc::c_int)
 -> libc::c_double {
    static mut coefs: [libc::c_float; 64] =
        [2.706f64 as libc::c_float, 4.605f64 as libc::c_float,
         6.251f64 as libc::c_float, 7.779f64 as libc::c_float,
         9.236f64 as libc::c_float, 10.645f64 as libc::c_float,
         12.017f64 as libc::c_float, 13.362f64 as libc::c_float,
         14.684f64 as libc::c_float, 15.987f64 as libc::c_float,
         17.275f64 as libc::c_float, 18.549f64 as libc::c_float,
         19.812f64 as libc::c_float, 21.064f64 as libc::c_float,
         22.307f64 as libc::c_float, 23.542f64 as libc::c_float,
         24.769f64 as libc::c_float, 25.989f64 as libc::c_float,
         27.204f64 as libc::c_float, 28.412f64 as libc::c_float,
         29.615f64 as libc::c_float, 30.813f64 as libc::c_float,
         32.007f64 as libc::c_float, 33.196f64 as libc::c_float,
         34.382f64 as libc::c_float, 35.563f64 as libc::c_float,
         36.741f64 as libc::c_float, 37.916f64 as libc::c_float,
         39.087f64 as libc::c_float, 40.256f64 as libc::c_float,
         41.422f64 as libc::c_float, 42.585f64 as libc::c_float,
         43.745f64 as libc::c_float, 44.903f64 as libc::c_float,
         46.059f64 as libc::c_float, 47.212f64 as libc::c_float,
         48.363f64 as libc::c_float, 49.513f64 as libc::c_float,
         50.660f64 as libc::c_float, 51.805f64 as libc::c_float,
         52.949f64 as libc::c_float, 54.090f64 as libc::c_float,
         55.230f64 as libc::c_float, 56.369f64 as libc::c_float,
         57.505f64 as libc::c_float, 58.641f64 as libc::c_float,
         59.774f64 as libc::c_float, 60.907f64 as libc::c_float,
         62.038f64 as libc::c_float, 63.167f64 as libc::c_float,
         64.295f64 as libc::c_float, 65.422f64 as libc::c_float,
         66.548f64 as libc::c_float, 67.673f64 as libc::c_float,
         68.796f64 as libc::c_float, 69.919f64 as libc::c_float,
         71.040f64 as libc::c_float, 72.160f64 as libc::c_float,
         73.279f64 as libc::c_float, 74.397f64 as libc::c_float,
         75.514f64 as libc::c_float, 76.630f64 as libc::c_float,
         77.745f64 as libc::c_float, 78.860f64 as libc::c_float];
    if dof <= 64 as libc::c_int {
        return coefs[(dof - 1 as libc::c_int) as usize] as libc::c_double
    } else {
        return 1.2f64 * dof as libc::c_double
        /* Until I can be bothered to do something better */
    };
}
/* ================================================== */
/* Critical value for number of runs of residuals with same sign.
   5% critical region for now. */
static mut critical_runs: [libc::c_char; 130] =
    [0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
     2 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
     4 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
     5 as libc::c_int as libc::c_char, 5 as libc::c_int as libc::c_char,
     5 as libc::c_int as libc::c_char, 6 as libc::c_int as libc::c_char,
     6 as libc::c_int as libc::c_char, 7 as libc::c_int as libc::c_char,
     7 as libc::c_int as libc::c_char, 7 as libc::c_int as libc::c_char,
     8 as libc::c_int as libc::c_char, 8 as libc::c_int as libc::c_char,
     9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
     9 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
     10 as libc::c_int as libc::c_char, 11 as libc::c_int as libc::c_char,
     11 as libc::c_int as libc::c_char, 11 as libc::c_int as libc::c_char,
     12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
     13 as libc::c_int as libc::c_char, 13 as libc::c_int as libc::c_char,
     14 as libc::c_int as libc::c_char, 14 as libc::c_int as libc::c_char,
     14 as libc::c_int as libc::c_char, 15 as libc::c_int as libc::c_char,
     15 as libc::c_int as libc::c_char, 16 as libc::c_int as libc::c_char,
     16 as libc::c_int as libc::c_char, 17 as libc::c_int as libc::c_char,
     17 as libc::c_int as libc::c_char, 18 as libc::c_int as libc::c_char,
     18 as libc::c_int as libc::c_char, 18 as libc::c_int as libc::c_char,
     19 as libc::c_int as libc::c_char, 19 as libc::c_int as libc::c_char,
     20 as libc::c_int as libc::c_char, 20 as libc::c_int as libc::c_char,
     21 as libc::c_int as libc::c_char, 21 as libc::c_int as libc::c_char,
     21 as libc::c_int as libc::c_char, 22 as libc::c_int as libc::c_char,
     22 as libc::c_int as libc::c_char, 23 as libc::c_int as libc::c_char,
     23 as libc::c_int as libc::c_char, 24 as libc::c_int as libc::c_char,
     24 as libc::c_int as libc::c_char, 25 as libc::c_int as libc::c_char,
     25 as libc::c_int as libc::c_char, 26 as libc::c_int as libc::c_char,
     26 as libc::c_int as libc::c_char, 26 as libc::c_int as libc::c_char,
     27 as libc::c_int as libc::c_char, 27 as libc::c_int as libc::c_char,
     28 as libc::c_int as libc::c_char, 28 as libc::c_int as libc::c_char,
     29 as libc::c_int as libc::c_char, 29 as libc::c_int as libc::c_char,
     30 as libc::c_int as libc::c_char, 30 as libc::c_int as libc::c_char,
     30 as libc::c_int as libc::c_char, 31 as libc::c_int as libc::c_char,
     31 as libc::c_int as libc::c_char, 32 as libc::c_int as libc::c_char,
     32 as libc::c_int as libc::c_char, 33 as libc::c_int as libc::c_char,
     33 as libc::c_int as libc::c_char, 34 as libc::c_int as libc::c_char,
     34 as libc::c_int as libc::c_char, 35 as libc::c_int as libc::c_char,
     35 as libc::c_int as libc::c_char, 35 as libc::c_int as libc::c_char,
     36 as libc::c_int as libc::c_char, 36 as libc::c_int as libc::c_char,
     37 as libc::c_int as libc::c_char, 37 as libc::c_int as libc::c_char,
     38 as libc::c_int as libc::c_char, 38 as libc::c_int as libc::c_char,
     39 as libc::c_int as libc::c_char, 39 as libc::c_int as libc::c_char,
     40 as libc::c_int as libc::c_char, 40 as libc::c_int as libc::c_char,
     40 as libc::c_int as libc::c_char, 41 as libc::c_int as libc::c_char,
     41 as libc::c_int as libc::c_char, 42 as libc::c_int as libc::c_char,
     42 as libc::c_int as libc::c_char, 43 as libc::c_int as libc::c_char,
     43 as libc::c_int as libc::c_char, 44 as libc::c_int as libc::c_char,
     44 as libc::c_int as libc::c_char, 45 as libc::c_int as libc::c_char,
     45 as libc::c_int as libc::c_char, 46 as libc::c_int as libc::c_char,
     46 as libc::c_int as libc::c_char, 46 as libc::c_int as libc::c_char,
     47 as libc::c_int as libc::c_char, 47 as libc::c_int as libc::c_char,
     48 as libc::c_int as libc::c_char, 48 as libc::c_int as libc::c_char,
     49 as libc::c_int as libc::c_char, 49 as libc::c_int as libc::c_char,
     50 as libc::c_int as libc::c_char, 50 as libc::c_int as libc::c_char,
     51 as libc::c_int as libc::c_char, 51 as libc::c_int as libc::c_char,
     52 as libc::c_int as libc::c_char, 52 as libc::c_int as libc::c_char,
     52 as libc::c_int as libc::c_char, 53 as libc::c_int as libc::c_char,
     53 as libc::c_int as libc::c_char, 54 as libc::c_int as libc::c_char,
     54 as libc::c_int as libc::c_char, 55 as libc::c_int as libc::c_char,
     55 as libc::c_int as libc::c_char, 56 as libc::c_int as libc::c_char];
/* ================================================== */
unsafe extern "C" fn n_runs_from_residuals(mut resid: *mut libc::c_double,
                                           mut n: libc::c_int)
 -> libc::c_int {
    let mut nruns: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    nruns = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i < n {
        if !(*resid.offset((i - 1 as libc::c_int) as isize) < 0.0f64 &&
                 *resid.offset(i as isize) < 0.0f64 ||
                 *resid.offset((i - 1 as libc::c_int) as isize) > 0.0f64 &&
                     *resid.offset(i as isize) > 0.0f64) {
            nruns += 1
        }
        i += 1
    }
    return nruns;
}
/* Return a status indicating whether there were enough points to
   carry out the regression */
/* ================================================== */
/* Return a boolean indicating whether we had enough points for
   regression */
#[no_mangle]
pub unsafe extern "C" fn RGR_FindBestRegression(mut x: *mut libc::c_double,
                                                mut y: *mut libc::c_double,
                                                mut w: *mut libc::c_double,
                                                mut n: libc::c_int,
                                                mut m: libc::c_int,
                                                mut min_samples: libc::c_int,
                                                mut b0: *mut libc::c_double,
                                                mut b1: *mut libc::c_double,
                                                mut s2: *mut libc::c_double,
                                                mut sb0: *mut libc::c_double,
                                                mut sb1: *mut libc::c_double,
                                                mut new_start:
                                                    *mut libc::c_int,
                                                mut n_runs: *mut libc::c_int,
                                                mut dof: *mut libc::c_int)
 -> libc::c_int 
 /* degrees of freedom in statistics (needed
                                   to get confidence intervals later) */
 {
    let mut P: libc::c_double = 0.; /* total */
    let mut Q: libc::c_double = 0.;
    let mut U: libc::c_double = 0.;
    let mut V: libc::c_double = 0.;
    let mut W: libc::c_double = 0.;
    let mut resid: [libc::c_double; 128] = [0.; 128];
    let mut ss: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut ui: libc::c_double = 0.;
    let mut aa: libc::c_double = 0.;
    let mut start: libc::c_int = 0;
    let mut resid_start: libc::c_int = 0;
    let mut nruns: libc::c_int = 0;
    let mut npoints: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if n <= 64 as libc::c_int && m >= 0 as libc::c_int {
    } else {
        __assert_fail(b"n <= MAX_POINTS && m >= 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"regress.c\x00" as *const u8 as *const libc::c_char,
                      242 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 143],
                                                &[libc::c_char; 143]>(b"int RGR_FindBestRegression(double *, double *, double *, int, int, int, double *, double *, double *, double *, double *, int *, int *, int *)\x00")).as_ptr());
    }
    if ((n * 2 as libc::c_int) as libc::c_ulong) <
           (::std::mem::size_of::<[libc::c_char; 130]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong) {
    } else {
        __assert_fail(b"n * REGRESS_RUNS_RATIO < sizeof (critical_runs) / sizeof (critical_runs[0])\x00"
                          as *const u8 as *const libc::c_char,
                      b"regress.c\x00" as *const u8 as *const libc::c_char,
                      243 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 143],
                                                &[libc::c_char; 143]>(b"int RGR_FindBestRegression(double *, double *, double *, int, int, int, double *, double *, double *, double *, double *, int *, int *, int *)\x00")).as_ptr());
    }
    if n < 3 as libc::c_int { return 0 as libc::c_int }
    start = 0 as libc::c_int;
    loop  {
        U = 0 as libc::c_int as libc::c_double;
        W = U;
        i = start;
        while i < n {
            U += *x.offset(i as isize) / *w.offset(i as isize);
            W += 1.0f64 / *w.offset(i as isize);
            i += 1
        }
        u = U / W;
        V = 0.0f64;
        Q = V;
        P = Q;
        i = start;
        while i < n {
            ui = *x.offset(i as isize) - u;
            P += *y.offset(i as isize) / *w.offset(i as isize);
            Q += *y.offset(i as isize) * ui / *w.offset(i as isize);
            V += ui * ui / *w.offset(i as isize);
            i += 1
        }
        b = Q / V;
        a = P / W - b * u;
        /* Get residuals also for the extra samples before start */
        resid_start = n - (n - start) * 2 as libc::c_int;
        if resid_start < -m { resid_start = -m }
        i = resid_start;
        while i < n {
            resid[(i - resid_start) as usize] =
                *y.offset(i as isize) - a - b * *x.offset(i as isize);
            i += 1
        }
        /* Count number of runs */
        nruns = n_runs_from_residuals(resid.as_mut_ptr(), n - resid_start);
        if nruns > critical_runs[(n - resid_start) as usize] as libc::c_int ||
               n - start <= 3 as libc::c_int || n - start <= min_samples {
            if start != resid_start {
                /* Ignore extra samples in returned nruns */
                nruns =
                    n_runs_from_residuals(resid.as_mut_ptr().offset((start -
                                                                         resid_start)
                                                                        as
                                                                        isize),
                                          n - start)
            }
            break ;
        } else {
            /* Try dropping one sample at a time until the runs test passes. */
            start += 1
        }
    }
    /* Work out statistics from full dataset */
    *b1 = b;
    *b0 = a;
    ss = 0.0f64;
    i = start;
    while i < n {
        ss +=
            resid[(i - resid_start) as usize] *
                resid[(i - resid_start) as usize] / *w.offset(i as isize);
        i += 1
    }
    npoints = n - start;
    ss /= (npoints - 2 as libc::c_int) as libc::c_double;
    *sb1 = sqrt(ss / V);
    aa = u * *sb1;
    *sb0 = sqrt(ss / W + aa * aa);
    *s2 = ss * npoints as libc::c_double / W;
    *new_start = start;
    *dof = npoints - 2 as libc::c_int;
    *n_runs = nruns;
    return 1 as libc::c_int;
}
/* ================================================== */
/* ================================================== */
/* Find the index'th biggest element in the array x of n elements.
   flags is an array where a 1 indicates that the corresponding entry
   in x is known to be sorted into its correct position and a 0
   indicates that the corresponding entry is not sorted.  However, if
   flags[m] = flags[n] = 1 with m<n, then x[m] must be <= x[n] and for
   all i with m<i<n, x[m] <= x[i] <= x[n].  In practice, this means
   flags[] has to be the result of a previous call to this routine
   with the same array x, and is used to remember which parts of the
   x[] array we have already sorted.

   The approach used is a cut-down quicksort, where we only bother to
   keep sorting the partition that contains the index we are after.
   The approach comes from Numerical Recipes in C (ISBN
   0-521-43108-5). */
unsafe extern "C" fn find_ordered_entry_with_flags(mut x: *mut libc::c_double,
                                                   mut n: libc::c_int,
                                                   mut index: libc::c_int,
                                                   mut flags:
                                                       *mut libc::c_char)
 -> libc::c_double {
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut temp: libc::c_double = 0.;
    let mut piv: libc::c_double = 0.;
    let mut pivind: libc::c_int = 0;
    if index >= 0 as libc::c_int {
    } else {
        __assert_fail(b"index >= 0\x00" as *const u8 as *const libc::c_char,
                      b"regress.c\x00" as *const u8 as *const libc::c_char,
                      350 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 65],
                                                &[libc::c_char; 65]>(b"double find_ordered_entry_with_flags(double *, int, int, char *)\x00")).as_ptr());
    }
    /* If this bit of the array is already sorted, simple! */
    if *flags.offset(index as isize) != 0 { return *x.offset(index as isize) }
    /* Find subrange to look at */
    v = index; /* New value */
    u = v; /* Pivot now in correct place */
    while u > 0 as libc::c_int && *flags.offset(u as isize) == 0 { u -= 1 }
    if *flags.offset(u as isize) != 0 { u += 1 }
    while v < n - 1 as libc::c_int && *flags.offset(v as isize) == 0 {
        v += 1
    }
    if *flags.offset(v as isize) != 0 { v -= 1 }
    loop  {
        if v - u < 2 as libc::c_int {
            if *x.offset(v as isize) < *x.offset(u as isize) {
                temp = *x.offset(v as isize);
                *x.offset(v as isize) = *x.offset(u as isize);
                *x.offset(u as isize) = temp
            }
            let ref mut fresh0 = *flags.offset(u as isize);
            *fresh0 = 1 as libc::c_int as libc::c_char;
            *flags.offset(v as isize) = *fresh0;
            return *x.offset(index as isize)
        } else {
            pivind = u + v >> 1 as libc::c_int;
            temp = *x.offset(u as isize);
            *x.offset(u as isize) = *x.offset(pivind as isize);
            *x.offset(pivind as isize) = temp;
            piv = *x.offset(u as isize);
            l = u + 1 as libc::c_int;
            r = v;
            loop  {
                while l < v && *x.offset(l as isize) < piv { l += 1 }
                while *x.offset(r as isize) > piv { r -= 1 }
                if r <= l { break ; }
                temp = *x.offset(l as isize);
                *x.offset(l as isize) = *x.offset(r as isize);
                *x.offset(r as isize) = temp;
                l += 1;
                r -= 1
            }
            temp = *x.offset(u as isize);
            *x.offset(u as isize) = *x.offset(r as isize);
            *x.offset(r as isize) = temp;
            *flags.offset(r as isize) = 1 as libc::c_int as libc::c_char;
            if index == r {
                return *x.offset(r as isize)
            } else {
                if index < r {
                    v = r - 1 as libc::c_int
                } else if index > r { u = l }
            }
        }
    };
}
/* ================================================== */
/* ================================================== */
/* Find the median entry of an array x[] with n elements. */
unsafe extern "C" fn find_median(mut x: *mut libc::c_double,
                                 mut n: libc::c_int) -> libc::c_double {
    let mut k: libc::c_int = 0;
    let mut flags: [libc::c_char; 64] = [0; 64];
    memset(flags.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (n as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong));
    k = n >> 1 as libc::c_int;
    if n & 1 as libc::c_int != 0 {
        return find_ordered_entry_with_flags(x, n, k, flags.as_mut_ptr())
    } else {
        return 0.5f64 *
                   (find_ordered_entry_with_flags(x, n, k, flags.as_mut_ptr())
                        +
                        find_ordered_entry_with_flags(x, n,
                                                      k - 1 as libc::c_int,
                                                      flags.as_mut_ptr()))
    };
}
/* Return the median value from an array */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn RGR_FindMedian(mut x: *mut libc::c_double,
                                        mut n: libc::c_int)
 -> libc::c_double {
    let mut tmp: [libc::c_double; 64] = [0.; 64];
    if n > 0 as libc::c_int && n <= 64 as libc::c_int {
    } else {
        __assert_fail(b"n > 0 && n <= MAX_POINTS\x00" as *const u8 as
                          *const libc::c_char,
                      b"regress.c\x00" as *const u8 as *const libc::c_char,
                      439 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"double RGR_FindMedian(double *, int)\x00")).as_ptr());
    }
    memcpy(tmp.as_mut_ptr() as *mut libc::c_void, x as *const libc::c_void,
           (n as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_double>()
                                                as libc::c_ulong));
    return find_median(tmp.as_mut_ptr(), n);
}
/* ================================================== */
/* This function evaluates the equation

   \sum_{i=0}^{n-1} x_i sign(y_i - a - b x_i)

   and chooses the value of a that minimises the absolute value of the
   result.  (See pp703-704 of Numerical Recipes in C). */
unsafe extern "C" fn eval_robust_residual(mut x: *mut libc::c_double,
                                          mut y: *mut libc::c_double,
                                          mut n: libc::c_int,
                                          mut b: libc::c_double,
                                          mut aa: *mut libc::c_double,
                                          mut rr: *mut libc::c_double) 
 /* Corresponding value of equation */
 {
    let mut i: libc::c_int = 0;
    let mut a: libc::c_double = 0.;
    let mut res: libc::c_double = 0.;
    let mut del: libc::c_double = 0.;
    let mut d: [libc::c_double; 64] = [0.; 64];
    i = 0 as libc::c_int;
    while i < n {
        d[i as usize] = *y.offset(i as isize) - b * *x.offset(i as isize);
        i += 1
    }
    a = find_median(d.as_mut_ptr(), n);
    res = 0.0f64;
    i = 0 as libc::c_int;
    while i < n {
        del = *y.offset(i as isize) - a - b * *x.offset(i as isize);
        if del > 0.0f64 {
            res += *x.offset(i as isize)
        } else if del < 0.0f64 { res -= *x.offset(i as isize) }
        i += 1
    }
    *aa = a;
    *rr = res;
}
/* ================================================== */
/* This routine performs a 'robust' regression, i.e. one which has low
   susceptibility to outliers amongst the data.  If one thinks of a
   normal (least squares) linear regression in 2D being analogous to
   the arithmetic mean in 1D, this algorithm in 2D is roughly
   analogous to the median in 1D.  This algorithm seems to work quite
   well until the number of outliers is approximately half the number
   of data points.

   The return value is a status indicating whether there were enough
   data points to run the routine or not. */
#[no_mangle]
pub unsafe extern "C" fn RGR_FindBestRobustRegression(mut x:
                                                          *mut libc::c_double,
                                                      mut y:
                                                          *mut libc::c_double,
                                                      mut n: libc::c_int,
                                                      mut tol: libc::c_double,
                                                      mut b0:
                                                          *mut libc::c_double,
                                                      mut b1:
                                                          *mut libc::c_double,
                                                      mut n_runs:
                                                          *mut libc::c_int,
                                                      mut best_start:
                                                          *mut libc::c_int)
 -> libc::c_int 
 /* The best starting index */
 {
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut n_points: libc::c_int = 0;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut P: libc::c_double = 0.;
    let mut U: libc::c_double = 0.;
    let mut V: libc::c_double = 0.;
    let mut W: libc::c_double = 0.;
    let mut X: libc::c_double = 0.;
    let mut resid: libc::c_double = 0.;
    let mut resids: [libc::c_double; 64] = [0.; 64];
    let mut blo: libc::c_double = 0.;
    let mut bhi: libc::c_double = 0.;
    let mut bmid: libc::c_double = 0.;
    let mut rlo: libc::c_double = 0.;
    let mut rhi: libc::c_double = 0.;
    let mut rmid: libc::c_double = 0.;
    let mut s2: libc::c_double = 0.;
    let mut sb: libc::c_double = 0.;
    let mut incr: libc::c_double = 0.;
    let mut mx: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut my: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut nruns: libc::c_int = 0 as libc::c_int;
    if n <= 64 as libc::c_int {
    } else {
        __assert_fail(b"n <= MAX_POINTS\x00" as *const u8 as
                          *const libc::c_char,
                      b"regress.c\x00" as *const u8 as *const libc::c_char,
                      525 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 100],
                                                &[libc::c_char; 100]>(b"int RGR_FindBestRobustRegression(double *, double *, int, double, double *, double *, int *, int *)\x00")).as_ptr());
    }
    if n < 2 as libc::c_int {
        return 0 as libc::c_int
    } else {
        if n == 2 as libc::c_int {
            /* Just a straight line fit (we need this for the manual mode) */
            *b1 =
                (*y.offset(1 as libc::c_int as isize) -
                     *y.offset(0 as libc::c_int as isize)) /
                    (*x.offset(1 as libc::c_int as isize) -
                         *x.offset(0 as libc::c_int as isize));
            *b0 =
                *y.offset(0 as libc::c_int as isize) -
                    *b1 * *x.offset(0 as libc::c_int as isize);
            *n_runs = 0 as libc::c_int;
            *best_start = 0 as libc::c_int;
            return 1 as libc::c_int
        }
    }
    /* else at least 3 points, apply normal algorithm */
    start = 0 as libc::c_int;
    loop 
         /* Loop to strip oldest points that cause the regression residuals
     to fail the number of runs test */
         {
        n_points = n - start;
        /* Use standard least squares regression to get starting estimate */
        U = 0.0f64;
        P = U;
        i = start;
        while i < n {
            P += *y.offset(i as isize);
            U += *x.offset(i as isize);
            i += 1
        }
        W = n_points as libc::c_double;
        my = P / W;
        mx = U / W;
        V = 0.0f64;
        X = V;
        i = start;
        while i < n {
            dy = *y.offset(i as isize) - my;
            dx = *x.offset(i as isize) - mx;
            X += dy * dx;
            V += dx * dx;
            i += 1
        }
        b = X / V;
        a = my - b * mx;
        s2 = 0.0f64;
        i = start;
        while i < n {
            resid = *y.offset(i as isize) - a - b * *x.offset(i as isize);
            s2 += resid * resid;
            i += 1
        }
        /* Need to expand range of b to get a root in the interval.
       Estimate standard deviation of b and expand range about b based
       on that. */
        sb =
            sqrt(s2 * W /
                     V); /* fn vals have same sign or one is zero,
                                   i.e. root not in interval (rlo, rhi). */
        incr = if sb > tol { sb } else { tol };
        loop  {
            incr *= 2.0f64;
            /* Give up if the interval is too large */
            if incr > 100.0f64 { return 0 as libc::c_int }
            blo = b - incr;
            bhi = b + incr;
            /* We don't want 'a' yet */
            eval_robust_residual(x.offset(start as isize),
                                 y.offset(start as isize), n_points, blo,
                                 &mut a, &mut rlo);
            eval_robust_residual(x.offset(start as isize),
                                 y.offset(start as isize), n_points, bhi,
                                 &mut a, &mut rhi);
            if !(rlo * rhi >= 0.0f64) { break ; }
        }
        loop 
             /* OK, so the root for b lies in (blo, bhi). Start bisecting */
             {
            bmid = 0.5f64 * (blo + bhi);
            if !(blo < bmid && bmid < bhi) { break ; }
            eval_robust_residual(x.offset(start as isize),
                                 y.offset(start as isize), n_points, bmid,
                                 &mut a, &mut rmid);
            if rmid == 0.0f64 { break ; }
            if rmid * rlo > 0.0f64 {
                blo = bmid;
                rlo = rmid
            } else if rmid * rhi > 0.0f64 {
                bhi = bmid;
                rhi = rmid
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"regress.c\x00" as *const u8 as
                                  *const libc::c_char,
                              616 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 100],
                                                        &[libc::c_char; 100]>(b"int RGR_FindBestRobustRegression(double *, double *, int, double, double *, double *, int *, int *)\x00")).as_ptr());
            }
            if !(bhi - blo > tol) { break ; }
        }
        *b0 = a;
        *b1 = bmid;
        /* Number of runs test, but not if we're already down to the
       minimum number of points */
        if n_points == 3 as libc::c_int { break ; }
        i = start;
        while i < n {
            resids[i as usize] =
                *y.offset(i as isize) - a - bmid * *x.offset(i as isize);
            i += 1
        }
        nruns =
            n_runs_from_residuals(resids.as_mut_ptr().offset(start as isize),
                                  n_points);
        if nruns > critical_runs[n_points as usize] as libc::c_int { break ; }
        start += 1
    }
    *n_runs = nruns;
    *best_start = start;
    return 1 as libc::c_int;
}
/* ================================================== */
/* This routine performs linear regression with two independent variables.
   It returns non-zero status if there were enough data points and there
   was a solution. */
#[no_mangle]
pub unsafe extern "C" fn RGR_MultipleRegress(mut x1: *mut libc::c_double,
                                             mut x2: *mut libc::c_double,
                                             mut y: *mut libc::c_double,
                                             mut n: libc::c_int,
                                             mut b2: *mut libc::c_double)
 -> libc::c_int 
 /* estimated second slope */
                                /* other values are not needed yet */
 {
    let mut Sx1: libc::c_double = 0.;
    let mut Sx2: libc::c_double = 0.;
    let mut Sx1x1: libc::c_double = 0.;
    let mut Sx1x2: libc::c_double = 0.;
    let mut Sx2x2: libc::c_double = 0.;
    let mut Sx1y: libc::c_double = 0.;
    let mut Sx2y: libc::c_double = 0.;
    let mut Sy: libc::c_double = 0.;
    let mut U: libc::c_double = 0.;
    let mut V: libc::c_double = 0.;
    let mut V1: libc::c_double = 0.;
    let mut V2: libc::c_double = 0.;
    let mut V3: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    if n < 4 as libc::c_int { return 0 as libc::c_int }
    Sy = 0.0f64;
    Sx2y = Sy;
    Sx1y = Sx2y;
    Sx2x2 = Sx1y;
    Sx1x2 = Sx2x2;
    Sx1x1 = Sx1x2;
    Sx2 = Sx1x1;
    Sx1 = Sx2;
    i = 0 as libc::c_int;
    while i < n {
        Sx1 += *x1.offset(i as isize);
        Sx2 += *x2.offset(i as isize);
        Sx1x1 += *x1.offset(i as isize) * *x1.offset(i as isize);
        Sx1x2 += *x1.offset(i as isize) * *x2.offset(i as isize);
        Sx2x2 += *x2.offset(i as isize) * *x2.offset(i as isize);
        Sx1y += *x1.offset(i as isize) * *y.offset(i as isize);
        Sx2y += *x2.offset(i as isize) * *y.offset(i as isize);
        Sy += *y.offset(i as isize);
        i += 1
    }
    U =
        n as libc::c_double * (Sx1x2 * Sx1y - Sx1x1 * Sx2y) + Sx1 * Sx1 * Sx2y
            - Sx1 * Sx2 * Sx1y + Sy * (Sx2 * Sx1x1 - Sx1 * Sx1x2);
    V1 = n as libc::c_double * (Sx1x2 * Sx1x2 - Sx1x1 * Sx2x2);
    V2 = Sx1 * Sx1 * Sx2x2 + Sx2 * Sx2 * Sx1x1;
    V3 = -2.0f64 * Sx1 * Sx2 * Sx1x2;
    V = V1 + V2 + V3;
    /* Check if there is a (numerically stable) solution */
    if fabs(V) * 1.0e10f64 <= -V1 + V2 + fabs(V3) { return 0 as libc::c_int }
    *b2 = U / V;
    return 1 as libc::c_int;
}
