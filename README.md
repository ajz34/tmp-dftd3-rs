# Rust bindings to `simple-dftd3` library

This crate performs safe wrapper on library `simple-dftd3`.

This implementation based on C wrapper of original DFT-D3 [dftd3/simple-dftd3](https://github.com/dftd3/simple-dftd3).

## Usage

As an example, given molecular information, functional `PW6B95` with D3(BJ) can be evaluated as
```rust
use rest_dftd3::prelude::*;
let structure = DFTD3Structure::new(natoms, &charges, &coords, &latice, &periodic);
let model = DFTD3Model::new(&structure);
// d3bj corresponds to rational_damping
let param = DFTD3Param::load_rational_damping("PW6B95", false);
let (energy, gradient, sigma) = get_dispersion(&structure, &model, &param);
```

For details, we refer to [test case](tests/test_d3bj.rs).

## Installation

### Shared library from conda-forge (recommended scheme)

The recommended installation scheme using by shared library:
- Make sure shared object files `libs-dftd3.so` and `libmctc-lib.so` are in environment variable `LD_LIBRARY_PATH`. These files can be obtained by conda/mamba installation (see also [simple-dftd3 installation guide](https://github.com/dftd3/simple-dftd3/?tab=readme-ov-file#conda-package)) and found in conda library list (usually in `<conda-base-path>/envs/<your-env>/lib/`).
- Then import this crate in your `Cargo.toml` file by 
    ```toml
    [dependencies]
    <...>
    rest_dftd3 = { version = "0.1" }
    ```

### Static library from conda-forge

For static library,
- Make sure shared object files `libs-dftd3.a` and `libmctc-lib.a` are in environment variable `LD_LIBRARY_PATH`. These files can also be obtained by conda/mamba installation.
- Then import this crate in your `Cargo.toml` file by 
    ```toml
    [dependencies]
    <...>
    rest_dftd3 = { version = "0.1", features = ["static"] }
    ```

Using static library have pros and cons:
- pro: It is more suitable for distribution, given the same architecture for compilation and usage.
- con: It is not fully static. It still links to external libraries `gomp` and `gfortran`. In current workflow, you may need to provide them as shared libraries in `LD_LIBRARY_PATH`. These libraries are also provided by conda-forge.
- license note: `simple-dftd3` library is LGPL-v3.0. Some restrictions may occur if you only distribute your program by static-linked binary if license of your program is not GPL-v3.

### Build both simple-dftd3 and its rust bindings

We also provide automatic installation, if use have no simple-dftd3 libraries at hand, and access to github.com is available. This is done by cmake compilation.

The following code may works:
```bash
git clone git@github.com:ajz34/tmp-dftd3-rs.git
cd tmp-dftd3-rs
cargo test
```
But if you also found that when incorporating this crate in other projects, it tells you `libs-dftd3.so` or `libmctc-lib.so` not found; then you may try to find this shared object in build directory, and add these libraries into `LD_LIBRARY_PATH`.

## License

This project is dual licensed by Apache and MIT.

This project is simply wrapper. For guides to use DFT-D3, please refer to [dftd3/simple-dftd3](https://github.com/dftd3/simple-dftd3). Also note that original simple-dftd3 library is licensed by LGPL-v3.
