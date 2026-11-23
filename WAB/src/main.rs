mod app;
mod ui;
mod utils;
mod setup;
use ratatui::DefaultTerminal;
use app::App;


fn main() {
    ratatui::run(|terminal: &mut DefaultTerminal| App::new().run(terminal)).unwrap();
}