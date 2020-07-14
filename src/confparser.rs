use nom::IResult;
use std::default::Default;
use ipnet::{IpNet, Ipv4Net};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::character::is_space;
use nom::bytes::complete::take_while1;
use nom::error::{context, ParseError, VerboseError};
use nom::multi::many1;
use nom::branch::alt;
use nom::combinator::{map, map_res, not};
use nom::sequence::preceded;

// IpSet is the argument that allow and deny take.
enum IpSet {
    All,
    Cidr(IpNet),
}

enum Line {
    Nothing,
    AcquisitionPort(i64),
    Allow(IpSet),
    BindAcqAddress(std::net::IpAddr),
    BindAddress(std::net::IpAddr),
    BindCmdAddress(std::net::IpAddr),
    // TODO: all the ones below this
    Broadcast,
    ClientLogLimit,
    CmdAllow,
    CmdDeny,
    CmdPort,
    CmdRateLimit,
    CombineLimit,
    CorrTimeRatio,
    Deny,
    Driftfile,
    Dumpdir,
    DumpOnExit,
    FallbackDrift,
    HwClockFile,
    HwTimestamp,
    Include,
    InitStepSlew,
    Keyfile,
    LeapsecMode,
    LeapsecTz,
    Local,
    LockAll,
    Log,
    LogBanner,
    LogChange,
    LogDir,
    MailOnChange,
    MakeStep,
    Manual,
    MaxChange,
    MaxClockError,
    MaxDistance,
    MaxDrift,
    MaxJitter,
    MaxSamples,
    MaxSlewRate,
    MaxUpdateSkew,
    MinSamples,
    MinSources,
    NoClientLog,
    NtpSignedSocket,
    Peer,
    Pool,
    Port,
    RateLimit,
    RefClock,
    ReselectDist,
    RtcAutoTrim,
    RtcDevice,
    RtcFile,
    RtcOnUtc,
    RtcSync,
    SchedPriority,
    Server,
    SmoothTime,
    StratumWeight,
    TempComp,
    User,
}

fn parse_num<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, i64, E> {
    alt((
        map_res(digit1, |digits: &str| {
            digits.parse::<i64>()
        }),
        map_res(preceded(tag("-"), digit1), |digits: &str| {
            digits.parse::<i64>().map(|i| i * -1)
        })
    ))(i)
}

fn parse_acquisition_port<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    preceded(
        tag("acquisition_port"),
        preceded(space1, map(parse_num, |p| Line::AcquisitionPort(p))),
    )(i)
}

fn parse_ipset<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, IpSet, E> {
    alt((
        map(tag("all"), |_| IpSet::All),
        map_res(take_while1(|c: char| c.is_whitespace()), |p: &str| p.parse::<IpNet>().map(|c| IpSet::Cidr(c))),
    ))(i)
}

fn parse_allow<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    preceded(
        tag("allow"),
        preceded(space1, map(parse_ipset, |p| Line::Allow(p))),
    )(i)
}

fn parse_line<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    alt((
      parse_acquisition_port,
      parse_allow,
    ))(i)
}
