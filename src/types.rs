use arrayvec::ArrayString;

// TODO: should the various bits of free text / comments / timestamps be in this struct?
// Data can look like:
// 0,   100.00          / PSS/E-30.3    WED, SEP 15 2021  21:04
//    SE SNAPSHOT 09-15-2021 PEAK CASE 18:00
//    FULL COPY OF ETC.

/// Case identification data.
pub struct CaseID {
    /// IC Change code:
    /// 0 - base case (i.e., clear the working case before adding data to it).
    /// 1 - add data to the working case.
    pub ic: i8,
    /// System base MVA.
    pub sbase: f64,
    /// PSSE revision number (if known).
    pub rev: Option<usize>,
    /// Units of transformer ratings (see [Transformer]).
    /// `xfrrat` ≤ 0 for MVA. `xfrrat` > 0 for current expressed as MVA.
    pub xfrrat: Option<i8>,
    /// Units of ratings of non-transformer branches (refer to Non-Transformer Branch Data).
    /// `nxfrat` ≤ 0 for MVA. `nxfrat` > 0 for current expressed as MVA.
    pub nxfrat: Option<i8>,
    /// System base frequency in Hertz.
    pub basfrq: Option<f64>,
}

// impl CaseID {
//     fn new(ic=0, sbase=100.0, rev=missing, xfrrat=missing, nxfrat=missing, basfrq=missing) {
//         Self{ic, sbase, rev, xfrrat, nxfrat, basfrq}
//     }
// }

pub type BusNum = i32;
pub type AreaNum = i16;
pub type ZoneNum = i16;
pub type OwnerNum = i16;
pub type LineNum = i16;

pub enum Records {
    Bus(Bus),
    Load(Load),
    FixedShunt(FixedShunt),
    Generator(Generator),
    Branch(Branch),
    Transformer(Transformer),
    AreaInterchange(AreaInterchange),
    TwoTerminalDCLine(TwoTerminalDCLine),
    VSCDCLine(VSCDCLine),
    SwitchedShunt(SwitchedShunt),
    ImpedanceCorrection(ImpedanceCorrection),
    MultiTerminalDCLine(MultiTerminalDCLine),
    MultiSectionLineGroup(MultiSectionLineGroup),
    Zone(Zone),
    InterAreaTransfer(InterAreaTransfer),
    Owner(Owner),
    FACTSDevice(FACTSDevice),
}

/// Each network bus to be represented in PSSE is introduced by a bus data record.
/// The bus data record depends on the PSSE version:
/// - See [Bus30] for PSSE v30 files.
/// - See [Bus33] for PSSE v33 files.
pub enum Bus {
    Bus30(Bus30),
    Bus33(Bus33),
}

/// Network bus data record (in PSSE v30 format).
///
/// Each bus data record includes not only data for the basic bus properties but also includes
/// information on an optionally connected shunt admittance to ground. That admittance can
/// represent a shunt capacitor or a shunt reactor (both with or without a real component) or a
/// shunt resistor. It must not represent line connected admittance, loads, line charging or
/// transformer magnetizing impedance, all of which are entered in other data categories.
pub struct Bus30 {
    /// Bus number (1 to 999997).
    pub i: BusNum,

    /// Alphanumeric identifier assigned to bus "I".
    /// The name may be up to twelve characters and must be enclosed in single quotes.
    /// NAME may contain any combination of blanks, uppercase letters, numbers and special characters, but the first character must not be a minus sign.
    pub name: ArrayString<15>,

    /// Bus base voltage; entered in kV.
    pub basekv: f64,

    /// Bus type code:
    /// 1 - load bus or other bus without any generator boundary condition.
    /// 2 - generator or plant bus either regulating voltage or with a fixed reactive power (Mvar).
    /// A generator that reaches its reactive power limit will no longer control voltage but rather hold reactive power at its limit.
    /// 3 - swing bus or slack bus.
    /// It has no power or reactive limits and regulates voltage at a fixed reference angle.
    /// 4 - disconnected or isolated bus.
    pub ide: i8, // 1, 2, 3 or 4

    /// Active component of shunt admittance to ground; entered in MW at one per unit voltage.
    /// GL should not include any resistive admittance load, which is entered as part of load data.
    pub gl: f64,

    /// Reactive component of shunt admittance to ground; entered in Mvar at one per unit voltage.
    /// BL should not include any reactive impedance load, which is entered as part of load data;
    /// line charging and line connected shunts, which are entered as part of non-transformer branch data;
    /// or transformer magnetizing admittance, which is entered as part of transformer data.
    /// BL is positive for a capacitor, and negative for a reactor or an inductive load.
    pub bl: f64,

    /// Area number. 1 through the maximum number of areas at the current size level.
    pub area: AreaNum,

    /// Zone number. 1 through the maximum number of zones at the current size level.
    /// See [Zone].
    pub zone: ZoneNum,

    /// Bus voltage magnitude; entered in pu.
    pub vm: f64,

    /// Bus voltage phase angle; entered in degrees.
    pub va: f64,

    /// Owner number.
    /// 1 through the maximum number of owners at the current size level.
    /// See [Owner].
    pub owner: OwnerNum,
}

/// Network bus data record (in PSSE v33 format).
pub struct Bus33 {
    /// Bus number (1 to 999997).
    pub i: BusNum,

    /// Alphanumeric identifier assigned to bus "I".
    /// The name may be up to twelve characters and must be enclosed in single quotes.
    /// NAME may contain any combination of blanks, uppercase letters, numbers and special characters, but the first character must not be a minus sign.
    pub name: ArrayString<15>,

    /// Bus base voltage; entered in kV.
    pub basekv: f64,

    /// Bus type code:
    /// 1 - load bus or other bus without any generator boundary condition.
    /// 2 - generator or plant bus either regulating voltage or with a fixed reactive power (Mvar).
    /// A generator that reaches its reactive power limit will no longer control voltage but rather hold reactive power at its limit.
    /// 3 - swing bus or slack bus.
    /// It has no power or reactive limits and regulates voltage at a fixed reference angle.
    /// 4 - disconnected or isolated bus.
    pub ide: i8, // 1, 2, 3 or 4

    /// Area number. 1 through the maximum number of areas at the current size level.
    pub area: AreaNum,

    /// Zone number. 1 through the maximum number of zones at the current size level.
    /// See [Zone].
    pub zone: ZoneNum,

    /// Owner number.
    /// 1 through the maximum number of owners at the current size level.
    /// See [Owner].
    pub owner: OwnerNum,

    /// Bus voltage magnitude; entered in pu.
    pub vm: f64,

    /// Bus voltage phase angle; entered in degrees.
    pub va: f64,

    /// Normal voltage magnitude high limit; entered in pu. `nvhi` = 1.1 by default.
    pub nvhi: f64,

    /// Normal voltage magnitude low limit, entered in pu. `nvlo` = 0.9 by default.
    pub nvlo: f64,

    /// Emergency voltage magnitude high limit; entered in pu. `evhi` = 1.1 by default.
    pub evhi: f64,

    /// Emergency voltage magnitude low limit; entered in pu. `evlo` = 0.9 by default.
    pub evlo: f64,
}

/// Each network bus at which a load is to be represented must be specified in at least one load
/// data record. If multiple loads are to be represented at a bus, they must be individually
/// identified in a load data record for the bus with a different load identifier.
/// Each load at a bus can be a mixture of loads with different characteristics.
pub struct Load {
    /// Buses number, or extended buses name enclosed in single quotes.
    pub i: BusNum,

    /// One- or two-character uppercase non blank alphanumeric load identifier used to distinguish among multiple loads at bus "I".
    /// It is recommended that, at buses for which a single load is present, the load be designated as having the load identifier '1'.
    pub id: ArrayString<3>, // TODO: confirm 3 is enough in practice, when whitespace can be present

    /// Initial load status of one for in-service and zero for out-of-service.
    pub status: bool,

    /// Area to which the load is assigned (1 through the maximum number of areas at the current size level).
    pub area: AreaNum,

    /// Zone to which the load is assigned (1 through the maximum number of zones at the current size level).
    /// See [Zone].
    pub zone: ZoneNum,

    /// Active power component of constant MVA load; entered in MW.
    pub pl: f64,

    /// Reactive power component of constant MVA load; entered in Mvar.
    pub ql: f64,

    /// Active power component of constant current load; entered in MW at one per unit voltage.
    pub ip: f64,

    /// Reactive power component of constant current load; entered in Mvar at one per unit voltage.
    pub iq: f64,

    /// Active power component of constant admittance load; entered in MW at one per unit voltage.
    pub yp: f64,

    /// Reactive power component of constant admittance load; entered in Mvar at one per unit voltage.
    /// YQ is a negative quantity for an inductive load and positive for a capacitive load.
    pub yq: f64,

    /// Owner to which the load is assigned.
    /// 1 through the maximum number of owners at the current size level.
    /// See [Owner].
    pub owner: OwnerNum,

    /// Load scaling flag of one for a scalable load and zero for a fixed load.
    /// `scale` = 1 by default.
    pub scale: Option<bool>,

    /// Interruptible load flag of one for an interruptible load for zero for a non interruptible load.
    /// `intrpt`=0 by default.
    pub intrpt: Option<bool>,
}

/// Each network bus at which fixed bus shunt is to be represented must be specified in at least
/// one fixed bus shunt data record. Multiple fixed bus shunts may be represented at a bus by
/// specifying more than one fixed bus shunt data record for the bus, each with a different shunt
/// identifier.
///
/// The admittance specified in the data record can represent a shunt capacitor or a shunt reactor
/// (both with or without a real component) or a shunt resistor. It must not represent line
/// connected admittance, switched shunts, loads, line charging or transformer magnetizing impedance,
/// all of which are entered in other data categories.
///
/// !!! compat "Not present in v30 files"
///     v30 files do not have `FixedShunts`; refer to [Bus] and [SwitchedShunt].
pub struct FixedShunt {
    /// Bus number, or extended bus name enclosed in single quotes. No default.
    pub i: BusNum,

    /// One- or two-character uppercase non-blank alphanumeric shunt identifier used to
    /// distinguish among multiple shunts at bus `i`. It is recommended that, at buses for which
    /// a single shunt is present, the shunt be designated as having the shunt identifier 1.
    /// `id` = 1 by default.
    pub id: ArrayString<3>,

    /// Shunt status of one for in-service and zero for out-of-service. `status` = 1 by default.
    pub status: bool,

    /// Active component of shunt admittance to ground; entered in MW at one per unit voltage.
    /// `gl` should not include any resistive impedance load, which is entered as part of load
    /// data (see [Load]. `gl` = 0.0 by default.
    pub gl: f64,

    /// Reactive component of shunt admittance to ground; entered in Mvar at one per unit voltage.
    /// `bl` should not include any reactive impedance load, which is entered as part of load data
    /// (see [Load]); line charging and line connected shunts, which are entered as part
    /// of non-transformer branch data (see [Branch]); transformer magnetizing
    /// admittance, which is entered as part of transformer data (see [Transformer]);
    /// or switched shunt admittance, which is entered as part of switched shunt data (see
    /// [SwitchedShunt]. `bl` is positive for a capacitor, and negative for a reactor
    /// or an inductive load. `bl` = 0.0 by default.
    pub bl: f64,
}

/// Each network bus to be represented as a generator or plant bus in PSS/E must be specified
/// in a generator data record. In particular, each bus specified in the bus data input with a
/// type code of two (2) or three (3) must have a generator data record entered for it.
pub struct Generator {
    /// Bus number, or extended bus name enclosed in single quotes.
    pub i: BusNum,

    /// One- or two-character uppercase non blank alphanumeric machine identifier used to distinguish among multiple machines at bus "I".
    /// It is recommended that, at buses for which a single machine is present, the machine be designated as having the machine identifier ’1’.
    /// ID = ’1’ by default.
    pub id: ArrayString<3>, // TODO: confirm 3 is enough in practice, when whitespace can be present

    /// Generator active power output; entered in MW. PG = 0.0 by default.
    pub pg: f64,

    /// Generator reactive power output; entered in Mvar.
    /// QG needs to be entered only if the case, as read in, is to be treated as a solved case.
    /// QG = 0.0 by default.
    pub qg: f64,

    /// Maximum generator reactive power output; entered in Mvar.
    /// For fixed output generators (i.e., nonregulating), QT must be equal to the fixed Mvar output.
    /// QT = 9999.0 by default.
    pub qt: f64,

    /// Minimum generator reactive power output; entered in Mvar.
    /// For fixed output generators, QB must be equal to the fixed Mvar output.
    /// QB = -9999.0 by default.
    pub qb: f64,

    /// Regulated voltage setpoint; entered in pu. VS = 1.0 by default.
    pub vs: f64,

    /// Bus number, or extended bus name enclosed in single quotes,
    /// of a remote type 1 or 2 bus whose voltage is to be regulated by this plant to the value specified by VS.
    /// If bus IREG is other than a type 1 or 2 bus, bus "I" regulates its own voltage to the value specified by VS.
    /// IREG is entered as zero if the plant is to regulate its own voltage and must be zero for a type three (swing) bus.
    /// IREG = 0 by default.
    pub ireg: BusNum,

    /// Total MVA base of the units represented by this machine; entered in MVA.
    /// This quantity is not needed in normal power flow and equivalent onstruction work,
    /// but is required for switching studies, fault analysis, and dynamic simulation.
    /// MBASE = system base MVA by default.
    pub mbase: f64,

    /// Complex machine impedance, ZSORCE; entered in pu on MBASE base.
    /// This data is not needed in normal power flow and equivalent construction work,
    /// but is required for switching studies, fault analysis, and dynamic simulation.
    /// For dynamic simulation, this impedance must be set equal to the unsaturated subtransient impedance for those generators to be modeled by subtransient level machine models,
    /// and to unsaturated transient impedance for those to be modeled by classical or transient level models.
    /// For short-circuit studies, the saturated subtransient or transient impedance should be used.
    /// ZR = 0.0 by default.
    pub zr: f64,

    /// See `zr`. ZX = 1.0 by default.
    pub zx: f64,

    /// Step-up transformer impedance, XTRAN; entered in pu on MBASE base.
    /// XTRAN should be entered as zero if the step-up transformer is explicitly modeled as a network branch and bus "I" is the terminal bus.
    /// RT+jXT = 0.0 by default.
    pub rt: f64,

    /// See `rt`. RT+jXT = 0.0 by default.
    pub xt: f64,

    /// Step-up transformer off-nominal turns ratio; entered in pu.
    /// GTAP is used only if XTRAN is nonzero.
    /// GTAP = 1.0 by default.
    pub gtap: f64,

    /// Initial machine status of one for in-service and zero for out-of-service.
    /// STAT = 1 by default.
    pub stat: bool,

    /// Percent of the total Mvar required to hold the voltage at the bus controlled by this bus "I" that are to be contributed by the generation at bus "I";
    /// RMPCT must be positive.
    /// RMPCT is needed if IREG specifies a valid remote bus and there is more than one local or remote voltage controlling device
    /// (plant, switched shunt, FACTS device shunt element, or VSC DC line converter) controlling the voltage at bus IREG to a setpoint.
    /// RMPCT is needed also if bus "I" itself is being controlled locally or remotely by one or more other setpoint mode voltage controlling devices.
    /// RMPCT = 100.0 by default.
    pub rmpct: f64,

    /// Maximum generator active power output; entered in MW. PT = 9999.0 by default.
    pub pt: f64,

    /// Minimum generator active power output; entered in MW. PB = -9999.0 by default.
    pub pb: f64,

    /// Owner number (1 through the maximum number of owners at the current size level).
    /// Each machine may have up to four owners. See [Owner].
    /// By default, `o1` is the owner to which bus `i` is assigned and `o2`, `o3`, and `o4` are
    /// zero.
    pub o1: OwnerNum,

    /// Fraction of total ownership assigned to owner `oi`; each `fi` must be positive.
    /// The `fi` values are normalized such that they sum to 1.0 before they are placed in the working case.
    /// By default, each `fi` is 1.0.
    pub f1: Option<f64>,
    pub o2: Option<OwnerNum>,
    pub f2: Option<f64>,
    pub o3: Option<OwnerNum>,
    pub f3: Option<f64>,
    pub o4: Option<OwnerNum>,
    pub f4: Option<f64>,

    /// Wind machine control mode; `wmod` is used to indicate whether a machine is a wind machine,
    /// and, if it is, the type of reactive power limits to be imposed.
    /// * 0 for a machine that is not a wind machine.
    /// * 1 for a wind machine for which reactive power limits are specified by QT and QB.
    /// * 2 for a wind machine for which reactive power limits are determined from the machine’s
    ///   active power output and `wpf`; limits are of equal magnitude and opposite sign.
    /// * 3 for a wind machine with a fixed reactive power setting determined from the machine’s
    ///   active power output and `wpf`; when `wpf` is positive, the machine’s reactive power has
    ///   the same sign as its active power; when `wpf` is negative, the machine’s reactive power
    ///   has the opposite sign of its active power.
    /// `wmod` = 0 by default.
    pub wmod: Option<i8>, // 0, 1, 2, or 3

    /// Power factor used in calculating reactive power limits or output when `wmod` is 2 or 3.
    /// `wpf` = 1.0 by default.
    pub wpf: Option<f64>,
}

pub enum Branch {
    Branch30(Branch30),
    Branch33(Branch33),
}

/// In PSS/E, the basic transmission line model is an Equivalent Pi connected between network buses.
///
/// Data for shunt equipment units, such as reactors, which are connected to and switched with the line,
/// are entered in the same data record.
///
/// !!! compat "Shunts connected to buses"
///     In PSSE v30, to represent shunts connected to buses, that shunt data should be entered in the [Bus] data records.
///
/// !!! note "Transformers"
///     Branches to be modeled as transformers are not specified in this data category;
///     rather, they are specified in the [Transformer] data category.
pub struct Branch30 {
    /// Branch "from bus" number, or extended bus name enclosed in single quotes.
    pub i: BusNum,

    /// Branch "to bus" number, or extended bus name enclosed in single quotes.
    /// "J" is entered as a negative number, or with a minus sign before the first character of the extended bus name,
    /// to designate it as the metered end; otherwise, bus "I" is assumed to be the metered end.
    pub j: BusNum,

    /// One- or two-character uppercase nonblank alphanumeric branch circuit identifier;
    /// the first character of CKT must not be an ampersand "&".
    /// It is recommended that single circuit branches be designated as having the circuit identifier '1'.
    /// CKT = '1' by default.
    pub ckt: ArrayString<3>,

    /// Branch resistance; entered in pu. A value of R must be entered for each branch.
    pub r: f64,

    /// Branch reactance; entered in pu. A nonzero value of X must be entered for each branch.
    pub x: f64,

    /// Total branch charging susceptance; entered in pu. B = 0.0 by default.
    pub b: f64,

    /// First loading rating; entered in MVA.
    /// If RATEA is set to 0.0, the default value, this branch will not be included in any examination of circuit loading.
    ///
    /// Ratings are entered as:
    /// ``MVA_{rated} = sqrt(3) × E_{base} × I_{rated} × 10^{-6}`` where:
    /// - ``E_{base}`` is the base line-to-line voltage in volts of the buses to which the terminal of the branch is connected.
    /// - ``I_{rated}`` is the branch rated phase current in amperes.
    pub rate_a: f64,

    /// Second loading rating; entered in MVA. RATEB = 0.0 by default.
    pub rate_b: f64,

    /// Third loading rating; entered in MVA. RATEC = 0.0 by default.
    pub rate_c: f64,

    /// Complex admittance of the line shunt at the bus "I" end of the branch; entered in pu.
    /// BI is negative for a line connected reactor and positive for line connected capacitor.
    /// GI + jBI = 0.0 by default.
    pub gi: f64,

    /// Complex admittance of the line shunt at the bus "I" end of the branch; entered in pu.
    /// BI is negative for a line connected reactor and positive for line connected capacitor.
    /// GI + jBI = 0.0 by default.
    pub bi: f64,

    /// Complex admittance of the line shunt at the bus "J" end of the branch; entered in pu.
    /// BJ is negative for a line connected reactor and positive for line connected capacitor.
    /// GJ + jBJ = 0.0 by default.
    pub gj: f64,

    /// Complex admittance of the line shunt at the bus "J" end of the branch; entered in pu.
    /// BJ is negative for a line connected reactor and positive for line connected capacitor.
    /// GJ + jBJ = 0.0 by default.
    pub bj: f64,

    /// Initial branch status where 1 designates in-service and 0 designates out-of-service.
    /// ST = 1 by default.
    pub st: bool,

    // $met_doc
    // $met_col
    /// Line length; entered in user-selected units. LEN = 0.0 by default.
    pub len: f64,

    /// Owner number; 1 through the maximum number of owners at the current size level.
    /// Each branch may have up to four owners. See [Owner].
    /// By default, `o1` is the owner to which bus `i` is assigned and `o2`, `o3`, and `o4` are zero.
    pub o1: OwnerNum,

    /// Fraction of total ownership assigned to owner ``O_i``; each ``F_i`` must be positive.
    /// The ``fi` values are normalized such that they sum to 1.0 before they are placed in the working case.
    /// By default, each `fi` is 1.0.
    pub f1: f64,
    pub o2: Option<OwnerNum>,
    pub f2: Option<f64>,
    pub o3: Option<OwnerNum>,
    pub f3: Option<f64>,
    pub o4: Option<OwnerNum>,
    pub f4: Option<f64>,
}

/// In PSS/E, the basic transmission line model is an Equivalent Pi connected between network buses.
///
/// Data for shunt equipment units, such as reactors, which are connected to and switched with the line,
/// are entered in the same data record.
///
/// !!! compat "Shunts connected to buses"
///     In PSSE v33, to represent shunts connected to buses, that shunt data should be entered in [FixedShunt] and/or [SwitchedShunt] data records.
///
/// !!! note "Transformers"
///     Branches to be modeled as transformers are not specified in this data category;
///     rather, they are specified in the [Transformer] data category.
pub struct Branch33 {
    /// Branch "from bus" number, or extended bus name enclosed in single quotes.
    pub i: BusNum,

    /// Branch "to bus" number, or extended bus name enclosed in single quotes.
    /// "J" is entered as a negative number, or with a minus sign before the first character of the extended bus name,
    /// to designate it as the metered end; otherwise, bus "I" is assumed to be the metered end.
    pub j: BusNum,

    /// One- or two-character uppercase nonblank alphanumeric branch circuit identifier;
    /// the first character of CKT must not be an ampersand "&".
    /// It is recommended that single circuit branches be designated as having the circuit identifier '1'.
    /// CKT = '1' by default.
    pub ckt: ArrayString<3>,

    /// Branch resistance; entered in pu. A value of R must be entered for each branch.
    pub r: f64,

    /// Branch reactance; entered in pu. A nonzero value of X must be entered for each branch.
    pub x: f64,

    /// Total branch charging susceptance; entered in pu. B = 0.0 by default.
    pub b: f64,

    /// First loading rating; entered in MVA.
    /// If RATEA is set to 0.0, the default value, this branch will not be included in any examination of circuit loading.
    ///
    /// Ratings are entered as:
    /// ``MVA_{rated} = sqrt(3) × E_{base} × I_{rated} × 10^{-6}`` where:
    /// - ``E_{base}`` is the base line-to-line voltage in volts of the buses to which the terminal of the branch is connected.
    /// - ``I_{rated}`` is the branch rated phase current in amperes.
    pub rate_a: f64,

    /// Second loading rating; entered in MVA. RATEB = 0.0 by default.
    pub rate_b: f64,

    /// Third loading rating; entered in MVA. RATEC = 0.0 by default.
    pub rate_c: f64,

    /// Complex admittance of the line shunt at the bus "I" end of the branch; entered in pu.
    /// BI is negative for a line connected reactor and positive for line connected capacitor.
    /// GI + jBI = 0.0 by default.
    pub gi: f64,

    /// Complex admittance of the line shunt at the bus "I" end of the branch; entered in pu.
    /// BI is negative for a line connected reactor and positive for line connected capacitor.
    /// GI + jBI = 0.0 by default.
    pub bi: f64,

    /// Complex admittance of the line shunt at the bus "J" end of the branch; entered in pu.
    /// BJ is negative for a line connected reactor and positive for line connected capacitor.
    /// GJ + jBJ = 0.0 by default.
    pub gj: f64,

    /// Complex admittance of the line shunt at the bus "J" end of the branch; entered in pu.
    /// BJ is negative for a line connected reactor and positive for line connected capacitor.
    /// GJ + jBJ = 0.0 by default.
    pub bj: f64,

    /// Initial branch status where 1 designates in-service and 0 designates out-of-service.
    /// ST = 1 by default.
    pub st: bool,

    /// Metered end flag.
    /// * ≤1 to designate bus `i` as the metered end.
    /// * ≥2 to designate bus `j` as the metered end.
    /// `met` = 1 by default.
    pub met: i8,

    /// Line length; entered in user-selected units. LEN = 0.0 by default.
    pub len: f64,

    /// Owner number; 1 through the maximum number of owners at the current size level.
    /// Each branch may have up to four owners. See [Owner].
    /// By default, `o1` is the owner to which bus `i` is assigned and `o2`, `o3`, and `o4` are zero.
    pub o1: OwnerNum,

    /// Fraction of total ownership assigned to owner ``O_i``; each ``F_i`` must be positive.
    /// The ``fi` values are normalized such that they sum to 1.0 before they are placed in the working case.
    /// By default, each `fi` is 1.0.
    pub f1: f64,
    pub o2: Option<OwnerNum>,
    pub f2: Option<f64>,
    pub o3: Option<OwnerNum>,
    pub f3: Option<f64>,
    pub o4: Option<OwnerNum>,
    pub f4: Option<f64>,
}

/// Each AC transformer to be represented in PSS/E is introduced through transformer data records
/// that specify all the data required to model transformers in power flow calculations, with
/// one exception.
///
/// That exception is a set of ancillary data, comprising transformer impedance correction records,
/// which define the manner in which transformer impedance changes as off-nominal turns ratio or
/// phase shift angle is adjusted. Those data records are described in Transformer Impedance
/// Correction Records, [ImpedanceCorrection].
///
/// Both two-winding and three-winding transformers are specified in the transformer data records.
/// The data records for the two-winding transformer are common to the three-winding transformer;
/// the data block for two-winding transformers is a subset of the data required for three-winding
/// transformers.
pub struct Transformer {
    // first row //
    /// The bus number, or extended bus name enclosed in single quotes, of the bus to which the
    /// first winding is connected. The transformer’s magnetizing admittance is modeled on winding one.
    /// The first winding is the only winding of a two-winding transformer whose tap ratio or
    /// phase shift angle may be adjusted by the power flow solution activities;
    /// any winding(s) of a three-winding transformer may be adjusted. No default is allowed.
    pub i: BusNum,

    /// The bus number, or extended bus name enclosed in single quotes, of the bus to which the
    /// second winding is connected. This winding may have a fixed, off-nominal tap ratio
    /// assigned to it. No default is allowed.
    pub j: BusNum,

    /// The bus number, or extended bus name enclosed in single quotes, of the bus to which the
    /// third winding is connected. Zero is used to indicate that no third winding is present.
    /// _Always equal to zero for a two-winding transformer._
    pub k: BusNum,

    /// One- or two-character uppercase nonblank alphanumeric transformer circuit identifier;
    /// the first character of `ckt` must not be an ampersand ('&').
    pub ckt: ArrayString<3>,

    /// The winding data I/O code which defines the units in which the turns ratios `windv1` and
    /// `windv2` are specified (the units of `rma1` and `rmi1` are also governed by `cw` when
    /// `|cod1|` is 1 or 2):
    /// * 1 for off-nominal turns ratio in pu of winding bus base voltage;
    /// * 2 for winding voltage in kV.
    /// `cw` = 1 by default.
    pub cw: i8, // 1 or 2

    /// The impedance data I/O code that defines the units in which the winding impedances
    /// `r1_2` and `x1_2` are specified:
    /// * 1 for resistance and reactance in pu on system base quantities;
    /// * 2 for resistance and reactance in pu on a specified base MVA and winding bus base voltage;
    /// * 3 for transformer load loss in watts and impedance magnitude in pu on a specified base MVA
    ///   and winding bus base voltage.
    /// `cz` = 1 by default.
    pub cz: i8, // 1, 2 or 3

    /// The magnetizing admittance I/O code that defines the units in which `mag1` and `mag2` are specified:
    /// * 1 for complex admittance in pu on system base quantities;
    /// * 2 for no load loss in watts and exciting current in pu on winding one to two base MVA
    ///   and nominal voltage.
    /// `cm` = 1 by default.
    pub cm: i8, // 1 or 2

    /// When `cm` is 1, `mag1` is the magnetizing conductance in pu on system base quantities;
    /// when `cm` is 2, `mag1` is the no load loss in watts.
    /// `mag1` = 0.0 by default.
    pub mag1: f64,

    /// When `cm` is 1, `mag2` is the magnetizing susceptance in pu on system base quantities;
    /// when `cm` is 2, `mag2` is the exciting current in pu on winding one to two base MVA (`sbase1_2`)
    /// and nominal voltage (`nomv1`).
    /// `mag2` = 0.0 by default.
    pub mag2: f64,

    /// The nonmetered end code of either:
    /// * 1 (for the winding one bus), or
    /// * 2 (for the winding two bus).
    /// `nmetr` = 2 by default.
    pub nmetr: i8, // 1 or 2

    /// An alphanumeric identifier assigned to the transformer. The name may be up to twelve characters.
    /// `name` may contain any combination of blanks, uppercase letters, numbers and special characters.
    /// `name` is twelve blanks by default.
    pub name: ArrayString<15>,

    /// The initial transformer status, where 1 designates in-service and 0 designates out-of-service.
    /// `stat` = 1 by default.
    pub stat: bool,

    /// An owner number; (1 through the maximum number of owners at the current size level).
    /// Each transformer may have up to four owners. See [Owner].
    /// By default, O1 is the owner to which bus "I" is assigned
    pub o1: OwnerNum,

    /// The fraction of total ownership assigned to owner `Oi`; each Fi must be positive.
    /// The Fi values are normalized such that they sum to 1.0 before they are placed in the working case.
    /// By default, each `fi` is 1.0.
    pub f1: f64,
    pub o2: Option<OwnerNum>,
    pub f2: Option<f64>,
    pub o3: Option<OwnerNum>,
    pub f3: Option<f64>,
    pub o4: Option<OwnerNum>,
    pub f4: Option<f64>,

    /// Alphanumeric identifier specifying vector group based on transformer winding connections and phase angles.
    /// `vecgrp` value is used for information purpose only.
    /// `vecgrp` is 12 blanks by default.
    pub vecgrp: Option<ArrayString<15>>,

    // second row //
    /// The measured impedance of the transformer between the buses to which its first and second
    /// windings are connected (see also `x1_2`).
    /// * When `cz` is 1, `r1_2` is the resistance in pu on system base quantities;
    /// * when `cz` is 2, `r1_2` is the resistance in pu on winding one to two base MVA (`sbase1_2`) and winding one bus base voltage;
    /// * when `cz` is 3, `r1_2` is the load loss in watts.
    /// `r1_2` = 0.0  by default.
    pub r1_2: f64,

    /// The measured impedance of the transformer between the buses to which its first and second
    /// windings are connected (see also `r1_2`).
    /// * When `cz` is 1, `x1_2` is the reactance in pu on system base quantities;
    /// * when `cz` is 2, `x1_2` is the reactance in pu on winding one to two base MVA (`sbase1_2`) and winding one bus base voltage;
    /// * when `cz` is 3, `x1_2` is the impedance magnitude in pu on winding one to two base MVA (`sbase1_2`) and winding one bus base voltage.
    /// `x1_2` has no default.
    pub x1_2: f64,

    /// The winding one to two base MVA of the transformer.
    /// `sbase1_2` = `sbase` (the system base MVA) by default.
    pub sbase1_2: f64,

    // second row, but 3-winding transformers only //
    /// The measured impedance of a three-winding transformer between the buses to which its
    /// second and third windings are connected (see also `x2_3`).
    /// * When `cz` is 1, `r2_3` is the resistance in pu on system base quantities;
    /// * when `cz` is 2, `r2_3` is the resistance in pu on winding two to three base MVA (`sbase2_3`) and winding two bus base voltage;
    /// * when `cz` is 3, `r2_3` is the load loss in watts
    /// `r2_3` = 0.0 by default.
    /// _Ignored for a two-winding transformer._
    pub r2_3: Option<f64>,

    /// The measured impedance of a three-winding transformer between the buses to which its
    /// second and third windings are connected (see also `x2_3`).
    /// * When `cz` is 1, `x2_3` is the reactance in pu on system base quantities;
    /// * when `cz` is 2, `x2_3` is the reactance in pu on winding one to two base MVA (`sbas2_3`) and winding one bus base voltage;
    /// * when `cz` is 3, `x2_3` is the impedance magnitude in pu on winding two to three base MVA (`sbase2_3`) and winding two bus base voltage.
    /// `x2_3` has no default.
    /// _Ignored for a two-winding transformer._
    pub x2_3: Option<f64>,

    /// The winding two to three base MVA of a three-winding transformer; ignored for a two-winding
    /// transformer.
    /// `sbase2_3` = `sbase` (the system base MVA) by default.
    /// _Ignored for a two-winding transformer._
    pub sbase2_3: Option<f64>,

    /// The measured impedance of a three-winding transformer between the buses to which its
    /// third and first windings are connected (see also `x3_1`).
    /// * When `cz` is 1, `r3_1` is the resistance in pu on system base quantities;
    /// * when `cz` is 2, `r3_1` is the resistance in pu on winding three to one base MVA (`sbase3_1`) and winding three bus base voltage;
    /// * when `cz` is 3, `r3_1` is the load loss in watts
    /// `r3_1` = 0.0 by default.
    /// _Ignored for a two-winding transformer._
    pub r3_1: Option<f64>,

    /// The measured impedance of a three-winding transformer between the buses to which its
    /// third and first windings are connected (see also `x3_1`).
    /// * When `cz` is 1, `x3_1` is the reactance in pu on system base quantities;
    /// * when `cz` is 2, `x3_1` is the reactance in pu on winding three to one base MVA (`sbas3_1`) and winding three bus base voltage;
    /// * when `cz` is 3, `x3_1` is the impedance magnitude in pu on winding three to one base MVA (`sbase3_1`) and winding three bus base voltage.
    /// `x3_1` has no default.
    /// _Ignored for a two-winding transformer._
    pub x3_1: Option<f64>,

    /// The winding three to one base MVA of a three-winding transformer.
    /// `sbase3_1` = `sbase` (the system base MVA) by default.
    /// _Ignored for a two-winding transformer._
    pub sbase3_1: Option<f64>,

    /// The voltage magnitude at the hidden star point bus; entered in pu.
    /// `vmstar` = 1.0 by default.
    /// _Ignored for a two-winding transformer._
    pub vmstar: Option<f64>,

    /// The bus voltage phase angle at the hidden star point bus; entered in degrees.
    /// `anstar` = 0.0 by default.
    /// _Ignored for a two-winding transformer._
    pub anstar: Option<f64>,

    // third row //
    /// When `cw` is 1, `windv1` is the winding one off-nominal turns ratio in pu of winding one bus base voltage,
    /// and windv1 = 1.0 by default.
    /// When `cw` is 2, `windv1` is the actual winding one voltage in kV,
    /// and `windv1` is equal to the base voltage of bus "I" by default.
    pub windv1: f64,

    /// The nominal (rated) winding one voltage in kV, or zero to indicate that nominal winding
    /// one voltage is to be taken as the base voltage of bus "I".
    /// `nomv1` is used only in converting magnetizing data between per unit admittance values
    /// and physical units when `cm` is 2.
    /// `nomv1` = 0.0 by default.
    pub nomv1: f64,

    /// The winding one phase shift angle in degrees.
    /// `ang1` is positive for a positive phase shift from the winding one side to the winding two side (for a two-winding transformer).
    /// `ang1` must be greater than -180.0 and less than or equal to +180.0.
    /// `ang1` = 0.0 by default.
    pub ang1: f64,

    /// The first winding’s first rating entered in MVA (not current expressed in MVA).
    pub rata1: f64,

    /// The first winding’s second rating entered in MVA (not current expressed in MVA).
    pub ratb1: f64,

    /// The first winding’s third rating entered in MVA (not current expressed in MVA).
    pub ratc1: f64,

    /// The transformer control mode for automatic adjustments of the winding one tap or
    /// phase shift angle during power flow solutions:
    /// * 0 for no control (fixed tap and phase shift);
    /// * ±1 for voltage control;
    /// * ±2 for reactive power flow control;
    /// * ±3 for active power flow control;
    /// * ±4 for control of a DC line quantity.
    /// If the control mode is entered as a positive number, automatic adjustment of this transformer
    /// winding is enabled when the corresponding adjustment is activated during power flow solutions;
    /// a negative control mode suppresses the automatic adjustment of this transformer winding.
    /// `cod1` = 0 by default.
    pub cod1: i8, // one of: -4, -3, -2, -1, 0, 1, 2, 3, 4

    /// The bus number, or extended bus name enclosed in single quotes, of the bus whose voltage
    /// is to be controlled by the transformer turns ratio adjustment option of the power flow
    /// solution activities when `cod1` is 1.
    ///
    /// `cont1` should be non-zero only for voltage controlling transformer windings.
    /// `cont1` may specify a bus other than "I", "J", or "K"; in this case, the sign of `cont1`
    /// defines the location of the controlled bus relative to the transformer winding.
    ///
    /// If `cont1` is entered as a positive number, the ratio is adjusted as if bus `cont1` is on the winding two side of the transformer;
    /// if `cont1` is entered as a negative number, the ratio is adjusted as if bus `|cont1|` is on the winding one side of the transformer.
    /// `cont1` = 0 by default.
    pub cont1: BusNum,

    /// `rma1` is the upper limit (and `rmi1` the lower limit) of either:
    /// * Off-nominal turns ratio in pu of winding one bus base voltage when `|cod1|` is 1 or 2 and `cw` is 1;
    ///   `rma1` = 1.1 and `rmi1` = 0.9 by default.
    /// * Actual winding one voltage in kV when `|cod1|` is 1 or 2 and `cw` is 2. No default is allowed.
    /// * Phase shift angl e in degrees when `|cod1|` is 3. No default is allowed.
    /// * Not used when `|cod1|` is 0 or 4;
    /// `rma1` = 1.1 and `rmi1` = 0.9 by default.
    pub rma1: f64,

    /// The lower limit to `rma1`'s upper limit. See `rma1` for details.
    pub rmi1: f64,

    /// `vma1` is the upper limit (and `vmi1` the lower limit) of either:
    /// * Voltage at the controlled bus (bus `|cont1|`) in pu when `|cod1|` is 1. `vma1` = 1.1 and `vmi1` = 0.9 by default.
    /// * Reactive power flow into the transformer at the winding one bus end in Mvar when `|cod1|` is 2. no default is allowed.
    /// * Active power flow into the transformer at the winding one bus end in MW when `|cod1|` is 3. no default is allowed.
    /// * Not used when `|cod1|` is 0 or 4; `vma1` = 1.1 and `vmi1` = 0.9 by default.
    pub vma1: f64,

    /// The lower limit to `vma1`'s upper limit. See `vma1` for details.
    pub vmi1: f64,

    /// The number of tap positions available; used when `cod1` is 1 or 2.
    /// `ntp1` must be between 2 and 9999.
    /// `ntp1` = 33 by default.
    pub ntp1: i16,

    /// The number of a transformer impedance correction record if this transformer winding’s
    /// impedance is to be a function of either off-nominal turns ratio or phase shift angle,
    /// or 0 if no transformer impedance correction is to be applied to this transformer winding.
    /// See [ImpedanceCorrection].
    /// `tab1` = 0 by default.
    pub tab1: isize,

    /// The load drop compensation impedance for voltage controlling transformers entered in pu
    /// on system base quantities; used when `cod1` is 1.
    /// `cr1` + j`cx1` = 0.0 by default.
    pub cr1: f64,

    /// See `cr1` for details.
    pub cx1: f64,

    /// Winding connection angle in degrees; used when `cod1` is 5.
    /// There are no restrictions on the value specified for `cnxa1`;
    /// if it is outside of the range from -90.0 to +90.0, `cnxa1` is normalized to within this range.
    /// `cnxa1` = 0.0 by default.
    pub cnxa1: Option<f64>,

    // fourth row //
    /// When `cw` is 1, `windv2` is the winding two off-nominal turns ratio in pu of winding two bus base voltage,
    /// and `windv2` = 1.0 by default.
    /// When `cw` is 2, `windv2` is the actual winding two voltage in kV,
    /// and `windv2` is equal to the base voltage of bus `j` by default.
    pub windv2: f64,

    /// The nominal (rated) winding two voltage in kV, or zero to indicate that nominal winding
    /// two voltage is to be taken as the base voltage of bus `j`.
    /// `nomv2` is present for information purposes only; it is not used in any of the calculations
    /// for modeling the transformer.
    /// `nomv2` = 0.0 by default.
    pub nomv2: f64,

    // fourth row, but 3-winding transformers only //
    /// The winding two phase shift angle in degrees.
    /// `ang2` is positive for a positive phase shift from the winding two side to the "T"
    /// (or star) point bus.
    /// `ang2` must be greater than -180.0 and less than or equal to +180.0.
    /// `ang2` = 0.0 by default.
    /// _Ignored for a two-winding transformer._
    pub ang2: Option<f64>,

    /// The second winding’s first rating entered in MVA (not current expressed in MVA).
    /// _Ignored for a two-winding transformer._
    pub rata2: Option<f64>,

    /// The second winding’s second rating entered in MVA (not current expressed in MVA).
    /// _Ignored for a two-winding transformer._
    pub ratb2: Option<f64>,

    /// The second winding’s third rating entered in MVA (not current expressed in MVA).
    /// _Ignored for a two-winding transformer._
    pub ratc2: Option<f64>,

    /// The transformer control mode for automatic adjustments of the winding two tap or phase
    /// shift angle during power flow solutions:
    /// * 0 for no control (fixed tap and phase shift);
    /// * ±1 for voltage control;
    /// * ±2 for reactive power flow control;
    /// * ±3 for active power flow control.
    /// If the control mode is entered as a positive number, automatic adjustment of this transformer
    /// winding is enabled when the corresponding adjustment is activated during power flow solutions;
    /// a negative control mode suppresses the automatic adjustment of this transformer winding.
    /// `cod2` = 0 by default.
    /// _Ignored for a two-winding transformer._
    pub cod2: Option<i8>, // one of: -3, -2, -1, 0, 1, 2, 3

    /// The bus number, or extended bus name enclosed in single quotes, of the bus whose voltage
    /// is to be controlled by the transformer turns ratio adjustment option of the power flow
    /// solution activities when `cod2` is 1.
    /// `cont2` should be nonzero only for voltage controlling transformer windings.
    ///
    /// `cont2` may specify a bus other than `i`, `j`, or `k`; in this case, the sign of
    /// `cont2` defines the location of the controlled bus relative to the transformer winding.
    /// If `cont2` is entered as a positive number, or a quoted extended bus name, the ratio is
    /// adjusted as if bus `cont2` is on the winding one or winding three side of the transformer;
    /// if `cont2` is entered as a negative number, or a quoted extended bus name with a minus sign
    /// preceding the first character, the ratio is adjusted as if bus `|cont2|` is on the winding
    /// two side of the transformer. `cont2` = 0 by default.
    /// _Ignored for a two-winding transformer._
    pub cont2: Option<BusNum>,

    /// `rma2` is the upper limit (and `rmi2` the lower limit) of either:
    /// * Off-nominal turns ratio in pu of winding two bus base voltage when `|cod2|` is 1 or 2 and `cw` is 1;
    ///     `rma2` = 1.1 and `rmi2` = 0.9 by default.
    /// * Actual winding one voltage in kV when `|cod2|` is 1 or 2 and `cw` is 2. No default is allowed.
    /// * Phase shift angle in degrees when `|cod2|` is 3. No default is allowed.
    /// * Not used when `|cod2|` is 0;
    /// `rma2` = 1.1 and `rmi2` = 0.9 by default.
    /// _Ignored for a two-winding transformer._
    pub rma2: Option<f64>,

    /// The lower limit to `rma2`'s upper limit. See `rma2` for details.
    /// _Ignored for a two-winding transformer._
    pub rmi2: Option<f64>,

    /// `vma2` is the upper limit (and `vmi2` the lower limit) of either:
    /// * Voltage at the controlled bus (bus `|cont2|`) in pu when `|cod2|` is 1.
    ///     `vma2` = 1.1 and `vmi2` = 0.9 by default.
    /// * Reactive power flow into the transformer at the winding two bus end in Mvar when `|cod2|` is 2.
    ///     No default is allowed.
    /// * Active power flow into the transformer at the winding two bus end in MW when `|cod2|` is 3.
    ///     No default is allowed.
    /// * Not used when `|cod2|` is 0; `vma2` = 1.1 and `vmi2` = 0.9 by default.
    /// _Ignored for a two-winding transformer._
    pub vma2: Option<f64>,

    /// The lower limit to `vma1`'s upper limit. See `vma1` for details.
    /// _Ignored for a two-winding transformer._
    pub vmi2: Option<f64>,

    /// The number of tap positions available; used when `cod2` is 1 or 2.
    /// `ntp2` must be between 2 and 9999.
    /// `ntp2` = 33 by default.
    /// _Ignored for a two-winding transformer._
    pub ntp2: Option<i16>,

    /// The number of a transformer impedance correction record if this transformer winding’s
    /// impedance is to be a function of either off-nominal turns ratio or phase shift angle,
    /// or 0 if no transformer impedance correction is to be applied to this transformer winding.
    /// See [ImpedanceCorrection].
    /// `tab2` = 0 by default.
    /// _Ignored for a two-winding transformer._
    pub tab2: Option<isize>,

    /// The load drop compensation impedance for voltage controlling transformers entered in pu
    /// on system base quantities; used when `cod2` is 1.
    /// `cr2` + j`cx2` = 0.0 by default.
    /// _Ignored for a two-winding transformer._
    pub cr2: Option<f64>,

    /// See `cr2` for details.
    /// _Ignored for a two-winding transformer._
    pub cx2: Option<f64>,

    /// Winding connection angle in degrees; used when `cod2` is 5.
    /// There are no restrictions on the value specified for `cnxa2`;
    /// if it is outside of the range from -90.0 to +90.0, `cnxa2` is normalized to within this range.
    /// `cnxa2` = 0.0 by default.
    pub cnxa2: Option<f64>,

    // fifth row, only 3-winding transformers //
    /// When `cw` is 1, `windv3` is the winding three off-nominal turns ratio in pu of winding three bus base voltage,
    /// and windv3 = 1.0 by default.
    /// When `cw` is 2, `windv3` is the actual winding three voltage in kV,
    /// and `windv3` is equal to the base voltage of bus `k` by default.
    /// _Ignored for a two-winding transformer._
    pub windv3: Option<f64>,

    /// The nominal (rated) winding three voltage in kV, or zero to indicate that nominal winding
    /// two voltage is to be taken as the base voltage of bus `j`.
    /// `nomv3` is present for information purposes only; it is not used in any of the calculations
    /// for modeling the transformer.
    /// `nomv3` = 0.0 by default.
    /// _Ignored for a two-winding transformer._
    pub nomv3: Option<f64>,

    /// The winding three phase shift angle in degrees.
    /// `ang3` is positive for a positive phase shift from the winding two side to the "T"
    /// (or star) point bus.
    /// `ang3` must be greater than -180.0 and less than or equal to +180.0.
    /// `ang3` = 0.0 by default.
    /// _Ignored for a two-winding transformer._
    pub ang3: Option<f64>,

    /// The third winding’s first rating entered in MVA (not current expressed in MVA).
    /// _Ignored for a two-winding transformer._
    pub rata3: Option<f64>,

    /// The third winding’s second rating entered in MVA (not current expressed in MVA).
    /// _Ignored for a two-winding transformer._
    pub ratb3: Option<f64>,

    /// The third winding’s third rating entered in MVA (not current expressed in MVA).
    /// _Ignored for a two-winding transformer._
    pub ratc3: Option<f64>,

    /// The transformer control mode for automatic adjustments of the winding three tap or phase
    /// shift angle during power flow solutions:
    /// * 0 for no control (fixed tap and phase shift);
    /// * ±1 for voltage control;
    /// * ±2 for reactive power flow control;
    /// * ±3 for active power flow control.
    /// If the control mode is entered as a positive number, automatic adjustment of this transformer
    /// winding is enabled when the corresponding adjustment is activated during power flow solutions;
    /// a negative control mode suppresses the automatic adjustment of this transformer winding.
    /// `cod3` = 0 by default.
    /// _Ignored for a two-winding transformer._
    pub cod3: Option<i8>, // one of: -3, -2, -1, 0, 1, 2, 3

    /// The bus number, or extended bus name enclosed in single quotes, of the bus whose voltage
    /// is to be controlled by the transformer turns ratio adjustment option of the power flow
    /// solution activities when `cod3` is 1.
    /// `cont3` should be nonzero only for voltage controlling transformer windings.
    ///
    /// `cont3` may specify a bus other than `i`, `j`, or `k`; in this case, the sign of
    /// `cont3` defines the location of the controlled bus relative to the transformer winding.
    /// If `cont3` is entered as a positive number, or a quoted extended bus name, the ratio is
    /// adjusted as if bus `cont3` is on the winding one or winding two side of the transformer;
    /// if `cont3` is entered as a negative number, or a quoted extended bus name with a minus sign
    /// preceding the first character, the ratio is adjusted as if bus `|cont3|` is on the winding
    /// three side of the transformer. `cont3` = 0 by default.
    /// _Ignored for a two-winding transformer._
    pub cont3: Option<BusNum>,

    /// `rma3` is the upper limit (and `rmi3` the lower limit) of either:
    /// * Off-nominal turns ratio in pu of winding three bus base voltage when `|cod3|` is 1 or 2 and `cw` is 1;
    ///   `rma3` = 1.1 and `rmi3` = 0.9 by default.
    /// * Actual winding one voltage in kV when `|cod3|` is 1 or 2 and `cw` is 2. No default is allowed.
    /// * Phase shift angle in degrees when `|cod3|` is 3. No default is allowed.
    /// * Not used when `|cod3|` is 0;
    /// `rma3` = 1.1 and `rmi3` = 0.9 by default.
    /// _Ignored for a two-winding transformer._
    pub rma3: Option<f64>,

    /// The lower limit to `rma3`'s upper limit. See `rma3` for details.
    /// _Ignored for a two-winding transformer._
    pub rmi3: Option<f64>,

    /// `vma3` is the upper limit (and `vmi3` the lower limit) of either:
    /// * Voltage at the controlled bus (bus `|cont3|`) in pu when `|cod3|` is 1.
    ///   `vma3` = 1.1 and `vmi3` = 0.9 by default.
    /// * Reactive power flow into the transformer at the winding three bus end in Mvar when `|cod3|` is 2.
    ///   No default is allowed.
    /// * Active power flow into the transformer at the winding two bus end in MW when `|cod3|` is 3.
    ///   No default is allowed.
    /// * Not used when `|cod3|` is 0; `vma3` = 1.1 and `vmi3` = 0.9 by default.
    /// _Ignored for a two-winding transformer._
    pub vma3: Option<f64>,

    /// The lower limit to `vma3`'s upper limit. See `vma3` for details.
    /// _Ignored for a two-winding transformer._
    pub vmi3: Option<f64>,

    /// The number of tap positions available; used when `cod3` is 1 or 2.
    /// `ntp3` must be between 2 and 9999.
    /// `ntp3` = 33 by default.
    /// _Ignored for a two-winding transformer._
    pub ntp3: Option<i16>,

    /// The number of a transformer impedance correction record if this transformer winding’s
    /// impedance is to be a function of either off-nominal turns ratio or phase shift angle,
    /// or 0 if no transformer impedance correction is to be applied to this transformer winding.
    /// See [ImpedanceCorrection].
    /// `tab3` = 0 by default.
    /// _Ignored for a two-winding transformer._
    pub tab3: Option<isize>,

    /// The load drop compensation impedance for voltage controlling transformers entered in pu
    /// on system base quantities; used when `cod3` is 1.
    /// `cr3` + j`cx3` = 0.0 by default.
    /// _Ignored for a two-winding transformer._
    pub cr3: Option<f64>,

    /// See `cr3` for details.
    /// _Ignored for a two-winding transformer._
    pub cx3: Option<f64>,

    /// Winding connection angle in degrees; used when `cod3` is 5.
    /// There are no restrictions on the value specified for `cnxa3`;
    /// if it is outside of the range from -90.0 to +90.0, `cnxa3` is normalized to within this range.
    /// `cnxa3` = 0.0 by default.
    pub cnxa3: Option<f64>,
}

/// Area interchange is a required net export of power from, or net import of power to, a
/// specific area. This does not imply that the power is destined to be transferred to or from
/// any other specific area. To specify transfers between specific pairs of areas see
/// `InterAreaTransfers`.
pub struct AreaInterchange {
    /// Area number (1 through the maximum number of areas at the current size level)
    pub i: AreaNum,

    /// Bus number, or extended bus name enclosed in single quotes, of the area slack bus for
    /// area interchange control. The bus must be a generator (type two) bus in the specified
    /// area. Any area containing a system swing bus (type three) must have either that swing
    /// bus or a bus number of zero specified for its area slack bus number.
    /// `isw` = 0 by default.
    pub isw: BusNum,

    /// Desired net interchange leaving the area (export); entered in MW.
    /// `pdes` = 0.0 by default.
    pub pdes: f64,

    /// Interchange tolerance bandwidth; entered in MW.
    /// `ptol` = 10.0 by default.
    pub ptol: f64,

    /// Alphanumeric identifier assigned to area I.
    /// The name may contain up to twelve characters.
    /// `arname` is set to twelve blanks by default.
    pub arname: ArrayString<15>,
}

/// The TwoTerminalDCLines data record depends on the PSSE version:
/// - See [TwoTerminalDCLine30] for PSSE v30 files.
/// - See [TwoTerminalDCLine33] for PSSE v33 files.
pub enum TwoTerminalDCLine {
    TwoTerminalDCLine30(TwoTerminalDCLine30),
    TwoTerminalDCLine33(TwoTerminalDCLine33),
}

/// The two-terminal DC transmission line model is used to simulate either a point-to-point
/// system with rectifier and inverter separated by a bipolar or mono-polar transmission system
/// or a Back-to-Back system where the rectifier and inverter are physically located at the same
/// site and separated only by a short bus-bar.
///
/// The data requirements fall into three groups:
/// * Control parameters and set-points
/// * Converter transformers
/// * The DC line characteristics
///
/// The steady-state model comprising this data enables not only power flow analysis but also
/// establishes the initial steady-state for dynamic analysis.
pub struct TwoTerminalDCLine30 {
    /// The DC line number.
    pub i: LineNum,

    /// Control mode:
    /// * 0 for blocked,
    /// * 1 for power,
    /// * 2 for current.
    /// `mdc` = 0 by default.
    pub mdc: i8, // 0, 1, or 2

    /// The DC line resistance; entered in ohms.
    /// No default.
    pub rdc: f64,

    /// Current (amps) or power (MW) demand.
    /// When `mdc` is 1, a positive value of `setvl` specifies desired power at the rectifier
    /// and a negative value specifies desired inverter power.
    /// No default.
    pub setvl: f64,

    /// Scheduled compounded DC voltage; entered in kV.
    /// No default.
    pub vschd: f64,

    /// Mode switch DC voltage; entered in kV.
    /// When the inverter DC voltage falls below this value and the line is in power control mode
    /// (i.e. `mdc` = 1), the line switches to current control mode with a desired current
    /// corresponding to the desired power at scheduled DC voltage.
    /// `vcmod` = 0.0 by default.
    pub vcmod: f64,

    /// Compounding resistance; entered in ohms.
    /// Gamma and/or TAPI is used to attempt to hold the compounded voltage (``vdci + dccur ∗ rcomp``) at `vschd`.
    /// * To control the inverter end DC voltage VDCI, set `rcomp` to zero;
    /// * to control the rectifier end DC voltage VDCR, set `rcomp` to the DC line resistance, `rdc`;
    /// * otherwise, set `rcomp` to the appropriate fraction of `rdc`.
    /// `rcomp` = 0.0 by default.
    pub rcomp: f64,

    /// Margin entered in per unit of desired DC power or current.
    /// This is the fraction by which the order is reduced when alpha is at its minimum (`alfmn`)
    /// and the inverter is controlling the line current.
    /// `delti` = 0.0 by default.
    pub delti: f64,

    /// Metered end code of either "R" (for rectifier) or "I" (for inverter).
    /// `meter` = "I" by default.
    pub meter: ArrayString<1>, // I or R

    /// Minimum compounded DC voltage; entered in kV.
    /// Only used in constant gamma operation (i.e. when `gammx` = `gammn`) when TAPI is held constant
    /// and an AC transformer tap is adjusted to control DC voltage
    /// (i.e. when `ifi`, `iti`, and `idi` specify a two-winding transformer).
    /// `dcvmin` = 0.0 by default.
    pub dcvmin: f64,

    /// Iteration limit for capacitor commutated two-terminal DC line Newton solution procedure.
    /// `cccitmx` = 20 by default.
    pub cccitmx: i32,

    /// Acceleration factor for capacitor commutated two-terminal DC line Newton solution procedure.
    /// `cccacc` = 1.0 by default.
    pub cccacc: f64,

    // Second line: defines rectifier end data quantities and control parameters //
    /// Rectifier converter bus number, or extended bus name enclosed in single quotes.
    /// No default.
    pub ipr: BusNum,

    /// Number of bridges in series (rectifier).
    /// No default.
    pub nbr: i32,

    /// Nominal maximum rectifier firing angle; entered in degrees.
    /// No default.
    pub alfmx: f64,

    /// Minimum steady-state rectifier firing angle; entered in degrees.
    /// No default.
    pub alfmn: f64,

    /// Rectifier commutating transformer resistance per bridge; entered in ohms.
    /// No default allowed.
    pub rcr: f64,

    /// Rectifier commutating transformer reactance per bridge; entered in ohms.
    /// No default allowed.
    pub xcr: f64,

    /// Rectifier primary base AC voltage; entered in kV.
    /// No default.
    pub ebasr: f64,

    /// Rectifier transformer ratio.
    /// `trr` = 1.0 by default.
    pub trr: f64,

    /// Rectifier tap setting.
    /// `tapr` = 1.0 by default.
    pub tapr: f64,

    /// Maximum rectifier tap setting.
    /// `tmxr` = 1.5 by default.
    pub tmxr: f64,

    /// Minimum rectifier tap setting.
    /// `tmnr` = 0.51 by default.
    pub tmnr: f64,

    /// Rectifier tap step; must be positive.
    /// `stpr` = 0.00625 by default.
    pub stpr: f64,

    /// Rectifier firing angle measuring bus number, or extended bus name enclosed in single quotes.
    /// The firing angle and angle limits used inside the DC model are adjusted by the difference
    /// between the phase angles at this bus and the AC/DC interface (i.e. the converter bus, `ipr`).
    /// `icr` = 0 by default.
    pub icr: BusNum,

    /// Winding one side "from bus" number, or extended bus name enclosed in single quotes,
    /// of a two-winding transformer.
    /// `ifr` = 0 by default.
    pub ifr: BusNum,

    /// Winding two side "to bus" number, or extended bus name enclosed in single quotes,
    /// of a two-winding transformer.
    /// `itr` = 0 by default.
    pub itr: BusNum,

    /// Circuit identifier; the branch described by `ifr`, `itr`, and `idr` must have been entered
    /// as a two-winding transformer; an AC transformer may control at most only one DC converter.
    /// `idr` = '1' by default.
    ///
    /// If no branch is specified, `tapr` is adjusted to keep alpha within limits;
    /// otherwise, `tapr` is held fixed and this transformer’s tap ratio is adjusted.
    /// The adjustment logic assumes that the rectifier converter bus is on the winding two side
    /// of the transformer. The limits `tmxr` and `tmnr` specified here are used; except for the
    /// transformer control mode flag (`cod` of `Transformers`), the AC tap adjustment data is ignored.
    pub idr: ArrayString<3>,

    /// Commutating capacitor reactance magnitude per bridge; entered in ohms.
    /// `xcapr` = 0.0 by default.
    pub xcapr: f64,

    // Third line: contains the inverter quantities corresponding to the rectifier quantities
    // specified on the second line above.  The significant difference is that the control angle
    // `ALFA` for the rectifier is replaced by the control angle `GAMMA` for the inverter.
    /// Inverter converter bus number, or extended bus name enclosed in single quotes.
    pub ipi: BusNum,

    /// Number of bridges in series (inverter).
    pub nbi: i32,

    /// Nominal maximum inverter firing angle; entered in degrees.
    pub gammx: f64,

    /// Minimum steady-state inverter firing angle; entered in degrees.
    pub gammn: f64,

    /// Inverter commutating transformer resistance per bridge; entered in ohms.
    pub rci: f64,

    /// Inverter commutating transformer reactance per bridge; entered in ohms.
    pub xci: f64,

    /// Inverter primary base AC voltage; entered in kV.
    pub ebasi: f64,

    /// Inverter transformer ratio.
    pub tri: f64,

    /// Inverter tap setting.
    pub tapi: f64,

    /// Maximum inverter tap setting.
    pub tmxi: f64,

    /// Minimum inverter tap setting.
    pub tmni: f64,

    /// Inverter tap step; must be positive.
    pub stpi: f64,

    /// Inverter firing angle measuring bus number, or extended bus name enclosed in single quotes.
    pub ici: BusNum,

    /// Winding one side "from bus" number, or extended bus name enclosed in single quotes,
    /// of a two-winding transformer.
    pub ifi: BusNum,

    /// Winding two side "to bus" number, or extended bus name enclosed in single quotes,
    /// of a two-winding transformer.
    pub iti: BusNum,

    /// Circuit identifier; the branch described by `ifr`, `itr`, and `idr` must have been entered
    /// as a two-winding transformer; an AC transformer may control at most only one DC converter.
    pub idi: ArrayString<3>,

    /// Commutating capacitor reactance magnitude per bridge; entered in ohms.
    pub xcapi: f64,
}

/// The two-terminal DC transmission line model is used to simulate either a point-to-point
/// system with rectifier and inverter separated by a bipolar or mono-polar transmission system
/// or a Back-to-Back system where the rectifier and inverter are physically located at the same
/// site and separated only by a short bus-bar.
///
/// The data requirements fall into three groups:
/// * Control parameters and set-points
/// * Converter transformers
/// * The DC line characteristics
///
/// The steady-state model comprising this data enables not only power flow analysis but also
/// establishes the initial steady-state for dynamic analysis.
pub struct TwoTerminalDCLine33 {
    /// The non-blank alphanumeric identifier assigned to this DC line.
    /// Each two-terminal DC line must have a unique `name.
    /// `name` may be up to twelve characters and may contain any combination of blanks, uppercase letters,
    /// numbers and special characters. name must be enclosed in single or double quotes if it contains any
    /// blanks or special characters. No default allowed.
    pub name: ArrayString<15>,
}

/// Voltage source converter (VSC) DC lines.
///
/// Defines line quantities and control parameters, and the converter buses (converter 1 and
/// converter 2), along with their data quantities and control parameters.
pub struct VSCDCLine {
    // First line of data //
    /// The non-blank alphanumeric identifier assigned to this VSC DC line.
    /// Each VSC DC line must have a unique `name`. The name may be up to twelve characters and
    /// must be enclosed in single quotes. `name` may contain any combination of blanks,
    /// uppercase letters, numbers and special characters.
    /// No default.
    pub name: ArrayString<15>,

    /// Control mode:
    /// * 0 for out-of-service,
    /// * 1 for in-service.
    /// `mdc` = 1 by default.
    pub mdc: i8,

    /// The DC line resistance entered in ohms. `rdc` must be positive.
    /// No default.
    pub rdc: f64,

    /// An owner number; (1 through the maximum number of owners at the current size level).
    /// Each VSC DC line may have up to four owners. See [Owner].
    /// By default, `01` is 1, and O2, O3 and O4 are zero.
    pub o1: OwnerNum,

    /// The fraction of total ownership assigned to owner `o1`; each F_i must be positive.
    /// The F_i values are normalized such that they sum to 1.0 before they are placed in the working case.
    /// By default, each F_i is 1.0.
    pub f1: f64,

    // TODO: are o2, f2, o3, f3, o4, f4 always present?
    /// An owner number; (1 through the maximum number of owners at the current size level).
    /// See [Owner].
    /// By default, `o2` is zero.
    pub o2: OwnerNum,

    /// The fraction of total ownership assigned to owner `o2`; must be positive.
    /// By default, `f2` is 1.0.
    pub f2: f64,

    /// An owner number; (1 through the maximum number of owners at the current size level).
    /// By default, `o3` is zero.
    pub o3: OwnerNum,

    /// The fraction of total ownership assigned to owner `o2`; must be positive.
    /// By default, `f3` is 1.0.
    pub f3: f64,

    /// An owner number; (1 through the maximum number of owners at the current size level).
    /// By default, `o4` is zero.
    pub o4: OwnerNum,

    /// The fraction of total ownership assigned to owner `o2`; must be positive.
    /// By default, `f4` is 1.0.
    pub f4: f64,

    // Second line: data for Converter 1 //
    /// Converter 1 bus number, or extended bus name enclosed in single quotes. No default.
    pub ibus1: BusNum,

    /// Code for the type of converter 1 DC control:
    /// * 0 for converter out-of-service,
    /// * 1 for DC voltage control,
    /// * 2 for MW control.
    /// When both converters are in-service, exactly one converter of each VSC DC line must be type 1.
    /// No default.
    pub type1: i8, // 0, 1, or 2

    /// Converter 1 AC control mode:
    /// * 1 for AC voltage control,
    /// * 2 for fixed AC power factor.
    /// `mode` = 1 by default.
    pub mode1: i8, // 1 or 2

    /// Converter 1 DC setpoint.
    /// * For `type` = 1, `dcset` is the scheduled DC voltage on the DC side of the converter bus;
    ///   entered in kV.
    /// * For `type` = 2, `dcset` is the power demand, where a positive value specifies that the
    ///   converter is feeding active power into the AC network at bus `ibus`, and a negative value
    ///   specifies that the converter is withdrawing active power from the AC network at bus `ibus`;
    ///   entered in MW.
    /// No default.
    pub docet1: f64,

    /// Converter 1 AC setpoint.
    /// * For `mode` = 1, `acset` is the regulated AC voltage setpoint; entered in pu.
    /// * For `mode` = 2, `acset` is the power factor setpoint.
    /// `acset` = 1.0 by default.
    pub acset1: f64,

    /// Coefficients of the linear equation used to calculate converter 1 losses:
    /// ``KW_{conv loss} = A_{loss} + I_{dc} * B_{loss}``
    /// `aloss` is entered in kW. `aloss` = `bloss` = 0.0 by default.
    pub aloss1: f64,

    /// Coefficients of the linear equation used to calculate converter 1 losses:
    /// ``KW_{conv loss} = A_{loss} + I_{dc} * B_{loss}``
    /// `bloss` is entered in kW/amp. `aloss` = `bloss` = 0.0 by default.
    pub bloss1: f64,

    /// Minimum converter 1 losses; entered in kW. `minloss` = 0.0 by default.
    pub minloss1: f64,

    /// Converter 1 MVA rating; entered in MVA.
    /// `smax` = 0.0 to allow unlimited converter MVA loading. `smax` = 0.0 by default.
    pub smax1: f64,

    /// Converter 1 AC current rating; entered in amps.
    /// `imax` = 0.0 to allow unlimited converter current loading.
    /// If a positive `imax` is specified, the base voltage assigned to bus `ibus` must be positive.
    /// `imax` = 0.0 by default.
    pub imax1: f64,

    /// Power weighting factor fraction (0.0 < `pwf` < 1.0) used in reducing the active power order
    /// and either the reactive power order (when `mode` is 2) or the reactive power limits (when `mode` is 1)
    /// when the converter MVA or current rating is violated.
    /// When `pwf` is 0.0, only the active power is reduced;
    /// when `PWF` is 1.0, only the reactive power is reduced;
    /// otherwise, a weighted reduction of both active and reactive power is applied.
    /// `pwf` = 1.0 by default.
    pub pwf1: f64,

    /// Reactive power upper limit; entered in Mvar.
    /// A positive value of reactive power indicates reactive power flowing into the AC network
    /// from the converter; a negative value of reactive power indicates reactive power withdrawn
    /// from the AC network. Not used if `mode` = 2.
    /// `maxq` = 9999.0 by default.
    pub maxq1: f64,

    /// Reactive power lower limit; entered in Mvar.
    /// A positive value of reactive power indicates reactive power flowing into the AC network
    /// from the converter; a negative value of reactive power indicates reactive power withdrawn
    /// from the AC network. Not used if `mode` = 2.
    /// `minq` = -9999.0 by default.
    pub minq1: f64,

    /// Bus number, or extended bus name enclosed in single quotes, of a remote type 1 or 2 bus
    /// whose voltage is to be regulated by this converter to the value specified by `acset`.
    /// If bus `remot` is other than a type 1 or 2 bus, bus `ibus` regulates its own voltage to
    /// the value specified by `acset`. `remot` is entered as zero if the converter is to regulate
    /// its own voltage. Not used if `mode` = 2.
    /// `remot` = 0 by default.
    pub remot1: BusNum,

    /// Percent of the total Mvar required to hold the voltage at the bus controlled by bus `ibus`
    /// that are to be contributed by this VSC; `rmpct` must be positive.
    /// `rmpct` is needed only if `remot` specifies a valid remote bus and there is more than one
    /// local or remote voltage controlling device (plant, switched shunt, FACTS device shunt element,
    /// or VSC DC line converter) controlling the voltage at bus `remot` to a setpoint, or `remot`
    /// is zero but bus `ibus` is the controlled bus, local or remote, of one or more other setpoint
    /// mode voltage controlling devices. Not used if `mode` = 2.
    /// `rmpct` = 100.0 by default.
    pub rmpct1: f64,

    // Third line: data for Converter 2 //
    /// Converter 2 bus number, or extended bus name enclosed in single quotes. No default.
    pub ibus2: BusNum,

    /// Code for the type of converter 2 DC control
    pub type2: i8, // 0, 1, or 2

    /// Converter 2 AC control mode
    pub mode2: i8, // 1 or 2

    /// Converter 2 DC setpoint.
    pub docet2: f64,

    /// Converter 2 AC setpoint.
    pub acset2: f64,

    /// Coefficient ``A_{loss}`` of the linear equation used to calculate converter 2 losses.
    pub aloss2: f64,

    /// Coefficient ``B_{loss}`` of the linear equation used to calculate converter 2 losses.
    pub bloss2: f64,

    /// Minimum converter 2 losses; entered in kW. `minloss` = 0.0 by default.
    pub minloss2: f64,

    /// Converter 2 MVA rating; entered in MVA.
    pub smax2: f64,

    /// Converter 2 AC current rating; entered in amps.
    pub imax2: f64,

    /// Power weighting factor fraction (0.0 < `pwf` < 1.0) for converter 2.
    pub pwf2: f64,

    /// Reactive power upper limit for converter 2; entered in Mvar.
    pub maxq2: f64,

    /// Reactive power lower limit for converter 2; entered in Mvar.
    pub minq2: f64,

    /// Bus number to be regulated by converter 2 to the value specified by `acset2`.
    pub remot2: BusNum,

    pub rmpct2: f64,
}

/// The SwitchedShunts data record depends on the PSSE version:
/// - See [SwitchedShunt30] for PSSE v30 files.
/// - See [SwitchedShunt33] for PSSE v33 files.
pub enum SwitchedShunt {
    SwitchedShunt30(SwitchedShunt30),
    SwitchedShunt33(SwitchedShunt33),
}

/// Represents switched shunt devices, in the form of capacitors and/or reactors on a network bus.
///
/// The switched shunt elements at a bus may consist entirely of blocks of shunt reactors
/// (each Bi is a negative quantity) or entirely of blocks of capacitor banks
/// (each Bi is a positive quantity). Any bus can have both switched capacitors and reactors.
///
/// Each network bus to be represented in PSS/E with switched shunt admittance devices must have
/// a switched shunt data record specified for it. The switched shunts are represented with up to
/// eight blocks of admittance, each one of which consists of up to nine steps of the specified
/// block admittance.
pub struct SwitchedShunt30 {
    /// Bus number, or extended bus name enclosed in single quotes.
    pub i: BusNum,

    /// Control mode:
    /// * 0 - fixed
    /// * 1 - discrete adjustment, controlling voltage locally or at bus `swrem`
    /// * 2 - continuous adjustment, controlling voltage locally or at bus `swrem`
    /// * 3 - discrete adjustment, controlling reactive power output of the plant at bus `swrem`
    /// * 4 - discrete adjustment, controlling reactive power output of the VSC DC line converter
    /// at bus `swrem` of the VSC DC line whose name is specified as `rmidnt`
    /// * 5 - discrete adjustment, controlling admittance setting of the switched shunt at bus `swrem`
    /// `modsw` = 1 by default.
    pub modsw: i8, // 0, 1, 2, 3, 4, or 5

    // $adjm_doc
    // $adjm_col
    // $stat_doc
    // $stat_col
    /// When `modsw` is 1 or 2, the controlled voltage upper limit; entered in pu.
    /// When `modsw` is 3, 4 or 5, the controlled reactive power range upper limit; entered in pu
    /// of the total reactive power range of the controlled voltage controlling device.
    /// `vswhi` is not used when `modsw` is 0.
    /// `vswhi` = 1.0 by default.
    pub vswhi: f64,

    /// When `modsw` is 1 or 2, the controlled voltage lower limit; entered in pu.
    /// When `modsw` is 3, 4 or 5, the controlled reactive power range lower limit; entered in pu
    /// of the total reactive power range of the controlled voltage controlling device.
    /// `vswlo` is not used when `modsw` is 0.
    /// `vswlo` = 1.0 by default.
    pub vswlo: f64,

    /// Bus number, or extended bus name enclosed in single quotes, of the bus whose voltage or
    /// connected equipment reactive power output is controlled by this switched shunt.
    /// * When `modsw` is 1 or 2, `swrem` is entered as 0 if the switched shunt is to regulate its own voltage;
    /// otherwise, `swrem` specifies the remote type one or two bus whose voltage is to be regulated by this switched shunt.
    /// * When `modsw` is 3, `swrem` specifies the type two or three bus whose plant reactive power output is to be regulated by this switched shunt.
    /// Set `swrem` to "I" if the switched shunt and the plant which it controls are connected to the same bus.
    /// * When `modsw` is 4, `swrem` specifies the converter bus of a VSC dc line whose converter reactive power output is to be regulated by this switched shunt.
    /// Set `swrem` to "I" if the switched shunt and the VSC dc line converter which it controls are connected to the same bus.
    /// * When `modsw` is 5, `swrem` specifies the remote bus to which the switched shunt whose admittance setting is to be regulated by this switched shunt is connected.
    /// * `swrem` is not used when `modsw` is 0.
    /// `swrem` = 0 by default.
    pub swrem: BusNum,

    /// Percent of the total Mvar required to hold the voltage at the bus controlled by bus `I`
    /// that are to be contributed by this switched shunt; `rmpct` must be positive.
    ///
    /// `rmpct` is needed only if `swrem` specifies a valid remote bus and there is more than one
    /// local or remote voltage controlling device (plant, switched shunt, FACTS device shunt element,
    /// or VSC DC line converter) controlling the voltage at bus `swrem` to a setpoint, or `swrem` is
    /// zero but bus I is the controlled bus, local or remote, of one or more other setpoint mode
    /// voltage controlling devices. Only used if `modsw` = 1 or 2.
    /// `rmpct` = 100.0 by default.
    pub rmpct: f64,

    /// When `modsw` is 4, the name of the VSC DC line whose converter bus is specified in `swrem`.
    /// `rmidnt` is not used for other values of `modsw`.
    /// `rmidnt` is a blank name by default.
    pub rmidnt: ArrayString<15>,

    /// Initial switched shunt admittance; entered in Mvar at unity voltage.
    /// `binit` = 0.0 by default.
    pub binit: f64,

    /// Number of steps for block i.
    /// The first zero value of N_i or B_i is interpreted as the end of the switched shunt blocks
    /// for bus I.
    /// `ni` = 0 by default.
    pub n1: i32,

    /// Admittance increment for each of N_i steps in block i; entered in Mvar at unity voltage.
    /// `bi` = 0.0 by default.
    pub b1: f64,
    pub n2: i32,
    pub b2: f64,
    pub n3: i32,
    pub b3: f64,
    pub n4: i32,
    pub b4: f64,
    pub n5: i32,
    pub b5: f64,
    pub n6: i32,
    pub b6: f64,
    pub n7: i32,
    pub b7: f64,
    pub n8: i32,
    pub b8: f64,
}

/// Represents switched shunt devices, in the form of capacitors and/or reactors on a network bus.
///
/// The switched shunt elements at a bus may consist entirely of blocks of shunt reactors
/// (each Bi is a negative quantity) or entirely of blocks of capacitor banks
/// (each Bi is a positive quantity). Any bus can have both switched capacitors and reactors.
///
/// Each network bus to be represented in PSS/E with switched shunt admittance devices must have
/// a switched shunt data record specified for it. The switched shunts are represented with up to
/// eight blocks of admittance, each one of which consists of up to nine steps of the specified
/// block admittance.
pub struct SwitchedShunt33 {
    /// Adjustment method:
    /// * 0 - steps and blocks are switched on in input order, and off in reverse input order;
    ///   this adjustment method was the only method available prior to PSS®E-32.0.
    /// * 1 - steps and blocks are switched on and off such that the next highest
    ///   (or lowest, as appropriate) total admittance is achieved.
    /// `adjm` = 0 by default.
    pub adjm: bool,

    /// Initial switched shunt status of one for in-service and zero for out-of-service.
    /// `stat` = 1 by default.
    pub stat: bool,
}

/// Transformer impedance corrections are used to model a change of transformer impedance
/// as off-nominal turns ratio or phase shift angle is adjusted.
///
/// The ``T_i`` values on a transformer impedance correction record must all be either tap
/// ratios or phase shift angles. They must be entered in strictly ascending order;
/// i.e. for each ``i``, ``T_{i+1} > T_i``. Each ``F_i`` entered must be greater than zero.
///
/// On each record, at least 2 pairs of values must be specified and up to 11 may be entered.
///
/// The ``T_i`` values that are a function of tap ratio (rather than phase shift angle)
/// are in units of the controlling winding’s off-nominal turns ratio in pu of the controlling
/// winding’s bus base voltage.
///
/// Although a transformer winding is assigned to an impedance correction record, each record may
/// be shared among many transformer windings. If the first ``T`` in a record is less than 0.5 or
/// the last ``T`` entered is greater than 1.5, ``T`` is assumed to be the phase shift angle and
/// the impedance of each transformer winding assigned to the record is treated as a function of
/// phase shift angle. Otherwise, the impedances of the transformer windings assigned to the record
/// are made sensitive to off-nominal turns ratio.
pub struct ImpedanceCorrection {
    /// Impedance correction record number.
    pub i: i16,

    /// Either off-nominal turns ratio in pu or phase shift angle in degrees.
    /// `ti` = 0.0 by default.
    pub t1: f64,

    /// Scaling factor by which transformer nominal impedance is to be multiplied to obtain the
    /// actual transformer impedance for the corresponding `ti`.
    /// `fi` = 0.0 by default.
    pub f1: f64,
    pub t2: f64,
    pub f2: f64,
    pub t3: f64,
    pub f3: f64,
    pub t4: f64,
    pub f4: f64,
    pub t5: f64,
    pub f5: f64,
    pub t6: f64,
    pub f6: f64,
    pub t7: f64,
    pub f7: f64,
    pub t8: f64,
    pub f8: f64,
    pub t9: f64,
    pub f9: f64,
    pub t10: f64,
    pub f10: f64,
    pub t11: f64,
    pub f11: f64,
}

// `MultiTerminalDCLines` records are a bit special...
// Each "record" is actually
// - a row of data about the line, followed by
// - arbitrary number of rows about the converters
// - arbitrary number of rows about the buses
// - arbitrary number of rows about the links
// So we treat `MultiTerminalDCLines` as a single column table, where each value is a
// dedicated `MultiTerminalDCLine` object. And each `MultiTerminalDCLine` is a bit like a
// `Network`, with a `DCLineID` (`CaseID`) and 3 `Records` (`ACConverters, `DCBuses`, `DCLinks`)

/// The DCLineID data record depends on the PSSE version:
/// - See [DCLineID30] for PSSE v30 files.
/// - See [DCLineID33] for PSSE v33 files.
pub enum DCLineID {
    DCLineID30(DCLineID30),
    DCLineID33(DCLineID33),
}

pub struct DCLineID30 {
    /// Multi-terminal DC line number.
    pub i: LineNum,

    /// Number of AC converter station buses in multi-terminal DC line `i`. No default.
    pub nconv: i8,

    /// Number of DC buses in multi-terminal DC line `i` (`nconv` < `ndcbs`). No default.
    pub ndcbs: i8,

    /// Number of DC links in multi-terminal DC line `i`. No default.
    pub ndcln: isize,

    /// Control mode
    /// * 0 - blocked
    /// * 1 - power
    /// * 2 - current
    /// `mdc` = 0 by default.
    pub mdc: i8, // 0, 1, or 2

    /// Bus number, or extended bus name enclosed in single quotes, of the AC converter station
    /// bus that controls DC voltage on the positive pole of multi-terminal DC line `i`.
    /// Bus `vconv` must be a positive pole inverter. No default.
    pub vconv: BusNum,

    /// Mode switch DC voltage; entered in kV.
    /// When any inverter DC voltage magnitude falls below this value and the line is in power
    /// control mode (i.e. `mdc` = 1), the line switches to current control mode with converter
    /// current setpoints corresponding to their desired powers at scheduled DC voltage.
    /// `vcmod` = 0.0 by default.
    pub vcmod: f64,

    /// Bus number, or extended bus name enclosed in single quotes, of the AC converter station
    /// bus that controls DC voltage on the negative pole of multi-terminal DC line `i`.
    /// If any negative pole converters are specified (see below), bus `vconvn` must be a
    /// negative pole inverter. If the negative pole is not being modeled, `vconvn` must be
    /// specified as zero. `vconvn` = 0 by default.
    pub vconvn: BusNum,
}

pub struct DCLineID33 {
    /// The non-blank alphanumeric identifier assigned to this DC line.
    /// Each multi-terminal DC line must have a unique `name.
    /// `name` may be up to twelve characters and may contain any combination of blanks, uppercase letters,
    /// numbers and special characters. name must be enclosed in single or double quotes if it contains any
    /// blanks or special characters. No default allowed.
    pub name: ArrayString<15>,
}

pub struct ACConverter {
    /// AC converter bus number, or extended bus name enclosed in single quotes.
    /// No default.
    pub ib: BusNum,

    /// Number of bridges in series. No default.
    pub n: i8,

    /// Nominal maximum ALPHA or GAMMA angle; entered in degrees. No default.
    pub angmx: f64,

    /// Minimum steady-state ALPHA or GAMMA angle; entered in degrees. No default.
    pub angmn: f64,

    /// Commutating resistance per bridge; entered in ohms. No default.
    pub rc: f64,

    /// Commutating reactance per bridge; entered in ohms. No default.
    pub xc: f64,

    /// Primary base AC voltage; entered in kV. No default.
    pub ebas: f64,

    /// Actual transformer ratio. `tr` = 1.0 by default.
    pub tr: f64,

    /// Tap setting. `tap` = 1.0 by default.
    pub tap: f64,

    /// Maximum tap setting. `tpmx` = 1.5 by default.
    pub tpmx: f64,

    /// Minimum tap setting. `tpmx` = 0.51 by default.
    pub tpmn: f64,

    /// Tap step; must be a positive number. `tstp` = 0.00625 by default.
    pub tstp: f64,

    /// Converter setpoint.
    /// When `ib` is equal to `vconv` or `vconvn`, `setvl` specifies the scheduled DC voltage magnitude,
    /// entered in kV, across the converter.
    /// For other converter buses, `setvl` contains the converter current (amps) or power (MW) demand;
    /// a positive value of `setvl` indicates that bus `ib` is a rectifier, and a negative value indicates an inverter.
    /// No default.
    pub setvl: f64,

    /// Converter participation factor.
    /// When the order at any rectifier in the multi-terminal DC line is reduced,
    /// either to maximum current or margin, the orders at the remaining converters on the same
    /// pole are modified according to their ``DCPF``s to: ``SETVL + (DCPF/SUM)∗R```
    /// where ``SUM`` is the sum of the ``DCPF``s at the unconstrained converte rs on the same
    /// pole as the constrained rectifier, and ``R`` is the order reduction at the constrained rectifier.
    /// `dcpf` = 1. by default.
    pub dcpf: f64,

    /// Rectifier margin entered in per unit of desired DC power or current.
    /// The converter order reduced by this fraction, ``(1.0 - MARG) ∗ SETVL``,
    /// defines the minimum order for this rectifier. `marg` is used only at rectifiers.
    /// `marg` = 0.0 by default.
    pub marg: f64,

    /// Converter code.
    /// A positive value or zero must be entered if the converter is on the positive pole of multi-terminal DC line `i`.
    /// A negative value must be entered for negative pole converters. `cnvcod` = 1 by default.
    pub cnvcod: i8,
}

pub struct DCBus {
    /// DC bus number (1 to `NDCBS`).
    /// The DC buses are used internally within each multi-terminal DC line and must be numbered
    /// 1 through `ndcbs`. no default.
    pub idc: BusNum,

    /// AC converter bus number, or extended bus name enclosed in single quotes, or zero.
    /// Each converter station bus specified in a converter record must be specified as `ib` in
    /// exactly one DC bus record. DC buses that are connected only to other DC buses by DC links
    /// and not to any AC converter buses must have a zero specified for `ib`. A DC bus specified
    /// as `idc2` on one or more other DC bus records must have a zero specified for `ib` on its
    /// own DC bus record. `ib` = 0 by default.
    pub ib: BusNum,

    /// Area number (1 through the maximum number of areas at the current size level).
    /// `ia` = 1 by default.
    pub ia: AreaNum,

    /// Zone number (1 through the maximum number of zones at the current size level).
    /// `zone` = 1 by default.
    pub zone: ZoneNum,

    /// Alphanumeric identifier assigned to DC bus `idc`.
    /// The name may be up to twelve characters and must be enclosed in single quotes. `name`
    /// may contain any combination of blanks, uppercase letters, numbers, and special characters.
    /// `name` is twelve blanks by default.
    pub name: ArrayString<15>,

    pub idc2: BusNum,

    /// Second DC bus to which converter `ib` is connected, or zero if the converter is connected directly to ground.
    /// * For voltage controlling converters, this is the DC bus with the lower DC voltage magnitude
    ///   and `setvl` specifies the voltage difference between buses `idc` and `idc2`.
    /// * For rectifiers, DC buses should be specified such that power flows from bus `idc2` to bus `idc`.
    /// * For inverters, DC buses should be specified such that power flows from bus `idc` to bus `idc2`.
    /// `idc2` is ignored on those dc bus records that have `ib` specified as zero. `idc2` = 0 by default.
    pub rgrnd: f64,

    /// Owner number (1 through the maximum number of owners at the current size level).
    /// `owner` = 1 by default.
    pub owner: OwnerNum,
}

pub struct DCLink {
    /// Branch "from bus" DC bus number.
    pub idc: BusNum,

    /// Branch "to bus" DC bus number.
    /// `jdc` is entered as a negative number to designate it as the metered end for area and
    /// zone interchange calculations. Otherwise, bus `idc` is assumed to be the metered end.
    pub jdc: BusNum,

    /// One-character uppercase alphanumeric branch circuit identifier.
    /// It is recommended that single circuit branches be designated as having the circuit identifier "1".
    /// `dcckt` = "1" by default.
    pub dcckt: ArrayString<1>,

    /// DC link resistance, entered in ohms. No default.
    pub rdc: f64,

    /// DC link inductance, entered in mH. `ldc` is not used by the power flow solution activities
    /// but is available to multi-terminal DC line dynamics models. `ldc` = 0.0 by default.
    pub ldc: f64,
}

// PSS/E allows the representation of up to 12 converter stations on one multi-terminal DC line.
// Further, it allows the modelling of multi-terminal networks of up to 20 buses including the
// AC converter buses and the DC network buses.
//
// ## Notes
//
// The following are notes on multi-terminal links:
// * Conventional two-terminal and multi-terminal DC lines are stored separately.
//   Therefore, there may simultaneously exist, for example, a two-terminal DC line identified
//   as DC line number 1 along with a multi-terminal line numbered 1.
// * Multi-terminal lines should have at least three converter terminals;
//   conventional DC lines consisting of two terminals should be modeled as two-terminal lines
//   (see [TwoTerminalDCLine].
// * AC converter buses may be type one, two, or three buses. Generators, loads, fixed and
//   switched shunt elements, other DC line converters, and FACTS device sending ends are
//   permitted at converter buses.
// * Each multi-terminal DC line is treated as a subnetwork of DC buses and DC links connecting
//   its AC converter buses. For each multi-terminal DC line, the DC buses must be numbered 1
//   through `ndcbs`.
// * Each AC converter bus must be specified as `ib` on exactly one DC bus record; there may be
//   DC buses connected only to other DC buses by DC links but not to any AC converter bus.
// * AC converter bus `ib` may be connected to a DC bus `idc`, which is connected directly to ground.
//   `ib` is specified on the DC bus record for DC bus `idc`; the `idc2` field is specified as zero.
// * Alternatively, AC converter bus `ib` may be connected to two DC buses `idc` and `idc2`,
//   the second of which is connected to ground through a specified resistance.
//   `ib` and `idc2` are specified on the DC bus record for DC bus `idc`;
//   on the DC bus record for bus `idc2`, the AC converter bus and second DC bus fields
//   (`ib` and `idc2`, respectively) must be specified as zero and the grounding resistance is
//   specified as `rgrnd`.
// * The same DC bus may be specified as the second DC bus for more than one AC converter bus.
// * All DC buses within a multi-terminal DC line must be reachable from any other point within the subnetwork.
// * The area number assigned to DC buses and the metered end designation of DC links are used in
//   calculating area interchange and assigning losses as well as in the interchange control option
//   of the power flow solution activities. Similarly, the zone assignment and metered end specification
//   are used in Zonal reporting activities.

/// Each multi-terminal DC line record defines the number of converters, number of DC buses and
/// number of DC links as well as related bus numbers and control mode (see [DCLineID]),
/// then data for:
/// * each converter (see [ACConverter])
/// * each DC bus (see [DCBus])
/// * each DC link (see [DCLink])
pub struct MultiTerminalDCLine {
    /// High-level data about this line.
    pub line_id: DCLineID,
    /// `line.nconv` converter records.
    pub converter: ACConverter,
    /// `line.ndcbs` DC bus records.
    pub bus: DCBus,
    /// `line.ndcln` DC link records.
    pub link: DCLink,
}

/// The MultiSectionLineGroups data record depends on the PSSE version:
/// - See [MultiSectionLineGroup30] for PSSE v30 files.
/// - See [MultiSectionLineGroup33] for PSSE v33 files.
pub enum MultiSectionLineGroup {
    MultiSectionLineGroup30(MultiSectionLineGroup30),
    MultiSectionLineGroup33(MultiSectionLineGroup33),
}

/// Multi-section line group.
///
/// Transmission lines commonly have a series of sections with varying physical structures.
/// The section might have different tower configurations, conductor types and bundles or various
/// combinations of these. The physical differences can result in the sections having different
/// resistance, reactance and charging.
///
/// A transmission line with several distinct sections can be represented as one
/// multi-section line group.
///
/// The DUM_i values on each record define the branches connecting bus `i` to bus `j`, and
/// are entered so as to trace the path from bus `i` to bus `j`.
///
/// ## Example
///
/// For a multi-section line grouping consisting of three line sections (and hence two dummy buses):
///
/// | From | To | Circuit |
/// |------|----|---------|
/// |    I | D1 |      C1 |
/// |   D1 | D2 |      C2 |
/// |   D2 |  J |      C3 |
///
/// If this multi-section line grouping is to be assigned the line identifier `id` "&1",
/// the corresponding multi-section line grouping data record is given by:
/// ```txt
/// I, J, '&1', D1, D2
/// ```
/// Or in v33 (and if I is the metered end):
/// ```txt
/// I, J, '&1', 1, D1, D2
/// ```
///
/// ## Notes
///
/// The following notes apply to multi-section line groups:
/// * Up to 10 line sections (and hence 9 dummy buses) may be defined in each multi-section line grouping.
/// A branch may be a line section of at most one multi-section line grouping.
/// * Each dummy bus must have exactly two branches connected to it,
/// both of which must be members of the same multi-section line grouping.
/// A multi-section line dummy bus may not be a converter bus of a DC transmission line.
/// A FACTS control device may not be connected to a multi-section line dummy bus.
/// * The status of line sections and type codes of dummy buses are set such that the multi-section
/// line is treated as a single entity with regards to its service status.
pub struct MultiSectionLineGroup30 {
    /// "From bus" number, or extended bus name enclosed in single quotes.
    pub i: BusNum,

    /// "To bus" number, or extended bus name enclosed in single quotes.
    /// `j` is entered as a negative number or with a minus sign before the first character of
    /// the extended bus name to designate it as the metered end;
    /// otherwise, bus `i` is assumed to be the metered end.
    pub j: BusNum,

    /// Two-character upper-case alphanumeric multi-section line grouping identifier.
    /// The first character must be an ampersand ("&").
    /// `id` = "&1" by default.
    pub id: ArrayString<3>,

    // $met_doc
    // $met_col
    /// Bus numbers, or extended bus names enclosed in single quotes, of the dummy buses
    /// connected by the branches that comprise this multi-section line grouping.
    /// No defaults.
    pub dum1: BusNum,
    pub dum2: Option<BusNum>,
    pub dum3: Option<BusNum>,
    pub dum4: Option<BusNum>,
    pub dum5: Option<BusNum>,
    pub dum6: Option<BusNum>,
    pub dum7: Option<BusNum>,
    pub dum8: Option<BusNum>,
    pub dum9: Option<BusNum>,
}

pub struct MultiSectionLineGroup33 {
    /// Metered end flag.
    /// * ≤1 to designate bus `i` as the metered end.
    /// * ≥2 to designate bus `j` as the metered end.
    /// `met` = 1 by default.
    pub met: i8,
}

/// All buses (AC and DC) and loads can be assigned to reside in a zone of the network.
/// To enable this facility, each zone should be assigned a name and number.
/// Specifically, the zone number is entered as part of the data records for the [buses](Bus) and [loads](Load).
/// The use of zones enables the user to develop reports and to check results on the basis of zones and,
/// consequently be highly specific when reporting and interpreting analytical results.
pub struct Zone {
    /// Zone number (1 through the maximum number of zones at the current size level)
    pub i: ZoneNum,

    /// Alphanumeric identifier assigned to zone `i`.
    /// The name may contain up to twelve characters and must be enclosed in single quotes.
    /// `zoname` may be any combination of blanks, uppercase letters, numbers, and special characters.
    /// `zoname` is set to twelve blanks by default.
    pub zoname: ArrayString<15>,
}

/// Using PSS/E, the user has the capability to identify in which area each [bus](Bus) or [load](Load) resides.
/// Further, the user can schedule active power transfers between pairs of areas.
///
/// See [AreaInterchange] for desired net interchange.
pub struct InterAreaTransfer {
    /// "From area" number (1 through the maximum number of areas at the current size level).
    pub arfrom: AreaNum,

    /// "To area" number (1 through the maximum number of areas at the current size level).
    pub arto: AreaNum,

    /// Single-character (0 through 9 or A through Z) upper-case interarea transfer identifier
    /// used to distinguish among multiple transfers between areas `arfrom` and `arto`.
    /// `trid` = "1" by default.
    pub trid: ArrayString<1>,

    /// MW comprising this transfer.
    /// A positive `ptran` indicates that area `arfrom` is selling to area `arto`.
    /// `ptran` = 0.0 by default.
    pub ptran: f64,
}

/// PSS/E allows the user to identify which organization or utility actually owns a facility,
/// a piece of equipment, or a load. Major network elements can have up to four different owners.
/// This facilitates interpretation of results and reporting of results on the basis of ownership.
pub struct Owner {
    /// Owner number (1 through the maximum number of owners at the current size level).
    pub i: OwnerNum,

    /// Alphanumeric identifier assigned to owner `i`.
    /// The name may contain up to twelve characters and must be enclosed in single quotes.
    /// `owname` may be any combination of blanks, uppercase letters, numbers, and special characters.
    /// `owname` is set to twelve blanks by default.
    pub owname: ArrayString<15>,
}

pub enum FACTSDevice {
    FACTSDevice30(FACTSDevice30),
    FACTSDevice33(FACTSDevice33),
}

/// Flexible AC Transmission System devices.
///
/// There is a multiplicity of Flexible AC Transmission System devices currently available
/// comprising shunt devices, such as the Static Compensator (STATCOM), series devices such as
/// the Static Synchronous Series Compensator (SSSC), combined devices such as the
/// Unified Power Flow Controller (UPFC) and the Interline Power Flow Controllers (IPFC),
/// of which the latter are parallel series devices.
pub struct FACTSDevice30 {
    /// FACTS device number.
    pub n: i16,

    /// Sending end bus number, or extended bus name enclosed in single quotes.
    /// No default.
    pub i: BusNum,

    /// Terminal end bus number, or extended bus name enclosed in single quotes.
    /// 0 for a STATCON.
    /// `j` = 0 by default.
    pub j: BusNum,

    /// Control mode:
    /// * 0 - out-of-service (i.e., series and shunt links open).
    /// * 1 - series and shunt links operating.
    /// * 2 - series link bypassed (i.e., like a zero impedance line) and shunt link operating as a STATCON.
    /// * 3 - series and shunt links operating with series link at constant series impedance.
    /// * 4 - series and shunt links operating with series link at constant series voltage.
    /// * 5 - master device of an IPFC with P and Q setpoints specified;
    ///     FACTS device N+1 must be the slave device (i.e., its `mode` is 6 or 8) of this IPFC.
    /// * 6 - slave device of an IPFC with P and Q setpoints specified;
    ///     FACTS device N-1 must be the master device (i.e., its `mode` is 5 or 7) of this IPFC.
    ///     The Q setpoint is ignored as the master device dictates the active power exchanged between the two devices.
    /// * 7 - master device of an IPFC with constant series voltage setpoints specified;
    ///     FACTS device N+1 must be the slave device (i.e., its `mode` is 6 or 8) of this IPFC.
    /// * 8 - slave device of an IPFC with constant series voltage setpoints specified;
    ///     FACTS device N-1 must be the master device (i.e., its `mode` is 5 or 7) of this IPFC.
    ///     The complex ``V_d + j V_q`` setpoint is modified during power flow solutions to reflect
    ///     the active power exchange determined by the master device.
    /// If `j` is specified as 0, `mode` must be either 0 or 1.
    /// `mode` = 1 by default.
    pub mode: i8,

    /// Desired active power flow arriving at the terminal end bus; entered in MW.
    /// `pdes` = 0.0 by default.
    pub pdes: f64,

    /// Desired reactive power flow arriving at the terminal end bus; entered in MVAR.
    /// `qdes` = 0.0 by default.
    pub qdes: f64,

    /// Voltage setpoint at the sending end bus; entered in pu. `vset` = 1.0 by default.
    pub vset: f64,

    /// Maximum shunt current at the sending end bus; entered in MVA at unity voltage.
    /// `shmx` = 9999.0 by default.
    pub shmx: f64,

    /// Maximum bridge active power transfer; entered in MW. `trmx` = 9999.0 by default.
    pub trmx: f64,

    /// Minimum voltage at the terminal end bus; entered in pu. `vtmn` = 0.9 by default.
    pub vtmn: f64,

    /// Maximum voltage at the terminal end bus; entered in pu. `vtmx` = 1.1 by default.
    pub vtmx: f64,

    /// Maximum series voltage; entered in pu. `vsmx` = 1.0 by default.
    pub vsmx: f64,

    /// Maximum series current, or zero for no series current limit; entered in MVA at unity voltage.
    /// `imx` = 0.0 by default.
    pub imx: f64,

    /// Reactance of the dummy series element used during model solution; entered in pu.
    /// `linx` = 0.05 by default.
    pub linx: f64,

    /// Percent of the total Mvar required to hold the voltage at bus `i` that are to be contributed
    /// by the shunt element of this FACTS device; `rmpct` must be positive.
    /// `rmpct` is needed only if there is more than one local or remote voltage controlling device
    /// (plant, switched shunt, FACTS device shunt element, or VSC dc line converter) controlling
    /// the voltage at bus `i` to a setpoint. `rmpct` = 100.0 by default.
    pub rmpct: f64,

    /// Owner number (1 through the maximum number of owners at the current size level).
    /// `owner` = 1 by default.
    pub owner: OwnerNum,

    /// If `mode` is 3, resistance and reactance respectively of the constant impedance, entered in pu;
    /// if `mode` is 4, the magnitude (in pu) and angle (in degrees) of the constant series voltage
    /// with respect to the quantity indicated by `vsref`;
    /// if `mode` is 7 or 8, the real (vd) and imaginary (vq) components (in pu) of the constant
    /// series voltage with respect to the quantity indicated by `vsref`;
    /// for other values of `mode`, `set1` and set2 are read, but not saved or used during power flow solutions.
    /// `set1` = 0.0 by default.
    pub set1: f64,

    /// See `set1`. `set2` = 0.0 by default.
    pub set2: f64,

    /// Series voltage reference code to indicate the series voltage reference of `set1` and `set2`
    /// when `mode` is 4, 7 or 8: 0 for sending end voltage, 1 for series current.
    /// `vsref` = 0 by default.
    pub vsref: i8, // 0 or 1... i think
}

pub struct FACTSDevice33 {
    /// The non-blank alphanumeric identifier assigned to this FACTS device.
    /// Each FACTS device must have a unique `name.
    /// `name` may be up to twelve characters and may contain any combination of blanks, uppercase letters,
    /// numbers and special characters. name must be enclosed in single or double quotes if it contains any
    /// blanks or special characters. No default allowed.
    pub name: ArrayString<15>,

    /// Bus number, or extended bus name enclosed in single quotes, of a remote Type 1 or 2 bus
    /// where voltage is to be regulated by the shunt element of this FACTS device to the value
    /// specified by `vset`. if bus `remot` is other than a type 1 or 2 bus, the shunt element
    /// regulates voltage at the sending end bus to the value specified by `vset`.
    /// `remot` is entered as zero if the shunt element is to regulate voltage at the sending end
    /// bus and must be zero if the sending end bus is a type 3 (swing) bus. `remot` = 0 by default.
    pub remot: BusNum,

    /// The name of the FACTS device that is the IPFC master device when this FACTS device is
    /// the "slave" device of an IPFC (i.e., its `mode` is specified as 6 or 8).
    /// `mname` must be enclosed in single or double quotes if it contains any blanks or special
    /// characters. `mname` is blank by default.
    pub mname: ArrayString<15>,
}

/// Representation of a power network.
///
/// The PSS/E data format comprises 16 data categories of network and equipment
/// elements, each of which requires a particular type of data.
///
/// Similarly, a `Network` stores the data from each category in its own dedicated structure.
///
/// Currently supported are:
/// 1. [CaseID]
/// 1. [Bus]
/// 1. [Load]
/// 1. [FixedShunt]
/// 1. [Generator]
/// 1. [Branch]
/// 1. [Transformer]
/// 1. [AreaInterchange]
/// 1. [TwoTerminalDCLine]
/// 1. [VSCDCLine]
/// 1. [SwitchedShunt]
/// 1. [ImpedanceCorrection]
/// 1. [MultiTerminalDCLine]
/// 1. [MultiSectionLineGroup]
/// 1. [Zone]
/// 1. [InterAreaTransfer]
/// 1. [Owner]
/// 1. [FACTSDevice]
pub struct Network {
    /// Version of the PSS/E data version given or detected when parsing.
    pub version: i8,

    /// Case identification data.
    pub caseid: CaseID,

    /// Bus records.
    pub buses: Vec<Bus>, // v30/v33

    /// Load records.
    pub loads: Vec<Load>,

    /// Fixed Bus Shunt records.
    pub fixed_shunts: Option<Vec<FixedShunt>>, // v33 only

    /// Generator records.
    pub generators: Vec<Generator>,

    /// Non-transformer Branch records.
    pub branches: Vec<Branch>, // v30/v33

    /// Transformer records.
    pub transformers: Vec<Transformer>,

    /// Area Interchange records.
    pub area_interchanges: Vec<AreaInterchange>,

    /// Two-terminal DC Line records.
    pub two_terminal_dc: Vec<TwoTerminalDCLine>, // v30/v33

    /// Voltage Source Converter DC Line records.
    pub vsc_dc: Vec<VSCDCLine>,

    /// Switched Shunt records.
    pub switched_shunts: Vec<SwitchedShunt>, // v30/v33

    /// Transformer impedance correction records.
    pub impedance_corrections: Vec<ImpedanceCorrection>,

    /// Multi-terminal DC Line records.
    pub multi_terminal_dc: Vec<MultiTerminalDCLine>,

    /// Multi-section line group records.
    pub multi_section_lines: Vec<MultiSectionLineGroup>, // v30/v33

    /// Zone records.
    pub zones: Vec<Zone>,

    /// Inter-area transfer records.
    pub area_transfers: Vec<InterAreaTransfer>,

    /// Owner records.
    pub owners: Vec<Owner>,

    /// FACTS device records.
    pub facts: Vec<FACTSDevice>, // v30/v33
}
