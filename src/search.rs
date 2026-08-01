use crate::recipe::Recipe;

#[derive(Debug)]
pub struct SearchResult<'a> {
    pub recipe: &'a Recipe,
    pub score: usize,
}

pub fn search<'a>(recipes: impl Iterator<Item = &'a Recipe>, query: &str) -> Vec<SearchResult<'a>> {
    let query = query.trim().to_lowercase();
    let mut results = recipes
        .filter_map(|recipe| score(recipe, &query).map(|score| SearchResult { recipe, score }))
        .collect::<Vec<_>>();

    results.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.recipe.key().cmp(&b.recipe.key()))
    });
    results.truncate(20);
    results
}

fn score(recipe: &Recipe, query: &str) -> Option<usize> {
    if query.is_empty() {
        return None;
    }

    let key = recipe.key().to_lowercase();
    let title = recipe.title.to_lowercase();
    let description = recipe.description.to_lowercase();
    let example = recipe.example.to_lowercase();
    let tags = recipe.tags.join(" ").to_lowercase();
    let haystack = format!("{key} {title} {description} {example} {tags}");

    if !haystack.contains(query) {
        return None;
    }

    if key == query {
        return Some(1000);
    }
    if key.contains(query) {
        return Some(800 + query.len());
    }
    if title.contains(query) {
        return Some(600 + query.len());
    }
    if tags.split_whitespace().any(|tag| tag == query) {
        return Some(500 + query.len());
    }
    if description.contains(query) || example.contains(query) || tags.contains(query) {
        return Some(300 + query.len());
    }

    Some(100 + query.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe::Danger;

    fn recipe() -> Recipe {
        Recipe {
            id: "status".to_string(),
            namespace: "git".to_string(),
            title: "Show repository status".to_string(),
            description: String::new(),
            example: "git status".to_string(),
            command: "git status".to_string(),
            tags: Vec::new(),
            danger: Danger::Low,
            args: Vec::new(),
        }
    }

    #[test]
    fn matches_case_insensitive_substrings() {
        let recipe = recipe();

        let results = search(std::iter::once(&recipe), "REPOSITORY STAT");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].recipe.key(), "git/status");
    }

    #[test]
    fn excludes_non_contiguous_fuzzy_matches() {
        let recipe = recipe();

        let results = search(std::iter::once(&recipe), "gs");

        assert!(results.is_empty());
    }
}
