use crate::user_data::UserData;
use num_complex::Complex64;
use sparsetools::coo::{CCoo, Coo};
use sparsetools::csc::{CCSC, CSC};
use std::iter::zip;
use sundials::nvector::NVector;
use sundials::sunmatrix::SparseMatrix;

#[allow(non_snake_case)]
pub(super) fn jac(
    u: &NVector,
    _fu: &mut NVector,
    J: &mut SparseMatrix,
    user_data: &Option<UserData>,
    _tmp1: &NVector,
    _tmp2: &NVector,
) -> i32 {
    // var (
    let user_data = user_data.as_ref().unwrap();
    // let net = &d.network;
    let nb = user_data.a.len();
    // let Sb = d.network.caseid.sbase;
    let Sb = user_data.s_base;

    let yd = u.as_slice();
    //fd = fu.Array()
    // )
    let n = yd.len();
    let mut A = sparsetools::dok::DoK::new(n, n);

    // for _, sub := range net.Substations {
    // 	for _, vl := range sub.VoltageLevels {

    for ld in &user_data.loads {
        let a = user_data.a[&ld.i];
        let v = user_data.v[&ld.i];
        let k = 0.0;

        //if vc < ld.Vmin {}
        //if vc > ld.Vmax {}

        A.add(a, v, (ld.pl / Sb) * k).unwrap();
        A.add(v, v, (ld.ql / Sb) * k).unwrap();
    }

    for sh in &user_data.fixed_shunts {
        let a = user_data.a[&sh.i];
        let v = user_data.v[&sh.i];

        let gs = sh.gl / Sb;
        let bs = sh.bl / Sb;

        // var dV2 float64
        // if sh.Bus != "" {
        let dV2 = 2.0 * yd[v];
        // }
        let dPdV = dV2 * gs;
        let dQdV = -dV2 * bs;

        A.add(a, v, dPdV).unwrap();
        A.add(v, v, dQdV).unwrap();
    }

    for (i, g) in user_data.generators.iter().enumerate() {
        // var u float64
        // if g.Bus != "" {
        let u = 1.0;
        // }
        let a = user_data.a[&g.i];
        let v = user_data.v[&g.i];
        let q = user_data.q[&i];

        A.set(v, v, -1e-6).unwrap();
        A.set(v, q, -u).unwrap();
        A.set(q, v, u).unwrap();
        A.set(q, q, u - 1.0 - 1e-6).unwrap();

        // if g.ParticipationFactor == 1 {
        if g.i == user_data.slack {
            let p = user_data.p[&i];
            A.set(a, p, -u).unwrap();
            A.set(p, a, u).unwrap();
            A.set(p, p, u - 1.0 - 1e-6).unwrap();
        }
    }

    // }
    // }

    let L = line_jac(nb, yd, &user_data.y_mat.to_csc());
    // zip(L.rowidx(), L.colidx())
    //     .zip(L.values())
    //     .for_each(|((r, c), v)| {
    //         A.add(*r, *c, *v).unwrap();
    //     });
    for (r, c, v) in L {
        A.add(r, c, v).unwrap();
    }

    // if Debug {
    println!("J:\n{}", A.to_csr().to_table());
    // }

    let nnz = J.nnz();
    if nnz < A.nnz() {
        J.reallocate(A.nnz()).unwrap();
        // if retval != 0 {
        // 	return -1
        // }
    }

    let (colptrs, rowvals, nz) = J.index_pointers_values_data_mut();

    // let Acoo = A.to_coo();
    // sparsetools::coo_tocsr(
    //     n,
    //     n,
    //     A.nnz(),
    //     Acoo.colidx(),
    //     Acoo.rowidx(),
    //     Acoo.values(),
    //     colptrs,
    //     rowvals,
    //     nz,
    // );

    let Ac = A.to_csc();

    // colptrs.copy_from_slice(Ac.colptr());
    zip(colptrs, Ac.colptr()).for_each(|colptr| {
        *colptr.0 = *colptr.1 as i64;
    });

    // rowvals.copy_from_slice(Ac.rowidx());
    zip(rowvals, Ac.rowidx()).for_each(|rowval| {
        *rowval.0 = *rowval.1 as i64;
    });

    nz.copy_from_slice(Ac.values());

    0
}

#[allow(non_snake_case)]
fn line_jac(nb: usize, yd: &[f64], Y: &CSC<usize, Complex64>) -> Coo<usize, f64> {
    // Vn := make([]complex128, nb)
    // Vc := make([]complex128, nb)
    // for i := 0; i < nb; i++ {
    // 	Vn[i] = cmplx.Rect(1, yd[i])
    // 	Vc[i] = complex(yd[nb+i], 0) * Vn[i]
    // }
    let Vn: Vec<Complex64> = yd[..nb]
        .iter()
        .map(|yd| Complex64::from_polar(1.0, *yd))
        .collect();
    let Vc: Vec<Complex64> = yd[nb..2 * nb]
        .iter()
        .map(|yd| Complex64::new(*yd, 0.0) * Complex64::from_polar(1.0, *yd))
        .collect();

    let Ic = Y * &Vc;
    // if err != nil {
    // 	return nil, err
    // }

    let diagVn = CSC::with_diagonal(Vn);
    let diagVc = CSC::with_diagonal(Vc);
    let diagIc = CSC::with_diagonal(Ic);

    let dS: Coo<usize, Complex64> = {
        // let dScsc = Y.mat_mat(&diagVn)?;
        let dScsc = Y * &diagVn;
        // if err != nil {
        // 	return nil, err
        // }
        // let dScsc = diagVc.mat_mat(&dScsc.conj())?;
        let dScsc = &diagVc * &dScsc.conj();
        // if err != nil {
        // 	return nil, err
        // }
        let temp = diagIc.conj() * &diagVn;
        // if err != nil {
        // 	return nil, err
        // }
        let dScsc = dScsc + temp;
        // if err != nil {
        // 	return nil, err
        // }
        dScsc.to_coo()
    };

    let dR: Coo<usize, Complex64> = {
        let dRcsc = diagIc;
        let mut temp = Y * &diagVc;
        // if err != nil {
        // 	return nil, err
        // }
        // neg(temp.Data())
        temp.values_mut().iter_mut().for_each(|v| *v = -*v);
        let dRcsc = dRcsc + temp;
        // if err != nil {
        // 	return nil, err
        // }
        let dRcsc = diagVc.conj() * &dRcsc;
        // if err != nil {
        // 	return nil, err
        // }
        dRcsc.to_coo()
    };

    let J = Coo::compose([[&dR.imag(), &dS.real()], [&dR.real(), &dS.imag()]]).unwrap();
    // if err != nil {
    // 	return nil, err
    // }

    J
}
