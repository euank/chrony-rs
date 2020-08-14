use ipnet::{IpNet, Ipv4Net};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::{digit1, space1};
use nom::character::is_space;
use nom::combinator::{map, map_res, not, opt};
use nom::error::{context, ParseError, VerboseError};
use nom::multi::many1;
use nom::number::complete::double;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::default::Default;
use std::net::IpAddr;
use std::path::PathBuf;

// IpSet is the argument that allow and deny take.
#[derive(Debug, PartialEq, Eq)]
enum IpSet {
    All,
    Cidr(IpNet),
}

#[derive(Debug, PartialEq, Eq)]
enum BindCmdAddress {
    IpAddr(IpAddr),
    Path(PathBuf),
}

#[derive(Debug, PartialEq, Eq)]
struct Broadcast {
    interval: i64,
    addr: IpAddr,
    port: u16,
}

#[derive(Debug, PartialEq, Eq)]
struct RateLimit {
    limit_localhost: bool,
    interval: i64,
    burst: i64,
    leak: i64,
}

struct OptRateLimit {
    interval: Option<i64>,
    burst: Option<i64>,
    leak: Option<i64>,
}

#[derive(Debug, PartialEq)]
struct MinMax {
    Min: i64,
    Max: i64,
}

#[derive(Debug, PartialEq)]
enum Line {
    Nothing,
    AcquisitionPort(i64),
    Allow(IpSet),
    BindAcqAddress(std::net::IpAddr),
    BindAddress(std::net::IpAddr),
    BindCmdAddress(BindCmdAddress),
    Broadcast(Broadcast),
    ClientLogLimit(i64),
    CmdAllow(IpSet),
    CmdDeny(IpSet),
    CmdPort(i64),
    CmdRateLimit(RateLimit),
    CombineLimit(f64),
    CorrTimeRatio(f64),
    Deny(IpSet),
    Driftfile(PathBuf),
    Dumpdir(PathBuf),
    FallbackDrift(MinMax),
    HwClockFile(PathBuf),
    HwTimestamp,
    Include(String), // Not a path due to glob support
    InitStepSlew,
    Keyfile(PathBuf),
    LeapsecMode,
    LeapsecTz(String), // TODO: more precise type
    Local,
    LockAll,
    Log,
    LogBanner,
    LogChange(f64),
    LogDir(PathBuf),
    MailOnChange,
    MakeStep,
    Manual,
    MaxChange,
    MaxClockError(f64),
    MaxDistance(f64),
    MaxDrift(f64),
    MaxJitter(f64),
    MaxSamples(i64),
    MaxSlewRate(f64),
    MaxUpdateSkew(f64),
    MinSources(i64),
    NoClientLog,
    NtpSigndSocket(PathBuf),
    Peer,
    Pool,
    Port(i64),
    RateLimit(RateLimit),
    RefClock,
    ReselectDist(f64),
    RtcAutoTrim(f64),
    RtcDevice(String),
    RtcFile(PathBuf),
    RtcOnUtc,
    RtcSync,
    SchedPriority,
    Server,
    SmoothTime,
    StratumWeight(f64),
    TempComp,
    User(String),
}

fn parse_num<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, i64, E> {
    alt((
        map_res(digit1, |digits: &str| digits.parse::<i64>()),
        map_res(preceded(tag("-"), digit1), |digits: &str| {
            digits.parse::<i64>().map(|i| i * -1)
        }),
    ))(i)
}

fn parse_min_max<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, i64, E> {
    preceded(space1, parse_num)(i)
}

// Mimic scanf %lf, which is what the old code used
fn parse_double<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, f64, E> {
    nom::number::complete::double(i)
}

fn parse_port<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, u16, E> {
    map_res(digit1, |digits: &str| digits.parse::<u16>())(i)
}

fn parse_ipset<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, IpSet, E> {
    alt((
        map(tag("all"), |_| IpSet::All),
        map_res(take_while1(|c: char| !c.is_whitespace()), |p: &str| {
            p.parse::<IpNet>().map(|c| IpSet::Cidr(c))
        }),
    ))(i)
}

fn parse_allow<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    preceded(
        tag("allow"),
        preceded(space1, map(parse_ipset, |p| Line::Allow(p))),
    )(i)
}

fn parse_cmdallow<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    preceded(
        tag("cmdallow"),
        preceded(space1, map(parse_ipset, |p| Line::CmdAllow(p))),
    )(i)
}

fn parse_cmddeny<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    preceded(
        tag("cmddeny"),
        preceded(space1, map(parse_ipset, |p| Line::CmdDeny(p))),
    )(i)
}

fn parse_address<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, IpAddr, E> {
    map_res(take_while1(|c: char| !c.is_whitespace()), |p: &str| {
        p.parse::<IpAddr>()
    })(i)
}

fn parse_bindacqaddress<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    preceded(
        tag("bindacqaddress"),
        preceded(space1, map(parse_address, |p| Line::BindAcqAddress(p))),
    )(i)
}

fn parse_bindaddress<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    preceded(
        tag("bindaddress"),
        preceded(space1, map(parse_address, |p| Line::BindAddress(p))),
    )(i)
}

fn parse_string<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, String, E> {
    map(take_while1(|c| c != '\r' && c != '\n'), |s: &str| {
        s.to_string()
    })(i)
}

fn parse_path<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, PathBuf, E> {
    map(parse_string, |s| PathBuf::from(s))(i)
}

fn parse_bindcmdaddress<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    preceded(
        tag("bindcmdaddress"),
        preceded(
            space1,
            alt((
                map(parse_address, |p| {
                    Line::BindCmdAddress(BindCmdAddress::IpAddr(p))
                }),
                map(parse_path, |p| {
                    Line::BindCmdAddress(BindCmdAddress::Path(p))
                }),
            )),
        ),
    )(i)
}

fn parse_broadcast<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    map(
        preceded(
            tag("broadcast"),
            tuple((
                preceded(space1, parse_num),
                preceded(space1, parse_address),
                opt(preceded(space1, parse_port)),
            )),
        ),
        |(interval, addr, port)| {
            Line::Broadcast(Broadcast {
                interval,
                addr,
                port: port.unwrap_or(123),
            })
        },
    )(i)
}

fn parse_cmdport<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    map(
        preceded(tag("cmdport"), preceded(space1, parse_num)),
        |num: i64| Line::CmdPort(num),
    )(i)
}

fn ratelimit<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, OptRateLimit, E> {
    map(
        nom::branch::permutation((
            opt(preceded(tag("interval"), preceded(space1, parse_num))),
            opt(preceded(tag("burst"), preceded(space1, parse_num))),
            opt(preceded(tag("leak"), preceded(space1, parse_num))),
        )),
        |(interval, burst, leak)| OptRateLimit {
            interval,
            burst,
            leak,
        },
    )(i)
}

fn parse_cmdratelimit<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    map(
        preceded(tag("cmdratelimit"), preceded(space1, ratelimit)),
        |limit: OptRateLimit| {
            Line::CmdRateLimit(RateLimit {
                limit_localhost: false,
                interval: limit.interval.unwrap_or_else(|| -4),
                burst: limit.burst.unwrap_or_else(|| 8),
                leak: limit.leak.unwrap_or_else(|| 2),
            })
        },
    )(i)
}

fn parse_ratelimit<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    map(
        preceded(tag("ratelimit"), preceded(space1, ratelimit)),
        |limit: OptRateLimit| {
            Line::RateLimit(RateLimit {
                limit_localhost: true,
                interval: limit.interval.unwrap_or_else(|| 3),
                burst: limit.burst.unwrap_or_else(|| 8),
                leak: limit.leak.unwrap_or_else(|| 2),
            })
        },
    )(i)
}

fn parse_tagged_line<'a, T, P, L, H, O2, E: ParseError<&'a str>>(
    t: T,
    p: P,
    l: L,
) -> impl Fn(&'a str) -> IResult<&'a str, Line, E>
where
    L: Fn(H) -> Line,
    T: Fn(&'a str) -> IResult<&'a str, O2, E>,
    P: Fn(&'a str) -> IResult<&'a str, H, E>,
{
    preceded(t, preceded(space1, map(p, l)))
}

fn parse_num_line<'a, G, O2, E: ParseError<&'a str>, F>(
    t: G,
    l: F,
) -> impl Fn(&'a str) -> IResult<&'a str, Line, E>
where
    F: Fn(i64) -> Line,
    G: Fn(&'a str) -> IResult<&'a str, O2, E>,
{
    parse_tagged_line(t, parse_num, l)
}

fn parse_float_line<'a, G, O2, E: ParseError<&'a str>, F>(
    t: G,
    l: F,
) -> impl Fn(&'a str) -> IResult<&'a str, Line, E>
where
    F: Fn(f64) -> Line,
    G: Fn(&'a str) -> IResult<&'a str, O2, E>,
{
    parse_tagged_line(t, double, l)
}

fn parse_allow_deny_line<'a, G, O2, E: ParseError<&'a str>, F>(
    t: G,
    l: F,
) -> impl Fn(&'a str) -> IResult<&'a str, Line, E>
where
    F: Fn(IpSet) -> Line,
    G: Fn(&'a str) -> IResult<&'a str, O2, E>,
{
    parse_tagged_line(t, parse_ipset, l)
}

fn parse_path_line<'a, G, O2, E: ParseError<&'a str>, F>(
    t: G,
    l: F,
) -> impl Fn(&'a str) -> IResult<&'a str, Line, E>
where
    F: Fn(PathBuf) -> Line,
    G: Fn(&'a str) -> IResult<&'a str, O2, E>,
{
    parse_tagged_line(t, parse_path, l)
}

fn parse_string_line<'a, G, O2, E: ParseError<&'a str>, F>(
    t: G,
    l: F,
) -> impl Fn(&'a str) -> IResult<&'a str, Line, E>
where
    F: Fn(String) -> Line,
    G: Fn(&'a str) -> IResult<&'a str, O2, E>,
{
    parse_tagged_line(t, parse_string, l)
}

fn parse_line<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    alt((
        alt((
            parse_allow_deny_line(tag("allow"), Line::Allow),
            parse_bindacqaddress,
            parse_bindaddress,
            parse_bindcmdaddress,
            parse_broadcast,
            parse_num_line(tag("clientloglimit"), Line::ClientLogLimit),
            parse_allow_deny_line(tag("cmdallow"), Line::Allow),
            parse_allow_deny_line(tag("cmddeny"), Line::Allow),
            parse_cmdratelimit,
            parse_ratelimit,
            parse_num_line(tag("cmdport"), Line::CmdPort),
            parse_float_line(tag("combinelimit"), Line::CombineLimit),
            parse_float_line(tag("corrtimeratio"), Line::CorrTimeRatio),
            parse_num_line(tag("acquisition_port"), Line::AcquisitionPort),
            parse_float_line(tag("maxclockerror"), Line::MaxClockError),
            parse_float_line(tag("logchange"), Line::LogChange),
            parse_float_line(tag("maxdistance"), Line::MaxDistance),
            parse_float_line(tag("maxdrift"), Line::MaxDrift),
            parse_float_line(tag("maxjitter"), Line::MaxJitter),
            parse_num_line(tag("maxsamples"), Line::MaxSamples),
            parse_float_line(tag("maxslewrate"), Line::MaxSlewRate),
        )),
        alt((
            parse_float_line(tag("maxupdateskew"), Line::MaxUpdateSkew),
            parse_num_line(tag("minsources"), Line::MinSources),
            parse_num_line(tag("port"), Line::Port),
            parse_float_line(tag("reselectdist"), Line::ReselectDist),
            parse_float_line(tag("rtcautotrim"), Line::RtcAutoTrim),
            parse_float_line(tag("stratumweight"), Line::StratumWeight),
            parse_allow_deny_line(tag("deny"), Line::Deny),
            parse_path_line(tag("driftfile"), Line::Driftfile),
            parse_path_line(tag("dumpdir"), Line::Dumpdir),
            parse_path_line(tag("hwclockfile"), Line::HwClockFile),
            parse_string_line(tag("include"), Line::Include),
            parse_path_line(tag("keyfile"), Line::Keyfile),
            parse_string_line(tag("leapsectz"), Line::LeapsecTz),
            parse_path_line(tag("logdir"), Line::LogDir),
            parse_path_line(tag("ntpsigndsocket") /* sic */, Line::NtpSigndSocket),
            parse_path_line(tag("rtcfile"), Line::RtcFile),
            parse_string_line(tag("user"), Line::User),
        )),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    #[test]
    fn parse_test() {
        let cases = vec![
            ("allow all", Line::Allow(IpSet::All)),
            (
                "allow 127.0.0.1/32",
                Line::Allow(IpSet::Cidr("127.0.0.1/32".parse().unwrap())),
            ),
            (
                "bindacqaddress 1.2.3.4",
                Line::BindAcqAddress(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))),
            ),
            (
                "bindcmdaddress /var/sock",
                Line::BindCmdAddress(BindCmdAddress::Path("/var/sock".into())),
            ),
            ("clientloglimit 100", Line::ClientLogLimit(100)),
            (
                "broadcast 30 192.168.1.255",
                Line::Broadcast(Broadcast {
                    interval: 30,
                    addr: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 255)),
                    port: 123,
                }),
            ),
            (
                "broadcast 40 192.168.1.254 132",
                Line::Broadcast(Broadcast {
                    interval: 40,
                    addr: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 254)),
                    port: 132,
                }),
            ),
        ];

        for case in cases {
            let res: Result<_, nom::Err<VerboseError<&str>>> = parse_line(case.0);
            assert_eq!(res, Ok(("", case.1)));
        }
    }
}
