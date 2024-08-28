use num_complex::Complex64;
use power_flow_data::{BusNum, Network};
use sparsetools::csr::CSR;
use std::collections::HashMap;

#[allow(non_snake_case)]
pub(crate) fn build_y(
    network: &Network,
    a: &HashMap<BusNum, usize>,
    // v_base: &HashMap<BusNum, f64>,
) -> CSR<usize, Complex64> {
    let nb = a.len();
    // let s_base = network.caseid.sbase;
    let mut Y = sparsetools::dok::DoK::new(nb, nb);

    for ln in network.branches.iter().filter(|br| br.st != 0) {
        // let Vn = v_base[ln.i]; // VoltageLevelId1
        // let Zb = Vn.powi(2) / s_base;
        // let Yb = 1.0 / Zb;

        let a1 = a[&ln.i];
        let a2 = a[&ln.j];

        // let y1 = Complex64::new(ln.gi / Yb, ln.bi / Yb);
        // let y2 = Complex64::new(ln.gj / Yb, ln.bj / Yb);
        // let y12 = Complex64::new(1.0, 0.0) / Complex64::new(ln.r / Zb, ln.x / Zb);
        let yc = Complex64::new(0.0, ln.b / 2.0);
        let y1 = Complex64::new(ln.gi, ln.bi);
        let y2 = Complex64::new(ln.gj, ln.bj);
        let y12 = Complex64::new(1.0, 0.0) / Complex64::new(ln.r, ln.x);

        Y.add(a1, a1, y12 + y1 + yc).unwrap();
        Y.sub(a1, a2, y12).unwrap();
        Y.sub(a2, a1, y12).unwrap();
        Y.add(a2, a2, y12 + y2 + yc).unwrap();
    }

    for tr in network.transformers.iter().filter(|tr| tr.stat != 0) {
        // let Vn = v_base[tr.i];
        // let Zb = Vn.powi(2) / s_base;
        // let Yb = 1.0 / Zb;

        let (g, b) = match tr.cm {
            1 => (tr.mag1, tr.mag2),
            _ => {
                panic!("cm must be 1: {}", tr.cm);
            }
        };

        let (r, x) = match tr.cz {
            1 => {
                // pu on system base
                (tr.r1_2, tr.x1_2)
            }
            _ => {
                panic!("cz must be 1: {}", tr.cz)
            }
        };

        let a1 = a[&tr.i];
        let a2 = a[&tr.j];

        let y0 = 0.5 * Complex64::new(g, b);
        let y12 = Complex64::new(1.0, 0.0) / Complex64::new(r, x);

        let tap = match tr.cw {
            1 => tr.windv1,
            _ => {
                panic!("cw must be 1: {}", tr.cw);
            }
        };
        let phi = tr.ang1;

        let m = Complex64::from_polar(tap, phi.to_radians());
        let m2 = Complex64::new(m.norm().powi(2), 0.0);

        Y.add(a1, a1, (y12 + y0) / m2).unwrap();
        Y.sub(a1, a2, y12 / m.conj()).unwrap();
        Y.sub(a2, a1, y12 / m).unwrap();
        Y.add(a2, a2, y12 + y0).unwrap();
    }

    let Y = Y.to_csr();

    print!("Y:\n{}", Y.to_table());

    Y
}
