use ratatui::{Frame, crossterm::event::{KeyCode, KeyEvent, KeyEventKind}, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style}, widgets::{Block, BorderType, List, ListItem, ListState}};
#[derive(PartialEq)]
enum Step {
    Difficulty,
    CharType
}
pub struct GameSetup {
    difficulty_menu: Vec<String>,
    chartype_menu: Vec<String>,
    pub difficulty_state: ListState,
    pub diff_index: usize,
    pub chartype_state: ListState,
    pub chartype_index: usize,
    step: Step
}

impl GameSetup{
    pub fn new() -> Self {
        Self {
            difficulty_menu: vec!["Easy".to_string(), "Normal".to_string(), "Hard".to_string()],
            difficulty_state: ListState::default(),
            diff_index: 0,
            chartype_menu: vec!["Fighter".to_string(), "Mage".to_string(), "Tank".to_string(), "Support".to_string(), "Miner".to_string()],
            chartype_state: ListState::default(),
            chartype_index: 0,
            step: Step::Difficulty
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if key.kind != KeyEventKind::Press {
            return false;
        }

        match key.code {
            KeyCode::Enter => {
                self.step = Step::CharType;
                false
            }
            KeyCode::Down => {
                self.next();
                false
            }
            KeyCode::Up => {
                self.prev();
                false
            }
            KeyCode::Esc => {
                if self.step == Step::CharType {
                    self.step = Step::Difficulty;
                    false
                } else {
                    true
                }


            }
            _ => { false }
        }
    }

    pub fn render_setup(&mut self, frame: &mut Frame, area: Rect) {
        let [difficulty, chartype] = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]).areas(area);

        //difficulty box
        let diff_block = Block::bordered().border_type(BorderType::Rounded).title(" Difficulty ");
        let diff_items: Vec<ListItem> = self.difficulty_menu
            .iter()
            .map(|item| {
                ListItem::new(item.to_string())
            })
            .collect();
        let diff_list = List::new(diff_items)
            .block(diff_block)
            .highlight_style(Style::new()
                .fg(Color::Cyan).add_modifier(Modifier::BOLD)
            )
            .highlight_symbol(">> ");
        frame.render_stateful_widget(diff_list, difficulty, &mut self.difficulty_state);

        //chartype box
        let char_box = Block::bordered().border_type(BorderType::Rounded).title(" Characters Type ");
        let char_items: Vec<ListItem> = self.chartype_menu
            .iter()
            .map(|item| {
                ListItem::new(item.to_string())
            })
            .collect();
        let char_list = List::new(char_items)
            .block(char_box)
            .highlight_style(Style::new()
                .fg(Color::Cyan).add_modifier(Modifier::BOLD)
            )
            .highlight_symbol(">> ");
        frame.render_stateful_widget(char_list, chartype, &mut self.chartype_state);
    }

    fn next(&mut self) {
        match self.step {
            Step::Difficulty => {
                if self.diff_index + 1 >= self.difficulty_menu.len() {
                    self.diff_index = 0;
                } else {
                    self.diff_index += 1;
                }

                self.difficulty_state.select(Some(self.diff_index));
            }
            Step::CharType => {
                if self.chartype_index + 1 >= self.chartype_menu.len() {
                    self.chartype_index = 0;
                } else {
                    self.chartype_index += 1;
                }

                self.chartype_state.select(Some(self.chartype_index));
            }
        }
    }

    fn prev(&mut self) {
        match self.step {
            Step::Difficulty => {
                if self.diff_index == 0 {
                    self.diff_index = self.difficulty_menu.len() - 1;
                } else {
                    self.diff_index -= 1;
                }

                self.difficulty_state.select(Some(self.diff_index));
            }
            Step::CharType => {
                if self.chartype_index == 0 {
                    self.chartype_index = self.chartype_menu.len() - 1;
                } else {
                    self.chartype_index -= 1;
                }

                self.chartype_state.select(Some(self.chartype_index));
            }
        }
    }
}