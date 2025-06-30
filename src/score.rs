use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::{cmp::Reverse, sync::LazyLock};

use crate::result_item::ResultItem;

static MATCHER: LazyLock<SkimMatcherV2> = LazyLock::new(|| SkimMatcherV2::default());

pub fn score_item(item: &str, pattern: &str) -> Option<i64> {
    return MATCHER.fuzzy_match(item, pattern);
}

pub fn score_items(items: &Vec<&str>, pattern: &str) -> Vec<ResultItem> {
    let mut items: Vec<ResultItem> = items
        .par_iter()
        .filter_map(|i| score_item(i, pattern).map(|score| ResultItem::new(i.to_string(), score)))
        .collect();
    items.par_sort_by_key(|item| Reverse(item.score));
    return items;
}
