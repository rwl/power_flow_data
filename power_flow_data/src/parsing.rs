use arrayvec::ArrayString;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{char, digit1, multispace0, newline, space0, space1};
use nom::combinator::{map_res, opt, recognize, value};
use nom::multi::separated_list1;
use nom::number::complete::double;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::IResult;
use std::str::FromStr;

use crate::{AreaNum, Bus, BusNum, CaseID, FixedShunt, Load, Network, OwnerNum, Stat, ZoneNum};

// Parser combinators

/// Parse a positive integer
pub(crate) fn parse_uint<T: FromStr>(input: &str) -> IResult<&str, T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    map_res(recognize(digit1), |s: &str| s.parse::<T>())(input)
}

/// Parse a string enclosed in single quotes
pub(crate) fn parse_quoted_string(input: &str) -> IResult<&str, &str> {
    delimited(char('\''), take_until("'"), char('\''))(input)
}

/// Parse an optional comma followed by whitespace
pub(crate) fn comma_ws(input: &str) -> IResult<&str, ()> {
    value((), tuple((opt(char(',')), space0)))(input)
}

/// Parse a floating point number
pub(crate) fn parse_float(input: &str) -> IResult<&str, f64> {
    double(input)
}

fn _parse_integer(input: &str) -> IResult<&str, i32> {
    map_res(recognize(pair(opt(char('-')), digit1)), |s: &str| {
        s.parse::<i32>()
    })(input)
}

/// Parse a boolean value from 0/1
pub(crate) fn parse_bool(input: &str) -> IResult<&str, bool> {
    alt((value(false, tag("0")), value(true, tag("1"))))(input)
}

/// Parse a status value (0 or 1)
fn parse_status(input: &str) -> IResult<&str, Stat> {
    map_res(alt((tag("0"), tag("1"))), |s: &str| s.parse::<Stat>())(input)
}

pub(crate) fn parse_i8(input: &str) -> IResult<&str, i8> {
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

// fn parse_i32(input: &str) -> IResult<&str, i32> {
//     // Define a parser that can handle an optional minus sign followed by digits
//     let parse_signed = recognize(preceded(opt(char('-')), digit1));
//
//     // Parse the resulting string into an i32
//     map_res(parse_signed, |s: &str| s.parse::<i32>())(input)
// }

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

// fn parse_bool(input: &str) -> IResult<&str, bool> {
//     map(parse_i32, |i| i != 0)(input)
// }

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

    let (input, status) = parse_i8(input)?;
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

/// Parse a fixed shunt record
pub fn parse_fixed_shunt(input: &str) -> IResult<&str, FixedShunt> {
    let (input, i) = terminated(parse_uint, comma_ws)(input)?;
    let (input, id_str) = terminated(parse_quoted_string, comma_ws)(input)?;

    // Create ArrayString from the parsed string
    let mut id = ArrayString::<3>::new();
    if id_str.len() <= 3 {
        id.push_str(id_str);
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TooLarge,
        )));
    }

    let (input, status) = terminated(parse_status, comma_ws)(input)?;
    let (input, gl) = terminated(parse_float, comma_ws)(input)?;
    let (input, bl) = parse_float(input)?; // Last field, no comma

    Ok((
        input,
        FixedShunt {
            i,
            id,
            status,
            gl,
            bl,
        },
    ))
}

/// Parse multiple fixed shunt records, one per line
pub fn parse_fixed_shunts(input: &str) -> IResult<&str, Vec<FixedShunt>> {
    let mut shunts = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        match parse_fixed_shunt(remaining) {
            Ok((rest, shunt)) => {
                shunts.push(shunt);
                // Skip any whitespace including newlines to get to the next record
                let (rest, _) = multispace0(rest)?;
                remaining = rest;
            }
            Err(_) => break,
        }
    }

    Ok((remaining, shunts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fixed_shunt() {
        let input = "9,' 1', 1,     0.000,    19.000";
        let (_, shunt) = parse_fixed_shunt(input).unwrap();

        assert_eq!(shunt.i, 9);
        assert_eq!(shunt.id.as_str(), " 1");
        assert_eq!(shunt.status, 1);
        assert_eq!(shunt.gl, 0.0);
        assert_eq!(shunt.bl, 19.0);
    }

    #[test]
    fn test_parse_multiple_fixed_shunts() {
        let input = "9,' 1', 1,     0.000,    19.000
11,' 1', 1,     0.000,    10.000";

        let (_, shunts) = parse_fixed_shunts(input).unwrap();

        assert_eq!(shunts.len(), 2);
        assert_eq!(shunts[0].i, 9);
        assert_eq!(shunts[0].bl, 19.0);
        assert_eq!(shunts[1].i, 11);
        assert_eq!(shunts[1].bl, 10.0);
    }
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
