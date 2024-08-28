use super::inc::calc_inc;
use super::jac::jac;
use super::user_data::{post_process, u0, UserData};

use anyhow::{format_err, Result};
use power_flow_data::Network;
use sundials::context::Context;
use sundials::kinsol::{Strategy, KIN};
use sundials::nvector::NVector;
use sundials::sunlinsol::LinearSolver;
use sundials::sunmatrix::{SparseMatrix, SparseType};

pub fn power_flow(network: &mut Network, tolerance: f64) -> Result<()> {
    validate_network(network)?;

    let user_data = UserData::new(network.clone());

    let a = user_data.a.clone();
    let v = user_data.v.clone();
    let q = user_data.q.clone();
    let p = user_data.p.clone();
    let slack = user_data.slack;

    let context = Context::new()?;

    let mut kin = KIN::<UserData>::new(&context)?;
    // kin.set_print_level(1)?;
    kin.set_func_norm_tol(tolerance)?;

    let mut u0: NVector = u0(&context, &network, &a, &v, &q, &p, &user_data.ang0, slack);

    kin.init(Some(calc_inc), Some(jac), Some(user_data), &u0)?;

    let mut scale = u0.clone();
    scale.fill_with(1); // No scaling used.

    let n = u0.len() as i32;
    let nz = 0;

    let j_mat = SparseMatrix::new(n, n, nz, SparseType::CSC, &context);

    let ls = LinearSolver::new_faer(&u0, &j_mat, &context);
    kin.set_linear_solver(&ls, &j_mat)?;

    kin.solve(&mut u0, Strategy::None, &scale, &scale)?;

    post_process(network, &u0, &a, &v, &q, &p, slack)?;

    Ok(())
}

fn validate_network(network: &Network) -> Result<()> {
    if network.caseid.ic != 0 {
        return Err(format_err!(
            "case ic must be 0 (base case): {}",
            network.caseid.ic
        ));
    }
    if network.caseid.sbase == 0.0 {
        return Err(format_err!("sbase must not be 0.0"));
    }

    for tr in &network.transformers {
        if tr.k != 0 {
            return Err(format_err!(
                "transformer must have two windings: {} {} {}",
                tr.i,
                tr.j,
                tr.k
            ));
        }
    }

    if !network.switched_shunts.is_empty() {
        return Err(format_err!(
            "switched shunts must be empty (not implemented)"
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::data::{entsoe2, ieee14};
    use crate::power_flow;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_entsoe2() {
        const PG_GRID: f64 = 0.763;
        const QG_GRID: f64 = 1.209;

        let mut net = entsoe2();

        power_flow(&mut net, 1e-6).unwrap();

        let r#ref = &net.generators[0];

        assert_approx_eq!(f64, r#ref.pg, PG_GRID, epsilon = 1e-3);
        assert_approx_eq!(f64, r#ref.qg, QG_GRID, epsilon = 1e-3);
    }

    #[test]
    fn test_ieee14() {
        const PG_GRID: f64 = 2.3239e2;
        const QG_GRID: f64 = -0.1689e2;

        let mut net = ieee14();

        power_flow(&mut net, 1e-6).unwrap();

        let r#ref = &net.generators[0];

        assert_approx_eq!(f64, r#ref.pg, PG_GRID, epsilon = 1e-3);
        assert_approx_eq!(f64, r#ref.qg, QG_GRID, epsilon = 1e-3);
    }
}
