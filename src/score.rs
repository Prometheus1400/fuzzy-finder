use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use std::{collections::BinaryHeap, sync::LazyLock};

use crate::result_item::ResultItem;

static MATCHER: LazyLock<SkimMatcherV2> = LazyLock::new(|| SkimMatcherV2::default());

pub fn score_item(item: &str, pattern: &str) -> Option<i64> {
    return MATCHER.fuzzy_match(item, pattern);
}

pub fn score_items(items: &Vec<String>, pattern: &str) -> BinaryHeap<ResultItem> {
    let mut heap = BinaryHeap::new();
    for item in items {
        if let Some(score) = score_item(item, pattern) {
            heap.push(ResultItem::new(item.clone(), score));
        }
    }
    return heap;
}
