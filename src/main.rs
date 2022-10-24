use std::{vec, io::{Read, stdout, Write}, process, env};
use clearscreen::ClearScreen;
use crossterm::{execute, terminal::{enable_raw_mode}, event::{Event, KeyEvent, KeyCode, read}, cursor::{self}};


fn clear_screen() {
    ClearScreen::default().clear().expect("failed to clear the screen");
}


/// It clears the screen, prints the file name and number of lines, and then prints each line of the
/// buffer
/// 
/// Arguments:
/// 
/// * `lines`: &Vec<String> - This is a reference to the vector of strings that contains the lines of
/// the file.
/// * `file_path`: The path to the file that is being edited.
fn refresh_screen(lines: &Vec<String>, file_path: &str) {
    clear_screen();
    println!("           Untitled Text Editor  -  {} - {} line(s)            ", file_path, lines.len());
    for buff_line in lines {
        println!("{}", buff_line);
    }
}


/// It opens a file, reads its contents into a string, splits the string into lines, and returns a
/// vector of strings
/// 
/// Arguments:
/// 
/// * `path`: &str - the path to the file to read
/// 
/// Returns:
/// 
/// A vector of strings.
fn read_file(path: &str) -> vec::Vec<String> {
    let mut file = std::fs::File::open(path).expect("failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read file");
    contents.lines().map(|s| s.to_string()).collect()
}


/// It takes a cursor position, a mutable reference to a vector of strings, and a character, and inserts
/// the character at the cursor position on the current line
/// 
/// Arguments:
/// 
/// * `cursor_position`: (u16, u16)
/// * `lines`: A vector of strings, each string representing a line of text.
/// * `key`: the key that was pressed
fn insert_char(cursor_position: (u16, u16), lines: &mut Vec<String>, key: char) {
    // insert char at cursor position on current line
    let (x, y) = cursor_position;
    let line = &mut lines[y as usize-1];
    line.insert(x as usize, key);
    
}


fn main() {
    let mut _lines: vec::Vec<String> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    clear_screen();
    println!("{} - {} line(s)", file_path, _lines.len());
    for buff_line in read_file(file_path) {
        println!("{}", buff_line);
        _lines.push(buff_line);
    }
    refresh_screen(&_lines, file_path);

    let mut stdout = stdout();
    //going into raw mode
    enable_raw_mode().unwrap();

    //clearing the screen, going to top left corner and printing welcoming message
    execute!(stdout, cursor::MoveTo(0, 1)).unwrap();
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                kind: _,
                state: _,
                modifiers: _,
            }) => { 
                if (_lines.len() as u16) > cursor::position().unwrap().1 {
                    execute!(stdout, cursor::MoveDown(1)).unwrap();
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                kind: _,
                state: _,
                modifiers: _,
            }) => {
                if cursor::position().unwrap().1 > 1 {
                    execute!(stdout, cursor::MoveToPreviousLine(1)).unwrap()
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                kind: _,
                state: _,
                modifiers: _,
            }) => execute!(stdout, cursor::MoveRight(1)).unwrap(),
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                kind: _,
                state: _,
                modifiers: _,
            }) => execute!(stdout, cursor::MoveLeft(1)).unwrap(),
            // enter any key to insert a character
            Event::Key(KeyEvent {
                code: KeyCode::Char(key),
                kind: _,
                state: _,
                modifiers: _,
            }) => {
                // get cursor position
                let cursor_position = cursor::position().unwrap();
                // insert char at cursor position on current line
                insert_char(cursor_position, &mut _lines, key);
                refresh_screen(&_lines, file_path);
                // keep cursor position
                execute!(stdout, cursor::MoveTo(cursor_position.0, cursor_position.1)).unwrap();
                execute!(stdout, cursor::MoveRight(1)).unwrap();
            },
            // press enter key to jump to next line
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: _,
                state: _,
                modifiers: _,
            }) => {
                // get cursor position
                let cursor_position = cursor::position().unwrap();
                // if cursor is at beginning of non-empty line, move current line to next line
                if cursor_position.0 == 0 && _lines[cursor_position.1 as usize - 1].len() > 0 {
                    _lines.insert(cursor_position.1 as usize, String::new());
                }
                // insert char at cursor position on current line
                insert_char(cursor_position, &mut _lines, ' ');
                refresh_screen(&_lines, file_path);
                // keep cursor position
                execute!(stdout, cursor::MoveTo(cursor_position.0, cursor_position.1)).unwrap();
                execute!(stdout, cursor::MoveToNextLine(1)).unwrap();
            }
            // press backspace key to delete character and jump to the previous line if needed
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                kind: _,
                state: _,
                modifiers: _,
            }) => {
                let cursor_position = cursor::position().unwrap();
                // save cursor position
                execute!(stdout, cursor::SavePosition).unwrap();
                let buff_line = _lines.get_mut(cursor_position.1 as usize - 1);
                if buff_line != None {
                    let buff_line = buff_line.unwrap();
                    let mut buff_line_chars = buff_line.chars().collect::<Vec<char>>();
                    if cursor_position.0 as usize - 1 < buff_line_chars.len() {
                        buff_line_chars.remove(cursor_position.0 as usize - 1);
                        buff_line.clear();
                        for c in buff_line_chars {
                            buff_line.push(c);
                        }
                    } else {
                        if cursor_position.1 as usize - 1 > 0 {
                            let prev_buff_line = _lines.get_mut(cursor_position.1 as usize - 2);
                            if prev_buff_line != None {
                                let prev_buff_line = prev_buff_line.unwrap();
                                let mut prev_buff_line_chars = prev_buff_line.chars().collect::<Vec<char>>();
                                prev_buff_line_chars.push(' ');
                                prev_buff_line.clear();
                                for c in prev_buff_line_chars {
                                    prev_buff_line.push(c);
                                }
                                _lines.remove(cursor_position.1 as usize - 1);
                            }
                        }
                    }
                }
                refresh_screen(&_lines, file_path);
                // restore cursor position
                execute!(stdout, cursor::RestorePosition).unwrap();
                // move cursor to the left if line isn't empty, else move cursor to the last character of the previous line
                if cursor_position.0 as usize - 1 > 0 {
                    execute!(stdout, cursor::MoveLeft(1)).unwrap();
                } else {
                    if cursor_position.1 as usize - 1 > 0 {
                        let prev_buff_line = _lines.get(cursor_position.1 as usize - 2);
                        if prev_buff_line != None {
                            let prev_buff_line = prev_buff_line.unwrap();
                            let prev_buff_line_chars = prev_buff_line.chars().collect::<Vec<char>>();
                            execute!(stdout, cursor::MoveTo(prev_buff_line_chars.len() as u16 + 1, cursor_position.1 - 1)).unwrap();
                        }
                    }
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                kind: _,
                state: _,
                modifiers: _,
            }) => {
                // save file
                let mut file = std::fs::File::create(file_path).expect("failed to create file");
                for line in _lines {
                    file.write_all(line.as_bytes()).expect("failed to write to file");
                    file.write_all("\n".as_bytes()).expect("failed to write to file");
                }
                clear_screen();
                process::exit(0);
            }
            _ => (),
        
        }
    }
}