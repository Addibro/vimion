# VimQuest ⚔

A dungeon RPG for learning Vim motions — inspired by vim-adventures.com but playable locally in your terminal!

## 🎮 How to Run

```bash
cargo run --release
```

Or build and run the binary directly:

```bash
cargo build --release
./target/release/vim-quest
```

## 🗡️ Gameplay

You are a **Vim Warrior** descending through **5 dungeon levels** to slay the Dragon.

### Controls (always available)

| Key | Action |
|-----|--------|
| `h` | Move left |
| `j` | Move down |
| `k` | Move up |
| `l` | Move right |
| `p` | Use health potion |
| `?` | Toggle help screen |
| `q` | Quit |

### 📜 Unlockable Vim Motions

Find **Scrolls of Knowledge** (`?` on the map) to unlock powerful Vim motions!

| Motion | What it does |
|--------|-------------|
| `w`    | Jump forward one open section |
| `b`    | Jump backward one section |
| `e`    | Jump to end of open section |
| `0`    | Teleport to start of current row |
| `$`    | Teleport to end of current row |
| `gg`   | Teleport to top of dungeon |
| `G`    | Teleport to bottom of dungeon |
| `H`    | Jump to top area of screen |
| `M`    | Jump to middle of screen |
| `L`    | Jump to bottom area of screen |

### 🗺 Map Symbols

```
@ — You (the Vim Warrior)
# — Wall
. — Floor
> — Stairs (next level)
E — Exit (victory!)
+ — Door (closed)
/ — Door (open)
* — Torch

! — Health Potion    $ — Gold
/ — Sword            ) — Shield
k — Key              ? — Scroll of Knowledge

g — Goblin   O — Orc     s — Slime
k — Skeleton T — Troll   D — Dragon (boss!)
```

### 💡 Tips

- Enemies move toward you when visible — use Vim motions to outmaneuver them!
- `p` to use a health potion (picked up automatically)
- Defeating enemies gives XP and gold; level up to grow stronger
- Later levels spawn tougher enemies — unlock motions early for an edge
- The Dragon awaits on level 5 — reach the `E` exit to win!

## 🦀 Built With

- [Rust](https://www.rust-lang.org/)
- [ratatui](https://ratatui.rs/) — TUI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — terminal I/O
- [rand](https://docs.rs/rand/) — procedural dungeon generation

## Requirements

- Rust 1.65+ (install via [rustup](https://rustup.rs/))
- A terminal at least 80×30 characters (recommended: 120×40)
