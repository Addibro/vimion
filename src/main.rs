mod types;
mod levelgen;
mod game;
mod render;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::types::GameScreen;
use crate::game::GameState;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;
    terminal.clear()?;

    let result = run(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
    Ok(())
}

fn run(terminal: &mut render::Term) -> io::Result<()> {
    let mut state = GameState::new();
    let mut last_key_was_g = false;

    loop {
        render::draw(terminal, &state)?;

        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                break;
            }

            match &state.screen.clone() {
                GameScreen::Title => {
                    if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                        state.screen = GameScreen::Playing;
                    }
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }

                GameScreen::GameOver => {
                    if key.code == KeyCode::Enter {
                        state = GameState::new();
                        last_key_was_g = false;
                    }
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }

                GameScreen::Victory => {
                    if key.code == KeyCode::Enter {
                        state = GameState::new();
                        last_key_was_g = false;
                    }
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }

                GameScreen::LevelUp => {
                    if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                        state.screen = GameScreen::Playing;
                    }
                }

                GameScreen::LessonPopup(_) => {
                    if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Esc) {
                        state.screen = GameScreen::Playing;
                    }
                }

                GameScreen::VimChallenge(idx, ref input) => {
                    let idx = *idx;
                    let mut input = input.clone();
                    let lessons = crate::types::all_lessons();
                    let answer = lessons[idx.min(lessons.len() - 1)].key;

                    match key.code {
                        KeyCode::Esc => {
                            state.push_log("❌ Challenge abandoned. Scroll wasted!".to_string());
                            state.screen = GameScreen::Playing;
                        }
                        KeyCode::Backspace => {
                            input.pop();
                            state.screen = GameScreen::VimChallenge(idx, input);
                        }
                        KeyCode::Enter => {
                            if input == answer {
                                let lesson = &lessons[idx.min(lessons.len() - 1)];
                                if !state.player.vim_lessons_learned.contains(&lesson.key.to_string()) {
                                    state.player.vim_lessons_learned.push(lesson.key.to_string());
                                }
                                state.push_log(format!("✅ Correct! Learned '{}' — {}", lesson.key, lesson.description));
                                state.screen = GameScreen::LessonPopup(idx);
                            } else {
                                state.push_log("❌ Wrong answer! Scroll crumbles to dust.".to_string());
                                state.screen = GameScreen::Playing;
                            }
                        }
                        KeyCode::Char(c) => {
                            input.push(c);
                            // Auto-submit single char answers (except 'g' which could be 'gg')
                            if input == answer && answer != "gg" {
                                let lesson = &lessons[idx.min(lessons.len() - 1)];
                                if !state.player.vim_lessons_learned.contains(&lesson.key.to_string()) {
                                    state.player.vim_lessons_learned.push(lesson.key.to_string());
                                }
                                state.push_log(format!("✅ Correct! Learned '{}' — {}", lesson.key, lesson.description));
                                state.screen = GameScreen::LessonPopup(idx);
                            } else if input.len() >= answer.len() && input != answer && answer != "gg" {
                                state.push_log("❌ Wrong answer! Scroll crumbles to dust.".to_string());
                                state.screen = GameScreen::Playing;
                            } else {
                                state.screen = GameScreen::VimChallenge(idx, input);
                            }
                        }
                        _ => {}
                    }
                }

                GameScreen::Help => {
                    if matches!(key.code, KeyCode::Char('?') | KeyCode::Esc) {
                        state.screen = GameScreen::Playing;
                    }
                }

                GameScreen::Playing => {
                    let learned = state.player.vim_lessons_learned.clone();

                    match key.code {
                        KeyCode::Char('h') | KeyCode::Left => {
                            last_key_was_g = false;
                            state.move_player(0, -1);
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            last_key_was_g = false;
                            state.move_player(1, 0);
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            last_key_was_g = false;
                            state.move_player(-1, 0);
                        }
                        KeyCode::Char('l') | KeyCode::Right => {
                            last_key_was_g = false;
                            state.move_player(0, 1);
                        }

                        KeyCode::Char('w') if learned.contains(&"w".to_string()) => {
                            last_key_was_g = false;
                            if !state.jump_word_forward() {
                                state.push_log("'w' — no open path ahead!".to_string());
                            }
                        }
                        KeyCode::Char('b') if learned.contains(&"b".to_string()) => {
                            last_key_was_g = false;
                            if !state.jump_word_back() {
                                state.push_log("'b' — nowhere to go back!".to_string());
                            }
                        }
                        KeyCode::Char('e') if learned.contains(&"e".to_string()) => {
                            last_key_was_g = false;
                            if !state.jump_word_end() {
                                state.push_log("'e' — no word end ahead!".to_string());
                            }
                        }
                        KeyCode::Char('0') if learned.contains(&"0".to_string()) => {
                            last_key_was_g = false;
                            if !state.jump_line_start() {
                                state.push_log("'0' — already at line start!".to_string());
                            }
                        }
                        KeyCode::Char('$') if learned.contains(&"$".to_string()) => {
                            last_key_was_g = false;
                            if !state.jump_line_end() {
                                state.push_log("'$' — already at line end!".to_string());
                            }
                        }
                        KeyCode::Char('G') if learned.contains(&"G".to_string()) => {
                            last_key_was_g = false;
                            if !state.jump_bottom() {
                                state.push_log("'G' — already at the bottom!".to_string());
                            }
                        }
                        KeyCode::Char('H') if learned.contains(&"H".to_string()) => {
                            last_key_was_g = false;
                            if !state.jump_screen_high() {
                                state.push_log("'H' — can't reach high area!".to_string());
                            }
                        }
                        KeyCode::Char('M') if learned.contains(&"M".to_string()) => {
                            last_key_was_g = false;
                            if !state.jump_screen_mid() {
                                state.push_log("'M' — can't reach middle!".to_string());
                            }
                        }
                        KeyCode::Char('L') if learned.contains(&"L".to_string()) => {
                            last_key_was_g = false;
                            if !state.jump_screen_low() {
                                state.push_log("'L' — can't reach low area!".to_string());
                            }
                        }

                        KeyCode::Char('g') => {
                            if last_key_was_g {
                                if learned.contains(&"gg".to_string()) {
                                    if !state.jump_top() {
                                        state.push_log("'gg' — can't reach the top!".to_string());
                                    }
                                } else {
                                    state.push_log("'gg' locked — find a scroll!".to_string());
                                }
                                last_key_was_g = false;
                            } else {
                                last_key_was_g = true;
                            }
                        }

                        KeyCode::Char('p') => {
                            last_key_was_g = false;
                            state.use_potion();
                        }
                        KeyCode::Char('?') => {
                            last_key_was_g = false;
                            state.screen = GameScreen::Help;
                        }
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Esc => {
                            last_key_was_g = false;
                        }

                        KeyCode::Char('w') => { last_key_was_g = false; state.push_log("'w' locked — find a scroll '?' to unlock!".to_string()); }
                        KeyCode::Char('b') => { last_key_was_g = false; state.push_log("'b' locked — find a scroll '?' to unlock!".to_string()); }
                        KeyCode::Char('e') => { last_key_was_g = false; state.push_log("'e' locked — find a scroll '?' to unlock!".to_string()); }
                        KeyCode::Char('$') => { last_key_was_g = false; state.push_log("'$' locked — find a scroll '?' to unlock!".to_string()); }
                        KeyCode::Char('G') => { last_key_was_g = false; state.push_log("'G' locked — find a scroll '?' to unlock!".to_string()); }
                        KeyCode::Char('H') => { last_key_was_g = false; state.push_log("'H' locked — find a scroll '?' to unlock!".to_string()); }
                        KeyCode::Char('M') => { last_key_was_g = false; state.push_log("'M' locked — find a scroll '?' to unlock!".to_string()); }
                        KeyCode::Char('L') => { last_key_was_g = false; state.push_log("'L' locked — find a scroll '?' to unlock!".to_string()); }

                        _ => { last_key_was_g = false; }
                    }
                }
            }
        }
    }

    Ok(())
}
