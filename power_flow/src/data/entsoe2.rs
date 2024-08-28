use arrayvec::ArrayString;
use power_flow_data::{Bus, CaseID, Generator, Load, Network, Transformer, LOADBUS, SLACKBUS};

/// Returns a 2 terminal test network from "Controller Tests in Test
/// Grid Configurations" by ENTSO-E System Protection and Dynamics, Nov 2013.
#[allow(non_snake_case)]
pub fn entsoe2() -> Network {
    let grid_bus = Bus {
        i: 1,
        name: ArrayString::from("NGRID").unwrap(),
        ide: SLACKBUS,
        basekv: 380.0,
        vm: 1.05,
        ..Default::default()
    };
    let gen_bus = Bus {
        i: 2,
        name: ArrayString::from("NGEN").unwrap(),
        ide: LOADBUS,
        basekv: 21.0,
        vm: 0.9917,
        va: 9.2327,
        ..Default::default()
    };

    let grid_load = Load {
        i: grid_bus.i,
        pl: 475.0,
        ql: 76.0,
        ..Default::default()
    };

    // Negative load representing PQ gen at 0.95pu
    let gen = {
        let Sg = 500.0;
        let pf = 0.95;
        let theta = f64::acos(pf);
        // Load {
        //     i: gen_bus.i,
        //     pl: -Sg * pf,
        //     ql: -Sg * theta.sin(),
        //     ..Default::default()
        // };
        Generator {
            i: gen_bus.i,
            pg: -Sg * pf,
            qg: -Sg * theta.sin(),
            ..Default::default()
        }
    };

    let grid = Generator {
        i: grid_bus.i,
        vs: 1.05,
        qt: 100.0,
        qb: 100.0,
        ..Default::default()
    };

    let Sb = 100.0;

    // Ref bus rated voltage differs from tfmr HV-side.
    let Vb = 380.0;

    // Tfmr impedances are relative to HV-side (419kV).
    let Sn = 500.0;
    //let Vb_hv = 380.0;
    let Vn = 419.0;

    // Convert to system base.
    let c = f64::powi(Vn / Vb, 2) * (Sb / Sn);
    //c2 = (Vb_hv.powi(2) * Sb) / (Vb.powi(2) * Sn);
    //if c != c2 {
    //	panic!(c);
    //}
    let rpu = (0.15 / 100.0) * c;
    let zpu = (16.0 / 100.0) * c;
    let xpu = (zpu * zpu - rpu * rpu).sqrt();

    println!("R: {} X: {}", rpu, xpu);

    // Off-nominal rated voltage on the HV side of the transformer
    // requires adjustment to the tap ratio of the branch element.
    let tap = Vn / Vb;

    // To achieve the published results the percentage impedance must
    // be assumed to be relative to the nominal voltage (380^2/500).
    // Our transformer model assumes that the impedance is relative to
    // the rated voltage (419^2/500).
    //                   Vold^2   Snew
    //     Znew = Zold * ------ * ----
    //                   Vnew^2   Sold
    //let (Zsc, Rsc) = (16.0, 0.15);
    //Zsc *= (380.0/419.0).powi(2);
    //Rsc *= (380.0/419.0).powi(2);

    let gen_tfmr = Transformer {
        i: gen_bus.i,
        j: grid_bus.i,
        r1_2: rpu,
        x1_2: xpu,
        windv1: tap,
        ..Default::default()
    };

    return Network {
        caseid: CaseID {
            sbase: Sb,
            ..Default::default()
        },
        buses: vec![grid_bus, gen_bus],
        loads: vec![grid_load /*, gen*/],
        generators: vec![grid, gen],
        transformers: vec![gen_tfmr],
        ..Default::default()
    };
}
