use crossterm::{cursor, execute, style::Print};
use ratatui::layout::Size;

pub fn build(size: &Size, symbols: &String, index: &mut usize) {
    let loading_symbol = symbols.chars().nth(*index).unwrap();
    let text = format!("  {} Loading...  ", loading_symbol);
    let padding = " ".repeat(text.chars().count());

    let x = (size.width / 2) as u16 - (text.len() as u16 / 2);
    let y = size.height / 2;

    execute!(
        std::io::stdout(),
        cursor::MoveTo(x, y - 1),
        Print(padding.clone()),
    )
    .unwrap();
    execute!(std::io::stdout(), cursor::MoveTo(x, y), Print(text)).unwrap();
    execute!(std::io::stdout(), cursor::MoveTo(x, y + 1), Print(padding)).unwrap();

    if *index < symbols.chars().count() - 1 {
        *index += 1;
    } else {
        *index = 0;
    };
}
