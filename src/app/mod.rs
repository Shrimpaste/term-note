use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io::stdout;

use crate::{
    db::Database,
    models::{EditMode, Note, ViewState},
    ui::UI,
};

pub struct State {
    pub view: ViewState,
    pub notes: Vec<Note>,
    pub selected: Option<usize>,
    pub current_note: Option<Note>,
    pub edit_mode: EditMode,
    pub edit_field: usize,
    pub edit_title: String,
    pub edit_content: String,
    pub editing_id: Option<String>,
    pub search_query: String,
    pub search_results: Vec<Note>,
    pub search_selected: Option<usize>,
}

impl State {
    fn new() -> Self {
        Self {
            view: ViewState::List,
            notes: Vec::new(),
            selected: Some(0),
            current_note: None,
            edit_mode: EditMode::Normal,
            edit_field: 0,
            edit_title: String::new(),
            edit_content: String::new(),
            editing_id: None,
            search_query: String::new(),
            search_results: Vec::new(),
            search_selected: Some(0),
        }
    }
}

pub struct App {
    db: Database,
    state: State,
    ui: UI,
}

impl App {
    pub fn new(db: Database) -> Result<Self> {
        let mut state = State::new();
        state.notes = db.get_all_notes()?;
        if state.notes.is_empty() {
            state.selected = None;
        }
        Ok(Self {
            db,
            state,
            ui: UI::new(),
        })
    }

    pub fn run(mut self) -> Result<()> {
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        stdout().execute(Clear(ClearType::All))?;

        loop {
            terminal.draw(|f| {
                self.ui.draw(f, &self.state);
            })?;

            if let Event::Key(key) = event::read()? {
                // 处理按键事件（兼容不同终端）
                match key.kind {
                    KeyEventKind::Press | KeyEventKind::Repeat => {
                        if !self.handle_key(key.code, key.modifiers)? {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn handle_key(
        &mut self,
        key: KeyCode,
        modifiers: KeyModifiers,
    ) -> Result<bool> {
        match self.state.view {
            ViewState::List => self.handle_list(key),
            ViewState::View => self.handle_view(key),
            ViewState::Edit => self.handle_edit(key, modifiers),
            ViewState::Search => self.handle_search(key),
            ViewState::Help => self.handle_help(key),
        }
    }

    fn handle_list(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Char('q') => return Ok(false),
            KeyCode::Char('?') | KeyCode::Char('h') => {
                self.state.view = ViewState::Help;
            }
            KeyCode::Char('j') | KeyCode::Down => self.next(),
            KeyCode::Char('k') | KeyCode::Up => self.prev(),
            KeyCode::Enter => {
                if let Some(idx) = self.state.selected {
                    if let Some(note) = self.state.notes.get(idx) {
                        self.state.current_note = Some(Note {
                            id: note.id.clone(),
                            title: note.title.clone(),
                            content: note.content.clone(),
                            tags: note.tags.clone(),
                            created_at: note.created_at,
                            updated_at: note.updated_at,
                        });
                        self.state.view = ViewState::View;
                    }
                }
            }
            KeyCode::Char('n') => {
                self.state.editing_id = None;
                self.state.edit_title.clear();
                self.state.edit_content.clear();
                self.state.edit_field = 0;
                self.state.edit_mode = EditMode::Insert;
                self.state.view = ViewState::Edit;
            }
            KeyCode::Char('/') => {
                self.state.search_query.clear();
                self.state.search_results.clear();
                self.state.view = ViewState::Search;
            }
            KeyCode::Char('d') => {
                if let Some(idx) = self.state.selected {
                    if let Some(note) = self.state.notes.get(idx) {
                        let id = note.id.clone();
                        self.db.delete_note(&id)?;
                        self.state.notes = self.db.get_all_notes()?;
                        if self.state.notes.is_empty() {
                            self.state.selected = None;
                        } else if idx >= self.state.notes.len() {
                            self.state.selected = Some(self.state.notes.len() - 1);
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(true)
    }

    fn handle_view(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.state.view = ViewState::List;
                self.state.current_note = None;
            }
            KeyCode::Char('e') => {
                if let Some(note) = &self.state.current_note {
                    self.state.editing_id = Some(note.id.clone());
                    self.state.edit_title = note.title.clone();
                    self.state.edit_content = note.content.clone();
                    self.state.edit_field = 0;
                    self.state.edit_mode = EditMode::Normal;
                    self.state.view = ViewState::Edit;
                }
            }
            _ => {}
        }
        Ok(true)
    }

    fn handle_edit(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<bool> {
        if modifiers == KeyModifiers::CONTROL && key == KeyCode::Char('s') {
            self.save_note()?;
            return Ok(true);
        }

        match self.state.edit_mode {
            EditMode::Normal => match key {
                KeyCode::Esc => {
                    self.state.view = ViewState::List;
                }
                KeyCode::Char('i') => {
                    self.state.edit_mode = EditMode::Insert;
                }
                KeyCode::Tab => {
                    self.state.edit_field = 1 - self.state.edit_field;
                }
                _ => {}
            },
            EditMode::Insert => match key {
                KeyCode::Esc => {
                    self.state.edit_mode = EditMode::Normal;
                }
                KeyCode::Char(c) => {
                    if self.state.edit_field == 0 {
                        self.state.edit_title.push(c);
                    } else {
                        self.state.edit_content.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if self.state.edit_field == 0 {
                        self.state.edit_title.pop();
                    } else {
                        self.state.edit_content.pop();
                    }
                }
                KeyCode::Enter => {
                    if self.state.edit_field == 1 {
                        self.state.edit_content.push('\n');
                    }
                }
                KeyCode::Tab => {
                    self.state.edit_field = 1 - self.state.edit_field;
                }
                _ => {}
            },
        }
        Ok(true)
    }

    fn handle_search(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Esc => {
                if self.state.search_query.is_empty() {
                    self.state.view = ViewState::List;
                } else {
                    self.state.search_query.clear();
                    self.state.search_results.clear();
                }
            }
            KeyCode::Enter => {
                if let Some(idx) = self.state.search_selected {
                    if let Some(note) = self.state.search_results.get(idx) {
                        self.state.current_note = Some(Note {
                            id: note.id.clone(),
                            title: note.title.clone(),
                            content: note.content.clone(),
                            tags: note.tags.clone(),
                            created_at: note.created_at,
                            updated_at: note.updated_at,
                        });
                        self.state.view = ViewState::View;
                    }
                }
            }
            KeyCode::Char(c) => {
                self.state.search_query.push(c);
                self.perform_search();
            }
            KeyCode::Backspace => {
                self.state.search_query.pop();
                self.perform_search();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.state.search_results.is_empty() {
                    let new_idx = match self.state.search_selected {
                        Some(i) => (i + 1).min(self.state.search_results.len() - 1),
                        None => 0,
                    };
                    self.state.search_selected = Some(new_idx);
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if !self.state.search_results.is_empty() {
                    let new_idx = match self.state.search_selected {
                        Some(i) => i.saturating_sub(1),
                        None => 0,
                    };
                    self.state.search_selected = Some(new_idx);
                }
            }
            _ => {}
        }
        Ok(true)
    }

    fn handle_help(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => {
                self.state.view = ViewState::List;
            }
            _ => {}
        }
        Ok(true)
    }

    fn next(&mut self) {
        if self.state.notes.is_empty() {
            return;
        }
        let new_idx = match self.state.selected {
            Some(i) => {
                if i >= self.state.notes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.selected = Some(new_idx);
    }

    fn prev(&mut self) {
        if self.state.notes.is_empty() {
            return;
        }
        let new_idx = match self.state.selected {
            Some(i) => {
                if i == 0 {
                    self.state.notes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.selected = Some(new_idx);
    }

    fn save_note(&mut self) -> Result<()> {
        let note = if let Some(id) = &self.state.editing_id {
            Note {
                id: id.clone(),
                title: self.state.edit_title.clone(),
                content: self.state.edit_content.clone(),
                tags: Vec::new(),
                created_at: chrono::Local::now(),
                updated_at: chrono::Local::now(),
            }
        } else {
            Note::new(&self.state.edit_title, &self.state.edit_content)
        };

        self.db.create_note(&note)?;
        self.state.notes = self.db.get_all_notes()?;
        self.state.view = ViewState::List;
        Ok(())
    }

    fn perform_search(&mut self) {
        if self.state.search_query.is_empty() {
            self.state.search_results.clear();
            return;
        }
        if let Ok(results) = self.db.search(&self.state.search_query) {
            self.state.search_results = results;
            self.state.search_selected = if self.state.search_results.is_empty() {
                None
            } else {
                Some(0)
            };
        }
    }
}