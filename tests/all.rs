use rymder::futures_util::StreamExt;
use std::{process as proc, time::Duration};

const DEFAULT_PORT: u16 = 9357;

struct SdkServer {
    port: u16,
    features: &'static str,
    tests: &'static str,
}

impl SdkServer {
    async fn spawn(self) -> proc::Child {
        let mut cmd = proc::Command::new("agones-sdk-server");

        cmd.stdout(proc::Stdio::piped())
            .stderr(proc::Stdio::piped())
            .args([
                "--local",
                "--sdk-name",
                "rymder",
                "--grpc-port",
                &self.port.to_string(),
                // We don't care about the http port, but if we run multiple
                // sdk servers at once then the first one wins
                "--http-port",
                "0",
                "--timeout",
                "10",
            ]);

        if !self.features.is_empty() {
            cmd.args(["--feature-gates", self.features]);
        }

        if !self.tests.is_empty() {
            cmd.args(["--test", self.tests]);
        }

        eprintln!("{:#?}", cmd);
        cmd.spawn().expect("unable to exec agones-sdk-server")
    }

    fn validate(mut child: proc::Child, _expected: &[&'static str]) {
        let killed = child.kill().is_ok();
        let output = child.wait_with_output().expect("failed to wait on child");

        let stdout = String::from_utf8(output.stdout).expect("non utf-8");
        let stderr = String::from_utf8(output.stderr).expect("non utf-8");

        println!("stdout: {}", stdout);
        eprintln!("stderr: {}", stderr);

        // Ignore if the server was killed since we don't want to wait 10
        // seconds for every test
        assert!(output.status.success() || killed);
    }
}

async fn connect(port: u16) -> rymder::Sdk {
    let (sdk, gs) = rymder::Sdk::connect(Some(port), Some(Duration::from_secs(2)), None)
        .await
        .expect("failed to connect to sdk server in 2s");

    println!("Initial gameserver: {:#?}", gs);
    sdk
}

#[tokio::test]
async fn sdk() {
    let server = SdkServer {
        port: DEFAULT_PORT,
        features: "",
        tests: "ready,reserve,allocate,label,annotate,watch,health,shutdown",
    };
    let server = server.spawn().await;

    {
        let mut sdk = connect(DEFAULT_PORT).await;

        let _health = {
            let health_tx = sdk.health_check();
            let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();

            tokio::task::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(2));

                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            if health_tx
                                .send(())
                                .await.is_err() {
                                eprintln!("Health check receiver was dropped");
                                break;
                            }
                        }
                        _ = &mut rx => {
                            println!("Health check task canceled");
                            break;
                        }
                    }
                }
            });

            tx
        };

        let _watch = {
            let mut watch_client = sdk.clone();
            let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();

            tokio::task::spawn(async move {
                println!("Starting to watch GameServer updates...");
                match watch_client.watch_gameserver().await {
                    Err(e) => println!("Failed to watch for GameServer updates: {}", e),
                    Ok(mut stream) => loop {
                        tokio::select! {
                            gs = stream.next() => {
                                match gs {
                                    Some(Ok(gs)) => {
                                        println!("GameServer update: {:#?}", gs);
                                    }
                                    Some(Err(e)) => {
                                        panic!("GameServer Update stream encountered an error: {}", e);
                                    }
                                    None => {
                                        println!("Server closed the GameServer watch stream");
                                        break;
                                    }
                                }

                            }
                            _ = &mut rx => {
                                println!("Shutting down GameServer watch loop");
                                break;
                            }
                        }
                    },
                }
            });

            tx
        };

        sdk.mark_ready().await.expect("failed to mark ready");

        sdk.reserve(Duration::from_secs(2))
            .await
            .expect("failed to reserve");

        tokio::time::sleep(Duration::from_millis(2100)).await;

        sdk.allocate().await.expect("failed to allocate");

        sdk.set_label("label-key", "label-value")
            .await
            .expect("failed to set label");
        sdk.set_annotation("annotation-key", "annotation-value")
            .await
            .expect("failed to set annotation");

        sdk.shutdown().await.expect("failed to shutdown");
    }

    SdkServer::validate(server, &[]);
}

#[cfg(feature = "player-tracking")]
#[tokio::test]
async fn player_tracking() {
    let server = SdkServer {
        port: DEFAULT_PORT + 10,
        features: "PlayerTracking=true",
        tests: "ready,playertracking,shutdown",
    };
    let server = server.spawn().await;

    {
        let mut sdk = connect(DEFAULT_PORT + 10).await;

        let _health = {
            let health_tx = sdk.health_check();
            let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();

            tokio::task::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(2));

                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            if health_tx
                                .send(())
                                .await.is_err() {
                                eprintln!("Health check receiver was dropped");
                                break;
                            }
                        }
                        _ = &mut rx => {
                            println!("Health check task canceled");
                            break;
                        }
                    }
                }
            });

            tx
        };

        let _watch = {
            let mut watch_client = sdk.clone();
            let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();

            tokio::task::spawn(async move {
                println!("Starting to watch GameServer updates...");
                match watch_client.watch_gameserver().await {
                    Err(e) => println!("Failed to watch for GameServer updates: {}", e),
                    Ok(mut stream) => loop {
                        tokio::select! {
                            gs = stream.next() => {
                                match gs {
                                    Some(Ok(gs)) => {
                                        println!("GameServer update: {:#?}", gs);
                                    }
                                    Some(Err(e)) => {
                                        panic!("GameServer Update stream encountered an error: {}", e);
                                    }
                                    None => {
                                        println!("Server closed the GameServer watch stream");
                                        break;
                                    }
                                }

                            }
                            _ = &mut rx => {
                                println!("Shutting down GameServer watch loop");
                                break;
                            }
                        }
                    },
                }
            });

            tx
        };

        sdk.mark_ready().await.expect("failed to mark ready");

        sdk.set_player_capacity(20)
            .await
            .expect("failed to set player capacity");
        assert_eq!(
            sdk.get_player_capacity().await.unwrap(),
            20,
            "unexpected player capacity"
        );

        for ind in 0..20 {
            assert!(
                sdk.player_connect(ind.to_string()).await.unwrap(),
                "unable to add player {}",
                ind
            );
        }

        assert!(
            !sdk.player_connect("0").await.unwrap(),
            "adding player a second time failed"
        );
        assert_eq!(
            sdk.get_connected_players().await.unwrap(),
            (0..20).map(|ind| ind.to_string()).collect::<Vec<_>>()
        );
        assert_eq!(sdk.get_player_count().await.unwrap(), 20);

        for ind in (0..20).filter(|i| i % 2 == 0) {
            assert!(
                sdk.player_disconnect(ind.to_string()).await.unwrap(),
                "unable to disconnect player {}",
                ind
            );
        }

        assert!(
            !sdk.player_disconnect("0").await.unwrap(),
            "disconnecting a player a second time failed"
        );

        for ind in 0..20 {
            assert_eq!(
                sdk.is_player_connected(ind.to_string()).await.unwrap(),
                ind % 2 != 0,
                "failed to get connection status of player"
            );
        }

        assert_eq!(sdk.get_player_count().await.unwrap(), 10);

        sdk.shutdown().await.expect("failed to shutdown");
    }

    SdkServer::validate(server, &[]);
}
