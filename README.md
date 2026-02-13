# welcome2u

A colorful system information display for your terminal, written in Rust. Shows a dashboard of system status every time you open a shell.

Inspired by [fancy-motd](https://github.com/bcyran/fancy-motd).

## What it shows

All segments gather data concurrently and render using [Ratatui](https://ratatui.rs/):

| Segment | Description |
|---------|-------------|
| **Heading** | ASCII art title (random Figlet font) with a fortune quote |
| **Quote** | Random quote from a fortune file |
| **User** | `username@hostname` |
| **IP** | Local IP address |
| **OS** | Operating system and version |
| **Uptime** | Time since last boot |
| **Load** | CPU load averages (1/5/15 min), color-coded by core count |
| **Temperatures** | Hardware sensor readings with status coloring |
| **Updates** | Available macOS system updates |
| **Disk** | Usage per mount point with progress bars |
| **Memory** | RAM usage with a visual gauge |
| **Docker** | Container status and uptime |

## Requirements

- **Rust** (2021 edition) — for building
- **macOS** — hardcoded paths assume macOS and Homebrew for now
- [figlet](http://www.figlet.org/) — `brew install figlet`
- [fortune](https://software.clapper.org/fortune/) — `brew install fortune`
- [Colima](https://github.com/abiosoft/colima) or Docker — for the Docker segment (optional; gracefully skipped if unavailable)

## Building

```shell
git clone https://github.com/technicalpickles/welcome2u.git
cd welcome2u
cargo build --release
```

The binary is at `target/release/welcome2u`.

## Usage

Run it directly:

```shell
./target/release/welcome2u
```

### Running at login

Add to your shell profile (`~/.profile`, `~/.zshrc`, `~/.config/fish/config.fish`, etc.):

```shell
~/path/to/welcome2u
```

### Environment variables

| Variable | Effect |
|----------|--------|
| `WELCOME2U=0` | Disable output entirely (exit immediately) |
| `MOTD_PROFILE=debug` | Enable debug logging and flame graph profiling to `log/` |

## Architecture

The codebase is organized as a Cargo workspace:

```
crates/
  segment/       # Core traits: SegmentRenderer, Info, InfoBuilder
  fortune/       # Fortune file parser
  lolcat/        # Rainbow text coloring
segments/
  heading/       # Each segment is its own crate
  disk/
  docker/
  ...
src/main.rs      # Spawns all segments concurrently, renders with Ratatui
```

Each segment implements three traits:
- `Info` — holds the gathered data
- `InfoBuilder` — async data gathering
- `SegmentRenderer` — renders into a Ratatui frame

To add a new segment: create a crate in `segments/`, implement the three traits, wire it into `main.rs`.

## Known limitations

- Paths for Figlet fonts and fortune files are hardcoded to Homebrew locations (`/opt/homebrew/opt/...`)
- Docker socket path is hardcoded to a specific Colima instance
- No configuration file yet — thresholds are set in code (e.g., memory warning at 80%, critical at 90%)
- macOS only for now

## Credits

Inspired by [fancy-motd](https://github.com/bcyran/fancy-motd) by Bazyli Cyran, which was in turn inspired by [MOTD](https://github.com/HermannBjorgvin/MOTD) by Hermann Bjorgvin.
