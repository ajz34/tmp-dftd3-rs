use crate::prelude::*;
use std::ffi::{c_char, c_double, c_int};

pub unsafe fn calc_dftd3_atm_rest_(
    num: *const c_int,
    num_size: *const c_int,
    xyz: *const c_double,
    charge: *const c_double,
    uhf: *const c_int,
    method: *const c_char,
    method_len: *const c_int,
    energy: *mut c_double,
    gradient: *mut c_double,
    sigma: *mut c_double,
    corr: *const c_char,
    corr_len: *const c_int,
) {
    let _ = charge;
    let _ = uhf;

    // convert c-style arguments to rust-style arguments
    let natoms = unsafe { *num_size } as usize;
    let charges = {
        let charges = unsafe { std::slice::from_raw_parts(num, natoms) };
        charges.iter().map(|&x| x as usize).collect::<Vec<usize>>()
    };
    let coords = {
        let coords = unsafe { std::slice::from_raw_parts(xyz, natoms * 3) };
        coords.iter().map(|&x| x).collect::<Vec<f64>>() // this may not required, since c_double is always f64
    };
    let method = {
        let method = unsafe { std::slice::from_raw_parts(method, *method_len as usize) };
        let method = method.iter().map(|&x| x as u8).collect::<Vec<u8>>();
        std::str::from_utf8(&method).unwrap().to_string()
    };
    let corr = {
        let corr = unsafe { std::slice::from_raw_parts(corr, *corr_len as usize) };
        let corr = corr.iter().map(|&x| x as u8).collect::<Vec<u8>>();
        std::str::from_utf8(&corr).unwrap().to_string()
    };

    // create structure and model
    let structure = DFTD3Structure::new(natoms, &charges, &coords, None, None);
    let model = DFTD3Model::new(&structure);

    // get dispersion energy and gradient
    let result = match corr.as_str() {
        "zero" | "d3zero" | "d3(zero)" | "d3" => {
            let param = DFTD3Param::load_zero_damping(&method, true);
            get_dispersion(&structure, &model, &param, true, true)
        }
        "bj" | "d3bj" | "d3(bj)" | "rational" => {
            let param = DFTD3Param::load_rational_damping(&method, true);
            get_dispersion(&structure, &model, &param, true, true)
        }
        _ => panic!("Unknown damping type"),
    };

    // set energy and gradient
    unsafe {
        *energy = result.0;
        let gradient = std::slice::from_raw_parts_mut(gradient, natoms * 3);
        let sigma = std::slice::from_raw_parts_mut(sigma, 3 * 3);
        gradient.copy_from_slice(&result.1.unwrap());
        sigma.copy_from_slice(&result.2.unwrap());
    }
}
