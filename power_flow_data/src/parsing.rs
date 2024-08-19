use arrayvec::ArrayString;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{char, digit1, newline, space0, space1};
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use std::str::FromStr;

use crate::{AreaNum, Bus, BusNum, CaseID, Load, Network, OwnerNum, ZoneNum};

fn _parse_integer(input: &str) -> IResult<&str, i32> {
    map_res(recognize(pair(opt(char('-')), digit1)), |s: &str| {
        s.parse::<i32>()
    })(input)
}

fn parse_i8(input: &str) -> IResult<&str, i8> {
    map_res(digit1, |s: &str| s.parse::<i8>())(input)
}

fn parse_int<I: FromStr>(input: &str) -> IResult<&str, I> {
    map_res(digit1, |s: &str| s.parse::<I>())(input)
}

fn parse_bus_num(input: &str) -> IResult<&str, BusNum> {
    map_res(digit1, |s: &str| s.parse::<BusNum>())(input)
}

fn _parse_metered_bus_num(input: &str) -> IResult<&str, BusNum> {
    // Bus number can be negative to indicate the metered end of a branch.
    map_res(recognize(preceded(opt(char('-')), digit1)), |s: &str| {
        s.parse::<BusNum>()
    })(input)
}

fn parse_area_num(input: &str) -> IResult<&str, AreaNum> {
    map_res(digit1, |s: &str| s.parse::<AreaNum>())(input)
}

fn parse_zone_num(input: &str) -> IResult<&str, ZoneNum> {
    map_res(digit1, |s: &str| s.parse::<ZoneNum>())(input)
}

fn parse_owner_num(input: &str) -> IResult<&str, OwnerNum> {
    map_res(digit1, |s: &str| s.parse::<OwnerNum>())(input)
}

fn _parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

// fn parse_f64(input: &str) -> IResult<&str, f64> {
//     map_res(
//         take_while(|c: char| c.is_digit(10) || c == '.'),
//         |s: &str| s.parse::<f64>(),
//     )(input)
// }

fn parse_i32(input: &str) -> IResult<&str, i32> {
    // Define a parser that can handle an optional minus sign followed by digits
    let parse_signed = recognize(preceded(opt(char('-')), digit1));

    // Parse the resulting string into an i32
    map_res(parse_signed, |s: &str| s.parse::<i32>())(input)
}

fn parse_f64(input: &str) -> IResult<&str, f64> {
    map_res(
        recognize(tuple((
            opt(char('-')),
            digit1,
            opt(tuple((char('.'), digit1))),
        ))),
        |s: &str| s.parse::<f64>(),
    )(input)
}

fn _parse_optional_f64(input: &str) -> IResult<&str, Option<f64>> {
    opt(parse_f64)(input)
}

fn parse_optional_bool(input: &str) -> IResult<&str, Option<bool>> {
    opt(parse_bool)(input)
}

fn _parse_float(input: &str) -> IResult<&str, f64> {
    map_res(
        recognize(pair(opt(char('-')), pair(digit1, pair(char('.'), digit1)))),
        |s: &str| s.parse::<f64>(),
    )(input)
}

fn parse_bool(input: &str) -> IResult<&str, bool> {
    map(parse_i32, |i| i != 0)(input)
}

fn _parse_string(input: &str) -> IResult<&str, &str> {
    delimited(char('\''), take_until("'"), char('\''))(input)
}

fn parse_array_string<const CAP: usize>(input: &str) -> IResult<&str, ArrayString<CAP>> {
    map_res(
        delimited(char('\''), take_while(|c| c != '\''), char('\'')),
        |s: &str| ArrayString::<CAP>::try_from(s),
    )(input)
}

fn _parse_comment(input: &str) -> IResult<&str, &str> {
    delimited(tag("/*"), take_until("*/"), tag("*/"))(input)
}

fn parse_zero_line(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((char('0'), opt(space1), newline))(input)?;
    Ok((input, ()))
}

//  0,    100.00, 33, 0, 0, 60.00       / May 16, 2017 17:17:11; Simulator Version 20 Beta; BuildDate 2017_5_15
pub(crate) fn parse_raw_case_id(input: &str) -> IResult<&str, CaseID> {
    let (input, _) = space0(input)?;

    let (input, ic) = parse_i8(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, sbase) = parse_f64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, rev) = opt(parse_int::<usize>)(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, xfrrat) = opt(parse_i8)(input)?;
    let (input, _) = opt(char(','))(input)?;
    let (input, _) = space0(input)?;

    let (input, nxfrat) = opt(parse_i8)(input)?;
    let (input, _) = opt(char(','))(input)?;
    let (input, _) = space0(input)?;

    let (input, basfrq) = opt(parse_f64)(input)?;

    let (input, _) = opt(separated_pair(char('/'), space0, take_until("\n")))(input)?;

    let case_id = CaseID {
        ic,
        sbase,
        rev,
        xfrrat,
        nxfrat,
        basfrq,
    };

    Ok((input, case_id))
}

// 111,'STBC      ',161.00,1,    0.00,    0.00,227,   1,1.09814,  -8.327,  1 /* [STBC   1   ] */
pub(crate) fn parse_raw_bus(input: &str) -> IResult<&str, Bus> {
    let (input, _) = space0(input)?;

    let (input, i) = parse_bus_num(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, name) = parse_array_string(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, basekv) = parse_f64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, ide) = parse_i8(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, area) = parse_area_num(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, zone) = parse_zone_num(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, owner) = parse_owner_num(input)?;
    let (input, _) = opt(char(','))(input)?;
    let (input, _) = space0(input)?;

    let (input, vm) = parse_f64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, va) = parse_f64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, nvhi) = parse_f64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, nvlo) = parse_f64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, evhi) = parse_f64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, evlo) = parse_f64(input)?;

    let bus = Bus {
        i,
        name,
        basekv,
        ide,
        area,
        zone,
        owner,
        vm,
        va,
        nvhi,
        nvlo,
        evhi,
        evlo,
    };

    Ok((input, bus))
}

fn parse_raw_buses(input: &str) -> IResult<&str, Vec<Bus>> {
    separated_list1(newline, parse_raw_bus)(input)
}

// 111,'G1',1,227,   1,   -0.004,   -0.000,   -0.003,   -0.000,    0.000,   -0.000,  1 /* [STBC   G1                   ] */
pub(crate) fn parse_raw_load(input: &str) -> IResult<&str, Load> {
    let (input, i) = parse_bus_num(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, id) = parse_array_string(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, status) = parse_bool(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, area) = parse_area_num(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, zone) = parse_zone_num(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, pl) = parse_f64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, ql) = parse_f64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, ip) = parse_f64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, iq) = parse_f64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, yp) = parse_f64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, yq) = parse_f64(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;

    let (input, owner) = parse_owner_num(input)?;
    let (input, _) = opt(char(','))(input)?;
    let (input, _) = space0(input)?;

    let (input, scale) = parse_optional_bool(input)?;
    let (input, _) = opt(char(','))(input)?;
    let (input, _) = space0(input)?;

    let (input, intrpt) = parse_optional_bool(input)?;
    let (input, _) = opt(char(','))(input)?;
    let (input, _) = space0(input)?;

    let load = Load {
        i,
        id,
        status,
        area,
        zone,
        pl,
        ql,
        ip,
        iq,
        yp,
        yq,
        owner,
        scale,
        intrpt,
    };

    Ok((input, load))
}

pub(crate) fn parse_raw_loads(input: &str) -> IResult<&str, Vec<Load>> {
    separated_list1(newline, parse_raw_load)(input)
}

pub fn parse_raw_case(input: &str) -> IResult<&str, Network> {
    let (input, caseid) = parse_raw_case_id(input)?;
    let (input, buses) = parse_raw_buses(input)?;
    let (input, _) = parse_zero_line(input)?;
    let (input, loads) = parse_raw_loads(input)?;

    let network = Network {
        version: 0,
        caseid,
        buses,
        loads,
        fixed_shunts: vec![],
        generators: vec![],
        branches: vec![],
        transformers: vec![],
        area_interchanges: vec![],
        two_terminal_dc: vec![],
        vsc_dc: vec![],
        switched_shunts: vec![],
        impedance_corrections: vec![],
        multi_terminal_dc: vec![],
        multi_section_lines: vec![],
        zones: vec![],
        area_transfers: vec![],
        owners: vec![],
        facts: vec![],
    };

    Ok((input, network))
}
