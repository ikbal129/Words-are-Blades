use ratatui::{Frame, crossterm::event::{KeyCode, KeyEvent, KeyEventKind}, layout::{Alignment, Constraint, Layout, Rect}, style::{Color, Modifier, Style}, widgets::{Block, BorderType, List, ListItem, ListState, Paragraph}};
use crate::utils::arrow;
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
                match self.step {
                    Step::Difficulty => {
                        self.diff_index = arrow::next(self.difficulty_menu.clone(), self.diff_index);
                        self.difficulty_state.select(Some(self.diff_index)); 
                    }
                    Step::CharType => {
                        self.chartype_index = arrow::next(self.chartype_menu.clone(), self.chartype_index);
                        self.chartype_state.select(Some(self.chartype_index)); 

                    }
                }
                false
            }
            KeyCode::Up => {
                match self.step {
                    Step::Difficulty => {
                        self.diff_index = arrow::prev(self.difficulty_menu.clone(), self.diff_index);
                        self.difficulty_state.select(Some(self.diff_index)); 
                    }
                    Step::CharType => {
                        self.chartype_index = arrow::prev(self.chartype_menu.clone(), self.chartype_index);
                        self.chartype_state.select(Some(self.chartype_index)); 
    
                    }
                }
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
        let color: Color = match self.step {
            Step::Difficulty => Color::Cyan,
            Step::CharType => Color::White
        };
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
                .fg(color).add_modifier(Modifier::BOLD)
            )
            .highlight_symbol(">> ");
        frame.render_stateful_widget(diff_list, difficulty, &mut self.difficulty_state);

        //chartype box
        let char_color: Color = match self.step {
            Step::Difficulty => Color::White,
            Step::CharType => Color::Cyan
        };
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
                .fg(char_color).add_modifier(Modifier::BOLD)
            )
            .highlight_symbol(">> ");
        frame.render_stateful_widget(char_list, chartype, &mut self.chartype_state);
    }

    pub fn render_details(&mut self, frame: &mut Frame, area: Rect) {
        let get_details: String = match self.step {
            Step::Difficulty => {
                if self.difficulty_menu[self.diff_index] == "Easy" {
                    "=== DIFFICULTY: EASY ===

Cocok untuk pemula atau yang ingin menikmati cerita dan merapalkan kata dengan santai.

[Aturan]:
• Waktu jeda musuh menyerang lebih panjang.
• Kata-kata mantra yang harus diketik cenderung pendek dan umum.
• Toleransi kesalahan ketik lebih longgar.".to_string()
                } else if self.difficulty_menu[self.diff_index] == "Normal" {
                    "=== DIFFICULTY: NORMAL ===

Pengalaman petualangan yang seimbang. Tantangan standar bagi para penakluk kata.

[Aturan]:
• Kecepatan musuh dan durasi waktu berada dalam tingkat normal.
• Kombinasi kata pendek dan panjang muncul secara acak.
• Ketepatan mengetik mulai diuji secara berkala.".to_string()
                } else {
                    "=== DIFFICULTY: HARD ===

Mode brutal khusus untuk jari-jari tercepat dan para master mantra sejati!

[Aturan]:
• Waktu pengetikan sangat singkat dan agresif.
• Kata-kata mantra yang muncul sangat panjang, rumit, dan menggunakan kosakata kuno.
• Sedikit saja salah ketik, pertahananmu akan langsung hancur!".to_string()
                }
            }
            Step::CharType => {
                let mut text: String = String::new();
                let get_type = self.chartype_menu[self.chartype_index].as_str();

                match get_type {
                    "Fighter" => {
                        text = "=== CLASS: FIGHTER ===
Prajurit garis depan yang mengandalkan kekuatan fisik murni dan tebasan pedang berkecepatan tinggi!

[Aturan & Statistik]:
• Spesialis pertempuran jarak dekat dengan agresivitas tinggi.
• Memiliki keseimbangan yang baik antara ketahanan tubuh dan daya serang.

[Skill Pasif - \"Brutal Force\"]:
• Peningkatan physical attack damage sebesar 20% (multiplier 1.2x).
• Setiap tebasan kata yang akurat memberikan hantaman fisik yang jauh lebih mematikan ke pertahanan musuh.".to_string();
                    }
                    "Mage" => {
                        text = "=== CLASS: MAGE ===
Penyihir agung yang mengalirkan energi sihir kuno ke dalam setiap rangkaian kata mantra!

[Aturan & Statistik]:
• Spesialis pemberi burst damage magis dalam tempo cepat.
• Mengandalkan ketepatan merapal mantra-mantra panjang untuk menghancurkan musuh seketika.

[Skill Pasif - \"Arcane Mastery\"]:
• Peningkatan magical attack damage sebesar 25% (multiplier 1.25x).
• Setiap mantra sihir yang berhasil diselesaikan akan memicu ledakan energi magis tambahan.".to_string();
                    }
                    "Tank" => {
                        text = "=== CLASS: TANK ===
Ksatria baja terkuat yang berdiri kokoh menahan segala gempuran kata dan mantra musuh!

[Aturan & Statistik]:
• Pertahanan absolut dengan ketahanan mental dan fisik yang luar biasa.
• Menjadi perisai pelindung utama di tengah kerasnya arena pertempuran pengetikan.

[Skill Pasif - \"Fortified Iron\"]:
• Kapasitas HP meningkat menjadi 2x lipat dari kapasitas standar.
• Mampu menyerap terjangan serangan musuh yang paling brutal sekalipun tanpa mudah tumbang.".to_string();
                    }
                    "Support" => {
                        text = "=== CLASS: SUPPORT ===
Pemuka religius atau penyembuh misterius yang menjaga stabilitas energi dan vitalitas kelompok!

[Aturan & Statistik]:
• Berfokus pada pemulihan berkala dan pertahanan jangka panjang.
• Sangat tangguh dalam pertempuran berdurasi panjang (war of attrition).

[Skill Pasif - \"Divine Grace\"]:
• Memulihkan HP (heal) secara otomatis setiap giliran sebesar 10% dari Max HP.
• Memastikan fisikmu tetap prima meskipun musuh melancarkan serangan bertubi-tubi.".to_string();
                    }
                    "Miner" => {
                        text = "=== CLASS: MINER ===
Pemburu harta karun dan penambang kata sakti yang selalu mencari keuntungan di setiap pertempuran!

[Aturan & Statistik]:
• Spesialis pengumpul kekayaan dan sumber daya dunia Words are Blades.
• Mengubah setiap ketukan kata yang sukses menjadi pundi-pundi kekayaan melimpah.

[Skill Pasif - \"Greed Vault\"]:
• Penambahan perolehan gold setiap kali berhasil menyerang sebesar 40% (multiplier 1.4x).
• Memudahkanmu memborong upgrade dan item legendaris di toko antar-level.".to_string();
                    }
                    _ => {}
                }

                text
            }
        };

        let detail_block = Block::bordered().border_type(BorderType::Rounded).title(" Details ");
        frame.render_widget(Paragraph::new(get_details).block(detail_block), area);
    }

    pub fn render_guide(&mut self, frame: &mut Frame, area: Rect) {
        let guide_layout = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33)
        ]).split(area);

        let text = ["[Up/Down] to Select", "[Esc] to Return", "[Enter] to Confirm"];
        frame.render_widget(Paragraph::new(text[0]).alignment(Alignment::Left), guide_layout[0]);
        frame.render_widget(Paragraph::new(text[1]).alignment(Alignment::Center), guide_layout[1]);
        frame.render_widget(Paragraph::new(text[2]).alignment(Alignment::Right), guide_layout[2]);
    }
}