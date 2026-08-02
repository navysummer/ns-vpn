# NS VPN

基于 **Tauri 2** + **Vue 3** 构建的现代化跨平台代理客户端，内核采用 **meow-rs**。

---

## 截图

<!-- TODO: 添加截图 -->

---

## 功能特性

### 核心代理引擎
- **meow-rs v0.19** 嵌入式内核 — 高性能、低内存占用的代理运行时
- **系统代理** — 一键开启/关闭系统级代理（macOS、Windows、Linux）
- **TUN 模式** — 虚拟网卡模式，接管所有网络流量
- **流量统计** — 实时上传/下载速度、总量、活跃连接数
- **连接管理** — 查看、筛选、关闭单个或全部连接
- **规则管理** — 浏览、搜索、筛选代理规则

### 多协议支持
- **Shadowsocks** — 完整支持（多种加密方式、插件协议）
- **ShadowsocksR** — 解析支持（v2rayN 格式解析，核心需 meow-rs 支持）
- **Trojan** — 完整支持（TLS、Vision 流控）
- **VMess** — 完整支持（多种传输协议、Alter ID）
- **VLESS** — 完整支持（Vision 流控、ML-KEM 加密、REALITY）
- **Snell** — 完整支持（v3/v4、Obfs）
- **Hysteria2** — 完整支持（QUIC、Obfs）
- **Hysteria (v1)** — 解析支持（v2rayN 格式解析，核心需 meow-rs 支持）
- **AnyTLS** — 完整支持（TLS 指纹、自定义 SNI）
- **TUIC** — 解析支持（v2rayN 格式解析，核心需 meow-rs 支持）
- **SSH** — 解析支持（v2rayN 格式解析，核心需 meow-rs 支持）
- **Naive** — 解析支持（v2rayN 格式解析，核心需 meow-rs 支持）
- **WireGuard** — 解析支持（v2rayN 格式解析，核心需 meow-rs 支持）
- **SOCKS5** — 代理节点 + 监听器
- **HTTP** — 代理节点 + 监听器
- **Direct** — 直连（内置）

### 订阅管理
- **Clash YAML** — 直接导入和应用
- **v2rayN Base64** — 自动转换为 Clash 格式（支持 ss/trojan/vmess/vless/hysteria2/snell/anytls）
- **Sing-box JSON** — 自动转换为 Clash 格式
- **订阅链接** — 远程拉取、自动刷新
- **本地文件** — 从文件系统导入
- **粘贴内容** — 直接粘贴原始订阅数据
- **自动更新** — 定时刷新订阅
- **覆盖配置** — 在订阅基础上合并自定义配置

### 用户界面
- **仪表盘** — 核心状态、流量图表、节点/分组概览、快捷操作
- **代理** — 分组代理选择、延迟测试、模式切换
- **规则** — 规则列表、搜索、筛选、类型标识
- **连接** — 活跃连接表格、实时统计
- **日志** — 实时日志查看器
- **设置** — 全面的配置界面
- **多语言** — 中文 (zh-CN)、English (en)
- **主题** — 深色/浅色/跟随系统、6 种背景主题、强调色选择
- **托盘图标** — 系统托盘、速度显示、快捷控制

### 平台支持
- **macOS** — x86_64、arm64（Apple Silicon）
- **Windows** — x86_64、arm64
- **Linux** — x86_64、arm64（AppImage、deb）
- **iOS** — arm64
- **Android** — arm64、x86_64

---

## 安装

### 从 Release 下载

1. 前往 [Releases](https://github.com/ns-vpn/ns-vpn/releases) 页面
2. 下载适合你平台的包：
   - **macOS**：`.dmg`（拖入 Applications）、`.zip`（手动安装）
   - **Windows**：`.msi`（安装器）、`.exe`（NSIS 安装器）
   - **Linux**：`.AppImage`（直接运行）、`.deb`（Debian/Ubuntu）
   - **iOS**：`.ipa`（侧载或 TestFlight）
   - **Android**：`.apk`（直接安装）

### 从源码构建

```bash
# 前置要求
# - Node.js 20+
# - pnpm 9+
# - Rust 1.77+
# - 平台相关依赖（见 Tauri 2 文档）

# 克隆
git clone https://github.com/ns-vpn/ns-vpn.git
cd ns-vpn

# 安装前端依赖
pnpm install

# 开发模式
pnpm tauri:dev

# 生产构建（桌面端）
pnpm tauri build

# 移动端构建
pnpm tauri ios build
pnpm tauri android build
```

---

## 使用说明

### 快速开始

1. **添加订阅**
   - 打开应用 → 进入 **订阅** 页面
   - 点击 **新建订阅** → 输入名称和链接
   - 选择格式：`Clash (YAML)`、`v2rayN (Base64)` 或 `Sing-box (JSON)`
   - 点击 **保存**，然后 **应用** 激活

2. **启动代理**
   - 在仪表盘页面，点击 **启动核心** 启动代理内核
   - 默认混合端口为 `7890`（可在设置中修改）

3. **配置系统代理**（可选）
   - 进入 **设置** → 开启 **系统代理** 将系统流量路由到代理
   - 或开启 **TUN 模式** 实现 VPN 级别的全局流量接管

4. **选择代理节点**
   - 进入 **代理** 页面 → 选择代理组 → 点击节点切换
   - 使用 **延迟测试** 检查节点延迟

### 订阅管理

- **远程链接**：添加订阅链接，可配置自动刷新间隔
- **本地文件**：从电脑导入 `.yaml`、`.json`、`.txt` 文件
- **粘贴内容**：直接粘贴原始订阅数据
- **格式检测**：支持 Clash、v2rayN Base64、Sing-box 格式
- **覆盖配置**：添加自定义配置段（规则、代理组等），合并到订阅数据之上

### 快捷键

| 操作 | 快捷键 |
|------|--------|
| 显示/隐藏窗口 | `Ctrl+Shift+Space` |
| 快速切换代理 | `Ctrl+Shift+Q` |

---

## 架构

```
┌──────────────────────────────────────────────────┐
│                   NS VPN GUI                      │
│  ┌────────────────────────────────────────────┐  │
│  │           Vue 3 前端 (Tauri)               │  │
│  │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐     │  │
│  │  │仪表盘│ │代理  │ │规则  │ │设置  │  ... │  │
│  │  └──────┘ └──────┘ └──────┘ └──────┘     │  │
│  │  Pinia 状态 ←→ Tauri IPC (invoke/events)   │  │
│  └──────────────────┬─────────────────────────┘  │
│                     │                             │
│  ┌──────────────────▼─────────────────────────┐  │
│  │           Rust 后端 (Tauri)                │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐   │  │
│  │  │ 配置管理 │ │ 核心管理 │ │ 命令(40+)│   │  │
│  │  └──────────┘ └────┬─────┘ └──────────┘   │  │
│  └────────────────────┼───────────────────────┘  │
│                       │                           │
│  ┌────────────────────▼───────────────────────┐  │
│  │           meow-rs v0.19 内核                │  │
│  │  (HTTP API :9090)                           │  │
│  │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐     │  │
│  │  │代理  │ │隧道  │ │ DNS  │ │规则  │     │  │
│  │  └──────┘ └──────┘ └──────┘ └──────┘     │  │
│  └──────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────┘
```

### 核心技术

- **前端**：Vue 3（Composition API、`<script setup>`）、Pinia、Tailwind CSS 4、Lucide 图标
- **后端**：Rust/Tauri 2、40+ 个注册 Tauri 命令
- **内核**：[meow-rs](https://github.com/meow-rs/meow) v0.19 — 用 Rust 编写的 Clash 兼容代理内核
- **配置**：基于 YAML（`~/.config/ns-vpn/ns-vpn.yaml` 为应用配置，`config.yaml` 为内核配置）

---

## 对比：NS VPN vs Clash vs v2rayN vs Sing-box

### 功能对比

| 功能 | NS VPN | Clash Verge Rev | v2rayN | Sing-box |
|------|--------|-----------------|--------|----------|
| **GUI 框架** | Tauri 2 (Rust + Vue 3) | Tauri 1 (Rust + React) | .NET WinForms | Flutter (Android) / SwiftUI (iOS) |
| **平台** | macOS, Windows, Linux, iOS, Android | macOS, Windows, Linux | 仅 Windows | iOS, Android, macOS, Linux |
| **代理内核** | meow-rs（嵌入式） | mihomo（外部） | Xray / v2fly（外部） | sing-box（嵌入式） |
| **系统代理** | ✅ 一键开启 | ✅ | ✅ | ❌（仅移动端） |
| **TUN 模式** | ✅ | ✅ | ❌ | ✅ |
| **流量统计** | ✅ 实时图表 | ✅ | ✅ | ✅ |
| **连接查看** | ✅ | ✅ | ✅ | ✅ |
| **规则编辑** | ✅ GUI 编辑器 | ✅ | ✅ | ✅ |
| **订阅管理** | ✅ Clash / v2rayN / Sing-box | ✅ Clash | ✅ v2rayN / Base64 | ✅ 远程链接 |
| **覆盖配置** | ✅ 合并配置 | ✅ | ❌ | ❌ |
| **多语言** | ✅ 中文 / English | ✅ | ✅ 中文 / English | ✅ |
| **主题** | ✅ 深色/浅色/自动 + 6 种主题 | ✅ | ❌ | ❌ |
| **托盘图标** | ✅ 显示实时速度 | ✅ | ✅（仅 Windows） | ❌ |
| **移动端支持** | ✅ iOS + Android | ❌ | ❌ | ✅ iOS + Android |
| **开源协议** | ✅ GPL-3.0 | ✅ GPL-3.0 | ✅ GPL-3.0 | ✅ GPL-3.0 |

### 协议支持对比

| 协议 | NS VPN (meow-rs) | Clash (mihomo) | v2rayN (Xray) | Sing-box |
|------|:----------------:|:---------------:|:--------------:|:--------:|
| Shadowsocks | ✅ | ✅ | ✅ | ✅ |
| ShadowsocksR | ✅ 解析¹ | ✅ | ✅ | ❌ |
| Trojan | ✅ | ✅ | ✅ | ✅ |
| VMess | ✅ | ✅ | ✅ | ✅ |
| VLESS | ✅ | ✅ | ✅ | ✅ |
| VLESS Vision | ✅ | ✅ | ✅ | ✅ |
| VLESS 加密 | ✅ | ✅ | ✅ | ✅ |
| Snell | ✅ | ✅ | ❌ | ❌ |
| Hysteria2 | ✅ | ✅ | ✅ | ✅ |
| Hysteria (v1) | ✅ 解析¹ | ✅ | ✅ | ✅ |
| AnyTLS | ✅ | ✅ | ❌ | ❌ |
| TUIC | ✅ 解析¹ | ❌ | ❌ | ✅ |
| SSH | ✅ 解析¹ | ❌ | ❌ | ✅ |
| Naive | ✅ 解析¹ | ❌ | ❌ | ✅ |
| WireGuard | ✅ 解析¹ | ❌ | ✅ | ✅ |
| SOCKS5 | ✅（代理 + 监听） | ✅ | ✅ | ✅ |
| HTTP | ✅（代理 + 监听） | ✅ | ✅ | ✅ |
| Direct（直连） | ✅（内置） | ✅ | ✅ | ✅ |

> ¹ 解析支持：可以在 v2rayN 订阅中解析该协议并生成配置，但运行需要 meow-rs 内核支持

### 传输协议支持

| 传输方式 | NS VPN (meow-rs) | Clash (mihomo) | v2rayN (Xray) | Sing-box |
|---------|:----------------:|:---------------:|:--------------:|:--------:|
| TCP | ✅ | ✅ | ✅ | ✅ |
| WebSocket (WS) | ✅ VLESS + VMess | ✅ | ✅ | ✅ |
| gRPC | ✅ 仅 VLESS | ✅ | ✅ | ✅ |
| HTTP/2 (H2) | ✅ 仅 VLESS | ✅ | ✅ | ✅ |
| HTTPUpgrade | ✅ 仅 VLESS | ✅ | ✅ | ❌ |
| WebTransport | ❌ | ✅ | ❌ | ❌ |
| QUIC | ❌ | ❌ | ✅ | ✅ |
| TLS | ✅ | ✅ | ✅ | ✅ |
| REALITY | ✅ 仅 VLESS | ✅ | ✅ | ✅ |

> VLESS 支持：tcp、ws、grpc、h2、httpupgrade、tls、reality
> VMess 支持：tcp、ws、tls

### 代理组类型支持
| Splice | ❌ | ✅ | ❌ | ❌ |

### 代理组类型支持

| 组类型 | NS VPN (meow-rs) | Clash (mihomo) | v2rayN (Xray) | Sing-box |
|--------|:----------------:|:---------------:|:--------------:|:--------:|
| 手动选择 (Select) | ✅ | ✅ | ✅ | ✅ |
| 自动测速 (URL Test) | ✅ | ✅ | ✅ | ✅ |
| 故障转移 (Fallback) | ✅ | ✅ | ✅ | ✅ |
| 负载均衡 (Load Balance) | ✅ | ✅ | ❌ | ✅ |
| 中继 (Relay) | ✅ | ✅ | ❌ | ✅ |
| 兼容模式 (Pass) | ✅ | ✅ | ❌ | ❌ |

### 协议功能详情

#### Shadowsocks
- 加密方式：aes-128-gcm、aes-256-gcm、chacha20-ietf-poly1305、2022-blake3-* 等
- 插件：obfs、v2ray-plugin（通过 plugin-opts）
- UDP：✅

#### Trojan
- TLS：✅（支持 sni、skip-cert-verify）
- UDP：✅
- 流控：✅（xtls-rprx-vision）

#### VMess
- 加密方式：auto、aes-128-gcm、chacha20-poly1305 等
- 传输：TCP、WS（仅支持这两种）
- TLS：✅（支持 sni、skip-cert-verify）
- UDP：✅
- Alter ID：✅（推荐 0）

#### VLESS
- 传输：TCP、WS、gRPC、H2、HTTPUpgrade（支持 5 种）
- TLS：✅（支持 sni、skip-cert-verify）
- REALITY：✅（public-key、short-id、client-fingerprint）
- 流控：✅（xtls-rprx-vision）
- UDP：✅
- 加密：✅（ML-KEM 后量子加密，vless-encryption 特性）

#### Hysteria2
- 基于 QUIC：✅
- Obfs：✅（password 类型）
- SNI：✅
- 跳过证书验证：✅
- UDP：✅

#### Snell
- 版本：3、4
- Obfs：✅（http、tls）
- UDP：✅（v4+）

#### AnyTLS
- TLS：✅
- 指纹：✅（chrome、safari、firefox 等）
- 跳过证书验证：✅
- UDP：✅

#### ShadowsocksR
- 协议：✅（origin、auth_sha1_v4、auth_aes128_md5 等）
- 混淆：✅（plain、http_simple、tls1.2_ticket_auth 等）
- 加密：✅（aes-128-ctr、chacha20 等）
- 备注：需内核支持 SSR 协议

#### Hysteria (v1)
- 基于 QUIC：✅
- 协议：✅（udp、faketcp、wechat-video）
- 上行/下行带宽：✅ 可配置
- 跳过证书验证：✅
- 备注：需内核支持 Hysteria v1 协议

#### TUIC
- 基于 QUIC：✅
- 拥塞控制：✅（bbr、cubic、new_reno 等）
- UDP：✅
- 备注：需内核支持 TUIC 协议

#### SSH
- 端口：✅ 可配置（默认 22）
- 认证：✅ 密码认证
- 备注：需内核支持 SSH 代理协议

#### Naive
- 基于 HTTP/2：✅
- 认证：✅ 用户名/密码
- 备注：需内核支持 Naive 代理协议

#### WireGuard
- 端口：✅ 可配置（默认 51820）
- 备注：需内核支持 WireGuard 协议

### 订阅格式支持

| 格式 | NS VPN | Clash Verge Rev | v2rayN | Sing-box |
|------|:------:|:---------------:|:------:|:--------:|
| Clash YAML | ✅ 原生 | ✅ 原生 | ❌ | ✅ 导入 |
| v2rayN Base64 | ✅ 转换 | ✅ 转换 | ✅ 原生 | ❌ |
| Sing-box JSON | ✅ 转换 | ❌ | ❌ | ✅ 原生 |
| OpenVPN | ❌ | ❌ | ✅ | ❌ |

### 为什么选择 NS VPN？

- **一体化**：无需安装单独代理内核 — meow-rs 直接嵌入
- **跨平台桌面 + 移动端**：一套代码覆盖所有平台（Tauri 2）
- **现代化界面**：Vue 3、Tailwind CSS、深色/浅色主题、强调色自定义
- **订阅格式无关**：Clash、v2rayN、Sing-box — 全部自动转换
- **高性能**：Rust 后端，内存占用极低
- **持续开发**：定期更新新功能

---

## 配置

### 应用配置文件

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

### 内核配置文件

`~/.config/ns-vpn/config.yaml`

由订阅数据 + 应用配置合并生成，标准的 Clash 兼容 YAML 格式。

---

## 开发

### 前置要求

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
# 安装 Visual Studio 2022（含 C++ 支持）
# 安装 WebView2（Windows 11 已内置）
```

### 常用命令

```bash
pnpm dev              # Vite 开发服务器（仅前端）
pnpm tauri:dev        # 完整 Tauri 开发（前端 + Rust）
pnpm build            # 生产构建（仅前端）
pnpm tauri build      # 生产构建（桌面应用）
pnpm tauri ios build  # 生产构建（iOS）
pnpm tauri android build  # 生产构建（Android）
```

### 代码质量

```bash
npx vue-tsc --noEmit              # TypeScript 检查（要求零错误）
cd src-tauri && cargo check        # Rust 检查（要求零警告）
```

---

## 许可证

GNU General Public License v3.0

---

## 致谢

- [meow-rs](https://github.com/meow-rs/meow) — 驱动 NS VPN 的代理内核
- [Tauri](https://tauri.app) — 桌面/移动应用框架
- [Vue 3](https://vuejs.org) — 前端框架
- [Tailwind CSS](https://tailwindcss.com) — CSS 框架
- [Lucide](https://lucide.dev) — 图标库
- 以及本项目中使用的所有开源库和工具