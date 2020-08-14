#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
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
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
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
pub struct ADF_AuthTableInst {
    pub base4: TableNode,
    pub base6: TableNode,
}
pub type TableNode = _TableNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _TableNode {
    pub state: State,
    pub extended: *mut _TableNode,
}
pub type State = libc::c_uint;
pub const AS_PARENT: State = 2;
pub const ALLOW: State = 1;
pub const DENY: State = 0;
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
pub type ADF_Status = libc::c_uint;
pub const ADF_BADSUBNET: ADF_Status = 1;
pub const ADF_SUCCESS: ADF_Status = 0;
/* IPv6 node */
/* ================================================== */
unsafe extern "C" fn split_ip6(mut ip: *mut IPAddr, mut dst: *mut uint32_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *dst.offset(i as isize) = ((*ip).addr.in6
            [(i * 4 as libc::c_int + 0 as libc::c_int) as usize]
            as uint32_t)
            << 24 as libc::c_int
            | (((*ip).addr.in6[(i * 4 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int)
                << 16 as libc::c_int) as libc::c_uint
            | (((*ip).addr.in6[(i * 4 as libc::c_int + 2 as libc::c_int) as usize] as libc::c_int)
                << 8 as libc::c_int) as libc::c_uint
            | (*ip).addr.in6[(i * 4 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_uint;
        i += 1
    }
}
/* ================================================== */
#[inline]
unsafe extern "C" fn get_subnet(mut addr: *mut uint32_t, mut where_0: libc::c_uint) -> uint32_t {
    let mut off: libc::c_int = 0;
    off = where_0.wrapping_div(32 as libc::c_int as libc::c_uint) as libc::c_int;
    where_0 = where_0.wrapping_rem(32 as libc::c_int as libc::c_uint);
    return ((*addr.offset(off as isize)
        >> ((32 as libc::c_int - 4 as libc::c_int) as libc::c_uint).wrapping_sub(where_0))
        as libc::c_ulong
        & ((1 as libc::c_ulong) << 4 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as uint32_t;
}
/* Create a new table.  The default rule is deny for everything */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn ADF_CreateTable() -> ADF_AuthTable {
    let mut result: ADF_AuthTable = 0 as *mut ADF_AuthTableInst;
    result = Malloc(::std::mem::size_of::<ADF_AuthTableInst>() as libc::c_ulong)
        as *mut ADF_AuthTableInst;
    /* Default is that nothing is allowed */
    (*result).base4.state = DENY;
    (*result).base4.extended = 0 as *mut _TableNode;
    (*result).base6.state = DENY;
    (*result).base6.extended = 0 as *mut _TableNode;
    return result;
}
/* ================================================== */
/* This function deletes all definitions of child nodes, in effect
pruning a whole subnet definition back to a single parent
record. */
unsafe extern "C" fn close_node(mut node: *mut TableNode) {
    let mut i: libc::c_int = 0;
    let mut child_node: *mut TableNode = 0 as *mut TableNode;
    if !(*node).extended.is_null() {
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (1 as libc::c_ulong) << 4 as libc::c_int {
            child_node = &mut *(*node).extended.offset(i as isize) as *mut _TableNode;
            close_node(child_node);
            i += 1
        }
        free((*node).extended as *mut libc::c_void);
        (*node).extended = 0 as *mut _TableNode
    };
}
/* ================================================== */
/* Allocate the extension field in a node, and set all the children's
states to default to that of the node being extended */
unsafe extern "C" fn open_node(mut node: *mut TableNode) {
    let mut i: libc::c_int = 0;
    let mut child_node: *mut TableNode = 0 as *mut TableNode;
    if (*node).extended.is_null() {
        (*node).extended = Malloc2(
            (1 as libc::c_ulong) << 4 as libc::c_int,
            ::std::mem::size_of::<_TableNode>() as libc::c_ulong,
        ) as *mut _TableNode;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (1 as libc::c_ulong) << 4 as libc::c_int {
            child_node = &mut *(*node).extended.offset(i as isize) as *mut _TableNode;
            (*child_node).state = AS_PARENT;
            (*child_node).extended = 0 as *mut _TableNode;
            i += 1
        }
    };
}
/* ================================================== */
unsafe extern "C" fn set_subnet(
    mut start_node: *mut TableNode,
    mut ip: *mut uint32_t,
    mut ip_len: libc::c_int,
    mut subnet_bits: libc::c_int,
    mut new_state: State,
    mut delete_children: libc::c_int,
) -> ADF_Status {
    let mut bits_to_go: libc::c_int = 0; /* Have to set multiple entries */
    let mut bits_consumed: libc::c_int = 0;
    let mut subnet: uint32_t = 0;
    let mut node: *mut TableNode = 0 as *mut TableNode;
    bits_consumed = 0 as libc::c_int;
    bits_to_go = subnet_bits;
    node = start_node;
    if subnet_bits < 0 as libc::c_int || subnet_bits > 32 as libc::c_int * ip_len {
        return ADF_BADSUBNET;
    } else {
        if bits_to_go & 4 as libc::c_int - 1 as libc::c_int == 0 as libc::c_int {
            while bits_to_go > 0 as libc::c_int {
                subnet = get_subnet(ip, bits_consumed as libc::c_uint);
                if (*node).extended.is_null() {
                    open_node(node);
                }
                node = &mut *(*node).extended.offset(subnet as isize) as *mut _TableNode;
                bits_to_go -= 4 as libc::c_int;
                bits_consumed += 4 as libc::c_int
            }
            if delete_children != 0 {
                close_node(node);
            }
            (*node).state = new_state
        } else {
            let mut N: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            let mut this_node: *mut TableNode = 0 as *mut TableNode;
            while bits_to_go >= 4 as libc::c_int {
                subnet = get_subnet(ip, bits_consumed as libc::c_uint);
                if (*node).extended.is_null() {
                    open_node(node);
                }
                node = &mut *(*node).extended.offset(subnet as isize) as *mut _TableNode;
                bits_to_go -= 4 as libc::c_int;
                bits_consumed += 4 as libc::c_int
            }
            /* How many subnet entries to set : 1->8, 2->4, 3->2 */
            N = (1 as libc::c_int) << 4 as libc::c_int - bits_to_go;
            subnet = get_subnet(ip, bits_consumed as libc::c_uint)
                & !(N - 1 as libc::c_int) as libc::c_uint;
            if subnet.wrapping_add(N as libc::c_uint) as libc::c_ulong
                <= (1 as libc::c_ulong) << 4 as libc::c_int
            {
            } else {
                __assert_fail(
                    b"subnet + N <= TABLE_SIZE\x00" as *const u8 as *const libc::c_char,
                    b"addrfilt.c\x00" as *const u8 as *const libc::c_char,
                    204 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                        b"ADF_Status set_subnet(TableNode *, uint32_t *, int, int, State, int)\x00",
                    ))
                    .as_ptr(),
                );
            }
            if (*node).extended.is_null() {
                open_node(node);
            }
            i = subnet as libc::c_int;
            j = 0 as libc::c_int;
            while j < N {
                this_node = &mut *(*node).extended.offset(i as isize) as *mut _TableNode;
                if delete_children != 0 {
                    close_node(this_node);
                }
                (*this_node).state = new_state;
                i += 1;
                j += 1
            }
        }
        return ADF_SUCCESS;
    };
}
/* ================================================== */
unsafe extern "C" fn set_subnet_(
    mut table: ADF_AuthTable,
    mut ip_addr: *mut IPAddr,
    mut subnet_bits: libc::c_int,
    mut new_state: State,
    mut delete_children: libc::c_int,
) -> ADF_Status {
    let mut ip6: [uint32_t; 4] = [0; 4];
    match (*ip_addr).family as libc::c_int {
        1 => {
            return set_subnet(
                &mut (*table).base4,
                &mut (*ip_addr).addr.in4,
                1 as libc::c_int,
                subnet_bits,
                new_state,
                delete_children,
            )
        }
        2 => {
            split_ip6(ip_addr, ip6.as_mut_ptr());
            return set_subnet(
                &mut (*table).base6,
                ip6.as_mut_ptr(),
                4 as libc::c_int,
                subnet_bits,
                new_state,
                delete_children,
            );
        }
        0 => {
            /* Apply to both, subnet_bits has to be 0 */
            if subnet_bits != 0 as libc::c_int {
                return ADF_BADSUBNET;
            }
            memset(
                ip6.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong,
            );
            if set_subnet(
                &mut (*table).base4,
                ip6.as_mut_ptr(),
                1 as libc::c_int,
                0 as libc::c_int,
                new_state,
                delete_children,
            ) as libc::c_uint
                == ADF_SUCCESS as libc::c_int as libc::c_uint
                && set_subnet(
                    &mut (*table).base6,
                    ip6.as_mut_ptr(),
                    4 as libc::c_int,
                    0 as libc::c_int,
                    new_state,
                    delete_children,
                ) as libc::c_uint
                    == ADF_SUCCESS as libc::c_int as libc::c_uint
            {
                return ADF_SUCCESS;
            }
        }
        _ => {}
    }
    return ADF_BADSUBNET;
}
/* Allow anything in the supplied subnet, EXCEPT for any more specific
subnets that are already defined */
#[no_mangle]
pub unsafe extern "C" fn ADF_Allow(
    mut table: ADF_AuthTable,
    mut ip: *mut IPAddr,
    mut subnet_bits: libc::c_int,
) -> ADF_Status {
    return set_subnet_(table, ip, subnet_bits, ALLOW, 0 as libc::c_int);
}
/* Allow anything in the supplied subnet, overwriting existing
definitions for any more specific subnets */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn ADF_AllowAll(
    mut table: ADF_AuthTable,
    mut ip: *mut IPAddr,
    mut subnet_bits: libc::c_int,
) -> ADF_Status {
    return set_subnet_(table, ip, subnet_bits, ALLOW, 1 as libc::c_int);
}
/* Deny anything in the supplied subnet, EXCEPT for any more specific
subnets that are already defined */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn ADF_Deny(
    mut table: ADF_AuthTable,
    mut ip: *mut IPAddr,
    mut subnet_bits: libc::c_int,
) -> ADF_Status {
    return set_subnet_(table, ip, subnet_bits, DENY, 0 as libc::c_int);
}
/* Deny anything in the supplied subnet, overwriting existing
definitions for any more specific subnets */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn ADF_DenyAll(
    mut table: ADF_AuthTable,
    mut ip: *mut IPAddr,
    mut subnet_bits: libc::c_int,
) -> ADF_Status {
    return set_subnet_(table, ip, subnet_bits, DENY, 1 as libc::c_int);
}
/* Clear up the table */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn ADF_DestroyTable(mut table: ADF_AuthTable) {
    close_node(&mut (*table).base4);
    close_node(&mut (*table).base6);
    free(table as *mut libc::c_void);
}
/* ================================================== */
unsafe extern "C" fn check_ip_in_node(
    mut start_node: *mut TableNode,
    mut ip: *mut uint32_t,
) -> libc::c_int {
    let mut subnet: uint32_t = 0;
    let mut bits_consumed: libc::c_int = 0 as libc::c_int;
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut finished: libc::c_int = 0 as libc::c_int;
    let mut node: *mut TableNode = 0 as *mut TableNode;
    let mut state: State = DENY;
    node = start_node;
    loop {
        if (*node).state as libc::c_uint != AS_PARENT as libc::c_int as libc::c_uint {
            state = (*node).state
        }
        if !(*node).extended.is_null() {
            subnet = get_subnet(ip, bits_consumed as libc::c_uint);
            node = &mut *(*node).extended.offset(subnet as isize) as *mut _TableNode;
            bits_consumed += 4 as libc::c_int
        } else {
            /* Make decision on this node */
            finished = 1 as libc::c_int
        }
        if !(finished == 0) {
            break;
        }
    }
    match state as libc::c_uint {
        1 => result = 1 as libc::c_int,
        0 => result = 0 as libc::c_int,
        2 => {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"addrfilt.c\x00" as *const u8 as *const libc::c_char,
                340 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"int check_ip_in_node(TableNode *, uint32_t *)\x00",
                ))
                .as_ptr(),
            );
        }
        _ => {}
    }
    return result;
}
/* Check whether a given IP address is allowed by the rules in
the table */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn ADF_IsAllowed(
    mut table: ADF_AuthTable,
    mut ip_addr: *mut IPAddr,
) -> libc::c_int {
    let mut ip6: [uint32_t; 4] = [0; 4];
    match (*ip_addr).family as libc::c_int {
        1 => return check_ip_in_node(&mut (*table).base4, &mut (*ip_addr).addr.in4),
        2 => {
            split_ip6(ip_addr, ip6.as_mut_ptr());
            return check_ip_in_node(&mut (*table).base6, ip6.as_mut_ptr());
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn is_any_allowed(mut node: *mut TableNode, mut parent: State) -> libc::c_int {
    let mut state: State = DENY;
    let mut i: libc::c_int = 0;
    state = if (*node).state as libc::c_uint != AS_PARENT as libc::c_int as libc::c_uint {
        (*node).state as libc::c_uint
    } else {
        parent as libc::c_uint
    } as State;
    if state as libc::c_uint != AS_PARENT as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(
            b"state != AS_PARENT\x00" as *const u8 as *const libc::c_char,
            b"addrfilt.c\x00" as *const u8 as *const libc::c_char,
            376 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"int is_any_allowed(TableNode *, State)\x00",
            ))
            .as_ptr(),
        );
    }
    if !(*node).extended.is_null() {
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (1 as libc::c_ulong) << 4 as libc::c_int {
            if is_any_allowed(&mut *(*node).extended.offset(i as isize), state) != 0 {
                return 1 as libc::c_int;
            }
            i += 1
        }
    } else if state as libc::c_uint == ALLOW as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* Check if at least one address from a given family is allowed by
the rules in the table */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn ADF_IsAnyAllowed(
    mut table: ADF_AuthTable,
    mut family: libc::c_int,
) -> libc::c_int {
    match family {
        1 => return is_any_allowed(&mut (*table).base4, AS_PARENT),
        2 => return is_any_allowed(&mut (*table).base6, AS_PARENT),
        _ => return 0 as libc::c_int,
    };
}
