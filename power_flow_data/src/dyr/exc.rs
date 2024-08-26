use crate::BusNum;

/// Simplified Excitation System
#[derive(Debug, Clone)]
pub struct SEXS {
    /// Buses number.
    pub i: BusNum,

    /// Model status is either one for in-service or zero for out-of-service.
    pub stat: i8,

    /// Time constant ratio `Ta/Tb`.
    pub ta_tb: f64,

    /// Denominator time constant `Tb`
    pub tb: f64,

    /// Gain `K`.
    pub k: f64,

    /// Exciter time constant `Te`.
    pub te: f64,

    /// Minimum exciter output `Emin`.
    pub emin: f64,

    /// Maximum exciter output `Emax`.
    pub emax: f64,
}

impl Default for SEXS {
    fn default() -> Self {
        SEXS {
            i: Default::default(),
            stat: 1,
            ta_tb: 0.1, // Typical range: 0 to 1
            tb: 10.0,   // Typical range: 1 to 20 seconds
            k: 100.0,   // Typical range: 10 to 200
            te: 0.05,   // Typical range: 0.01 to 1 second
            emin: -5.0, // Typical range: -5 to 0
            emax: 5.0,  // Typical range: 3 to 7
        }
    }
}
