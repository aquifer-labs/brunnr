// SPDX-License-Identifier: Apache-2.0

use brunnr_test_support::TempDir;
use hvergelmir::{ScratchWorkspaceProvider, WorkspaceProvider};

#[tokio::test]
async fn concurrent_mock_workers_receive_isolated_workspaces() {
    let tempdir = TempDir::new("workspace-provider");
    let repo = tempdir.join("repo");
    std::fs::create_dir_all(&repo).expect("repo dir should be created");
    std::fs::write(repo.join("shared.txt"), "source").expect("source file should be written");
    let provider = ScratchWorkspaceProvider::new(tempdir.join("scratch"));

    let left = provider
        .lease(&repo, "worker-a")
        .await
        .expect("left lease should succeed");
    let right = provider
        .lease(&repo, "worker-b")
        .await
        .expect("right lease should succeed");

    std::fs::write(left.path.join("output.txt"), "left").expect("left output should be written");
    std::fs::write(right.path.join("output.txt"), "right").expect("right output should be written");

    assert_ne!(left.path, right.path);
    assert_eq!(
        std::fs::read_to_string(repo.join("shared.txt")).expect("repo source should remain"),
        "source"
    );
    assert_eq!(
        std::fs::read_to_string(left.path.join("output.txt")).expect("left output should exist"),
        "left"
    );
    assert_eq!(
        std::fs::read_to_string(right.path.join("output.txt")).expect("right output should exist"),
        "right"
    );

    left.cleanup().expect("left cleanup should succeed");
    right.cleanup().expect("right cleanup should succeed");
    assert!(!left.path.exists());
    assert!(!right.path.exists());
}
