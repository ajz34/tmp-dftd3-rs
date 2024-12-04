use crate::ffi;
use std::ffi::{c_char, c_int, CStr};
use std::result::Result;

/// Get the version of the DFTD3 library.
pub fn get_api_version() -> String {
    let version = unsafe { ffi::dftd3_get_version() };
    format!(
        "{}.{}.{}",
        version / 10000,
        version / 100 % 100,
        version % 100
    )
}

/// Get the version of the DFTD3 library in list of integers.
pub fn get_api_version_compact() -> [usize; 3] {
    let version = unsafe { ffi::dftd3_get_version() } as usize;
    [version / 10000, version / 100 % 100, version % 100]
}

pub enum DFTD3Error {
    C(ffi::dftd3_error),
    Rust(String),
}

impl Drop for DFTD3Error {
    fn drop(&mut self) {
        match self {
            DFTD3Error::C(ptr) => unsafe { ffi::dftd3_delete_error(&mut ptr.clone()) },
            DFTD3Error::Rust(_) => (),
        }
    }
}

impl DFTD3Error {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::dftd3_new_error() };
        DFTD3Error::C(ptr)
    }

    pub fn check(&self) -> bool {
        match self {
            DFTD3Error::C(ptr) => unsafe { ffi::dftd3_check_error(*ptr) != 0 },
            DFTD3Error::Rust(_) => true,
        }
    }

    pub fn get_c_ptr(&mut self) -> ffi::dftd3_error {
        match self {
            DFTD3Error::C(ptr) => *ptr,
            DFTD3Error::Rust(_) => std::ptr::null_mut(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            DFTD3Error::C(ptr) => {
                const LEN_BUFFER: usize = 512;
                let buffer = [0u8; LEN_BUFFER];
                let raw = buffer.as_ptr() as *mut c_char;
                let msg = unsafe {
                    ffi::dftd3_get_error(*ptr, raw, &(LEN_BUFFER as c_int));
                    CStr::from_ptr(raw)
                };
                return msg.to_string_lossy().to_string();
            }
            DFTD3Error::Rust(msg) => msg.clone(),
        }
    }
}

impl std::fmt::Debug for DFTD3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.check() {
            write!(f, "DFTD3Error: {}", self.get_message())
        } else {
            write!(f, "DFTD3Error: No error")
        }
    }
}

impl std::fmt::Display for DFTD3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.check() {
            write!(f, "DFTD3Error: {}", self.get_message())
        } else {
            write!(f, "")
        }
    }
}

impl std::error::Error for DFTD3Error {}

pub struct DFTD3Structure {
    ptr: ffi::dftd3_structure,
    natoms: usize,
}

impl Drop for DFTD3Structure {
    fn drop(&mut self) {
        unsafe { ffi::dftd3_delete_structure(&mut self.ptr) };
    }
}

impl DFTD3Structure {
    /// Get number of atoms
    pub fn get_natoms(&self) -> usize {
        self.natoms
    }

    /// Create new molecular structure data (quantities in Bohr) (failable)
    pub fn new_f(
        natoms: usize,
        numbers: &[usize],
        positions: &[f64],
        lattice: &[f64],
        periodic: &[bool],
    ) -> Result<Self, DFTD3Error> {
        // check dimension
        if numbers.len() != natoms {
            return Err(DFTD3Error::Rust(format!(
                "Invalid dimension for numbers, expected {}, got {}",
                natoms,
                numbers.len()
            )));
        }
        if positions.len() != 3 * natoms {
            return Err(DFTD3Error::Rust(format!(
                "Invalid dimension for positions, expected {}, got {}",
                3 * natoms,
                positions.len()
            )));
        }
        if lattice.len() != 9 {
            return Err(DFTD3Error::Rust(format!(
                "Invalid dimension for lattice, expected 9, got {}",
                lattice.len()
            )));
        }
        // type conversion from usual definitions
        let natoms_c_int = natoms as c_int;
        let atomic_numbers = numbers.iter().map(|&x| x as c_int).collect::<Vec<c_int>>();
        // actual driver for creating the structure
        let mut error = DFTD3Error::new();
        let ptr = unsafe {
            ffi::dftd3_new_structure(
                error.get_c_ptr(),
                natoms_c_int,
                atomic_numbers.as_ptr(),
                positions.as_ptr(),
                lattice.as_ptr(),
                periodic.as_ptr(),
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr, natoms }),
        }
    }

    /// Create new molecular structure data (quantities in Bohr)
    ///
    /// # Arguments
    ///
    /// * `numbers` - numbers [natoms]
    /// * `positions` - positions [natoms][3]
    /// * `lattice` - lattice [3][3]
    /// * `periodic` - periodic [3]
    pub fn new(
        natoms: usize,
        numbers: &[usize],
        positions: &[f64],
        lattice: &[f64],
        periodic: &[bool],
    ) -> Self {
        Self::new_f(natoms, numbers, positions, lattice, periodic).unwrap()
    }

    /// Update coordinates and lattice parameters (quantities in Bohr) (failable)
    pub fn update_f(&self, positions: &[f64], lattice: &[f64]) -> Result<(), DFTD3Error> {
        // check dimension
        if positions.len() != 3 * self.natoms {
            return Err(DFTD3Error::Rust(format!(
                "Invalid dimension for positions, expected {}, got {}",
                3 * self.natoms,
                positions.len()
            )));
        }
        if lattice.len() != 9 {
            return Err(DFTD3Error::Rust(format!(
                "Invalid dimension for lattice, expected 9, got {}",
                lattice.len()
            )));
        }
        // actual driver for updating the structure
        let mut error = DFTD3Error::new();
        unsafe {
            ffi::dftd3_update_structure(
                error.get_c_ptr(),
                self.ptr,
                positions.as_ptr(),
                lattice.as_ptr(),
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(()),
        }
    }

    /// Update coordinates and lattice parameters (quantities in Bohr)
    ///
    /// # Arguments
    ///
    /// * `positions` - positions [natoms][3]
    /// * `lattice` - lattice [3][3]
    pub fn update(&self, positions: &[f64], lattice: &[f64]) {
        self.update_f(positions, lattice).unwrap()
    }
}

pub struct DFTD3Model {
    ptr: ffi::dftd3_model,
}

impl Drop for DFTD3Model {
    fn drop(&mut self) {
        unsafe { ffi::dftd3_delete_model(&mut self.ptr) };
    }
}

impl DFTD3Model {
    /// Create new D3 dispersion model (failable)
    pub fn new_f(structure: &DFTD3Structure) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let ptr = unsafe { ffi::dftd3_new_d3_model(error.get_c_ptr(), structure.ptr) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new D3 dispersion model
    pub fn new(structure: &DFTD3Structure) -> Self {
        Self::new_f(structure).unwrap()
    }

    /// Set realspace cutoffs (quantities in Bohr) (failable)
    pub fn set_realspace_cutoff_f(
        &self,
        disp2: f64,
        disp3: f64,
        cn: f64,
    ) -> Result<(), DFTD3Error> {
        let mut error = DFTD3Error::new();
        unsafe {
            ffi::dftd3_set_model_realspace_cutoff(error.get_c_ptr(), self.ptr, disp2, disp3, cn)
        };
        match error.check() {
            true => Err(error),
            false => Ok(()),
        }
    }

    /// Set realspace cutoffs (quantities in Bohr)
    pub fn set_realspace_cutoff(&self, r0: f64, r1: f64, r2: f64) {
        self.set_realspace_cutoff_f(r0, r1, r2).unwrap()
    }
}

pub struct DFTD3Param {
    ptr: ffi::dftd3_param,
}

impl Drop for DFTD3Param {
    fn drop(&mut self) {
        unsafe { ffi::dftd3_delete_param(&mut self.ptr) };
    }
}

impl DFTD3Param {
    /// Create new zero damping parameters (failable)
    pub fn new_zero_damping_f(
        s6: f64,
        s8: f64,
        s9: f64,
        rs6: f64,
        rs8: f64,
        alp: f64,
    ) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let ptr =
            unsafe { ffi::dftd3_new_zero_damping(error.get_c_ptr(), s6, s8, s9, rs6, rs8, alp) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new zero damping parameters
    pub fn new_zero_damping(s6: f64, s8: f64, s9: f64, rs6: f64, rs8: f64, alp: f64) -> Self {
        Self::new_zero_damping_f(s6, s8, s9, rs6, rs8, alp).unwrap()
    }

    /// Load zero damping parameters from internal storage (failable)
    pub fn load_zero_damping_f(method: &str, atm: bool) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let token = std::ffi::CString::new(method).unwrap();
        let ptr = unsafe { ffi::dftd3_load_zero_damping(error.get_c_ptr(), token.into_raw(), atm) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Load zero damping parameters from internal storage
    pub fn load_zero_damping(method: &str, atm: bool) -> Self {
        Self::load_zero_damping_f(method, atm).unwrap()
    }

    /// Create new rational damping parameters (failable)
    pub fn new_rational_damping_f(
        s6: f64,
        s8: f64,
        s9: f64,
        a1: f64,
        a2: f64,
        alp: f64,
    ) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let ptr =
            unsafe { ffi::dftd3_new_rational_damping(error.get_c_ptr(), s6, s8, s9, a1, a2, alp) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new rational damping parameters
    pub fn new_rational_damping(s6: f64, s8: f64, s9: f64, a1: f64, a2: f64, alp: f64) -> Self {
        Self::new_rational_damping_f(s6, s8, s9, a1, a2, alp).unwrap()
    }

    /// Load rational damping parameters from internal storage (failable)
    pub fn load_rational_damping_f(method: &str, atm: bool) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let token = std::ffi::CString::new(method).unwrap();
        let ptr =
            unsafe { ffi::dftd3_load_rational_damping(error.get_c_ptr(), token.into_raw(), atm) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Load rational damping parameters from internal storage
    pub fn load_rational_damping(method: &str, atm: bool) -> Self {
        Self::load_rational_damping_f(method, atm).unwrap()
    }

    /// Create new modified zero damping parameters (failable)
    pub fn new_mzero_damping_f(
        s6: f64,
        s8: f64,
        s9: f64,
        rs6: f64,
        rs8: f64,
        alp: f64,
        bet: f64,
    ) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let ptr = unsafe {
            ffi::dftd3_new_mzero_damping(error.get_c_ptr(), s6, s8, s9, rs6, rs8, alp, bet)
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new modified zero damping parameters
    pub fn new_mzero_damping(
        s6: f64,
        s8: f64,
        s9: f64,
        rs6: f64,
        rs8: f64,
        alp: f64,
        bet: f64,
    ) -> Self {
        Self::new_mzero_damping_f(s6, s8, s9, rs6, rs8, alp, bet).unwrap()
    }

    /// Load modified zero damping parameters from internal storage (failable)
    pub fn load_mzero_damping_f(method: &str, atm: bool) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let token = std::ffi::CString::new(method).unwrap();
        let ptr =
            unsafe { ffi::dftd3_load_mzero_damping(error.get_c_ptr(), token.into_raw(), atm) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Load modified zero damping parameters from internal storage
    pub fn load_mzero_damping(method: &str, atm: bool) -> Self {
        Self::load_mzero_damping_f(method, atm).unwrap()
    }

    /// Create new modified rational damping parameters (failable)
    pub fn new_mrational_damping_f(
        s6: f64,
        s8: f64,
        s9: f64,
        a1: f64,
        a2: f64,
        alp: f64,
    ) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let ptr =
            unsafe { ffi::dftd3_new_mrational_damping(error.get_c_ptr(), s6, s8, s9, a1, a2, alp) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new modified rational damping parameters
    pub fn new_mrational_damping(s6: f64, s8: f64, s9: f64, a1: f64, a2: f64, alp: f64) -> Self {
        Self::new_mrational_damping_f(s6, s8, s9, a1, a2, alp).unwrap()
    }

    /// Load modified rational damping parameters from internal storage (failable)
    pub fn load_mrational_damping_f(method: &str, atm: bool) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let token = std::ffi::CString::new(method).unwrap();
        let ptr =
            unsafe { ffi::dftd3_load_mrational_damping(error.get_c_ptr(), token.into_raw(), atm) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Load modified rational damping parameters from internal storage
    pub fn load_mrational_damping(method: &str, atm: bool) -> Self {
        Self::load_mrational_damping_f(method, atm).unwrap()
    }

    /// Create new optimized damping parameters (failable)
    pub fn new_optimizedpower_damping_f(
        s6: f64,
        s8: f64,
        s9: f64,
        a1: f64,
        a2: f64,
        alp: f64,
        bet: f64,
    ) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let ptr = unsafe {
            ffi::dftd3_new_optimizedpower_damping(error.get_c_ptr(), s6, s8, s9, a1, a2, alp, bet)
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new optimized damping parameters
    pub fn new_optimizedpower_damping(
        s6: f64,
        s8: f64,
        s9: f64,
        a1: f64,
        a2: f64,
        alp: f64,
        bet: f64,
    ) -> Self {
        Self::new_optimizedpower_damping_f(s6, s8, s9, a1, a2, alp, bet).unwrap()
    }

    /// Load optimized damping parameters from internal storage (failable)
    pub fn load_optimizedpower_damping_f(method: &str, atm: bool) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let token = std::ffi::CString::new(method).unwrap();
        let ptr = unsafe {
            ffi::dftd3_load_optimizedpower_damping(error.get_c_ptr(), token.into_raw(), atm)
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Load optimized damping parameters from internal storage
    pub fn load_optimizedpower_damping(method: &str, atm: bool) -> Self {
        Self::load_optimizedpower_damping_f(method, atm).unwrap()
    }
}

pub struct DFTD3GCP {
    ptr: ffi::dftd3_gcp,
}

impl Drop for DFTD3GCP {
    fn drop(&mut self) {
        unsafe { ffi::dftd3_delete_gcp(&mut self.ptr) };
    }
}

impl DFTD3GCP {
    /// Load geometric counter-poise parameters from internal storage (failable)
    pub fn load_gcp_param_f(
        structure: &DFTD3Structure,
        method: &str,
        basis: &str,
    ) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let token_method = std::ffi::CString::new(method).unwrap();
        let token_basis = std::ffi::CString::new(basis).unwrap();
        let ptr = unsafe {
            ffi::dftd3_load_gcp_param(
                error.get_c_ptr(),
                structure.ptr,
                token_method.into_raw(),
                token_basis.into_raw(),
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Load geometric counter-poise parameters from internal storage
    pub fn load_gcp_param(structure: &DFTD3Structure, method: &str, basis: &str) -> Self {
        Self::load_gcp_param_f(structure, method, basis).unwrap()
    }

    /// Set realspace cutoffs (quantities in Bohr) (failable)
    pub fn set_realspace_cutoff_f(&self, bas: f64, srb: f64) -> Result<(), DFTD3Error> {
        let mut error = DFTD3Error::new();
        unsafe { ffi::dftd3_set_gcp_realspace_cutoff(error.get_c_ptr(), self.ptr, bas, srb) };
        match error.check() {
            true => Err(error),
            false => Ok(()),
        }
    }

    /// Set realspace cutoffs (quantities in Bohr)
    pub fn set_realspace_cutoff(&self, bas: f64, srb: f64) {
        self.set_realspace_cutoff_f(bas, srb).unwrap()
    }
}

/// Evaluate the dispersion energy and its derivatives (failable)
pub fn get_dispersion_f(
    structure: &DFTD3Structure,
    model: &DFTD3Model,
    param: &DFTD3Param,
) -> Result<(f64, Vec<f64>, Vec<f64>), DFTD3Error> {
    let natoms = structure.get_natoms();
    let mut energy = 0.0;
    let mut grad = vec![0.0; 3 * natoms];
    let mut sigma = vec![0.0; 9];
    let mut error = DFTD3Error::new();

    unsafe {
        ffi::dftd3_get_dispersion(
            error.get_c_ptr(),
            structure.ptr,
            model.ptr,
            param.ptr,
            &mut energy,
            grad.as_mut_ptr(),
            sigma.as_mut_ptr(),
        )
    };
    match error.check() {
        true => Err(error),
        false => Ok((energy, grad, sigma)),
    }
}

/// Evaluate the dispersion energy and its derivatives
pub fn get_dispersion(
    structure: &DFTD3Structure,
    model: &DFTD3Model,
    param: &DFTD3Param,
) -> (f64, Vec<f64>, Vec<f64>) {
    get_dispersion_f(structure, model, param).unwrap()
}

/// Evaluate the pairwise representation of the dispersion energy (failable)
pub fn get_pairwise_dispersion_f(
    structure: &DFTD3Structure,
    model: &DFTD3Model,
    param: &DFTD3Param,
) -> Result<(Vec<f64>, Vec<f64>), DFTD3Error> {
    let natoms = structure.get_natoms();
    let mut pair_energy2 = vec![0.0; natoms * natoms];
    let mut pair_energy3 = vec![0.0; natoms * natoms];
    let mut error = DFTD3Error::new();

    unsafe {
        ffi::dftd3_get_pairwise_dispersion(
            error.get_c_ptr(),
            structure.ptr,
            model.ptr,
            param.ptr,
            pair_energy2.as_mut_ptr(),
            pair_energy3.as_mut_ptr(),
        )
    };
    match error.check() {
        true => Err(error),
        false => Ok((pair_energy2, pair_energy3)),
    }
}

/// Evaluate the pairwise representation of the dispersion energy
pub fn get_pairwise_dispersion(
    structure: &DFTD3Structure,
    model: &DFTD3Model,
    param: &DFTD3Param,
) -> (Vec<f64>, Vec<f64>) {
    get_pairwise_dispersion_f(structure, model, param).unwrap()
}

/// Evaluate the counterpoise correction (failable)
pub fn get_counterpoise_f(
    structure: &DFTD3Structure,
    gcp: &DFTD3GCP,
) -> Result<(f64, Vec<f64>, Vec<f64>), DFTD3Error> {
    let natoms = structure.get_natoms();
    let mut energy = 0.0;
    let mut grad = vec![0.0; 3 * natoms];
    let mut sigma = vec![0.0; 9];
    let mut error = DFTD3Error::new();

    unsafe {
        ffi::dftd3_get_counterpoise(
            error.get_c_ptr(),
            structure.ptr,
            gcp.ptr,
            &mut energy,
            grad.as_mut_ptr(),
            sigma.as_mut_ptr(),
        )
    };
    match error.check() {
        true => Err(error),
        false => Ok((energy, grad, sigma)),
    }
}

/// Evaluate the counterpoise correction
pub fn get_counterpoise(structure: &DFTD3Structure, gcp: &DFTD3GCP) -> (f64, Vec<f64>, Vec<f64>) {
    get_counterpoise_f(structure, gcp).unwrap()
}

#[cfg(test)]
mod tests {
    use ffi::dftd3_load_optimizedpower_damping;

    use super::*;

    #[test]
    fn test_get_api_version() {
        println!("API version: {}", get_api_version());
    }

    #[test]
    fn test_get_api_version_compact() {
        println!("API version: {:?}", get_api_version_compact());
    }

    #[test]
    fn test_dftd3_error() {
        let mut error = DFTD3Error::new();
        println!("Error check   : {}", error.check());
        println!("Error message : {}", error.get_message());
        let token = std::ffi::CString::new("Hello").unwrap();
        unsafe { dftd3_load_optimizedpower_damping(error.get_c_ptr(), token.into_raw(), false) };
        println!("Error check   : {}", error.check());
        println!("Error message : {}", error.get_message());
        let token = std::ffi::CString::new("B3LYP").unwrap();
        unsafe { dftd3_load_optimizedpower_damping(error.get_c_ptr(), token.into_raw(), false) };
        println!("Error check   : {}", error.check());
        println!("Error message : {}", error.get_message());
    }

    #[test]
    fn test_get_dispersion() {
        let natoms = 2;
        let numbers = vec![1, 1];
        let positions = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        let lattice = vec![10.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 10.0];
        let periodic = vec![false, false, false];
        let structure = DFTD3Structure::new(natoms, &numbers, &positions, &lattice, &periodic);
        let model = DFTD3Model::new(&structure);
        let param = DFTD3Param::load_mrational_damping("B3LYP", false);
        let (energy, grad, sigma) = get_dispersion(&structure, &model, &param);
        println!("Dispersion energy: {}", energy);
        println!("Dispersion gradient: {:?}", grad);
        println!("Dispersion sigma: {:?}", sigma);
    }
}
