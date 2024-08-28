use power_flow_data::{
    Branch, Bus, CaseID, FixedShunt, Generator, Load, Network, Transformer, GENBUS, SLACKBUS,
};

// IEEE14 returns an instance of the IEEE 14 terminal test network.
#[allow(non_snake_case)]
pub fn ieee14() -> Network {
    let Sb = 100.0;

    let buses = vec![
        Bus {
            i: 1,
            basekv: 69.0,
            ide: SLACKBUS,
            ..Default::default()
        },
        Bus {
            i: 2,
            basekv: 69.0,
            ide: GENBUS,
            ..Default::default()
        },
        Bus {
            i: 3,
            basekv: 69.0,
            ide: GENBUS,
            ..Default::default()
        },
        Bus {
            i: 4,
            basekv: 69.0,
            ..Default::default()
        },
        Bus {
            i: 5,
            basekv: 69.0,
            ..Default::default()
        },
        Bus {
            i: 6,
            basekv: 13.8,
            ide: GENBUS,
            ..Default::default()
        },
        Bus {
            i: 7,
            basekv: 13.8,
            ..Default::default()
        },
        Bus {
            i: 8,
            basekv: 18.0,
            ide: GENBUS,
            ..Default::default()
        },
        Bus {
            i: 9,
            basekv: 13.8,
            ..Default::default()
        },
        Bus {
            i: 10,
            basekv: 13.8,
            ..Default::default()
        },
        Bus {
            i: 11,
            basekv: 13.8,
            ..Default::default()
        },
        Bus {
            i: 12,
            basekv: 13.8,
            ..Default::default()
        },
        Bus {
            i: 13,
            basekv: 13.8,
            ..Default::default()
        },
        Bus {
            i: 14,
            basekv: 13.8,
            ..Default::default()
        },
    ];

    // fn new_branch(
    //     i: BusNum,
    //     j: BusNum,
    //     Rpu: f64,
    //     Xpu: f64,
    //     Bpu: f64,
    //     online: bool,
    //     Vn: f64,
    // ) -> Branch {
    //     Branch {
    //         i,
    //         j,
    //         r: Rpu,
    //         x: Xpu,
    //         b: Bpu,
    //         st: online,
    //         ..Default::default()
    //     }
    // }

    // let branches = vec![
    //     new_branch(0, 1, 0.01938, 0.05917, 0.0528, true, 69.0),
    //     new_branch(0, 4, 0.05403, 0.22304, 0.0492, true, 69.0),
    //     new_branch(1, 2, 0.04699, 0.19797, 0.0438, true, 69.0),
    //     new_branch(1, 3, 0.05811, 0.17632, 0.0374, true, 69.0),
    //     new_branch(1, 4, 0.05695, 0.17388, 0.0340, true, 69.0),
    //     new_branch(2, 3, 0.06701, 0.17103, 0.0346, true, 69.0),
    //     new_branch(3, 4, 0.01335, 0.04211, 0.0128, true, 69.0),
    //     new_branch(5, 10, 0.09498, 0.19890, 0.0, true, 13.8),
    //     new_branch(5, 11, 0.12291, 0.25581, 0.0, true, 13.8),
    //     new_branch(5, 12, 0.06615, 0.13027, 0.0, true, 13.8),
    //     new_branch(6, 8, 0.0, 0.11001, 0.0, true, 13.8),
    //     new_branch(8, 9, 0.03181, 0.08450, 0.0, true, 13.8),
    //     new_branch(8, 13, 0.12711, 0.27038, 0.0, true, 13.8),
    //     new_branch(9, 10, 0.08205, 0.19207, 0.0, true, 13.8),
    //     new_branch(11, 12, 0.22092, 0.19988, 0.0, true, 13.8),
    //     new_branch(12, 13, 0.17093, 0.34802, 0.0, true, 13.8),
    // ];

    let branches = vec![
        Branch {
            i: 1,
            j: 2,
            r: 0.01938,
            x: 0.05917,
            b: 0.0528,
            ..Default::default()
        },
        Branch {
            i: 1,
            j: 5,
            r: 0.05403,
            x: 0.22304,
            b: 0.0492,
            ..Default::default()
        },
        Branch {
            i: 2,
            j: 3,
            r: 0.04699,
            x: 0.19797,
            b: 0.0438,
            ..Default::default()
        },
        Branch {
            i: 2,
            j: 4,
            r: 0.05811,
            x: 0.17632,
            b: 0.0374,
            ..Default::default()
        },
        Branch {
            i: 2,
            j: 5,
            r: 0.05695,
            x: 0.17388,
            b: 0.0340,
            ..Default::default()
        },
        Branch {
            i: 3,
            j: 4,
            r: 0.06701,
            x: 0.17103,
            b: 0.0346,
            ..Default::default()
        },
        Branch {
            i: 4,
            j: 5,
            r: 0.01335,
            x: 0.04211,
            b: 0.0128,
            ..Default::default()
        },
        Branch {
            i: 6,
            j: 11,
            r: 0.09498,
            x: 0.19890,
            ..Default::default()
        },
        Branch {
            i: 6,
            j: 12,
            r: 0.12291,
            x: 0.25581,
            ..Default::default()
        },
        Branch {
            i: 6,
            j: 13,
            r: 0.06615,
            x: 0.13027,
            ..Default::default()
        },
        Branch {
            i: 7,
            j: 9,
            x: 0.11001,
            ..Default::default()
        },
        Branch {
            i: 9,
            j: 10,
            r: 0.03181,
            x: 0.08450,
            ..Default::default()
        },
        Branch {
            i: 9,
            j: 14,
            r: 0.12711,
            x: 0.27038,
            ..Default::default()
        },
        Branch {
            i: 10,
            j: 11,
            r: 0.08205,
            x: 0.19207,
            ..Default::default()
        },
        Branch {
            i: 12,
            j: 13,
            r: 0.22092,
            x: 0.19988,
            ..Default::default()
        },
        Branch {
            i: 13,
            j: 14,
            r: 0.17093,
            x: 0.34802,
            ..Default::default()
        },
    ];

    //const Sn = 100

    // fn new_transformer(
    //     i: BusNum,
    //     j: BusNum,
    //     Rpu: f64,
    //     Xpu: f64,
    //     tap: f64,
    //     shift: f64,
    //     online: bool,
    //     Vn: f64,
    // ) -> Transformer {
    //     Transformer {
    //         i,
    //         j,
    //         r1_2: Rpu,
    //         x1_2: Xpu,
    //         B: 0,
    //         windv1: tap,
    //         ang1: shift,
    //         stat: if online { 1 } else { 0 },
    //     }
    // }
    let transformers = vec![
        Transformer {
            i: 3,
            j: 6,
            x1_2: 0.20912,
            windv1: 0.978,
            ..Default::default()
        }, // 69:13.8
        Transformer {
            i: 3,
            j: 8,
            x1_2: 0.55618,
            windv1: 0.969,
            ..Default::default()
        }, // 69:13.8
        Transformer {
            i: 4,
            j: 5,
            x1_2: 0.25202,
            windv1: 0.932,
            ..Default::default()
        }, // 69:13.8
        Transformer {
            i: 6,
            j: 7,
            x1_2: 0.17615,
            windv1: 1.0,
            ..Default::default()
        }, // 13.8:18
    ];

    let loads = vec![
        Load {
            i: 2,
            pl: 0.217 * Sb,
            ql: 0.127 * Sb,
            ..Default::default()
        },
        Load {
            i: 3,
            pl: 0.942 * Sb,
            ql: 0.19 * Sb,
            ..Default::default()
        },
        Load {
            i: 4,
            pl: 0.478 * Sb,
            ql: -0.039 * Sb,
            ..Default::default()
        },
        Load {
            i: 5,
            pl: 0.076 * Sb,
            ql: 0.016 * Sb,
            ..Default::default()
        },
        Load {
            i: 6,
            pl: 0.112 * Sb,
            ql: 0.075 * Sb,
            ..Default::default()
        },
        Load {
            i: 9,
            pl: 0.295 * Sb,
            ql: 0.166 * Sb,
            ..Default::default()
        },
        Load {
            i: 10,
            pl: 0.09 * Sb,
            ql: 0.058 * Sb,
            ..Default::default()
        },
        Load {
            i: 11,
            pl: 0.035 * Sb,
            ql: 0.018 * Sb,
            ..Default::default()
        },
        Load {
            i: 12,
            pl: 0.061 * Sb,
            ql: 0.016 * Sb,
            ..Default::default()
        },
        Load {
            i: 13,
            pl: 0.135 * Sb,
            ql: 0.058 * Sb,
            ..Default::default()
        },
        Load {
            i: 14,
            pl: 0.149 * Sb,
            ql: 0.05 * Sb,
            ..Default::default()
        },
    ];

    let generators = vec![
        Generator {
            i: 1,
            // pg: 2.324 * Sb,
            qt: 9.9 * Sb,
            qb: -9.9 * Sb,
            vs: 1.06,
            ..Default::default()
        },
        Generator {
            i: 2,
            pg: 0.4 * Sb,
            qt: 0.5 * Sb,
            qb: -0.4 * Sb,
            vs: 1.045,
            ..Default::default()
        },
        Generator {
            i: 3,
            qt: 0.4 * Sb,
            qb: f64::NEG_INFINITY,
            vs: 1.01,
            ..Default::default()
        },
        Generator {
            i: 6,
            qt: 0.24 * Sb,
            qb: -0.06 * Sb,
            vs: 1.07,
            ..Default::default()
        },
        Generator {
            i: 8,
            qt: 0.24 * Sb,
            qb: -0.06 * Sb,
            vs: 1.09,
            ..Default::default()
        },
    ];

    let shunt = FixedShunt {
        i: 9,
        bl: 0.19 * Sb,
        ..Default::default()
    };

    Network {
        caseid: CaseID {
            sbase: Sb,
            ..Default::default()
        },
        buses,
        loads,
        fixed_shunts: vec![shunt],
        generators,
        branches,
        transformers,
        ..Default::default()
    }
}
