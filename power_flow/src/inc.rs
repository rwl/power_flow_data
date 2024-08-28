use super::user_data::UserData;
use num_complex::Complex64;

pub(super) fn calc_inc(uu: &[f64], fval: &mut [f64], user_data: &Option<UserData>) -> i32 {
    let user_data = user_data.as_ref().unwrap();
    // let network = &user_data.network;
    // let s_base = user_data.network.caseid.sbase;
    let s_base = user_data.s_base;
    let n = user_data.v.len();
    let slack = user_data.slack;
    let y_mat = &user_data.y_mat;

    println!("uu: {:?}", uu);

    fval.fill(0.0);

    // for motor in ac_system.motors {
    for load in &user_data.loads {
        let &a = user_data.a.get(&load.i).unwrap();
        let &v = user_data.v.get(&load.i).unwrap();

        //let vc = uu[v];
        //if vc < load.v_min {}
        //if vc > load.v_max {}

        // let (pl, ql) = rect_power_factor(load.sr, load.cosphi);
        let (pl, ql) = (load.pl, load.ql);

        fval[a] += pl / s_base;
        fval[v] += ql / s_base;
    }
    for fixed in &user_data.fixed_shunts {
        let &a = user_data.a.get(&fixed.i).unwrap();
        let &v = user_data.v.get(&fixed.i).unwrap();

        let vc2 = uu[v] * uu[v];
        let gs = fixed.gl / s_base;
        let bs = fixed.bl / s_base;

        fval[a] += vc2 * gs;
        fval[v] -= vc2 * bs;
    }
    for (i, generator) in user_data.generators.iter().enumerate() {
        let &a = user_data.a.get(&generator.i).unwrap();
        let &v = user_data.v.get(&generator.i).unwrap();

        let q = user_data.q[generator.id.as_str()];
        let p = user_data.p[generator.id.as_str()];
        let ang0 = user_data.ang0[&(i as i32)];

        if generator.i != slack {
            // fval[a] -= generator.sr / s_base;
            fval[a] -= generator.pg / s_base;
            fval[v] -= uu[q];
            // fval[q] += uu[v] - (generator.ug / generator.ur);
            fval[q] += uu[v] - generator.vs;
        } else {
            fval[a] -= uu[p];
            fval[v] -= uu[q];
            fval[q] = uu[v] - generator.vs;
            fval[p] = uu[a] - ang0;
        }
    }
    // for (i, feeder) in network.feeders.iter().enumerate() {
    //     let a = user_data.a.get(&feeder.node).unwrap();
    //     let v = user_data.v.get(&feeder.node).unwrap();
    //
    //     let q = user_data.q[&i];
    //     let p = user_data.p[&i];
    //     let ang0 = user_data.ang0[&feeder.node];
    //
    //     fval[a] -= uu[p];
    //     fval[v] -= uu[q];
    //     fval[q] = uu[v] - (feeder.ur / vl.NominalV);
    //     fval[p] = uu[a] - ang0;
    // }

    let vc: Vec<Complex64> = (0..n)
        .map(|i| Complex64::from_polar(uu[n + i], uu[i]))
        .collect();
    let ic = y_mat * &vc;

    for i in 0..n {
        let s: Complex64 = vc[i] * ic[i].conj();

        fval[i] = s.re;
        fval[n + i] = s.im;
    }

    println!("f: {:?}", fval);
    0
}
