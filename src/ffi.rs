/* automatically generated by rust-bindgen 0.69.4 and has been modified after */

#[doc = "Error handle class"]
pub type dftd3_error = *mut std::ffi::c_void;

#[doc = "Molecular structure data class"]
pub type dftd3_structure = *mut std::ffi::c_void;

#[doc = "Dispersion model class"]
pub type dftd3_model = *mut std::ffi::c_void;

#[doc = "Counter-poisecorrection parameters class"]
pub type dftd3_gcp = *mut std::ffi::c_void;

#[doc = "Damping parameter class"]
pub type dftd3_param = *mut std::ffi::c_void;

extern "C" {
    #[doc = "Obtain library version as major * 10000 + minor + 100 + patch"]
    pub fn dftd3_get_version() -> std::ffi::c_int;
}
extern "C" {
    #[doc = "Create new error handle object"]
    pub fn dftd3_new_error() -> dftd3_error;
}
extern "C" {
    #[doc = "Check error handle status"]
    pub fn dftd3_check_error(arg1: dftd3_error) -> std::ffi::c_int;
}
extern "C" {
    #[doc = "Get error message from error handle"]
    pub fn dftd3_get_error(
        arg1: dftd3_error,
        arg2: *mut std::ffi::c_char,
        arg3: *const std::ffi::c_int,
    );
}
extern "C" {
    #[doc = "Delete error handle object"]
    pub fn dftd3_delete_error(arg1: *mut dftd3_error);
}
extern "C" {
    #[doc = "Create new molecular structure data (quantities in Bohr)"]
    pub fn dftd3_new_structure(
        arg1: dftd3_error,
        arg2: std::ffi::c_int,
        arg3: *const std::ffi::c_int,
        arg4: *const f64,
        arg5: *const f64,
        arg6: *const bool,
    ) -> dftd3_structure;
}
extern "C" {
    #[doc = "Delete molecular structure data"]
    pub fn dftd3_delete_structure(arg1: *mut dftd3_structure);
}
extern "C" {
    #[doc = "Update coordinates and lattice parameters (quantities in Bohr)"]
    pub fn dftd3_update_structure(
        arg1: dftd3_error,
        arg2: dftd3_structure,
        arg3: *const f64,
        arg4: *const f64,
    );
}
extern "C" {
    #[doc = "Create new D3 dispersion model"]
    pub fn dftd3_new_d3_model(arg1: dftd3_error, arg2: dftd3_structure) -> dftd3_model;
}
extern "C" {
    #[doc = "Set realspace cutoffs (quantities in Bohr)"]
    pub fn dftd3_set_model_realspace_cutoff(
        arg1: dftd3_error,
        arg2: dftd3_model,
        arg3: f64,
        arg4: f64,
        arg5: f64,
    );
}
extern "C" {
    #[doc = "Delete dispersion model"]
    pub fn dftd3_delete_model(arg1: *mut dftd3_model);
}
extern "C" {
    #[doc = "Create new zero damping parameters"]
    pub fn dftd3_new_zero_damping(
        arg1: dftd3_error,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: f64,
        arg7: f64,
    ) -> dftd3_param;
}
extern "C" {
    #[doc = "Load zero damping parameters from internal storage"]
    pub fn dftd3_load_zero_damping(
        arg1: dftd3_error,
        arg2: *mut std::ffi::c_char,
        arg3: bool,
    ) -> dftd3_param;
}
extern "C" {
    #[doc = "Create new rational damping parameters"]
    pub fn dftd3_new_rational_damping(
        arg1: dftd3_error,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: f64,
        arg7: f64,
    ) -> dftd3_param;
}
extern "C" {
    #[doc = "Load rational damping parameters from internal storage"]
    pub fn dftd3_load_rational_damping(
        arg1: dftd3_error,
        arg2: *mut std::ffi::c_char,
        arg3: bool,
    ) -> dftd3_param;
}
extern "C" {
    #[doc = "Create new modified zero damping parameters"]
    pub fn dftd3_new_mzero_damping(
        arg1: dftd3_error,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: f64,
        arg7: f64,
        arg8: f64,
    ) -> dftd3_param;
}
extern "C" {
    #[doc = "Load modified zero damping parameters from internal storage"]
    pub fn dftd3_load_mzero_damping(
        arg1: dftd3_error,
        arg2: *mut std::ffi::c_char,
        arg3: bool,
    ) -> dftd3_param;
}
extern "C" {
    #[doc = "Create new modified rational damping parameters"]
    pub fn dftd3_new_mrational_damping(
        arg1: dftd3_error,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: f64,
        arg7: f64,
    ) -> dftd3_param;
}
extern "C" {
    #[doc = "Load modified rational damping parameters from internal storage"]
    pub fn dftd3_load_mrational_damping(
        arg1: dftd3_error,
        arg2: *mut std::ffi::c_char,
        arg3: bool,
    ) -> dftd3_param;
}
extern "C" {
    #[doc = "Create new optimized power damping parameters"]
    pub fn dftd3_new_optimizedpower_damping(
        arg1: dftd3_error,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: f64,
        arg7: f64,
        arg8: f64,
    ) -> dftd3_param;
}
extern "C" {
    #[doc = "Load optimized power damping parameters from internal storage"]
    pub fn dftd3_load_optimizedpower_damping(
        arg1: dftd3_error,
        arg2: *mut std::ffi::c_char,
        arg3: bool,
    ) -> dftd3_param;
}
extern "C" {
    #[doc = "Delete damping parameters"]
    pub fn dftd3_delete_param(arg1: *mut dftd3_param);
}
extern "C" {
    #[doc = "Load geometric counter-poise parameters from internal storage"]
    pub fn dftd3_load_gcp_param(
        arg1: dftd3_error,
        arg2: dftd3_structure,
        arg3: *mut std::ffi::c_char,
        arg4: *mut std::ffi::c_char,
    ) -> dftd3_gcp;
}
extern "C" {
    #[doc = "Set realspace cutoffs (quantities in Bohr)"]
    pub fn dftd3_set_gcp_realspace_cutoff(arg1: dftd3_error, arg2: dftd3_gcp, arg3: f64, arg4: f64);
}
extern "C" {
    #[doc = "Delete counter-poise parameters"]
    pub fn dftd3_delete_gcp(arg1: *mut dftd3_gcp);
}
extern "C" {
    #[doc = "Evaluate the dispersion energy and its derivatives"]
    pub fn dftd3_get_dispersion(
        arg1: dftd3_error,
        arg2: dftd3_structure,
        arg3: dftd3_model,
        arg4: dftd3_param,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut f64,
    );
}
extern "C" {
    #[doc = "Evaluate the pairwise representation of the dispersion energy"]
    pub fn dftd3_get_pairwise_dispersion(
        arg1: dftd3_error,
        arg2: dftd3_structure,
        arg3: dftd3_model,
        arg4: dftd3_param,
        arg5: *mut f64,
        arg6: *mut f64,
    );
}
extern "C" {
    #[doc = "Evaluate the dispersion energy and its derivatives"]
    pub fn dftd3_get_counterpoise(
        arg1: dftd3_error,
        arg2: dftd3_structure,
        arg3: dftd3_gcp,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut f64,
    );
}