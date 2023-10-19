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

use amp_common::http::{Client, Endpoint, HTTPError};
use amp_common::sync::Synchronization;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Actor {
    /// The actor id
    pub id: u64,
    /// The title of the actor
    pub title: String,
    /// The description of the actor
    pub description: String,
    /// When the actor was created in Amphitheatre.
    pub created_at: String,
    /// When the actor was last updated in Amphitheatre.
    pub updated_at: String,
}

struct ActorEndpoint;

impl Endpoint for ActorEndpoint {
    type Output = Actor;
}

struct ActorsEndpoint;

impl Endpoint for ActorsEndpoint {
    type Output = Vec<Actor>;
}

struct ValueEndpoint;

impl Endpoint for ValueEndpoint {
    type Output = Value;
}

/// The Actors Service handles the actors endpoint of the Amphitheatre API.
///
/// See [API Documentation: playbook](https://docs.amphitheatre.app/api/actor)
pub struct Actors<'a> {
    pub client: &'a Client,
}

impl Actors<'_> {
    /// Lists the actors of playbook.
    ///
    /// # Arguments
    ///
    /// `playbook_id`: The playbook id
    /// `options`: The `HashMap<String, String>`
    ///             - Sort: `id`, `label`, `email`
    pub fn list(
        &self,
        playbook_id: &str,
        options: Option<HashMap<String, String>>,
    ) -> Result<Vec<Actor>, HTTPError> {
        let path = format!("/playbooks/{}/actors", playbook_id);
        let res = self.client.get::<ActorsEndpoint>(&path, options)?;
        Ok(res.data.unwrap())
    }

    /// Retrieve a actor
    ///
    /// # Arguments
    ///
    /// `pid`: The ID of the playbook
    /// `name`: The name of the actor
    pub fn get(&self, pid: &str, name: &str) -> Result<Actor, HTTPError> {
        let path = format!("/actors/{}/{}", pid, name);
        let res = self.client.get::<ActorEndpoint>(&path, None)?;
        Ok(res.data.unwrap())
    }

    /// Retrieve the log streams of actor
    ///
    /// # Arguments
    ///
    /// `pid`: The ID of the playbook
    /// `name`: The name of the actor
    pub fn logs(&self, _pid: &str, _name: &str) -> String {
        // let path = format!("/actors/{}/{}/logs", pid, name);
        String::from("event stream (JSON)")
    }

    /// Retrieve actor's info, including enviroments, volumes...
    ///
    /// # Arguments
    ///
    /// `pid`: The ID of the playbook
    /// `name`: The name of the actor
    pub fn info(&self, pid: &str, name: &str) -> Result<Value, HTTPError> {
        let path = format!("/actors/{}/{}/info", pid, name);
        let res = self.client.get::<ValueEndpoint>(&path, None)?;
        Ok(res.data.unwrap())
    }

    /// Retrieve actor's stats
    ///
    /// # Arguments
    ///
    /// `pid`: The ID of the playbook
    /// `name`: The name of the actor
    pub fn stats(&self, pid: &str, name: &str) -> Result<Value, HTTPError> {
        let path = format!("/actors/{}/{}/stats", pid, name);
        let res = self.client.get::<ValueEndpoint>(&path, None)?;
        Ok(res.data.unwrap())
    }

    /// Sync the actor's source code
    ///
    /// # Arguments
    ///
    /// `pid`: The ID of the playbook
    /// `name`: The name of the actor
    pub fn sync(&self, pid: &str, name: &str, payload: Synchronization) -> Result<u16, HTTPError> {
        let path = format!("/actors/{}/{}/sync", pid, name);
        match serde_json::to_value(payload) {
            Ok(json) => {
                let res = self
                    .client
                    ._agent
                    .post(&self.client.url(&path))
                    .send_json(json)
                    .map_err(|e| HTTPError::Deserialization(e.to_string()))?;
                Ok(res.status())
            }
            Err(_) => Err(HTTPError::Deserialization(String::from(
                "Cannot deserialize json payload",
            ))),
        }
    }
}
