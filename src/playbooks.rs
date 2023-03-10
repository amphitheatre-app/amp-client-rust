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

use std::collections::HashMap;

use amp_common::client::{Client, ClientError, Endpoint};
use amp_common::schema::Source;
use serde::{Deserialize, Serialize};

use crate::Wrapper;

#[derive(Debug, Deserialize, Serialize)]
pub struct Playbook {
    /// The playbook ID in Amphitheatre.
    pub id: String,
    /// The title of the playbook.
    pub title: String,
    /// The description of the playbook.
    pub description: String,
    /// When the playbook was created in Amphitheatre.
    pub created_at: String,
    /// When the playbook was last updated in Amphitheatre.
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlaybookPayload {
    /// The title of the playbook
    pub title: String,
    /// The description of the playbook
    pub description: String,
    /// The leading character manifest of the playbook
    pub preface: Source,
}

struct PlaybookEndpoint;

impl Endpoint for PlaybookEndpoint {
    type Output = Wrapper<Playbook>;
}

struct PlaybooksEndpoint;

impl Endpoint for PlaybooksEndpoint {
    type Output = Wrapper<Vec<Playbook>>;
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
    pub fn list(&self, options: Option<HashMap<String, String>>) -> Result<Vec<Playbook>, ClientError> {
        let res = self.client.get::<PlaybooksEndpoint>("/playbooks", options)?;
        Ok(res.data.unwrap().data)
    }

    /// Create a playbook in the account.
    ///
    /// # Arguments
    ///
    /// `payload`: the `PlaybookPayload` with the information needed to create
    /// the playbook
    pub fn create(&self, payload: PlaybookPayload) -> Result<Playbook, ClientError> {
        match serde_json::to_value(payload) {
            Ok(json) => {
                let res = self.client.post::<PlaybookEndpoint>("/playbooks", json)?;
                Ok(res.data.unwrap().data)
            }
            Err(_) => Err(ClientError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Retrieve a playbook
    ///
    /// # Arguments
    ///
    /// `playbook_id`: The ID of the playbook we want to retrieve
    pub fn get(&self, playbook_id: &str) -> Result<Playbook, ClientError> {
        let path = format!("/playbooks/{}", playbook_id);
        let res = self.client.get::<PlaybookEndpoint>(&path, None)?;
        Ok(res.data.unwrap().data)
    }

    /// Update a playbook
    ///
    /// # Arguments
    ///
    /// `playbook_id`: The playbook id
    /// `payload`: The `PlaybookPayload` with the information needed to update
    pub fn update(&self, playbook_id: &str, payload: PlaybookPayload) -> Result<Playbook, ClientError> {
        let path = format!("/playbooks/{}", playbook_id);

        match serde_json::to_value(payload) {
            Ok(json) => {
                let res = self.client.patch::<PlaybookEndpoint>(&path, json)?;
                Ok(res.data.unwrap().data)
            }
            Err(_) => Err(ClientError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }

    /// Delete a playbook
    ///
    /// # Arguments
    ///
    /// `playbook_id`: The playbook id
    pub fn delete(&self, playbook_id: &str) -> Result<u16, ClientError> {
        let path = format!("/playbooks/{}", playbook_id);
        Ok(self.client.delete(&path)?.status)
    }

    /// Retrieve the event streams of playbook
    ///
    /// # Arguments
    ///
    /// `playbook_id`: The playbook id
    pub fn events(&self, _playbook_id: &str) -> String {
        // let path = format!("/playbooks/{}/events", playbook_id);
        String::from("event stream (JSON)")
    }

    /// Start a playbook
    ///
    /// # Arguments
    ///
    /// `playbook_id`: The playbook id
    pub fn start(&self, playbook_id: &str) -> Result<u16, ClientError> {
        let path = format!("/playbooks/{}/actions/start", playbook_id);
        Ok(self.client.empty_post(&path)?.status)
    }

    /// Stop a playbook
    ///
    /// # Arguments
    ///
    /// `playbook_id`: The playbook id
    pub fn stop(&self, playbook_id: &str) -> Result<u16, ClientError> {
        let path = format!("/playbooks/{}/actions/stop", playbook_id);
        Ok(self.client.empty_post(&path)?.status)
    }
}
