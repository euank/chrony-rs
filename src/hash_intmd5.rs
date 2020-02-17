#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5_CTX {
    pub i: [UINT4; 2],
    pub buf: [UINT4; 4],
    pub in_0: [libc::c_uchar; 64],
    pub digest: [libc::c_uchar; 16],
}
/*
 ***********************************************************************
 ** md5.h -- header file for implementation of MD5                    **
 ** RSA Data Security, Inc. MD5 Message-Digest Algorithm              **
 ** Created: 2/17/90 RLR                                              **
 ** Revised: 12/27/90 SRD,AJ,BSK,JT Reference C version               **
 ** Revised (for MD5): RLR 4/27/91                                    **
 ***********************************************************************
 */
/*
 ***********************************************************************
 ** Copyright (C) 1990, RSA Data Security, Inc. All rights reserved.  **
 **                                                                   **
 ** License to copy and use this software is granted provided that    **
 ** it is identified as the "RSA Data Security, Inc. MD5 Message-     **
 ** Digest Algorithm" in all material mentioning or referencing this  **
 ** software or this function.                                        **
 **                                                                   **
 ** License is also granted to make and use derivative works          **
 ** provided that such works are identified as "derived from the RSA  **
 ** Data Security, Inc. MD5 Message-Digest Algorithm" in all          **
 ** material mentioning or referencing the derived work.              **
 **                                                                   **
 ** RSA Data Security, Inc. makes no representations concerning       **
 ** either the merchantability of this software or the suitability    **
 ** of this software for any particular purpose.  It is provided "as  **
 ** is" without express or implied warranty of any kind.              **
 **                                                                   **
 ** These notices must be retained in any copies of any part of this  **
 ** documentation and/or software.                                    **
 ***********************************************************************
 */
/* typedef a 32-bit type */
pub type UINT4 = uint32_t;
/*
 ***********************************************************************
 ** md5.c -- the source code for MD5 routines                         **
 ** RSA Data Security, Inc. MD5 Message-Digest Algorithm              **
 ** Created: 2/17/90 RLR                                              **
 ** Revised: 1/91 SRD,AJ,BSK,JT Reference C Version                   **
 ** Revised (for MD5): RLR 4/27/91                                    **
 **   -- G modified to have y&~z instead of y&z                       **
 **   -- FF, GG, HH modified to add in last register done             **
 **   -- Access pattern: round 2 works mod 5, round 3 works mod 3     **
 **   -- distinct additive constant for each step                     **
 **   -- round 4 added, working mod 7                                 **
 ***********************************************************************
 */
/*
 ***********************************************************************
 ** Copyright (C) 1990, RSA Data Security, Inc. All rights reserved.  **
 **                                                                   **
 ** License to copy and use this software is granted provided that    **
 ** it is identified as the "RSA Data Security, Inc. MD5 Message-     **
 ** Digest Algorithm" in all material mentioning or referencing this  **
 ** software or this function.                                        **
 **                                                                   **
 ** License is also granted to make and use derivative works          **
 ** provided that such works are identified as "derived from the RSA  **
 ** Data Security, Inc. MD5 Message-Digest Algorithm" in all          **
 ** material mentioning or referencing the derived work.              **
 **                                                                   **
 ** RSA Data Security, Inc. makes no representations concerning       **
 ** either the merchantability of this software or the suitability    **
 ** of this software for any particular purpose.  It is provided "as  **
 ** is" without express or implied warranty of any kind.              **
 **                                                                   **
 ** These notices must be retained in any copies of any part of this  **
 ** documentation and/or software.                                    **
 ***********************************************************************
 */
/*
 ***********************************************************************
 **  Message-digest routines:                                         **
 **  To form the message digest for a message M                       **
 **    (1) Initialize a context buffer mdContext using MD5Init        **
 **    (2) Call MD5Update on mdContext and M                          **
 **    (3) Call MD5Final on mdContext                                 **
 **  The message digest is now in mdContext->digest[0...15]           **
 ***********************************************************************
 */
/* forward declaration */
static mut PADDING: [libc::c_uchar; 64] =
    [0x80 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar];
/* F, G, H and I are basic MD5 functions */
/* ROTATE_LEFT rotates x left n bits */
/* FF, GG, HH, and II transformations for rounds 1, 2, 3, and 4 */
/* Rotation is separate from addition to prevent recomputation */
/* The routine MD5Init initializes the message-digest context
   mdContext. All fields are set to zero.
 */
/* Data structure for MD5 (Message-Digest) computation */
/* number of _bits_ handled mod 2^64 */
/* scratch buffer */
/* input buffer */
/* actual digest after MD5Final call */
#[no_mangle]
pub unsafe extern "C" fn MD5Init(mut mdContext: *mut MD5_CTX) {
    (*mdContext).i[1 as libc::c_int as usize] = 0 as libc::c_int as UINT4;
    (*mdContext).i[0 as libc::c_int as usize] =
        (*mdContext).i[1 as libc::c_int as usize];
    /* Load magic initialization constants.
   */
    (*mdContext).buf[0 as libc::c_int as usize] =
        0x67452301 as libc::c_int as UINT4;
    (*mdContext).buf[1 as libc::c_int as usize] = 0xefcdab89 as libc::c_uint;
    (*mdContext).buf[2 as libc::c_int as usize] = 0x98badcfe as libc::c_uint;
    (*mdContext).buf[3 as libc::c_int as usize] =
        0x10325476 as libc::c_int as UINT4;
}
/* The routine MD5Update updates the message-digest context to
   account for the presence of each of the characters inBuf[0..inLen-1]
   in the message whose digest is being computed.
 */
#[no_mangle]
pub unsafe extern "C" fn MD5Update(mut mdContext: *mut MD5_CTX,
                                   mut inBuf: *const libc::c_uchar,
                                   mut inLen: libc::c_uint) {
    let mut in_0: [UINT4; 16] = [0; 16];
    let mut mdi: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut ii: libc::c_uint = 0;
    /* compute number of bytes mod 64 */
    mdi =
        ((*mdContext).i[0 as libc::c_int as usize] >> 3 as libc::c_int &
             0x3f as libc::c_int as libc::c_uint) as libc::c_int;
    /* update number of bits */
    if (*mdContext).i[0 as libc::c_int as
                          usize].wrapping_add(inLen << 3 as libc::c_int) <
           (*mdContext).i[0 as libc::c_int as usize] {
        (*mdContext).i[1 as libc::c_int as usize] =
            (*mdContext).i[1 as libc::c_int as usize].wrapping_add(1)
    }
    (*mdContext).i[0 as libc::c_int as usize] =
        ((*mdContext).i[0 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(inLen << 3 as libc::c_int) as UINT4 as
            UINT4;
    (*mdContext).i[1 as libc::c_int as usize] =
        ((*mdContext).i[1 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(inLen >> 29 as libc::c_int) as UINT4
            as UINT4;
    loop  {
        let fresh0 = inLen;
        inLen = inLen.wrapping_sub(1);
        if !(fresh0 != 0) { break ; }
        /* add new character to buffer, increment mdi */
        let fresh1 = inBuf;
        inBuf = inBuf.offset(1);
        let fresh2 = mdi;
        mdi = mdi + 1;
        (*mdContext).in_0[fresh2 as usize] = *fresh1;
        /* transform if necessary */
        if mdi == 0x40 as libc::c_int {
            i = 0 as libc::c_int as libc::c_uint;
            ii = 0 as libc::c_int as libc::c_uint;
            while i < 16 as libc::c_int as libc::c_uint {
                in_0[i as usize] =
                    ((*mdContext).in_0[ii.wrapping_add(3 as libc::c_int as
                                                           libc::c_uint) as
                                           usize] as UINT4) <<
                        24 as libc::c_int |
                        ((*mdContext).in_0[ii.wrapping_add(2 as libc::c_int as
                                                               libc::c_uint)
                                               as usize] as UINT4) <<
                            16 as libc::c_int |
                        ((*mdContext).in_0[ii.wrapping_add(1 as libc::c_int as
                                                               libc::c_uint)
                                               as usize] as UINT4) <<
                            8 as libc::c_int |
                        (*mdContext).in_0[ii as usize] as UINT4;
                i = i.wrapping_add(1);
                ii = ii.wrapping_add(4 as libc::c_int as libc::c_uint)
            }
            Transform((*mdContext).buf.as_mut_ptr(), in_0.as_mut_ptr());
            mdi = 0 as libc::c_int
        }
    };
}
/* The routine MD5Final terminates the message-digest computation and
   ends with the desired message digest in mdContext->digest[0...15].
 */
#[no_mangle]
pub unsafe extern "C" fn MD5Final(mut mdContext: *mut MD5_CTX) {
    let mut in_0: [UINT4; 16] = [0; 16];
    let mut mdi: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut ii: libc::c_uint = 0;
    let mut padLen: libc::c_uint = 0;
    /* save number of bits */
    in_0[14 as libc::c_int as usize] =
        (*mdContext).i[0 as libc::c_int as usize];
    in_0[15 as libc::c_int as usize] =
        (*mdContext).i[1 as libc::c_int as usize];
    /* compute number of bytes mod 64 */
    mdi =
        ((*mdContext).i[0 as libc::c_int as usize] >> 3 as libc::c_int &
             0x3f as libc::c_int as libc::c_uint) as libc::c_int;
    /* pad out to 56 mod 64 */
    padLen =
        if mdi < 56 as libc::c_int {
            (56 as libc::c_int) - mdi
        } else { (120 as libc::c_int) - mdi } as libc::c_uint;
    MD5Update(mdContext, PADDING.as_ptr(), padLen);
    /* append length in bits and transform */
    i = 0 as libc::c_int as libc::c_uint;
    ii = 0 as libc::c_int as libc::c_uint;
    while i < 14 as libc::c_int as libc::c_uint {
        in_0[i as usize] =
            ((*mdContext).in_0[ii.wrapping_add(3 as libc::c_int as
                                                   libc::c_uint) as usize] as
                 UINT4) << 24 as libc::c_int |
                ((*mdContext).in_0[ii.wrapping_add(2 as libc::c_int as
                                                       libc::c_uint) as usize]
                     as UINT4) << 16 as libc::c_int |
                ((*mdContext).in_0[ii.wrapping_add(1 as libc::c_int as
                                                       libc::c_uint) as usize]
                     as UINT4) << 8 as libc::c_int |
                (*mdContext).in_0[ii as usize] as UINT4;
        i = i.wrapping_add(1);
        ii = ii.wrapping_add(4 as libc::c_int as libc::c_uint)
    }
    Transform((*mdContext).buf.as_mut_ptr(), in_0.as_mut_ptr());
    /* store buffer in digest */
    i = 0 as libc::c_int as libc::c_uint;
    ii = 0 as libc::c_int as libc::c_uint;
    while i < 4 as libc::c_int as libc::c_uint {
        (*mdContext).digest[ii as usize] =
            ((*mdContext).buf[i as usize] &
                 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        (*mdContext).digest[ii.wrapping_add(1 as libc::c_int as libc::c_uint)
                                as usize] =
            ((*mdContext).buf[i as usize] >> 8 as libc::c_int &
                 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        (*mdContext).digest[ii.wrapping_add(2 as libc::c_int as libc::c_uint)
                                as usize] =
            ((*mdContext).buf[i as usize] >> 16 as libc::c_int &
                 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        (*mdContext).digest[ii.wrapping_add(3 as libc::c_int as libc::c_uint)
                                as usize] =
            ((*mdContext).buf[i as usize] >> 24 as libc::c_int &
                 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        i = i.wrapping_add(1);
        ii = ii.wrapping_add(4 as libc::c_int as libc::c_uint)
    };
}
/* Basic MD5 step. Transforms buf based on in.
 */
unsafe extern "C" fn Transform(mut buf: *mut UINT4, mut in_0: *mut UINT4) {
    let mut a: UINT4 = *buf.offset(0 as libc::c_int as isize);
    let mut b: UINT4 = *buf.offset(1 as libc::c_int as isize);
    let mut c: UINT4 = *buf.offset(2 as libc::c_int as isize);
    let mut d: UINT4 = *buf.offset(3 as libc::c_int as isize);
    /* Round 1 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & c |
                                             !b &
                                                 d).wrapping_add(*in_0.offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0xd76aa478
                                                                                                           as
                                                                                                           libc::c_uint))
            as UINT4 as UINT4;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 1 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & b |
                                             !a &
                                                 c).wrapping_add(*in_0.offset(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0xe8c7b756
                                                                                                           as
                                                                                                           libc::c_uint))
            as UINT4 as UINT4;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 2 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & a |
                                             !d &
                                                 b).wrapping_add(*in_0.offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0x242070db
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           UINT4))
            as UINT4 as UINT4;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 3 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & d |
                                             !c &
                                                 a).wrapping_add(*in_0.offset(3
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0xc1bdceee
                                                                                                           as
                                                                                                           libc::c_uint))
            as UINT4 as UINT4;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 4 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & c |
                                             !b &
                                                 d).wrapping_add(*in_0.offset(4
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0xf57c0faf
                                                                                                           as
                                                                                                           libc::c_uint))
            as UINT4 as UINT4;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 5 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & b |
                                             !a &
                                                 c).wrapping_add(*in_0.offset(5
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0x4787c62a
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           UINT4))
            as UINT4 as UINT4;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 6 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & a |
                                             !d &
                                                 b).wrapping_add(*in_0.offset(6
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0xa8304613
                                                                                                           as
                                                                                                           libc::c_uint))
            as UINT4 as UINT4;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 7 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & d |
                                             !c &
                                                 a).wrapping_add(*in_0.offset(7
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0xfd469501
                                                                                                           as
                                                                                                           libc::c_uint))
            as UINT4 as UINT4;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 8 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & c |
                                             !b &
                                                 d).wrapping_add(*in_0.offset(8
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0x698098d8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           UINT4))
            as UINT4 as UINT4;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 9 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & b |
                                             !a &
                                                 c).wrapping_add(*in_0.offset(9
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0x8b44f7af
                                                                                                           as
                                                                                                           libc::c_uint))
            as UINT4 as UINT4;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 10 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & a |
                                             !d &
                                                 b).wrapping_add(*in_0.offset(10
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0xffff5bb1
                                                                                                           as
                                                                                                           libc::c_uint))
            as UINT4 as UINT4;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 11 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & d |
                                             !c &
                                                 a).wrapping_add(*in_0.offset(11
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0x895cd7be
                                                                                                           as
                                                                                                           libc::c_uint))
            as UINT4 as UINT4;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 12 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & c |
                                             !b &
                                                 d).wrapping_add(*in_0.offset(12
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0x6b901122
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           UINT4))
            as UINT4 as UINT4;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 13 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & b |
                                             !a &
                                                 c).wrapping_add(*in_0.offset(13
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0xfd987193
                                                                                                           as
                                                                                                           libc::c_uint))
            as UINT4 as UINT4;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 14 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & a |
                                             !d &
                                                 b).wrapping_add(*in_0.offset(14
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0xa679438e
                                                                                                           as
                                                                                                           libc::c_uint))
            as UINT4 as UINT4;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 15 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & d |
                                             !c &
                                                 a).wrapping_add(*in_0.offset(15
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(0x49b40821
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           UINT4))
            as UINT4 as UINT4;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 16 */
    /* Round 2 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & d |
                                             c &
                                                 !d).wrapping_add(*in_0.offset(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0xf61e2562
                                                                                                            as
                                                                                                            libc::c_uint))
            as UINT4 as UINT4;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 17 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & c |
                                             b &
                                                 !c).wrapping_add(*in_0.offset(6
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0xc040b340
                                                                                                            as
                                                                                                            libc::c_uint))
            as UINT4 as UINT4;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 18 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & b |
                                             a &
                                                 !b).wrapping_add(*in_0.offset(11
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0x265e5a51
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            UINT4))
            as UINT4 as UINT4;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 19 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & a |
                                             d &
                                                 !a).wrapping_add(*in_0.offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0xe9b6c7aa
                                                                                                            as
                                                                                                            libc::c_uint))
            as UINT4 as UINT4;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 20 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & d |
                                             c &
                                                 !d).wrapping_add(*in_0.offset(5
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0xd62f105d
                                                                                                            as
                                                                                                            libc::c_uint))
            as UINT4 as UINT4;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 21 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & c |
                                             b &
                                                 !c).wrapping_add(*in_0.offset(10
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0x2441453
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            UINT4))
            as UINT4 as UINT4;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 22 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & b |
                                             a &
                                                 !b).wrapping_add(*in_0.offset(15
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0xd8a1e681
                                                                                                            as
                                                                                                            libc::c_uint))
            as UINT4 as UINT4;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 23 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & a |
                                             d &
                                                 !a).wrapping_add(*in_0.offset(4
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0xe7d3fbc8
                                                                                                            as
                                                                                                            libc::c_uint))
            as UINT4 as UINT4;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 24 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & d |
                                             c &
                                                 !d).wrapping_add(*in_0.offset(9
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0x21e1cde6
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            UINT4))
            as UINT4 as UINT4;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 25 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & c |
                                             b &
                                                 !c).wrapping_add(*in_0.offset(14
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0xc33707d6
                                                                                                            as
                                                                                                            libc::c_uint))
            as UINT4 as UINT4;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 26 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & b |
                                             a &
                                                 !b).wrapping_add(*in_0.offset(3
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0xf4d50d87
                                                                                                            as
                                                                                                            libc::c_uint))
            as UINT4 as UINT4;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 27 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & a |
                                             d &
                                                 !a).wrapping_add(*in_0.offset(8
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0x455a14ed
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            UINT4))
            as UINT4 as UINT4;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 28 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & d |
                                             c &
                                                 !d).wrapping_add(*in_0.offset(13
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0xa9e3e905
                                                                                                            as
                                                                                                            libc::c_uint))
            as UINT4 as UINT4;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 29 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & c |
                                             b &
                                                 !c).wrapping_add(*in_0.offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0xfcefa3f8
                                                                                                            as
                                                                                                            libc::c_uint))
            as UINT4 as UINT4;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 30 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & b |
                                             a &
                                                 !b).wrapping_add(*in_0.offset(7
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0x676f02d9
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            UINT4))
            as UINT4 as UINT4;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 31 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & a |
                                             d &
                                                 !a).wrapping_add(*in_0.offset(12
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(0x8d2a4c8a
                                                                                                            as
                                                                                                            libc::c_uint))
            as UINT4 as UINT4;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 32 */
    /* Round 3 */
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(5 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xfffa3942
                                                                                                       as
                                                                                                       libc::c_uint))
            as UINT4 as UINT4;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 33 */
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(8 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0x8771f681
                                                                                                       as
                                                                                                       libc::c_uint))
            as UINT4 as UINT4;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 34 */
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(11
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0x6d9d6122
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       UINT4))
            as UINT4 as UINT4;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 35 */
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(14
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xfde5380c
                                                                                                       as
                                                                                                       libc::c_uint))
            as UINT4 as UINT4;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 36 */
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(1 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xa4beea44
                                                                                                       as
                                                                                                       libc::c_uint))
            as UINT4 as UINT4;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 37 */
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(4 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0x4bdecfa9
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       UINT4))
            as UINT4 as UINT4;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 38 */
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(7 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xf6bb4b60
                                                                                                       as
                                                                                                       libc::c_uint))
            as UINT4 as UINT4;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 39 */
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(10
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xbebfbc70
                                                                                                       as
                                                                                                       libc::c_uint))
            as UINT4 as UINT4;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 40 */
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(13
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0x289b7ec6
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       UINT4))
            as UINT4 as UINT4;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 41 */
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xeaa127fa
                                                                                                       as
                                                                                                       libc::c_uint))
            as UINT4 as UINT4;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 42 */
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(3 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xd4ef3085
                                                                                                       as
                                                                                                       libc::c_uint))
            as UINT4 as UINT4;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 43 */
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(6 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0x4881d05
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       UINT4))
            as UINT4 as UINT4;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 44 */
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(9 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xd9d4d039
                                                                                                       as
                                                                                                       libc::c_uint))
            as UINT4 as UINT4;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 45 */
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(12
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xe6db99e5
                                                                                                       as
                                                                                                       libc::c_uint))
            as UINT4 as UINT4;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 46 */
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(15
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0x1fa27cf8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       UINT4))
            as UINT4 as UINT4;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 47 */
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(2 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(0xc4ac5665
                                                                                                       as
                                                                                                       libc::c_uint))
            as UINT4 as UINT4;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 48 */
    /* Round 4 */
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xf4292244
                                                                                                              as
                                                                                                              libc::c_uint))
            as UINT4 as UINT4;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 49 */
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(7
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x432aff97
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              UINT4))
            as UINT4 as UINT4;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 50 */
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(14
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xab9423a7
                                                                                                              as
                                                                                                              libc::c_uint))
            as UINT4 as UINT4;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 51 */
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(5
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xfc93a039
                                                                                                              as
                                                                                                              libc::c_uint))
            as UINT4 as UINT4;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 52 */
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(12
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x655b59c3
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              UINT4))
            as UINT4 as UINT4;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 53 */
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(3
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x8f0ccc92
                                                                                                              as
                                                                                                              libc::c_uint))
            as UINT4 as UINT4;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 54 */
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(10
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xffeff47d
                                                                                                              as
                                                                                                              libc::c_uint))
            as UINT4 as UINT4;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 55 */
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x85845dd1
                                                                                                              as
                                                                                                              libc::c_uint))
            as UINT4 as UINT4;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 56 */
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x6fa87e4f
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              UINT4))
            as UINT4 as UINT4;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 57 */
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(15
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xfe2ce6e0
                                                                                                              as
                                                                                                              libc::c_uint))
            as UINT4 as UINT4;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 58 */
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(6
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xa3014314
                                                                                                              as
                                                                                                              libc::c_uint))
            as UINT4 as UINT4;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 59 */
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(13
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x4e0811a1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              UINT4))
            as UINT4 as UINT4;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 60 */
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(4
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xf7537e82
                                                                                                              as
                                                                                                              libc::c_uint))
            as UINT4 as UINT4;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    /* 61 */
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(11
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xbd3af235
                                                                                                              as
                                                                                                              libc::c_uint))
            as UINT4 as UINT4;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    /* 62 */
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(2
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0x2ad7d2bb
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              UINT4))
            as UINT4 as UINT4;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    /* 63 */
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(9
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(0xeb86d391
                                                                                                              as
                                                                                                              libc::c_uint))
            as UINT4 as UINT4;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    /* 64 */
    let ref mut fresh3 = *buf.offset(0 as libc::c_int as isize);
    *fresh3 = (*fresh3 as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    let ref mut fresh4 = *buf.offset(1 as libc::c_int as isize);
    *fresh4 = (*fresh4 as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    let ref mut fresh5 = *buf.offset(2 as libc::c_int as isize);
    *fresh5 = (*fresh5 as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    let ref mut fresh6 = *buf.offset(3 as libc::c_int as isize);
    *fresh6 = (*fresh6 as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
}
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2012
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

  Routines implementing crypto hashing using internal MD5 implementation.

  */
static mut ctx: MD5_CTX =
    MD5_CTX{i: [0; 2], buf: [0; 4], in_0: [0; 64], digest: [0; 16],};
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2012
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

  Header file for crypto hashing.

  */
/* length of hash values produced by SHA512 */
#[no_mangle]
pub unsafe extern "C" fn HSH_GetHashId(mut name: *const libc::c_char)
 -> libc::c_int {
    /* only MD5 is supported */
    if strcmp(name, b"MD5\x00" as *const u8 as *const libc::c_char) != 0 {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HSH_Hash(mut id: libc::c_int,
                                  mut in1: *const libc::c_uchar,
                                  mut in1_len: libc::c_uint,
                                  mut in2: *const libc::c_uchar,
                                  mut in2_len: libc::c_uint,
                                  mut out: *mut libc::c_uchar,
                                  mut out_len: libc::c_uint) -> libc::c_uint {
    MD5Init(&mut ctx);
    MD5Update(&mut ctx, in1, in1_len);
    if !in2.is_null() { MD5Update(&mut ctx, in2, in2_len); }
    MD5Final(&mut ctx);
    out_len =
        if out_len < 16 as libc::c_int as libc::c_uint {
            out_len
        } else { 16 as libc::c_int as libc::c_uint };
    memcpy(out as *mut libc::c_void,
           ctx.digest.as_mut_ptr() as *const libc::c_void,
           out_len as libc::c_ulong);
    return out_len;
}
#[no_mangle]
pub unsafe extern "C" fn HSH_Finalise() { }
