use crate::BusNum;

/// Represents the parameters for the IEEEST (IEEE Stabilizing Model) in PSS/E rev 33
#[derive(Debug, Clone)]
pub struct IEEEST {
    /// Buses number.
    pub i: BusNum,

    /// Model status is either one for in-service or zero for out-of-service.
    pub stat: i8,

    /// Input mode.
    ///  * 1 - Rotor speed deviation
    ///  * 2 - Bus frequency deviation
    ///  * 3 - Generator electrical power
    ///  * 4 - Generator accelerating power
    ///  * 5 - Bus voltage
    ///  * 5 - Derivative of bus voltage
    pub mode: i32,

    /// Reference bus number.
    pub busr: BusNum,

    /// Signal conditioning frequency filter constant `A1`.
    pub a1: f64,
    /// Signal conditioning frequency filter constant `A2`.
    pub a2: f64,
    /// Signal conditioning frequency filter constant `A3`.
    pub a3: f64,
    /// Signal conditioning frequency filter constant `A4`.
    pub a4: f64,
    /// Signal conditioning frequency filter constant `A5`.
    pub a5: f64,
    /// Signal conditioning frequency filter constant `A6`.
    pub a6: f64,

    /// Lead/lag time constant (seconds)
    pub t1: f64,
    /// Lead/lag time constant (seconds)
    pub t2: f64,
    /// Lead/lag time constant (seconds)
    pub t3: f64,
    /// Lead/lag time constant (seconds)
    pub t4: f64,
    /// Washout numerator time constant (seconds)
    pub t5: f64,
    /// Washout denominator time constant (seconds)
    pub t6: f64,

    /// Stabilizer gain `Ks`.
    pub k: f64,

    /// Maximum stabilizer output
    pub vmax: f64,
    /// Minimum stabilizer output
    pub vmin: f64,
}

impl Default for IEEEST {
    fn default() -> Self {
        IEEEST {
            i: 0,    // Default bus number, should be set by user
            stat: 1, // 1 for in-service
            mode: 1, // 1 for rotor speed deviation
            busr: 0, // Default reference bus, should be set by user
            a1: 0.0,
            a2: 0.0,
            a3: 0.0,
            a4: 0.0,
            a5: 0.0,
            a6: 0.0,
            t1: 0.3,
            t2: 0.05,
            t3: 0.3,
            t4: 0.05,
            t5: 3.0,    // Typical value for washout numerator
            t6: 0.15,   // Typical value for washout denominator
            k: 10.0,    // Typical stabilizer gain
            vmax: 0.1,  // Typical maximum output
            vmin: -0.1, // Typical minimum output
        }
    }
}

/// Parameters for the ST2CUT model (Dual-input stabilizing model) in PSS/E.
#[derive(Debug, Clone, Copy)]
pub struct ST2CUT {
    /// Buses number.
    pub i: BusNum,

    /// Model status is either one for in-service or zero for out-of-service.
    pub stat: i8,

    /// Input mode.
    ///  * 1 - Rotor speed deviation
    ///  * 2 - Bus frequency deviation
    ///  * 3 - Generator electrical power
    ///  * 4 - Generator accelerating power
    ///  * 5 - Bus voltage
    ///  * 5 - Derivative of bus voltage
    pub mode1: i32,

    /// Reference bus number.
    pub busr1: BusNum,

    /// Input mode.
    ///  * 1 - Rotor speed deviation
    ///  * 2 - Bus frequency deviation
    ///  * 3 - Generator electrical power
    ///  * 4 - Generator accelerating power
    ///  * 5 - Bus voltage
    ///  * 5 - Derivative of bus voltage
    pub mode2: i32,

    /// Reference bus number.
    pub busr2: BusNum,

    /// Input 1 gain `K1`.
    pub k1: f64,
    /// Input 2 gain `K2`.
    pub k2: f64,

    /// Input 1 time constant `T1`.
    pub t1: f64,
    /// Input 2 time constant `T2`.
    pub t2: f64,

    /// Washout numerator time constant `T3`.
    pub t3: f64,
    /// Lead/lag denominator time constant `T4`.
    pub t4: f64,

    /// Lead/lag numerator time constant `T5`.
    pub t5: f64,
    /// Lead/lag denominator time constant `T6`.
    pub t6: f64,

    /// Lead/lag numerator time constant `T7`.
    pub t7: f64,
    /// Lead/lag denominator time constant `T8`.
    pub t8: f64,

    /// Lead/lag numerator time constant `T9`.
    pub t9: f64,
    /// Lead/lag denominator time constant `T10`.
    pub t10: f64,

    /// Stabilizer output upper limit.
    pub lsmax: f64,
    /// Stabilizer output lower limit.
    pub lsmin: f64,

    /// Output cutoff upper limit.
    pub vcu: f64,
    /// Output cutoff lower limit.
    pub vcl: f64,
}

impl Default for ST2CUT {
    fn default() -> Self {
        ST2CUT {
            i: Default::default(),
            stat: 1,  // 1 for in-service
            mode1: 1, // 1 for rotor speed deviation (input 1)
            busr1: 0,
            mode2: 3, // 3 for generator electrical power (input 2)
            busr2: 0,
            k1: 1.0,
            k2: 1.0,
            t1: 0.1,
            t2: 0.1,
            t3: 3.0,
            t4: 0.5,
            t5: 0.1,
            t6: 0.1,
            t7: 0.1,
            t8: 0.1,
            t9: 0.1,
            t10: 0.1,
            lsmax: 0.1,
            lsmin: -0.1,
            vcu: 999.0,  // High value for essentially no upper cutoff
            vcl: -999.0, // Low value for essentially no lower cutoff
        }
    }
}
