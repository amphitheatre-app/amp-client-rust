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
//
// Heavily inspired by https://github.com/dnsimple/dnsimple-rust

use serde::{Deserialize, Serialize};

pub mod accounts;
pub mod actors;
pub mod client;
pub mod oauth;
pub mod playbooks;

#[derive(Debug, Deserialize, Serialize)]
pub struct Wrapper<T> {
    pub data: T,
    /// Any API endpoint that returns a list of items requires pagination.
    pub pagination: Option<Pagination>,
}

/// Any API endpoint that returns a list of items requires pagination.
/// By default we will return 30 records from any listing endpoint. If an API
/// endpoint returns a list of items, then it will include a pagination object
/// that contains pagination information.
#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    /// The page currently returned (default: 1)
    pub current_page: u64,
    /// The number of entries returned per page (default: 30)
    pub per_page: u64,
    /// The Total number of entries available in the entrire collection.
    pub total_entries: u64,
    /// The total number of pages available given the current `per_page` value
    pub total_pages: u64,
}
