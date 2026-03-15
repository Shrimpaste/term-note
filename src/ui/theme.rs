use ratatui::style::{Color, Modifier, Style};

/// UI 主题配置 - 精致墨绿主题
/// 配色：深墨绿 + 薄荷绿 + 暖白
#[derive(Debug, Clone)]
pub struct Theme {
    /// 主背景色（最深墨绿）
    pub bg: Color,
    /// 次背景色（卡片背景）
    pub bg_secondary: Color,
    /// 三级背景（输入框背景）
    pub bg_tertiary: Color,
    /// 前景色（暖白）
    pub fg: Color,
    /// 前景色次要
    pub fg_secondary: Color,
    /// 边框色（暗淡）
    pub border: Color,
    /// 边框激活（明亮薄荷绿）
    pub border_active: Color,
    /// 主强调色（亮薄荷绿）
    pub accent: Color,
    /// 强调色次要
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
    /// 错误色
    pub error: Color,
    /// 警告色
    pub warning: Color,
    /// 成功色
    pub success: Color,
    /// 阴影色（更深）
    pub shadow: Color,
    /// 渐变起始
    pub gradient_start: Color,
    /// 渐变结束
    pub gradient_end: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// 深色主题（精致墨绿）
    pub fn dark() -> Self {
        Self {
            bg: Color::Rgb(6, 10, 8),              // #060a08 最深背景
            bg_secondary: Color::Rgb(14, 22, 18),  // #0e1612 卡片
            bg_tertiary: Color::Rgb(22, 34, 28),   // #16221c 输入框
            fg: Color::Rgb(245, 243, 240),         // #f5f3f0 暖白
            fg_secondary: Color::Rgb(170, 165, 160), // #aaa5a0 灰白
            border: Color::Rgb(40, 55, 48),        // #283730 深绿边框
            border_active: Color::Rgb(90, 210, 160), // #5ad2a0 薄荷绿
            accent: Color::Rgb(110, 230, 180),     // #6ee6b4 亮薄荷
            accent_dim: Color::Rgb(70, 160, 125),  // #46a07d 中薄荷
            accent_subtle: Color::Rgb(45, 100, 80), // #2d6450 暗薄荷
            highlight_bg: Color::Rgb(28, 48, 40),  // #1c3028 选中背景
            highlight_fg: Color::Rgb(130, 245, 200), // #82f5c8 选中前景
            status_bg: Color::Rgb(10, 16, 13),     // #0a100d 状态栏
            status_fg: Color::Rgb(130, 135, 130),  // #828782 状态文字
            preview: Color::Rgb(120, 125, 120),    // #787d78 预览
            date: Color::Rgb(90, 95, 90),          // #5a5f5a 日期
            error: Color::Rgb(230, 110, 110),      // #e66e6e 柔和红
            warning: Color::Rgb(230, 190, 110),    // #e6be6e 柔和黄
            success: Color::Rgb(110, 210, 160),    // #6ed2a0 柔和绿
            shadow: Color::Rgb(4, 8, 6),           // #040806 阴影
            gradient_start: Color::Rgb(80, 200, 150),
            gradient_end: Color::Rgb(120, 240, 190),
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
