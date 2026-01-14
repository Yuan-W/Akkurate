# Akkurate - AI 语法助手

Akkurate 是一款基于 Gemini API 的 AI 驱动语法检查与文本润色工具。它轻量、快速，且注重隐私。

现已提供 **桌面应用**（专为 Linux Wayland 设计）和 **浏览器扩展**（Chrome/Edge/Brave 跨平台支持）。

## ✨ 功能特点

- **语法检查** - 检测并修复英文语法、拼写和标点错误。
- **文本润色** - 根据不同场景（日常、商务、学术、创意）优化写作风格。
- **隐私优先** - 使用你自己的 API Key，数据直接与 Gemini API 通信，不经过第三方服务器。

### 🖥️ 桌面端 (Linux/Wayland)
- **即时检查** - 在任意应用中选中文字，按下热键即可检查。
- **原生 Wayland 支持** - 在 Sway 和 Hyprland 上经过验证。
- **系统托盘** - 快速访问设置和状态。

### 🧩 浏览器扩展 (Chrome/Edge)
- **悬浮按钮** - 在输入框聚焦时自动出现。
- **一键修复** - 直接将修正后的文本应用到输入框中。
- **WASM 驱动** - 核心逻辑在本地或浏览器中运行，快速且安全。

## 📦 构建与安装

前置依赖：`rust`, `cargo`。

### 1. 构建项目
Akkurate 采用 Cargo Workspace 结构组织。

```bash
# 克隆并构建整个工作空间
git clone https://github.com/yourusername/akkurate.git
cd akkurate
cargo build --release
```

### 2. Chrome 浏览器扩展

```bash
# 构建扩展包（支持 Linux, Windows, macOS）
cargo xtask dist
```

该命令会在项目根目录生成 `akkurate-extension.zip`。

**安装步骤：**
1. 打开浏览器的扩展管理页面 (`chrome://extensions` 或 `edge://extensions`)。
2. 开启右上角的 "开发者模式" (Developer mode)。
3. 将 `akkurate-extension.zip` 拖入页面（或加载解压后的 `extension/pkg` 目录）。

### 3. 桌面应用 (Linux)

```bash
# 安装到本地 bin 目录
cp target/release/akkurate-desktop ~/.local/bin/akkurate
```

**运行时依赖：**
- `wl-clipboard` (用于 Wayland 剪贴板访问)

## 🚀 使用指南

### 浏览器扩展
1. 点击浏览器工具栏的 Akkurate 图标，设置你的 **Gemini API Key**。
2. 点击任意文本输入框（如 GitHub 评论、邮件撰写窗口）。
3. 一个悬浮的 **✨** 按钮会自动出现。
4. 点击按钮进行语法检查。
5. 在弹出的结果卡片中查看建议，点击 **Apply Fixes** 应用修改。

### 桌面应用 (热键)
1. 选中任意文本。
2. 按下你配置的热键（默认行为是运行 `akkurate -s`）。
3. 结果窗口将自动弹出。

**Sway 配置示例：**
```bash
bindsym $mod+g exec akkurate -s
```

**Hyprland 配置示例：**
```bash
bind = SUPER, G, exec, akkurate -s
```

## ⚙️ 配置

**获取 API Key：** 请前往 [Google AI Studio](https://aistudio.google.com/apikey) 免费申请。

- **桌面端**：配置文件位于 `~/.config/akkurate/config.toml`。
- **扩展端**：通过点击扩展图标在弹出窗口中设置（存储在浏览器本地存储）。

## 🛠️ 项目结构

- `core/`: 共享 Rust 逻辑 (API 客户端, 文本处理,配置 Traits)。
- `desktop/`: 基于 Iced 的 Linux GUI 应用。
- `extension/`: 基于 WASM 的浏览器扩展逻辑。
- `xtask/`: 跨平台构建自动化工具。

## 📄 许可证

MIT License
