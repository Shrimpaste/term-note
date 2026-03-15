# Term Note

终端里的笔记本，支持 Markdown 和全文搜索。

![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## 特性

- 📝 **笔记管理**：创建、编辑、删除笔记
- 🔍 **全文搜索**：基于 SQLite FTS5 的快速搜索
- 🖥️ **终端 UI**：优雅的用户界面，支持 Windows Terminal / iTerm2 / Linux
- 🎨 **墨绿配色**：护眼的深色主题
- ⌨️ **Vim 风格快捷键**：熟悉的操作体验
- 📄 **Markdown 预览**：实时渲染 Markdown
- 💾 **本地存储**：数据保存在本地 SQLite 数据库

## 截图

```
┌────────────────────────────────────────────┐
│       📒 Term Note - 终端笔记本              │
├────────────────────────────────────────────┤
│ 笔记列表                                    │
│ ▶ 第一篇笔记                                │
│   这是笔记的预览内容...      2025-08-01    │
│                                             │
│   Rust 学习笔记                             │
│   所有权和生命周期...        2025-07-28    │
│                                             │
│   项目想法                                  │
│   1. 终端音乐播放器...       2025-07-20    │
└────────────────────────────────────────────┘
 3 个笔记 | j/k 移动 | Enter 查看 | n 新建
```

## 安装

### 前提条件

- [Rust](https://rustup.rs/) 1.70 或更高版本
- Windows Terminal / iTerm2 / 任意支持 Unicode 的现代终端

### 从源码安装

```bash
# 克隆仓库
git clone https://github.com/Shrimpaste/term-note.git
cd term-note

# 构建并安装
cargo build --release

# 将可执行文件添加到 PATH
cp target/release/term-note ~/.local/bin/
# 或
cp target/release/term-note /usr/local/bin/
```

### 快速运行（不安装）

```bash
cargo run --release
```

## 快捷键

| 快捷键 | 功能 |
|--------|------|
| **全局** | |
| `q` | 退出应用 |
| `?` / `h` | 显示帮助 |
| `ESC` | 返回/取消 |
| **列表视图** | |
| `j` / `↓` | 向下移动 |
| `k` / `↑` | 向上移动 |
| `Enter` | 查看选中笔记 |
| `n` | 新建笔记 |
| `/` | 搜索笔记 |
| `d` | 删除选中笔记 |
| **查看笔记** | |
| `e` | 编辑笔记 |
| `p` | 预览 Markdown |
| **编辑模式** | |
| `i` | 进入插入模式 |
| `ESC` | 返回普通模式 |
| `Tab` | 切换标题/内容 |
| `Ctrl+S` | 保存笔记 |
| `:w` | 保存（Vim风格） |
| `:q` | 退出编辑 |

## 数据存储

- **Windows**: `%APPDATA%\com.shrimpaste\term-note\notes.db`
- **macOS**: `~/Library/Application Support/com.shrimpaste.term-note/notes.db`
- **Linux**: `~/.local/share/term-note/notes.db`

## 开发

```bash
# 运行测试
cargo test

# 开发模式运行
cargo run

# 格式化代码
cargo fmt

# 检查代码
cargo clippy
```

## 技术栈

- **语言**: Rust
- **终端 UI**: [ratatui](https://github.com/ratatui-org/ratatui)
- **终端控制**: [crossterm](https://github.com/crossterm-rs/crossterm)
- **数据库**: SQLite + [rusqlite](https://github.com/rusqlite/rusqlite)
- **Markdown**: [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)

## 项目结构

```
term-note/
├── src/
│   ├── main.rs          # 程序入口
│   ├── app/             # 应用逻辑
│   ├── db/              # 数据库操作
│   ├── models/          # 数据模型
│   ├── ui/              # 界面渲染
│   └── utils/           # 工具函数
├── Cargo.toml
└── README.md
```

## 路线图

- [x] 基础 CRUD 操作
- [x] 全文搜索
- [x] Markdown 预览
- [ ] 标签管理
- [ ] 导入/导出（Markdown 文件）
- [ ] 同步功能
- [ ] 语法高亮

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License

---

由 [艾遥昕](https://github.com/Shrimpaste) 制作