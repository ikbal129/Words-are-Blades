use ratatui::{DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, layout::{Constraint, Layout}, widgets::{ListState}};
use crate::setup::{GameSetup};
use crate::utils::arrow;
enum Mode {
    MainMenu,
    GameSetup
}

pub struct App {
    pub running: bool,
    panel: Mode,
    menu: Vec<String>,
    pub state: ListState,
    pub state_index: usize,
    setup: GameSetup
}

impl App {
    pub fn new() -> Self {
        let app = Self {
            running: true,
            panel: Mode::MainMenu,
            menu: vec![],
            state: ListState::default(),
            state_index: 0,
            setup: GameSetup::new()
        };

        app
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while self.running {
            terminal.draw(|frame: &mut Frame| {
                self.render(frame);
            })?;

            if event::poll(std::time::Duration::from_millis(10))? {
                if let Event::Key(key) = event::read()? {
                    match self.panel {
                        Mode::MainMenu => self.handle_input(key),
                        Mode::GameSetup => {
                            let is_back = self.setup.handle_key(key);
                            if is_back {
                                self.panel = Mode::MainMenu;
                            }
                        },
                    }
                }
            }
        }
        Ok(())
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Esc => self.running = false,
            KeyCode::Down => {
                self.state_index = arrow::next(self.menu.clone(), self.state_index);
                self.state.select(Some(self.state_index));
            }
            KeyCode::Up => {
                self.state_index = arrow::prev(self.menu.clone(), self.state_index);
                self.state.select(Some(self.state_index));
            }
            KeyCode::Enter => {
                self.handle_enter(self.menu[self.state_index].clone().as_str());
            }
            _ => {}
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        match self.panel {
            Mode::MainMenu => {
                self.state.select(Some(self.state_index));
                let [title, main, guide] = Layout::vertical([
                    Constraint::Length(14),
                    Constraint::Length(12),
                    Constraint::Length(3)
                ]).areas(frame.area());

                self.menu = vec![
                    "Start new battle".to_string(),
                    "Codex / Grimoire".to_string(),
                    "Credits".to_string(),
                    "Exit".to_string()
                ];
                    
                self.render_title(frame, title);
                self.render_menu(frame, main, self.menu.clone());
                self.render_guide(frame, guide);
            }
            Mode::GameSetup => {
                self.setup.difficulty_state.select(Some(self.setup.diff_index));
                self.setup.chartype_state.select(Some(self.setup.chartype_index));
                let [title, setup, detail, guide] = Layout::vertical([
                    Constraint::Length(14),
                    Constraint::Length(7),
                    Constraint::Length(20),
                    Constraint::Length(3)
                ]).areas(frame.area());

                self.render_title(frame, title);
                self.setup.render_setup(frame, setup);
                self.setup.render_details(frame, detail);
                self.setup.render_guide(frame, guide);
            }
        }
    }

    fn handle_enter(&mut self, keyword: &str) {

        match keyword {
            "Exit" => self.running = false,
            "Start new battle" => self.panel = Mode::GameSetup,
            _ => {}
        }
    }
}