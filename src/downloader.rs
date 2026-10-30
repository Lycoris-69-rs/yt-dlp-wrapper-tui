use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::event::{AppEvent, DownloadEvent, MetaEvent};
use crate::queue::{DownloadMode, QueueItem};

pub async fn fetch_title(url: String, id: usize, tx: mpsc::UnboundedSender<AppEvent>) {
    let out = Command::new("yt-dlp")
        .args([
            "--no-warnings",
            "--skip-download",
            "--no-playlist",
            "--get-title",
            &url,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .output()
        .await;

    match out {
        Ok(o) if o.status.success() => {
            let title = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if title.is_empty() {
                let _ = tx.send(AppEvent::Meta(MetaEvent::TitleFailed { id }));
            } else {
                let _ = tx.send(AppEvent::Meta(MetaEvent::Title { id, title }));
            }
        }
        _ => {
            let _ = tx.send(AppEvent::Meta(MetaEvent::TitleFailed { id }));
        }
    }
}

pub async fn run_download(
    item: QueueItem,
    tx: mpsc::UnboundedSender<AppEvent>,
    token: CancellationToken,
) {
    let mut args: Vec<String> = Vec::new();

    if item.is_playlist {
        args.push("--yes-playlist".into());
    } else {
        args.push("--no-playlist".into());
    }

    match item.mode {
        DownloadMode::Audio => {
            for a in [
                "-x",
                "--audio-format",
                "mp3",
                "--audio-quality",
                "0",
                "--embed-thumbnail",
                "--add-metadata",
            ] {
                args.push(a.into());
            }
        }
        DownloadMode::Video => {
            args.push("-f".into());
            args.push(item.quality.format_str().into());
            args.push("--merge-output-format".into());
            args.push("mp4".into());
            args.push("--embed-thumbnail".into());
            args.push("--add-metadata".into());
        }
    }

    for a in [
        "--newline",
        "--no-color",
        "--no-abort-on-error",
        "--socket-timeout",
        "30",
        "--retries",
        "10",
        "--fragment-retries",
        "10",
        "--progress",
    ] {
        args.push(a.into());
    }

    args.push("--progress-template".into());
    args.push("download:PROGRESS|%(progress._percent_str)s|%(progress._speed_str)s".into());
    args.extend(item.advanced.to_args());
    args.push("-o".into());
    args.push(format!("{}/%(title)s.%(ext)s", item.path));
    args.push(item.url.clone());

    let _ = tx.send(AppEvent::Download(DownloadEvent::Started(item.id)));

    let mut child = match Command::new("yt-dlp")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            let _ = tx.send(AppEvent::Download(DownloadEvent::Finished {
                id: item.id,
                success: false,
                message: format!("Cannot spawn yt-dlp: {}", e),
            }));
            return;
        }
    };

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    let _ = child.kill().await;
                    break;
                }
                line_result = lines.next_line() => {
                    match line_result {
                        Ok(Some(line)) => {
                            if let Some(rest) = line.strip_prefix("PROGRESS|") {
                                let parts: Vec<&str> = rest.split('|').collect();
                                let pct = parts
                                    .first()
                                    .map(|s| s.trim().trim_end_matches('%').trim())
                                    .and_then(|s| s.parse::<f32>().ok())
                                    .unwrap_or(0.0);
                                let speed = parts
                                    .get(1)
                                    .map(|s| s.trim().to_string())
                                    .unwrap_or_default();
                                let _ = tx.send(AppEvent::Download(
                                    DownloadEvent::Progress {
                                        id: item.id,
                                        percent: pct,
                                        speed,
                                    },
                                ));
                            } else {
                                let _ = tx.send(AppEvent::Download(
                                    DownloadEvent::Log {
                                        id: item.id,
                                        line,
                                    },
                                ));
                            }
                        }
                        _ => break,
                    }
                }
            }
        }
    }

    let status = child.wait().await;
    let success = status.map(|s| s.success()).unwrap_or(false);

    let _ = tx.send(AppEvent::Download(DownloadEvent::Finished {
        id: item.id,
        success,
        message: if success {
            "OK".into()
        } else {
            "Cancelled".into()
        },
    }));
}
