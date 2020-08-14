#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(llvm_asm, const_raw_ptr_to_usize_cast, extern_types, label_break_value, register_tool)]

use c2rust_asm_casts::AsmCastTrait;
extern "C" {
    pub type ARR_Instance_Record;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
    fn ARR_GetElement(array: ARR_Instance, index: libc::c_uint) -> *mut libc::c_void;
    /* Return pointer to the internal array of elements */
    #[no_mangle]
    fn ARR_GetElements(array: ARR_Instance) -> *mut libc::c_void;
    /* Return current size of the array */
    #[no_mangle]
    fn ARR_GetSize(array: ARR_Instance) -> libc::c_uint;
    #[no_mangle]
    fn Malloc2(nmemb: size_t, size: size_t) -> *mut libc::c_void;
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
    /* Convert a timeval into a timespec */
    #[no_mangle]
    fn UTI_TimevalToTimespec(tv: *mut timeval, ts: *mut timespec);
    /* Convert a timespec into a timeval */
    #[no_mangle]
    fn UTI_TimespecToTimeval(ts: *mut timespec, tv: *mut timeval);
    /* Convert a timespec into a floating point number of seconds */
    #[no_mangle]
    fn UTI_TimespecToDouble(ts: *mut timespec) -> libc::c_double;
    /* Returns -1 if a comes earlier than b, 0 if a is the same time as b,
    and +1 if a comes after b */
    #[no_mangle]
    fn UTI_CompareTimespecs(a: *mut timespec, b: *mut timespec) -> libc::c_int;
    /* Calculate result = a - b */
    #[no_mangle]
    fn UTI_DiffTimespecs(result: *mut timespec, a: *mut timespec, b: *mut timespec);
    /* Calculate result = a - b and return as a double */
    #[no_mangle]
    fn UTI_DiffTimespecsToDouble(a: *mut timespec, b: *mut timespec) -> libc::c_double;
    /* Add a double increment to a timespec to get a new one. 'start' is
    the starting time, 'end' is the result that we return.  This is
    safe to use if start and end are the same */
    #[no_mangle]
    fn UTI_AddDoubleToTimespec(start: *mut timespec, increment: libc::c_double, end: *mut timespec);
    /* Adjust time following a frequency/offset change */
    #[no_mangle]
    fn UTI_AdjustTimespec(
        old_ts: *mut timespec,
        when: *mut timespec,
        new_ts: *mut timespec,
        delta_time: *mut libc::c_double,
        dfreq: libc::c_double,
        doffset: libc::c_double,
    );
    /* Fill buffer with random bytes from /dev/urandom or a faster source if it's
    available (e.g. arc4random()), which may not necessarily be suitable for
    generating long-term keys */
    #[no_mangle]
    fn UTI_GetRandomBytes(buf: *mut libc::c_void, len: libc::c_uint);
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
    /* Convert raw time to cooked. */
    #[no_mangle]
    fn LCL_CookTime(raw: *mut timespec, cooked: *mut timespec, err: *mut libc::c_double);
    /* Add a handler.  Then handler MUST NOT deregister itself!!! */
    #[no_mangle]
    fn LCL_AddParameterChangeHandler(
        handler: LCL_ParameterChangeHandler,
        anything: *mut libc::c_void,
    );
    /* Check if a handler is invoked first when dispatching */
    #[no_mangle]
    fn LCL_IsFirstParameterChangeHandler(handler: LCL_ParameterChangeHandler) -> libc::c_int;
    /* Routine to invoke notify handlers on an unexpected time jump
    in system clock */
    #[no_mangle]
    fn LCL_NotifyExternalTimeStep(
        raw: *mut timespec,
        cooked: *mut timespec,
        offset: libc::c_double,
        dispersion: libc::c_double,
    );
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char, _: ...);
}
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
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

 Exported header file for sched.c
 */
/* Type for timeout IDs, valid IDs are always greater than zero */
pub type SCH_TimeoutID = libc::c_uint;
pub type SCH_TimeoutClass = libc::c_uint;
pub const SCH_NumberOfClasses: SCH_TimeoutClass = 4;
pub const SCH_NtpBroadcastClass: SCH_TimeoutClass = 3;
pub const SCH_NtpPeerClass: SCH_TimeoutClass = 2;
pub const SCH_NtpClientClass: SCH_TimeoutClass = 1;
pub const SCH_ReservedTimeoutValue: SCH_TimeoutClass = 0;
pub type SCH_ArbitraryArgument = *mut libc::c_void;
pub type SCH_FileHandler =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: SCH_ArbitraryArgument) -> ()>;
pub type SCH_TimeoutHandler = Option<unsafe extern "C" fn(_: SCH_ArbitraryArgument) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _TimerQueueEntry {
    pub next: *mut _TimerQueueEntry,
    pub prev: *mut _TimerQueueEntry,
    pub ts: timespec,
    pub id: SCH_TimeoutID,
    pub class: SCH_TimeoutClass,
    pub handler: SCH_TimeoutHandler,
    pub arg: SCH_ArbitraryArgument,
}
/* ================================================== */
/* Variables to handler the timer queue */
pub type TimerQueueEntry = _TimerQueueEntry;
pub type LCL_ChangeType = libc::c_uint;
pub const LCL_ChangeUnknownStep: LCL_ChangeType = 2;
pub const LCL_ChangeStep: LCL_ChangeType = 1;
pub const LCL_ChangeAdjust: LCL_ChangeType = 0;
pub type LCL_ParameterChangeHandler = Option<
    unsafe extern "C" fn(
        _: *mut timespec,
        _: *mut timespec,
        _: libc::c_double,
        _: libc::c_double,
        _: LCL_ChangeType,
        _: *mut libc::c_void,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileHandlerEntry {
    pub handler: SCH_FileHandler,
    pub arg: SCH_ArbitraryArgument,
    pub events: libc::c_int,
}
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
/* Forward and back links in the list */
/* Local system time at which the
timeout is to expire.  Clearly this
must be in terms of what the
operating system thinks of as
system time, because it will be an
argument to select().  Therefore,
any fudges etc that our local time
driver module would apply to time
that we pass to clients etc doesn't
apply to this. */
/* ID to allow client to delete
timeout */
/* The class that the epoch is in */
/* The handler routine to use */
/* The argument to pass to the handler */
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Richard P. Curnow  1997-2003
* Copyright (C) Miroslav Lichvar  2011, 2013-2016
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

 This file contains the scheduling loop and the timeout queue.

 */
/* ================================================== */
/* Flag indicating that we are initialised */
static mut initialised: libc::c_int = 0 as libc::c_int;
/* ================================================== */
/* One more than the highest file descriptor that is registered */
static mut one_highest_fd: libc::c_uint = 0;
static mut file_handlers: ARR_Instance =
    0 as *const ARR_Instance_Record as *mut ARR_Instance_Record;
/* Timestamp when last select() returned */
static mut last_select_ts: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
static mut last_select_ts_raw: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
static mut last_select_ts_err: libc::c_double = 0.;
/* The timer queue.  We only use the next and prev entries of this
record, these chain to the real entries. */
static mut timer_queue: TimerQueueEntry = TimerQueueEntry {
    next: 0 as *const _TimerQueueEntry as *mut _TimerQueueEntry,
    prev: 0 as *const _TimerQueueEntry as *mut _TimerQueueEntry,
    ts: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    id: 0,
    class: SCH_ReservedTimeoutValue,
    handler: None,
    arg: 0 as *const libc::c_void as *mut libc::c_void,
};
static mut n_timer_queue_entries: libc::c_ulong = 0;
static mut next_tqe_id: SCH_TimeoutID = 0;
/* Pointer to head of free list */
static mut tqe_free_list: *mut TimerQueueEntry =
    0 as *const TimerQueueEntry as *mut TimerQueueEntry;
/* Timestamp when was last timeout dispatched for each class */
static mut last_class_dispatch: [timespec; 4] = [timespec {
    tv_sec: 0,
    tv_nsec: 0,
}; 4];
/* ================================================== */
static mut need_to_exit: libc::c_int = 0;
/* Exported functions */
/* Initialisation function for the module */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCH_Initialise() {
    file_handlers = ARR_CreateInstance(
        ::std::mem::size_of::<FileHandlerEntry>() as libc::c_ulong as libc::c_uint
    );
    n_timer_queue_entries = 0 as libc::c_int as libc::c_ulong;
    next_tqe_id = 0 as libc::c_int as SCH_TimeoutID;
    timer_queue.next = &mut timer_queue;
    timer_queue.prev = &mut timer_queue;
    need_to_exit = 0 as libc::c_int;
    LCL_AddParameterChangeHandler(
        Some(
            handle_slew
                as unsafe extern "C" fn(
                    _: *mut timespec,
                    _: *mut timespec,
                    _: libc::c_double,
                    _: libc::c_double,
                    _: LCL_ChangeType,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
    LCL_ReadRawTime(&mut last_select_ts_raw);
    last_select_ts = last_select_ts_raw;
    initialised = 1 as libc::c_int;
}
/* Finalisation function for the module */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCH_Finalise() {
    ARR_DestroyInstance(file_handlers);
    initialised = 0 as libc::c_int;
}
/* Register a handler for when select goes true on a file descriptor */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCH_AddFileHandler(
    mut fd: libc::c_int,
    mut events: libc::c_int,
    mut handler: SCH_FileHandler,
    mut arg: SCH_ArbitraryArgument,
) {
    let mut ptr: *mut FileHandlerEntry = 0 as *mut FileHandlerEntry;
    if initialised != 0 {
    } else {
        __assert_fail(
            b"initialised\x00" as *const u8 as *const libc::c_char,
            b"sched.c\x00" as *const u8 as *const libc::c_char,
            161 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 74], &[libc::c_char; 74]>(
                b"void SCH_AddFileHandler(int, int, SCH_FileHandler, SCH_ArbitraryArgument)\x00",
            ))
            .as_ptr(),
        );
    }
    if events != 0 {
    } else {
        __assert_fail(
            b"events\x00" as *const u8 as *const libc::c_char,
            b"sched.c\x00" as *const u8 as *const libc::c_char,
            162 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 74], &[libc::c_char; 74]>(
                b"void SCH_AddFileHandler(int, int, SCH_FileHandler, SCH_ArbitraryArgument)\x00",
            ))
            .as_ptr(),
        );
    }
    if fd >= 0 as libc::c_int {
    } else {
        __assert_fail(
            b"fd >= 0\x00" as *const u8 as *const libc::c_char,
            b"sched.c\x00" as *const u8 as *const libc::c_char,
            163 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 74], &[libc::c_char; 74]>(
                b"void SCH_AddFileHandler(int, int, SCH_FileHandler, SCH_ArbitraryArgument)\x00",
            ))
            .as_ptr(),
        );
    }
    if fd >= 1024 as libc::c_int {
        LOG_Message(
            LOGS_FATAL,
            b"Too many file descriptors\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    /* Resize the array if the descriptor is highest so far */
    while ARR_GetSize(file_handlers) <= fd as libc::c_uint {
        ptr = ARR_GetNewElement(file_handlers) as *mut FileHandlerEntry;
        (*ptr).handler = None;
        (*ptr).arg = 0 as *mut libc::c_void;
        (*ptr).events = 0 as libc::c_int
    }
    ptr = ARR_GetElement(file_handlers, fd as libc::c_uint) as *mut FileHandlerEntry;
    /* Don't want to allow the same fd to register a handler more than
    once without deleting a previous association - this suggests
    a bug somewhere else in the program. */
    if (*ptr).handler.is_none() {
    } else {
        __assert_fail(
            b"!ptr->handler\x00" as *const u8 as *const libc::c_char,
            b"sched.c\x00" as *const u8 as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 74], &[libc::c_char; 74]>(
                b"void SCH_AddFileHandler(int, int, SCH_FileHandler, SCH_ArbitraryArgument)\x00",
            ))
            .as_ptr(),
        );
    }
    (*ptr).handler = handler;
    (*ptr).arg = arg;
    (*ptr).events = events;
    if one_highest_fd < (fd + 1 as libc::c_int) as libc::c_uint {
        one_highest_fd = (fd + 1 as libc::c_int) as libc::c_uint
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCH_RemoveFileHandler(mut fd: libc::c_int) {
    let mut ptr: *mut FileHandlerEntry = 0 as *mut FileHandlerEntry;
    if initialised != 0 {
    } else {
        __assert_fail(
            b"initialised\x00" as *const u8 as *const libc::c_char,
            b"sched.c\x00" as *const u8 as *const libc::c_char,
            199 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void SCH_RemoveFileHandler(int)\x00",
            ))
            .as_ptr(),
        );
    }
    ptr = ARR_GetElement(file_handlers, fd as libc::c_uint) as *mut FileHandlerEntry;
    /* Check that a handler was registered for the fd in question */
    if (*ptr).handler.is_some() {
    } else {
        __assert_fail(
            b"ptr->handler\x00" as *const u8 as *const libc::c_char,
            b"sched.c\x00" as *const u8 as *const libc::c_char,
            204 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void SCH_RemoveFileHandler(int)\x00",
            ))
            .as_ptr(),
        );
    }
    (*ptr).handler = None;
    (*ptr).arg = 0 as *mut libc::c_void;
    (*ptr).events = 0 as libc::c_int;
    /* Find new highest file descriptor */
    while one_highest_fd > 0 as libc::c_int as libc::c_uint {
        ptr = ARR_GetElement(
            file_handlers,
            one_highest_fd.wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) as *mut FileHandlerEntry;
        if (*ptr).handler.is_some() {
            break;
        }
        one_highest_fd = one_highest_fd.wrapping_sub(1)
    }
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCH_SetFileHandlerEvent(
    mut fd: libc::c_int,
    mut event: libc::c_int,
    mut enable: libc::c_int,
) {
    let mut ptr: *mut FileHandlerEntry = 0 as *mut FileHandlerEntry;
    ptr = ARR_GetElement(file_handlers, fd as libc::c_uint) as *mut FileHandlerEntry;
    if enable != 0 {
        (*ptr).events |= event
    } else {
        (*ptr).events &= !event
    };
}
/* Get the time stamp taken after a file descriptor became ready or a timeout expired */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCH_GetLastEventTime(
    mut cooked: *mut timespec,
    mut err: *mut libc::c_double,
    mut raw: *mut timespec,
) {
    if !cooked.is_null() {
        *cooked = last_select_ts;
        if !err.is_null() {
            *err = last_select_ts_err
        }
    }
    if !raw.is_null() {
        *raw = last_select_ts_raw
    };
}
unsafe extern "C" fn allocate_tqe() -> *mut TimerQueueEntry {
    let mut new_block: *mut TimerQueueEntry = 0 as *mut TimerQueueEntry;
    let mut result: *mut TimerQueueEntry = 0 as *mut TimerQueueEntry;
    let mut i: libc::c_int = 0;
    if tqe_free_list.is_null() {
        new_block = Malloc2(
            32 as libc::c_int as size_t,
            ::std::mem::size_of::<TimerQueueEntry>() as libc::c_ulong,
        ) as *mut TimerQueueEntry;
        i = 1 as libc::c_int;
        while i < 32 as libc::c_int {
            let ref mut fresh0 = (*new_block.offset(i as isize)).next;
            *fresh0 =
                &mut *new_block.offset((i - 1 as libc::c_int) as isize) as *mut TimerQueueEntry;
            i += 1
        }
        let ref mut fresh1 = (*new_block.offset(0 as libc::c_int as isize)).next;
        *fresh1 = 0 as *mut _TimerQueueEntry;
        tqe_free_list = &mut *new_block.offset((32 as libc::c_int - 1 as libc::c_int) as isize)
            as *mut TimerQueueEntry
    }
    result = tqe_free_list;
    tqe_free_list = (*tqe_free_list).next;
    return result;
}
/* ================================================== */
unsafe extern "C" fn release_tqe(mut node: *mut TimerQueueEntry) {
    (*node).next = tqe_free_list;
    tqe_free_list = node;
}
/* ================================================== */
unsafe extern "C" fn get_new_tqe_id() -> SCH_TimeoutID {
    let mut ptr: *mut TimerQueueEntry = 0 as *mut TimerQueueEntry;
    'c_9678: loop {
        next_tqe_id = next_tqe_id.wrapping_add(1);
        if next_tqe_id == 0 {
            continue;
        }
        /* Make sure the ID isn't already used */
        ptr = timer_queue.next;
        loop {
            if !(ptr != &mut timer_queue as *mut TimerQueueEntry) {
                break 'c_9678;
            }
            if (*ptr).id == next_tqe_id {
                break;
            }
            ptr = (*ptr).next
        }
    }
    return next_tqe_id;
}
/* This queues a timeout to elapse at a given (raw) local time */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCH_AddTimeout(
    mut ts: *mut timespec,
    mut handler: SCH_TimeoutHandler,
    mut arg: SCH_ArbitraryArgument,
) -> SCH_TimeoutID {
    let mut new_tqe: *mut TimerQueueEntry = 0 as *mut TimerQueueEntry;
    let mut ptr: *mut TimerQueueEntry = 0 as *mut TimerQueueEntry;
    if initialised != 0 {
    } else {
        __assert_fail(b"initialised\x00" as *const u8 as *const libc::c_char,
                      b"sched.c\x00" as *const u8 as *const libc::c_char,
                      309 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 91],
                                                &[libc::c_char; 91]>(b"SCH_TimeoutID SCH_AddTimeout(struct timespec *, SCH_TimeoutHandler, SCH_ArbitraryArgument)\x00")).as_ptr());
    }
    new_tqe = allocate_tqe();
    (*new_tqe).id = get_new_tqe_id();
    (*new_tqe).handler = handler;
    (*new_tqe).arg = arg;
    (*new_tqe).ts = *ts;
    (*new_tqe).class = SCH_ReservedTimeoutValue;
    /* Now work out where to insert the new entry in the list */
    ptr = timer_queue.next;
    while ptr != &mut timer_queue as *mut TimerQueueEntry {
        if UTI_CompareTimespecs(&mut (*new_tqe).ts, &mut (*ptr).ts) == -(1 as libc::c_int) {
            break;
        }
        ptr = (*ptr).next
    }
    /* At this stage, we want to insert the new entry immediately before
    the entry identified by 'ptr' */
    (*new_tqe).next = ptr;
    (*new_tqe).prev = (*ptr).prev;
    (*(*ptr).prev).next = new_tqe;
    (*ptr).prev = new_tqe;
    n_timer_queue_entries = n_timer_queue_entries.wrapping_add(1);
    return (*new_tqe).id;
}
/* This queues a timeout to elapse at a given delta time relative to the current (raw) time */
/* ================================================== */
/* This queues a timeout to elapse at a given delta time relative to
the current (raw) time */
#[no_mangle]
pub unsafe extern "C" fn SCH_AddTimeoutByDelay(
    mut delay: libc::c_double,
    mut handler: SCH_TimeoutHandler,
    mut arg: SCH_ArbitraryArgument,
) -> SCH_TimeoutID {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut then: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if initialised != 0 {
    } else {
        __assert_fail(b"initialised\x00" as *const u8 as *const libc::c_char,
                      b"sched.c\x00" as *const u8 as *const libc::c_char,
                      350 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 87],
                                                &[libc::c_char; 87]>(b"SCH_TimeoutID SCH_AddTimeoutByDelay(double, SCH_TimeoutHandler, SCH_ArbitraryArgument)\x00")).as_ptr());
    }
    if delay >= 0.0f64 {
    } else {
        __assert_fail(b"delay >= 0.0\x00" as *const u8 as *const libc::c_char,
                      b"sched.c\x00" as *const u8 as *const libc::c_char,
                      351 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 87],
                                                &[libc::c_char; 87]>(b"SCH_TimeoutID SCH_AddTimeoutByDelay(double, SCH_TimeoutHandler, SCH_ArbitraryArgument)\x00")).as_ptr());
    }
    LCL_ReadRawTime(&mut now);
    UTI_AddDoubleToTimespec(&mut now, delay, &mut then);
    if UTI_CompareTimespecs(&mut now, &mut then) > 0 as libc::c_int {
        LOG_Message(
            LOGS_FATAL,
            b"Timeout overflow\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return SCH_AddTimeout(&mut then, handler, arg);
}
/* This queues a timeout in a particular class, ensuring that the
expiry time is at least a given separation away from any other
timeout in the same class, given randomness is added to the delay
and separation */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCH_AddTimeoutInClass(
    mut min_delay: libc::c_double,
    mut separation: libc::c_double,
    mut randomness: libc::c_double,
    mut class: SCH_TimeoutClass,
    mut handler: SCH_TimeoutHandler,
    mut arg: SCH_ArbitraryArgument,
) -> SCH_TimeoutID {
    let mut new_tqe: *mut TimerQueueEntry = 0 as *mut TimerQueueEntry;
    let mut ptr: *mut TimerQueueEntry = 0 as *mut TimerQueueEntry;
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut diff: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut new_min_delay: libc::c_double = 0.;
    if initialised != 0 {
    } else {
        __assert_fail(b"initialised\x00" as *const u8 as *const libc::c_char,
                      b"sched.c\x00" as *const u8 as *const libc::c_char,
                      376 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 121],
                                                &[libc::c_char; 121]>(b"SCH_TimeoutID SCH_AddTimeoutInClass(double, double, double, SCH_TimeoutClass, SCH_TimeoutHandler, SCH_ArbitraryArgument)\x00")).as_ptr());
    }
    if min_delay >= 0.0f64 {
    } else {
        __assert_fail(b"min_delay >= 0.0\x00" as *const u8 as
                          *const libc::c_char,
                      b"sched.c\x00" as *const u8 as *const libc::c_char,
                      377 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 121],
                                                &[libc::c_char; 121]>(b"SCH_TimeoutID SCH_AddTimeoutInClass(double, double, double, SCH_TimeoutClass, SCH_TimeoutHandler, SCH_ArbitraryArgument)\x00")).as_ptr());
    }
    if (class as libc::c_uint) < SCH_NumberOfClasses as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"class < SCH_NumberOfClasses\x00" as *const u8 as
                          *const libc::c_char,
                      b"sched.c\x00" as *const u8 as *const libc::c_char,
                      378 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 121],
                                                &[libc::c_char; 121]>(b"SCH_TimeoutID SCH_AddTimeoutInClass(double, double, double, SCH_TimeoutClass, SCH_TimeoutHandler, SCH_ArbitraryArgument)\x00")).as_ptr());
    }
    if randomness > 0.0f64 {
        let mut rnd: uint32_t = 0;
        UTI_GetRandomBytes(
            &mut rnd as *mut uint32_t as *mut libc::c_void,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong as libc::c_uint,
        );
        r = rnd as libc::c_double
            * (randomness / -(1 as libc::c_int) as uint32_t as libc::c_double)
            + 1.0f64;
        min_delay *= r;
        separation *= r
    }
    LCL_ReadRawTime(&mut now);
    new_min_delay = min_delay;
    /* Check the separation from the last dispatched timeout */
    diff = UTI_DiffTimespecsToDouble(
        &mut now,
        &mut *last_class_dispatch.as_mut_ptr().offset(class as isize),
    );
    if diff < separation && diff >= 0.0f64 && diff + new_min_delay < separation {
        new_min_delay = separation - diff
    }
    /* Scan through list for entries in the same class and increase min_delay
    if necessary to keep at least the separation away */
    ptr = timer_queue.next;
    while ptr != &mut timer_queue as *mut TimerQueueEntry {
        if (*ptr).class as libc::c_uint == class as libc::c_uint {
            diff = UTI_DiffTimespecsToDouble(&mut (*ptr).ts, &mut now);
            if new_min_delay > diff {
                if new_min_delay - diff < separation {
                    new_min_delay = diff + separation
                }
            } else if diff - new_min_delay < separation {
                new_min_delay = diff + separation
            }
        }
        ptr = (*ptr).next
    }
    ptr = timer_queue.next;
    while ptr != &mut timer_queue as *mut TimerQueueEntry {
        diff = UTI_DiffTimespecsToDouble(&mut (*ptr).ts, &mut now);
        if diff > new_min_delay {
            break;
        }
        ptr = (*ptr).next
    }
    /* We have located the insertion point */
    new_tqe = allocate_tqe();
    (*new_tqe).id = get_new_tqe_id();
    (*new_tqe).handler = handler;
    (*new_tqe).arg = arg;
    UTI_AddDoubleToTimespec(&mut now, new_min_delay, &mut (*new_tqe).ts);
    (*new_tqe).class = class;
    (*new_tqe).next = ptr;
    (*new_tqe).prev = (*ptr).prev;
    (*(*ptr).prev).next = new_tqe;
    (*ptr).prev = new_tqe;
    n_timer_queue_entries = n_timer_queue_entries.wrapping_add(1);
    return (*new_tqe).id;
}
/* The next one probably ought to return a status code */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCH_RemoveTimeout(mut id: SCH_TimeoutID) {
    let mut ptr: *mut TimerQueueEntry = 0 as *mut TimerQueueEntry;
    if initialised != 0 {
    } else {
        __assert_fail(
            b"initialised\x00" as *const u8 as *const libc::c_char,
            b"sched.c\x00" as *const u8 as *const libc::c_char,
            447 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void SCH_RemoveTimeout(SCH_TimeoutID)\x00",
            ))
            .as_ptr(),
        );
    }
    if id == 0 {
        return;
    }
    ptr = timer_queue.next;
    while ptr != &mut timer_queue as *mut TimerQueueEntry {
        if (*ptr).id == id {
            /* Found the required entry */
            /* Unlink from the queue */
            (*(*ptr).next).prev = (*ptr).prev;
            (*(*ptr).prev).next = (*ptr).next;
            /* Decrement entry count */
            n_timer_queue_entries = n_timer_queue_entries.wrapping_sub(1);
            /* Release memory back to the operating system */
            release_tqe(ptr);
            return;
        }
        ptr = (*ptr).next
    }
    /* Catch calls with invalid non-zero ID */
    __assert_fail(
        b"0\x00" as *const u8 as *const libc::c_char,
        b"sched.c\x00" as *const u8 as *const libc::c_char,
        472 as libc::c_int as libc::c_uint,
        (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
            b"void SCH_RemoveTimeout(SCH_TimeoutID)\x00",
        ))
        .as_ptr(),
    );
}
/* ================================================== */
/* Try to dispatch any timeouts that have already gone by, and
keep going until all are done.  (The earlier ones may take so
long to do that the later ones come around by the time they are
completed). */
unsafe extern "C" fn dispatch_timeouts(mut now: *mut timespec) {
    let mut ptr: *mut TimerQueueEntry = 0 as *mut TimerQueueEntry;
    let mut handler: SCH_TimeoutHandler = None;
    let mut arg: SCH_ArbitraryArgument = 0 as *mut libc::c_void;
    let mut n_done: libc::c_int = 0 as libc::c_int;
    let mut n_entries_on_start: libc::c_int = n_timer_queue_entries as libc::c_int;
    loop {
        LCL_ReadRawTime(now);
        if !(n_timer_queue_entries > 0 as libc::c_int as libc::c_ulong
            && UTI_CompareTimespecs(now, &mut (*timer_queue.next).ts) >= 0 as libc::c_int)
        {
            break;
        }
        ptr = timer_queue.next;
        last_class_dispatch[(*ptr).class as usize] = *now;
        handler = (*ptr).handler;
        arg = (*ptr).arg;
        SCH_RemoveTimeout((*ptr).id);
        /* Dispatch the handler */
        handler.expect("non-null function pointer")(arg);
        /* Increment count of timeouts handled */
        n_done += 1;
        /* If more timeouts were handled than there were in the timer queue on
        start and there are now, assume some code is scheduling timeouts with
        negative delays and abort.  Make the actual limit higher in case the
        machine is temporarily overloaded and dispatching the handlers takes
        more time than was delay of a scheduled timeout. */
        if n_done as libc::c_ulong
            > n_timer_queue_entries.wrapping_mul(4 as libc::c_int as libc::c_ulong)
            && n_done > n_entries_on_start * 4 as libc::c_int
        {
            LOG_Message(
                LOGS_FATAL,
                b"Possible infinite loop in scheduling\x00" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
}
/* ================================================== */
/* nfd is the number of bits set in all fd_sets */
unsafe extern "C" fn dispatch_filehandlers(
    mut nfd: libc::c_int,
    mut read_fds: *mut fd_set,
    mut write_fds: *mut fd_set,
    mut except_fds: *mut fd_set,
) {
    let mut ptr: *mut FileHandlerEntry = 0 as *mut FileHandlerEntry;
    let mut fd: libc::c_int = 0;
    fd = 0 as libc::c_int;
    while nfd != 0 && (fd as libc::c_uint) < one_highest_fd {
        if !except_fds.is_null()
            && (*except_fds).fds_bits[(fd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                as usize]
                & ((1 as libc::c_ulong)
                    << fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as __fd_mask
                != 0 as libc::c_int as libc::c_long
        {
            /* This descriptor has an exception, dispatch its handler */
            ptr = ARR_GetElement(file_handlers, fd as libc::c_uint) as *mut FileHandlerEntry;
            if (*ptr).handler.is_some() {
                (*ptr).handler.expect("non-null function pointer")(
                    fd,
                    4 as libc::c_int,
                    (*ptr).arg,
                );
            }
            nfd -= 1;
            /* Don't try to read from it now */
            if !read_fds.is_null()
                && (*read_fds).fds_bits[(fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as usize]
                    & ((1 as libc::c_ulong)
                        << fd
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
            {
                (*read_fds).fds_bits[(fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as usize] &= !(((1 as libc::c_ulong)
                    << fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as __fd_mask);
                nfd -= 1
            }
        }
        if !read_fds.is_null()
            && (*read_fds).fds_bits[(fd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                as usize]
                & ((1 as libc::c_ulong)
                    << fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as __fd_mask
                != 0 as libc::c_int as libc::c_long
        {
            /* This descriptor can be read from, dispatch its handler */
            ptr = ARR_GetElement(file_handlers, fd as libc::c_uint) as *mut FileHandlerEntry;
            if (*ptr).handler.is_some() {
                (*ptr).handler.expect("non-null function pointer")(
                    fd,
                    1 as libc::c_int,
                    (*ptr).arg,
                );
            }
            nfd -= 1
        }
        if !write_fds.is_null()
            && (*write_fds).fds_bits[(fd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                as usize]
                & ((1 as libc::c_ulong)
                    << fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as __fd_mask
                != 0 as libc::c_int as libc::c_long
        {
            /* This descriptor can be written to, dispatch its handler */
            ptr = ARR_GetElement(file_handlers, fd as libc::c_uint) as *mut FileHandlerEntry;
            if (*ptr).handler.is_some() {
                (*ptr).handler.expect("non-null function pointer")(
                    fd,
                    2 as libc::c_int,
                    (*ptr).arg,
                );
            }
            nfd -= 1
        }
        fd += 1
    }
}
/* ================================================== */
/* ================================================== */
unsafe extern "C" fn handle_slew(
    mut raw: *mut timespec,
    mut cooked: *mut timespec,
    mut dfreq: libc::c_double,
    mut doffset: libc::c_double,
    mut change_type: LCL_ChangeType,
    mut anything: *mut libc::c_void,
) {
    let mut ptr: *mut TimerQueueEntry = 0 as *mut TimerQueueEntry;
    let mut delta: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    if change_type as libc::c_uint != LCL_ChangeAdjust as libc::c_int as libc::c_uint {
        /* Make sure this handler is invoked first in order to not shift new timers
        added from other handlers */
        if LCL_IsFirstParameterChangeHandler(Some(
            handle_slew
                as unsafe extern "C" fn(
                    _: *mut timespec,
                    _: *mut timespec,
                    _: libc::c_double,
                    _: libc::c_double,
                    _: LCL_ChangeType,
                    _: *mut libc::c_void,
                ) -> (),
        )) != 0
        {
        } else {
            __assert_fail(b"LCL_IsFirstParameterChangeHandler(handle_slew)\x00"
                              as *const u8 as *const libc::c_char,
                          b"sched.c\x00" as *const u8 as *const libc::c_char,
                          583 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 95],
                                                    &[libc::c_char; 95]>(b"void handle_slew(struct timespec *, struct timespec *, double, double, LCL_ChangeType, void *)\x00")).as_ptr());
        }
        /* If a step change occurs, just shift all raw time stamps by the offset */
        ptr = timer_queue.next;
        while ptr != &mut timer_queue as *mut TimerQueueEntry {
            UTI_AddDoubleToTimespec(&mut (*ptr).ts, -doffset, &mut (*ptr).ts);
            ptr = (*ptr).next
        }
        i = 0 as libc::c_int;
        while i < SCH_NumberOfClasses as libc::c_int {
            UTI_AddDoubleToTimespec(
                &mut *last_class_dispatch.as_mut_ptr().offset(i as isize),
                -doffset,
                &mut *last_class_dispatch.as_mut_ptr().offset(i as isize),
            );
            i += 1
        }
        UTI_AddDoubleToTimespec(&mut last_select_ts_raw, -doffset, &mut last_select_ts_raw);
    }
    UTI_AdjustTimespec(
        &mut last_select_ts,
        cooked,
        &mut last_select_ts,
        &mut delta,
        dfreq,
        doffset,
    );
}
/* ================================================== */
unsafe extern "C" fn fill_fd_sets(
    mut read_fds: *mut *mut fd_set,
    mut write_fds: *mut *mut fd_set,
    mut except_fds: *mut *mut fd_set,
) {
    let mut handlers: *mut FileHandlerEntry = 0 as *mut FileHandlerEntry;
    let mut rd: *mut fd_set = 0 as *mut fd_set;
    let mut wr: *mut fd_set = 0 as *mut fd_set;
    let mut ex: *mut fd_set = 0 as *mut fd_set;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut events: libc::c_int = 0;
    n = ARR_GetSize(file_handlers) as libc::c_int;
    handlers = ARR_GetElements(file_handlers) as *mut FileHandlerEntry;
    ex = 0 as *mut fd_set;
    wr = ex;
    rd = wr;
    i = 0 as libc::c_int;
    while i < n {
        events = (*handlers.offset(i as isize)).events;
        if !(events == 0) {
            if events & 1 as libc::c_int != 0 {
                if rd.is_null() {
                    rd = *read_fds;
                    let mut __d0: libc::c_int = 0;
                    let mut __d1: libc::c_int = 0;
                    let fresh2 = &mut __d0;
                    let fresh3;
                    let fresh4 = &mut __d1;
                    let fresh5;
                    let fresh6 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
                    let fresh7 = &mut *(*rd)
                        .fds_bits
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize)
                        as *mut __fd_mask;
                    llvm_asm!("cld; rep; stosq" : "={cx}" (fresh3), "={di}"
                         (fresh5) : "{ax}" (0 as libc::c_int), "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh6)),
                         "1"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh4, fresh7))
                         : "memory" : "volatile");
                    c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh6, fresh3);
                    c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh7, fresh5);
                }
                (*rd).fds_bits[(i
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as usize] |= ((1 as libc::c_ulong)
                    << i % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as __fd_mask
            }
            if events & 2 as libc::c_int != 0 {
                if wr.is_null() {
                    wr = *write_fds;
                    let mut __d0_0: libc::c_int = 0;
                    let mut __d1_0: libc::c_int = 0;
                    let fresh8 = &mut __d0_0;
                    let fresh9;
                    let fresh10 = &mut __d1_0;
                    let fresh11;
                    let fresh12 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
                    let fresh13 = &mut *(*wr)
                        .fds_bits
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize)
                        as *mut __fd_mask;
                    llvm_asm!("cld; rep; stosq" : "={cx}" (fresh9), "={di}"
                         (fresh11) : "{ax}" (0 as libc::c_int), "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh8, fresh12)),
                         "1"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh10, fresh13))
                         : "memory" : "volatile");
                    c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh12, fresh9);
                    c2rust_asm_casts::AsmCast::cast_out(fresh10, fresh13, fresh11);
                }
                (*wr).fds_bits[(i
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as usize] |= ((1 as libc::c_ulong)
                    << i % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as __fd_mask
            }
            if events & 4 as libc::c_int != 0 {
                if ex.is_null() {
                    ex = *except_fds;
                    let mut __d0_1: libc::c_int = 0;
                    let mut __d1_1: libc::c_int = 0;
                    let fresh14 = &mut __d0_1;
                    let fresh15;
                    let fresh16 = &mut __d1_1;
                    let fresh17;
                    let fresh18 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
                    let fresh19 = &mut *(*ex)
                        .fds_bits
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize)
                        as *mut __fd_mask;
                    llvm_asm!("cld; rep; stosq" : "={cx}" (fresh15), "={di}"
                         (fresh17) : "{ax}" (0 as libc::c_int), "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh14, fresh18)),
                         "1"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh16, fresh19))
                         : "memory" : "volatile");
                    c2rust_asm_casts::AsmCast::cast_out(fresh14, fresh18, fresh15);
                    c2rust_asm_casts::AsmCast::cast_out(fresh16, fresh19, fresh17);
                }
                (*ex).fds_bits[(i
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as usize] |= ((1 as libc::c_ulong)
                    << i % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as __fd_mask
            }
        }
        i += 1
    }
    if rd.is_null() {
        *read_fds = 0 as *mut fd_set
    }
    if wr.is_null() {
        *write_fds = 0 as *mut fd_set
    }
    if ex.is_null() {
        *except_fds = 0 as *mut fd_set
    };
}
unsafe extern "C" fn check_current_time(
    mut prev_raw: *mut timespec,
    mut raw: *mut timespec,
    mut timeout: libc::c_int,
    mut orig_select_tv: *mut timeval,
    mut rem_select_tv: *mut timeval,
) -> libc::c_int {
    let mut elapsed_min: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut elapsed_max: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut orig_select_ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut rem_select_ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut step: libc::c_double = 0.;
    let mut elapsed: libc::c_double = 0.;
    UTI_TimevalToTimespec(orig_select_tv, &mut orig_select_ts);
    /* Get an estimate of the time spent waiting in the select() call. On some
    systems (e.g. Linux) the timeout timeval is modified to return the
    remaining time, use that information. */
    if timeout != 0 {
        elapsed_min = orig_select_ts;
        elapsed_max = elapsed_min
    } else if !rem_select_tv.is_null()
        && (*rem_select_tv).tv_sec >= 0 as libc::c_int as libc::c_long
        && (*rem_select_tv).tv_sec <= (*orig_select_tv).tv_sec
        && ((*rem_select_tv).tv_sec != (*orig_select_tv).tv_sec
            || (*rem_select_tv).tv_usec != (*orig_select_tv).tv_usec)
    {
        UTI_TimevalToTimespec(rem_select_tv, &mut rem_select_ts);
        UTI_DiffTimespecs(&mut elapsed_min, &mut orig_select_ts, &mut rem_select_ts);
        elapsed_max = elapsed_min
    } else {
        if !rem_select_tv.is_null() {
            elapsed_max = orig_select_ts
        } else {
            UTI_DiffTimespecs(&mut elapsed_max, raw, prev_raw);
        }
        UTI_ZeroTimespec(&mut elapsed_min);
    }
    if last_select_ts_raw.tv_sec + elapsed_min.tv_sec
        > (*raw).tv_sec + 10 as libc::c_int as libc::c_long
    {
        LOG_Message(
            LOGS_WARN,
            b"Backward time jump detected!\x00" as *const u8 as *const libc::c_char,
        );
    } else if ((*prev_raw).tv_sec + elapsed_max.tv_sec + 10 as libc::c_int as libc::c_long)
        < (*raw).tv_sec
    {
        LOG_Message(
            LOGS_WARN,
            b"Forward time jump detected!\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        return 1 as libc::c_int;
    }
    step = UTI_DiffTimespecsToDouble(&mut last_select_ts_raw, raw);
    elapsed = UTI_TimespecToDouble(&mut elapsed_min);
    step += elapsed;
    /* Cooked time may no longer be valid after dispatching the handlers */
    LCL_NotifyExternalTimeStep(raw, raw, step, fabs(step));
    return 0 as libc::c_int;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCH_MainLoop() {
    let mut read_fds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut write_fds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut except_fds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut p_read_fds: *mut fd_set = 0 as *mut fd_set;
    let mut p_write_fds: *mut fd_set = 0 as *mut fd_set;
    let mut p_except_fds: *mut fd_set = 0 as *mut fd_set;
    let mut status: libc::c_int = 0;
    let mut errsv: libc::c_int = 0;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut saved_tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ptv: *mut timeval = 0 as *mut timeval;
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut saved_now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut cooked: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut err: libc::c_double = 0.;
    if initialised != 0 {
    } else {
        __assert_fail(
            b"initialised\x00" as *const u8 as *const libc::c_char,
            b"sched.c\x00" as *const u8 as *const libc::c_char,
            719 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"void SCH_MainLoop(void)\x00",
            ))
            .as_ptr(),
        );
    }
    while need_to_exit == 0 {
        /* Dispatch timeouts and fill now with current raw time */
        dispatch_timeouts(&mut now);
        saved_now = now;
        /* The timeout handlers may request quit */
        if need_to_exit != 0 {
            break;
        }
        /* Check whether there is a timeout and set it up */
        if n_timer_queue_entries > 0 as libc::c_int as libc::c_ulong {
            UTI_DiffTimespecs(&mut ts, &mut (*timer_queue.next).ts, &mut now);
            if ts.tv_sec > 0 as libc::c_int as libc::c_long
                || ts.tv_nsec > 0 as libc::c_int as libc::c_long
            {
            } else {
                __assert_fail(
                    b"ts.tv_sec > 0 || ts.tv_nsec > 0\x00" as *const u8 as *const libc::c_char,
                    b"sched.c\x00" as *const u8 as *const libc::c_char,
                    733 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                        b"void SCH_MainLoop(void)\x00",
                    ))
                    .as_ptr(),
                );
            }
            UTI_TimespecToTimeval(&mut ts, &mut tv);
            ptv = &mut tv;
            saved_tv = tv
        } else {
            ptv = 0 as *mut timeval;
            saved_tv.tv_usec = 0 as libc::c_int as __suseconds_t;
            saved_tv.tv_sec = saved_tv.tv_usec
        }
        p_read_fds = &mut read_fds;
        p_write_fds = &mut write_fds;
        p_except_fds = &mut except_fds;
        fill_fd_sets(&mut p_read_fds, &mut p_write_fds, &mut p_except_fds);
        /* if there are no file descriptors being waited on and no
        timeout set, this is clearly ridiculous, so stop the run */
        if ptv.is_null() && p_read_fds.is_null() && p_write_fds.is_null() {
            LOG_Message(
                LOGS_FATAL,
                b"Nothing to do\x00" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        status = select(
            one_highest_fd as libc::c_int,
            p_read_fds,
            p_write_fds,
            p_except_fds,
            ptv,
        );
        errsv = *__errno_location();
        LCL_ReadRawTime(&mut now);
        LCL_CookTime(&mut now, &mut cooked, &mut err);
        /* Check if the time didn't jump unexpectedly */
        if check_current_time(
            &mut saved_now,
            &mut now,
            (status == 0 as libc::c_int) as libc::c_int,
            &mut saved_tv,
            ptv,
        ) == 0
        {
            /* Cook the time again after handling the step */
            LCL_CookTime(&mut now, &mut cooked, &mut err);
        }
        last_select_ts_raw = now;
        last_select_ts = cooked;
        last_select_ts_err = err;
        if status < 0 as libc::c_int {
            if need_to_exit == 0 && errsv != 4 as libc::c_int {
                LOG_Message(
                    LOGS_FATAL,
                    b"select() failed : %s\x00" as *const u8 as *const libc::c_char,
                    strerror(errsv),
                );
                exit(1 as libc::c_int);
            }
        } else if status > 0 as libc::c_int {
            /* A file descriptor is ready for input or output */
            dispatch_filehandlers(status, p_read_fds, p_write_fds, p_except_fds);
        } else if !ptv.is_null() {
        } else {
            __assert_fail(
                b"ptv\x00" as *const u8 as *const libc::c_char,
                b"sched.c\x00" as *const u8 as *const libc::c_char,
                779 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                    b"void SCH_MainLoop(void)\x00",
                ))
                .as_ptr(),
            );
        }
    }
}
/* No descriptors readable, timeout must have elapsed.
Therefore, tv must be non-null */
/* There's nothing to do here, since the timeouts
will be dispatched at the top of the next loop
cycle */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn SCH_QuitProgram() {
    need_to_exit = 1 as libc::c_int;
}
/* ================================================== */
