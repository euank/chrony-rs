use std::net::IpAddr;

use ipnet::IpNet;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

pub type NTP_Remote_Address = std::net::SocketAddr;
pub enum LeapMode {
    Ignore,
    Step,
    Slew,
    System,
}

pub struct AllowDeny {
    all: bool,
    allow: bool,
    addr: ipnet::IpNet,
}

#[derive(Copy, Clone)]
pub struct NTP_Broadcast_Destination {
    pub addr: std::net::SocketAddr,
    pub interval: i64,
}

#[derive(Clone, Debug)]
pub struct RefclockParameters {
    pub driver_name: String,
    pub driver_parameter: String,
    pub driver_poll: i64,
    pub poll: i64,
    pub filter_length: i64,
    pub pps_forced: i64,
    pub pps_rate: i64,
    pub min_samples: i64,
    pub max_samples: i64,
    pub sel_options: i64,
    pub max_lock_age: i64,
    pub stratum: i64,
    pub tai: i64,
    pub ref_id: u32,
    pub lock_ref_id: i32,
    pub offset: f64,
    pub delay: f64,
    pub precision: f64,
    pub max_dispersion: f64,
    pub pulse_width: f64,
}

#[derive(Clone)]
pub struct NTP_Source {
    pub typ: NTPSourceType,
    pub pool: i64,
    pub params: CPS_NTP_Source,
}

#[derive(Clone)]
pub struct CPS_NTP_Source {
    pub name: String,
    pub port: u16,
    pub params: SourceParameters,
}

#[derive(Copy, Clone)]
pub struct SourceParameters {
    pub minpoll: i64,
    pub maxpoll: i64,
    pub connectivity: Connectivity,
    pub auto_offline: i64,
    pub presend_minpoll: i64,
    pub burst: i64,
    pub iburst: i64,
    pub min_stratum: i64,
    pub poll_target: i64,
    pub version: i64,
    pub max_sources: i64,
    pub min_samples: i64,
    pub max_samples: i64,
    pub filter_length: i64,
    pub interleaved: i64,
    pub sel_options: i64,
    pub authkey: u32,
    pub max_delay: f64,
    pub max_delay_ratio: f64,
    pub max_delay_dev_ratio: f64,
    pub min_delay: f64,
    pub asymmetry: f64,
    pub offset: f64,
}

#[derive(Copy, Clone, Debug)]
enum Connectivity {
    MaybeOnline,
    Online,
    Offline,
}

#[derive(Copy, Clone, Debug)]
pub enum NTPSourceType {
    Peer,
    Server,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CNF_HwTsInterface {
    pub name: String,
    pub minpoll: i64,
    pub min_samples: i64,
    pub max_samples: i64,
    pub nocrossts: i64,
    pub rxfilter: CNF_HwTs_RxFilter,
    pub precision: f64,
    pub tx_comp: f64,
    pub rx_comp: f64,
}
pub type CNF_HwTs_RxFilter = u64;
pub const CNF_HWTS_RXFILTER_ALL: CNF_HwTs_RxFilter = 3;
pub const CNF_HWTS_RXFILTER_NTP: CNF_HwTs_RxFilter = 2;
pub const CNF_HWTS_RXFILTER_NONE: CNF_HwTs_RxFilter = 1;
pub const CNF_HWTS_RXFILTER_ANY: CNF_HwTs_RxFilter = 0;
pub type LOG_Severity = i64;
pub const LOGS_FATAL: LOG_Severity = 3;
pub const LOGS_ERR: LOG_Severity = 2;
pub const LOGS_WARN: LOG_Severity = 1;
pub const LOGS_INFO: LOG_Severity = 0;
pub const LOGS_DEBUG: LOG_Severity = -1;
pub const DNS_Success: DNS_Status = 0;
pub type DNS_Status = u64;
pub const DNS_Failure: DNS_Status = 2;
pub const DNS_TryAgain: DNS_Status = 1;
pub type NSR_Status = u64;
pub const NSR_UnresolvedName: NSR_Status = 6;
pub const NSR_InvalidName: NSR_Status = 5;
pub const NSR_InvalidAF: NSR_Status = 4;
pub const NSR_TooManySources: NSR_Status = 3;
pub const NSR_AlreadyInUse: NSR_Status = 2;
pub const NSR_NoSuchSource: NSR_Status = 1;
pub const NSR_Success: NSR_Status = 0;


impl Default for Conf {
    fn default() -> Self {
        Conf{
            acquisition_port: -1,
            ntp_port: 123,
            max_update_skew: 1000.0,
            correction_time_ratio: 3.0,
            max_clock_error: 1.0,
            max_drift: 500000.0f64,
            max_slew_rate: 1e6f64 / 12.0f64,
            max_distance: 3.0f64,
            max_jitter: 1.0f64,
            reselect_distance: 1e-4f64,
            stratum_weight: 1e-3f64,
            combine_limit: 3.0f64,
            cmd_port: 323,
            log_banner: 32,
            min_sources: 1,
            max_offset_delay: -1,
            min_samples: 6,
            log_change_threshold: 1.0f64,
            client_log_limit: 524288,
            ntp_ratelimit_interval: 3,
            ntp_ratelimit_burst: 8,
            ntp_ratelimit_leak: 2,
            cmd_ratelimit_interval: -4,
            cmd_ratelimit_burst: 8,
            cmd_ratelimit_leak: 2,
            leapsec_mode: LeapMode::LeapModeSystem,
            rtc_device: "/dev/rtc".to_string(),
            user: "root".to_string(),
            bind_cmd_path: "/var/run/chrony/chronyd.sock".to_string()

            ..Default::default()
        }
    }
}


pub struct Conf {
    restarted: bool,
    rtc_device: String,
    acquisition_port: i64,
    ntp_port: i64,
    keys_file: String,
    drift_file: String,
    rtc_file: String,
    max_update_skew: f64,
    correction_time_ratio: f64,
    max_clock_error: f64,
    max_drift: f64,
    max_slew_rate: f64,
    max_distance: f64,
    max_jitter: f64,
    reselect_distance: f64,
    stratum_weight: f64,
    combine_limit: f64,
    cmd_port: i64,
    raw_measurements: i64,
    do_log_measurements: i64,
    do_log_statistics: i64,
    do_log_tracking: i64,
    do_log_rtc: i64,
    do_log_refclocks: i64,
    do_log_tempcomp: i64,
    log_banner: i64,
    logdir: String,
    dumpdir: String,
    enable_local: i64,
    local_stratum: i64,
    local_orphan: i64,
    local_distance: f64,
    /* Threshold (in seconds) - if absolute value of initial error is less
       than this, slew instead of stepping */
    init_slew_threshold: f64,
    /* Array of IPAddr */
    init_sources: Vec<IpAddr>,
    enable_manual: i64,
    /* Flag set if the RTC runs UTC (default is it runs local time
       incl. daylight saving). */
    rtc_on_utc: i64,
    /* Filename used to read the hwclock(8) LOCAL/UTC setting */
    hwclock_file: String,
    /* Flag set if the RTC should be automatically synchronised by kernel */
    rtc_sync: i64,
    /* Limit and threshold for clock stepping */
    make_step_limit: i64,
    make_step_threshold: f64,
    /* Threshold for automatic RTC trimming */
    rtc_autotrim_threshold: f64,
    /* Minimum number of selectables sources required to update the clock */
    min_sources: i64,
    /* Number of updates before offset checking, number of ignored updates
       before exiting and the maximum allowed offset */
    max_offset_delay: i64,
    max_offset_ignore: i64,
    max_offset: f64,
    /* Maximum and minimum number of samples per source */
    max_samples: i64,
    /* no limit */
    min_samples: i64,
    /* Threshold for a time adjustment to be logged to syslog */
    log_change_threshold: f64,
    mail_user_on_change: String,
    mail_change_threshold: f64,
    /* Flag indicating that we don't want to log clients, e.g. to save
       memory */
    no_client_log: i64,
    /* Limit memory allocated for the clients log */
    client_log_limit: u64,
    /* Minimum and maximum fallback drift intervals */
    fb_drift_min: i64,
    fb_drift_max: i64,
    /* IP addresses for binding the NTP server sockets to.  UNSPEC family means
       INADDR_ANY will be used */
    bind_address4: IpAddr,
    bind_address6: IpAddr,
    /* IP addresses for binding the NTP client sockets to.  UNSPEC family means
       INADDR_ANY will be used */
    bind_acq_address4: IpAddr,
    bind_acq_address6: IpAddr,
    /* IP addresses for binding the command socket to.  UNSPEC family means
       the loopback address will be used */
    bind_cmd_address4: IpAddr,
    bind_cmd_address6: IpAddr,
    /* Path to the Unix domain command socket. */
    bind_cmd_path: String,
    /* Path to Samba (ntp_signd) socket. */
    ntp_signd_socket: String,
    /* Rate limiting parameters */
    ntp_ratelimit_enabled: i64,
    ntp_ratelimit_interval: i64,
    ntp_ratelimit_burst: i64,
    ntp_ratelimit_leak: i64,
    cmd_ratelimit_enabled: i64,
    cmd_ratelimit_interval: i64,
    cmd_ratelimit_burst: i64,
    cmd_ratelimit_leak: i64,
    /* Smoothing constants */
    smooth_max_freq: f64,
    /* in ppm */
    smooth_max_wander: f64,
    /* in ppm/s */
    smooth_leap_only: i64,
    /* Temperature sensor, update interval and compensation coefficients */
    tempcomp_sensor_file: String,
    tempcomp_point_file: String,
    tempcomp_interval: f64,
    tempcomp_T0: f64,
    tempcomp_k0: f64,
    tempcomp_k1: f64,
    tempcomp_k2: f64,
    sched_priority: i64,
    lock_memory: i64,
    /* Leap second handling mode */
    leapsec_mode: LeapMode,
    /* Name of a system timezone containing leap seconds occuring at midnight */
    leapsec_tz: String,
    /* Name of the user to which will be dropped root privileges. */
    user: String,
    /* Array of CNF_HwTsInterface */
    hwts_interfaces: Vec<CNF_HwTsInterface>,
    /* Array of NTP_Source */
    ntp_sources: Vec<NTP_Source>,
    /* Array of RefclockParameters */
    refclock_sources: Vec<RefclockParameters>,
    /* Arrays of AllowDeny */
    ntp_restrictions: Vec<AllowDeny>,
    cmd_restrictions: Vec<AllowDeny>,
    /* Array of NTP_Broadcast_Destination */
    broadcasts: Vec<NTP_Broadcast_Destination>,
}

/* ================================================== */
/* The line number in the configuration file being processed */
static mut line_number: i64 = 0;
static mut processed_file: *const libc::c_char = 0 as *const libc::c_char;
static mut processed_command: *const libc::c_char = 0 as *const libc::c_char;
/* ================================================== */
unsafe extern "C" fn command_parse_error() {
    LOG_Message(LOGS_FATAL,
                b"Could not parse %s directive at line %d%s%s\x00" as
                    *const u8 as *const libc::c_char, processed_command,
                line_number,
                if !processed_file.is_null() {
                    b" in file \x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                if !processed_file.is_null() {
                    processed_file
                } else { b"\x00" as *const u8 as *const libc::c_char });
    exit(1 as i64);
}
/* ================================================== */
unsafe extern "C" fn other_parse_error(mut message: *const libc::c_char) {
    LOG_Message(LOGS_FATAL,
                b"%s at line %d%s%s\x00" as *const u8 as *const libc::c_char,
                message, line_number,
                if !processed_file.is_null() {
                    b" in file \x00" as *const u8 as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char },
                if !processed_file.is_null() {
                    processed_file
                } else { b"\x00" as *const u8 as *const libc::c_char });
    exit(1 as i64);
}
/* ================================================== */
unsafe extern "C" fn get_number_of_args(mut line: *mut libc::c_char)
 -> i64 {
    let mut num: i64 = 0 as i64;
    /* The line is normalized, between arguments is just one space */
    if *line as i64 == ' ' as i32 { line = line.offset(1) }
    if *line != 0 { num += 1 }
    while *line != 0 {
        if *line as i64 == ' ' as i32 { num += 1 }
        line = line.offset(1)
    }
    return num;
}
/* ================================================== */
unsafe extern "C" fn check_number_of_args(mut line: *mut libc::c_char,
                                          mut num: i64) {
    num -= get_number_of_args(line);
    if num != 0 {
        LOG_Message(LOGS_FATAL,
                    b"%s arguments for %s directive at line %d%s%s\x00" as
                        *const u8 as *const libc::c_char,
                    if num > 0 as i64 {
                        b"Missing\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"Too many\x00" as *const u8 as *const libc::c_char
                    }, processed_command, line_number,
                    if !processed_file.is_null() {
                        b" in file \x00" as *const u8 as *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char },
                    if !processed_file.is_null() {
                        processed_file
                    } else { b"\x00" as *const u8 as *const libc::c_char });
        exit(1 as i64);
    };
}
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
/* ================================================== */

impl Conf {
    pub fn new(restarted: bool, client_only: bool) -> Self {
        let mut conf = Conf{
            restarted,

            ..Default::default()
        };
        if client_only {
            conf.ntp_port = 0;
            conf.cmd_port = 0;
            conf.bind_cmd_path = "".to_string();
        }
        conf
    }

    // TODO
    pub fn from_file(filename: &str) -> Self {

    }
}


/* Read the configuration file */
#[no_mangle]
pub unsafe extern "C" fn CNF_ReadFile(mut filename: *const libc::c_char) {
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut line: [libc::c_char; 2048] = [0; 2048];
    let mut i: i64 = 0;
    in_0 =
        UTI_OpenFile(0 as *const libc::c_char, filename,
                     0 as *const libc::c_char, 'R' as i32 as libc::c_char,
                     0 as i64 as mode_t);
    i = 1 as i64;
    while !fgets(line.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 2048]>() as
                     libc::c_ulong as i64, in_0).is_null() {
        CNF_ParseLine(filename, i, line.as_mut_ptr());
        i += 1
    }
    fclose(in_0);
}
/* ================================================== */
/* Parse one configuration line */
#[no_mangle]
pub unsafe extern "C" fn CNF_ParseLine(mut filename: *const libc::c_char,
                                       mut number: i64,
                                       mut line: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Set global variables used in error messages */
    processed_file = filename;
    line_number = number;
    /* Remove extra white-space and comments */
    CPS_NormalizeLine(line);
    /* Skip blank lines */
    if *line == 0 { return }
    /* We have a real line, now try to match commands */
    command = line;
    processed_command = command;
    p = CPS_SplitWord(line);
    if strcasecmp(command,
                  b"acquisitionport\x00" as *const u8 as *const libc::c_char)
           == 0 {
        parse_int(p, &mut acquisition_port);
    } else if strcasecmp(command,
                         b"allow\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        parse_allow_deny(p, ntp_restrictions, 1 as i64);
    } else if strcasecmp(command,
                         b"bindacqaddress\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_bindacqaddress(p);
    } else if strcasecmp(command,
                         b"bindaddress\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_bindaddress(p);
    } else if strcasecmp(command,
                         b"bindcmdaddress\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_bindcmdaddress(p);
    } else if strcasecmp(command,
                         b"broadcast\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        parse_broadcast(p);
    } else if strcasecmp(command,
                         b"clientloglimit\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_clientloglimit(p);
    } else if strcasecmp(command,
                         b"cmdallow\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        parse_allow_deny(p, cmd_restrictions, 1 as i64);
    } else if strcasecmp(command,
                         b"cmddeny\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        parse_allow_deny(p, cmd_restrictions, 0 as i64);
    } else if strcasecmp(command,
                         b"cmdport\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        parse_int(p, &mut cmd_port);
    } else if strcasecmp(command,
                         b"cmdratelimit\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_ratelimit(p, &mut cmd_ratelimit_enabled,
                        &mut cmd_ratelimit_interval, &mut cmd_ratelimit_burst,
                        &mut cmd_ratelimit_leak);
    } else if strcasecmp(command,
                         b"combinelimit\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_double(p, &mut combine_limit);
    } else if strcasecmp(command,
                         b"corrtimeratio\x00" as *const u8 as
                             *const libc::c_char) == 0 {
        parse_double(p, &mut correction_time_ratio);
    } else if strcasecmp(command,
                         b"deny\x00" as *const u8 as *const libc::c_char) == 0
     {
        parse_allow_deny(p, ntp_restrictions, 0 as i64);
    } else if strcasecmp(command,
                         b"driftfile\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        parse_string(p, &mut drift_file);
    } else if strcasecmp(command,
                         b"dumpdir\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        parse_string(p, &mut dumpdir);
    } else if !(strcasecmp(command,
                           b"dumponexit\x00" as *const u8 as
                               *const libc::c_char) == 0) {
        if strcasecmp(command,
                      b"fallbackdrift\x00" as *const u8 as
                          *const libc::c_char) == 0 {
            parse_fallbackdrift(p);
        } else if strcasecmp(command,
                             b"hwclockfile\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut hwclock_file);
        } else if strcasecmp(command,
                             b"hwtimestamp\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_hwtimestamp(p);
        } else if strcasecmp(command,
                             b"include\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_include(p);
        } else if strcasecmp(command,
                             b"initstepslew\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_initstepslew(p);
        } else if strcasecmp(command,
                             b"keyfile\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut keys_file);
        } else if strcasecmp(command,
                             b"leapsecmode\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_leapsecmode(p);
        } else if strcasecmp(command,
                             b"leapsectz\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut leapsec_tz);
        } else if strcasecmp(command,
                             b"local\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            parse_local(p);
        } else if strcasecmp(command,
                             b"lock_all\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            lock_memory = parse_null(p)
        } else if strcasecmp(command,
                             b"log\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            parse_log(p);
        } else if strcasecmp(command,
                             b"logbanner\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_int(p, &mut log_banner);
        } else if strcasecmp(command,
                             b"logchange\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut log_change_threshold);
        } else if strcasecmp(command,
                             b"logdir\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut logdir);
        } else if strcasecmp(command,
                             b"mailonchange\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_mailonchange(p);
        } else if strcasecmp(command,
                             b"makestep\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_makestep(p);
        } else if strcasecmp(command,
                             b"manual\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            enable_manual = parse_null(p)
        } else if strcasecmp(command,
                             b"maxchange\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_maxchange(p);
        } else if strcasecmp(command,
                             b"maxclockerror\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut max_clock_error);
        } else if strcasecmp(command,
                             b"maxdistance\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut max_distance);
        } else if strcasecmp(command,
                             b"maxdrift\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut max_drift);
        } else if strcasecmp(command,
                             b"maxjitter\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut max_jitter);
        } else if strcasecmp(command,
                             b"maxsamples\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_int(p, &mut max_samples);
        } else if strcasecmp(command,
                             b"maxslewrate\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut max_slew_rate);
        } else if strcasecmp(command,
                             b"maxupdateskew\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut max_update_skew);
        } else if strcasecmp(command,
                             b"minsamples\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_int(p, &mut min_samples);
        } else if strcasecmp(command,
                             b"minsources\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_int(p, &mut min_sources);
        } else if strcasecmp(command,
                             b"noclientlog\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            no_client_log = parse_null(p)
        } else if strcasecmp(command,
                             b"ntpsigndsocket\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut ntp_signd_socket);
        } else if strcasecmp(command,
                             b"peer\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            parse_source(p, NTP_PEER, 0 as i64);
        } else if strcasecmp(command,
                             b"pool\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            parse_source(p, NTP_SERVER, 1 as i64);
        } else if strcasecmp(command,
                             b"port\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            parse_int(p, &mut ntp_port);
        } else if strcasecmp(command,
                             b"ratelimit\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_ratelimit(p, &mut ntp_ratelimit_enabled,
                            &mut ntp_ratelimit_interval,
                            &mut ntp_ratelimit_burst,
                            &mut ntp_ratelimit_leak);
        } else if strcasecmp(command,
                             b"refclock\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_refclock(p);
        } else if strcasecmp(command,
                             b"reselectdist\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut reselect_distance);
        } else if strcasecmp(command,
                             b"rtcautotrim\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut rtc_autotrim_threshold);
        } else if strcasecmp(command,
                             b"rtcdevice\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut rtc_device);
        } else if strcasecmp(command,
                             b"rtcfile\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_string(p, &mut rtc_file);
        } else if strcasecmp(command,
                             b"rtconutc\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            rtc_on_utc = parse_null(p)
        } else if strcasecmp(command,
                             b"rtcsync\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            rtc_sync = parse_null(p)
        } else if strcasecmp(command,
                             b"sched_priority\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_int(p, &mut sched_priority);
        } else if strcasecmp(command,
                             b"server\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_source(p, NTP_SERVER, 0 as i64);
        } else if strcasecmp(command,
                             b"smoothtime\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_smoothtime(p);
        } else if strcasecmp(command,
                             b"stratumweight\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_double(p, &mut stratum_weight);
        } else if strcasecmp(command,
                             b"tempcomp\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            parse_tempcomp(p);
        } else if strcasecmp(command,
                             b"user\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            parse_string(p, &mut user);
        } else if strcasecmp(command,
                             b"commandkey\x00" as *const u8 as
                                 *const libc::c_char) == 0 ||
                      strcasecmp(command,
                                 b"generatecommandkey\x00" as *const u8 as
                                     *const libc::c_char) == 0 ||
                      strcasecmp(command,
                                 b"linux_freq_scale\x00" as *const u8 as
                                     *const libc::c_char) == 0 ||
                      strcasecmp(command,
                                 b"linux_hz\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
            LOG_Message(LOGS_WARN,
                        b"%s directive is no longer supported\x00" as
                            *const u8 as *const libc::c_char, command);
        } else {
            other_parse_error(b"Invalid command\x00" as *const u8 as
                                  *const libc::c_char);
        }
    };
}
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2009-2017
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

  Module that reads and processes the configuration file.
  */
/* ================================================== */
/* Forward prototypes */
/* ================================================== */
unsafe extern "C" fn parse_string(mut line: *mut libc::c_char,
                                  mut result: *mut *mut libc::c_char)
 -> i64 {
    check_number_of_args(line, 1 as i64);
    free(*result as *mut libc::c_void);
    *result = Strdup(line);
    return 1 as i64;
}
/* ================================================== */
unsafe extern "C" fn parse_int(mut line: *mut libc::c_char,
                               mut result: *mut i64) -> i64 {
    check_number_of_args(line, 1 as i64);
    if sscanf(line, b"%d\x00" as *const u8 as *const libc::c_char, result) !=
           1 as i64 {
        command_parse_error();
        return 0 as i64
    }
    return 1 as i64;
}
/* ================================================== */
unsafe extern "C" fn parse_double(mut line: *mut libc::c_char,
                                  mut result: *mut f64)
 -> i64 {
    check_number_of_args(line, 1 as i64);
    if sscanf(line, b"%lf\x00" as *const u8 as *const libc::c_char, result) !=
           1 as i64 {
        command_parse_error();
        return 0 as i64
    }
    return 1 as i64;
}
/* ================================================== */
unsafe extern "C" fn parse_null(mut line: *mut libc::c_char) -> i64 {
    check_number_of_args(line, 0 as i64);
    return 1 as i64;
}
/* ================================================== */
unsafe extern "C" fn parse_source(mut line: *mut libc::c_char,
                                  mut type_0: NTP_Source_Type,
                                  mut pool: i64) {
    let mut source: NTP_Source =
        NTP_Source{type_0: NTP_SERVER,
                   pool: 0,
                   params:
                       CPS_NTP_Source{name: 0 as *mut libc::c_char,
                                      port: 0,
                                      params:
                                          SourceParameters{minpoll: 0,
                                                           maxpoll: 0,
                                                           connectivity:
                                                               SRC_OFFLINE,
                                                           auto_offline: 0,
                                                           presend_minpoll: 0,
                                                           burst: 0,
                                                           iburst: 0,
                                                           min_stratum: 0,
                                                           poll_target: 0,
                                                           version: 0,
                                                           max_sources: 0,
                                                           min_samples: 0,
                                                           max_samples: 0,
                                                           filter_length: 0,
                                                           interleaved: 0,
                                                           sel_options: 0,
                                                           authkey: 0,
                                                           max_delay: 0.,
                                                           max_delay_ratio:
                                                               0.,
                                                           max_delay_dev_ratio:
                                                               0.,
                                                           min_delay: 0.,
                                                           asymmetry: 0.,
                                                           offset: 0.,},},};
    source.type_0 = type_0;
    source.pool = pool;
    if CPS_ParseNTPSourceAdd(line, &mut source.params) == 0 {
        command_parse_error();
        return
    }
    source.params.name = Strdup(source.params.name);
    ARR_AppendElement(ntp_sources,
                      &mut source as *mut NTP_Source as *mut libc::c_void);
}
/* ================================================== */
unsafe extern "C" fn parse_ratelimit(mut line: *mut libc::c_char,
                                     mut enabled: *mut i64,
                                     mut interval: *mut i64,
                                     mut burst: *mut i64,
                                     mut leak: *mut i64) {
    let mut n: i64 = 0;
    let mut val: i64 = 0;
    let mut opt: *mut libc::c_char = 0 as *mut libc::c_char;
    *enabled = 1 as i64;
    while *line != 0 {
        opt = line;
        line = CPS_SplitWord(line);
        if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                  &mut val as *mut i64, &mut n as *mut i64) !=
               1 as i64 {
            command_parse_error();
            return
        }
        line = line.offset(n as isize);
        if strcasecmp(opt,
                      b"interval\x00" as *const u8 as *const libc::c_char) ==
               0 {
            *interval = val
        } else if strcasecmp(opt,
                             b"burst\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            *burst = val
        } else if strcasecmp(opt,
                             b"leak\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            *leak = val
        } else { command_parse_error(); }
    };
}
/* ================================================== */
unsafe extern "C" fn parse_refclock(mut line: *mut libc::c_char) {
    let mut n: i64 = 0;
    let mut poll: i64 = 0;
    let mut dpoll: i64 = 0;
    let mut filter_length: i64 = 0;
    let mut pps_rate: i64 = 0;
    let mut min_samples_0: i64 = 0;
    let mut max_samples_0: i64 = 0;
    let mut sel_options: i64 = 0;
    let mut max_lock_age: i64 = 0;
    let mut pps_forced: i64 = 0;
    let mut stratum: i64 = 0;
    let mut tai: i64 = 0;
    let mut ref_id: uint32_t = 0;
    let mut lock_ref_id: uint32_t = 0;
    let mut offset: f64 = 0.;
    let mut delay: f64 = 0.;
    let mut precision: f64 = 0.;
    let mut max_dispersion: f64 = 0.;
    let mut pulse_width: f64 = 0.;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut param: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ref_0: [libc::c_uchar; 5] = [0; 5];
    let mut refclock: *mut RefclockParameters = 0 as *mut RefclockParameters;
    poll = 4 as i64;
    dpoll = 0 as i64;
    filter_length = 64 as i64;
    pps_forced = 0 as i64;
    pps_rate = 0 as i64;
    min_samples_0 = -(1 as i64);
    max_samples_0 = -(1 as i64);
    sel_options = 0 as i64;
    offset = 0.0f64;
    delay = 1e-9f64;
    precision = 0.0f64;
    max_dispersion = 0.0f64;
    pulse_width = 0.0f64;
    ref_id = 0 as i64 as uint32_t;
    max_lock_age = 2 as i64;
    lock_ref_id = 0 as i64 as uint32_t;
    stratum = 0 as i64;
    tai = 0 as i64;
    if *line == 0 { command_parse_error(); return }
    p = line;
    line = CPS_SplitWord(line);
    if *line == 0 { command_parse_error(); return }
    name = Strdup(p);
    p = line;
    line = CPS_SplitWord(line);
    param = Strdup(p);
    cmd = line;
    while *cmd != 0 {
        line = CPS_SplitWord(line);
        if strcasecmp(cmd, b"refid\x00" as *const u8 as *const libc::c_char)
               == 0 {
            if sscanf(line, b"%4s%n\x00" as *const u8 as *const libc::c_char,
                      ref_0.as_mut_ptr() as *mut libc::c_char,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
            ref_id =
                (ref_0[0 as i64 as usize] as uint32_t) <<
                    24 as i64 |
                    ((ref_0[1 as i64 as usize] as i64) <<
                         16 as i64) as u64 |
                    ((ref_0[2 as i64 as usize] as i64) <<
                         8 as i64) as u64 |
                    ref_0[3 as i64 as usize] as u64
        } else if strcasecmp(cmd,
                             b"lock\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%4s%n\x00" as *const u8 as *const libc::c_char,
                      ref_0.as_mut_ptr() as *mut libc::c_char,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
            lock_ref_id =
                (ref_0[0 as i64 as usize] as uint32_t) <<
                    24 as i64 |
                    ((ref_0[1 as i64 as usize] as i64) <<
                         16 as i64) as u64 |
                    ((ref_0[2 as i64 as usize] as i64) <<
                         8 as i64) as u64 |
                    ref_0[3 as i64 as usize] as u64
        } else if strcasecmp(cmd,
                             b"poll\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut poll as *mut i64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"dpoll\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut dpoll as *mut i64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"filter\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut filter_length as *mut i64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"rate\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut pps_rate as *mut i64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"minsamples\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut min_samples_0 as *mut i64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"maxlockage\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut max_lock_age as *mut i64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"maxsamples\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut max_samples_0 as *mut i64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"offset\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut offset as *mut f64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"delay\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut delay as *mut f64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"pps\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            n = 0 as i64;
            pps_forced = 1 as i64
        } else if strcasecmp(cmd,
                             b"precision\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut precision as *mut f64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"maxdispersion\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut max_dispersion as *mut f64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"stratum\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut stratum as *mut i64,
                      &mut n as *mut i64) != 1 as i64 ||
                   stratum >= 16 as i64 || stratum < 0 as i64
               {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"tai\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            n = 0 as i64;
            tai = 1 as i64
        } else if strcasecmp(cmd,
                             b"width\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut pulse_width as *mut f64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(cmd,
                             b"noselect\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            n = 0 as i64;
            sel_options |= 0x1 as i64
        } else if strcasecmp(cmd,
                             b"prefer\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            n = 0 as i64;
            sel_options |= 0x2 as i64
        } else if strcasecmp(cmd,
                             b"trust\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            n = 0 as i64;
            sel_options |= 0x4 as i64
        } else if strcasecmp(cmd,
                             b"require\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            n = 0 as i64;
            sel_options |= 0x8 as i64
        } else {
            other_parse_error(b"Invalid refclock option\x00" as *const u8 as
                                  *const libc::c_char);
            return
        }
        line = line.offset(n as isize);
        cmd = line
    }
    if *cmd != 0 { command_parse_error(); return }
    refclock = ARR_GetNewElement(refclock_sources) as *mut RefclockParameters;
    (*refclock).driver_name = name;
    (*refclock).driver_parameter = param;
    (*refclock).driver_poll = dpoll;
    (*refclock).poll = poll;
    (*refclock).filter_length = filter_length;
    (*refclock).pps_forced = pps_forced;
    (*refclock).pps_rate = pps_rate;
    (*refclock).min_samples = min_samples_0;
    (*refclock).max_samples = max_samples_0;
    (*refclock).sel_options = sel_options;
    (*refclock).stratum = stratum;
    (*refclock).tai = tai;
    (*refclock).offset = offset;
    (*refclock).delay = delay;
    (*refclock).precision = precision;
    (*refclock).max_dispersion = max_dispersion;
    (*refclock).pulse_width = pulse_width;
    (*refclock).ref_id = ref_id;
    (*refclock).max_lock_age = max_lock_age;
    (*refclock).lock_ref_id = lock_ref_id;
}
/* ================================================== */
unsafe extern "C" fn parse_log(mut line: *mut libc::c_char) {
    let mut log_name: *mut libc::c_char = 0 as *mut libc::c_char;
    loop  {
        log_name = line;
        line = CPS_SplitWord(line);
        if !(*log_name != 0) { break ; }
        if strcmp(log_name,
                  b"rawmeasurements\x00" as *const u8 as *const libc::c_char)
               == 0 {
            do_log_measurements = 1 as i64;
            raw_measurements = 1 as i64
        } else if strcmp(log_name,
                         b"measurements\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            do_log_measurements = 1 as i64
        } else if strcmp(log_name,
                         b"statistics\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            do_log_statistics = 1 as i64
        } else if strcmp(log_name,
                         b"tracking\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            do_log_tracking = 1 as i64
        } else if strcmp(log_name,
                         b"rtc\x00" as *const u8 as *const libc::c_char) == 0
         {
            do_log_rtc = 1 as i64
        } else if strcmp(log_name,
                         b"refclocks\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            do_log_refclocks = 1 as i64
        } else if strcmp(log_name,
                         b"tempcomp\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            do_log_tempcomp = 1 as i64
        } else {
            other_parse_error(b"Invalid log parameter\x00" as *const u8 as
                                  *const libc::c_char);
            break ;
        }
    };
}
/* ================================================== */
unsafe extern "C" fn parse_local(mut line: *mut libc::c_char) {
    if CPS_ParseLocal(line, &mut local_stratum, &mut local_orphan,
                      &mut local_distance) == 0 {
        command_parse_error();
    }
    enable_local = 1 as i64;
}
/* ================================================== */
unsafe extern "C" fn parse_initstepslew(mut line: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip_addr: IPAddr =
        IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
    /* Ignore the line if chronyd was started with -R. */
    if restarted != 0 { return }
    ARR_SetSize(init_sources, 0 as i64 as u64);
    p = CPS_SplitWord(line);
    if sscanf(line, b"%lf\x00" as *const u8 as *const libc::c_char,
              &mut init_slew_threshold as *mut f64) !=
           1 as i64 {
        command_parse_error();
        return
    }
    while *p != 0 {
        hostname = p;
        p = CPS_SplitWord(p);
        if *hostname != 0 {
            if DNS_Name2IPAddress(hostname, &mut ip_addr, 1 as i64) as
                   u64 == DNS_Success as i64 as u64
               {
                ARR_AppendElement(init_sources,
                                  &mut ip_addr as *mut IPAddr as
                                      *mut libc::c_void);
            } else {
                LOG_Message(LOGS_WARN,
                            b"Could not resolve address of initstepslew server %s\x00"
                                as *const u8 as *const libc::c_char,
                            hostname);
            }
        }
    };
}
/* ================================================== */
unsafe extern "C" fn parse_leapsecmode(mut line: *mut libc::c_char) {
    if strcasecmp(line, b"system\x00" as *const u8 as *const libc::c_char) ==
           0 {
        leapsec_mode = REF_LeapModeSystem
    } else if strcasecmp(line,
                         b"slew\x00" as *const u8 as *const libc::c_char) == 0
     {
        leapsec_mode = REF_LeapModeSlew
    } else if strcasecmp(line,
                         b"step\x00" as *const u8 as *const libc::c_char) == 0
     {
        leapsec_mode = REF_LeapModeStep
    } else if strcasecmp(line,
                         b"ignore\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        leapsec_mode = REF_LeapModeIgnore
    } else { command_parse_error(); };
}
/* ================================================== */
unsafe extern "C" fn parse_clientloglimit(mut line: *mut libc::c_char) {
    check_number_of_args(line, 1 as i64);
    if sscanf(line, b"%lu\x00" as *const u8 as *const libc::c_char,
              &mut client_log_limit as *mut libc::c_ulong) != 1 as i64
       {
        command_parse_error();
    };
}
/* ================================================== */
unsafe extern "C" fn parse_fallbackdrift(mut line: *mut libc::c_char) {
    check_number_of_args(line, 2 as i64);
    if sscanf(line, b"%d %d\x00" as *const u8 as *const libc::c_char,
              &mut fb_drift_min as *mut i64,
              &mut fb_drift_max as *mut i64) != 2 as i64 {
        command_parse_error();
    };
}
/* ================================================== */
unsafe extern "C" fn parse_makestep(mut line: *mut libc::c_char) {
    check_number_of_args(line, 2 as i64);
    if sscanf(line, b"%lf %d\x00" as *const u8 as *const libc::c_char,
              &mut make_step_threshold as *mut f64,
              &mut make_step_limit as *mut i64) != 2 as i64 {
        make_step_limit = 0 as i64;
        command_parse_error();
    }
    /* Disable limited makestep if chronyd was started with -R. */
    if restarted != 0 && make_step_limit > 0 as i64 {
        make_step_limit = 0 as i64
    };
}
/* ================================================== */
unsafe extern "C" fn parse_maxchange(mut line: *mut libc::c_char) {
    check_number_of_args(line, 3 as i64);
    if sscanf(line, b"%lf %d %d\x00" as *const u8 as *const libc::c_char,
              &mut max_offset as *mut f64,
              &mut max_offset_delay as *mut i64,
              &mut max_offset_ignore as *mut i64) != 3 as i64
       {
        max_offset_delay = -(1 as i64);
        command_parse_error();
    };
}
/* ================================================== */
unsafe extern "C" fn parse_mailonchange(mut line: *mut libc::c_char) {
    let mut address: *mut libc::c_char = 0 as *mut libc::c_char;
    check_number_of_args(line, 2 as i64);
    address = line;
    line = CPS_SplitWord(line);
    free(mail_user_on_change as *mut libc::c_void);
    if sscanf(line, b"%lf\x00" as *const u8 as *const libc::c_char,
              &mut mail_change_threshold as *mut f64) ==
           1 as i64 {
        mail_user_on_change = Strdup(address)
    } else {
        mail_user_on_change = 0 as *mut libc::c_char;
        command_parse_error();
    };
}
/* ================================================== */
unsafe extern "C" fn parse_allow_deny(mut line: *mut libc::c_char,
                                      mut restrictions: ARR_Instance,
                                      mut allow: i64) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a: libc::c_ulong = 0;
    let mut b: libc::c_ulong = 0;
    let mut c: libc::c_ulong = 0;
    let mut d: libc::c_ulong = 0;
    let mut n: libc::c_ulong = 0;
    let mut all: i64 = 0 as i64;
    let mut new_node: *mut AllowDeny = 0 as *mut AllowDeny;
    let mut ip_addr: IPAddr =
        IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
    p = line;
    if strncmp(p, b"all\x00" as *const u8 as *const libc::c_char,
               3 as i64 as libc::c_ulong) == 0 {
        all = 1 as i64;
        p = CPS_SplitWord(line)
    }
    if *p == 0 {
        /* Empty line applies to all addresses */
        new_node = ARR_GetNewElement(restrictions) as *mut AllowDeny;
        (*new_node).allow = allow;
        (*new_node).all = all;
        (*new_node).ip.family = 0 as i64 as uint16_t;
        (*new_node).subnet_bits = 0 as i64
    } else {
        let mut slashpos: *mut libc::c_char = 0 as *mut libc::c_char;
        slashpos = strchr(p, '/' as i32);
        if !slashpos.is_null() {
            *slashpos = 0 as i64 as libc::c_char
        }
        check_number_of_args(p, 1 as i64);
        n = 0 as i64 as libc::c_ulong;
        if UTI_StringToIP(p, &mut ip_addr) != 0 ||
               {
                   n =
                       sscanf(p,
                              b"%lu.%lu.%lu.%lu\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut a as *mut libc::c_ulong,
                              &mut b as *mut libc::c_ulong,
                              &mut c as *mut libc::c_ulong,
                              &mut d as *mut libc::c_ulong) as libc::c_ulong;
                   (n) >= 1 as i64 as libc::c_ulong
               } {
            new_node = ARR_GetNewElement(restrictions) as *mut AllowDeny;
            (*new_node).allow = allow;
            (*new_node).all = all;
            if n == 0 as i64 as libc::c_ulong {
                (*new_node).ip = ip_addr;
                if ip_addr.family as i64 == 2 as i64 {
                    (*new_node).subnet_bits = 128 as i64
                } else { (*new_node).subnet_bits = 32 as i64 }
            } else {
                (*new_node).ip.family = 1 as i64 as uint16_t;
                a &= 0xff as i64 as libc::c_ulong;
                b &= 0xff as i64 as libc::c_ulong;
                c &= 0xff as i64 as libc::c_ulong;
                d &= 0xff as i64 as libc::c_ulong;
                match n {
                    1 => {
                        (*new_node).ip.addr.in4 =
                            (a << 24 as i64) as uint32_t;
                        (*new_node).subnet_bits = 8 as i64
                    }
                    2 => {
                        (*new_node).ip.addr.in4 =
                            (a << 24 as i64 | b << 16 as i64)
                                as uint32_t;
                        (*new_node).subnet_bits = 16 as i64
                    }
                    3 => {
                        (*new_node).ip.addr.in4 =
                            (a << 24 as i64 | b << 16 as i64 |
                                 c << 8 as i64) as uint32_t;
                        (*new_node).subnet_bits = 24 as i64
                    }
                    4 => {
                        (*new_node).ip.addr.in4 =
                            (a << 24 as i64 | b << 16 as i64 |
                                 c << 8 as i64 | d) as uint32_t;
                        (*new_node).subnet_bits = 32 as i64
                    }
                    _ => {
                        __assert_fail(b"0\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"conf.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1063 as i64 as u64,
                                      (*::std::mem::transmute::<&[u8; 49],
                                                                &[libc::c_char; 49]>(b"void parse_allow_deny(char *, ARR_Instance, int)\x00")).as_ptr());
                    }
                }
            }
            if !slashpos.is_null() {
                let mut specified_subnet_bits: i64 = 0;
                let mut n_0: i64 = 0;
                n_0 =
                    sscanf(slashpos.offset(1 as i64 as isize),
                           b"%d\x00" as *const u8 as *const libc::c_char,
                           &mut specified_subnet_bits as *mut i64);
                if n_0 == 1 as i64 {
                    (*new_node).subnet_bits = specified_subnet_bits
                } else { command_parse_error(); }
            }
        } else if slashpos.is_null() &&
                      DNS_Name2IPAddress(p, &mut ip_addr, 1 as i64) as
                          u64 ==
                          DNS_Success as i64 as u64 {
            new_node = ARR_GetNewElement(restrictions) as *mut AllowDeny;
            (*new_node).allow = allow;
            (*new_node).all = all;
            (*new_node).ip = ip_addr;
            if ip_addr.family as i64 == 2 as i64 {
                (*new_node).subnet_bits = 128 as i64
            } else { (*new_node).subnet_bits = 32 as i64 }
        } else { command_parse_error(); }
    };
}
/* ================================================== */
unsafe extern "C" fn parse_bindacqaddress(mut line: *mut libc::c_char) {
    let mut ip: IPAddr =
        IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
    check_number_of_args(line, 1 as i64);
    if UTI_StringToIP(line, &mut ip) != 0 {
        if ip.family as i64 == 1 as i64 {
            bind_acq_address4 = ip
        } else if ip.family as i64 == 2 as i64 {
            bind_acq_address6 = ip
        }
    } else { command_parse_error(); };
}
/* ================================================== */
unsafe extern "C" fn parse_bindaddress(mut line: *mut libc::c_char) {
    let mut ip: IPAddr =
        IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
    check_number_of_args(line, 1 as i64);
    if UTI_StringToIP(line, &mut ip) != 0 {
        if ip.family as i64 == 1 as i64 {
            bind_address4 = ip
        } else if ip.family as i64 == 2 as i64 {
            bind_address6 = ip
        }
    } else { command_parse_error(); };
}
/* ================================================== */
unsafe extern "C" fn parse_bindcmdaddress(mut line: *mut libc::c_char) {
    let mut ip: IPAddr =
        IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
    check_number_of_args(line, 1 as i64);
    /* Address starting with / is for the Unix domain socket */
    if *line.offset(0 as i64 as isize) as i64 == '/' as i32 {
        parse_string(line, &mut bind_cmd_path);
        /* / disables the socket */
        if strcmp(bind_cmd_path, b"/\x00" as *const u8 as *const libc::c_char)
               == 0 {
            *bind_cmd_path.offset(0 as i64 as isize) =
                '\u{0}' as i32 as libc::c_char
        }
    } else if UTI_StringToIP(line, &mut ip) != 0 {
        if ip.family as i64 == 1 as i64 {
            bind_cmd_address4 = ip
        } else if ip.family as i64 == 2 as i64 {
            bind_cmd_address6 = ip
        }
    } else { command_parse_error(); };
}
/* ================================================== */
unsafe extern "C" fn parse_broadcast(mut line: *mut libc::c_char) {
    /* Syntax : broadcast <interval> <broadcast-IP-addr> [<port>] */
    let mut destination: *mut NTP_Broadcast_Destination =
        0 as *mut NTP_Broadcast_Destination;
    let mut port: i64 = 0;
    let mut interval: i64 = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip: IPAddr =
        IPAddr{addr: C2RustUnnamed{in4: 0,}, family: 0, _pad: 0,};
    p = line;
    line = CPS_SplitWord(line);
    if sscanf(p, b"%d\x00" as *const u8 as *const libc::c_char,
              &mut interval as *mut i64) != 1 as i64 {
        command_parse_error();
        return
    }
    p = line;
    line = CPS_SplitWord(line);
    if UTI_StringToIP(p, &mut ip) == 0 { command_parse_error(); return }
    p = line;
    line = CPS_SplitWord(line);
    if *p != 0 {
        if sscanf(p, b"%d\x00" as *const u8 as *const libc::c_char,
                  &mut port as *mut i64) != 1 as i64 ||
               *line as i64 != 0 {
            command_parse_error();
            return
        }
    } else {
        /* default port */
        port = 123 as i64
    }
    destination =
        ARR_GetNewElement(broadcasts) as *mut NTP_Broadcast_Destination;
    (*destination).addr = ip;
    (*destination).port = port as libc::c_ushort;
    (*destination).interval = interval;
}
/* ================================================== */
unsafe extern "C" fn parse_smoothtime(mut line: *mut libc::c_char) {
    if get_number_of_args(line) != 3 as i64 {
        check_number_of_args(line, 2 as i64);
    }
    if sscanf(line, b"%lf %lf\x00" as *const u8 as *const libc::c_char,
              &mut smooth_max_freq as *mut f64,
              &mut smooth_max_wander as *mut f64) !=
           2 as i64 {
        smooth_max_freq = 0.0f64;
        command_parse_error();
    }
    line = CPS_SplitWord(CPS_SplitWord(line));
    smooth_leap_only = 0 as i64;
    if *line != 0 {
        if strcasecmp(line,
                      b"leaponly\x00" as *const u8 as *const libc::c_char) ==
               0 {
            smooth_leap_only = 1 as i64
        } else { command_parse_error(); }
    };
}
/* ================================================== */
unsafe extern "C" fn parse_tempcomp(mut line: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut point_form: i64 = 0;
    point_form =
        (get_number_of_args(line) == 3 as i64) as i64;
    if point_form == 0 { check_number_of_args(line, 6 as i64); }
    p = line;
    line = CPS_SplitWord(line);
    if *p == 0 { command_parse_error(); return }
    free(tempcomp_point_file as *mut libc::c_void);
    if point_form != 0 {
        if sscanf(line, b"%lf\x00" as *const u8 as *const libc::c_char,
                  &mut tempcomp_interval as *mut f64) !=
               1 as i64 {
            command_parse_error();
            return
        }
        tempcomp_point_file = Strdup(CPS_SplitWord(line))
    } else {
        if sscanf(line,
                  b"%lf %lf %lf %lf %lf\x00" as *const u8 as
                      *const libc::c_char,
                  &mut tempcomp_interval as *mut f64,
                  &mut tempcomp_T0 as *mut f64,
                  &mut tempcomp_k0 as *mut f64,
                  &mut tempcomp_k1 as *mut f64,
                  &mut tempcomp_k2 as *mut f64) != 5 as i64
           {
            command_parse_error();
            return
        }
        tempcomp_point_file = 0 as *mut libc::c_char
    }
    free(tempcomp_sensor_file as *mut libc::c_void);
    tempcomp_sensor_file = Strdup(p);
}
/* ================================================== */
unsafe extern "C" fn parse_hwtimestamp(mut line: *mut libc::c_char) {
    let mut iface: *mut CNF_HwTsInterface = 0 as *mut CNF_HwTsInterface;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filter: [libc::c_char; 5] = [0; 5];
    let mut n: i64 = 0;
    if *line == 0 { command_parse_error(); return }
    p = line;
    line = CPS_SplitWord(line);
    iface = ARR_GetNewElement(hwts_interfaces) as *mut CNF_HwTsInterface;
    (*iface).name = Strdup(p);
    (*iface).minpoll = 0 as i64;
    (*iface).min_samples = 2 as i64;
    (*iface).max_samples = 16 as i64;
    (*iface).nocrossts = 0 as i64;
    (*iface).rxfilter = CNF_HWTS_RXFILTER_ANY;
    (*iface).precision = 100.0e-9f64;
    (*iface).tx_comp = 0.0f64;
    (*iface).rx_comp = 0.0f64;
    p = line;
    while *p != 0 {
        line = CPS_SplitWord(line);
        if strcasecmp(p,
                      b"maxsamples\x00" as *const u8 as *const libc::c_char)
               == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*iface).max_samples as *mut i64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(p,
                             b"minpoll\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*iface).minpoll as *mut i64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(p,
                             b"minsamples\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*iface).min_samples as *mut i64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(p,
                             b"precision\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*iface).precision as *mut f64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(p,
                             b"rxcomp\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*iface).rx_comp as *mut f64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(p,
                             b"txcomp\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%lf%n\x00" as *const u8 as *const libc::c_char,
                      &mut (*iface).tx_comp as *mut f64,
                      &mut n as *mut i64) != 1 as i64 {
                break ;
            }
        } else if strcasecmp(p,
                             b"rxfilter\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            if sscanf(line, b"%4s%n\x00" as *const u8 as *const libc::c_char,
                      filter.as_mut_ptr(), &mut n as *mut i64) !=
                   1 as i64 {
                break ;
            }
            if strcasecmp(filter.as_mut_ptr(),
                          b"none\x00" as *const u8 as *const libc::c_char) ==
                   0 {
                (*iface).rxfilter = CNF_HWTS_RXFILTER_NONE
            } else if strcasecmp(filter.as_mut_ptr(),
                                 b"ntp\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                (*iface).rxfilter = CNF_HWTS_RXFILTER_NTP
            } else {
                if !(strcasecmp(filter.as_mut_ptr(),
                                b"all\x00" as *const u8 as
                                    *const libc::c_char) == 0) {
                    break ;
                }
                (*iface).rxfilter = CNF_HWTS_RXFILTER_ALL
            }
        } else {
            if !(strcasecmp(p,
                            b"nocrossts\x00" as *const u8 as
                                *const libc::c_char) == 0) {
                break ;
            }
            n = 0 as i64;
            (*iface).nocrossts = 1 as i64
        }
        line = line.offset(n as isize);
        p = line
    }
    if *p != 0 { command_parse_error(); };
}
/* ================================================== */
unsafe extern "C" fn parse_include(mut line: *mut libc::c_char) {
    let mut gl: glob_t =
        glob_t{gl_pathc: 0,
               gl_pathv: 0 as *mut *mut libc::c_char,
               gl_offs: 0,
               gl_flags: 0,
               gl_closedir: None,
               gl_readdir: None,
               gl_opendir: None,
               gl_lstat: None,
               gl_stat: None,};
    let mut i: size_t = 0;
    let mut r: i64 = 0;
    check_number_of_args(line, 1 as i64);
    r =
        glob(line,
             (1 as i64) << 11 as i64 |
                 (1 as i64) << 0 as i64, None, &mut gl);
    if r != 0 as i64 {
        if r != 3 as i64 {
            LOG_Message(LOGS_FATAL,
                        b"Could not search for files matching %s\x00" as
                            *const u8 as *const libc::c_char, line);
            exit(1 as i64);
        }
        if 0 as i64 != 0 &&
               log_min_severity as i64 == LOGS_DEBUG as i64 {
            LOG_Message(LOGS_DEBUG,
                        b"glob of %s failed\x00" as *const u8 as
                            *const libc::c_char, line);
        }
        return
    }
    i = 0 as i64 as size_t;
    while i < gl.gl_pathc {
        CNF_ReadFile(*gl.gl_pathv.offset(i as isize));
        i = i.wrapping_add(1)
    }
    globfree(&mut gl);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_CreateDirs(mut uid: uid_t, mut gid: gid_t) {
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Create a directory for the Unix domain command socket */
    if *bind_cmd_path.offset(0 as i64 as isize) != 0 {
        dir = UTI_PathToDir(bind_cmd_path);
        UTI_CreateDirAndParents(dir, 0o770 as i64 as mode_t, uid,
                                gid);
        /* Check the permissions and owner/group in case the directory already
       existed.  It MUST NOT be accessible by others as permissions on Unix
       domain sockets are ignored on some systems (e.g. Solaris). */
        if UTI_CheckDirPermissions(dir, 0o770 as i64 as mode_t, uid,
                                   gid) == 0 {
            LOG_Message(LOGS_WARN,
                        b"Disabled command socket %s\x00" as *const u8 as
                            *const libc::c_char, bind_cmd_path);
            *bind_cmd_path.offset(0 as i64 as isize) =
                '\u{0}' as i32 as libc::c_char
        }
        free(dir as *mut libc::c_void);
    }
    if *logdir.offset(0 as i64 as isize) != 0 {
        UTI_CreateDirAndParents(logdir, 0o755 as i64 as mode_t, uid,
                                gid);
    }
    if *dumpdir.offset(0 as i64 as isize) != 0 {
        UTI_CreateDirAndParents(dumpdir, 0o755 as i64 as mode_t, uid,
                                gid);
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_AddInitSources() {
    let mut cps_source: CPS_NTP_Source =
        CPS_NTP_Source{name: 0 as *mut libc::c_char,
                       port: 0,
                       params:
                           SourceParameters{minpoll: 0,
                                            maxpoll: 0,
                                            connectivity: SRC_OFFLINE,
                                            auto_offline: 0,
                                            presend_minpoll: 0,
                                            burst: 0,
                                            iburst: 0,
                                            min_stratum: 0,
                                            poll_target: 0,
                                            version: 0,
                                            max_sources: 0,
                                            min_samples: 0,
                                            max_samples: 0,
                                            filter_length: 0,
                                            interleaved: 0,
                                            sel_options: 0,
                                            authkey: 0,
                                            max_delay: 0.,
                                            max_delay_ratio: 0.,
                                            max_delay_dev_ratio: 0.,
                                            min_delay: 0.,
                                            asymmetry: 0.,
                                            offset: 0.,},};
    let mut ntp_addr: NTP_Remote_Address =
        NTP_Remote_Address{ip_addr:
                               IPAddr{addr: C2RustUnnamed{in4: 0,},
                                      family: 0,
                                      _pad: 0,},
                           port: 0,};
    let mut dummy_hostname: [libc::c_char; 2] =
        *::std::mem::transmute::<&[u8; 2], &mut [libc::c_char; 2]>(b"H\x00");
    let mut i: u64 = 0;
    i = 0 as i64 as u64;
    while i < ARR_GetSize(init_sources) {
        /* Get the default NTP params */
        CPS_ParseNTPSourceAdd(dummy_hostname.as_mut_ptr(), &mut cps_source);
        /* Add the address as an offline iburst server */
        ntp_addr.ip_addr = *(ARR_GetElement(init_sources, i) as *mut IPAddr);
        ntp_addr.port = cps_source.port;
        cps_source.params.iburst = 1 as i64;
        cps_source.params.connectivity = SRC_OFFLINE;
        NSR_AddSource(&mut ntp_addr, NTP_SERVER, &mut cps_source.params);
        i = i.wrapping_add(1)
    }
    ARR_SetSize(init_sources, 0 as i64 as u64);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_AddSources() {
    let mut source: *mut NTP_Source = 0 as *mut NTP_Source;
    let mut i: u64 = 0;
    i = 0 as i64 as u64;
    while i < ARR_GetSize(ntp_sources) {
        source = ARR_GetElement(ntp_sources, i) as *mut NTP_Source;
        NSR_AddSourceByName((*source).params.name,
                            (*source).params.port as i64,
                            (*source).pool, (*source).type_0,
                            &mut (*source).params.params);
        free((*source).params.name as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    ARR_SetSize(ntp_sources, 0 as i64 as u64);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_AddRefclocks() {
    let mut i: u64 = 0;
    i = 0 as i64 as u64;
    while i < ARR_GetSize(refclock_sources) {
        RCL_AddRefclock(ARR_GetElement(refclock_sources, i) as
                            *mut RefclockParameters);
        i = i.wrapping_add(1)
    }
    ARR_SetSize(refclock_sources, 0 as i64 as u64);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_AddBroadcasts() {
    let mut i: u64 = 0;
    let mut destination: *mut NTP_Broadcast_Destination =
        0 as *mut NTP_Broadcast_Destination;
    i = 0 as i64 as u64;
    while i < ARR_GetSize(broadcasts) {
        destination =
            ARR_GetElement(broadcasts, i) as *mut NTP_Broadcast_Destination;
        NCR_AddBroadcastDestination(&mut (*destination).addr,
                                    (*destination).port,
                                    (*destination).interval);
        i = i.wrapping_add(1)
    }
    ARR_SetSize(broadcasts, 0 as i64 as u64);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetNTPPort() -> i64 { return ntp_port; }
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetAcquisitionPort() -> i64 {
    return acquisition_port;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetDriftFile() -> *mut libc::c_char {
    return drift_file;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogBanner() -> i64 {
    return log_banner;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogDir() -> *mut libc::c_char {
    return logdir;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetDumpDir() -> *mut libc::c_char {
    return dumpdir;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogMeasurements(mut raw: *mut i64)
 -> i64 {
    *raw = raw_measurements;
    return do_log_measurements;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogStatistics() -> i64 {
    return do_log_statistics;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogTracking() -> i64 {
    return do_log_tracking;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogRtc() -> i64 { return do_log_rtc; }
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogRefclocks() -> i64 {
    return do_log_refclocks;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogTempComp() -> i64 {
    return do_log_tempcomp;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetKeysFile() -> *mut libc::c_char {
    return keys_file;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetRtcAutotrim() -> f64 {
    return rtc_autotrim_threshold;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetRtcFile() -> *mut libc::c_char {
    return rtc_file;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetRtcDevice() -> *mut libc::c_char {
    return rtc_device;
}
/* Value returned in ppm, as read from file */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxUpdateSkew() -> f64 {
    return max_update_skew;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxDrift() -> f64 {
    return max_drift;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxClockError() -> f64 {
    return max_clock_error;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetCorrectionTimeRatio() -> f64 {
    return correction_time_ratio;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxSlewRate() -> f64 {
    return max_slew_rate;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxDistance() -> f64 {
    return max_distance;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxJitter() -> f64 {
    return max_jitter;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetReselectDistance() -> f64 {
    return reselect_distance;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetStratumWeight() -> f64 {
    return stratum_weight;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetCombineLimit() -> f64 {
    return combine_limit;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetManualEnabled() -> i64 {
    return enable_manual;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetCommandPort() -> i64 {
    return cmd_port;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_AllowLocalReference(mut stratum:
                                                     *mut i64,
                                                 mut orphan: *mut i64,
                                                 mut distance:
                                                     *mut f64)
 -> i64 {
    if enable_local != 0 {
        *stratum = local_stratum;
        *orphan = local_orphan;
        *distance = local_distance;
        return 1 as i64
    } else { return 0 as i64 };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetRtcOnUtc() -> i64 {
    return rtc_on_utc;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetRtcSync() -> i64 { return rtc_sync; }
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMakeStep(mut limit: *mut i64,
                                         mut threshold: *mut f64) {
    *limit = make_step_limit;
    *threshold = make_step_threshold;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxChange(mut delay: *mut i64,
                                          mut ignore: *mut i64,
                                          mut offset: *mut f64) {
    *delay = max_offset_delay;
    *ignore = max_offset_ignore;
    *offset = max_offset;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLogChange() -> f64 {
    return log_change_threshold;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMailOnChange(mut enabled: *mut i64,
                                             mut threshold:
                                                 *mut f64,
                                             mut user_0:
                                                 *mut *mut libc::c_char) {
    if !mail_user_on_change.is_null() {
        *enabled = 1 as i64;
        *threshold = mail_change_threshold;
        *user_0 = mail_user_on_change
    } else {
        *enabled = 0 as i64;
        *threshold = 0.0f64;
        *user_0 = 0 as *mut libc::c_char
    };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_SetupAccessRestrictions() {
    let mut node: *mut AllowDeny = 0 as *mut AllowDeny;
    let mut status: i64 = 0;
    let mut i: u64 = 0;
    i = 0 as i64 as u64;
    while i < ARR_GetSize(ntp_restrictions) {
        node = ARR_GetElement(ntp_restrictions, i) as *mut AllowDeny;
        status =
            NCR_AddAccessRestriction(&mut (*node).ip, (*node).subnet_bits,
                                     (*node).allow, (*node).all);
        if status == 0 {
            LOG_Message(LOGS_FATAL,
                        b"Bad subnet in %s/%d\x00" as *const u8 as
                            *const libc::c_char,
                        UTI_IPToString(&mut (*node).ip), (*node).subnet_bits);
            exit(1 as i64);
        }
        i = i.wrapping_add(1)
    }
    i = 0 as i64 as u64;
    while i < ARR_GetSize(cmd_restrictions) {
        node = ARR_GetElement(cmd_restrictions, i) as *mut AllowDeny;
        status =
            CAM_AddAccessRestriction(&mut (*node).ip, (*node).subnet_bits,
                                     (*node).allow, (*node).all);
        if status == 0 {
            LOG_Message(LOGS_FATAL,
                        b"Bad subnet in %s/%d\x00" as *const u8 as
                            *const libc::c_char,
                        UTI_IPToString(&mut (*node).ip), (*node).subnet_bits);
            exit(1 as i64);
        }
        i = i.wrapping_add(1)
    }
    ARR_SetSize(ntp_restrictions, 0 as i64 as u64);
    ARR_SetSize(cmd_restrictions, 0 as i64 as u64);
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetNoClientLog() -> i64 {
    return no_client_log;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetClientLogLimit() -> libc::c_ulong {
    return client_log_limit;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetFallbackDrifts(mut min: *mut i64,
                                               mut max: *mut i64) {
    *min = fb_drift_min;
    *max = fb_drift_max;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetBindAddress(mut family: i64,
                                            mut addr: *mut IPAddr) {
    if family == 1 as i64 {
        *addr = bind_address4
    } else if family == 2 as i64 {
        *addr = bind_address6
    } else { (*addr).family = 0 as i64 as uint16_t };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetBindAcquisitionAddress(mut family:
                                                           i64,
                                                       mut addr:
                                                           *mut IPAddr) {
    if family == 1 as i64 {
        *addr = bind_acq_address4
    } else if family == 2 as i64 {
        *addr = bind_acq_address6
    } else { (*addr).family = 0 as i64 as uint16_t };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetBindCommandPath() -> *mut libc::c_char {
    return bind_cmd_path;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetBindCommandAddress(mut family: i64,
                                                   mut addr: *mut IPAddr) {
    if family == 1 as i64 {
        *addr = bind_cmd_address4
    } else if family == 2 as i64 {
        *addr = bind_cmd_address6
    } else { (*addr).family = 0 as i64 as uint16_t };
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetNtpSigndSocket() -> *mut libc::c_char {
    return ntp_signd_socket;
}
/* ================================================== */
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLeapSecMode() -> REF_LeapMode {
    return leapsec_mode;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLeapSecTimezone() -> *mut libc::c_char {
    return leapsec_tz;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetSchedPriority() -> i64 {
    return sched_priority;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetLockMemory() -> i64 {
    return lock_memory;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetNTPRateLimit(mut interval: *mut i64,
                                             mut burst: *mut i64,
                                             mut leak: *mut i64)
 -> i64 {
    *interval = ntp_ratelimit_interval;
    *burst = ntp_ratelimit_burst;
    *leak = ntp_ratelimit_leak;
    return ntp_ratelimit_enabled;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetCommandRateLimit(mut interval:
                                                     *mut i64,
                                                 mut burst: *mut i64,
                                                 mut leak: *mut i64)
 -> i64 {
    *interval = cmd_ratelimit_interval;
    *burst = cmd_ratelimit_burst;
    *leak = cmd_ratelimit_leak;
    return cmd_ratelimit_enabled;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetSmooth(mut max_freq: *mut f64,
                                       mut max_wander: *mut f64,
                                       mut leap_only: *mut i64) {
    *max_freq = smooth_max_freq;
    *max_wander = smooth_max_wander;
    *leap_only = smooth_leap_only;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetTempComp(mut file: *mut *mut libc::c_char,
                                         mut interval: *mut f64,
                                         mut point_file:
                                             *mut *mut libc::c_char,
                                         mut T0: *mut f64,
                                         mut k0: *mut f64,
                                         mut k1: *mut f64,
                                         mut k2: *mut f64) {
    *file = tempcomp_sensor_file;
    *point_file = tempcomp_point_file;
    *interval = tempcomp_interval;
    *T0 = tempcomp_T0;
    *k0 = tempcomp_k0;
    *k1 = tempcomp_k1;
    *k2 = tempcomp_k2;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetUser() -> *mut libc::c_char { return user; }
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMaxSamples() -> i64 {
    return max_samples;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMinSamples() -> i64 {
    return min_samples;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetMinSources() -> i64 {
    return min_sources;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetHwclockFile() -> *mut libc::c_char {
    return hwclock_file;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetInitSources() -> i64 {
    return ARR_GetSize(init_sources) as i64;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetInitStepThreshold() -> f64 {
    return init_slew_threshold;
}
/* ================================================== */
#[no_mangle]
pub unsafe extern "C" fn CNF_GetHwTsInterface(mut index: u64,
                                              mut iface:
                                                  *mut *mut CNF_HwTsInterface)
 -> i64 {
    if index >= ARR_GetSize(hwts_interfaces) { return 0 as i64 }
    *iface = ARR_GetElement(hwts_interfaces, index) as *mut CNF_HwTsInterface;
    return 1 as i64;
}



mod test {
    use super::*;
    use std::io::Write;
    // Config file tests
    const FILE_01: &str = include_str!("./testconfs/01.conf");

    unsafe fn assert_cstr(expected: &str, cstr: *mut i8) {
        assert_eq!(expected, std::ffi::CStr::from_ptr(cstr).to_str().unwrap());
    }

    #[test]
    fn test_01() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        tmp.write_all(FILE_01.as_bytes()).unwrap();

        unsafe { 
            CNF_Initialise(0, 0);
            CNF_ReadFile(tmp.path().to_str().unwrap().as_bytes().as_ptr() as *const i8);

            // I guess do some assertions on globals now. TODO: move the globals into a struct we
            // return up to callers, do the right thing, etc.
            assert_cstr("/etc/chrony/chrony.keys", keys_file);
            assert_cstr("/var/lib/chrony/chrony.drift", drift_file);
            assert_cstr("/var/log/chrony", logdir);
            assert_eq!(100.0, max_update_skew);
            assert_eq!(1, rtc_sync);
            assert_eq!(1.0, make_step_threshold);
            assert_eq!(3, make_step_limit);
            assert_cstr("_chrony", user);
        }
    }
}
