use super::Recipe;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct Registry {
    recipes: BTreeMap<String, Recipe>,
}

impl Registry {
    pub fn new(recipes: Vec<Recipe>) -> Self {
        let mut indexed = BTreeMap::new();
        for recipe in recipes {
            indexed.insert(recipe.key(), recipe);
        }

        Self { recipes: indexed }
    }

    pub fn namespaces(&self) -> Vec<&str> {
        self.recipes
            .values()
            .map(|recipe| recipe.namespace.as_str())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    pub fn all(&self) -> impl Iterator<Item = &Recipe> {
        self.recipes.values()
    }

    pub fn list(&self, namespace: Option<&str>) -> Vec<&Recipe> {
        self.all()
            .filter(|recipe| namespace.is_none_or(|ns| recipe.namespace == ns))
            .collect()
    }

    pub fn get(&self, key: &str) -> Option<&Recipe> {
        self.recipes.get(key)
    }
}
