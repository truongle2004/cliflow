use crate::error::Result;
use crate::recipe::{Danger, Recipe, Registry};
use crate::search;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use std::io;

pub fn run(registry: &Registry) -> Result<()> {
    let mut terminal = setup_terminal()?;
    let result = App::new(registry).run(&mut terminal);
    restore_terminal(&mut terminal)?;
    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

struct App<'a> {
    registry: &'a Registry,
    query: String,
    selected: usize,
}

impl<'a> App<'a> {
    fn new(registry: &'a Registry) -> Self {
        Self {
            registry,
            query: String::new(),
            selected: 0,
        }
    }

    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => break,
                    KeyCode::Backspace => {
                        self.query.pop();
                        self.selected = 0;
                    }
                    KeyCode::Char(char) => {
                        self.query.push(char);
                        self.selected = 0;
                    }
                    KeyCode::Down => self.move_selection(1),
                    KeyCode::Up => self.move_selection(-1),
                    KeyCode::Home => self.selected = 0,
                    KeyCode::End => self.selected = self.visible_recipes().len().saturating_sub(1),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame<'_>) {
        let recipes = self.visible_recipes();
        self.selected = self.selected.min(recipes.len().saturating_sub(1));

        let root = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(8),
                Constraint::Length(3),
            ])
            .split(root);

        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
            .split(chunks[1]);

        frame.render_widget(Clear, root);
        self.draw_search(frame, chunks[0]);
        self.draw_list(frame, body[0], &recipes);
        self.draw_details(frame, body[1], recipes.get(self.selected).copied());
        self.draw_help(frame, chunks[2], recipes.len());
    }

    fn draw_search(&self, frame: &mut Frame<'_>, area: Rect) {
        let text = if self.query.is_empty() {
            Line::from(Span::styled(
                "Type to search recipes...",
                Style::default().fg(Color::DarkGray),
            ))
        } else {
            Line::from(self.query.as_str())
        };

        frame.render_widget(
            Paragraph::new(text).block(Block::default().title("Search").borders(Borders::ALL)),
            area,
        );
    }

    fn draw_list(&self, frame: &mut Frame<'_>, area: Rect, recipes: &[&Recipe]) {
        let items = recipes
            .iter()
            .map(|recipe| {
                ListItem::new(Line::from(vec![
                    Span::styled(recipe.key(), Style::default().fg(Color::Cyan)),
                    Span::raw("  "),
                    Span::raw(recipe.title.as_str()),
                ]))
            })
            .collect::<Vec<_>>();

        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(self.selected));
        }

        frame.render_stateful_widget(
            List::new(items)
                .block(Block::default().title("Recipes").borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("  "),
            area,
            &mut state,
        );
    }

    fn draw_details(&self, frame: &mut Frame<'_>, area: Rect, recipe: Option<&Recipe>) {
        let lines = match recipe {
            Some(recipe) => recipe_lines(recipe),
            None => vec![Line::from("No recipes match your search.")],
        };

        frame.render_widget(
            Paragraph::new(lines)
                .block(Block::default().title("Information").borders(Borders::ALL))
                .wrap(Wrap { trim: false }),
            area,
        );
    }

    fn draw_help(&self, frame: &mut Frame<'_>, area: Rect, total: usize) {
        let help = format!(
            "{} result{} | Type: search | Up/Down: select | Esc/Ctrl-C: quit",
            total,
            if total == 1 { "" } else { "s" }
        );
        frame.render_widget(
            Paragraph::new(help).block(Block::default().title("Help").borders(Borders::ALL)),
            area,
        );
    }

    fn visible_recipes(&self) -> Vec<&'a Recipe> {
        let query = self.query.trim();
        if query.is_empty() {
            self.registry.list(None)
        } else {
            search::search(self.registry.all(), query)
                .into_iter()
                .map(|result| result.recipe)
                .collect()
        }
    }

    fn move_selection(&mut self, delta: isize) {
        let len = self.visible_recipes().len();
        if len == 0 {
            self.selected = 0;
            return;
        }

        self.selected = self
            .selected
            .saturating_add_signed(delta)
            .min(len.saturating_sub(1));
    }
}

fn recipe_lines(recipe: &Recipe) -> Vec<Line<'_>> {
    let mut lines = vec![
        Line::from(Span::styled(
            recipe.title.as_str(),
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(recipe.key(), Style::default().fg(Color::Cyan))),
        Line::from(""),
    ];

    if !recipe.description.is_empty() {
        lines.push(Line::from(recipe.description.as_str()));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(vec![
        Span::styled("Command: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(recipe.command.as_str(), Style::default().fg(Color::Green)),
    ]));

    if !recipe.args.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Placeholders",
            Style::default().add_modifier(Modifier::BOLD),
        )));
        for arg in &recipe.args {
            let default = arg
                .default
                .as_ref()
                .map(|value| format!(" [{value}]"))
                .unwrap_or_default();
            lines.push(Line::from(format!(
                "{}: {}{}",
                arg.name, arg.prompt, default
            )));
        }
    }

    if !recipe.tags.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Tags: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(recipe.tags.join(", ")),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("Danger: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(recipe.danger.to_string(), danger_style(recipe.danger)),
    ]));

    lines
}

fn danger_style(danger: Danger) -> Style {
    match danger {
        Danger::Low => Style::default().fg(Color::Green),
        Danger::Medium => Style::default().fg(Color::Yellow),
        Danger::High => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    }
}
