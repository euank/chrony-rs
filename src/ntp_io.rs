#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    /* This routine is called by ntp_io when a new packet arrives off the network */
    #[no_mangle]
    fn NSR_ProcessRx(remote_addr: *mut NTP_Remote_Address,
                     local_addr: *mut NTP_Local_Address,
                     rx_ts: *mut NTP_Local_Timestamp,
                     message: *mut NTP_Packet, length: libc::c_int);
    /* Register a handler for when select goes true on a file descriptor */
    #[no_mangle]
    fn SCH_AddFileHandler(fd: libc::c_int, events: libc::c_int,
                          handler: SCH_FileHandler,
                          arg: SCH_ArbitraryArgument);
    #[no_mangle]
    fn SCH_RemoveFileHandler(fd: libc::c_int);
    /* Get the time stamp taken after a file descriptor became ready or a timeout expired */
    #[no_mangle]
    fn SCH_GetLastEventTime(cooked: *mut timespec, err: *mut libc::c_double,
                            raw: *mut timespec);
    /* Check if support for the IP family was enabled in the build */
    #[no_mangle]
    fn SCK_IsFamilySupported(family: libc::c_int) -> libc::c_int;
    /* Get the 0.0.0.0/::0 or 127.0.0.1/::1 address */
    #[no_mangle]
    fn SCK_GetAnyLocalIPAddress(family: libc::c_int, local_addr: *mut IPAddr);
    /* Open socket */
    #[no_mangle]
    fn SCK_OpenUdpSocket(remote_addr: *mut IPSockAddr,
                         local_addr: *mut IPSockAddr, flags: libc::c_int)
     -> libc::c_int;
    /* Enable RX timestamping socket option */
    #[no_mangle]
    fn SCK_EnableKernelRxTimestamping(sock_fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SCK_ReceiveMessages(sock_fd: libc::c_int, messages: *mut SCK_Message,
                           max_messages: libc::c_int, flags: libc::c_int)
     -> libc::c_int;
    /* Initialise a new message (e.g. before sending) */
    #[no_mangle]
    fn SCK_InitMessage(message: *mut SCK_Message, addr_type: SCK_AddressType);
    /* Send a message */
    #[no_mangle]
    fn SCK_SendMessage(sock_fd: libc::c_int, message: *mut SCK_Message,
                       flags: libc::c_int) -> libc::c_int;
    /* Close the socket */
    #[no_mangle]
    fn SCK_CloseSocket(sock_fd: libc::c_int);
    /* Minimum severity of messages to be logged */
    #[no_mangle]
    static mut log_min_severity: LOG_Severity;
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
    #[no_mangle]
    fn CNF_GetNTPPort() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetAcquisitionPort() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetBindAcquisitionAddress(family: libc::c_int, addr: *mut IPAddr);
    #[no_mangle]
    fn CNF_GetBindAddress(family: libc::c_int, addr: *mut IPAddr);
    #[no_mangle]
    fn UTI_IPSockAddrToString(sa: *mut IPSockAddr) -> *mut libc::c_char;
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec)
     -> libc::c_double;
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

  This is the header file for the Linux-specific NTP socket I/O bits.
  */
    #[no_mangle]
    fn NIO_Linux_Initialise();
    #[no_mangle]
    fn NIO_Linux_Finalise();
    #[no_mangle]
    fn NIO_Linux_SetTimestampSocketOptions(sock_fd: libc::c_int,
                                           client_only: libc::c_int,
                                           events: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn NIO_Linux_ProcessEvent(sock_fd: libc::c_int, event: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn NIO_Linux_ProcessMessage(message: *mut SCK_Message,
                                local_addr: *mut NTP_Local_Address,
                                local_ts: *mut NTP_Local_Timestamp,
                                event: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn NIO_Linux_RequestTxTimestamp(message: *mut SCK_Message,
                                    sock_fd: libc::c_int);
    #[no_mangle]
    fn NIO_Linux_NotifySocketClosing(sock_fd: libc::c_int);
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_int64 {
    pub hi: uint32_t,
    pub lo: uint32_t,
}
pub type NTP_int32 = uint32_t;
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
/* The buffer used to hold a datagram read from the network */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_Receive_Buffer {
    pub ntp_pkt: NTP_Packet,
    pub extensions: [uint8_t; 1024],
}
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
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCK_Message {
    pub data: *mut libc::c_void,
    pub length: libc::c_uint,
    pub addr_type: SCK_AddressType,
    pub if_index: libc::c_int,
    pub remote_addr: C2RustUnnamed_2,
    pub local_addr: C2RustUnnamed_1,
    pub timestamp: C2RustUnnamed_0,
    pub descriptor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub kernel: timespec,
    pub hw: timespec,
    pub if_index: libc::c_int,
    pub l2_length: libc::c_int,
    pub tx_flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ip: IPAddr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ip: IPSockAddr,
    pub path: *const libc::c_char,
}
pub type SCK_AddressType = libc::c_uint;
pub const SCK_ADDR_UNIX: SCK_AddressType = 2;
pub const SCK_ADDR_IP: SCK_AddressType = 1;
pub const SCK_ADDR_UNSPEC: SCK_AddressType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NTP_Local_Timestamp {
    pub ts: timespec,
    pub err: libc::c_double,
    pub source: NTP_Timestamp_Source,
}
pub type NTP_Timestamp_Source = libc::c_uint;
pub const NTP_TS_HARDWARE: NTP_Timestamp_Source = 2;
pub const NTP_TS_KERNEL: NTP_Timestamp_Source = 1;
pub const NTP_TS_DAEMON: NTP_Timestamp_Source = 0;
pub type SCH_ArbitraryArgument = *mut libc::c_void;
pub type SCH_FileHandler
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                _: SCH_ArbitraryArgument) -> ()>;
/* The server/peer and client sockets for IPv4 and IPv6 */
static mut server_sock_fd4: libc::c_int = 0;
static mut server_sock_fd6: libc::c_int = 0;
static mut client_sock_fd4: libc::c_int = 0;
static mut client_sock_fd6: libc::c_int = 0;
/* Reference counters for server sockets to keep them open only when needed */
static mut server_sock_ref4: libc::c_int = 0;
static mut server_sock_ref6: libc::c_int = 0;
/* Flag indicating we create a new connected client socket for each
   server instead of sharing client_sock_fd4 and client_sock_fd6 */
static mut separate_client_sockets: libc::c_int = 0;
/* Flag indicating the server sockets are not created dynamically when needed,
   either to have a socket for client requests when separate client sockets
   are disabled and client port is equal to server port, or the server port is
   disabled */
static mut permanent_server_sockets: libc::c_int = 0;
/* Flag indicating the server IPv4 socket is bound to an address */
static mut bound_server_sock_fd4: libc::c_int = 0;
/* Flag indicating that we have been initialised */
static mut initialised: libc::c_int = 0 as libc::c_int;
/* ================================================== */
unsafe extern "C" fn open_socket(mut family: libc::c_int,
                                 mut local_port: libc::c_int,
                                 mut client_only: libc::c_int,
                                 mut remote_addr: *mut IPSockAddr)
 -> libc::c_int {
    let mut sock_fd: libc::c_int = 0;
    let mut sock_flags: libc::c_int = 0;
    let mut events: libc::c_int = 1 as libc::c_int;
    let mut local_addr: IPSockAddr =
        IPSockAddr{ip_addr:
                       IPAddr{addr: C2RustUnnamed{in4: 0,},
                              family: 0,
                              _pad: 0,},
                   port: 0,};
    if SCK_IsFamilySupported(family) == 0 { return -(1 as libc::c_int) }
    if client_only == 0 {
        CNF_GetBindAddress(family, &mut local_addr.ip_addr);
    } else { CNF_GetBindAcquisitionAddress(family, &mut local_addr.ip_addr); }
    if local_addr.ip_addr.family as libc::c_int != family {
        SCK_GetAnyLocalIPAddress(family, &mut local_addr.ip_addr);
    }
    local_addr.port = local_port as uint16_t;
    sock_flags = 4 as libc::c_int | 16 as libc::c_int;
    if client_only == 0 { sock_flags |= 2 as libc::c_int }
    sock_fd = SCK_OpenUdpSocket(remote_addr, &mut local_addr, sock_flags);
    if sock_fd < 0 as libc::c_int {
        if client_only == 0 {
            LOG_Message(LOGS_ERR,
                        b"Could not open NTP socket on %s\x00" as *const u8 as
                            *const libc::c_char,
                        UTI_IPSockAddrToString(&mut local_addr));
        }
        return -(1 as libc::c_int)
    }
    if client_only == 0 && family == 1 as libc::c_int &&
           local_addr.port as libc::c_int > 0 as libc::c_int {
        bound_server_sock_fd4 =
            (local_addr.ip_addr.addr.in4 != 0 as libc::c_int as in_addr_t) as
                libc::c_int
    }
    /* Enable kernel/HW timestamping of packets */
    if NIO_Linux_SetTimestampSocketOptions(sock_fd, client_only, &mut events)
           == 0 {
        (SCK_EnableKernelRxTimestamping(sock_fd)) == 0;
    }
    /* Register handler for read and possibly exception events on the socket */
    SCH_AddFileHandler(sock_fd, events,
                       Some(read_from_socket as
                                unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int,
                                                     _: *mut libc::c_void)
                                    -> ()), 0 as *mut libc::c_void);
    return sock_fd;
}
/* ================================================== */
unsafe extern "C" fn open_separate_client_socket(mut remote_addr:
                                                     *mut IPSockAddr)
 -> libc::c_int {
    return open_socket((*remote_addr).ip_addr.family as libc::c_int,
                       0 as libc::c_int, 1 as libc::c_int, remote_addr);
}
/* ================================================== */
unsafe extern "C" fn close_socket(mut sock_fd: libc::c_int) {
    if sock_fd == -(1 as libc::c_int) { return }
    NIO_Linux_NotifySocketClosing(sock_fd);
    SCH_RemoveFileHandler(sock_fd);
    SCK_CloseSocket(sock_fd);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_Initialise(mut family: libc::c_int) {
    let mut server_port: libc::c_int = 0;
    let mut client_port: libc::c_int = 0;
    if initialised == 0 {
    } else {
        __assert_fail(b"!initialised\x00" as *const u8 as *const libc::c_char,
                      b"ntp_io.c\x00" as *const u8 as *const libc::c_char,
                      159 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"void NIO_Initialise(int)\x00")).as_ptr());
    }
    initialised = 1 as libc::c_int;
    NIO_Linux_Initialise();
    server_port = CNF_GetNTPPort();
    client_port = CNF_GetAcquisitionPort();
    /* Use separate connected sockets if client port is negative */
    separate_client_sockets = (client_port < 0 as libc::c_int) as libc::c_int;
    if client_port < 0 as libc::c_int { client_port = 0 as libc::c_int }
    permanent_server_sockets =
        (server_port == 0 ||
             separate_client_sockets == 0 && client_port == server_port) as
            libc::c_int;
    server_sock_fd4 = -(1 as libc::c_int);
    server_sock_fd6 = -(1 as libc::c_int);
    client_sock_fd4 = -(1 as libc::c_int);
    client_sock_fd6 = -(1 as libc::c_int);
    server_sock_ref4 = 0 as libc::c_int;
    server_sock_ref6 = 0 as libc::c_int;
    if family == 0 as libc::c_int || family == 1 as libc::c_int {
        if permanent_server_sockets != 0 && server_port != 0 {
            server_sock_fd4 =
                open_socket(1 as libc::c_int, server_port, 0 as libc::c_int,
                            0 as *mut IPSockAddr)
        }
        if separate_client_sockets == 0 {
            if client_port != server_port || server_port == 0 {
                client_sock_fd4 =
                    open_socket(1 as libc::c_int, client_port,
                                1 as libc::c_int, 0 as *mut IPSockAddr)
            } else { client_sock_fd4 = server_sock_fd4 }
        }
    }
    if family == 0 as libc::c_int || family == 2 as libc::c_int {
        if permanent_server_sockets != 0 && server_port != 0 {
            server_sock_fd6 =
                open_socket(2 as libc::c_int, server_port, 0 as libc::c_int,
                            0 as *mut IPSockAddr)
        }
        if separate_client_sockets == 0 {
            if client_port != server_port || server_port == 0 {
                client_sock_fd6 =
                    open_socket(2 as libc::c_int, client_port,
                                1 as libc::c_int, 0 as *mut IPSockAddr)
            } else { client_sock_fd6 = server_sock_fd6 }
        }
    }
    if server_port != 0 && permanent_server_sockets != 0 &&
           server_sock_fd4 == -(1 as libc::c_int) &&
           server_sock_fd6 == -(1 as libc::c_int) ||
           separate_client_sockets == 0 &&
               client_sock_fd4 == -(1 as libc::c_int) &&
               client_sock_fd6 == -(1 as libc::c_int) {
        LOG_Message(LOGS_FATAL,
                    b"Could not open NTP sockets\x00" as *const u8 as
                        *const libc::c_char);
        exit(1 as libc::c_int);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_Finalise() {
    if server_sock_fd4 != client_sock_fd4 { close_socket(client_sock_fd4); }
    close_socket(server_sock_fd4);
    client_sock_fd4 = -(1 as libc::c_int);
    server_sock_fd4 = client_sock_fd4;
    if server_sock_fd6 != client_sock_fd6 { close_socket(client_sock_fd6); }
    close_socket(server_sock_fd6);
    client_sock_fd6 = -(1 as libc::c_int);
    server_sock_fd6 = client_sock_fd6;
    NIO_Linux_Finalise();
    initialised = 0 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_OpenClientSocket(mut remote_addr:
                                                  *mut NTP_Remote_Address)
 -> libc::c_int {
    if separate_client_sockets != 0 {
        return open_separate_client_socket(remote_addr)
    } else {
        match (*remote_addr).ip_addr.family as libc::c_int {
            1 => { return client_sock_fd4 }
            2 => { return client_sock_fd6 }
            _ => { return -(1 as libc::c_int) }
        }
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_OpenServerSocket(mut remote_addr:
                                                  *mut NTP_Remote_Address)
 -> libc::c_int {
    match (*remote_addr).ip_addr.family as libc::c_int {
        1 => {
            if permanent_server_sockets != 0 { return server_sock_fd4 }
            if server_sock_fd4 == -(1 as libc::c_int) {
                server_sock_fd4 =
                    open_socket(1 as libc::c_int, CNF_GetNTPPort(),
                                0 as libc::c_int, 0 as *mut IPSockAddr)
            }
            if server_sock_fd4 != -(1 as libc::c_int) {
                server_sock_ref4 += 1
            }
            return server_sock_fd4
        }
        2 => {
            if permanent_server_sockets != 0 { return server_sock_fd6 }
            if server_sock_fd6 == -(1 as libc::c_int) {
                server_sock_fd6 =
                    open_socket(2 as libc::c_int, CNF_GetNTPPort(),
                                0 as libc::c_int, 0 as *mut IPSockAddr)
            }
            if server_sock_fd6 != -(1 as libc::c_int) {
                server_sock_ref6 += 1
            }
            return server_sock_fd6
        }
        _ => { return -(1 as libc::c_int) }
    };
}
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

  This is the header file for the NTP socket I/O bits.

  */
/* Function to initialise the module. */
/* Function to finalise the module */
/* Function to obtain a socket for sending client packets */
/* Function to obtain a socket for sending server/peer packets */
/* Function to close a socket returned by NIO_OpenClientSocket() */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_CloseClientSocket(mut sock_fd: libc::c_int) {
    if separate_client_sockets != 0 { close_socket(sock_fd); };
}
/* Function to close a socket returned by NIO_OpenServerSocket() */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_CloseServerSocket(mut sock_fd: libc::c_int) {
    if permanent_server_sockets != 0 || sock_fd == -(1 as libc::c_int) {
        return
    }
    if sock_fd == server_sock_fd4 {
        server_sock_ref4 -= 1;
        if server_sock_ref4 <= 0 as libc::c_int {
            close_socket(server_sock_fd4);
            server_sock_fd4 = -(1 as libc::c_int)
        }
    } else if sock_fd == server_sock_fd6 {
        server_sock_ref6 -= 1;
        if server_sock_ref6 <= 0 as libc::c_int {
            close_socket(server_sock_fd6);
            server_sock_fd6 = -(1 as libc::c_int)
        }
    } else {
        __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                      b"ntp_io.c\x00" as *const u8 as *const libc::c_char,
                      320 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"void NIO_CloseServerSocket(int)\x00")).as_ptr());
    };
}
/* Function to check if socket is a server socket */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_IsServerSocket(mut sock_fd: libc::c_int)
 -> libc::c_int {
    return (sock_fd != -(1 as libc::c_int) &&
                (sock_fd == server_sock_fd4 || sock_fd == server_sock_fd6)) as
               libc::c_int;
}
/* Function to check if a server socket is currently open */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_IsServerSocketOpen() -> libc::c_int {
    return (server_sock_fd4 != -(1 as libc::c_int) ||
                server_sock_fd6 != -(1 as libc::c_int)) as libc::c_int;
}
/* Function to check if client packets can be sent to a server */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn NIO_IsServerConnectable(mut remote_addr:
                                                     *mut NTP_Remote_Address)
 -> libc::c_int {
    let mut sock_fd: libc::c_int = 0;
    sock_fd = open_separate_client_socket(remote_addr);
    if sock_fd == -(1 as libc::c_int) { return 0 as libc::c_int }
    close_socket(sock_fd);
    return 1 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn process_message(mut message: *mut SCK_Message,
                                     mut sock_fd: libc::c_int,
                                     mut event: libc::c_int) {
    let mut local_addr: NTP_Local_Address =
        NTP_Local_Address{ip_addr:
                              IPAddr{addr: C2RustUnnamed{in4: 0,},
                                     family: 0,
                                     _pad: 0,},
                          if_index: 0,
                          sock_fd: 0,};
    let mut local_ts: NTP_Local_Timestamp =
        NTP_Local_Timestamp{ts: timespec{tv_sec: 0, tv_nsec: 0,},
                            err: 0.,
                            source: NTP_TS_DAEMON,};
    let mut sched_ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    SCH_GetLastEventTime(&mut local_ts.ts, &mut local_ts.err,
                         0 as *mut timespec);
    local_ts.source = NTP_TS_DAEMON;
    sched_ts = local_ts.ts;
    if (*message).addr_type as libc::c_uint !=
           SCK_ADDR_IP as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Unexpected address type\x00" as *const u8 as
                            *const libc::c_char);
        }
        return
    }
    local_addr.ip_addr = (*message).local_addr.ip;
    local_addr.if_index = (*message).if_index;
    local_addr.sock_fd = sock_fd;
    if NIO_Linux_ProcessMessage(message, &mut local_addr, &mut local_ts,
                                event) != 0 {
        return
    }
    if local_ts.source as libc::c_uint !=
           NTP_TS_DAEMON as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Updated RX timestamp delay=%.9f tss=%u\x00" as
                            *const u8 as *const libc::c_char,
                        UTI_DiffTimespecsToDouble(&mut sched_ts,
                                                  &mut local_ts.ts),
                        local_ts.source as libc::c_uint);
        }
    }
    /* Just ignore the packet if it's not of a recognized length */
    if (*message).length < 48 as libc::c_ulong as libc::c_int as libc::c_uint
           ||
           (*message).length as libc::c_ulong >
               ::std::mem::size_of::<NTP_Receive_Buffer>() as libc::c_ulong {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"Unexpected length\x00" as *const u8 as
                            *const libc::c_char);
        }
        return
    }
    NSR_ProcessRx(&mut (*message).remote_addr.ip, &mut local_addr,
                  &mut local_ts, (*message).data as *mut NTP_Packet,
                  (*message).length as libc::c_int);
}
/* ================================================== */
/* Forward prototypes */
/* ================================================== */
unsafe extern "C" fn read_from_socket(mut sock_fd: libc::c_int,
                                      mut event: libc::c_int,
                                      mut anything: *mut libc::c_void) {
    /* This should only be called when there is something
     to read, otherwise it may block */
    let mut messages: [SCK_Message; 4] =
        [SCK_Message{data: 0 as *mut libc::c_void,
                     length: 0,
                     addr_type: SCK_ADDR_UNSPEC,
                     if_index: 0,
                     remote_addr:
                         C2RustUnnamed_2{ip:
                                             IPSockAddr{ip_addr:
                                                            IPAddr{addr:
                                                                       C2RustUnnamed{in4:
                                                                                         0,},
                                                                   family: 0,
                                                                   _pad: 0,},
                                                        port: 0,},},
                     local_addr:
                         C2RustUnnamed_1{ip:
                                             IPAddr{addr:
                                                        C2RustUnnamed{in4:
                                                                          0,},
                                                    family: 0,
                                                    _pad: 0,},},
                     timestamp:
                         C2RustUnnamed_0{kernel:
                                             timespec{tv_sec: 0, tv_nsec: 0,},
                                         hw: timespec{tv_sec: 0, tv_nsec: 0,},
                                         if_index: 0,
                                         l2_length: 0,
                                         tx_flags: 0,},
                     descriptor: 0,}; 4];
    let mut i: libc::c_int = 0;
    let mut received: libc::c_int = 0;
    let mut flags: libc::c_int = 0 as libc::c_int;
    if NIO_Linux_ProcessEvent(sock_fd, event) != 0 { return }
    if event == 4 as libc::c_int { flags |= 1 as libc::c_int }
    received =
        SCK_ReceiveMessages(sock_fd, messages.as_mut_ptr(), 4 as libc::c_int,
                            flags);
    if received <= 0 as libc::c_int { return }
    i = 0 as libc::c_int;
    while i < received {
        process_message(&mut *messages.as_mut_ptr().offset(i as isize),
                        sock_fd, event);
        i += 1
    };
}
/* Function to transmit a packet */
/* ================================================== */
/* Send a packet to remote address from local address */
#[no_mangle]
pub unsafe extern "C" fn NIO_SendPacket(mut packet: *mut NTP_Packet,
                                        mut remote_addr:
                                            *mut NTP_Remote_Address,
                                        mut local_addr:
                                            *mut NTP_Local_Address,
                                        mut length: libc::c_int,
                                        mut process_tx: libc::c_int)
 -> libc::c_int {
    let mut message: SCK_Message =
        SCK_Message{data: 0 as *mut libc::c_void,
                    length: 0,
                    addr_type: SCK_ADDR_UNSPEC,
                    if_index: 0,
                    remote_addr:
                        C2RustUnnamed_2{ip:
                                            IPSockAddr{ip_addr:
                                                           IPAddr{addr:
                                                                      C2RustUnnamed{in4:
                                                                                        0,},
                                                                  family: 0,
                                                                  _pad: 0,},
                                                       port: 0,},},
                    local_addr:
                        C2RustUnnamed_1{ip:
                                            IPAddr{addr:
                                                       C2RustUnnamed{in4: 0,},
                                                   family: 0,
                                                   _pad: 0,},},
                    timestamp:
                        C2RustUnnamed_0{kernel:
                                            timespec{tv_sec: 0, tv_nsec: 0,},
                                        hw: timespec{tv_sec: 0, tv_nsec: 0,},
                                        if_index: 0,
                                        l2_length: 0,
                                        tx_flags: 0,},
                    descriptor: 0,};
    if initialised != 0 {
    } else {
        __assert_fail(b"initialised\x00" as *const u8 as *const libc::c_char,
                      b"ntp_io.c\x00" as *const u8 as *const libc::c_char,
                      444 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 86],
                                                &[libc::c_char; 86]>(b"int NIO_SendPacket(NTP_Packet *, NTP_Remote_Address *, NTP_Local_Address *, int, int)\x00")).as_ptr());
    }
    if (*local_addr).sock_fd == -(1 as libc::c_int) {
        if 0 as libc::c_int != 0 &&
               log_min_severity as libc::c_int == LOGS_DEBUG as libc::c_int {
            LOG_Message(LOGS_DEBUG,
                        b"No socket to send to %s\x00" as *const u8 as
                            *const libc::c_char,
                        UTI_IPSockAddrToString(remote_addr));
        }
        return 0 as libc::c_int
    }
    SCK_InitMessage(&mut message, SCK_ADDR_IP);
    message.data = packet as *mut libc::c_void;
    message.length = length as libc::c_uint;
    /* Specify remote address if the socket is not connected */
    if NIO_IsServerSocket((*local_addr).sock_fd) != 0 ||
           separate_client_sockets == 0 {
        message.remote_addr.ip.ip_addr = (*remote_addr).ip_addr;
        message.remote_addr.ip.port = (*remote_addr).port
    }
    message.if_index = (*local_addr).if_index;
    message.local_addr.ip = (*local_addr).ip_addr;
    if process_tx != 0 {
        NIO_Linux_RequestTxTimestamp(&mut message, (*local_addr).sock_fd);
    }
    if SCK_SendMessage((*local_addr).sock_fd, &mut message, 0 as libc::c_int)
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
