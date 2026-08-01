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
