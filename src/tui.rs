use crate::domain::workflow::Workflow;
use crate::error::Result;
use crate::recipe::{Danger, Recipe, Registry};
use crate::search;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
    MouseButton, MouseEvent, MouseEventKind,
};
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

pub fn run(registry: &Registry, workflows: &[Workflow]) -> Result<()> {
    let mut terminal = setup_terminal()?;
    let result = App::new(registry, workflows).run(&mut terminal);
    restore_terminal(&mut terminal)?;
    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    Ok(())
}

struct App<'a> {
    registry: &'a Registry,
    workflows: &'a [Workflow],
    query: String,
    searching: bool,
    active_pane: ActivePane,
    selected_recipe: usize,
    selected_workflow: usize,
    detail_scroll: u16,
    search_area: Rect,
    recipe_area: Rect,
    workflow_area: Rect,
    detail_area: Rect,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ActivePane {
    Recipes,
    Workflows,
}

impl<'a> App<'a> {
    fn new(registry: &'a Registry, workflows: &'a [Workflow]) -> Self {
        Self {
            registry,
            workflows,
            query: String::new(),
            searching: false,
            active_pane: ActivePane::Recipes,
            selected_recipe: 0,
            selected_workflow: 0,
            detail_scroll: 0,
            search_area: Rect::default(),
            recipe_area: Rect::default(),
            workflow_area: Rect::default(),
            detail_area: Rect::default(),
        }
    }

    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if self.handle_key(key.code, key.modifiers) {
                        break;
                    }
                }
                Event::Mouse(mouse) => self.handle_mouse(mouse),
                _ => {}
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) -> bool {
        match code {
            KeyCode::Char('c') if modifiers == KeyModifiers::CONTROL => return true,
            KeyCode::Esc if self.searching => {
                self.query.clear();
                self.searching = false;
                self.reset_selection();
            }
            KeyCode::Esc => return true,
            KeyCode::Char('/') if !self.searching => {
                self.searching = true;
            }
            KeyCode::Enter if self.searching => self.searching = false,
            KeyCode::Backspace if self.searching => {
                self.query.pop();
                self.reset_selection_for_query();
            }
            KeyCode::Char(char) if self.searching => {
                self.query.push(char);
                self.reset_selection_for_query();
            }
            KeyCode::Tab => self.toggle_active_pane(),
            KeyCode::Down | KeyCode::Char('j') => self.move_selection(1),
            KeyCode::Up | KeyCode::Char('k') => self.move_selection(-1),
            KeyCode::Home => self.set_selection(0),
            KeyCode::End => self.move_selection(isize::MAX),
            KeyCode::PageDown => self.scroll_details(8),
            KeyCode::PageUp => self.scroll_details(-8),
            _ => {}
        }

        false
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) {
        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) if area_contains(self.search_area, &mouse) => {
                self.searching = true;
            }
            MouseEventKind::Down(MouseButton::Left) if area_contains(self.recipe_area, &mouse) => {
                self.searching = false;
                self.active_pane = ActivePane::Recipes;
                if let Some(index) = list_index(self.recipe_area, mouse.row)
                    && index < self.visible_recipes().len()
                {
                    self.set_selection(index);
                }
            }
            MouseEventKind::Down(MouseButton::Left)
                if area_contains(self.workflow_area, &mouse) =>
            {
                self.searching = false;
                self.active_pane = ActivePane::Workflows;
                if let Some(index) = list_index(self.workflow_area, mouse.row)
                    && index < self.visible_workflows().len()
                {
                    self.set_selection(index);
                }
            }
            MouseEventKind::ScrollDown if area_contains(self.detail_area, &mouse) => {
                self.scroll_details(3);
            }
            MouseEventKind::ScrollUp if area_contains(self.detail_area, &mouse) => {
                self.scroll_details(-3);
            }
            _ => {}
        }
    }

    fn draw(&mut self, frame: &mut Frame<'_>) {
        let recipes = self.visible_recipes();
        let workflows = self.visible_workflows();
        self.selected_recipe = self.selected_recipe.min(recipes.len().saturating_sub(1));
        self.selected_workflow = self
            .selected_workflow
            .min(workflows.len().saturating_sub(1));

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
        let lists = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(body[0]);

        self.search_area = chunks[0];
        self.recipe_area = lists[0];
        self.workflow_area = lists[1];
        self.detail_area = body[1];

        frame.render_widget(Clear, root);
        self.draw_search(frame, chunks[0]);
        self.draw_recipe_list(frame, lists[0], &recipes);
        self.draw_workflow_list(frame, lists[1], &workflows);
        self.draw_details(frame, body[1], recipes.as_slice(), workflows.as_slice());
        self.draw_help(frame, chunks[2], recipes.len(), workflows.len());
    }

    fn draw_search(&self, frame: &mut Frame<'_>, area: Rect) {
        let text = if self.query.is_empty() {
            Line::from(Span::styled(
                if self.searching {
                    "Type to search recipes and workflows..."
                } else {
                    "Press / to search recipes and workflows..."
                },
                Style::default().fg(Color::DarkGray),
            ))
        } else {
            Line::from(self.query.as_str())
        };

        frame.render_widget(
            Paragraph::new(text).block(
                Block::default()
                    .title(if self.searching {
                        "Search (active)"
                    } else {
                        "Search"
                    })
                    .borders(Borders::ALL),
            ),
            area,
        );
    }

    fn draw_recipe_list(&self, frame: &mut Frame<'_>, area: Rect, recipes: &[&Recipe]) {
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
            state.select(Some(self.selected_recipe));
        }

        frame.render_stateful_widget(
            List::new(items)
                .block(section_block(
                    "Recipes",
                    self.active_pane == ActivePane::Recipes,
                ))
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

    fn draw_workflow_list(&self, frame: &mut Frame<'_>, area: Rect, workflows: &[&Workflow]) {
        let items = workflows
            .iter()
            .map(|workflow| {
                ListItem::new(Line::from(vec![
                    Span::styled(
                        format!("{}/{}", workflow.tool.id, workflow.id),
                        Style::default().fg(Color::Magenta),
                    ),
                    Span::raw("  "),
                    Span::raw(workflow.title.as_str()),
                ]))
            })
            .collect::<Vec<_>>();

        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(self.selected_workflow));
        }

        frame.render_stateful_widget(
            List::new(items)
                .block(section_block(
                    "Workflows",
                    self.active_pane == ActivePane::Workflows,
                ))
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("  "),
            area,
            &mut state,
        );
    }

    fn draw_details(
        &self,
        frame: &mut Frame<'_>,
        area: Rect,
        recipes: &[&Recipe],
        workflows: &[&Workflow],
    ) {
        let lines = match self.active_pane {
            ActivePane::Recipes => recipes
                .get(self.selected_recipe)
                .map(|recipe| recipe_lines(recipe))
                .unwrap_or_else(|| vec![Line::from("No recipes match your search.")]),
            ActivePane::Workflows => workflows
                .get(self.selected_workflow)
                .map(|workflow| workflow_lines(workflow))
                .unwrap_or_else(|| vec![Line::from("No workflows match your search.")]),
        };

        let title = match self.active_pane {
            ActivePane::Recipes => "Recipe Information",
            ActivePane::Workflows => "Workflow Information",
        };

        frame.render_widget(
            Paragraph::new(lines)
                .block(Block::default().title(title).borders(Borders::ALL))
                .scroll((self.detail_scroll, 0))
                .wrap(Wrap { trim: false }),
            area,
        );
    }

    fn draw_help(&self, frame: &mut Frame<'_>, area: Rect, recipes: usize, workflows: usize) {
        let help = format!(
            "{} recipe{} | {} workflow{} | /: search | j/k or Up/Down: select | Tab: switch pane | PgUp/PgDn: details | Mouse: select/scroll | Enter: finish search | Esc/Ctrl-C: quit",
            recipes,
            if recipes == 1 { "" } else { "s" },
            workflows,
            if workflows == 1 { "" } else { "s" }
        );
        frame.render_widget(
            Paragraph::new(help).block(Block::default().title("Help").borders(Borders::ALL)),
            area,
        );
    }

    fn visible_recipes(&self) -> Vec<&'a Recipe> {
        let query = self.query.trim();
        if query.is_empty() {
            self.registry.list(None).into_iter().collect::<Vec<_>>()
        } else {
            search::search(self.registry.all(), query)
                .into_iter()
                .map(|result| result.recipe)
                .collect()
        }
    }

    fn visible_workflows(&self) -> Vec<&'a Workflow> {
        let query = self.query.trim();
        self.workflows
            .iter()
            .filter(|workflow| query.is_empty() || workflow_matches(workflow, query))
            .collect()
    }

    fn move_selection(&mut self, delta: isize) {
        let len = match self.active_pane {
            ActivePane::Recipes => self.visible_recipes().len(),
            ActivePane::Workflows => self.visible_workflows().len(),
        };
        if len == 0 {
            self.set_selection(0);
            return;
        }

        let selected = self.current_selection();
        self.set_selection(
            selected
                .saturating_add_signed(delta)
                .min(len.saturating_sub(1)),
        );
    }

    fn current_selection(&self) -> usize {
        match self.active_pane {
            ActivePane::Recipes => self.selected_recipe,
            ActivePane::Workflows => self.selected_workflow,
        }
    }

    fn set_selection(&mut self, selected: usize) {
        match self.active_pane {
            ActivePane::Recipes => self.selected_recipe = selected,
            ActivePane::Workflows => self.selected_workflow = selected,
        }
        self.detail_scroll = 0;
    }

    fn reset_selection(&mut self) {
        self.selected_recipe = 0;
        self.selected_workflow = 0;
        self.detail_scroll = 0;
    }

    fn reset_selection_for_query(&mut self) {
        self.reset_selection();

        let query = self.query.trim();
        if query.split_whitespace().count() > 1 && !self.visible_workflows().is_empty() {
            self.active_pane = ActivePane::Workflows;
        }
    }

    fn toggle_active_pane(&mut self) {
        self.active_pane = match self.active_pane {
            ActivePane::Recipes => ActivePane::Workflows,
            ActivePane::Workflows => ActivePane::Recipes,
        };
        self.detail_scroll = 0;
    }

    fn scroll_details(&mut self, delta: i16) {
        if delta.is_negative() {
            self.detail_scroll = self.detail_scroll.saturating_sub(delta.unsigned_abs());
        } else {
            self.detail_scroll = self.detail_scroll.saturating_add(delta as u16);
        }
    }
}

fn area_contains(area: Rect, mouse: &MouseEvent) -> bool {
    mouse.column >= area.x
        && mouse.column < area.x.saturating_add(area.width)
        && mouse.row >= area.y
        && mouse.row < area.y.saturating_add(area.height)
}

fn list_index(area: Rect, row: u16) -> Option<usize> {
    let first_content_row = area.y.saturating_add(1);
    let bottom_border_row = area.y.saturating_add(area.height).saturating_sub(1);

    (row >= first_content_row && row < bottom_border_row)
        .then_some(usize::from(row - first_content_row))
}

fn section_block(title: &'static str, active: bool) -> Block<'static> {
    let style = if active {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    Block::default()
        .title(Span::styled(title, style))
        .borders(Borders::ALL)
}

fn workflow_matches(workflow: &Workflow, query: &str) -> bool {
    let query = query.to_lowercase();
    let key = format!("{}/{}", workflow.tool.id, workflow.id).to_lowercase();
    let title = workflow.title.to_lowercase();
    let description = workflow.description.to_lowercase();
    let tags = workflow.tags.join(" ").to_lowercase();
    let steps = workflow
        .steps
        .iter()
        .map(|step| {
            format!(
                "{} {} {}",
                step.title,
                step.description,
                workflow_command(&step.command)
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();

    let haystack = format!("{key} {title} {description} {tags} {steps}");

    query.split_whitespace().all(|term| haystack.contains(term))
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

    if !recipe.example.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "CLI Example",
            Style::default().add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(recipe.example.as_str()));
    }

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

fn workflow_lines(workflow: &Workflow) -> Vec<Line<'_>> {
    let mut lines = vec![
        Line::from(Span::styled(
            workflow.title.as_str(),
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("{}/{}", workflow.tool.id, workflow.id),
            Style::default().fg(Color::Magenta),
        )),
        Line::from(""),
    ];

    if !workflow.description.is_empty() {
        lines.push(Line::from(workflow.description.as_str()));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(
        "Workflow CLI Example",
        Style::default().add_modifier(Modifier::BOLD),
    )));

    for (index, step) in workflow.steps.iter().enumerate() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(
                format!("{}. ", index + 1),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                step.title.as_str(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]));
        if !step.description.is_empty() {
            lines.push(Line::from(step.description.as_str()));
        }
        lines.push(Line::from(Span::styled(
            workflow_command(&step.command),
            Style::default().fg(Color::Green),
        )));
    }

    lines
}

fn workflow_command(command: &crate::domain::command::Command) -> String {
    std::iter::once(command.program.as_str())
        .chain(command.args.iter().map(String::as_str))
        .map(quote_arg)
        .collect::<Vec<_>>()
        .join(" ")
}

fn quote_arg(arg: &str) -> String {
    if arg.contains(char::is_whitespace) {
        format!("\"{}\"", arg.replace('"', "\\\""))
    } else {
        arg.to_string()
    }
}

fn danger_style(danger: Danger) -> Style {
    match danger {
        Danger::Low => Style::default().fg(Color::Green),
        Danger::Medium => Style::default().fg(Color::Yellow),
        Danger::High => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    }
}

#[cfg(test)]
#[path = "../tests/support/tui.rs"]
mod tests;
