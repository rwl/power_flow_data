use arrayvec::ArrayString;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{char, digit1, space0};
use nom::combinator::{map, map_res, opt, recognize};
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;

use crate::{AreaNum, BusNum, Load, OwnerNum, ZoneNum};

fn _parse_integer(input: &str) -> IResult<&str, i32> {
    map_res(recognize(pair(opt(char('-')), digit1)), |s: &str| {
        s.parse::<i32>()
    })(input)
}

fn _parse_i8(input: &str) -> IResult<&str, i8> {
    map_res(digit1, |s: &str| s.parse::<i8>())(input)
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

// 111,'G1',1,227,   1,   -0.004,   -0.000,   -0.003,   -0.000,    0.000,   -0.000,  1 /* [STBC   G1                   ] */
fn parse_raw_load(input: &str) -> IResult<&str, Load> {
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_raw_load() {
        let input = "111,'G1',1,227,1,-0.004,-0.000,-0.003,-0.000,0.000,-0.000,1";
        // let input = "111, 'G1', 1, 227, 1, -0.004, -0.000, -0.003, -0.000, 0.000, -0.000, 1";
        let expected = Load {
            i: 111,
            id: ArrayString::from("G1").unwrap(),
            status: true,
            area: 227,
            zone: 1,
            pl: -0.004,
            ql: -0.0,
            ip: -0.003,
            iq: -0.0,
            yp: 0.0,
            yq: -0.0,
            owner: 1,
            scale: None,
            intrpt: None,
        };
        assert_eq!(parse_raw_load(input).unwrap().1, expected);
    }
}
