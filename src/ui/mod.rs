use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};

use crate::models::{EditMode, ViewState};
use crate::ui::theme::Theme;

pub mod theme;

pub struct UI {
    theme: Theme,
}

impl UI {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }

    /// 绘制 ASCII Logo - 更精致的版本
    fn draw_logo(&self, theme: &Theme) -> Text {
        Text::from(vec![
            Line::from(vec![
                Span::styled("    ╔══════════════════════════════════════════════════╗", theme.border_subtle_style()),
            ]),
            Line::from(vec![
                Span::styled("    ║  ", theme.border_subtle_style()),
                Span::styled("██╗  ██╗████████╗███████╗██████╗ ███╗   ███╗", theme.accent_style()),
                Span::styled("  ║", theme.border_subtle_style()),
            ]),
            Line::from(vec![
                Span::styled("    ║  ", theme.border_subtle_style()),
                Span::styled("╚██╗██╔╝╚══██╔══╝██╔════╝██╔══██╗████╗ ████║", theme.accent_dim_style()),
                Span::styled("  ║", theme.border_subtle_style()),
            ]),
            Line::from(vec![
                Span::styled("    ║  ", theme.border_subtle_style()),
                Span::styled(" ╚███╔╝    ██║   █████╗  ██████╔╝██╔████╔██║", theme.accent_style()),
                Span::styled("  ║", theme.border_subtle_style()),
            ]),
            Line::from(vec![
                Span::styled("    ║  ", theme.border_subtle_style()),
                Span::styled(" ██╔██╗    ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║", theme.accent_dim_style()),
                Span::styled("  ║", theme.border_subtle_style()),
            ]),
            Line::from(vec![
                Span::styled("    ║  ", theme.border_subtle_style()),
                Span::styled("██╔╝ ██╗   ██║   ███████╗██║  ██║██║ ╚═╝ ██║", theme.accent_style()),
                Span::styled("  ║", theme.border_subtle_style()),
            ]),
            Line::from(vec![
                Span::styled("    ║  ", theme.border_subtle_style()),
                Span::styled("╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝", theme.accent_dim_style()),
                Span::styled("  ║", theme.border_subtle_style()),
            ]),
            Line::from(vec![
                Span::styled("    ╚══════════════════════════════════════════════════╝", theme.border_subtle_style()),
            ]),
        ])
    }

    /// 绘制装饰分隔线
    fn draw_separator(&self, theme: &Theme, width: usize) -> Line {
        let left = "╾─╼ ".to_string();
        let right = " ╾─╼".to_string();
        let middle_len = width.saturating_sub(left.len() + right.len());
        let middle = "─".repeat(middle_len);
        Line::from(vec![
            Span::styled(left, theme.border_active_style()),
            Span::styled(middle, theme.border_style()),
            Span::styled(right, theme.border_active_style()),
        ])
    }

    pub fn draw(&self, frame: &mut Frame, state: &crate::app::State) {
        // 设置全局背景色
        frame.render_widget(
            Block::default().style(self.theme.content_style()),
            frame.size(),
        );

        match state.view {
            ViewState::List => self.draw_list(frame, state),
            ViewState::View => self.draw_view(frame, state),
            ViewState::Edit => self.draw_edit(frame, state),
            ViewState::Search => self.draw_search(frame, state),
            ViewState::Help => self.draw_help(frame),
        }
    }

    /// 绘制装饰性角落
    fn draw_corners(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let corners = vec![
            (area.x, area.y, "┌"),
            (area.x + area.width - 1, area.y, "┐"),
            (area.x, area.y + area.height - 1, "└"),
            (area.x + area.width - 1, area.y + area.height - 1, "┘"),
        ];
        for (x, y, c) in corners {
            frame.render_widget(
                Paragraph::new(Span::styled(c, theme.border_style())),
                Rect::new(x, y, 1, 1),
            );
        }
    }

    /// 绘制双行分隔线
    fn draw_double_separator(&self, theme: &Theme, width: usize) -> Line {
        let left = "╠══ ".to_string();
        let right = " ══╣".to_string();
        let middle_len = width.saturating_sub(left.len() + right.len());
        let middle = "═".repeat(middle_len);
        Line::from(vec![
            Span::styled(left, theme.border_active_style()),
            Span::styled(middle, theme.border_style()),
            Span::styled(right, theme.border_active_style()),
        ])
    }

    /// 绘制列表视图
    fn draw_list(&self, frame: &mut Frame, state: &crate::app::State) {
        let area = frame.size();
        let theme = &self.theme;

        // 绘制外边框角落
        self.draw_corners(frame, area, theme);

        // 整体布局
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),   // 顶部边距
                Constraint::Length(9),   // ASCII Logo 区域
                Constraint::Length(1),   // 分隔线
                Constraint::Min(5),      // 列表区
                Constraint::Length(2),   // 状态栏
            ])
            .margin(2)
            .split(area);

        // 顶部 ASCII Logo
        let logo = self.draw_logo(theme);
        let logo_para = Paragraph::new(logo).alignment(Alignment::Center);
        frame.render_widget(logo_para, chunks[1]);

        // 装饰分隔线
        let separator = self.draw_separator(theme, area.width as usize - 8);
        let sep_para = Paragraph::new(separator).alignment(Alignment::Center);
        frame.render_widget(sep_para, chunks[2]);

        // 笔记列表
        let list_items: Vec<ListItem> = state
            .notes
            .iter()
            .enumerate()
            .map(|(idx, note)| {
                let is_selected = state.selected == Some(idx);

                // 构建笔记项内容
                let mut spans = vec![];

                // 选中标记
                if is_selected {
                    spans.push(Span::styled("▸ ", theme.accent_style()));
                } else {
                    spans.push(Span::styled("  ", theme.content_style()));
                }

                // 标题
                let title_style = if is_selected {
                    theme.note_title_selected_style()
                } else {
                    theme.note_title_style()
                };
                spans.push(Span::styled(&note.title, title_style));

                // 标签（如果有）
                if !note.tags.is_empty() {
                    let tag_str = format!("  {}", note.tags.join(", "));
                    spans.push(Span::styled(tag_str, theme.accent_dim_style()));
                }

                // 笔记图标根据选中状态变化
                let note_icon = if is_selected { "◆ " } else { "◇ " };
                spans.insert(0, Span::styled(note_icon.to_string(),
                    if is_selected { theme.accent_style() } else { theme.ornament_style() }));

                // 添加装饰箭头
                if is_selected {
                    spans.push(Span::styled(" ◀", theme.accent_subtle_style()));
                }

                let lines = vec![
                    Line::from(spans),
                    Line::from(vec![
                        Span::styled(if is_selected { "   ╰─▸ " } else { "   ├─▸ " },
                            if is_selected { theme.border_active_style() } else { theme.border_style() }),
                        Span::styled(note.preview(40), if is_selected { theme.preview_selected_style() } else { theme.preview_style() }),
                        Span::styled(" ◆ ", theme.ornament_style()),
                        Span::styled(note.format_date(), theme.date_style()),
                    ]),
                ];

                let style = if is_selected {
                    theme.selected_row_style()
                } else {
                    theme.content_style()
                };

                ListItem::new(lines).style(style)
            })
            .collect();

        // 列表容器 - 带装饰边框
        let list_title = if state.notes.is_empty() {
            " ◆ 笔记列表 ◆ ".to_string()
        } else {
            format!(" ◆ 笔记列表 [{}] ◆ ", state.notes.len())
        };

        let list_block = Block::default()
            .title(list_title)
            .title_style(theme.accent_style())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(theme.border_style())
            .padding(Padding::horizontal(2));

        let list = List::new(list_items)
            .block(list_block)
            .highlight_style(theme.highlight_style())
            .highlight_symbol("");

        let mut list_state = ratatui::widgets::ListState::default();
        list_state.select(state.selected);

        frame.render_stateful_widget(list, chunks[3], &mut list_state);

        // 底部状态栏 - 按键提示（带装饰）
        let status_line = if state.notes.is_empty() {
            vec![
                Span::styled("╾──────  ", theme.separator_style()),
                Span::styled("[", theme.status_style()),
                Span::styled("n", theme.key_style()),
                Span::styled("]新建笔记  ", theme.status_style()),
                Span::styled("[", theme.status_style()),
                Span::styled("q", theme.key_style()),
                Span::styled("]退出  ", theme.status_style()),
                Span::styled("──────╼", theme.separator_style()),
            ]
        } else {
            vec![
                Span::styled("╾──  ", theme.separator_style()),
                Span::styled("[", theme.status_style()),
                Span::styled("j/k", theme.key_style()),
                Span::styled("]选择  ", theme.status_style()),
                Span::styled("[", theme.status_style()),
                Span::styled("Enter", theme.key_style()),
                Span::styled("]查看  ", theme.status_style()),
                Span::styled("[", theme.status_style()),
                Span::styled("n", theme.key_style()),
                Span::styled("]新建  ", theme.status_style()),
                Span::styled("[", theme.status_style()),
                Span::styled("/", theme.key_style()),
                Span::styled("]搜索  ", theme.status_style()),
                Span::styled("[", theme.status_style()),
                Span::styled("d", theme.key_style()),
                Span::styled("]删除  ", theme.status_style()),
                Span::styled("──╼", theme.separator_style()),
            ]
        };

        let status = Paragraph::new(Line::from(status_line))
            .alignment(Alignment::Center);
        frame.render_widget(status, chunks[4]);

        // 底部装饰线
        let sep = self.draw_separator(theme, area.width as usize - 8);
        let sep_para = Paragraph::new(sep).alignment(Alignment::Center);
        let sep_area = Rect::new(
            area.x + 4,
            area.y + area.height - 2,
            area.width - 8,
            1
        );
        frame.render_widget(sep_para, sep_area);
    }

    /// 绘制查看视图
    fn draw_view(&self, frame: &mut Frame, state: &crate::app::State) {
        let area = frame.size();
        let theme = &self.theme;

        // 绘制外边框角落
        self.draw_corners(frame, area, theme);

        if let Some(note) = &state.current_note {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Length(5),
                    Constraint::Length(1),
                    Constraint::Min(5),
                    Constraint::Length(2),
                ])
                .margin(2)
                .split(area);

            // 标题栏 - 带装饰边框
            let title_block = Block::default()
                .title(format!(" ◆ {} ◆ ", &note.title))
                .title_style(theme.title_style())
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(theme.border_active_style())
                .padding(Padding::uniform(1));

            let title_content = vec![
                Line::from(vec![
                    Span::styled("  ◆ 更新时间: ", theme.content_secondary_style()),
                    Span::styled(note.format_date(), theme.date_style()),
                ]),
                Line::from(vec![
                    Span::styled("  ◆ ID: ", theme.content_secondary_style()),
                    Span::styled(&note.id[..8.min(note.id.len())], theme.preview_style()),
                ]),
                Line::from(vec![
                    Span::styled("  ◆ 字数: ", theme.content_secondary_style()),
                    Span::styled(format!("{}", note.content.len()), theme.preview_style()),
                ]),
            ];
            let title = Paragraph::new(title_content).block(title_block);
            frame.render_widget(title, chunks[1]);

            // 内容区 - 带行号效果
            let content_lines: Vec<Line> = note.content
                .lines()
                .enumerate()
                .map(|(i, line)| {
                    Line::from(vec![
                        Span::styled(format!("{:3} │ ", i + 1), theme.border_subtle_style()),
                        Span::styled(line.to_string(), theme.content_style()),
                    ])
                })
                .collect();

            let content = Paragraph::new(Text::from(content_lines))
                .block(
                    Block::default()
                        .title(" ◆ 内容 ◆ ")
                        .title_style(theme.accent_style())
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_style(theme.border_style())
                        .padding(Padding::horizontal(1)),
                );
            frame.render_widget(content, chunks[3]);

            // 底部状态栏
            let status_spans = vec![
                Span::styled("╾──────────  ", theme.separator_style()),
                Span::styled("[", theme.status_style()),
                Span::styled("e", theme.key_style()),
                Span::styled("]编辑笔记  ", theme.status_style()),
                Span::styled("[", theme.status_style()),
                Span::styled("Esc", theme.key_style()),
                Span::styled("]返回列表  ", theme.status_style()),
                Span::styled("──────────╼", theme.separator_style()),
            ];
            let status = Paragraph::new(Line::from(status_spans)).alignment(Alignment::Center);
            frame.render_widget(status, chunks[4]);
        }
    }

    /// 绘制编辑视图
    fn draw_edit(&self, frame: &mut Frame, state: &crate::app::State) {
        let area = frame.size();
        let theme = &self.theme;

        // 绘制外边框角落
        self.draw_corners(frame, area, theme);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(4),
                Constraint::Length(1),
                Constraint::Min(8),
                Constraint::Length(2),
            ])
            .margin(2)
            .split(area);

        // 标题输入区
        let is_title_active = state.edit_field == 0;
        let title_text = if state.edit_mode == EditMode::Insert && is_title_active {
            format!("{}_", state.edit_title)
        } else {
            state.edit_title.clone()
        };

        let title_style = if is_title_active {
            theme.input_active_style()
        } else {
            theme.input_style()
        };

        let title_indicator = if is_title_active { " ◆ 标题 ◆ " } else { " ◇ 标题 ◇ " };
        let title_block = Block::default()
            .title(title_indicator)
            .title_style(if is_title_active { theme.accent_style() } else { theme.subtitle_style() })
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(if is_title_active { theme.border_active_style() } else { theme.border_style() })
            .padding(Padding::horizontal(2));

        let title = Paragraph::new(title_text).style(title_style).block(title_block);
        frame.render_widget(title, chunks[1]);

        // 内容输入区
        let is_content_active = state.edit_field == 1;
        let content_text = if state.edit_content.is_empty() && !is_content_active {
            "请输入内容...".to_string()
        } else if state.edit_mode == EditMode::Insert && is_content_active {
            format!("{}_", state.edit_content)
        } else {
            state.edit_content.clone()
        };

        let content_style = if state.edit_content.is_empty() && !is_content_active {
            theme.placeholder_style()
        } else if is_content_active {
            theme.input_active_style()
        } else {
            theme.input_style()
        };

        let content_indicator = if is_content_active { " ◆ 内容 ◆ " } else { " ◇ 内容 ◇ " };
        let content_block = Block::default()
            .title(content_indicator)
            .title_style(if is_content_active { theme.accent_style() } else { theme.subtitle_style() })
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(if is_content_active { theme.border_active_style() } else { theme.border_style() })
            .padding(Padding::uniform(2));

        let content = Paragraph::new(content_text).style(content_style).block(content_block);
        frame.render_widget(content, chunks[3]);

        // 模式指示器和帮助
        let mode_str = match state.edit_mode {
            EditMode::Normal => "NORMAL",
            EditMode::Insert => "INSERT",
        };

        // 底部状态栏 - 根据模式显示不同提示
        let status_spans = if state.edit_mode == EditMode::Normal {
            vec![
                Span::styled("╾─╼ 模式:", theme.status_style()),
                Span::styled(format!(" {} ", mode_str), theme.mode_style(mode_str)),
                Span::styled("[", theme.status_style()),
                Span::styled("i", theme.key_style()),
                Span::styled("]插入 [", theme.status_style()),
                Span::styled("Tab", theme.key_style()),
                Span::styled("]切换 [", theme.status_style()),
                Span::styled("Ctrl+S", theme.key_style()),
                Span::styled("]保存 [", theme.status_style()),
                Span::styled("Esc", theme.key_style()),
                Span::styled("]返回 ╼─╾", theme.status_style()),
            ]
        } else {
            vec![
                Span::styled("╾─╼ 模式:", theme.status_style()),
                Span::styled(format!(" {} ", mode_str), theme.mode_style(mode_str)),
                Span::styled("[", theme.status_style()),
                Span::styled("Tab", theme.key_style()),
                Span::styled("]切换 [", theme.status_style()),
                Span::styled("Ctrl+S", theme.key_style()),
                Span::styled("]保存 [", theme.status_style()),
                Span::styled("Esc", theme.key_style()),
                Span::styled("]返回 ╼─╾", theme.status_style()),
            ]
        };
        let status = Paragraph::new(Line::from(status_spans)).alignment(Alignment::Center);
        frame.render_widget(status, chunks[4]);
    }

    /// 绘制搜索视图
    fn draw_search(&self, frame: &mut Frame, state: &crate::app::State) {
        let area = frame.size();
        let theme = &self.theme;

        // 绘制外边框角落
        self.draw_corners(frame, area, theme);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Min(5),
                Constraint::Length(2),
            ])
            .margin(2)
            .split(area);

        // 搜索输入框
        let search_block = Block::default()
            .title(" ◆ 搜索 ◆ ")
            .title_style(theme.accent_style())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(theme.border_active_style())
            .padding(Padding::horizontal(2));

        let search_input = Paragraph::new(format!("◇ {}", state.search_query))
            .style(theme.accent_normal_style())
            .block(search_block);
        frame.render_widget(search_input, chunks[1]);

        // 搜索结果列表
        let list_items: Vec<ListItem> = state
            .search_results
            .iter()
            .enumerate()
            .map(|(idx, note)| {
                let is_selected = state.search_selected == Some(idx);

                let mut spans = vec![];
                if is_selected {
                    spans.push(Span::styled("▸ ", theme.accent_style()));
                } else {
                    spans.push(Span::styled("  ", theme.content_style()));
                }

                spans.push(Span::styled(&note.title, if is_selected { theme.note_title_selected_style() } else { theme.note_title_style() }));

                let lines = vec![
                    Line::from(spans),
                    Line::from(vec![
                        Span::styled("   ", theme.content_style()),
                        Span::styled(note.preview(45), theme.preview_style()),
                    ]),
                ];

                let style = if is_selected {
                    theme.selected_row_style()
                } else {
                    theme.content_style()
                };

                ListItem::new(lines).style(style)
            })
            .collect();

        let result_count = state.search_results.len();
        let list_title = if result_count == 0 {
            " ◆ 搜索结果 ◆ ".to_string()
        } else {
            format!(" ◆ 搜索结果 [{}] ◆ ", result_count)
        };
        let list_block = Block::default()
            .title(list_title)
            .title_style(theme.accent_style())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(theme.border_style())
            .padding(Padding::horizontal(2));

        let list = List::new(list_items)
            .block(list_block)
            .highlight_style(theme.highlight_style());

        let mut list_state = ratatui::widgets::ListState::default();
        list_state.select(state.search_selected);

        frame.render_stateful_widget(list, chunks[3], &mut list_state);

        // 底部提示
        let status_spans = vec![
            Span::styled("╾──────────  ", theme.separator_style()),
            Span::styled("[", theme.status_style()),
            Span::styled("j/k", theme.key_style()),
            Span::styled("]选择  ", theme.status_style()),
            Span::styled("[", theme.status_style()),
            Span::styled("Enter", theme.key_style()),
            Span::styled("]打开  ", theme.status_style()),
            Span::styled("[", theme.status_style()),
            Span::styled("Esc", theme.key_style()),
            Span::styled("]取消  ", theme.status_style()),
            Span::styled("──────────╼", theme.separator_style()),
        ];
        let status = Paragraph::new(Line::from(status_spans)).alignment(Alignment::Center);
        frame.render_widget(status, chunks[4]);
    }

    /// 绘制帮助视图
    fn draw_help(&self, frame: &mut Frame) {
        let area = frame.size();
        let theme = &self.theme;

        let popup_area = centered_rect(65, 85, area);

        // 清除背景
        frame.render_widget(Clear, popup_area);

        let help_text = Text::from(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  ╔═══════════════════════════╗", theme.border_style()),
            ]),
            Line::from(vec![
                Span::styled("  ║  ◆ ", theme.border_style()),
                Span::styled("快捷键帮助", theme.help_header_style()),
                Span::styled(" ◆  ║", theme.border_style()),
            ]),
            Line::from(vec![
                Span::styled("  ╚═══════════════════════════╝", theme.border_style()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  ◆ ", theme.ornament_style()),
                Span::styled("全局", theme.help_category_style()),
            ]),
            Line::from(vec![
                Span::styled("    [", theme.help_text_style()),
                Span::styled("q", theme.key_style()),
                Span::styled("]      退出程序", theme.help_text_style()),
            ]),
            Line::from(vec![
                Span::styled("    [", theme.help_text_style()),
                Span::styled("?/h", theme.key_style()),
                Span::styled("]    显示帮助", theme.help_text_style()),
            ]),
            Line::from(vec![
                Span::styled("    [", theme.help_text_style()),
                Span::styled("Esc", theme.key_style()),
                Span::styled("]    返回/取消", theme.help_text_style()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  ◆ ", theme.ornament_style()),
                Span::styled("列表视图", theme.help_category_style()),
            ]),
            Line::from(vec![
                Span::styled("    [", theme.help_text_style()),
                Span::styled("j/k", theme.key_style()),
                Span::styled("]    上下移动", theme.help_text_style()),
            ]),
            Line::from(vec![
                Span::styled("    [", theme.help_text_style()),
                Span::styled("Enter", theme.key_style()),
                Span::styled("]  查看笔记", theme.help_text_style()),
            ]),
            Line::from(vec![
                Span::styled("    [", theme.help_text_style()),
                Span::styled("n", theme.key_style()),
                Span::styled("]      新建笔记", theme.help_text_style()),
            ]),
            Line::from(vec![
                Span::styled("    [", theme.help_text_style()),
                Span::styled("/", theme.key_style()),
                Span::styled("]      搜索笔记", theme.help_text_style()),
            ]),
            Line::from(vec![
                Span::styled("    [", theme.help_text_style()),
                Span::styled("d", theme.key_style()),
                Span::styled("]      删除笔记", theme.help_text_style()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  ◆ ", theme.ornament_style()),
                Span::styled("查看笔记", theme.help_category_style()),
            ]),
            Line::from(vec![
                Span::styled("    [", theme.help_text_style()),
                Span::styled("e", theme.key_style()),
                Span::styled("]      编辑笔记", theme.help_text_style()),
            ]),
            Line::from(vec![
                Span::styled("    [", theme.help_text_style()),
                Span::styled("Esc/q", theme.key_style()),
                Span::styled("] 返回列表", theme.help_text_style()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  ◆ ", theme.ornament_style()),
                Span::styled("编辑模式", theme.help_category_style()),
            ]),
            Line::from(vec![
                Span::styled("    [", theme.help_text_style()),
                Span::styled("i", theme.key_style()),
                Span::styled("]      插入模式", theme.help_text_style()),
            ]),
            Line::from(vec![
                Span::styled("    [", theme.help_text_style()),
                Span::styled("Esc", theme.key_style()),
                Span::styled("]    普通模式", theme.help_text_style()),
            ]),
            Line::from(vec![
                Span::styled("  [", theme.help_text_style()),
                Span::styled("Tab", theme.key_style()),
                Span::styled("]      切换字段", theme.help_text_style()),
            ]),
            Line::from(vec![
                Span::styled("  [", theme.help_text_style()),
                Span::styled("Ctrl+S", theme.key_style()),
                Span::styled("]   保存笔记", theme.help_text_style()),
            ]),
            Line::from(""),
        ]);

        let help = Paragraph::new(help_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(" ◆ 帮助 ◆ ")
                    .title_style(theme.accent_style())
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(theme.border_active_style())
                    .padding(Padding::uniform(2)),
            );
        frame.render_widget(help, popup_area);
    }
}

/// 计算居中矩形
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
