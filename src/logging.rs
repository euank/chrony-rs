#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types,
           label_break_value, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn setvbuf(__stream: *mut FILE, __buf: *mut libc::c_char,
               __modes: libc::c_int, __n: size_t) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                 _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn gmtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn closelog();
    #[no_mangle]
    fn openlog(__ident: *const libc::c_char, __option: libc::c_int,
               __facility: libc::c_int);
    #[no_mangle]
    fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CNF_GetLogDir() -> *mut libc::c_char;
    #[no_mangle]
    fn CNF_GetLogBanner() -> libc::c_int;
    /* Open a file.  The full path of the file is constructed from the basedir
   (may be NULL), '/' (if basedir is not NULL), name, and suffix (may be NULL).
   Created files have specified permissions (umasked).  Returns NULL on error.
   The following modes are supported (if the mode is an uppercase character,
   errors are fatal):
   r/R - open an existing file for reading
   w/W - open a new file for writing (remove existing file)
   a/A - open an existing file for appending (create if does not exist) */
    #[no_mangle]
    fn UTI_OpenFile(basedir: *const libc::c_char, name: *const libc::c_char,
                    suffix: *const libc::c_char, mode: libc::c_char,
                    perm: mode_t) -> *mut FILE;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type mode_t = __mode_t;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type LOG_Severity = libc::c_int;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LogFile {
    pub name: *const libc::c_char,
    pub banner: *const libc::c_char,
    pub file: *mut FILE,
    pub writes: libc::c_ulong,
}
/* File logging functions */
pub type LOG_FileID = libc::c_int;
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2011-2014, 2018
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

  Module to handle logging of diagnostic information
  */
/* This is used by DEBUG_LOG macro */
#[no_mangle]
pub static mut log_min_severity: LOG_Severity = LOGS_INFO;
/* ================================================== */
/* Flag indicating we have initialised */
static mut initialised: libc::c_int = 0 as libc::c_int;
static mut file_log: *mut FILE = 0 as *const FILE as *mut FILE;
static mut system_log: libc::c_int = 0 as libc::c_int;
static mut parent_fd: libc::c_int = 0 as libc::c_int;
static mut n_filelogs: libc::c_int = 0 as libc::c_int;
/* Increase this when adding a new logfile */
static mut logfiles: [LogFile; 6] =
    [LogFile{name: 0 as *const libc::c_char,
             banner: 0 as *const libc::c_char,
             file: 0 as *const FILE as *mut FILE,
             writes: 0,}; 6];
/* Minimum severity of messages to be logged */
/* Init function */
/* ================================================== */
/* Init function */
#[no_mangle]
pub unsafe extern "C" fn LOG_Initialise() {
    initialised = 1 as libc::c_int;
    LOG_OpenFileLog(0 as *const libc::c_char);
}
/* Fini function */
/* ================================================== */
/* Fini function */
#[no_mangle]
pub unsafe extern "C" fn LOG_Finalise() {
    if system_log != 0 { closelog(); }
    if !file_log.is_null() { fclose(file_log); }
    LOG_CycleLogFiles();
    initialised = 0 as libc::c_int;
}
/* ================================================== */
unsafe extern "C" fn log_message(mut fatal: libc::c_int,
                                 mut severity: LOG_Severity,
                                 mut message: *const libc::c_char) {
    if system_log != 0 {
        let mut priority: libc::c_int = 0;
        match severity as libc::c_int {
            -1 => { priority = 7 as libc::c_int }
            0 => { priority = 6 as libc::c_int }
            1 => { priority = 4 as libc::c_int }
            2 => { priority = 3 as libc::c_int }
            3 => { priority = 2 as libc::c_int }
            _ => {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"logging.c\x00" as *const u8 as
                                  *const libc::c_char,
                              114 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"void log_message(int, LOG_Severity, const char *)\x00")).as_ptr());
            }
        }
        syslog(priority,
               if fatal != 0 {
                   b"Fatal error : %s\x00" as *const u8 as *const libc::c_char
               } else { b"%s\x00" as *const u8 as *const libc::c_char },
               message);
    } else if !file_log.is_null() {
        fprintf(file_log,
                if fatal != 0 {
                    b"Fatal error : %s\n\x00" as *const u8 as
                        *const libc::c_char
                } else { b"%s\n\x00" as *const u8 as *const libc::c_char },
                message);
    };
}
/* Line logging function */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LOG_Message(mut severity: LOG_Severity,
                                     mut format: *const libc::c_char,
                                     mut args: ...) {
    let mut buf: [libc::c_char; 2048] = [0; 2048];
    let mut other_args: ::std::ffi::VaListImpl;
    let mut t: time_t = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    if system_log == 0 && !file_log.is_null() &&
           severity as libc::c_int >= log_min_severity as libc::c_int {
        /* Don't clutter up syslog with timestamps and internal debugging info */
        time(&mut t);
        tm = gmtime(&mut t);
        if !tm.is_null() {
            strftime(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 2048]>() as
                         libc::c_ulong,
                     b"%Y-%m-%dT%H:%M:%SZ\x00" as *const u8 as
                         *const libc::c_char, tm);
            fprintf(file_log, b"%s \x00" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr());
        }
    }
    other_args = args.clone();
    vsnprintf(buf.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
              format, other_args.as_va_list());
    match severity as libc::c_int {
        -1 | 0 | 1 | 2 => {
            if severity as libc::c_int >= log_min_severity as libc::c_int {
                log_message(0 as libc::c_int, severity, buf.as_mut_ptr());
            }
        }
        3 => {
            if severity as libc::c_int >= log_min_severity as libc::c_int {
                log_message(1 as libc::c_int, severity, buf.as_mut_ptr());
            }
            /* Send the message also to the foreground process if it is
         still running, or stderr if it is still open */
            if parent_fd > 0 as libc::c_int {
                (write(parent_fd, buf.as_mut_ptr() as *const libc::c_void,
                       strlen(buf.as_mut_ptr()).wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)))
                    < 0 as libc::c_int as libc::c_long;
                /* Not much we can do here */
            } else if system_log != 0 && parent_fd == 0 as libc::c_int {
                system_log = 0 as libc::c_int;
                log_message(1 as libc::c_int, severity, buf.as_mut_ptr());
            }
            exit(1 as libc::c_int);
        }
        _ => {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"logging.c\x00" as *const u8 as
                              *const libc::c_char,
                          177 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 50],
                                                    &[libc::c_char; 50]>(b"void LOG_Message(LOG_Severity, const char *, ...)\x00")).as_ptr());
        }
    };
}
/* Log messages to a file instead of stderr, or stderr again if NULL */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LOG_OpenFileLog(mut log_file: *const libc::c_char) {
    let mut f: *mut FILE = 0 as *mut FILE;
    if !log_file.is_null() {
        f =
            UTI_OpenFile(0 as *const libc::c_char, log_file,
                         0 as *const libc::c_char, 'A' as i32 as libc::c_char,
                         0o644 as libc::c_int as mode_t)
    } else { f = stderr }
    /* Enable line buffering */
    setvbuf(f, 0 as *mut libc::c_char, 1 as libc::c_int,
            8192 as libc::c_int as size_t);
    if !file_log.is_null() && file_log != stderr { fclose(file_log); }
    file_log = f;
}
/* Log messages to syslog instead of stderr */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LOG_OpenSystemLog() {
    system_log = 1 as libc::c_int;
    openlog(b"chronyd\x00" as *const u8 as *const libc::c_char,
            0x1 as libc::c_int, (3 as libc::c_int) << 3 as libc::c_int);
}
/* Set the minimum severity of a message to be logged or printed to terminal.
   If the severity is LOGS_DEBUG and DEBUG is enabled, all messages will be
   prefixed with the filename, line number, and function name. */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LOG_SetMinSeverity(mut severity: LOG_Severity) {
    /* Don't print any debug messages in a non-debug build */
    log_min_severity =
        if (if 0 as libc::c_int > 0 as libc::c_int {
                LOGS_DEBUG as libc::c_int
            } else { LOGS_INFO as libc::c_int }) >
               (if (severity as libc::c_int) < LOGS_FATAL as libc::c_int {
                    severity as libc::c_int
                } else { LOGS_FATAL as libc::c_int }) {
            if 0 as libc::c_int > 0 as libc::c_int {
                LOGS_DEBUG as libc::c_int
            } else { LOGS_INFO as libc::c_int }
        } else if (severity as libc::c_int) < LOGS_FATAL as libc::c_int {
            severity as libc::c_int
        } else { LOGS_FATAL as libc::c_int } as LOG_Severity;
}
/* Stop using stderr and send fatal message to the foreground process */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LOG_SetParentFd(mut fd: libc::c_int) {
    parent_fd = fd;
    if file_log == stderr { file_log = 0 as *mut FILE };
}
/* Close the pipe to the foreground process so it can exit */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LOG_CloseParentFd() {
    if parent_fd > 0 as libc::c_int { close(parent_fd); }
    parent_fd = -(1 as libc::c_int);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LOG_FileOpen(mut name: *const libc::c_char,
                                      mut banner: *const libc::c_char)
 -> LOG_FileID {
    if n_filelogs < 6 as libc::c_int {
    } else {
        __assert_fail(b"n_filelogs < MAX_FILELOGS\x00" as *const u8 as
                          *const libc::c_char,
                      b"logging.c\x00" as *const u8 as *const libc::c_char,
                      246 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 52],
                                                &[libc::c_char; 52]>(b"LOG_FileID LOG_FileOpen(const char *, const char *)\x00")).as_ptr());
    }
    logfiles[n_filelogs as usize].name = name;
    logfiles[n_filelogs as usize].banner = banner;
    logfiles[n_filelogs as usize].file = 0 as *mut FILE;
    logfiles[n_filelogs as usize].writes = 0 as libc::c_int as libc::c_ulong;
    let fresh0 = n_filelogs;
    n_filelogs = n_filelogs + 1;
    return fresh0;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LOG_FileWrite(mut id: LOG_FileID,
                                       mut format: *const libc::c_char,
                                       mut args: ...) {
    let mut other_args: ::std::ffi::VaListImpl;
    let mut banner: libc::c_int = 0;
    if id < 0 as libc::c_int || id >= n_filelogs ||
           logfiles[id as usize].name.is_null() {
        return
    }
    if logfiles[id as usize].file.is_null() {
        let mut logdir: *mut libc::c_char = CNF_GetLogDir();
        if *logdir.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\u{0}' as i32 {
            LOG_Message(LOGS_WARN,
                        b"logdir not specified\x00" as *const u8 as
                            *const libc::c_char);
            logfiles[id as usize].name = 0 as *const libc::c_char;
            return
        }
        logfiles[id as usize].file =
            UTI_OpenFile(logdir, logfiles[id as usize].name,
                         b".log\x00" as *const u8 as *const libc::c_char,
                         'a' as i32 as libc::c_char,
                         0o644 as libc::c_int as mode_t);
        if logfiles[id as usize].file.is_null() {
            /* Disable the log */
            logfiles[id as usize].name = 0 as *const libc::c_char;
            return
        }
    }
    banner = CNF_GetLogBanner();
    if banner != 0 &&
           {
               let fresh1 = logfiles[id as usize].writes;
               logfiles[id as usize].writes =
                   logfiles[id as usize].writes.wrapping_add(1);
               (fresh1.wrapping_rem(banner as libc::c_ulong)) ==
                   0 as libc::c_int as libc::c_ulong
           } {
        let mut bannerline: [libc::c_char; 256] = [0; 256];
        let mut i: libc::c_int = 0;
        let mut bannerlen: libc::c_int = 0;
        bannerlen =
            if strlen(logfiles[id as usize].banner) <
                   (::std::mem::size_of::<[libc::c_char; 256]>() as
                        libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong) {
                strlen(logfiles[id as usize].banner)
            } else {
                (::std::mem::size_of::<[libc::c_char; 256]>() as
                     libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong)
            } as libc::c_int;
        i = 0 as libc::c_int;
        while i < bannerlen {
            bannerline[i as usize] = '=' as i32 as libc::c_char;
            i += 1
        }
        bannerline[i as usize] = '\u{0}' as i32 as libc::c_char;
        fprintf(logfiles[id as usize].file,
                b"%s\n\x00" as *const u8 as *const libc::c_char,
                bannerline.as_mut_ptr());
        fprintf(logfiles[id as usize].file,
                b"%s\n\x00" as *const u8 as *const libc::c_char,
                logfiles[id as usize].banner);
        fprintf(logfiles[id as usize].file,
                b"%s\n\x00" as *const u8 as *const libc::c_char,
                bannerline.as_mut_ptr());
    }
    other_args = args.clone();
    vfprintf(logfiles[id as usize].file, format, other_args.as_va_list());
    fprintf(logfiles[id as usize].file,
            b"\n\x00" as *const u8 as *const libc::c_char);
    fflush(logfiles[id as usize].file);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn LOG_CycleLogFiles() {
    let mut i: LOG_FileID = 0;
    i = 0 as libc::c_int;
    while i < n_filelogs {
        if !logfiles[i as usize].file.is_null() {
            fclose(logfiles[i as usize].file);
        }
        logfiles[i as usize].file = 0 as *mut FILE;
        logfiles[i as usize].writes = 0 as libc::c_int as libc::c_ulong;
        i += 1
    };
}
/* ================================================== */
