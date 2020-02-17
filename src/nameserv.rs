#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn getaddrinfo(__name: *const libc::c_char,
                   __service: *const libc::c_char, __req: *const addrinfo,
                   __pai: *mut *mut addrinfo) -> libc::c_int;
    #[no_mangle]
    fn freeaddrinfo(__ai: *mut addrinfo);
    #[no_mangle]
    fn getnameinfo(__sa: *const sockaddr, __salen: socklen_t,
                   __host: *mut libc::c_char, __hostlen: socklen_t,
                   __serv: *mut libc::c_char, __servlen: socklen_t,
                   __flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __res_init() -> libc::c_int;
    #[no_mangle]
    fn SCK_IPSockAddrToSockaddr(ip_sa: *mut IPSockAddr, sa: *mut sockaddr,
                                sa_length: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn UTI_IPToString(ip: *mut IPAddr) -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
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
pub type DNS_Status = libc::c_uint;
pub const DNS_Failure: DNS_Status = 2;
pub const DNS_TryAgain: DNS_Status = 1;
pub const DNS_Success: DNS_Status = 0;
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2009-2011
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

  Functions to do name to IP address conversion

  */
/* ================================================== */
static mut address_family: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn DNS_SetAddressFamily(mut family: libc::c_int) {
    address_family = family;
}
/* Resolve names only to selected address family */
/* Maximum number of addresses returned by DNS_Name2IPAddress */
#[no_mangle]
pub unsafe extern "C" fn DNS_Name2IPAddress(mut name: *const libc::c_char,
                                            mut ip_addrs: *mut IPAddr,
                                            mut max_addrs: libc::c_int)
 -> DNS_Status {
    let mut hints: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut res: *mut addrinfo = 0 as *mut addrinfo;
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    let mut i: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    max_addrs =
        if max_addrs < 16 as libc::c_int {
            max_addrs
        } else { 16 as libc::c_int };
    memset(&mut hints as *mut addrinfo as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    match address_family {
        1 => { hints.ai_family = 2 as libc::c_int }
        2 => { hints.ai_family = 10 as libc::c_int }
        _ => { hints.ai_family = 0 as libc::c_int }
    }
    hints.ai_socktype = SOCK_DGRAM as libc::c_int;
    result =
        getaddrinfo(name, 0 as *const libc::c_char, &mut hints, &mut res);
    if result != 0 { return DNS_TryAgain }
    ai = res;
    i = 0 as libc::c_int;
    while i < max_addrs && !ai.is_null() {
        match (*ai).ai_family {
            2 => {
                if !(address_family != 0 as libc::c_int &&
                         address_family != 1 as libc::c_int) {
                    (*ip_addrs.offset(i as isize)).family =
                        1 as libc::c_int as uint16_t;
                    (*ip_addrs.offset(i as isize)).addr.in4 =
                        ntohl((*((*ai).ai_addr as
                                     *mut sockaddr_in)).sin_addr.s_addr);
                    i += 1
                }
            }
            10 => {
                if !(address_family != 0 as libc::c_int &&
                         address_family != 2 as libc::c_int) {
                    (*ip_addrs.offset(i as isize)).family =
                        2 as libc::c_int as uint16_t;
                    memcpy(&mut (*ip_addrs.offset(i as isize)).addr.in6 as
                               *mut [uint8_t; 16] as *mut libc::c_void,
                           &mut (*((*ai).ai_addr as
                                       *mut sockaddr_in6)).sin6_addr.__in6_u.__u6_addr8
                               as *mut [uint8_t; 16] as *const libc::c_void,
                           ::std::mem::size_of::<[uint8_t; 16]>() as
                               libc::c_ulong);
                    i += 1
                }
            }
            _ => { }
        }
        ai = (*ai).ai_next
    }
    while i < max_addrs {
        (*ip_addrs.offset(i as isize)).family = 0 as libc::c_int as uint16_t;
        i += 1
    }
    freeaddrinfo(res);
    return if max_addrs == 0 ||
                  (*ip_addrs.offset(0 as libc::c_int as isize)).family as
                      libc::c_int != 0 as libc::c_int {
               DNS_Success as libc::c_int
           } else { DNS_Failure as libc::c_int } as DNS_Status;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn DNS_IPAddress2Name(mut ip_addr: *mut IPAddr,
                                            mut name: *mut libc::c_char,
                                            mut len: libc::c_int)
 -> libc::c_int {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut in6: sockaddr_in6 =
        sockaddr_in6{sin6_family: 0,
                     sin6_port: 0,
                     sin6_flowinfo: 0,
                     sin6_addr:
                         in6_addr{__in6_u:
                                      C2RustUnnamed{__u6_addr8: [0; 16],},},
                     sin6_scope_id: 0,};
    let mut ip_saddr: IPSockAddr =
        IPSockAddr{ip_addr:
                       IPAddr{addr: C2RustUnnamed_0{in4: 0,},
                              family: 0,
                              _pad: 0,},
                   port: 0,};
    let mut slen: socklen_t = 0;
    let mut hbuf: [libc::c_char; 1025] = [0; 1025];
    ip_saddr.ip_addr = *ip_addr;
    ip_saddr.port = 0 as libc::c_int as uint16_t;
    slen =
        SCK_IPSockAddrToSockaddr(&mut ip_saddr,
                                 &mut in6 as *mut sockaddr_in6 as
                                     *mut sockaddr,
                                 ::std::mem::size_of::<sockaddr_in6>() as
                                     libc::c_ulong as libc::c_int) as
            socklen_t;
    if getnameinfo(&mut in6 as *mut sockaddr_in6 as *mut sockaddr, slen,
                   hbuf.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 1025]>() as
                       libc::c_ulong as socklen_t, 0 as *mut libc::c_char,
                   0 as libc::c_int as socklen_t, 0 as libc::c_int) == 0 {
        result = hbuf.as_mut_ptr()
    }
    if result.is_null() { result = UTI_IPToString(ip_addr) }
    if snprintf(name, len as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char, result) >= len
       {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn DNS_Reload() { __res_init(); }
/* ================================================== */
