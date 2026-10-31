use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dependency {
    YtDlp,
    Ffmpeg,
}

impl Dependency {
    pub fn name(&self) -> &'static str {
        match self {
            Dependency::YtDlp => "yt-dlp",
            Dependency::Ffmpeg => "ffmpeg",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Dependency::YtDlp => "video downloader (required)",
            Dependency::Ffmpeg => "media converter for merging and audio (required)",
        }
    }

    pub fn download_url(&self) -> &'static str {
        match self {
            Dependency::YtDlp => "https://github.com/yt-dlp/yt-dlp/releases/latest",
            Dependency::Ffmpeg => "https://ffmpeg.org/download.html",
        }
    }

    pub fn action(&self) -> &'static str {
        match self {
            Dependency::YtDlp => "download automatically",
            Dependency::Ffmpeg => "open the download page",
        }
    }
}

pub fn is_installed(dep: Dependency) -> bool {
    match dep {
        Dependency::YtDlp => local_yt_dlp_path().is_file() || command_ok("yt-dlp", "--version"),
        Dependency::Ffmpeg => command_ok("ffmpeg", "-version"),
    }
}

pub fn missing() -> Vec<Dependency> {
    [Dependency::YtDlp, Dependency::Ffmpeg]
        .into_iter()
        .filter(|dep| !is_installed(*dep))
        .collect()
}

pub fn yt_dlp_program() -> PathBuf {
    let local = local_yt_dlp_path();
    if local.is_file() {
        local
    } else {
        PathBuf::from("yt-dlp")
    }
}

pub fn download(dep: Dependency) -> Result<String> {
    match dep {
        Dependency::YtDlp => download_yt_dlp(),
        Dependency::Ffmpeg => {
            open_url(dep.download_url())?;
            Ok("opened the download page".into())
        }
    }
}

fn command_ok(program: &str, arg: &str) -> bool {
    Command::new(program)
        .arg(arg)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn bin_dir() -> PathBuf {
    dirs::data_local_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| PathBuf::from("."))
        .join("yt-dlp-wrapper-tui")
        .join("bin")
}

fn local_yt_dlp_path() -> PathBuf {
    let name = if cfg!(windows) {
        "yt-dlp.exe"
    } else {
        "yt-dlp"
    };
    bin_dir().join(name)
}

fn download_yt_dlp() -> Result<String> {
    let dir = bin_dir();
    std::fs::create_dir_all(&dir).with_context(|| format!("cannot create {}", dir.display()))?;
    let target = local_yt_dlp_path();
    let url = if cfg!(windows) {
        "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe"
    } else {
        "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp"
    };

    if try_download("curl", &target, url) || try_download("wget", &target, url) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&target)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&target, perms)?;
        }
        Ok(format!("downloaded to {}", target.display()))
    } else {
        open_url(url)?;
        Ok("opened the download page".into())
    }
}

fn try_download(program: &str, target: &PathBuf, url: &str) -> bool {
    let mut cmd = Command::new(program);
    if program == "curl" {
        cmd.arg("-fsSL").arg("-o").arg(target);
    } else {
        cmd.arg("-q").arg("-O").arg(target);
    }
    cmd.arg(url).stdout(Stdio::null()).stderr(Stdio::null());

    match cmd.status() {
        Ok(status) if status.success() && target.is_file() => true,
        _ => {
            let _ = std::fs::remove_file(target);
            false
        }
    }
}

fn open_url(url: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    let mut cmd = {
        let mut c = Command::new("cmd");
        c.args(["/C", "start", "", url]);
        c
    };
    #[cfg(target_os = "macos")]
    let mut cmd = {
        let mut c = Command::new("open");
        c.arg(url);
        c
    };
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut cmd = {
        let mut c = Command::new("xdg-open");
        c.arg(url);
        c
    };

    cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| format!("cannot open {}", url))?;
    Ok(())
}
