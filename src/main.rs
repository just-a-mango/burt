use std::{io::{stdout, Read, Write}, env};
use crossterm::{style::{self, SetBackgroundColor, SetForegroundColor, ResetColor}, execute, terminal::{Clear, ClearType}, cursor::{MoveTo, self, MoveUp, MoveDown}};


fn refresh(file_path: &str, lines: Vec<&str>) {
    // calculate terminal width and convert it to usize
    let terminal_width = crossterm::terminal::size().unwrap().0 as usize;
    // clear the terminal and print relevant info on the file
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0),SetBackgroundColor(style::Color::White), SetForegroundColor(style::Color::Black),style::Print(format!("{: ^terminal_width$}", format!("BURT   --   Editing: {}   --   {} line(s)", file_path, lines.len()))), ResetColor, style::Print("\n")).unwrap();
    // print the file

    let to_print = lines.join("\n");
    execute!(stdout(), style::Print(to_print)).unwrap();
}

fn insert_char(mut lines: Vec<String>, char_to_insert: char, current_line: usize) -> Vec<String> {
    // // insert the character at the current line and position
    // lines[current_line].insert(cursor::position().unwrap().0 as usize, char_to_insert);
    // // re-render the file with the correct lines type
    // refresh(&env::args().nth(1).unwrap(), lines.iter().map(|x| x.as_str()).collect());
    // // return the modified lines
    return lines;
}

fn remove_char(mut lines: Vec<String>) -> Vec<String> {
    // get the cursor position
    let (x, mut y) = cursor::position().unwrap();
    y -= 1;
    // get the line the cursor is on
    // let line = lines.get_mut(y as usize).unwrap();
    // remove the character
    let mut cursor_pos = (0, 0);
    if lines[y as usize].len() > 0 && x as usize > 0 {
        lines[y as usize].remove(x as usize - 1);
        cursor_pos = (x - 1, y);
        // erase the current line and print the new one
        execute!(stdout(), MoveTo(0, y + 1), Clear(ClearType::CurrentLine), style::Print(&lines[y as usize])).unwrap();
        // move the cursor to the saved position
        execute!(stdout(), MoveTo(cursor_pos.0, cursor_pos.1 + 1)).unwrap();
        return lines;
    } else if y as usize > 0 {
        if y > 0 {
            // move cursor to where the line above ends
            let line = lines.remove(y as usize);
            let prev_line = lines.get_mut(y as usize - 1).unwrap();
            cursor_pos = (prev_line.len() as u16, y - 1);
            prev_line.push_str(&line);
        }
    } else {
        // if the line is empty, remove it, and move the cursor to the end of the previous line
        if y > 0 {
            lines.remove(y as usize);
            cursor_pos = (lines.get(y as usize - 1).unwrap().len() as u16, y - 1);
        }
        
    }
    // save cursor position
    
    // refresh the terminal
    refresh(&env::args().nth(1).unwrap(), lines.iter().map(|x| x.as_str()).collect());
    // move the cursor to the saved position
    execute!(stdout(), MoveTo(cursor_pos.0, cursor_pos.1 + 1)).unwrap();

    return lines;
}

fn insert_new_line(temp_two_lines: Vec<String>) -> Vec<String> {
    let mut lines = temp_two_lines;
    let (x, mut y) = cursor::position().unwrap();
    y -= 1;
    // get the line the cursor is on
    let line = lines.get_mut(y as usize).unwrap();
    // split the line into two lines
    let new_line = line.split_off(x as usize);
    // add the new line to the lines vector
    lines.insert(y as usize + 1, new_line);
    // refresh the terminal
    refresh(&env::args().nth(1).unwrap(), lines.iter().map(|x| x.as_str()).collect());
    // move the cursor to the saved position
    execute!(stdout(), MoveTo(0, y + 2)).unwrap();

    return lines;
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
    // strip shown_lines to terminal height
    let shown_lines = &shown_lines[0..(crossterm::terminal::size().unwrap().1 as usize - 1)];
    // convert lines to strings
    let temp_lines: Vec<String> = shown_lines.iter().map(|x| x.to_string()).collect();
    refresh(file_path, temp_lines.iter().map(|x| x.as_str()).collect());
    // enable raw mode
    crossterm::terminal::enable_raw_mode().unwrap();
    // move cursor to the end of the file
    // execute!(stdout(), MoveTo(file_content.split('\n').last().unwrap().len() as u16, file_content.split('\n').count() as u16)).unwrap();
    execute!(stdout(), MoveTo(0, 1)).unwrap();
    let mut lines: Vec<String> = file_content.split('\n').map(|x| x.to_string()).collect();
    let mut current_line = 0;
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
                        // if the cursor is on the first line and current_line is greater than terminal_height, scroll up
                        if cursor_position.1 == 1 && current_line > 0 {
                            // save cursor position
                            let cursor_pos = (cursor_position.0, cursor_position.1);
                            current_line -= 1;
                            let temp_lines = &lines[current_line..(current_line + crossterm::terminal::size().unwrap().1 as usize - 1)];
                            let temp_lines: Vec<&str> = temp_lines.iter().map(|x| x.as_str()).collect();
                            refresh(file_path, temp_lines);
                            // move the cursor to the saved position
                            execute!(stdout(), MoveTo(cursor_pos.0, cursor_pos.1)).unwrap();
                        } else if cursor_position.1 > 1 {
                            current_line -= 1;
                            execute!(stdout(), MoveUp(1)).unwrap();
                        }
                    },
                    crossterm::event::KeyCode::Down => {
                        // if the cursor is on the last line and current_line is less than lines.len() - terminal_height, scroll down, else, move the cursor down
                        if cursor_position.1 == crossterm::terminal::size().unwrap().1 - 1 && current_line < lines.len() - crossterm::terminal::size().unwrap().1 as usize - 1 {
                            // save cursor position
                            let cursor_pos = (cursor_position.0, cursor_position.1);
                            current_line += 1;
                            let temp_lines = &lines[current_line..(current_line + crossterm::terminal::size().unwrap().1 as usize - 1)];
                            let temp_lines: Vec<&str> = temp_lines.iter().map(|x| x.as_str()).collect();
                            refresh(file_path, temp_lines);
                            // move the cursor to the saved position
                            execute!(stdout(), MoveTo(cursor_pos.0, cursor_pos.1)).unwrap();
                        } else if cursor_position.1 < crossterm::terminal::size().unwrap().1 {
                            current_line += 1;
                            execute!(stdout(), MoveDown(1)).unwrap();
                        }
                    },
                    crossterm::event::KeyCode::Left => {
                        // move cursor left
                        execute!(stdout(), crossterm::cursor::MoveLeft(1)).unwrap();
                    },
                    crossterm::event::KeyCode::Right => {
                        // move cursor right if it is not at the end of the line (use current_line to get the line)
                        if cursor_position.0 < lines[current_line].len() as u16 {
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
                            // insert character
                            let temp_two_lines: Vec<String> = lines.clone().iter().map(|x| x.to_string()).collect();
                            // set lines to the new lines and convert them to a vector of strings without a borrow checker error
                            lines = insert_char(temp_two_lines, c, current_line);
                        }
                    },
                    crossterm::event::KeyCode::Backspace => {
                        // remove character
                        let temp_lines = lines.clone();
                        let temp_two_lines: Vec<String> = temp_lines.iter().map(|x| x.to_string()).collect();
                        // set lines to the new lines and convert them to a vector of strings without a borrow checker error
                        lines = remove_char(temp_two_lines);
                    },
                    crossterm::event::KeyCode::Enter => {
                        // insert new line
                        let temp_lines = lines.clone();
                        let temp_two_lines: Vec<String> = temp_lines.iter().map(|x| x.to_string()).collect();
                        // set lines to the new lines and convert them to a vector of strings without a borrow checker error
                        lines = insert_new_line(temp_two_lines);
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}