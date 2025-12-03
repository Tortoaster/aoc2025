use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    iter, thread,
    thread::JoinHandle,
    time::{Duration, Instant},
};

use arboard::Clipboard;
use crossterm::{
    event,
    event::{Event, KeyCode, KeyEvent},
};
use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Padding},
    Frame,
};
use throbber_widgets_tui::{Throbber, ThrobberState};

mod day1;
mod day2;
mod vec2;

fn main() {
    #[rustfmt::skip]
    let days: Vec<(_, _, fn(_) -> _, fn(_) -> _)> = vec![
        (1, include_str!("../input/day1.txt"), day1::solve_a, day1::solve_b),
        (2, include_str!("../input/day2.txt"), day2::solve_a, day2::solve_b),
    ];

    let mut outputs: BTreeMap<(i32, Part), _> = days
        .into_iter()
        .flat_map(|(day, input, solve_a, solve_b)| {
            [
                (day, Part::One, input, solve_a),
                (day, Part::Two, input, solve_b),
            ]
        })
        .map(|(day, part, input, solve)| {
            (
                (day, part),
                Remember::new(thread::spawn(move || {
                    let start = Instant::now();
                    (solve(input), start.elapsed())
                })),
            )
        })
        .collect();
    let mut state = State::new(outputs.keys());

    let mut clipboard = Clipboard::new().unwrap();
    let mut terminal = ratatui::init();
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    loop {
        state.throbber_state.calc_next();
        terminal
            .draw(|frame| draw(frame, &mut outputs, &state))
            .unwrap();

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Left | KeyCode::Char('h') => state.move_left(),
                    KeyCode::Down | KeyCode::Char('j') => state.move_down(),
                    KeyCode::Up | KeyCode::Char('k') => state.move_up(),
                    KeyCode::Right | KeyCode::Char('l') => state.move_right(),
                    KeyCode::Enter => {
                        if let Some(&output) = outputs
                            .get_mut(&(state.day, state.part))
                            .and_then(Remember::poll)
                        {
                            clipboard.set_text(output.0.to_string()).unwrap();
                            state.copy();
                        }
                    }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    ratatui::restore();
}

fn draw(
    frame: &mut Frame,
    outputs: &mut BTreeMap<(i32, Part), Remember<(u64, Duration)>>,
    state: &State,
) {
    for (row_index, row) in Layout::vertical(iter::repeat_n(Constraint::Length(8), 5))
        .split(frame.area())
        .iter()
        .enumerate()
    {
        for (col_index, tile) in Layout::horizontal(iter::repeat_n(
            Constraint::Length(u64::MAX.to_string().len() as u16 + 14),
            5,
        ))
        .split(*row)
        .iter()
        .enumerate()
        {
            let day = (row_index * 5 + col_index + 1) as i32;

            let mut block = Block::bordered()
                .border_type(BorderType::Rounded)
                .title_alignment(Alignment::Center)
                .padding(Padding::proportional(1))
                .title(format!("[  Day {day}  ]"));
            block = if state.day == day {
                block.light_yellow()
            } else if outputs.contains_key(&(day, Part::One))
                || outputs.contains_key(&(day, Part::Two))
            {
                block.white()
            } else {
                block.gray()
            };
            let block_inner = block.inner(*tile);
            frame.render_widget(block, *tile);

            for (part, mut lines) in Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(block_inner)
            .iter()
            .chunks(2)
            .into_iter()
            .enumerate()
            {
                let part = Part::try_from(part + 1).unwrap();

                let chunks = Layout::horizontal([
                    Constraint::Length(8),
                    Constraint::Length(u64::MAX.to_string().len() as u16),
                ])
                .split(*lines.next().unwrap());

                let mut label = match state.copied {
                    Some((copy_day, copy_part, time))
                        if copy_day == day
                            && copy_part == part
                            && time.elapsed() < Duration::from_secs(1) =>
                    {
                        Span::from("Copied! ")
                    }
                    _ => Span::from(format!("Part {}: ", part)),
                };
                label = if state.day == day && state.part == part {
                    label.light_yellow()
                } else if outputs.contains_key(&(day, part)) {
                    label.white()
                } else {
                    label.gray()
                };
                frame.render_widget(label, chunks[0]);

                match outputs.get_mut(&(day, part)) {
                    None => {
                        let output = Span::from("-").gray().into_right_aligned_line();
                        frame.render_widget(output, chunks[1]);
                    }
                    Some(handle) => match handle.poll() {
                        None => {
                            let throbber = Throbber::default()
                                .throbber_set(throbber_widgets_tui::BRAILLE_SIX_DOUBLE)
                                .style(Style::new())
                                .to_line(&state.throbber_state)
                                .right_aligned();
                            frame.render_widget(throbber, chunks[1]);
                        }
                        Some(output) => {
                            let output = Span::from(output.0.to_string())
                                .cyan()
                                .into_right_aligned_line();
                            frame.render_widget(output, chunks[1]);
                        }
                    },
                }

                if let Some(&(_, duration)) = outputs.get_mut(&(day, part)).and_then(Remember::poll)
                {
                    let time = Span::from(format!("{:.5} s", duration.as_secs_f64()))
                        .gray()
                        .into_centered_line();
                    frame.render_widget(time, *lines.next().unwrap());
                }
            }
        }
    }
}

#[derive(Debug)]
struct State {
    day: i32,
    part: Part,
    copied: Option<(i32, Part, Instant)>,
    throbber_state: ThrobberState,
}

impl State {
    pub fn new<'a>(outputs: impl IntoIterator<Item = &'a (i32, Part)>) -> Self {
        let (day, part) = outputs.into_iter().max().copied().unwrap_or((1, Part::One));
        Self {
            day,
            part,
            copied: None,
            throbber_state: ThrobberState::default(),
        }
    }

    fn copy(&mut self) {
        self.copied = Some((self.day, self.part, Instant::now()));
    }

    fn move_left(&mut self) {
        self.day = (self.day - 1) / 5 * 5 + ((self.day - 1) % 5 - 1).rem_euclid(5) + 1;
    }

    fn move_right(&mut self) {
        self.day = (self.day - 1) / 5 * 5 + ((self.day - 1) % 5 + 1) % 5 + 1;
    }

    fn move_up(&mut self) {
        if self.part == Part::Two {
            self.part = Part::One;
        } else {
            self.day = ((self.day - 1) / 5 - 1).rem_euclid(5) * 5 + (self.day - 1) % 5 + 1;
            self.part = Part::Two;
        }
    }

    fn move_down(&mut self) {
        if self.part == Part::One {
            self.part = Part::Two;
        } else {
            self.day = ((self.day - 1) / 5 + 1) % 5 * 5 + (self.day - 1) % 5 + 1;
            self.part = Part::One;
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Part {
    One,
    Two,
}

impl TryFrom<usize> for Part {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            _ => Err("invalid part"),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

pub struct Remember<T> {
    handle: Option<JoinHandle<T>>,
    output: Option<T>,
}

impl<T> Remember<T> {
    pub fn new(handle: JoinHandle<T>) -> Self {
        Self {
            handle: Some(handle),
            output: None,
        }
    }

    pub fn poll(&mut self) -> Option<&T> {
        if self.output.is_some() {
            self.output.as_ref()
        } else if self.handle.as_ref()?.is_finished() {
            self.output = Some(self.handle.take()?.join().unwrap());
            self.output.as_ref()
        } else {
            None
        }
    }
}
