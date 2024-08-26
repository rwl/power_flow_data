use crate::BusNum;

/// Simple Steam Turbine Governor
#[derive(Debug, Clone)]
pub struct TGOV1 {
    /// Buses number.
    pub i: BusNum,

    /// Model status is either one for in-service or zero for out-of-service.
    pub stat: i8,

    /// Permanent droop `R`.
    pub r: f64,

    /// Controller lag time constant `T1`.
    pub t1: f64,

    /// Maximum valve position.
    pub v_max: f64,

    /// Minimum valve position.
    pub v_min: f64,

    /// Turbine lead time constant `T2`.
    pub t2: f64,

    /// Turbine lag time constant `T3`.
    pub t3: f64,

    /// Turbine damping coefficient `Dt`.
    pub dt: f64,
}

impl Default for TGOV1 {
    fn default() -> Self {
        TGOV1 {
            i: Default::default(),
            stat: 1,
            r: 0.05,    // Typical range: 0.03 to 0.05 (3% to 5%)
            t1: 0.1,    // Typical range: 0.1 to 0.5 seconds
            v_max: 1.0, // Typical value: 1.0 pu
            v_min: 0.0, // Typical value: 0.0 pu
            t2: 0.5,    // Typical range: 0 to 3 seconds
            t3: 10.0,   // Typical range: 5 to 10 seconds
            dt: 0.0,    // Often set to 0, but can be up to 0.5
        }
    }
}

/// Gas Turbine Governor
#[derive(Debug, Clone)]
pub struct GAST {
    /// Buses number.
    pub i: BusNum,

    /// Model status is either one for in-service or zero for out-of-service.
    pub stat: i8,

    /// Permanent droop `R`.
    pub r: f64,

    /// Controller lag time constant `T1`.
    pub t1: f64,

    /// Fuel system lag time constant `T2`.
    pub t2: f64,

    /// Load limiter time constant `T3`.
    pub t3: f64,

    /// Ambient temperature load limit.
    pub at: f64,

    /// Temperature control loop gain `Kt`.
    pub kt: f64,

    /// Maximum valve position.
    pub vmax: f64,

    /// Minimum valve position.
    pub vmin: f64,

    /// Turbine damping factor.
    pub dturb: f64,
}

impl Default for GAST {
    fn default() -> Self {
        GAST {
            i: Default::default(),
            stat: 1,
            r: 0.05,     // Typical range: 0.03 to 0.05 (3% to 5%)
            t1: 0.4,     // Typical range: 0.1 to 0.5 seconds
            t2: 0.1,     // Typical range: 0.05 to 0.2 seconds
            t3: 3.0,     // Typical range: 2 to 5 seconds
            at: 1.0,     // Typical range: 0.8 to 1.2 pu
            kt: 2.0,     // Typical range: 1 to 3
            vmax: 1.0,   // Typical value: 1.0 pu
            vmin: 0.0,   // Typical value: 0.0 pu
            dturb: 0.03, // Typical range: 0 to 0.05
        }
    }
}

#[derive(Debug, Clone)]
pub struct HYGOV {
    /// Buses number.
    pub i: BusNum,

    /// Model status is either one for in-service or zero for out-of-service.
    pub stat: i8,

    /// Permanent droop `R`.
    pub r: f64,

    /// Temporary droop `r`.
    pub r_temp: f64,

    /// Governor time constant `Tr`.
    pub tr: f64,

    /// Filter time constant `Tf`.
    pub tf: f64,

    /// Gate servo time constant `Tg`.
    pub tg: f64,

    /// Gate velocity limit.
    pub velm: f64,

    /// Maximum gate position.
    pub gmax: f64,

    /// Minimum gate position.
    pub gmin: f64,

    /// Water time constant `Tw`.
    pub tw: f64,

    /// Turbine gain `At`.
    pub at: f64,

    /// Turbine damping factor `Dturb`.
    pub dturb: f64,

    /// No-load flow at nominal head.
    pub q_nl: f64,
}

impl Default for HYGOV {
    fn default() -> Self {
        HYGOV {
            i: Default::default(),
            stat: 1,
            r: 0.05,     // Typical range: 0.03 to 0.06 (3% to 6%)
            r_temp: 0.3, // Typical range: 0.2 to 0.5
            tr: 5.0,     // Typical range: 2.5 to 5.0 seconds
            tf: 0.05,    // Typical range: 0.02 to 0.1 seconds
            tg: 0.5,     // Typical range: 0.2 to 0.5 seconds
            velm: 0.2,   // Typical range: 0.1 to 0.3 pu/sec
            gmax: 1.0,   // Typical value: 1.0 pu
            gmin: 0.0,   // Typical value: 0.0 pu
            tw: 1.0,     // Typical range: 0.5 to 5.0 seconds
            at: 1.2,     // Typical range: 1.0 to 1.5
            dturb: 0.5,  // Typical range: 0.2 to 0.7
            q_nl: 0.08,  // Typical range: 0.05 to 0.1 pu
        }
    }
}
