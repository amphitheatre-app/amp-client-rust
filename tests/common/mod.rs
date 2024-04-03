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

// Heavily inspired by https://github.com/dnsimple/dnsimple-rust

use std::fs;

use amp_client::client::Client;
use mockito::{Server, ServerGuard};

/// Creates a mock server and a client (changing the url of the client
/// to that of the mock server to capture the requests).
///
/// It builds a response struct for the mock server using the fixture.
pub fn setup_mock_for(path: &str, fixture: &str, method: &str) -> (Client, ServerGuard) {
    let path = format!("/v1{}", path);
    let (status, body) = parse_fixture(fixture);

    let mut server = Server::new();
    server
        .mock(method, path.as_str())
        .with_header("x-ratelimit-limit", "2")
        .with_header("x-ratelimit-remaining", "2")
        .with_header("x-ratelimit-after", "never")
        .with_status(status)
        .with_body(body)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = Client::new(&base_url, Some("some-token".to_string()));

    (client, server)
}

#[allow(dead_code)]
pub async fn setup_async_mock_for(path: &str, fixture: &str, method: &str) -> (Client, ServerGuard) {
    let path = format!("/v1{}", path);
    let (status, body) = parse_fixture(fixture);

    let mut server = Server::new_async().await;
    server
        .mock(method, path.as_str())
        .with_header("x-ratelimit-limit", "2")
        .with_header("x-ratelimit-remaining", "2")
        .with_header("x-ratelimit-after", "never")
        .with_status(status)
        .with_body(body)
        .create_async()
        .await;

    let base_url = format!("{}/v1", server.url());
    let client = Client::new(&base_url, Some("some-token".to_string()));

    (client, server)
}

fn parse_fixture(fixture: &str) -> (usize, String) {
    let fixture = format!("./tests/fixtures/v1/api/{}.http", fixture);

    let content = fs::read_to_string(fixture.as_str()).expect("Something went wrong: Couldn't read the file");

    let lines = content.lines();
    let status = &content[9..12];
    let body = lines.last().unwrap();

    (status.parse().unwrap(), body.to_string())
}
