# yt-dlp-wrapper-tui

A terminal UI (TUI) for [yt-dlp](https://github.com/yt-dlp/yt-dlp): a convenient form for adding links, a download queue with live progress, and settings that are saved to a config file.

## Features

- Add links to the queue directly from the interface.
- **Video** and **Audio** modes:
  - Video — choose quality (Best, 1080p, 720p, 480p, 360p) and merge into MP4;
  - Audio — extract to MP3 with embedded thumbnail and metadata.
- Download a full playlist or a single video.
- Sequential queue: items are processed one at a time.
- Real-time progress: percentage, speed, and a progress bar.
- Video titles are fetched with `yt-dlp --get-title`.
- Queue management: pause, resume, retry failed downloads, remove items, and cancel active downloads.
- Favorite folders for download destinations and a path picker.
- Built-in settings (`F2`):
  - defaults for new downloads;
  - extra yt-dlp options: proxy, geo-bypass, cookies, user-agent, rate limit, and custom arguments;
  - status bar configuration;
  - queue table configuration: visible columns, order, fixed width/weight, and alignment.
- All settings changes are saved to `config.toml` immediately.
- Built-in keyboard shortcut help (`F1`).

## Requirements

- `yt-dlp` available in `PATH`;
- `ffmpeg` available in `PATH` (for merging video streams and MP3 conversion);
- Rust 1.89+ — only needed to build from source (the project uses edition 2024).

## Installation

### From source

```bash
git clone https://github.com/Lycoris-69-rs/yt-dlp-wrapper-tui.git
cd yt-dlp-wrapper-tui
cargo build --release --locked
./target/release/yt-dlp-wrapper-tui
```

Or install it into `~/.cargo/bin`:

```bash
cargo install --path .
```

### Arch Linux

The repository includes `PKGBUILD` (stable release) and `PKGBUILD-git` (git build). Before using them, replace `USERNAME` in the `_gh` variable with the actual GitHub account.

```bash
makepkg -si
```

## Running

```bash
yt-dlp-wrapper-tui
```

You can also run it without installing:

```bash
cargo run --release
```

## Keyboard shortcuts

### General

| Key | Action |
| --- | --- |
| `F1` | Open/close help |
| `F2` | Open/close settings |
| `Tab` | Switch focus between the queue and the form |
| `Esc` | Close a menu/popup or leave URL editing |
| `Ctrl+C` | Quit the application |
| `q` | Quit (when the queue is focused) |

### New download form

| Key | Action |
| --- | --- |
| `↑` / `↓`, `←` / `→` | Previous/next field |
| `Enter` | Activate a field, start URL editing, or add to the queue |
| `Space` | Quickly toggle type, playlist, or quality |
| `Shift+Enter` | Previous quality |
| `Ctrl+S` | Add to the queue (when the URL is not being edited) |

While editing the URL: `←`/`→` move the cursor, `Home`/`End` jump to the start/end, and `Backspace`/`Delete` remove characters.

### Queue

| Key | Action |
| --- | --- |
| `↑` / `↓` | Select an item |
| `d` / `Delete` | Remove an item; an active download will be cancelled |
| `r` | Retry a failed download |
| `s` | Pause/resume the queue |

### Help

| Key | Action |
| --- | --- |
| `↑` / `↓` | Scroll |
| `PageUp` / `PageDown` | Scroll by 10 lines |
| `Home` | Jump to the top |
| `Esc`, `q` | Close |

## Settings

Open settings with `F2`. There are three sections:

- **Defaults**
  - *General* — default download type, quality, playlist behavior, and path;
  - *Advanced* — proxy, geo-bypass, cookies, user-agent, rate limit, and extra yt-dlp arguments.
- **Status Bar** — visibility of the status bar and its individual elements.
- **Queue** — table columns:
  - *Columns* — show/hide columns and change their order;
  - *Widths* — fixed width or automatic (`AUTO`) width with a weight;
  - *Style* — content alignment (Left / Center / Right).

Advanced options apply to new queue items. Default values are used when the application starts.

### Config location

| OS | Path |
| --- | --- |
| Linux | `~/.config/yt-dlp-wrapper-tui/config.toml` |
| macOS | `~/Library/Application Support/yt-dlp-wrapper-tui/config.toml` |

### Example `config.toml`

```toml
[general]
default_path = "/home/user/Downloads"
default_quality = "Best"
default_mode = "Video"
default_playlist = false

[general.advanced]
proxy = ""
geo_bypass = false
cookies = ""
user_agent = ""
rate_limit = ""
extra_args = ""

[paths]
favorites = ["/home/user/Downloads"]

[status_bar]
enabled = true
show_hints = true
show_queue_stats = true
show_active = true
show_status_message = true

[queue_table]
columns = ["num", "name", "url", "progress", "percent", "status"]

[queue_table.widths]
num = 1
name = 10
url = 10
progress = 10
percent = 10
status = 10

[queue_table.weights]
num = 1
name = 5
url = 5
progress = 5
percent = 5
status = 5

[queue_table.auto_width]
name = true
url = true
progress = true
percent = true
status = true

[queue_table.alignment]
num = "Right"
name = "Left"
url = "Left"
progress = "Left"
percent = "Left"
status = "Left"
```

Available columns: `num`, `added`, `name`, `url`, `mode`, `quality`, `status`, `progress`, `percent`, `speed`.

## How it works

1. You enter a link in the form and press **Add to Queue**.
2. The application runs `yt-dlp --get-title` to display the video title.
3. Queue items are downloaded one at a time. Video uses the selected format and `--merge-output-format mp4`; audio uses `-x --audio-format mp3`.
4. Progress is read from yt-dlp's output and displayed in the table.
5. The next item starts automatically when the current one finishes.

## Limitations

- The queue is kept in memory and is not persisted between runs.
- Only one download runs at a time.
- `yt-dlp` and `ffmpeg` must be installed on the system.
- Thumbnails and metadata are embedded only if the installed `ffmpeg` supports the required features.

## Project structure

```
src/
├── main.rs          # application startup and event loop
├── app/             # state, key handling, form, and settings
├── config/          # config.toml model and load/save logic
├── ui/              # interface rendering (ratatui)
├── downloader.rs    # yt-dlp invocation and progress parsing
├── event.rs         # keyboard and download events
└── queue.rs         # queue and quality model
```
