// Copyright (c) 2021 bb8-mongodb developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `bb8-mongodb` errors

/// A `bb8-mongodb` Error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error from the `mongodb` library
    #[error("There was an error connecting to the database!")]
    MongoDB(#[from] mongodb::error::Error),
}
