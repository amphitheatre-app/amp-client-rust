// Copyright 2022 The Amphitheatre Authors.
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

use amp_common::client::{ClientError, Endpoint, RequestOptions, Response};

use super::accounts::Accounts;
use super::actors::Actors;
use super::oauth::OAuth;
use super::playbooks::Playbooks;

/// Represents the Rust client for the Amphitheatre API
///
/// The client is your entrypoint to the Amphitheatre API. Using it you will be
/// able to call all the enpoints of the Amphitheatre API and their respective functions.
///
/// # Examples
///
/// ```no_run
/// use amp_client::client::Client;
///
/// let client = Client::new(
///     String::from("https://cloud.amphitheatre.app"),
///     String::from("AUTH_TOKEN"),
/// );
/// let response = client.accounts().me().unwrap();
///
/// let account = response.data.unwrap();
/// ```
pub struct Client {
    client: amp_common::client::Client,
}

impl Client {
    pub fn new(base_url: String, token: String) -> Self {
        Self {
            client: amp_common::client::Client::new(&base_url, &token),
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
        options: Option<RequestOptions>,
    ) -> Result<Response<E::Output>, ClientError> {
        self.client.get::<E>(path, options)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    const BASE_URL: &str = "https://cloud.amphitheatre.app";

    #[test]
    fn creates_a_client() {
        let token = "some-auth-token";
        let _client = Client::new(String::from(BASE_URL), String::from(token));
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
