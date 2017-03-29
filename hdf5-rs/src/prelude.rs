//! The HDF5 prelude module.
//!
//! The purpose of this module is to provide reexports of many core `hdf5` traits so that
//! they can be then glob-imported all at once:
//!
//! ```ignore
//! use h5::prelude::*;
//! ```
//! This module provides reexports of such traits as `Object`, `Location` and `Container` and
//! does not expose any structures or functions.


pub use super::Object;
pub use super::Location;
pub use super::Container;
pub use super::Dimension;
pub use hdf5_types::H5Type;