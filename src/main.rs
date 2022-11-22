use std::{io::{stdout, Read, Write}, env};
use crossterm::{style::{self, SetBackgroundColor, SetForegroundColor, ResetColor}, execute, terminal::{Clear, ClearType}, cursor::{MoveTo, MoveUp, MoveDown}};

fn refresh(file_path: &str, lines: Vec<&str>) {
    // calculate terminal width and convert it to usize
    let terminal_width = crossterm::terminal::size().unwrap().0 as usize;
    // clear the terminal and print relevant info on the file
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0),SetBackgroundColor(style::Color::White), SetForegroundColor(style::Color::Black),style::Print(format!("{: ^terminal_width$}", format!("BURT   --   Editing: {}   --   {} line(s)", file_path, lines.len()))), ResetColor, style::Print("\n")).unwrap();
    // print the file

    let to_print = lines.join("\n");
    execute!(stdout(), style::Print(to_print)).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("BURT\nBy Just_a_Mango\n\nUsage: burt <file>");
        return;
    }
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
    let shown_lines:Vec<&str> = file_content.split('\n').collect();
    // let terminal_height = crossterm::terminal::size().unwrap().1 as usize;
    // strip shown_lines to terminal height
    // REMOVE COMMENT IF NOT WORKING {
    // if (terminal_height - 1) < shown_lines.len() {
    //     let shown_lines = &shown_lines[0..(terminal_height - 1)];
    // }
    // let temp_lines: Vec<String> = shown_lines.iter().map(|x| x.to_string()).collect();
    // } REMOVE COMMENT IF NOT WORKING
    // enable raw mode
    crossterm::terminal::enable_raw_mode().unwrap();
    // move cursor to the end of the file
    // execute!(stdout(), MoveTo(file_content.split('\n').last().unwrap().len() as u16, file_content.split('\n').count() as u16)).unwrap();
    execute!(stdout(), MoveTo(0, 1)).unwrap();
    let mut lines: Vec<String> = file_content.split('\n').map(|x| x.to_string()).collect();
    let mut current_line = 0;
    let mut line_index = 1;
    let terminal_height = crossterm::terminal::size().unwrap().1 as usize;
    refresh(file_path, lines[current_line..(current_line + terminal_height - 1)].iter().map(|x| x.as_str()).collect());
    execute!(stdout(), MoveTo(0, 1)).unwrap();
    loop {
        // read terminal height and width
        // let terminal_height = crossterm::terminal::size().unwrap().1 as usize;
        // let terminal_width = crossterm::terminal::size().unwrap().0 as usize;
        // read cursor position
        let cursor_position = crossterm::cursor::position().unwrap();
        // read key
        let key = crossterm::event::read().unwrap();
        // handle key
        match key {
            crossterm::event::Event::Key(key) => {
                match key.code {
                    crossterm::event::KeyCode::Up => {
                        // if the cursor is on the first line and current_line is greater than terminal_height, scroll up, otherwise move the cursor up
                        if cursor_position.1 > 1 {
                            execute!(stdout(), MoveUp(1)).unwrap();
                            line_index -= 1;
                        } else if current_line > 0 {
                            current_line -= 1;
                            line_index -= 1;
                            let terminal_height = crossterm::terminal::size().unwrap().1 as usize;
                            let shown_lines = &lines[current_line..(current_line + terminal_height - 1)];
                            refresh(file_path, shown_lines.iter().map(|x| x.as_str()).collect());

                            execute!(stdout(), MoveTo(cursor_position.0, cursor_position.1)).unwrap();
                        }
                    },
                    crossterm::event::KeyCode::Down => {
                        // if the cursor is on the last line and current_line is less than the number of lines in the file, scroll down, otherwise move the cursor down
                        if cursor_position.1 < crossterm::terminal::size().unwrap().1 - 1 {
                            execute!(stdout(), MoveDown(1)).unwrap();
                            line_index += 1;
                        } else if (terminal_height - 1) > lines.len() {
                            current_line += 1;
                            line_index += 1;
                            if current_line + 1 == shown_lines.len() {
                                execute!(stdout(), MoveDown(1)).unwrap();
                            }
                        }
                        else if current_line < (lines.len() - 1 ) && current_line < lines.len() - (crossterm::terminal::size().unwrap().1 as usize - 2) && (current_line + terminal_height - 1) < lines.len() {
                            current_line += 1;
                            line_index += 1;
                            let terminal_height = crossterm::terminal::size().unwrap().1 as usize;
                            let shown_lines = &lines[current_line..(current_line + terminal_height - 1)];
                            refresh(file_path, shown_lines.iter().map(|x| x.as_str()).collect());
                            execute!(stdout(), MoveTo(cursor_position.0, cursor_position.1)).unwrap();
                        }
                    },
                    crossterm::event::KeyCode::Left => {
                        // move cursor left
                            execute!(stdout(), crossterm::cursor::MoveLeft(1)).unwrap();
                    },
                    crossterm::event::KeyCode::Right => {
                        // move cursor right if it is not at the end of the line (use current_line to get the line)
                        if cursor_position.0 < (lines[line_index - 1].len() as u16 - 1) {
                            execute!(stdout(), crossterm::cursor::MoveRight(1)).unwrap();
                        }
                    },
                    crossterm::event::KeyCode::Char(c) => {
                        if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                            match c {
                                's' => {
                                    // save the file without erasing the first line
                                    let mut file = std::fs::OpenOptions::new()
                                        .write(true)
                                        .truncate(true)
                                        .open(file_path)
                                        .unwrap();
                                    file.write_all(lines.join("\n").as_bytes()).unwrap();
                                },
                                'q' => {
                                    // disable raw mode
                                    crossterm::terminal::disable_raw_mode().unwrap();
                                    // clear the terminal
                                    execute!(stdout(), Clear(ClearType::All)).unwrap();
                                    // exit
                                    std::process::exit(0);
                                },
                                'r' => {
                                    refresh(file_path, lines.iter().map(|x| x.as_str()).collect());
                                }
                                _ => {}
                            }
                        } else {
                            // insert the character at the cursor position
                            let mut line = lines[line_index - 1].clone();
                            line.insert(cursor_position.0 as usize, c);
                            let new_line = line.clone();
                            lines[line_index - 1] = line;
                            // REMOVE COMMENT IF NOT WORKING
                            // let shown_lines = &lines[current_line..(current_line + terminal_height - 1)];
                            // move the cursor right
                            execute!(stdout(), crossterm::cursor::MoveRight(1)).unwrap();
                            // REMOVE COMMENT IF NOT WORKING
                            // let terminal_height = crossterm::terminal::size().unwrap().1 as usize;
                            // erase the line and print the new line
                            execute!(stdout(), Clear(ClearType::CurrentLine), MoveTo(0, cursor_position.1), style::Print(new_line), MoveTo(cursor_position.0 + 1, cursor_position.1)).unwrap();
                        }
                    },
                    crossterm::event::KeyCode::Backspace => {
                        // remove the character at the cursor position
                        let mut line = lines[line_index - 1].clone();
                        line.remove(cursor_position.0 as usize - 1);
                        let new_line = line.clone();
                        lines[line_index - 1] = line;
                        // REMOVE COMMENT IF NOT WORKING
                        // let shown_lines = &lines[current_line..(current_line + terminal_height - 1)];
                        // move the cursor left
                        execute!(stdout(), crossterm::cursor::MoveLeft(1)).unwrap();
                        // REMOVE COMMENT IF NOT WORKING
                        // let terminal_height = crossterm::terminal::size().unwrap().1 as usize;
                        // erase the line and print the new line
                        execute!(stdout(), Clear(ClearType::CurrentLine), MoveTo(0, cursor_position.1), style::Print(new_line), MoveTo(cursor_position.0 - 1, cursor_position.1)).unwrap();
                    
                    },
                    crossterm::event::KeyCode::Enter => {
                        // insert a new line at the cursor position
                        let mut line = lines[line_index - 1].clone();
                        let new_line = line.split_off(cursor_position.0 as usize);
                        // save cursor position
                        let cursor_position = crossterm::cursor::position().unwrap();
                        lines.insert(line_index, new_line);
                        // refresh the screen
                        refresh(file_path, lines.iter().map(|x| x.as_str()).collect());
                        // move the cursor to the new line
                        execute!(stdout(), MoveTo(0, cursor_position.1 + 1)).unwrap();
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}