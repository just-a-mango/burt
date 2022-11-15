use std::{io::{stdout, Read, Write}, env};
use crossterm::{style::{self, SetBackgroundColor, SetForegroundColor, ResetColor}, execute, terminal::{Clear, ClearType}, cursor::{MoveTo, self}};

fn refresh(file_path: &str, file_content: &str, lines: Vec<&str>) {
    // calculate terminal width and convert it to usize
    let terminal_width = crossterm::terminal::size().unwrap().0 as usize;
    // clear the terminal and print relevant info on the file
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0),SetBackgroundColor(style::Color::White), SetForegroundColor(style::Color::Black),style::Print(format!("{: ^terminal_width$}", format!("BURT   --   Editing: {}   --   {} line(s)", file_path, lines.len()))), ResetColor, style::Print("\n")).unwrap();
    // print the file
    execute!(stdout(), style::Print(file_content)).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // read file to string with write permissions
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();
    // read file to string
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    // split file content into lines
    
    refresh(file_path, &file_content, file_content.split('\n').collect());
    // enable raw mode
    crossterm::terminal::enable_raw_mode().unwrap();
    execute!(stdout(), MoveTo(0, 1)).unwrap();
    let lines: Vec<&str> = file_content.split('\n').collect();
    loop {
        // read terminal height and width
        let terminal_height = crossterm::terminal::size().unwrap().1 as usize;
        let terminal_width = crossterm::terminal::size().unwrap().0 as usize;
        // read cursor position
        let cursor_position = crossterm::cursor::position().unwrap();
        // read key
        let key = crossterm::event::read().unwrap();
        // handle key
        match key {
            crossterm::event::Event::Key(key) => {
                match key.code {
                    crossterm::event::KeyCode::Esc => {
                        // disable raw mode
                        crossterm::terminal::disable_raw_mode().unwrap();
                        // clear the terminal
                        execute!(stdout(), Clear(ClearType::All)).unwrap();
                        // exit
                        std::process::exit(0);
                    },
                    crossterm::event::KeyCode::Up => {
                        if cursor_position.1 > 1 {
                            execute!(stdout(), crossterm::cursor::MoveUp(1)).unwrap();
                            if lines[cursor_position.1 as usize - 2].len() < cursor_position.0 as usize && lines[cursor_position.1 as usize - 2].len() > 2 {
                                execute!(stdout(), MoveTo(lines[cursor_position.1 as usize - 2].len() as u16, cursor_position.1 - 1)).unwrap();
                            }
                        }
                    },
                    crossterm::event::KeyCode::Down => {
                        // move cursor down
                        if cursor_position.1 < lines.len() as u16 {
                            execute!(stdout(), crossterm::cursor::MoveDown(1)).unwrap();
                            if lines[cursor_position.1 as usize].len() < cursor_position.0 as usize && lines[cursor_position.1 as usize].len() > 2 {
                                execute!(stdout(), MoveTo(lines[cursor_position.1 as usize].len() as u16, cursor_position.1 + 1)).unwrap();
                            }
                        }
                    },
                    crossterm::event::KeyCode::Left => {
                        // move cursor left
                        execute!(stdout(), crossterm::cursor::MoveLeft(1)).unwrap();
                    },
                    crossterm::event::KeyCode::Right => {
                        if lines[cursor_position.1 as usize - 1].len() as u16 > cursor_position.0 {
                            execute!(stdout(), crossterm::cursor::MoveRight(1)).unwrap();
                        }
                        // move cursor right
                        // execute!(stdout(), crossterm::cursor::MoveRight(1)).unwrap();
                    },
                    crossterm::event::KeyCode::Char(c) => {
                        // write char to file
                        file.write_all(c.to_string().as_bytes()).unwrap();
                        // write char to terminal
                        execute!(stdout(), style::Print(c)).unwrap();
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}