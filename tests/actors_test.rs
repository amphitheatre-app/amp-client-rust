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

use amp_common::sync::{EventKinds, Path, Synchronization};
use futures::StreamExt;
use reqwest_eventsource::Event;

use crate::common::{setup_async_mock_for, setup_mock_for};
mod common;

#[test]
fn list_actors_test() {
    let pid = "1";
    let setup = setup_mock_for(
        format!("/playbooks/{}/actors", pid).as_str(),
        "actors/list-actors-success",
        "GET",
    );
    let client = setup.0;

    let actors = client.actors().list(pid, None).unwrap();

    assert_eq!(1, actors.len());

    let actor = actors.first().unwrap();

    assert_eq!("amp-example-go", actor.name);
}

#[test]
fn get_actor_test() {
    let setup = setup_mock_for("/actors/1/hello", "actors/get-actor-success", "GET");
    let client = setup.0;
    let pid = "1";
    let name = "hello";

    let actor = client.actors().get(pid, name).unwrap();

    assert_eq!("amp-example-go", actor.name);
}

#[tokio::test]
async fn get_actor_logs() {
    let setup = setup_async_mock_for("/actors/1/hello/logs", "actors/get-actor-logs-success", "GET").await;
    let client = setup.0;
    let pid = "1";
    let name = "hello";

    let mut es = client.actors().logs(pid, name);

    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Open) => println!("Connection Open!"),
            Ok(Event::Message(message)) => println!("Message: {:#?}", message),
            Err(err) => {
                println!("Error: {}", err);
                es.close();
            }
        }
    }
}

#[test]
fn get_actor_info_test() {
    let setup = setup_mock_for("/actors/1/hello/info", "actors/get-actor-info-success", "GET");
    let client = setup.0;
    let pid = "1";
    let name = "hello";

    let json = client.actors().info(pid, name).unwrap();

    assert_eq!("RdqNLMXRiRsHJhmxKurR", json["environments"]["K3S_TOKEN"]);
    assert_eq!(
        "/var/lib/docker/volumes/f64c2f2cf81cfde89879f2a17924b31bd2f2e6a6a738f7df949bf6bd57102d25/_data",
        json["mounts"]["/VAR/LOG"]
    );
    assert_eq!("0.0.0.0:42397", json["port"]["6443/tcp"]);
}

#[test]
fn get_actor_stats_test() {
    let setup = setup_mock_for("/actors/1/hello/stats", "actors/get-actor-stats-success", "GET");
    let client = setup.0;
    let pid = "1";
    let name = "hello";

    let json = client.actors().stats(pid, name).unwrap();

    assert_eq!("1.98%", json["CPU USAGE"]);
    assert_eq!("5.3MB / 43.7 MB", json["DISK READ/WRITE"]);
    assert_eq!("65.8MB", json["MEMORY USAGE"]);
    assert_eq!("5.7 kB / 3 kB", json["NETWORK I/O"]);
}

#[test]
fn sync_actor_test() {
    let setup = setup_mock_for("/actors/1/hello/sync", "actors/sync-actor-success", "POST");
    let client = setup.0;
    let pid = "1";
    let name = "hello";

    let payload = Synchronization {
        kind: EventKinds::Create,
        paths: vec![Path::File(String::from("a.txt"))],
        attributes: None,
        payload: None,
    };

    let response = client.actors().sync(pid, name, payload);
    println!("{:?}", response);
    assert!(response.is_ok());
    assert_eq!(202, response.unwrap());
}
