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

/// Represents the payload used to exchange this information for the
/// access token (`AccessToken`).
#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthTokenPayload {
    /// The client ID you received from Amphitheatre when you registered the application.
    pub client_id: String,
    /// The client secret you received from Amphitheatre when you registered the application.
    pub client_secret: String,
    /// The code acquired in the previous authorization step.
    pub code: String,
    /// Only used to validate that it matches the original /oauth/authorize, not used to redirect again.
    pub redirect_uri: String,
    /// The state content originally passed to /oauth/authorize.
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct OAuthTokenParams {
    grant_type: String,
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
    state: String,
}

/// Represents an access token containing the token to access the API
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessToken {
    /// The token you can use to authenticate.
    pub access_token: String,
    /// The account ID in Amphitheatre this token belongs to.
    pub account_id: u64,
    /// The token scope (not used for now).
    pub scope: Option<String>,
    /// The token type.
    pub token_type: String,
}

impl Endpoint for AccessToken {
    type Output = Self;
}

/// The Oauth Service is used to request access to the API
///
/// See [API Documentation: oauth](https://docs.amphitheatre.app/api/oauth/)
pub struct OAuth<'a> {
    pub client: &'a Client,
}

impl OAuth<'_> {
    /// Exchange the short-lived authorization code for an access token
    /// you can use to authenticate your API calls.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use amp_client::client::Client;
    /// use amp_client::oauth::OAuthTokenPayload;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let token = Some(String::from("AUTH_TOKEN"));
    ///     let client = Client::new("https://cloud.amphitheatre.app", token);
    ///     let payload = OAuthTokenPayload {
    ///         client_id: "id".to_string(),
    ///         client_secret: "secret".to_string(),
    ///         code: "code".to_string(),
    ///         redirect_uri: "/redirect_uri".to_string(),
    ///         state: "state".to_string(),
    ///     };
    ///
    ///     let access_token = client.oauth().exchange_authorization_for_token(payload).await.unwrap();
    /// }
    /// ```
    ///
    /// # Attributes
    ///
    /// `payload`: The `OAuthTokenPayload` with the necessary information to get the access token.
    pub async fn exchange_authorization_for_token(
        &self,
        payload: OAuthTokenPayload,
    ) -> Result<AccessToken, HTTPError> {
        let path = "/oauth/access_token";
        let data = OAuthTokenParams {
            grant_type: "authorization_code".to_string(),
            client_id: payload.client_id,
            client_secret: payload.client_secret,
            code: payload.code,
            redirect_uri: payload.redirect_uri,
            state: payload.state,
        };

        let res = self
            .client
            .post::<AccessToken, OAuthTokenParams>(path, &data)
            .await?;
        Ok(res.data.unwrap())
    }
}
