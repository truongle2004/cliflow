use super::*;

#[test]
fn reentering_search_keeps_and_extends_the_previous_query() {
    let registry = Registry::new(Vec::new());
    let mut app = App::new(&registry, &[]);
    app.query = "dock".to_string();

    assert!(!app.handle_key(KeyCode::Char('/'), KeyModifiers::NONE));
    assert!(app.searching);
    assert_eq!(app.query, "dock");

    app.handle_key(KeyCode::Char('e'), KeyModifiers::NONE);
    assert_eq!(app.query, "docke");

    app.handle_key(KeyCode::Enter, KeyModifiers::NONE);
    assert!(!app.searching);
    assert_eq!(app.query, "docke");
}

#[test]
fn mouse_selects_a_recipe_and_scrolls_details() {
    let registry = Registry::new(vec![Recipe {
        id: "list".to_string(),
        namespace: "linux".to_string(),
        title: "List files".to_string(),
        description: String::new(),
        example: "ls".to_string(),
        command: "ls".to_string(),
        tags: Vec::new(),
        danger: Danger::Low,
        args: Vec::new(),
    }]);
    let mut app = App::new(&registry, &[]);
    app.recipe_area = Rect::new(0, 0, 20, 3);
    app.detail_area = Rect::new(20, 0, 20, 10);

    app.handle_mouse(MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 1,
        row: 1,
        modifiers: KeyModifiers::NONE,
    });
    assert!(matches!(app.active_pane, ActivePane::Recipes));
    assert_eq!(app.selected_recipe, 0);

    app.handle_mouse(MouseEvent {
        kind: MouseEventKind::ScrollDown,
        column: 21,
        row: 1,
        modifiers: KeyModifiers::NONE,
    });
    assert_eq!(app.detail_scroll, 3);
}

#[test]
fn mouse_scrolls_recipe_selection() {
    let registry = Registry::new(
        (0..5)
            .map(|index| Recipe {
                id: format!("recipe-{index}"),
                namespace: "linux".to_string(),
                title: format!("Recipe {index}"),
                description: String::new(),
                example: String::new(),
                command: "true".to_string(),
                tags: Vec::new(),
                danger: Danger::Low,
                args: Vec::new(),
            })
            .collect(),
    );
    let mut app = App::new(&registry, &[]);
    app.recipe_area = Rect::new(0, 0, 20, 4);

    app.handle_mouse(MouseEvent {
        kind: MouseEventKind::ScrollDown,
        column: 1,
        row: 1,
        modifiers: KeyModifiers::NONE,
    });
    assert!(matches!(app.active_pane, ActivePane::Recipes));
    assert_eq!(app.selected_recipe, 3);

    app.handle_mouse(MouseEvent {
        kind: MouseEventKind::ScrollUp,
        column: 1,
        row: 1,
        modifiers: KeyModifiers::NONE,
    });
    assert_eq!(app.selected_recipe, 0);
}

#[test]
fn mouse_scrolls_workflow_selection() {
    let workflows = (0..5)
        .map(|index| Workflow {
            id: format!("workflow-{index}"),
            title: format!("Workflow {index}"),
            description: String::new(),
            tool: crate::domain::tool::Tool {
                id: "git".to_string(),
                name: "Git".to_string(),
                description: String::new(),
            },
            category: crate::domain::category::Category {
                id: "test".to_string(),
                name: "Test".to_string(),
                description: String::new(),
            },
            risk: crate::domain::risk::Risk::Low,
            tags: Vec::new(),
            steps: Vec::new(),
        })
        .collect::<Vec<_>>();
    let registry = Registry::new(Vec::new());
    let mut app = App::new(&registry, &workflows);
    app.workflow_area = Rect::new(0, 0, 20, 4);

    app.handle_mouse(MouseEvent {
        kind: MouseEventKind::ScrollDown,
        column: 1,
        row: 1,
        modifiers: KeyModifiers::NONE,
    });
    assert!(matches!(app.active_pane, ActivePane::Workflows));
    assert_eq!(app.selected_workflow, 3);

    app.handle_mouse(MouseEvent {
        kind: MouseEventKind::ScrollUp,
        column: 1,
        row: 1,
        modifiers: KeyModifiers::NONE,
    });
    assert_eq!(app.selected_workflow, 0);
}
