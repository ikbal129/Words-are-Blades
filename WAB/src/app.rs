use ratatui::{DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, layout::{Constraint, Layout}, widgets::{ListState}};

enum Mode {
    MainMenu
}

pub struct App {
    pub running: bool,
    panel: Mode,
    menu: Vec<String>,
    pub state: ListState,
    pub state_index: usize
}

impl App {
    pub fn new() -> Self {
        let app = Self {
            running: true,
            panel: Mode::MainMenu,
            menu: vec![],
            state: ListState::default(),
            state_index: 0
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
                    self.handle_input(key);
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
                self.next();
            }
            KeyCode::Up => {
                self.prev();
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
        }
    }

    fn next(&mut self) {
        if self.state_index + 1 >= self.menu.len() {
            self.state_index = 0;
        } else {
            self.state_index += 1;
        }

        self.state.select(Some(self.state_index));
    }

    fn prev(&mut self) {
        if self.state_index == 0 {
            self.state_index = self.menu.len() - 1;
        } else {
            self.state_index -= 1;
        }

        self.state.select(Some(self.state_index));
    }

    fn handle_enter(&mut self, keyword: &str) {

        match keyword {
            "Exit" => self.running = false,
            _ => {}
        }
    }
}