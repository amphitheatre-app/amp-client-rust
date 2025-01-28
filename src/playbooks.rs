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

use std::collections::HashMap;

use amp_common::{
    http::{
        endpoint::{Empty, Endpoint},
        Client, HTTPError,
    },
    resource::{PlaybookSpec, Preface},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlaybookPayload {
    /// The title of the playbook
    pub title: String,
    /// The description of the playbook
    pub description: String,
    /// The leading character manifest of the playbook
    pub preface: Preface,
}

struct PlaybookEndpoint;

impl Endpoint for PlaybookEndpoint {
    type Output = PlaybookSpec;
}

struct PlaybooksEndpoint;

impl Endpoint for PlaybooksEndpoint {
    type Output = Vec<PlaybookSpec>;
}

/// The Playbooks Service handles the playbooks endpoint of the Amphitheatre API.
///
/// See [API Documentation: playbook](https://docs.amphitheatre.app/api/playbook)
pub struct Playbooks<'a> {
    pub client: &'a Client,
}

impl Playbooks<'_> {
    /// Lists the playbooks in the current account.
    ///
    /// # Arguments
    ///
    /// `options`: The `RequestOptions`
    ///             - Sort: `id`, `label`, `email`
    pub async fn list(
        &self,
        options: Option<HashMap<String, String>>,
    ) -> Result<Vec<PlaybookSpec>, HTTPError> {
        let res = self
            .client
            .get::<PlaybooksEndpoint>("/playbooks", options)
            .await?;
        Ok(res.data.unwrap())
    }

    /// Create a playbook in the account.
    ///
    /// # Arguments
    ///
    /// `payload`: the `PlaybookPayload` with the information needed to create
    /// the playbook
    pub async fn create(&self, payload: PlaybookPayload) -> Result<PlaybookSpec, HTTPError> {
        let res = self
            .client
            .post::<PlaybookEndpoint, PlaybookPayload>("/playbooks", &payload)
            .await?;
        Ok(res.data.unwrap())
    }

    /// Retrieve a playbook
    ///
    /// # Arguments
    ///
    /// `pid`: The ID of the playbook we want to retrieve
    pub async fn get(&self, pid: &str) -> Result<PlaybookSpec, HTTPError> {
        let path = format!("/playbooks/{}", pid);
        let res = self.client.get::<PlaybookEndpoint>(&path, None).await?;
        Ok(res.data.unwrap())
    }

    /// Update a playbook
    ///
    /// # Arguments
    ///
    /// `pid`: The playbook id
    /// `payload`: The `PlaybookPayload` with the information needed to update
    pub async fn update(&self, pid: &str, payload: PlaybookPayload) -> Result<PlaybookSpec, HTTPError> {
        let path = format!("/playbooks/{}", pid);
        let res = self
            .client
            .patch::<PlaybookEndpoint, PlaybookPayload>(&path, &payload)
            .await?;
        Ok(res.data.unwrap())
    }

    /// Delete a playbook
    ///
    /// # Arguments
    ///
    /// `pid`: The playbook id
    pub async fn delete(&self, pid: &str) -> Result<u16, HTTPError> {
        let path = format!("/playbooks/{}", pid);
        Ok(self.client.delete::<Empty>(&path).await?.status.as_u16())
    }

    /// Retrieve the event streams of playbook
    ///
    /// # Arguments
    ///
    /// `playbook_id`: The playbook id
    pub fn events(&self, _pid: &str) -> String {
        // let path = format!("/playbooks/{}/events", pid);
        String::from("event stream (JSON)")
    }

    /// Start a playbook
    ///
    /// # Arguments
    ///
    /// `pid`: The playbook id
    pub async fn start(&self, pid: &str) -> Result<u16, HTTPError> {
        let path = format!("/playbooks/{}/actions/start", pid);
        Ok(self
            .client
            .post::<Empty, Value>(&path, &json!(null))
            .await?
            .status
            .as_u16())
    }

    /// Stop a playbook
    ///
    /// # Arguments
    ///
    /// `pid`: The playbook id
    pub async fn stop(&self, pid: &str) -> Result<u16, HTTPError> {
        let path = format!("/playbooks/{}/actions/stop", pid);
        Ok(self
            .client
            .post::<Empty, Value>(&path, &json!(null))
            .await?
            .status
            .as_u16())
    }
}
