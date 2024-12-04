#![allow(non_camel_case_types)]

pub mod ffi;
pub mod library;
pub mod prelude {
    pub use crate::library::*;
}
