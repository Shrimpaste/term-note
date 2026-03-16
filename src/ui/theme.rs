use ratatui::style::{Color, Modifier, Style};

/// UI 主题配置 - 终端黑客风格
/// 配色：纯黑背景 + 荧光绿 + 青色高亮
#[derive(Debug, Clone)]
pub struct Theme {
    /// 主背景色（纯黑终端）
    pub bg: Color,
    /// 次背景色（卡片背景）
    pub bg_secondary: Color,
    /// 三级背景（输入框背景）
    pub bg_tertiary: Color,
    /// 前景色（荧光绿）
    pub fg: Color,
    /// 前景色次要（暗绿）
    pub fg_secondary: Color,
    /// 边框色（暗绿）
    pub border: Color,
    /// 边框激活（青色）
    pub border_active: Color,
    /// 主强调色（荧光绿）
    pub accent: Color,
    /// 强调色次要（暗绿）
    pub accent_dim: Color,
    /// 强调色暗淡
    pub accent_subtle: Color,
    /// 选中背景
    pub highlight_bg: Color,
    /// 选中前景
    pub highlight_fg: Color,
    /// 状态栏背景
    pub status_bg: Color,
    /// 状态栏前景
    pub status_fg: Color,
    /// 预览文字
    pub preview: Color,
    /// 日期颜色
    pub date: Color,
    /// 错误色（红）
    pub error: Color,
    /// 警告色（黄）
    pub warning: Color,
    /// 成功色（青）
    pub success: Color,
    /// 阴影色
    pub shadow: Color,
    /// 渐变起始
    pub gradient_start: Color,
    /// 渐变结束
    pub gradient_end: Color,
    /// 提示符颜色（青）
    pub prompt: Color,
    /// 路径颜色（绿）
    pub path: Color,
    /// 命令颜色（紫）
    pub command: Color,
    /// 用户信息颜色（红）
    pub user: Color,
    /// 分类颜色（橙）
    pub category: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::terminal()
    }
}

impl Theme {
    /// 终端黑客风格主题
    pub fn terminal() -> Self {
        Self {
            bg: Color::Rgb(10, 10, 10),            // #0a0a0a 纯黑终端
            bg_secondary: Color::Rgb(18, 18, 18),  // #121212 卡片
            bg_tertiary: Color::Rgb(26, 26, 26),   // #1a1a1a 输入框
            fg: Color::Rgb(0, 255, 0),             // #00ff00 荧光绿
            fg_secondary: Color::Rgb(0, 180, 0),   // #00b400 暗绿
            border: Color::Rgb(40, 60, 40),        // #283c28 暗绿边框
            border_active: Color::Rgb(0, 255, 255), // #00ffff 青色
            accent: Color::Rgb(0, 255, 0),         // #00ff00 荧光绿
            accent_dim: Color::Rgb(0, 200, 0),     // #00c800 中绿
            accent_subtle: Color::Rgb(0, 100, 0),  // #006400 暗绿
            highlight_bg: Color::Rgb(0, 60, 0),    // #003c00 选中背景
            highlight_fg: Color::Rgb(0, 255, 100), // #00ff64 亮绿
            status_bg: Color::Rgb(12, 12, 12),     // #0c0c0c 状态栏
            status_fg: Color::Rgb(128, 128, 128),  // #808080 状态文字
            preview: Color::Rgb(120, 120, 120),    // #787878 预览
            date: Color::Rgb(90, 90, 90),          // #5a5a5a 日期
            error: Color::Rgb(255, 68, 68),        // #ff4444 终端红
            warning: Color::Rgb(255, 230, 109),    // #ffe66d 终端黄
            success: Color::Rgb(78, 205, 196),     // #4ecdc4 终端青
            shadow: Color::Rgb(5, 5, 5),           // #050505 阴影
            gradient_start: Color::Rgb(0, 255, 0),
            gradient_end: Color::Rgb(0, 200, 200),
            prompt: Color::Rgb(0, 255, 255),       // #00ffff 青色提示符
            path: Color::Rgb(139, 195, 74),        // #8bc34a 路径绿
            command: Color::Rgb(156, 39, 176),     // #9c27b0 命令紫
            user: Color::Rgb(255, 107, 107),       // #ff6b6b 用户红
            category: Color::Rgb(255, 152, 0),     // #ff9800 分类橙
        }
    }

    // ========== 基础样式 ==========

    /// 主标题样式（渐变效果模拟）
    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    /// 副标题样式
    pub fn subtitle_style(&self) -> Style {
        Style::default()
            .fg(self.fg_secondary)
            .add_modifier(Modifier::ITALIC)
    }

    /// 笔记标题样式
    pub fn note_title_style(&self) -> Style {
        Style::default()
            .fg(self.fg)
            .add_modifier(Modifier::BOLD)
    }

    /// 选中笔记标题样式
    pub fn note_title_selected_style(&self) -> Style {
        Style::default()
            .fg(self.highlight_fg)
            .add_modifier(Modifier::BOLD)
    }

    // ========== 边框样式 ==========

    /// 基础边框样式
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// 激活边框样式（发光效果）
    pub fn border_active_style(&self) -> Style {
        Style::default()
            .fg(self.border_active)
            .add_modifier(Modifier::BOLD)
    }

    /// 次要边框
    pub fn border_subtle_style(&self) -> Style {
        Style::default().fg(self.bg_tertiary)
    }

    // ========== 选中/高亮样式 ==========

    /// 高亮样式（强烈）
    pub fn highlight_style(&self) -> Style {
        Style::default()
            .bg(self.highlight_bg)
            .fg(self.highlight_fg)
            .add_modifier(Modifier::BOLD)
    }

    /// 选中行样式（柔和）
    pub fn selected_row_style(&self) -> Style {
        Style::default()
            .bg(self.highlight_bg)
            .fg(self.fg)
    }

    /// 悬停样式
    pub fn hover_style(&self) -> Style {
        Style::default()
            .bg(self.bg_tertiary)
            .fg(self.fg)
    }

    // ========== 状态栏样式 ==========

    /// 状态栏基础样式
    pub fn status_style(&self) -> Style {
        Style::default()
            .bg(self.status_bg)
            .fg(self.status_fg)
    }

    /// 状态栏强调
    pub fn status_accent_style(&self) -> Style {
        Style::default()
            .bg(self.status_bg)
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    // ========== 内容样式 ==========

    /// 预览文字样式
    pub fn preview_style(&self) -> Style {
        Style::default().fg(self.preview)
    }

    /// 预览文字选中样式
    pub fn preview_selected_style(&self) -> Style {
        Style::default()
            .fg(self.fg_secondary)
    }

    /// 日期样式
    pub fn date_style(&self) -> Style {
        Style::default()
            .fg(self.date)
            .add_modifier(Modifier::ITALIC)
    }

    /// 内容样式
    pub fn content_style(&self) -> Style {
        Style::default().fg(self.fg)
    }

    /// 内容次要样式
    pub fn content_secondary_style(&self) -> Style {
        Style::default().fg(self.fg_secondary)
    }

    // ========== 输入样式 ==========

    /// 输入框样式
    pub fn input_style(&self) -> Style {
        Style::default()
            .fg(self.fg)
            .bg(self.bg_tertiary)
    }

    /// 激活输入框样式（发光）
    pub fn input_active_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .bg(self.bg_tertiary)
            .add_modifier(Modifier::BOLD)
    }

    /// 占位符样式
    pub fn placeholder_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    // ========== 强调样式 ==========

    /// 强调样式（主色）
    pub fn accent_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    /// 强调样式（次要）
    pub fn accent_dim_style(&self) -> Style {
        Style::default()
            .fg(self.accent_dim)
            .add_modifier(Modifier::ITALIC)
    }

    /// 强调样式（ subtle）
    pub fn accent_subtle_style(&self) -> Style {
        Style::default().fg(self.accent_subtle)
    }

    /// 普通强调（不加粗）
    pub fn accent_normal_style(&self) -> Style {
        Style::default().fg(self.accent)
    }

    // ========== 快捷键样式 ==========

    /// 快捷键样式
    pub fn key_style(&self) -> Style {
        Style::default()
            .fg(self.accent_dim)
            .add_modifier(Modifier::BOLD)
    }

    /// 快捷键激活样式
    pub fn key_active_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .bg(self.bg_tertiary)
            .add_modifier(Modifier::BOLD)
    }

    // ========== 帮助样式 ==========

    /// 帮助标题样式
    pub fn help_header_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    }

    /// 帮助分类标题
    pub fn help_category_style(&self) -> Style {
        Style::default()
            .fg(self.accent_dim)
            .add_modifier(Modifier::BOLD)
    }

    /// 帮助文字样式
    pub fn help_text_style(&self) -> Style {
        Style::default().fg(self.fg_secondary)
    }

    // ========== 状态样式 ==========

    /// 错误样式
    pub fn error_style(&self) -> Style {
        Style::default()
            .fg(self.error)
            .add_modifier(Modifier::BOLD)
    }

    /// 成功样式
    pub fn success_style(&self) -> Style {
        Style::default()
            .fg(self.success)
            .add_modifier(Modifier::BOLD)
    }

    /// 警告样式
    pub fn warning_style(&self) -> Style {
        Style::default()
            .fg(self.warning)
            .add_modifier(Modifier::BOLD)
    }

    // ========== 模式标签样式 ==========

    /// 模式标签样式（NORMAL/INSERT）
    pub fn mode_style(&self, mode: &str) -> Style {
        match mode {
            "INSERT" => Style::default()
                .bg(self.accent_dim)
                .fg(self.bg)
                .add_modifier(Modifier::BOLD),
            _ => Style::default()
                .bg(self.border)
                .fg(self.fg)
                .add_modifier(Modifier::BOLD),
        }
    }

    /// 搜索高亮样式
    pub fn search_highlight_style(&self) -> Style {
        Style::default()
            .bg(self.accent_subtle)
            .fg(self.fg)
            .add_modifier(Modifier::BOLD)
    }

    // ========== 终端风格样式 ==========

    /// 提示符样式
    pub fn prompt_style(&self) -> Style {
        Style::default()
            .fg(self.prompt)
            .add_modifier(Modifier::BOLD)
    }

    /// 路径样式
    pub fn path_style(&self) -> Style {
        Style::default().fg(self.path)
    }

    /// 命令样式
    pub fn command_style(&self) -> Style {
        Style::default().fg(self.command)
    }

    /// 用户样式
    pub fn user_style(&self) -> Style {
        Style::default().fg(self.user)
    }

    /// 分类样式
    pub fn category_style(&self) -> Style {
        Style::default().fg(self.category)
    }

    /// 终端输出错误样式
    pub fn output_error_style(&self) -> Style {
        Style::default().fg(self.error)
    }

    /// 终端输出信息样式
    pub fn output_info_style(&self) -> Style {
        Style::default().fg(self.warning)
    }

    /// 终端输出成功样式
    pub fn output_success_style(&self) -> Style {
        Style::default().fg(self.success)
    }

    // ========== 装饰样式 ==========

    /// 阴影样式
    pub fn shadow_style(&self) -> Style {
        Style::default().fg(self.shadow)
    }

    /// 分隔线样式
    pub fn separator_style(&self) -> Style {
        Style::default()
            .fg(self.border)
            .add_modifier(Modifier::DIM)
    }

    /// 装饰符号样式
    pub fn ornament_style(&self) -> Style {
        Style::default()
            .fg(self.accent_subtle)
            .add_modifier(Modifier::BOLD)
    }
}
