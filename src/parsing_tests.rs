use crate::{Bus, CaseID, Load};
use arrayvec::ArrayString;

#[test]
fn test_parse_raw_case_id() {
    // let input = " 0,    100.00, 33, 0, 0, 60.00       / May 16, 2017 17:17:11; Simulator Version 20 Beta; BuildDate 2017_5_15";
    let input = " 0,    100.00, 33, 0, 0, 60.00";
    let expected = CaseID {
        ic: 0,
        sbase: 100.0,
        rev: Some(33),
        xfrrat: Some(0),
        nxfrat: Some(0),
        basfrq: Some(60.0),
    };
    assert_eq!(
        crate::parsing::parse_raw_case_id(input).unwrap().1,
        expected
    );
}

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
    assert_eq!(crate::parsing::parse_raw_load(input).unwrap().1, expected);
}

#[test]
fn test_parse_raw_loads() {
    let input = "111,'G1',1,227,1,-0.004,-0.000,-0.003,-0.000,0.000,-0.000,1
113,' G2',1,227,   1,    0.345,    0.024,    0.711,    0.076,   -0.028,    0.003,  2";
    // let input = "111, 'G1', 1, 227, 1, -0.004, -0.000, -0.003, -0.000, 0.000, -0.000, 1";
    let expected = vec![
        Load {
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
        },
        Load {
            i: 113,
            id: ArrayString::from(" G2").unwrap(),
            status: true,
            area: 227,
            zone: 1,
            pl: 0.345,
            ql: 0.024,
            ip: 0.711,
            iq: 0.076,
            yp: -0.028,
            yq: 0.003,
            owner: 2,
            scale: None,
            intrpt: None,
        },
    ];
    assert_eq!(crate::parsing::parse_raw_loads(input).unwrap().1, expected);
}

#[test]
fn test_parse_raw_bus() {
    // let input = "111,'STBC      ',161.00,1,    0.00,    0.00,227,   1,1.09814,  -8.327,  1 /* [STBC   1   ] */";
    let input = "    1,'WINNSBORO 0 ', 138.0000,1,   1,   2,   1,1.01215947, -10.768348, 1.10000, 0.90000, 1.10000, 0.90000";
    let expected = Bus {
        i: 1,
        name: ArrayString::from("WINNSBORO 0 ").unwrap(),
        basekv: 138.0,
        ide: 1,
        area: 1,
        zone: 2,
        owner: 1,
        vm: 1.01215947,
        va: -10.768348,
        nvhi: 1.1,
        nvlo: 0.9,
        evhi: 1.1,
        evlo: 0.9,
    };
    let actual = crate::parsing::parse_raw_bus(input).unwrap().1;
    assert_eq!(actual, expected);
}
