// Copyright (c) The Amphitheatre Authors. All rights reserved.
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

use amp_common::http::{endpoint::Endpoint, Client, HTTPError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    /// The account ID
    pub id: u64,
    /// The account email
    pub email: String,
    /// The account name
    pub name: String,
    /// When the account was created in Amphitheatre
    pub created_at: String,
    /// When the account was updated in Amphitheatre
    pub updated_at: String,
}

impl Endpoint for Account {
    type Output = Account;
}

/// The Accounts Service handles the account endpoint of the Amphitheatre API.
///
/// See [API Documentation: Account](https://docs.amphitheatre.app/api/account)
pub struct Accounts<'a> {
    pub client: &'a Client,
}

impl Accounts<'_> {
    /// Retrieves the details about the current authenticated entity used to access the API.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use amp_client::client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///
    ///     let token = Some(String::from("AUTH_TOKEN"));
    ///     let client = Client::new("https://cloud.amphitheatre.app", token);
    ///     let account = client.accounts().me().await.unwrap();
    /// }
    /// ```
    pub async fn me(&self) -> Result<Account, HTTPError> {
        let res = self.client.get::<Account>("/me", None).await?;
        Ok(res.data.unwrap())
    }
}
