use nom::IResult;
use std::default::Default;
use ipnet::{IpNet, Ipv4Net};
use std::net::IpAddr;
use std::path::PathBuf;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::character::is_space;
use nom::bytes::complete::take_while1;
use nom::error::{context, ParseError, VerboseError};
use nom::multi::many1;
use nom::branch::alt;
use nom::combinator::{map, map_res, not, opt};
use nom::sequence::{preceded, tuple};

// IpSet is the argument that allow and deny take.
#[derive(Debug, PartialEq, Eq)]
enum IpSet {
    All,
    Cidr(IpNet),
}

#[derive(Debug, PartialEq, Eq)]
enum BindCmdAddress {
    IpAddr(IpAddr),
    Path(PathBuf)
}

#[derive(Debug, PartialEq, Eq)]
struct Broadcast {
    interval: i64,
    addr: IpAddr,
    port: u16,
}

#[derive(Debug, PartialEq, Eq)]
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
    // TODO: all the ones below this
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

fn parse_port<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, u16, E> {
    map_res(digit1, |digits: &str| {
        digits.parse::<u16>()
    })(i)
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
        map_res(take_while1(|c: char| !c.is_whitespace()), |p: &str| p.parse::<IpNet>().map(|c| IpSet::Cidr(c))),
    ))(i)
}

fn parse_allow<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    preceded(
        tag("allow"),
        preceded(space1, map(parse_ipset, |p| Line::Allow(p))),
    )(i)
}

fn parse_address<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, IpAddr, E> {
    map_res(take_while1(|c: char| !c.is_whitespace()), |p: &str| p.parse::<IpAddr>())(i)
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

fn parse_string<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    take_while1(|c| c != '\r' && c != '\n')(i)
}

fn parse_path<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, PathBuf, E> {
    map(parse_string, |s| PathBuf::from(s))(i)
}


fn parse_bindcmdaddress<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    preceded(
        tag("bindcmdaddress"),
        preceded(space1,
            alt((
                map(parse_address, |p| Line::BindCmdAddress(BindCmdAddress::IpAddr(p))),
                map(parse_path, |p| Line::BindCmdAddress(BindCmdAddress::Path(p))),
            ))
        )
    )(i)
}

fn parse_broadcast<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    map(preceded(
        tag("broadcast"),
        tuple((parse_num, parse_address, opt(parse_port))),
    ), |(interval, addr, port)| {
        Line::Broadcast(Broadcast{
            interval,
            addr,
            port: port.unwrap_or(123),
        })
    }
    )(i)
}

fn parse_clientloglimit<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    map(preceded(
        tag("clientloglimit"),
        parse_num,
    ), |num: i64| {
        Line::ClientLogLimit(num)
    })(i)
}

fn parse_line<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Line, E> {
    alt((
      parse_acquisition_port,
      parse_allow,
      parse_bindacqaddress,
      parse_bindaddress,
      parse_bindcmdaddress,
      parse_broadcast,
      parse_clientloglimit,
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
            ("allow 127.0.0.1/32", Line::Allow(IpSet::Cidr("127.0.0.1/32".parse().unwrap()))),
            ("bindacqaddress 1.2.3.4", Line::BindAcqAddress(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)))),
            ("bindcmdaddress /var/sock", Line::BindCmdAddress(BindCmdAddress::Path("/var/sock".into()))),
        ];

        for case in cases {
            let res: Result<_, nom::Err<VerboseError<&str>>> = parse_line(case.0);
            assert_eq!(res, Ok(("", case.1)));
        }

    }
}
