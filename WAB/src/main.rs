mod app;
mod ui;
use ratatui::DefaultTerminal;
use app::App;


fn main() {
    ratatui::run(|terminal: &mut DefaultTerminal| App::new().run(terminal)).unwrap();
}