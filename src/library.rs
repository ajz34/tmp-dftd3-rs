use crate::ffi;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_api_version() {
        println!("API version: {}", get_api_version());
    }

    #[test]
    fn test_get_api_version_compact() {
        println!("API version: {:?}", get_api_version_compact());
    }
}
