use ratatui::crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::widgets::{Block, BorderType, List, ListItem, ListState, Paragraph};
use ratatui::text::{Line};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::{DefaultTerminal, Frame};

enum Panel {
    MainMenu
}

struct App {
    running: bool,
    page: Panel,
    menu: Vec<ListItem<'static>>,
    state: ListState,
    select: usize
}

fn main() {
    ratatui::run(|terminal: &mut DefaultTerminal| App::new().run(terminal)).unwrap()
}

impl App {
    fn new() -> Self {
        let app = Self {
            running: true,
            page: Panel::MainMenu,
            menu: vec![],
            state: ListState::default(),
            select: 0
        };

        app
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                self.render(frame);
            })?;

            if event::poll(std::time::Duration::from_secs(1))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key);
                }
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
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
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let panel = Block::bordered().border_type(BorderType::Rounded).fg(Color::LightBlue);
        let panel_area = panel.inner(frame.area().clone());
        frame.render_widget(panel, frame.area());
        let [title, main] = Layout::vertical([
            Constraint::Length(13),
            Constraint::Fill(1)
        ]).areas(panel_area);

        self.render_title(frame, title);
        self.render_main_menu(frame, main);
    }

    fn render_title(&mut self, frame: &mut Frame, area: Rect) {
        let style = Style::default()
        .fg(Color::LightBlue)
        .add_modifier(Modifier::BOLD);

        let title: Vec<Line> = vec![
            Line::styled(
                r"__      __                .___                              ___.   .__              .___             ",
                style,
            ),
            Line::styled(
                r"/  \    /  \___________  __| _/______ _____ _______   ____   \_ |__ |  | _____     __| _/____   ______",
                style,
            ),
            Line::styled(
                r"\   \/\/   /  _ \_  __ \/ __ |/  ___/ \__  \\_  __ \_/ __ \   | __ \|  | \__  \   / __ |/ __ \ /  ___/",
                style,
            ),
            Line::styled(
                r" \        (  <_> )  | \/ /_/ |\___ \   / __ \|  | \/\  ___/   | \_\ \  |__/ __ \_/ /_/ \  ___/ \___ \ ",
                style,
            ),
            Line::styled(
                r"  \__/\  / \____/|__|  \____ /____  > (____  /__|    \___  >  |___  /____(____  /\____ |\___  >____  >",
                style,
            ),
            Line::styled(
                r"       \/                   \/    \/       \/            \/       \/          \/      \/    \/     \/ ",
                style,
            ),
        ];
        frame.render_widget(Paragraph::new(title).alignment(Alignment::Center), area);
    }

    fn render_main_menu(&mut self, frame: &mut Frame, area: Rect) {
        self.state.select(Some(self.select));
        self.menu = vec![
            ListItem::new("Start new Battle"),
            ListItem::new("Credits"),
            ListItem::new("Exit")
        ];

        let [one, two] = Layout::horizontal([
            Constraint::Percentage(42),
            Constraint::Length(30),
        ]).areas(area);

        let list_menu = List::new(self.menu.clone())
            .style(Style::new().fg(Color::LightBlue))
            .highlight_style(Style::new().fg(Color::Gray))
            .highlight_symbol("> ");
        frame.render_stateful_widget(list_menu, two, &mut self.state);
    }

    fn next(&mut self) {
        if self.select + 1 >= self.menu.len() {
            self.select = 0;
        } else {
            self.select += 1;
        }

        self.state.select(Some(self.select));
    }

    fn prev(&mut self) {
        if self.select == 0 {
            self.select = self.menu.len() - 1;
        } else {
            self.select -= 1;
        }

        self.state.select(Some(self.select));
    }
}