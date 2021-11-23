use crate::compat::DegaussCompatMode;

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

#[derive(Debug)]
pub enum Auth {
    Basic { username: String, password: String },
    Skip,
}
use strum_macros::{Display, EnumIter, EnumString, EnumVariantNames};
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SchemaRegistryErrResponse {
    pub error_code: i32,
    pub message: String,
}

/// The subject for this we are going to register a schema
#[derive(
    EnumIter,
    EnumVariantNames,
    EnumString,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
)]
pub enum SchemaSubjectType {
    #[strum(serialize = "key")]
    Key,
    #[strum(serialize = "value")]
    Value,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SchemaRegistrationResponse {
    pub id: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SchemaCompatibleResponse {
    pub is_compatible: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SubjectCompatibilitySetResponse {
    pub compatibility: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SubjectCompatibilityGetResponse {
    #[serde(rename(serialize = "compatibilityLevel", deserialize = "compatibilityLevel"))]
    pub compatibility_level: DegaussCompatMode,
}
