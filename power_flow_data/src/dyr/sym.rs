use crate::BusNum;

/// Parameters for the GENCLS model (Classical synchronous machine) in PSS/E.
#[derive(Debug, Clone, Copy)]
pub struct GENCLS {
    /// Buses number.
    pub i: BusNum,

    /// Model status is either one for in-service or zero for out-of-service.
    pub stat: i8,

    /// Inertia constant `H`.
    pub h: f64,

    /// Damping factor `D`.
    pub d: f64,
}

impl Default for GENCLS {
    fn default() -> Self {
        GENCLS {
            i: Default::default(),
            stat: 1,
            h: 3.5,
            d: 0.0,
        }
    }
}

/// Round rotor generator model parameters.
#[derive(Debug, Clone)]
pub struct GENROU {
    /// Buses number.
    pub i: BusNum,

    /// Model status is either one for in-service or zero for out-of-service.
    pub stat: i8,

    /// d-axis transient open-circuit time constant `T'do`.
    pub tdo_p: f64,
    /// d-axis subtransient open-circuit time constant `T''do`.
    pub tdo_pp: f64,

    /// q-axis transient open-circuit time constant `T'qo`.
    pub tqo_p: f64,
    /// q-axis subtransient open-circuit time constant `T''qo`.
    pub tqo_pp: f64,

    /// Inertia constant `H`.
    pub h: f64,

    /// Speed damping `D`.
    pub d: f64,

    /// d-axis synchronous reactance `Xd`.
    pub xd: f64,
    /// q-axis synchronous reactance `Xq`.
    pub xq: f64,

    /// d-axis transient reactance `X'd`.
    pub xd_p: f64,
    /// q-axis transient reactance `X'q`.
    pub xq_p: f64,
    /// d-axis subtransient reactance `X''d`.
    pub xd_pp: f64,

    /// Leakage reactance `Xl`.
    pub xl: f64,

    /// Saturation factor at 1.0 pu flux.
    pub s1_0: f64,
    /// Saturation factor at 1.2 pu flux.
    pub s1_2: f64,
}

impl Default for GENROU {
    fn default() -> Self {
        GENROU {
            i: Default::default(),
            stat: 1,
            tdo_p: 7.0,   // Typical range: 3.0 to 10.0 seconds
            tdo_pp: 0.03, // Typical range: 0.02 to 0.05 seconds
            tqo_p: 0.7,   // Typical range: 0.5 to 2.0 seconds
            tqo_pp: 0.05, // Typical range: 0.02 to 0.1 seconds
            h: 4.0,       // Typical range: 2.0 to 10.0 seconds
            d: 0.0,       // Often set to 0 for stability studies
            xd: 1.8,      // Typical range: 1.5 to 2.5 pu
            xq: 1.7,      // Typically slightly less than x_d
            xd_p: 0.3,    // Typical range: 0.2 to 0.5 pu
            xq_p: 0.55,   // Typical range: 0.3 to 0.8 pu
            xd_pp: 0.25,  // Typical range: 0.2 to 0.35 pu
            xl: 0.15,     // Typical range: 0.1 to 0.2 pu
            s1_0: 0.1,    // Depends on the specific machine
            s1_2: 0.3,    // Depends on the specific machine
        }
    }
}
