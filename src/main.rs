#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(llvm_asm, const_raw_ptr_to_usize_cast, extern_types, label_break_value, main,
           register_tool, ptr_wrapping_offset_from, c_variadic)]

use anyhow::{Result, Error};
use log::error;
use std::io::Read;

mod addrfilt;
mod array;
mod clientlog;
mod cmdmon;
mod cmdparse;
mod conf;
mod confparser;
mod confrs;
mod getdate;
mod hash_intmd5;
mod hwclock;
mod keys;
mod local;
mod logging;
mod manual;
mod memory;
mod nameserv_async;
mod nameserv;
mod ntp_core;
mod ntp_io_linux;
mod ntp_io;
mod ntp_sources;
mod pktlength;
mod refclock_phc;
mod refclock_pps;
mod refclock;
mod refclock_shm;
mod refclock_sock;
mod reference;
mod regress;
mod rtc_linux;
mod rtc;
mod samplefilt;
mod sched;
mod smooth;
mod socket;
mod sources;
mod sourcestats;
mod stubs;
mod sys_generic;
mod sys_linux;
mod sys_null;
mod sys_posix;
mod sys;
mod sys_timex;
mod tempcomp;
mod util;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
              __shortopts: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    fn geteuid() -> __uid_t;
    #[no_mangle]
    fn getuid() -> __uid_t;
    #[no_mangle]
    fn getsid(__pid: __pid_t) -> __pid_t;
    #[no_mangle]
    fn setsid() -> __pid_t;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    /* Exported functions */
    /* Initialisation function for the module */
    #[no_mangle]
    fn SCH_Initialise();
    /* Finalisation function for the module */
    #[no_mangle]
    fn SCH_Finalise();
    /* This queues a timeout to elapse at a given delta time relative to the current (raw) time */
    #[no_mangle]
    fn SCH_AddTimeoutByDelay(delay: libc::c_double, _: SCH_TimeoutHandler,
                             _: SCH_ArbitraryArgument) -> SCH_TimeoutID;
    #[no_mangle]
    fn SCH_MainLoop();
    #[no_mangle]
    fn SCH_QuitProgram();
    /* Routine to initialise the module (to be called once at program
   start-up) */
    #[no_mangle]
    fn LCL_Initialise();
    /* Routine to finalise the module (to be called once at end of
   run). */
    #[no_mangle]
    fn LCL_Finalise();
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

  This is the header for the file that links in the operating system-
  specific parts of the software

*/
    /* Called at the start of the run to do initialisation */
    #[no_mangle]
    fn SYS_Initialise(clock_control: libc::c_int);
    /* Called at the end of the run to do final clean-up */
    #[no_mangle]
    fn SYS_Finalise();
    /* Drop root privileges to the specified user and group */
    #[no_mangle]
    fn SYS_DropRoot(uid: uid_t, gid: gid_t);
    /* Enable a system call filter to allow only system calls
   which chronyd normally needs after initialization */
    #[no_mangle]
    fn SYS_EnableSystemCallFilter(level: libc::c_int);
    #[no_mangle]
    fn SYS_SetScheduler(SchedPriority: libc::c_int);
    #[no_mangle]
    fn SYS_LockMemory();
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
    fn HSH_Finalise();
    #[no_mangle]
    fn NIO_Initialise(family: libc::c_int);
    #[no_mangle]
    fn NIO_Finalise();
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

  Header for MS-SNTP authentication via Samba (ntp_signd) */
    /* Initialisation function */
    #[no_mangle]
    fn NSD_Initialise();
    /* Finalisation function */
    #[no_mangle]
    fn NSD_Finalise();
    /* Init and fini functions */
    #[no_mangle]
    fn NCR_Initialise();
    #[no_mangle]
    fn NCR_Finalise();
    /* Init and fini functions */
    #[no_mangle]
    fn SST_Initialise();
    /* Initialisation function */
    #[no_mangle]
    fn SRC_Initialise();
    #[no_mangle]
    fn SRC_Finalise();
    #[no_mangle]
    fn SRC_ReloadSources();
    #[no_mangle]
    fn SRC_RemoveDumpFiles();
    #[no_mangle]
    fn SRC_ActiveSources() -> libc::c_int;
    #[no_mangle]
    fn SRC_DumpSources();
    #[no_mangle]
    fn NSR_Finalise();
    #[no_mangle]
    fn SST_Finalise();
    #[no_mangle]
    fn NSR_SetSourceResolvingEndHandler(handler:
                                            NSR_SourceResolvingEndHandler);
    #[no_mangle]
    fn NSR_ResolveSources();
    #[no_mangle]
    fn NSR_StartSources();
    #[no_mangle]
    fn NSR_AutoStartSources();
    #[no_mangle]
    fn NSR_RemoveAllSources();
    #[no_mangle]
    fn NSR_Initialise();
    /* Initialisation function */
    #[no_mangle]
    fn SCK_Initialise();
    /* Finalisation function */
    #[no_mangle]
    fn SCK_Finalise();
    /* Init function */
    #[no_mangle]
    fn REF_Initialise();
    /* Fini function */
    #[no_mangle]
    fn REF_Finalise();
    /* Set reference update mode */
    #[no_mangle]
    fn REF_SetMode(mode: REF_Mode);
    /* Get reference update mode */
    #[no_mangle]
    fn REF_GetMode() -> REF_Mode;
    /* Set the handler for being notified of mode ending */
    #[no_mangle]
    fn REF_SetModeEndHandler(handler: REF_ModeEndHandler);
    /* Mark the local clock as unsynchronised */
    #[no_mangle]
    fn REF_SetUnsynchronised();
    /* Return the current stratum of this host or 16 if the host is not
   synchronised */
    #[no_mangle]
    fn REF_GetOurStratum() -> libc::c_int;
    /* Init function */
    #[no_mangle]
    fn LOG_Initialise();
    /* Fini function */
    #[no_mangle]
    fn LOG_Finalise();
    /* Line logging function */
    #[no_mangle]
    fn LOG_Message(severity: LOG_Severity, format: *const libc::c_char,
                   _: ...);
    /* Set the minimum severity of a message to be logged or printed to terminal.
   If the severity is LOGS_DEBUG and DEBUG is enabled, all messages will be
   prefixed with the filename, line number, and function name. */
    #[no_mangle]
    fn LOG_SetMinSeverity(severity: LOG_Severity);
    /* Log messages to a file instead of stderr, or stderr again if NULL */
    #[no_mangle]
    fn LOG_OpenFileLog(log_file: *const libc::c_char);
    /* Log messages to syslog instead of stderr */
    #[no_mangle]
    fn LOG_OpenSystemLog();
    /* Stop using stderr and send fatal message to the foreground process */
    #[no_mangle]
    fn LOG_SetParentFd(fd: libc::c_int);
    /* Close the pipe to the foreground process so it can exit */
    #[no_mangle]
    fn LOG_CloseParentFd();
    /*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2013-2014
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

  Header file for configuration module
  */
    #[no_mangle]
    fn CNF_Initialise(restarted: libc::c_int, client_only: libc::c_int);
    #[no_mangle]
    fn CNF_Finalise();
    #[no_mangle]
    fn CNF_ReadFile(filename: *const libc::c_char);
    #[no_mangle]
    fn CNF_ParseLine(filename: *const libc::c_char, number: libc::c_int,
                     line: *mut libc::c_char);
    #[no_mangle]
    fn CNF_CreateDirs(uid: uid_t, gid: gid_t);
    #[no_mangle]
    fn CNF_AddInitSources();
    #[no_mangle]
    fn CNF_AddSources();
    #[no_mangle]
    fn CNF_AddBroadcasts();
    #[no_mangle]
    fn CNF_GetDumpDir() -> *mut libc::c_char;
    #[no_mangle]
    fn CNF_SetupAccessRestrictions();
    #[no_mangle]
    fn CNF_GetSchedPriority() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetLockMemory() -> libc::c_int;
    #[no_mangle]
    fn CNF_GetUser() -> *mut libc::c_char;
    #[no_mangle]
    fn CNF_GetInitSources() -> libc::c_int;
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

  Header file for the control and monitoring module in the software
  */
    #[no_mangle]
    fn CAM_Initialise(family: libc::c_int);
    #[no_mangle]
    fn CAM_Finalise();
    #[no_mangle]
    fn CAM_OpenUnixSocket();
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

  Header for key management module
  */
    #[no_mangle]
    fn KEY_Initialise();
    #[no_mangle]
    fn KEY_Finalise();
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

  Header file for manual time input module.

  */
    #[no_mangle]
    fn MNL_Initialise();
    #[no_mangle]
    fn MNL_Finalise();
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

  */
    #[no_mangle]
    fn RTC_Initialise(initial_set: libc::c_int);
    #[no_mangle]
    fn RTC_Finalise();
    #[no_mangle]
    fn RTC_TimeInit(after_hook:
                        Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> ()>, anything: *mut libc::c_void);
    #[no_mangle]
    fn RTC_StartMeasurements();
    #[no_mangle]
    fn RCL_Initialise();
    #[no_mangle]
    fn RCL_Finalise();
    #[no_mangle]
    fn RCL_StartRefclocks();
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
    #[no_mangle]
    fn CLG_Initialise();
    #[no_mangle]
    fn CLG_Finalise();
    /* Resolve names only to selected address family */
    #[no_mangle]
    fn DNS_SetAddressFamily(family: libc::c_int);
    /*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2015
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

  This module implements time smoothing.
  */
    #[no_mangle]
    fn SMT_Initialise();
    #[no_mangle]
    fn SMT_Finalise();
    /*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2011
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

  Header file for temperature compensation.

  */
    #[no_mangle]
    fn TMC_Initialise();
    #[no_mangle]
    fn TMC_Finalise();
    #[no_mangle]
    fn UTI_SetQuitSignalsHandler(handler:
                                     Option<unsafe extern "C" fn(_:
                                                                     libc::c_int)
                                                -> ()>,
                                 ignore_sigpipe: libc::c_int);
    #[no_mangle]
    fn UTI_OpenFile(basedir: *const libc::c_char, name: *const libc::c_char,
                    suffix: *const libc::c_char, mode: libc::c_char,
                    perm: mode_t) -> *mut FILE;
    #[no_mangle]
    fn UTI_RemoveFile(basedir: *const libc::c_char, name: *const libc::c_char,
                      suffix: *const libc::c_char) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type mode_t = __mode_t;
pub type gid_t = __gid_t;
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
pub type int32_t = __int32_t;
pub type uid_t = __uid_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub type NSR_SourceResolvingEndHandler = Option<unsafe extern "C" fn() -> ()>;
pub type REF_Mode = libc::c_uint;
pub const REF_ModeIgnore: REF_Mode = 4;
pub const REF_ModePrintOnce: REF_Mode = 3;
pub const REF_ModeUpdateOnce: REF_Mode = 2;
pub const REF_ModeInitStepSlew: REF_Mode = 1;
pub const REF_ModeNormal: REF_Mode = 0;
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
pub type SCH_ArbitraryArgument = *mut libc::c_void;
pub type SCH_TimeoutHandler
    =
    Option<unsafe extern "C" fn(_: SCH_ArbitraryArgument) -> ()>;
/* Function type for handlers to be called back when mode ends */
pub type REF_ModeEndHandler
    =
    Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) John G. Hasler  2009
 * Copyright (C) Miroslav Lichvar  2012-2018
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

  The main program
  */
/* ================================================== */
/* Set when the initialisation chain has been completed.  Prevents finalisation
 * chain being run if a fatal error happened early. */
static mut initialised: libc::c_int = 0 as libc::c_int;
static mut exit_status: libc::c_int = 0 as libc::c_int;
static mut reload: libc::c_int = 0 as libc::c_int;
static mut ref_mode: REF_Mode = REF_ModeNormal;
/* ================================================== */
unsafe extern "C" fn do_platform_checks() {
    /* Require at least 32-bit integers, two's complement representation and
     the usual implementation of conversion of unsigned integers */
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >=
           4 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"sizeof (int) >= 4\x00" as *const u8 as
                          *const libc::c_char,
                      b"main.c\x00" as *const u8 as *const libc::c_char,
                      79 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"void do_platform_checks(void)\x00")).as_ptr());
    }
    if -(1 as libc::c_int) == !(0 as libc::c_int) {
    } else {
        __assert_fail(b"-1 == ~0\x00" as *const u8 as *const libc::c_char,
                      b"main.c\x00" as *const u8 as *const libc::c_char,
                      80 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"void do_platform_checks(void)\x00")).as_ptr());
    }
    if 4294967295 as libc::c_uint as int32_t == -(1 as libc::c_int) {
    } else {
        __assert_fail(b"(int32_t)4294967295U == (int32_t)-1\x00" as *const u8
                          as *const libc::c_char,
                      b"main.c\x00" as *const u8 as *const libc::c_char,
                      81 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"void do_platform_checks(void)\x00")).as_ptr());
    };
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

  Header file for main routine
  */
/* Function to clean up at end of run */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn MAI_CleanupAndExit() {
    if initialised == 0 { exit(exit_status); }
    if *CNF_GetDumpDir().offset(0 as libc::c_int as isize) as libc::c_int !=
           '\u{0}' as i32 {
        SRC_DumpSources();
    }
    /* Don't update clock when removing sources */
    REF_SetMode(REF_ModeIgnore);
    SMT_Finalise();
    TMC_Finalise();
    MNL_Finalise();
    CLG_Finalise();
    NSD_Finalise();
    NSR_Finalise();
    SST_Finalise();
    NCR_Finalise();
    NIO_Finalise();
    CAM_Finalise();
    SCK_Finalise();
    KEY_Finalise();
    RCL_Finalise();
    SRC_Finalise();
    REF_Finalise();
    RTC_Finalise();
    SYS_Finalise();
    SCH_Finalise();
    LCL_Finalise();
    CNF_Finalise();
    HSH_Finalise();
    LOG_Finalise();
    exit(exit_status);
}
/* ================================================== */
unsafe extern "C" fn signal_cleanup(mut x: libc::c_int) {
    if initialised == 0 { exit(0 as libc::c_int); }
    SCH_QuitProgram();
}
/* ================================================== */
unsafe extern "C" fn quit_timeout(mut arg: *mut libc::c_void) {
    /* Return with non-zero status if the clock is not synchronised */
    exit_status = (REF_GetOurStratum() >= 16 as libc::c_int) as libc::c_int;
    SCH_QuitProgram();
}
/* ================================================== */
unsafe extern "C" fn ntp_source_resolving_end() {
    NSR_SetSourceResolvingEndHandler(None);
    if reload != 0 {
        /* Note, we want reload to come well after the initialisation from
       the real time clock - this gives us a fighting chance that the
       system-clock scale for the reloaded samples still has a
       semblence of validity about it. */
        SRC_ReloadSources();
    }
    SRC_RemoveDumpFiles();
    RTC_StartMeasurements();
    RCL_StartRefclocks();
    NSR_StartSources();
    NSR_AutoStartSources();
    /* Special modes can end only when sources update their reachability.
     Give up immediatelly if there are no active sources. */
    if ref_mode as libc::c_uint !=
           REF_ModeNormal as libc::c_int as libc::c_uint &&
           SRC_ActiveSources() == 0 {
        REF_SetUnsynchronised();
    };
}
/* ================================================== */
unsafe extern "C" fn post_init_ntp_hook(mut anything: *mut libc::c_void) {
    if ref_mode as libc::c_uint ==
           REF_ModeInitStepSlew as libc::c_int as libc::c_uint {
        /* Remove the initstepslew sources and set normal mode */
        NSR_RemoveAllSources();
        ref_mode = REF_ModeNormal;
        REF_SetMode(ref_mode);
    }
    /* Close the pipe to the foreground process so it can exit */
    LOG_CloseParentFd();
    CNF_AddSources();
    CNF_AddBroadcasts();
    NSR_SetSourceResolvingEndHandler(Some(ntp_source_resolving_end as
                                              unsafe extern "C" fn() -> ()));
    NSR_ResolveSources();
}
/* ================================================== */
unsafe extern "C" fn reference_mode_end(mut result: libc::c_int) {
    match ref_mode as libc::c_uint {
        0 | 2 | 3 => {
            exit_status = (result == 0) as libc::c_int;
            SCH_QuitProgram();
        }
        1 => {
            /* Switch to the normal mode, the delay is used to prevent polling
         interval shorter than the burst interval if some configured servers
         were used also for initstepslew */
            SCH_AddTimeoutByDelay(2.0f64,
                                  Some(post_init_ntp_hook as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void)
                                               -> ()),
                                  0 as *mut libc::c_void);
        }
        _ => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"main.c\x00" as *const u8 as *const libc::c_char,
                          230 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 29],
                                                    &[libc::c_char; 29]>(b"void reference_mode_end(int)\x00")).as_ptr());
        }
    };
}
/* ================================================== */
unsafe extern "C" fn post_init_rtc_hook(mut anything: *mut libc::c_void) {
    if CNF_GetInitSources() > 0 as libc::c_int {
        CNF_AddInitSources();
        NSR_StartSources();
        if REF_GetMode() as libc::c_uint !=
               REF_ModeNormal as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"REF_GetMode() != REF_ModeNormal\x00" as *const u8
                              as *const libc::c_char,
                          b"main.c\x00" as *const u8 as *const libc::c_char,
                          242 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 32],
                                                    &[libc::c_char; 32]>(b"void post_init_rtc_hook(void *)\x00")).as_ptr());
        }
        /* Wait for mode end notification */
    } else { post_init_ntp_hook(0 as *mut libc::c_void); };
}
/* ================================================== */
unsafe extern "C" fn print_help(mut progname: *const libc::c_char) {
    printf(b"Usage: %s [-4|-6] [-d] [-q|-Q] [-r] [-R] [-s] [-t TIMEOUT] [-f FILE|COMMAND...]\n\x00"
               as *const u8 as *const libc::c_char, progname);
}
/* ================================================== */
unsafe extern "C" fn print_version() {
    printf(b"chronyd (chrony) version %s (%s)\n\x00" as *const u8 as
               *const libc::c_char,
           b"DEVELOPMENT\x00" as *const u8 as *const libc::c_char,
           b"+CMDMON +NTP +REFCLOCK +RTC -PRIVDROP -SCFILTER -SIGND +ASYNCDNS -SECHASH +IPV6 -DEBUG\x00"
               as *const u8 as *const libc::c_char);
}
/* ================================================== */
unsafe extern "C" fn parse_int_arg(mut arg: *const libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if sscanf(arg, b"%d\x00" as *const u8 as *const libc::c_char,
              &mut i as *mut libc::c_int) != 1 as libc::c_int {
        LOG_Message(LOGS_FATAL,
                    b"Invalid argument %s\x00" as *const u8 as
                        *const libc::c_char, arg);
        exit(1 as libc::c_int);
    }
    return i;
}
/* ================================================== */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut conf_file: *const libc::c_char =
        b"/etc/chrony.conf\x00" as *const u8 as *const libc::c_char;
    let mut progname: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut log_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut opt: libc::c_int = 0;
    let mut debug: libc::c_int = 0 as libc::c_int;
    let mut address_family: libc::c_int = 0 as libc::c_int;
    let mut do_init_rtc: libc::c_int = 0 as libc::c_int;
    let mut restarted: libc::c_int = 0 as libc::c_int;
    let mut client_only: libc::c_int = 0 as libc::c_int;
    let mut timeout: libc::c_int = 0 as libc::c_int;
    let mut scfilter_level: libc::c_int = 0 as libc::c_int;
    let mut lock_memory: libc::c_int = 0 as libc::c_int;
    let mut sched_priority: libc::c_int = 0 as libc::c_int;
    let mut clock_control: libc::c_int = 1 as libc::c_int;
    let mut system_log: libc::c_int = 1 as libc::c_int;
    let mut log_severity: libc::c_int = LOGS_INFO as libc::c_int;
    let mut config_args: libc::c_int = 0 as libc::c_int;
    do_platform_checks();
    LOG_Initialise();
    /* Parse (undocumented) long command-line options */
    optind = 1 as libc::c_int;
    while optind < argc {
        if strcmp(b"--help\x00" as *const u8 as *const libc::c_char,
                  *argv.offset(optind as isize)) == 0 {
            print_help(progname);
            return 0 as libc::c_int
        } else {
            if strcmp(b"--version\x00" as *const u8 as *const libc::c_char,
                      *argv.offset(optind as isize)) == 0 {
                print_version();
                return 0 as libc::c_int
            }
        }
        optind += 1
    }
    optind = 1 as libc::c_int;
    loop 
         /* Parse short command-line options */
         {
        opt =
            getopt(argc, argv,
                   b"46df:F:hl:L:mP:qQrRst:u:vx\x00" as *const u8 as
                       *const libc::c_char);
        if !(opt != -(1 as libc::c_int)) { break ; }
        match opt {
            52 | 54 => {
                address_family =
                    if opt == '4' as i32 {
                        1 as libc::c_int
                    } else { 2 as libc::c_int }
            }
            100 => {
                debug += 1;
                system_log = 0 as libc::c_int
            }
            102 => { conf_file = optarg }
            70 => { scfilter_level = parse_int_arg(optarg) }
            108 => { log_file = optarg }
            76 => { log_severity = parse_int_arg(optarg) }
            109 => { lock_memory = 1 as libc::c_int }
            80 => { sched_priority = parse_int_arg(optarg) }
            113 => {
                ref_mode = REF_ModeUpdateOnce;
                client_only = 0 as libc::c_int;
                system_log = 0 as libc::c_int
            }
            81 => {
                ref_mode = REF_ModePrintOnce;
                client_only = 1 as libc::c_int;
                clock_control = 0 as libc::c_int;
                system_log = 0 as libc::c_int
            }
            114 => { reload = 1 as libc::c_int }
            82 => { restarted = 1 as libc::c_int }
            115 => { do_init_rtc = 1 as libc::c_int }
            116 => { timeout = parse_int_arg(optarg) }
            117 => { user = optarg }
            118 => { print_version(); return 0 as libc::c_int }
            120 => { clock_control = 0 as libc::c_int }
            _ => {
                print_help(progname);
                return (opt != 'h' as i32) as libc::c_int
            }
        }
    }
    // TODO: logging rework here too
    if !log_file.is_null() {
        LOG_OpenFileLog(log_file);
    } else if system_log != 0 { LOG_OpenSystemLog(); }

    let log_level = if debug >= 2 {
        log::LevelFilter::Debug
    } else {
        match log_severity {
            // No fatal level in rust's log create, and it seems easier to just log fatal at error
            // too.
            LOGS_FATAL => log::LevelFilter::Error,
            LOGS_ERR => log::LevelFilter::Error,
            LOGS_WARN => log::LevelFilter::Warn,
            LOGS_INFO=> log::LevelFilter::Info,
            LOGS_DEBUG => log::LevelFilter::Debug,
            _ => panic!("invalid log level"),
        }
    };
    env_logger::builder()
        .filter_level(log_level)
        .init();
     println!("logging at {}", log_level);

    if getuid() != 0 && client_only == 0 {
        error!("Fatal error: Not superuser");
        exit(1 as libc::c_int);
    }

    if debug >= 2 {
        LOG_SetMinSeverity(debug);
    } else {
        // TODO: log
        LOG_SetMinSeverity(log_severity);
    }
    LOG_Message(LOGS_INFO,
                b"chronyd version %s starting (%s)\x00" as *const u8 as
                    *const libc::c_char,
                b"DEVELOPMENT\x00" as *const u8 as *const libc::c_char,
                b"+CMDMON +NTP +REFCLOCK +RTC -PRIVDROP -SCFILTER -SIGND +ASYNCDNS -SECHASH +IPV6 -DEBUG\x00"
                    as *const u8 as *const libc::c_char);
    DNS_SetAddressFamily(address_family);
    CNF_Initialise(restarted, client_only);
    /* Parse the config file or the remaining command line arguments */
    config_args = argc - optind;
    if config_args == 0 {
        CNF_ReadFile(conf_file);
    } else {
        while optind < argc {
            CNF_ParseLine(0 as *const libc::c_char,
                          config_args + optind - argc + 1 as libc::c_int,
                          *argv.offset(optind as isize));
            optind += 1
        }
    }
    if user.is_null() { user = CNF_GetUser() }
    pw = getpwnam(user);
    if pw.is_null() {
        LOG_Message(LOGS_FATAL,
                    b"Could not get user/group ID of %s\x00" as *const u8 as
                        *const libc::c_char, user);
        exit(1 as libc::c_int);
    }
    /* Create directories for sockets, log files, and dump files */
    CNF_CreateDirs((*pw).pw_uid, (*pw).pw_gid);
    LCL_Initialise();
    SCH_Initialise();
    SYS_Initialise(clock_control);
    RTC_Initialise(do_init_rtc);
    SRC_Initialise();
    RCL_Initialise();
    KEY_Initialise();
    SCK_Initialise();
    /* Open privileged ports before dropping root */
    CAM_Initialise(address_family);
    NIO_Initialise(address_family);
    NCR_Initialise();
    CNF_SetupAccessRestrictions();
    /* Command-line switch must have priority */
    if sched_priority == 0 { sched_priority = CNF_GetSchedPriority() }
    if sched_priority != 0 { SYS_SetScheduler(sched_priority); }
    if lock_memory != 0 || CNF_GetLockMemory() != 0 { SYS_LockMemory(); }
    /* Drop root privileges if the specified user has a non-zero UID */
    if geteuid() == 0 && ((*pw).pw_uid != 0 || (*pw).pw_gid != 0) {
        SYS_DropRoot((*pw).pw_uid, (*pw).pw_gid);
    }
    REF_Initialise();
    SST_Initialise();
    NSR_Initialise();
    NSD_Initialise();
    CLG_Initialise();
    MNL_Initialise();
    TMC_Initialise();
    SMT_Initialise();
    /* From now on, it is safe to do finalisation on exit */
    initialised = 1 as libc::c_int;
    UTI_SetQuitSignalsHandler(Some(signal_cleanup as
                                       unsafe extern "C" fn(_: libc::c_int)
                                           -> ()), 1 as libc::c_int);
    CAM_OpenUnixSocket();
    if scfilter_level != 0 { SYS_EnableSystemCallFilter(scfilter_level); }
    if ref_mode as libc::c_uint ==
           REF_ModeNormal as libc::c_int as libc::c_uint &&
           CNF_GetInitSources() > 0 as libc::c_int {
        ref_mode = REF_ModeInitStepSlew
    }
    REF_SetModeEndHandler(Some(reference_mode_end as
                                   unsafe extern "C" fn(_: libc::c_int)
                                       -> ()));
    REF_SetMode(ref_mode);
    if timeout > 0 as libc::c_int {
        SCH_AddTimeoutByDelay(timeout as libc::c_double,
                              Some(quit_timeout as
                                       unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           -> ()), 0 as *mut libc::c_void);
    }
    if do_init_rtc != 0 {
        RTC_TimeInit(Some(post_init_rtc_hook as
                              unsafe extern "C" fn(_: *mut libc::c_void)
                                  -> ()), 0 as *mut libc::c_void);
    } else { post_init_rtc_hook(0 as *mut libc::c_void); }
    /* The program normally runs under control of the main loop in
     the scheduler. */
    SCH_MainLoop();
    LOG_Message(LOGS_INFO,
                b"chronyd exiting\x00" as *const u8 as *const libc::c_char);
    MAI_CleanupAndExit();
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
/* ================================================== */
