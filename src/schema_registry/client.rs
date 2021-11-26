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

use crate::compat::DegaussCompatMode;
use crate::errors::DegaussError;
use crate::schema_registry::types::*;

use crate::schema_registry::ResponseExt;
use avro_rs::Schema;
use isahc::{
    auth::{Authentication, Credentials},
    config::{RedirectPolicy, VersionNegotiation},
    prelude::*,
    HttpClient, Request,
};
use serde::Serialize;
use std::time::Duration;

/// Create an instance of SchemaRegistryClient
///
/// ```rust,no_run
/// use degauss::prelude::*;
///
/// let client = SchemaRegistryClient::new("http://localhost:8081",
///         Auth::Basic{
///         username: "username".to_string(),
///         password: "password".to_string(),
///     })
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct SchemaRegistryClient {
    httpclient: isahc::HttpClient,
    url: String,
}

impl SchemaRegistryClient {
    fn get_subject_from_topic(&self, topic: &str, subject: SchemaSubjectType) -> String {
        match subject {
            SchemaSubjectType::Key => format!("{}-key", topic),
            SchemaSubjectType::Value => format!("{}-value", topic),
        }
    }

    fn make_request<T: ?Sized + Serialize, U: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        method: isahc::http::Method,
        body: Option<&T>,
    ) -> Result<U, DegaussError> {
        let op = match method {
            isahc::http::Method::PUT => Request::put(url),
            isahc::http::Method::GET => Request::get(url),
            isahc::http::Method::POST => Request::post(url),
            isahc::http::Method::DELETE => Request::delete(url),
            _ => {
                return Err(DegaussError::Custom(
                    "Unsupported method for make_request".to_string(),
                ));
            }
        };

        let b = match body {
            Some(body) => serde_json::to_vec(body)?,
            None => vec![],
        };

        let request = op.body(b)?;
        let resp = self
            .httpclient
            .send(request)?
            .check_for_error()?
            .json::<U>()?;
        Ok(resp)
    }

    /// Create an instance of the schema registry client
    ///
    /// ```rust,no_run
    /// use degauss::prelude::*;
    ///
    /// let client = SchemaRegistryClient::new("http://localhost:8081",
    ///         Auth::Basic{
    ///         username: "username".to_string(),
    ///         password: "password".to_string(),
    ///     })
    ///     .unwrap();
    /// ```
    pub fn new<T: Into<String>>(url: T, auth: Auth) -> Result<Self, DegaussError> {
        let builder = HttpClient::builder()
            .version_negotiation(VersionNegotiation::http11())
            .redirect_policy(RedirectPolicy::Limit(10))
            .timeout(Duration::from_secs(20))
            .default_header("Content-Type", "application/vnd.schemaregistry.v1+json");

        let httpclient = match auth {
            Auth::Basic { username, password } => builder
                .authentication(Authentication::basic())
                .credentials(Credentials::new(username, password)),
            Auth::Skip => builder,
        }
        .build()?;
        Ok(SchemaRegistryClient {
            httpclient,
            url: url.into(),
        })
    }

    /// Register the given schema to schema-registry
    pub fn register_schema(
        self,
        schema: &Schema,
        topic: &str,
        subject_type: SchemaSubjectType,
    ) -> Result<SchemaRegistrationResponse, DegaussError> {
        let url = format!(
            "{url}/subjects/{subject}/versions",
            url = self.url,
            subject = self.get_subject_from_topic(topic, subject_type),
        );

        let payload = serde_json::json!({
            "schema": schema.canonical_form()
        });
        let resp = self.make_request(&url, isahc::http::Method::POST, Some(&payload))?;
        Ok(resp)
    }

    /// Set the compatibility of a given subject.
    /// Subject is evaluated using the topic name and the type of subject.
    ///
    /// subject = topic-key or topic-value
    pub fn set_compatibility(
        self,
        topic: &str,
        subject_type: SchemaSubjectType,
        compatibility: DegaussCompatMode,
    ) -> Result<SubjectCompatibilitySetResponse, DegaussError> {
        let url = format!(
            "{url}/config/{subject}",
            url = self.url,
            subject = self.get_subject_from_topic(topic, subject_type),
        );

        let payload = serde_json::json!({
            "compatibility": compatibility.to_string()
        });

        let resp = self.make_request(&url, isahc::http::Method::PUT, Some(&payload))?;
        Ok(resp)
    }

    /// Get the compatibility of a given subject.
    /// Subject is evaluated using the topic name and the type of subject.
    ///
    /// subject = topic-key or topic-value
    pub fn get_compatibility(
        self,
        topic: &str,
        subject_type: SchemaSubjectType,
    ) -> Result<SubjectCompatibilityGetResponse, DegaussError> {
        let url = format!(
            "{url}/config/{subject}",
            url = self.url,
            subject = self.get_subject_from_topic(topic, subject_type),
        );

        let none: Option<String> = None;
        let resp = self.make_request(&url, isahc::http::Method::GET, none.as_ref())?;
        Ok(resp)
    }

    /// Check the compatibility with given Schema
    pub fn check_compatibility(
        self,
        schema: &Schema,
        topic: &str,
        subject_type: SchemaSubjectType,
        verbose: bool,
    ) -> Result<SchemaCompatibleResponse, DegaussError> {
        let url = format!(
            "{url}/compatibility/subjects/{subject}/versions?verbose={verbose}",
            url = self.url,
            subject = self.get_subject_from_topic(topic, subject_type),
            verbose = verbose,
        );
        let payload = serde_json::json!({
            "schema": schema.canonical_form()
        });
        let resp = self.make_request(&url, isahc::http::Method::POST, Some(&payload))?;
        Ok(resp)
    }
}

/// previous schema.
#[cfg(test)]
mod tests {

    use super::*;
    use crate::prelude::FromFile;
    use avro_rs::Schema;

    fn test_client() -> SchemaRegistryClient {
        use std::env;
        let username = env::var("DEGAUSS_SCHEMA_REGISTRY_USER").unwrap();
        let password = env::var("DEGAUSS_SCHEMA_REGISTRY_PASS").unwrap();
        let url = env::var("DEGAUSS_SCHEMA_REGISTRY_URL").unwrap();

        SchemaRegistryClient::new(
            url,
            Auth::Basic {
                username: username,
                password: password,
            },
        )
        .unwrap()
    }

    fn test_schema() -> Schema {
        Schema::parse_file("tests/data/schema2.avsc").unwrap()
    }

    /// Generate a random length string
    pub fn random_chars(length: usize, prefix: &str) -> String {
        use rand::{distributions::Alphanumeric, Rng};

        let suffix: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
        format!("{}{}", prefix, suffix)
    }

    #[test]
    fn test_register_schema() {
        let client = test_client();
        let topic = random_chars(10, "test");
        let schema = test_schema();
        let res = client
            .register_schema(&schema, &topic, SchemaSubjectType::Value)
            .unwrap();
        assert!(res.id > 0)
    }

    #[test]
    fn test_set_schema() {
        let client = test_client();
        let topic = random_chars(10, "test");
        let res = client
            .set_compatibility(&topic, SchemaSubjectType::Value, DegaussCompatMode::Forward)
            .unwrap();
        assert!(res.compatibility == DegaussCompatMode::Forward.to_string());
    }

    #[test]
    fn test_set_and_get_schema() {
        let client = test_client();
        let topic = random_chars(10, "test");

        let res = client
            .clone()
            .set_compatibility(
                &topic,
                SchemaSubjectType::Value,
                DegaussCompatMode::Backward,
            )
            .unwrap();
        assert!(res.compatibility == DegaussCompatMode::Backward.to_string());

        let res = client
            .get_compatibility(&topic, SchemaSubjectType::Value)
            .unwrap();

        let want = DegaussCompatMode::Backward;
        assert!(
            res.compatibility_level == want,
            "got {} want {}",
            res.compatibility_level,
            want
        )
    }

    #[test]
    fn test_check_compatibility() {
        let client = test_client();
        let topic = "test";
        let schema = test_schema();
        let res = client
            .check_compatibility(&schema, topic, SchemaSubjectType::Value, true)
            .unwrap();

        let want = true;
        assert!(
            res.is_compatible == want,
            "got {} want {}",
            res.is_compatible,
            want
        )
    }
}
