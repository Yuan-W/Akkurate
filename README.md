# Akkurate - AI 语法助手

一款基于 Gemini API 的 AI 驱动语法检查与文本润色工具，专为 Linux (Wayland) 设计。

## ✨ 功能特点

- **语法检查** - 检测并修复英文语法、拼写和标点错误
- **文本润色** - 根据不同场景（日常/商务/学术/创意）优化写作风格
- **双语界面** - 支持中文和英文界面切换
- **热键触发** - 选中文字后一键检查，无需复制粘贴
- **原生 Wayland** - 完美支持 Sway、Hyprland 等 Wayland 合成器

## 📦 安装

### Arch Linux / CachyOS

```bash
cd packaging/arch
makepkg -si
```

### 便携安装

```bash
# 下载发布包
tar -xzf akkurate-*-linux-x86_64.tar.gz
cd akkurate-*/

# 安装到 ~/.local/bin
./install.sh
```

### 从源码构建

```bash
# 依赖：rust, cargo, wl-clipboard
cargo build --release
cp target/release/akkurate ~/.local/bin/
```

## 🚀 使用方法

### 方式一：热键（推荐）

1. 在任意应用中选中英文文本（高亮即可，无需复制）
2. 按热键触发 `akkurate -s`
3. 自动检查语法并显示结果

**配置热键：**

```bash
# Sway (~/.config/sway/config)
bindsym $mod+g exec akkurate -s

# Hyprland (~/.config/hypr/hyprland.conf)
bind = SUPER, G, exec, akkurate -s

# KDE Plasma
# 系统设置 > 快捷键 > 自定义快捷键 > akkurate -s
```

### 方式二：图形界面

```bash
akkurate
```

## ⚙️ 配置

首次运行需要配置 Gemini API 密钥：

1. 访问 https://aistudio.google.com/apikey 获取密钥
2. 在设置页面输入密钥并保存

配置文件位置：`~/.config/akkurate/config.toml`

## 🎨 文风预设

| 预设 | 适用场景 |
|------|----------|
| 日常 | 聊天、社交媒体 |
| 商务 | 邮件、报告 |
| 学术 | 论文、文档 |
| 创意 | 故事、博客 |

## 🔧 命令行参数

```bash
akkurate              # 启动图形界面
akkurate -s           # 检查当前选中的文本
akkurate --check "text"   # 检查指定文本
akkurate --enhance "text" # 润色指定文本
akkurate --help       # 查看帮助
```

## 📋 依赖

- `wl-clipboard` - 用于读取选中文本
- Gemini API 密钥

## 📄 许可证

MIT License

## 🙏 致谢

- [iced](https://github.com/iced-rs/iced) - Rust GUI 框架
- [Gemini API](https://ai.google.dev/) - AI 语言模型
