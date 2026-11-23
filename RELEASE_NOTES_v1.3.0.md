### New Feature: Path Column

The queue table has a new **Path** column that shows the download folder of every item.

- Enable it in **Settings → Queue → Columns** (it is listed under **Hidden** by default).
- Long paths are truncated to the column and can scroll horizontally like other text columns.

### New Feature: Horizontal Marquee

Long titles, URLs, paths, statuses, and speeds no longer just get cut off — the text scrolls inside its
column with a small gap and wraps around.

- The speed is configurable in **Settings → Queue → General → Scroll offset** (characters per tick, `1..20`).

### New Feature: Download Logs (`F3`)

`F3` opens a live viewer for the raw `yt-dlp` output.

- Each line is prefixed with the queue item id: `#id line`.
- The viewer follows new output by default.
- `↑` / `↓` scroll, `PageUp` / `PageDown` scroll by 10, `Home` jumps to the top, `End` resumes following,
  `Esc` / `q` closes.

### New Feature: Queue Settings — General Tab

**Settings → Queue** now starts with a **General** tab:

- **Scroll offset** — marquee characters per tick (`1..20`, default `1`).
- **Side padding** — spaces on each side of the table (`0..10`, default `1`). The row highlight keeps
  spanning the full inner width, including the padding zones.

### New Keybindings for the Queue

- `r` — retry the selected download.
- `d` / `Delete` — remove the item (cancels it when it is downloading).
- `p` — pause / resume the queue. Pausing cancels the current download and puts the item back to `Pending`.
- `s` — on a **running** item: skip it (pause it and start the next pending item);
  on a **pending** item: stop the current download and start this one.

Downloads now carry a run id, so late events from a cancelled process cannot overwrite the state of a
newly started item.

### Name Column

- While the title is still being fetched, the Name column shows a small rotating spinner (`-`, `/`, `|`, `\`).
- After the title arrives, only the **track name** is shown: the last part of `Artist - Album - Track`.
  Regular, en, em and minus dashes surrounded by spaces are recognised.
- After a download finishes, the Speed column shows `-`.

### Column Order

- Hiding a column and restoring it later puts it back at its previous position instead of the end.
- The cursor follows the column between the **Visible** and **Hidden** lists.

### Updated
- In-app help (`F1`) updated with the logs viewer, new queue keybindings, and the Queue → General settings.
- `README.md` updated: new columns, settings, keybindings, and the logs section.
- Packaging artifacts (`pkg/`, `*.pkg.tar.zst`, `*.tar.gz`) remain ignored by git.
