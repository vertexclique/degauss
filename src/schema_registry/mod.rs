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

//!
//! A lightweight schema registry client to interact with Kafka Schema Registry.
//!
//! ### Usage
//! ```rust, no_run
//! use degauss::prelude::*;
//! // if username/password is provided
//! let auth = Auth::Basic {
//!  username: "user".to_string(),
//!  password: "pass".to_string(),
//! };
//!
//! // if no username/password then
//! // let auth = Auth::Skip;
//! let client = SchemaRegistryClient::new("http://url-of-schema-registry", auth).expect("Failed to create a Schema Registry client");
//! // Use your client to interact with schema registry
//!```
//!
mod client;
pub use client::SchemaRegistryClient;
mod response_ext;
pub mod types;
pub use response_ext::ResponseExt;
mod serde_ext;
pub use serde_ext::SerdeExt;
