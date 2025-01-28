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

use common::mock;

mod common;

#[tokio::test]
async fn me_success_with_account() {
    let setup = mock("/me", "accounts/get-me-success", "GET").await;
    let client = setup.0;
    let account = client.accounts().me().await.unwrap();

    assert_eq!(1, account.id);
    assert_eq!("example-account@example.com", account.email);
    assert_eq!("example-account", account.name);
}
