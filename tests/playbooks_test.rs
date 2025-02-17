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

use amp_client::playbooks::PlaybookPayload;
use amp_common::resource::Preface;
use common::mock;
mod common;

#[tokio::test]
async fn list_playbooks_test() {
    let setup = mock("/playbooks", "playbooks/list-playbooks-success", "GET").await;
    let client = setup.0;

    let playbooks = client.playbooks().list(None).await.unwrap();

    assert_eq!(1, playbooks.len());

    let playbook = playbooks.first().unwrap();

    assert_eq!("a82abba3-df2f-4608-b1a5-9e058ff80468", playbook.id);
    assert_eq!("Untitled", playbook.title);
    assert_eq!(Some("".into()), playbook.description);
    // assert_eq!("2016-01-19T20:50:26Z", playbook.created_at);
    // assert_eq!("2016-01-19T20:50:26Z", playbook.updated_at);
}

#[tokio::test]
async fn create_playbook_test() {
    let setup = mock("/playbooks", "playbooks/create-playbook-created", "POST").await;
    let client = setup.0;

    let payload = PlaybookPayload {
        title: String::from("Untitled"),
        description: String::from(""),
        preface: Preface::default(),
    };

    let playbook = client.playbooks().create(payload).await.unwrap();

    assert_eq!("a82abba3-df2f-4608-b1a5-9e058ff80468", playbook.id);
    assert_eq!("Untitled", playbook.title);
    assert_eq!(Some("".into()), playbook.description);
    // assert_eq!("2016-01-19T20:50:26Z", playbook.created_at);
    // assert_eq!("2016-01-19T20:50:26Z", playbook.updated_at);
}

#[tokio::test]
async fn get_playbook_test() {
    let setup = mock(
        "/playbooks/a82abba3-df2f-4608-b1a5-9e058ff80468",
        "playbooks/get-playbook-success",
        "GET",
    )
    .await;
    let client = setup.0;
    let playbook_id = "a82abba3-df2f-4608-b1a5-9e058ff80468";

    let playbook = client.playbooks().get(playbook_id).await.unwrap();

    assert_eq!("a82abba3-df2f-4608-b1a5-9e058ff80468", playbook.id);
    assert_eq!("Untitled", playbook.title);
    assert_eq!(Some("".into()), playbook.description);
    // assert_eq!("2016-01-19T20:50:26Z", playbook.created_at);
    // assert_eq!("2016-01-19T20:50:26Z", playbook.updated_at);
}

#[tokio::test]
async fn update_playbook_test() {
    let setup = mock(
        "/playbooks/a82abba3-df2f-4608-b1a5-9e058ff80468",
        "playbooks/update-playbook-success",
        "PATCH",
    )
    .await;
    let client = setup.0;
    let playbook_id = "a82abba3-df2f-4608-b1a5-9e058ff80468";

    let payload = PlaybookPayload {
        title: String::from("Untitled"),
        description: String::from(""),
        preface: Preface::default(),
    };

    let playbook = client.playbooks().update(playbook_id, payload).await.unwrap();

    assert_eq!("a82abba3-df2f-4608-b1a5-9e058ff80468", playbook.id);
    assert_eq!("Untitled", playbook.title);
    assert_eq!(Some("".into()), playbook.description);
    // assert_eq!("2016-01-19T20:50:26Z", playbook.created_at);
    // assert_eq!("2016-01-19T20:50:26Z", playbook.updated_at);
}

#[tokio::test]
async fn delete_playbook_test() {
    let setup = mock(
        "/playbooks/a82abba3-df2f-4608-b1a5-9e058ff80468",
        "playbooks/delete-playbook-success",
        "DELETE",
    )
    .await;
    let client = setup.0;
    let playbook_id = "a82abba3-df2f-4608-b1a5-9e058ff80468";

    let response = client.playbooks().delete(playbook_id).await;

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap());
}

#[tokio::test]
async fn get_playbook_events() {
    let setup = mock(
        "/playbooks/a82abba3-df2f-4608-b1a5-9e058ff80468/events",
        "playbooks/get-playbook-events-success",
        "GET",
    )
    .await;
    let client = setup.0;
    let playbook_id = "a82abba3-df2f-4608-b1a5-9e058ff80468";

    let response = client.playbooks().events(playbook_id);

    assert_eq!(String::from("event stream (JSON)"), response);
}

#[tokio::test]
async fn start_playbook_test() {
    let setup = mock(
        "/playbooks/a82abba3-df2f-4608-b1a5-9e058ff80468/actions/start",
        "playbooks/start-playbook-success",
        "POST",
    )
    .await;
    let client = setup.0;
    let playbook_id = "a82abba3-df2f-4608-b1a5-9e058ff80468";

    let response = client.playbooks().start(playbook_id).await;

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap());
}

#[tokio::test]
async fn stop_playbook_test() {
    let setup = mock(
        "/playbooks/a82abba3-df2f-4608-b1a5-9e058ff80468/actions/stop",
        "playbooks/stop-playbook-success",
        "POST",
    )
    .await;
    let client = setup.0;
    let playbook_id = "a82abba3-df2f-4608-b1a5-9e058ff80468";

    let response = client.playbooks().stop(playbook_id).await;

    assert!(response.is_ok());
    assert_eq!(204, response.unwrap());
}
