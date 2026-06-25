use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io::Stdout;

use crate::game::GameState;
use crate::types::*;

pub type Term = Terminal<CrosstermBackend<Stdout>>;

pub fn draw(terminal: &mut Term, state: &GameState) -> std::io::Result<()> {
    terminal.draw(|f| match &state.screen {
        GameScreen::Title => draw_title(f, state),
        GameScreen::Playing => draw_playing(f, state),
        GameScreen::LevelUp => {
            draw_playing(f, state);
            draw_level_up_overlay(f, state);
        }
        GameScreen::LessonPopup(idx) => {
            draw_playing(f, state);
            draw_lesson_overlay(f, state, *idx);
        }
        GameScreen::VimChallenge(idx, ref input) => {
            draw_playing(f, state);
            draw_challenge_overlay(f, *idx, input);
        }
        GameScreen::GameOver => draw_game_over(f, state),
        GameScreen::Victory => draw_victory(f, state),
        GameScreen::Help => draw_help(f, state),
    })?;
    Ok(())
}

fn draw_title(f: &mut Frame, _state: &GameState) {
    let size = f.size();
    let block = Block::default().style(Style::default().bg(Color::Black));
    f.render_widget(block, size);

    let lines = vec![
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            " ██╗   ██╗██╗███╗   ███╗     ██████╗ ██╗   ██╗███████╗███████╗████████╗",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            " ██║   ██║██║████╗ ████║    ██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            " ██║   ██║██║██╔████╔██║    ██║   ██║██║   ██║█████╗  ███████╗   ██║   ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            " ╚██╗ ██╔╝██║██║╚██╔╝██║    ██║▄▄ ██║██║   ██║██╔══╝  ╚════██║   ██║   ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "  ╚████╔╝ ██║██║ ╚═╝ ██║    ╚██████╔╝╚██████╔╝███████╗███████║   ██║   ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "   ╚═══╝  ╚═╝╚═╝     ╚═╝     ╚══▀▀═╝  ╚═════╝ ╚══════╝╚══════╝   ╚═╝   ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "        ⚔  A Dungeon RPG for Learning Vim Motions  ⚔",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  ═══════════════════════════════════════════════════════════════",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  Your mission: Descend through 5 dungeon levels and slay the Dragon!",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![
            Span::styled("  Start with:  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "h j k l",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  — move left / down / up / right",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  Find scrolls to unlock: ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled("w b e 0 $ gg G H M L f", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  ═══════════════════════════════════════════════════════════════",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "                    Press ENTER to begin your quest",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
        )]),
        Line::from(vec![Span::styled(
            "                    Press '?' anytime during play for help",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let paragraph = Paragraph::new(Text::from(lines))
        .alignment(Alignment::Left)
        .style(Style::default().bg(Color::Black));
    f.render_widget(paragraph, size);
}

fn draw_playing(f: &mut Frame, state: &GameState) {
    let size = f.size();

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(52), Constraint::Length(28)])
        .split(size);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(20), Constraint::Length(9)])
        .split(cols[0]);

    draw_map(f, state, left[0]);
    draw_log(f, state, left[1]);
    draw_sidebar(f, state, cols[1]);
}

fn draw_map(f: &mut Frame, state: &GameState, area: Rect) {
    let block = Block::default()
        .title(format!(
            " ⚔  VimQuest — Dungeon Level {} ",
            state.current_level
        ))
        .title_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .style(Style::default().bg(Color::Black));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let vh = inner.height as usize;
    let vw = inner.width as usize;
    let (cam_y, cam_x) = state.camera;
    let mut lines: Vec<Line> = vec![];

    for row in 0..vh {
        let my = cam_y + row;
        if my >= state.map.height {
            lines.push(Line::from(vec![]));
            continue;
        }

        let mut spans: Vec<Span> = vec![];
        for col in 0..vw {
            let mx = cam_x + col;
            if mx >= state.map.width {
                break;
            }

            let (py, px) = state.player.pos;
            if (my, mx) == (py, px) {
                spans.push(Span::styled(
                    "@",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ));
                continue;
            }

            let visible = state.map.visible[my][mx];
            let explored = state.map.explored[my][mx];

            if !explored {
                spans.push(Span::styled(" ", Style::default().bg(Color::Black)));
                continue;
            }

            if visible {
                if let Some(enemy) = state.enemies.iter().find(|e| e.pos == (my, mx)) {
                    let color = match enemy.kind {
                        EntityKind::Goblin => Color::LightGreen,
                        EntityKind::Orc => Color::LightRed,
                        EntityKind::Slime => Color::Green,
                        EntityKind::Skeleton => Color::Gray,
                        EntityKind::Troll => Color::Red,
                        EntityKind::Boss => Color::Magenta,
                    };
                    spans.push(Span::styled(
                        enemy.symbol.to_string(),
                        Style::default().fg(color).add_modifier(Modifier::BOLD),
                    ));
                    continue;
                }
                if let Some(item) = state.items.iter().find(|i| i.pos == (my, mx)) {
                    let (sym, color) = match &item.kind {
                        ItemKind::HealthPotion => ('!', Color::LightRed),
                        ItemKind::Sword => ('/', Color::LightBlue),
                        ItemKind::Shield => (')', Color::LightBlue),
                        ItemKind::Gold(_) => ('$', Color::Yellow),
                        ItemKind::Key => ('k', Color::Yellow),
                        ItemKind::Scroll => ('?', Color::Magenta),
                    };
                    spans.push(Span::styled(sym.to_string(), Style::default().fg(color)));
                    continue;
                }
            }

            let dim = !visible;
            let (ch, fg, bg) = match state.map.get(my, mx) {
                Tile::Wall => ('#', Color::DarkGray, Color::Black),
                Tile::Floor => (
                    '.',
                    if dim {
                        Color::DarkGray
                    } else {
                        Color::Rgb(60, 60, 60)
                    },
                    Color::Black,
                ),
                Tile::Door(true) => (
                    '/',
                    if dim { Color::DarkGray } else { Color::Yellow },
                    Color::Black,
                ),
                Tile::Door(false) => (
                    '+',
                    if dim {
                        Color::DarkGray
                    } else {
                        Color::LightYellow
                    },
                    Color::Black,
                ),
                Tile::Stairs => (
                    '>',
                    if dim {
                        Color::DarkGray
                    } else {
                        Color::LightCyan
                    },
                    Color::Black,
                ),
                Tile::Torch => (
                    '*',
                    if dim { Color::DarkGray } else { Color::Yellow },
                    Color::Black,
                ),
                Tile::Chest(false) => (
                    '=',
                    if dim {
                        Color::DarkGray
                    } else {
                        Color::LightYellow
                    },
                    Color::Black,
                ),
                Tile::Chest(true) => ('_', Color::DarkGray, Color::Black),
                Tile::Water => (
                    '~',
                    if dim { Color::DarkGray } else { Color::Blue },
                    Color::Black,
                ),
                Tile::Grass => (
                    ',',
                    if dim { Color::DarkGray } else { Color::Green },
                    Color::Black,
                ),
                Tile::Tree => (
                    '%',
                    if dim { Color::DarkGray } else { Color::Green },
                    Color::Black,
                ),
                Tile::Exit => (
                    'E',
                    if dim {
                        Color::DarkGray
                    } else {
                        Color::LightGreen
                    },
                    Color::Black,
                ),
            };

            let mut style = Style::default().fg(fg).bg(bg);
            if !dim && matches!(state.map.get(my, mx), Tile::Torch) {
                style = style.add_modifier(Modifier::BOLD);
            }
            spans.push(Span::styled(ch.to_string(), style));
        }
        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(Text::from(lines)).style(Style::default().bg(Color::Black));
    f.render_widget(paragraph, inner);
}

fn draw_log(f: &mut Frame, state: &GameState, area: Rect) {
    let block = Block::default()
        .title(" 📋 Log ")
        .title_style(Style::default().fg(Color::DarkGray))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .style(Style::default().bg(Color::Black));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let max_lines = inner.height as usize;
    let start = state.log.len().saturating_sub(max_lines);
    let recent: Vec<&LogEntry> = state.log[start..].iter().collect();

    let items: Vec<ListItem> = recent
        .iter()
        .rev()
        .map(|entry| {
            ListItem::new(Line::from(vec![Span::styled(
                entry.message.clone(),
                Style::default().fg(Color::White),
            )]))
        })
        .collect();

    let list = List::new(items).style(Style::default().bg(Color::Black));
    f.render_widget(list, inner);
}

fn draw_sidebar(f: &mut Frame, state: &GameState, area: Rect) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(14),
            Constraint::Length(8),
            Constraint::Min(6),
        ])
        .split(area);

    draw_char_stats(f, state, sections[0]);
    draw_bars(f, state, sections[1]);
    draw_vim_sheet(f, state, sections[2]);
}

fn draw_char_stats(f: &mut Frame, state: &GameState, area: Rect) {
    let p = &state.player;
    let block = Block::default()
        .title(" ⚔  Hero ")
        .title_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .style(Style::default().bg(Color::Black));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let sword_str = if p.has_sword {
        "⚔ Iron Sword"
    } else {
        "  Fists"
    };
    let shield_str = if p.has_shield {
        "🛡 Wooden Shield"
    } else {
        "  None"
    };

    let lines = vec![
        Line::from(vec![
            Span::styled(
                "  @ ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "The Vim Warrior",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::styled(
            format!("  Level  : {}", p.level),
            Style::default().fg(Color::LightYellow),
        )]),
        Line::from(vec![Span::styled(
            format!("  HP     : {}/{}", p.hp, p.max_hp),
            if p.hp < p.max_hp / 3 {
                Style::default().fg(Color::Red)
            } else if p.hp < p.max_hp * 2 / 3 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Green)
            },
        )]),
        Line::from(vec![Span::styled(
            format!("  XP     : {}/{}", p.xp, p.xp_to_next_level()),
            Style::default().fg(Color::LightBlue),
        )]),
        Line::from(vec![Span::styled(
            format!("  Gold   : {} 💰", p.gold),
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![Span::styled(
            format!("  Atk    : {}", p.attack + if p.has_sword { 3 } else { 0 }),
            Style::default().fg(Color::LightRed),
        )]),
        Line::from(vec![Span::styled(
            format!(
                "  Def    : {}",
                p.defense + if p.has_shield { 2 } else { 0 }
            ),
            Style::default().fg(Color::LightBlue),
        )]),
        Line::from(vec![Span::styled(
            format!("  Weapon : {}", sword_str),
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            format!("  Armor  : {}", shield_str),
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            format!("  Potions: {} 🧪", p.potions),
            Style::default().fg(Color::LightRed),
        )]),
        Line::from(vec![Span::styled(
            format!("  Keys   : {} 🗝", p.keys),
            Style::default().fg(Color::Yellow),
        )]),
    ];

    let paragraph = Paragraph::new(Text::from(lines)).style(Style::default().bg(Color::Black));
    f.render_widget(paragraph, inner);
}

fn draw_bars(f: &mut Frame, state: &GameState, area: Rect) {
    let p = &state.player;
    let block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray))
        .style(Style::default().bg(Color::Black));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let bar_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(1),
        ])
        .split(inner);

    let hp_ratio = p.hp as f64 / p.max_hp as f64;
    let hp_color = if hp_ratio < 0.3 {
        Color::Red
    } else if hp_ratio < 0.6 {
        Color::Yellow
    } else {
        Color::Green
    };
    let hp_gauge = Gauge::default()
        .label(format!("HP {}/{}", p.hp, p.max_hp))
        .ratio(hp_ratio.min(1.0))
        .gauge_style(Style::default().fg(hp_color).bg(Color::DarkGray));
    f.render_widget(hp_gauge, bar_areas[0]);

    let xp_ratio = p.xp as f64 / p.xp_to_next_level() as f64;
    let xp_gauge = Gauge::default()
        .label(format!("XP {}/{}", p.xp, p.xp_to_next_level()))
        .ratio(xp_ratio.min(1.0))
        .gauge_style(Style::default().fg(Color::LightBlue).bg(Color::DarkGray));
    f.render_widget(xp_gauge, bar_areas[2]);
}

fn draw_vim_sheet(f: &mut Frame, state: &GameState, area: Rect) {
    let learned = &state.player.vim_lessons_learned;
    let all = all_lessons();

    let block = Block::default()
        .title(" 📜 Vim Motions ")
        .title_style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .style(Style::default().bg(Color::Black));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = vec![
        Line::from(vec![Span::styled(
            " Movement (always): ",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(vec![
            Span::styled(
                " h",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ←  ", Style::default().fg(Color::White)),
            Span::styled(
                "j",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ↓  ", Style::default().fg(Color::White)),
            Span::styled(
                "k",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ↑  ", Style::default().fg(Color::White)),
            Span::styled(
                "l",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" →", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![Span::styled(
            " ─────────────────────────",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(vec![Span::styled(
            " Unlocked motions:",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    for lesson in &all {
        if learned.contains(&lesson.key.to_string()) {
            lines.push(Line::from(vec![
                Span::styled(
                    format!(" {:4}", lesson.key),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(lesson.description, Style::default().fg(Color::White)),
            ]));
        }
    }

    if learned.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            " Find scrolls (?) to unlock!",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )]));
    }

    lines.push(Line::from(vec![Span::styled("", Style::default())]));
    lines.push(Line::from(vec![Span::styled(
        " ─────────────────────────",
        Style::default().fg(Color::DarkGray),
    )]));
    lines.push(Line::from(vec![
        Span::styled(" p", Style::default().fg(Color::LightRed)),
        Span::styled(" Use potion  ", Style::default().fg(Color::DarkGray)),
        Span::styled("?", Style::default().fg(Color::White)),
        Span::styled(" Help", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled(" q", Style::default().fg(Color::DarkGray)),
        Span::styled(" Quit", Style::default().fg(Color::DarkGray)),
    ]));

    let paragraph = Paragraph::new(Text::from(lines)).style(Style::default().bg(Color::Black));
    f.render_widget(paragraph, inner);
}

fn draw_level_up_overlay(f: &mut Frame, state: &GameState) {
    let size = f.size();
    let area = centered_rect(50, 40, size);
    let p = &state.player;

    let text = vec![
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  ✨  LEVEL UP!  ✨",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            format!("  You reached level {}!", p.level),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            format!("  HP:     {}/{}", p.hp, p.max_hp),
            Style::default().fg(Color::Green),
        )]),
        Line::from(vec![Span::styled(
            format!("  Attack: {}", p.attack),
            Style::default().fg(Color::LightRed),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  Press ENTER to continue",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let popup = Paragraph::new(Text::from(text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .style(Style::default().bg(Color::Black)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(ratatui::widgets::Clear, area);
    f.render_widget(popup, area);
}

fn draw_lesson_overlay(f: &mut Frame, _state: &GameState, idx: usize) {
    let size = f.size();
    let area = centered_rect(60, 50, size);
    let lessons = all_lessons();
    let lesson = &lessons[idx.min(lessons.len() - 1)];

    let text = vec![
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  📜  NEW VIM MOTION UNLOCKED!",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  ─────────────────────────────",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(vec![
            Span::styled("  Key:    ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("  {}  ", lesson.key),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![
            Span::styled("  Action: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                lesson.description,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![
            Span::styled("  Tip:    ", Style::default().fg(Color::DarkGray)),
            Span::styled(lesson.example, Style::default().fg(Color::LightCyan)),
        ]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  ─────────────────────────────",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  This motion is now active in the dungeon!",
            Style::default().fg(Color::Green),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  Press ENTER to continue your quest...",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let popup = Paragraph::new(Text::from(text))
        .block(
            Block::default()
                .title(" Scroll of Knowledge ")
                .title_style(Style::default().fg(Color::Magenta))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta))
                .style(Style::default().bg(Color::Black)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(ratatui::widgets::Clear, area);
    f.render_widget(popup, area);
}

fn draw_challenge_overlay(f: &mut Frame, idx: usize, input: &str) {
    let size = f.size();
    let area = centered_rect(60, 50, size);
    let lessons = all_lessons();
    let lesson = &lessons[idx.min(lessons.len() - 1)];

    let text = vec![
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  📜  VIM SCROLL CHALLENGE",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  ─────────────────────────────",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            format!("  {}", lesson.challenge),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![
            Span::styled("  Hint: ", Style::default().fg(Color::DarkGray)),
            Span::styled(lesson.description, Style::default().fg(Color::LightCyan)),
        ]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  ─────────────────────────────",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![
            Span::styled("  Your answer: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!(" {} ", if input.is_empty() { "_" } else { input }),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  Type the motion key(s), Enter to confirm",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(vec![Span::styled(
            "  Esc to abandon (scroll is lost!)",
            Style::default().fg(Color::Red),
        )]),
    ];

    let popup = Paragraph::new(Text::from(text))
        .block(
            Block::default()
                .title(" Scroll Challenge ")
                .title_style(Style::default().fg(Color::Yellow))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .style(Style::default().bg(Color::Black)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(ratatui::widgets::Clear, area);
    f.render_widget(popup, area);
}

fn draw_game_over(f: &mut Frame, state: &GameState) {
    let size = f.size();
    f.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        size,
    );

    let text = vec![
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  ☠  GAME OVER  ☠",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  You have fallen in the dungeon...",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            format!("  Reached Level:    {}", state.player.level),
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![Span::styled(
            format!("  Dungeon Floor:    {}", state.current_level),
            Style::default().fg(Color::LightCyan),
        )]),
        Line::from(vec![Span::styled(
            format!("  Gold collected:   {}", state.player.gold),
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![Span::styled(
            format!(
                "  Vim motions learned: {}",
                state.player.vim_lessons_learned.len()
            ),
            Style::default().fg(Color::Magenta),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  Press ENTER to try again | q to quit",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    f.render_widget(
        Paragraph::new(Text::from(text))
            .alignment(Alignment::Left)
            .style(Style::default().bg(Color::Black)),
        size,
    );
}

fn draw_victory(f: &mut Frame, state: &GameState) {
    let size = f.size();
    f.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        size,
    );

    let learned = &state.player.vim_lessons_learned;

    let text = vec![
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  🏆  VICTORY!  🏆",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  You have defeated the Dragon and mastered the dungeon!",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            format!("  Final Level:    {}", state.player.level),
            Style::default().fg(Color::LightYellow),
        )]),
        Line::from(vec![Span::styled(
            format!("  Gold:           {}", state.player.gold),
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  ✨ Vim Motions Mastered:",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            format!("  {}", learned.join("  ")),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  You are now ready to wield Vim in the real world!",
            Style::default().fg(Color::Green),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  Press ENTER to play again | q to quit",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    f.render_widget(
        Paragraph::new(Text::from(text))
            .alignment(Alignment::Left)
            .style(Style::default().bg(Color::Black)),
        size,
    );
}

fn draw_help(f: &mut Frame, _state: &GameState) {
    let size = f.size();
    let area = centered_rect(70, 85, size);

    let text = vec![
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  ⚔  VIMQUEST HELP",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  MOVEMENT (always available)",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "  h / j / k / l   Move left / down / up / right",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  UNLOCKABLE VIM MOTIONS (find scrolls ?)",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "  w     Jump forward (next open section)",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  b     Jump backward one section",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  e     Jump to end of open section",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  0     Teleport to start of current row",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  $     Teleport to end of current row",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  gg    Teleport to top of dungeon (same column)",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  G     Teleport to bottom of dungeon",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  H     Jump to top area of screen",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  M     Jump to middle of screen",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  L     Jump to bottom area of screen",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  ACTIONS",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "  p     Use health potion (from inventory)",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  q     Quit the game",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  ?     Toggle this help screen",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  MAP SYMBOLS",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "  @  You (Vim Warrior)    # Wall     . Floor",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  >  Stairs (next level)  + Door     E Exit (win!)",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  !  Health Potion         $  Gold    ?  Scroll",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  / Sword   ) Shield   k Key",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled(
            "  g Goblin  O Orc   s Slime  k Skeleton  T Troll  D Dragon",
            Style::default().fg(Color::White),
        )]),
        Line::from(vec![Span::styled("", Style::default())]),
        Line::from(vec![Span::styled(
            "  Press ? or Esc to return to game",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let popup = Paragraph::new(Text::from(text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .style(Style::default().bg(Color::Black)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(ratatui::widgets::Clear, area);
    f.render_widget(popup, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
