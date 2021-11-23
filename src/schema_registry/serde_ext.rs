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

use serde::Serialize;

/// SerdeExt trait to convert any Serializable struct to
/// json string.
pub trait SerdeExt {
    /// Converts a serializable struct to pretty json
    /// Fails in case the serde fails to convert it to pretty string.
    ///
    /// ```rust, no_run
    /// use serde::Serialize;
    /// use degauss::prelude::*;
    ///
    /// #[derive(Serialize)]
    /// pub struct Test{
    ///   pub name: String
    /// }
    ///
    /// let test = Test{name:"degauss".to_string()};
    /// println!("{}", test.pretty_string());
    ///
    fn pretty_string(&self) -> String;
}

impl<T> SerdeExt for T
where
    T: ?Sized + Serialize,
{
    fn pretty_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
