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

use amp_common::client::{Client, ClientError, Endpoint};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Wrapper;

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
    type Output = Wrapper<Actor>;
}

struct ActorsEndpoint;

impl Endpoint for ActorsEndpoint {
    type Output = Wrapper<Vec<Actor>>;
}

struct ValueEndpoint;

impl Endpoint for ValueEndpoint {
    type Output = Wrapper<Value>;
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
        playbook_id: u64,
        options: Option<HashMap<String, String>>,
    ) -> Result<Vec<Actor>, ClientError> {
        let path = format!("/playbooks/{}/actors", playbook_id);
        let res = self.client.get::<ActorsEndpoint>(&path, options)?;
        Ok(res.data.unwrap().data)
    }

    /// Retrieve a actor
    ///
    /// # Arguments
    ///
    /// `actor_id`: The ID of the actor we want to retrieve
    pub fn get(&self, actor_id: u64) -> Result<Actor, ClientError> {
        let path = format!("/actors/{}", actor_id);
        let res = self.client.get::<ActorEndpoint>(&path, None)?;
        Ok(res.data.unwrap().data)
    }

    /// Retrieve the log streams of actor
    ///
    /// # Arguments
    ///
    /// `actor_id`: The actor id
    pub fn logs(&self, _actor_id: u64) -> String {
        // let path = format!("/actors/{}/logs", actor_id);
        String::from("event stream (JSON)")
    }

    /// Retrieve actor's info, including enviroments, volumes...
    ///
    /// # Arguments
    ///
    /// `actor_id`: The actor id
    pub fn info(&self, actor_id: u64) -> Result<Value, ClientError> {
        let path = format!("/actors/{}/info", actor_id);
        let res = self.client.get::<ValueEndpoint>(&path, None)?;
        Ok(res.data.unwrap().data)
    }

    /// Retrieve actor's stats
    ///
    /// # Arguments
    ///
    /// `actor_id`: The actor id
    pub fn stats(&self, actor_id: u64) -> Result<Value, ClientError> {
        let path = format!("/actors/{}/stats", actor_id);
        let res = self.client.get::<ValueEndpoint>(&path, None)?;
        Ok(res.data.unwrap().data)
    }
}
