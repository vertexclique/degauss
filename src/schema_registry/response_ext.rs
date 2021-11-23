// MIT License
//
// Copyright (c) 2021 Theo M. Bulut, Ankur Srivastava
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Trait to check for errors from Schema Registry and convert it to
//! a struct. This is helpful in deserializing the schema registry errors.
//!
//! In case the status is not successful, parse the errors and return the
//! corresponding error code
//! ```json
//! { error_code: i32, message: String }
//! ```

use crate::errors::DegaussError;
use crate::schema_registry::types::*;

use isahc::{prelude::*, Body, Response};

/// ResponseExt trait for checking errors in the incoming response
/// from Kafka Schema Registry
pub trait ResponseExt {
    /// Check for error in the incoming response from Kafka Schema Registry
    ///
    /// In case the status is not successful, parse the errors and return the
    /// corresponding error code
    ///
    /// ```json
    /// { error_code: i32, message: String }
    /// ```
    fn check_for_error(self) -> Result<Response<Body>, DegaussError>;
}

impl ResponseExt for Response<Body> {
    fn check_for_error(mut self) -> Result<Response<Body>, DegaussError> {
        match self.status().is_success() {
            true => Ok(Response::new(self.into_body())),
            false => {
                let err_response = self.json::<SchemaRegistryErrResponse>()?;
                Err(DegaussError::SrHttp {
                    error_code: err_response.error_code,
                    message: err_response.message,
                })
            }
        }
    }
}
