### New Feature: Pinned Paths in the Path Picker

The **Select Path** window (`Enter` on the Path field in the form) has been reworked:

- The window is now split into three sections:
  - **Pinned paths:** — your pinned folders, always on top;
  - **Paths:** — all other saved folders;
  - **Actions:** — `Pin path`, `+ Add current path`, `+ Add new path…`, `− Select and delete path…`.
- Paths are marked with a dim label next to them:
  - **Default** — the default download path;
  - **Selected** — the path currently selected in the form;
  - **Selected by default** — both at once.
- Pinned folders are saved to `config.toml` (`[paths] pinned = [...]`). The key is optional — existing configs keep working.
- Removing a folder from favorites also removes it from pinned.

### New Pin Manager

The **Pin path** button (first in the **Actions** section) opens a separate **Pin Paths** window:

**Pinned paths:**
- Folders that are currently pinned.

**Paths:**
- Folders that are not pinned.

Pressing `Enter` on a path moves it from one category to the other.

### New Keybindings for Path Picker

**Select Path:**
- `P`                                       Pin / unpin the highlighted path.
- `Enter`                               Select the highlighted path or run the highlighted action.
- `↑` / `↓`                             Move between paths and actions.

**Pin Paths:**
- `↑` / `↓`                             Select a path.
- `Enter` / `P`                     Move the path between pinned and regular categories.
- `Esc` / `q`                         Back to the path picker.

### Updated
- In-app help (`F1`) updated: new path picker section and a separate pin manager section.
- `README.md` updated: pinned paths added to features, new keyboard shortcut tables, `pinned` added to the example `config.toml`.
- Packaging artifacts (`pkg/`, `*.pkg.tar.zst`, `*.tar.gz`) are now ignored by git.
