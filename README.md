# NS VPN

A modern, cross-platform proxy client GUI built with **Tauri 2** + **Vue 3**, powered by the **meow-rs** proxy kernel.

---

## Screenshots

<!-- TODO: Add screenshots -->

---

## Features

### Core Proxy Engine
- **meow-rs v0.19** embedded kernel — high-performance, low-memory proxy runtime
- **System Proxy** — one-click system-wide proxy enable/disable (macOS, Windows, Linux)
- **TUN Mode** — virtual network card mode for global traffic interception
- **Traffic Statistics** — real-time upload/download speed, totals, active connections
- **Connections Management** — view, filter, close individual or all connections
- **Rules Management** — browse, search, filter proxy rules

### Subscription Management
- **Clash YAML** — direct import and apply
- **v2rayN Base64** — auto-convert to Clash format (supports ss/trojan/vmess/vless/hysteria2/snell/anytls)
- **Sing-box JSON** — auto-convert to Clash format
- **Subscription URL** — remote fetch, automatic refresh
- **Local File** — import from local file system
- **Paste Content** — paste raw subscription data
- **Auto Update** — periodic subscription refresh
- **Override Config** — merge custom config on top of subscription

### Multi-Protocol Support
| Protocol | Status |
|----------|--------|
| Shadowsocks (ss) | ✅ Full |
| Trojan | ✅ Full |
| VMess | ✅ Full |
| VLESS | ✅ Full (Vision, Encryption) |
| Snell | ✅ Full |
| Hysteria2 | ✅ Full |
| AnyTLS | ✅ Full |

### User Interface
- **Dashboard** — core status, traffic chart, node/group overview, quick actions
- **Proxies** — group-based proxy selection, delay testing, mode switching
- **Rules** — rule list with search, filter, type indicators
- **Connections** — active connections table with real-time stats
- **Logs** — real-time log viewer
- **Settings** — comprehensive configuration UI
- **Multi-language** — 中文 (zh-CN), English (en)
- **Theme** — Dark/Light/System auto, 6 background themes, accent color picker
- **Tray Icon** — system tray with speed display and quick controls

### Platform Support
- **macOS** — x86_64, arm64 (Apple Silicon)
- **Windows** — x86_64, arm64
- **Linux** — x86_64, arm64 (AppImage, deb)
- **iOS** — arm64
- **Android** — arm64, x86_64

---

## Installation

### Download from Releases

1. Go to the [Releases](https://github.com/ns-vpn/ns-vpn/releases) page
2. Download the appropriate package for your platform:
   - **macOS**: `.dmg` (drag to Applications), `.zip` (manual install)
   - **Windows**: `.msi` (installer), `.exe` (NSIS installer)
   - **Linux**: `.AppImage` (run directly), `.deb` (Debian/Ubuntu)
   - **iOS**: `.ipa` (sideload or TestFlight)
   - **Android**: `.apk` (install directly)

### Build from Source

```bash
# Prerequisites
# - Node.js 20+
# - pnpm 9+
# - Rust 1.77+
# - Platform-specific deps (see Tauri 2 docs)

# Clone
git clone https://github.com/ns-vpn/ns-vpn.git
cd ns-vpn

# Install frontend deps
pnpm install

# Development
pnpm tauri:dev

# Production build (desktop)
pnpm tauri build

# Mobile builds
pnpm tauri ios build
pnpm tauri android build
```

---

## Usage

### Quick Start

1. **Add a Subscription**
   - Open the app → go to **Subscriptions** page
   - Click **New Subscription** → enter a name and URL
   - Select format: `Clash (YAML)`, `v2rayN (Base64)`, or `Sing-box (JSON)`
   - Click **Save**, then **Apply** to activate

2. **Start the Proxy**
   - On the Dashboard, click **Start Core** to launch the proxy kernel
   - The default mixed port is `7890` (configurable in Settings)

3. **Configure System Proxy** (optional)
   - Go to **Settings** → toggle **System Proxy** to route system traffic through the proxy
   - Or enable **TUN Mode** for full VPN-like traffic interception

4. **Select Proxy Node**
   - Go to **Proxies** page → select a proxy group → click a node to switch
   - Use **Test Delay** to check node latency

### Subscription Management

- **Remote URLs**: Add subscription URLs, auto-refresh with configurable intervals
- **Local Files**: Import `.yaml`, `.json`, `.txt` files from your computer
- **Paste Content**: Paste raw subscription data directly
- **Format Detection**: Supports Clash, v2rayN Base64, and Sing-box formats
- **Override Config**: Add custom config sections (rules, proxy-groups, etc.) that merge on top of subscription data

### Key Shortcuts

| Action | Shortcut |
|--------|----------|
| Show/Hide Window | `Ctrl+Shift+Space` |
| Quick Switch Proxy | `Ctrl+Shift+Q` |

---

## Architecture

```
┌──────────────────────────────────────────────────┐
│                   NS VPN GUI                      │
│  ┌────────────────────────────────────────────┐  │
│  │           Vue 3 Frontend (Tauri)            │  │
│  │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐     │  │
│  │  │Dashboard│Proxies│Rules │Settings│  ...  │  │
│  │  └──────┘ └──────┘ └──────┘ └──────┘     │  │
│  │  Pinia Store ←→ Tauri IPC (invoke/events)  │  │
│  └──────────────────┬─────────────────────────┘  │
│                     │                             │
│  ┌──────────────────▼─────────────────────────┐  │
│  │           Rust Backend (Tauri)              │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐   │  │
│  │  │  Config  │ │  Core    │ │Commands │   │  │
│  │  │  Manager │ │  Manager │ │(40+)    │   │  │
│  │  └──────────┘ └────┬─────┘ └──────────┘   │  │
│  └────────────────────┼───────────────────────┘  │
│                       │                           │
│  ┌────────────────────▼───────────────────────┐  │
│  │           meow-rs v0.19 Kernel              │  │
│  │  (HTTP API :9090)                           │  │
│  │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐     │  │
│  │  │Proxy │ │Tunnel│ │ DNS  │ │Rules │     │  │
│  │  └──────┘ └──────┘ └──────┘ └──────┘     │  │
│  └──────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────┘
```

### Key Technologies

- **Frontend**: Vue 3 (Composition API, `<script setup>`), Pinia, Tailwind CSS 4, Lucide Icons
- **Backend**: Rust/Tauri 2, 40+ registered Tauri commands
- **Kernel**: [meow-rs](https://github.com/meow-rs/meow) v0.19 — a Clash-compatible proxy kernel written in Rust
- **Config**: YAML-based (`~/.config/ns-vpn/ns-vpn.yaml` for app config, `config.yaml` for kernel config)

---

## Comparison: NS VPN vs Clash vs v2rayN vs Sing-box

### Feature Comparison

| Feature | NS VPN | Clash Verge Rev | v2rayN | Sing-box |
|---------|--------|-----------------|--------|----------|
| **GUI Framework** | Tauri 2 (Rust + Vue 3) | Tauri 1 (Rust + React) | .NET WinForms | Flutter (Android) / SwiftUI (iOS) |
| **Platform** | macOS, Windows, Linux, iOS, Android | macOS, Windows, Linux | Windows only | iOS, Android, macOS, Linux |
| **Proxy Kernel** | meow-rs (embedded) | mihomo (external) | Xray / v2fly (external) | sing-box (embedded) |
| **System Proxy** | ✅ One-click | ✅ | ✅ | ❌ (mobile only) |
| **TUN Mode** | ✅ | ✅ | ❌ | ✅ |
| **Traffic Stats** | ✅ Real-time chart | ✅ | ✅ | ✅ |
| **Connections View** | ✅ | ✅ | ✅ | ✅ |
| **Rules Editor** | ✅ GUI editor | ✅ | ✅ | ✅ |
| **Subscription** | ✅ Clash / v2rayN / Sing-box | ✅ Clash | ✅ v2rayN / Base64 | ✅ Remote URL |
| **Override Config** | ✅ Merge config | ✅ | ❌ | ❌ |
| **Multi-language** | ✅ zh-CN / en | ✅ | ✅ zh-CN / en | ✅ |
| **Theme** | ✅ Dark/Light/Auto + 6 themes | ✅ | ❌ | ❌ |
| **Tray Icon** | ✅ Traffic speed | ✅ | ✅ (Windows) | ❌ |
| **Mobile Support** | ✅ iOS + Android | ❌ | ❌ | ✅ iOS + Android |
| **Open Source** | ✅ GPL-3.0 | ✅ GPL-3.0 | ✅ GPL-3.0 | ✅ GPL-3.0 |

### Protocol Support Comparison

| Protocol | NS VPN (meow-rs) | Clash (mihomo) | v2rayN (Xray) | Sing-box |
|----------|:----------------:|:---------------:|:--------------:|:--------:|
| Shadowsocks | ✅ | ✅ | ✅ | ✅ |
| Trojan | ✅ | ✅ | ✅ | ✅ |
| VMess | ✅ | ✅ | ✅ | ✅ |
| VLESS | ✅ | ✅ | ✅ | ✅ |
| VLESS Vision | ✅ | ✅ | ✅ | ✅ |
| VLESS Encryption | ✅ | ✅ | ✅ | ✅ |
| Snell | ✅ | ✅ | ❌ | ❌ |
| Hysteria2 | ✅ | ✅ | ✅ | ✅ |
| AnyTLS | ✅ | ✅ | ❌ | ❌ |
| SOCKS5 | ✅ (listener) | ✅ | ✅ | ✅ |
| HTTP | ✅ (listener) | ✅ | ✅ | ✅ |
| ShadowsocksR | ❌ | ✅ | ✅ | ❌ |
| WireGuard | ❌ | ❌ | ✅ | ✅ |
| TUIC | ❌ | ❌ | ❌ | ✅ |
| SSH | ❌ | ❌ | ❌ | ✅ |
| Naive | ❌ | ❌ | ❌ | ✅ |
| Hysteria | ❌ | ❌ | ✅ | ❌ |

### Subscription Format Support

| Format | NS VPN | Clash Verge Rev | v2rayN | Sing-box |
|--------|:------:|:---------------:|:------:|:--------:|
| Clash YAML | ✅ Native | ✅ Native | ❌ | ✅ Import |
| v2rayN Base64 | ✅ Convert | ✅ Convert | ✅ Native | ❌ |
| Sing-box JSON | ✅ Convert | ❌ | ❌ | ✅ Native |
| OpenVPN | ❌ | ❌ | ✅ | ❌ |

### Why NS VPN?

- **All-in-One**: No need to install a separate proxy kernel — meow-rs is embedded
- **Cross-Platform Desktop + Mobile**: One codebase for all platforms (Tauri 2)
- **Modern UI**: Built with Vue 3, Tailwind CSS, dark/light themes, accent color customization
- **Subscription Format Agnostic**: Import from Clash, v2rayN, or Sing-box — all auto-converted
- **Performance**: Rust backend with minimal memory footprint
- **Active Development**: Regular updates with new features

---

## Configuration

### App Config File

`~/.config/ns-vpn/ns-vpn.yaml`

```yaml
mixed_port: 7890
api_port: 9090
allow_lan: false
mode: rule
log_level: info
ipv6: false
system_proxy: false
tun_mode: false
language: zh-CN
theme: dark
```

### Kernel Config File

`~/.config/ns-vpn/config.yaml`

Generated from subscription data + app config. Standard Clash-compatible YAML format.

---

## Development

### Prerequisites

```bash
# Node.js 20+
# pnpm 9+
# Rust 1.77+

# macOS
brew install librsvg

# Linux (Ubuntu/Debian)
sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev librsvg2-dev libappindicator3-dev patchelf

# Windows
# Install Visual Studio 2022 with C++ support
# Install WebView2 (included in Windows 11)
```

### Commands

```bash
pnpm dev              # Vite dev server only (frontend)
pnpm tauri:dev        # Full Tauri dev (frontend + Rust)
pnpm build            # Production build (frontend only)
pnpm tauri build      # Production build (desktop app)
pnpm tauri ios build  # Production build (iOS)
pnpm tauri android build  # Production build (Android)
```

### Code Quality

```bash
npx vue-tsc --noEmit  # TypeScript check (zero errors required)
cd src-tauri && cargo check  # Rust check (zero warnings required)
```

---

## License

GNU General Public License v3.0

---

## Acknowledgements

- [meow-rs](https://github.com/meow-rs/meow) — The proxy kernel that powers NS VPN
- [Tauri](https://tauri.app) — Desktop/mobile application framework
- [Vue 3](https://vuejs.org) — Frontend framework
- [Tailwind CSS](https://tailwindcss.com) — CSS framework
- [Lucide](https://lucide.dev) — Icons
- All open-source libraries and tools used in this project