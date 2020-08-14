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
pub enum Connectivity {
    MaybeOnline,
    Online,
    Offline,
}

#[derive(Copy, Clone, Debug)]
pub enum NTPSourceType {
    Peer,
    Server,
}

#[derive(Clone)]
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
        Conf {
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
            leapsec_mode: LeapMode::System,
            rtc_device: "/dev/rtc".to_string(),
            user: "root".to_string(),
            bind_cmd_path: "/var/run/chrony/chronyd.sock".to_string(),

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
        let mut conf = Conf {
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
        panic!("TODO");
    }
}
