use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::{Frame, layout::Rect};
use ratatui::text::{Line};
use ratatui::widgets::{Block, ListItem, Paragraph, BorderType, List};

pub use crate::app::App;

impl App {
    pub fn render_title(&mut self, frame: &mut Frame, area: Rect) {
        let [top, bottom] = Layout::vertical([
            Constraint::Percentage(40),
            Constraint::Percentage(60)
        ]).areas(area);
        let words_ascii: Vec<Line> = vec![
            Line::from("  __      __                .___      "),
            Line::from("/  \\    /  \\___________  __| _/______"),
            Line::from("\\   \\/\\    /  _ \\_  __ \\/ __ |/  ___/"),
            Line::from(" \\        (  <_> )  | \\/ /_/ |\\___ \\ "),
            Line::from("  \\__/\\  / \\____/|__|  \\____ /____  >"),
            Line::from("       \\/                   \\/    \\/ "),
        ];

        let are_blades: Vec<Line> = vec![
            Line::from("                       __________.__              .___             "),
            Line::from("_____ _______   ____   \\______   \\  | _____     __| _/____   ______"),
            Line::from("\\__  \\\\_  __ \\_/ __ \\   |    |  _/  | \\__  \\   / __ |/ __ \\ /  ___/"),
            Line::from(" / __ \\|  | \\/\\  ___/   |    |   \\  |__/ __ \\_/ /_/ \\  ___/ \\___ \\ "),
            Line::from("(____  /__|    \\___  >  |______  /____(____  /\\____ |\\___  >____  >"),
            Line::from("     \\/            \\/          \\/          \\/      \\/    \\/     \\/ "),
        ];

        frame.render_widget(Paragraph::new(words_ascii).alignment(Alignment::Center), top);
        frame.render_widget(Paragraph::new(are_blades).alignment(Alignment::Center), bottom);
    }

    pub fn render_menu(&mut self , frame: &mut Frame, area: Rect, items: Vec<String>) {
        let [menu, detail] = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]).areas(area);

        let menu_block = Block::bordered().border_type(BorderType::Rounded)
            .title(" Menu ");
        let detail_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(" Detail ");

        //list menu
        let item_list: Vec<ListItem<'_>> = items.iter().map(|item| {
            ListItem::new(item.to_string())
        }).collect();
        let list = List::new(item_list)
            .block(menu_block)
            .highlight_style(Style::new()
            .fg(Color::Cyan))
            .highlight_symbol(">> ");

        //detail text
        let detailed_text = match self.state_index {
            0 => "=== START NEW BATTLE ===

Masuki arena pertarungan pengetikan (typing RPG).
Ketik mantra dan kata-kata dengan cepat serta akurat
untuk merobohkan pertahanan musuh sebelum waktu habis

[Saran]: Siapkan jarimu di atas keyboard.
Kecepatan dan ketepatan adalah kunci hidup 
dan mati di sini!".to_string(),
            1 => "=== CODEX / GRIMOIRE ===

Arsip rahasia dan perpustakaan mantra kuno.
Tempat di mana seluruh daftar kata sakti,
istilah misterius, dan catatan lore dunia 
Words are Blades yang pernah kamu temukan 
tersimpan rapi.

[Status]: Terkunci. Selesaikan beberapa 
pertempuran untuk membuka halaman pertama 
grimoire ini.".to_string(),
            2 => "=== CREDITS ===

Words are Blades v0.1-alpha
Sebuah proyek eksperimental pengembaraan kode 
dalam bahasa pemrograman Rust menggunakan framework 
TUI (Ratatui & Crossterm)

[Lead Developer]: Kamu Sendiri
[Inspirasi]: Terminal retro, game bergenre typing, 
dan cinta pada estetika baris perintah.".to_string(),
            3 => "=== EXIT / TERMINATE ===

Keluar dari sesi permainan dan kembali 
ke terminal sistem utama

[Peringatan]: Pastikan progres atau data pentingmu 
sudah tersimpan sebelum meninggalkan sesi pertempuran ini. 
Sampai jumpa kembali, Penakluk Kata!".to_string(),
            _ => {"".to_string()}
        };

        frame.render_stateful_widget(list, menu, &mut self.state);
        frame.render_widget(Paragraph::new(detailed_text).block(detail_block), detail);
    }

    pub fn render_guide(&mut self, frame: &mut Frame, area: Rect) {
        let [left, right] = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]).areas(area);

        let left_guide = "[Up/Down] to Select".to_string();
        let right_guide = "[Enter] to Confirm".to_string();
        
        frame.render_widget(Paragraph::new(left_guide), left);
        frame.render_widget(Paragraph::new(right_guide).alignment(Alignment::Right), right);
    }
}