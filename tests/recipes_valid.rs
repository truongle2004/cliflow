use cliflow::recipe::loader::load_embedded_recipes;
use std::collections::BTreeSet;

#[test]
fn embedded_recipes_are_valid() {
    let recipes = load_embedded_recipes().expect("embedded recipes should load");
    assert!(!recipes.is_empty(), "expected embedded recipes");

    for recipe in recipes {
        assert!(!recipe.id.trim().is_empty(), "recipe id is required");
        assert!(
            !recipe.namespace.trim().is_empty(),
            "recipe namespace is required for {}",
            recipe.id
        );
        assert!(
            !recipe.title.trim().is_empty(),
            "recipe title is required for {}",
            recipe.key()
        );
        assert!(
            !recipe.command.trim().is_empty(),
            "recipe command is required for {}",
            recipe.key()
        );

        let arg_names = recipe
            .args
            .iter()
            .map(|arg| arg.name.as_str())
            .collect::<BTreeSet<_>>();

        for token in command_tokens(&recipe.command) {
            assert!(
                arg_names.contains(token.as_str()),
                "{} uses {{{}}} but has no matching arg",
                recipe.key(),
                token
            );
        }
    }
}

fn command_tokens(command: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut rest = command;

    while let Some(start) = rest.find('{') {
        rest = &rest[start + 1..];
        let Some(end) = rest.find('}') else {
            break;
        };
        tokens.push(rest[..end].to_string());
        rest = &rest[end + 1..];
    }

    tokens
}
