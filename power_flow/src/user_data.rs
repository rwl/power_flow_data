use crate::y::build_y;
use anyhow::Result;
use arrayvec::ArrayString;
use num_complex::Complex64;
use power_flow_data::{BusNum, FixedShunt, Generator, Load, Network};
use sparsetools::csr::CSR;
use sparsetools::Complex;
use std::collections::HashMap;
use sundials::context::Context;
use sundials::nvector;
use sundials::nvector::NVector;

#[derive(Clone)]
pub(super) struct UserData {
    // pub(super) struct UserData<'a> {
    // pub(super) network: &'a Network,
    // pub(super) network: Network,
    pub(super) s_base: f64,
    pub(super) loads: Vec<Load>,
    pub(super) generators: Vec<Generator>,
    pub(super) fixed_shunts: Vec<FixedShunt>,

    pub(super) a: HashMap<BusNum, usize>,
    pub(super) v: HashMap<BusNum, usize>,
    pub(super) q: HashMap<ArrayString<3>, usize>,
    pub(super) p: HashMap<ArrayString<3>, usize>,
    pub(super) slack: BusNum,

    // pub(super) v_base: HashMap<BusNum, f64>, // kV
    pub(super) ang0: HashMap<BusNum, f64>, // radians

    pub(super) y_mat: CSR<usize, Complex64>,
}

// impl<'a> UserData<'a> {
impl UserData {
    // pub(super) fn new(network: &'a Network) -> Self {
    pub(super) fn new(network: Network) -> Self {
        let mut a = HashMap::default();
        let mut v = HashMap::default();
        let mut q = HashMap::default();
        let mut p = HashMap::default();
        // let mut v_base = HashMap::default();
        let mut ang0 = HashMap::default();

        let nb = network.buses.len();
        let ng = network.generators.len();
        // let index = BusbarIndex::new(&ac_system.busbars);
        // for _, sub := range nd.net.Substations {
        //     for _, vl := range sub.VoltageLevels {
        //         nb += len(vl.Buses)
        //         ng += len(vl.Generators)
        //     }
        // }
        // let (node_index, n) = ac_system.nodes();

        let slack: Vec<BusNum> = network
            .buses
            .iter()
            .filter(|bus| bus.ide == 3)
            .map(|bus| bus.i)
            .collect();
        assert_eq!(slack.len(), 1);
        let slack = slack[0];

        // let mut i = 0;
        // let mut j = 0;
        let mut k = 0;
        // for _, sub := range nd.net.Substations {
        //     for _, vl := range sub.VoltageLevels {
        for (i, bus) in network.buses.iter().enumerate() {
            //nd.buses[bus.Id] = i
            a.insert(bus.i, i);
            v.insert(bus.i, nb + i);

            // v_base.insert(bus.i, bus.basekv);
            ang0.insert(bus.i, bus.va.to_radians());
            // i += 1;
        }
        for (j, gen) in network.generators.iter().enumerate() {
            q.insert(gen.id, 2 * nb + j);
            // j += 1;

            // if gen.ParticipationFactor == 1 {
            if gen.i == slack {
                p.insert(gen.id, 2 * nb + ng + k);
                k += 1;
            }
        }
        // }
        // }&

        let y_mat = build_y(&network, &a /*, &v_base*/);

        // if Debug {
        println!("a: {:?}", a);
        println!("v: {:?}", v);
        println!("q: {:?}", q);
        println!("p: {:?}", p);
        // }

        Self {
            // network,
            s_base: network.caseid.sbase,

            loads: network.loads.clone(),
            generators: network.generators.clone(),
            fixed_shunts: network.fixed_shunts.clone(),

            a,
            v,
            q,
            p,
            slack,
            // v_base,
            ang0,

            y_mat,
        }
    }
}

pub struct Flow {
    pub pi: f64,
    pub qi: f64,
    pub pj: f64,
    pub qj: f64,
    pub schgi: Complex64,
    pub schgj: Complex64,
}

pub fn post_process(
    network: &mut Network,
    y: &NVector,
    a: &HashMap<BusNum, usize>,
    v: &HashMap<BusNum, usize>,
    q: &HashMap<ArrayString<3>, usize>,
    p: &HashMap<ArrayString<3>, usize>,
    slack: BusNum,
) -> Result<(Vec<Flow>, Vec<Flow>)> {
    let s_base = network.caseid.sbase;
    let yd = y.as_slice();

    let mut brch_flows = Vec::with_capacity(network.branches.len());
    let mut tfmr_flows = Vec::with_capacity(network.transformers.len());

    // for _, sub := range nd.net.Substations {
    //     for _, vl := range sub.VoltageLevels {
    for bus in &mut network.buses {
        let a = a[&bus.i];
        let v = v[&bus.i];

        bus.va = yd[a].to_degrees();
        bus.vm = yd[v];
    }
    for gen in &mut network.generators {
        // if gen.Bus != "" {
        let q = q[gen.id.as_str()];
        gen.qg = yd[q] * s_base;

        // if gen.ParticipationFactor == 1 {
        if gen.i == slack {
            let p = p[gen.id.as_str()];
            gen.pg = yd[p] * s_base;
        } else {
            // gen.pg = gen.TargetP
        }
        // } else {
        //     gen.Q = 0
        //     gen.P = 0
        // }
    }
    // }

    #[allow(non_snake_case)]
    for tr in &mut network.transformers {
        // if tr.Bus1 == "" || tr.Bus2 == "" { // TODO: semi-connected
        //     tr.P1 = 0
        //     tr.Q1 = 0
        //     tr.P2 = 0
        //     tr.Q2 = 0
        //     continue
        // }
        let Sb = Complex64::new(s_base, 0.0);

        let (g, b) = match tr.cm {
            1 => (tr.mag1, tr.mag2),
            _ => {
                panic!("cm must be 1: {}", tr.cm);
            }
        };

        let y0 = 0.5 * Complex64::new(g, b);
        let y12 = 1.0 / Complex64::new(tr.r1_2, tr.x1_2);

        let tap = tr.windv1;
        let phi = tr.ang1;
        let m = Complex64::from_polar(tap, phi.to_radians());
        let mconj = m.conj();
        let m2 = Complex64::new(m.norm().powi(2), 0.0);

        let a1 = a[&tr.i];
        let a2 = a[&tr.j];
        let v1 = v[&tr.i];
        let v2 = v[&tr.j];

        let V1 = Complex64::new(yd[v1], yd[a1]);
        let V2 = Complex64::new(yd[v2], yd[a2]);

        let I1 = (V1 * ((y12 + y0/*+ y1*/) / m2)) - (V2 * (y12 / mconj));
        let I2 = (V2 * (y12 + y0/*+ y2*/)) - (V1 * (y12 / m));
        let S1 = V1 * I1.conj() * Sb;
        let S2 = V2 * I2.conj() * Sb;

        tfmr_flows.push(Flow {
            pi: S1.real(),
            qi: S1.imag(),
            pj: S2.real(),
            qj: S2.imag(),
            schgi: y0 * (V1.powi(2) / m2) * Sb,
            schgj: y0 * V2.powi(2) * Sb,
        });
    }
    // }

    #[allow(non_snake_case)]
    for l in &network.branches {
        // if l.Bus1 == "" || l.Bus2 == "" { // TODO: semi-connected
        //     l.P1 = 0
        //     l.Q1 = 0
        //     l.P2 = 0
        //     l.Q2 = 0
        //     continue
        // }
        let Sb = Complex64::new(s_base, 0.0);

        let y0 = 0.5 * Complex64::new(0.0, l.b);
        // let y1 = Complex64::new(l.G1, l.B1);
        // let y2 = Complex64::new(l.G2, l.B2);
        let y1 = Complex64::new(l.gi, l.bi);
        let y2 = Complex64::new(l.gj, l.bj);
        let y12 = Complex64::new(1.0, 0.0) / Complex64::new(l.r, l.x);

        let a1 = a[&l.i];
        let a2 = a[&l.j];
        let v1 = v[&l.i];
        let v2 = v[&l.j];

        let V1 = Complex64::new(yd[v1], yd[a1]);
        let V2 = Complex64::new(yd[v2], yd[a2]);

        let I1 = (V1 * (y12 + y0 + y1)) - (V2 * y12);
        let I2 = (V2 * (y12 + y0 + y2)) - (V1 * y12);
        let S1 = V1 * I1.conj() * Sb;
        let S2 = V2 * I2.conj() * Sb;

        brch_flows.push(Flow {
            pi: S1.real(),
            qi: S1.imag(),
            pj: S2.real(),
            qj: S2.imag(),
            schgi: Complex64::new(l.gi, l.bi) * V1.powi(2) * Sb,
            schgj: Complex64::new(l.gj, l.bj) * V2.powi(2) * Sb,
        });
    }

    // Buses with more than one generator have the total reactive power
    // dispatch for the bus divided equally among each online generator.
    // The reactive power is divided in proportion to the reactive range
    // of each generator, according to the logic in the pfsoln function
    // by Ray Zimmerman from MATPOWER v7.
    let mut cg = HashMap::<ArrayString<3>, Vec<usize>>::new();
    // for _, sub := range nd.net.Substations {
    //     for _, vl := range sub.VoltageLevels {
    for (j, gen) in network.generators.iter().enumerate() {
        // if gen.Bus != "" {
        if !cg.contains_key(&gen.id) {
            cg.insert(gen.id, vec![]);
        }
        cg.get_mut(&gen.id).unwrap().push(j);
        // }
    }
    // }
    // }

    for (_, l) in &cg {
        if l.len() < 2 {
            continue;
        }
        let mut qg_tot: f64 = 0.0; // Total Qg at the bus.
        for &g in l {
            qg_tot += network.generators[g].qg;
        }

        // The sum of absolute Qg, Qmax and Qmin for all generators
        // at the bus (if Qmax/Qmin is non-infinite). Used as limit
        // when Qmax/Qmin is infinite.
        let mut m: f64 = 0.0;
        for &j in l {
            let pv = &network.generators[j];
            let mut mj = pv.qg.abs();
            if !pv.qt.is_infinite() {
                mj += pv.qt.abs();
            }
            if !pv.qb.is_infinite() {
                mj += pv.qb.abs();
            }
            m += mj
        }
        let mut q_min = vec![0.0; l.len()];
        let mut q_max = vec![0.0; l.len()];
        for (i, j) in l.iter().enumerate() {
            let pv = &network.generators[*j];

            q_min[i] = pv.qb;
            q_max[i] = pv.qt;

            if pv.qb == f64::INFINITY {
                q_min[i] = m;
            }
            if pv.qb == f64::NEG_INFINITY {
                q_min[i] = -m;
            }
            if pv.qt == f64::INFINITY {
                q_max[i] = m;
            }
            if pv.qt == f64::NEG_INFINITY {
                q_max[i] = -m;
            }
        }
        let qg_min: f64 = q_min.iter().sum(); // Minimum total Qg at the bus.
        let qg_max: f64 = q_max.iter().sum(); // Maximum total Qg at the bus.

        if (qg_min - qg_max).abs() > 1e-13 {
            let q = (qg_tot - qg_min) / (qg_max - qg_min);
            for (i, &j) in l.iter().enumerate() {
                //pv.Qg = pv.QMin + (((QgTot - QgMin) / (QgMax - QgMin)) / (pv.QMax - pv.QMin))
                network.generators[j].qg = q_min[i] + (q * (q_max[i] - q_min[i]));
            }
        } else {
            // Zero Qg range at bus. Qg set such that all generators
            // at the bus violate their limits by the same amount.

            // Total mismatch at bus divided by number of online generators.
            let mis = (qg_tot - qg_min) / (l.len() as f64);
            for (i, &j) in l.iter().enumerate() {
                network.generators[j].qg = q_min[i] + mis;
            }
        }
    }

    Ok((brch_flows, tfmr_flows))
}

pub fn u0(
    ctx: &Context,
    network: &Network,
    a: &HashMap<BusNum, usize>,
    v: &HashMap<BusNum, usize>,
    q: &HashMap<ArrayString<3>, usize>,
    p: &HashMap<ArrayString<3>, usize>,
    ang0: &HashMap<BusNum, f64>,
    slack: BusNum,
) -> NVector {
    // nd.indexNetwork()

    let n = a.len() + v.len() + q.len() + p.len();
    let v0 = nvector::NVector::new_serial(n as i64, ctx).unwrap(); // [Va Vm Qg Pref]
    let data = v0.as_slice_mut();

    for bus in &network.buses {
        let a = a[&bus.i];
        let v = v[&bus.i];
        let ang0 = ang0[&bus.i];

        data[a] = ang0;
        if bus.vm != 0.0 {
            data[v] = bus.vm;
        } else {
            data[v] = 1.0;
        }
    }

    for gen in &network.generators {
        if gen.vs != 0.0 {
            let v = v[&gen.i];
            data[v] = gen.vs;
        }

        let qg = q[gen.id.as_str()];
        data[qg] = gen.qg;

        if gen.i == slack {
            let pg = p[gen.id.as_str()];
            data[pg] = gen.pg
        }
    }

    // println!("v0: {}", floats.String(data, prec))
    println!("v0: {:?}", data);

    v0
}
