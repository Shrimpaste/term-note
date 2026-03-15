use anyhow::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

mod app;
mod db;
mod models;
mod ui;

use app::App;

fn main() -> Result<()> {
    // 初始化 panic hook
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        original_hook(info);
    }));

    // 初始化数据库
    let db = db::Database::new()?;
    db.init()?;

    // 创建应用
    let app = App::new(db)?;

    // 运行
    enable_raw_mode()?;
    let result = app.run();
    disable_raw_mode()?;

    result
}