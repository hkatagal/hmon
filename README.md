# ⚡ hmon (Harish's System Monitor) v0.1.0

A fast, interactive, cross-platform terminal system resource monitor written in **Rust**.

![CI/CD](https://github.com/hkatagal/hmon/actions/workflows/ci.yml/badge.svg)
![Crates.io](https://img.shields.io/crates/v/hmon.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

---

## 🌟 Features
- 💻 **Multi-Core CPU Monitor:** Per-core frequency, usage percentage, and real-time sparkline graphs.
- 🧠 **Physical RAM & Swap Memory:** Memory breakdown and rolling 60-second usage history curves.
- 🌐 **Network Bandwidth Tracking:** Live download/upload speed (KB/s), interfaces, and MAC addresses.
- 💾 **Storage Partition Stats:** Disk mount points, space utilization, and file systems.
- ⚙️ **Process Explorer & Inspector:** Interactive process table with live search (`/`), sorting by CPU/RAM/PID/Name (`s`), process inspector popup modal (`Enter`), and process termination (`Shift + K`).
- 🎨 **Theme Engine:** On-the-fly theme switching (`t`) between **Default**, **Dracula**, **Nord**, **Gruvbox**, and **Cyberpunk**.

---

## 🚀 Installation

### Option 1: Install via Cargo (Crates.io)
```bash
cargo install hmon
```

### Option 2: Install via Homebrew (macOS / Linux)
```bash
brew tap hkatagal/tap
brew install hmon
```

---

## ⌨️ Keybindings Quick Reference

| Key | Action |
| --- | --- |
| `1` - `6` | Direct Tab Navigation (1: Overview, 2: CPU, 3: RAM, 4: Processes, 5: Disks, 6: Network) |
| `Tab` / `Shift+Tab` | Next / Previous Tab |
| `j` / `k` or `Down` / `Up` | Scroll Process List |
| `Enter` | Open Process Inspector Modal |
| `s` | Cycle Sort Order (CPU % → Memory → PID → Name) |
| `/` | Live Search Filter Mode |
| `t` | Cycle Theme (Default → Dracula → Nord → Gruvbox → Cyberpunk) |
| `Shift + K` | Terminate Selected Process |
| `q` or `Esc` | Quit `hmon` |

---

## ⚙️ Configuration (`~/.config/hmon/config.toml`)

```toml
theme = "dracula"            # Options: default, dracula, nord, gruvbox, cyberpunk
refresh_interval_ms = 500   # Refresh rate in milliseconds
```

---

## 🛠️ Building From Source

```bash
git clone https://github.com/hkatagal/hmon.git
cd hmon
cargo run --release
```

---

## 📜 License
Distributed under the **MIT License**. Created by [hkatagal](https://github.com/hkatagal).
