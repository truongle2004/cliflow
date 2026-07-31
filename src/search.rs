use crate::recipe::Recipe;

#[derive(Debug)]
pub struct SearchResult<'a> {
    pub recipe: &'a Recipe,
    pub score: usize,
}

pub fn search<'a>(recipes: impl Iterator<Item = &'a Recipe>, query: &str) -> Vec<SearchResult<'a>> {
    let query = query.to_lowercase();
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

    fuzzy_score(&haystack, query)
}

fn fuzzy_score(haystack: &str, query: &str) -> Option<usize> {
    let mut score = 0;
    let mut chars = haystack.chars();

    for needle in query.chars() {
        for candidate in chars.by_ref() {
            if candidate == needle {
                score += 1;
                break;
            }
        }
    }

    (score == query.chars().count()).then_some(100 + score)
}
