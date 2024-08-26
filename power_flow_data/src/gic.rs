use crate::BusNum;

pub struct Substation {
    pub i: i32,
    pub name: String,
    pub _n: i32,
    /// degrees
    pub lat: f64,
    /// degrees
    pub lng: f64,
    /// Ohms
    pub rg: f64,
}

pub struct BusSubstation {
    pub i: i32,
    pub sub: i32,
}

pub struct Transformer {
    pub i: i32,
    pub j: i32,
    pub k: i32,
    pub ckt: String,
    /// Ohm/ph
    pub wri: f64,
    /// Ohm/ph
    pub wrj: f64,
    /// Ohm/ph
    pub wrk: f64,

    pub gicbdi: i32,
    pub gicbdj: i32,
    pub gicbdk: i32,
    pub vecgrp: String,
    pub core: i32,
    pub kfactor: f64,
    /// Ohm
    pub grdri: f64,
    /// Ohm
    pub grdrj: f64,
    /// Ohm
    pub grdrk: f64,
    pub tmodel: i32,
}

pub struct FixedShunt {
    pub i: BusNum,
    pub id: String,
    /// Ohm/ph
    pub rfxsh: f64,
    /// Ohm
    pub rgrdfxsh: f64,
}

pub struct Branch {
    pub busi: BusNum,
    pub busj: BusNum,
    pub ckt: String,
    /// Ohm/ph
    pub rbrn: f64,
    /// Volts
    pub indvp: f64,
    /// Volts
    pub indvq: f64,
}

pub struct EarthModel {
    pub name: String,
    /// Beta factor.
    pub b: f64,
    pub desc: String,
    /// Resistivity (ohm.m)
    pub r: Vec<f64>,
    /// Thickness (km).
    pub t: Vec<f64>,
}

pub struct GIC {
    substations: Vec<Substation>,
    bus_substations: Vec<BusSubstation>,
    transformers: Vec<Transformer>,
    fixed_shunts: Vec<FixedShunt>,
    branches: Vec<Branch>,
    earth_models: Vec<EarthModel>,
}
