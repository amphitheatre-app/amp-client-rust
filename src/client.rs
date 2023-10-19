// Copyright 2023 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

use amp_common::http::{Endpoint, HTTPError, Response};

use super::accounts::Accounts;
use super::actors::Actors;
use super::oauth::OAuth;
use super::playbooks::Playbooks;

/// Represents the Rust client for the Amphitheatre API
///
/// The client is your entrypoint to the Amphitheatre API. Using it you will be
/// able to call all the endpoints of the Amphitheatre API and their respective functions.
///
/// # Examples
///
/// ```no_run
/// use amp_client::client::Client;
///
/// let token = Some(String::from("AUTH_TOKEN"));
/// let client = Client::new("https://cloud.amphitheatre.app", token);
/// let account = client.accounts().me().unwrap();
/// ```
pub struct Client {
    client: amp_common::http::Client,
}

impl Client {
    pub fn new(base_url: &str, token: Option<String>) -> Self {
        Self {
            client: amp_common::http::Client::new(base_url, token),
        }
    }

    /// Sends a GET request to the Amphitheatre API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `options`: optionally a `RequestOptions` with things like pagination,
    /// filtering and sorting
    pub fn get<E: Endpoint>(
        &self,
        path: &str,
        options: Option<HashMap<String, String>>,
    ) -> Result<Response<E::Output>, HTTPError> {
        self.client.get::<E>(path, options)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    const BASE_URL: &str = "https://cloud.amphitheatre.app";

    #[test]
    fn creates_a_client() {
        let token = Some("some-auth-token".to_string());
        let _client = Client::new(BASE_URL, token);
    }
}

impl Client {
    /// Returns the `accounts` services attached to this client
    pub fn accounts(&self) -> Accounts {
        Accounts { client: &self.client }
    }

    /// Returns the `actors` services attached to this client
    pub fn actors(&self) -> Actors {
        Actors { client: &self.client }
    }

    /// Returns the `oauth` service attached to this client
    pub fn oauth(&self) -> OAuth {
        OAuth { client: &self.client }
    }

    /// Returns the `playbooks` service attached to this client
    pub fn playbooks(&self) -> Playbooks {
        Playbooks { client: &self.client }
    }
}
