//! Runs completion for testing purposes.

use crate::{
    completion::{completion_item::CompletionKind, CompletionConfig},
    mock_analysis::{analysis_and_position, single_file_with_position},
    CompletionItem,
};

pub(crate) fn do_completion(code: &str, kind: CompletionKind) -> Vec<CompletionItem> {
    do_completion_with_options(code, kind, &CompletionConfig::default())
}

pub(crate) fn do_completion_with_options(
    code: &str,
    kind: CompletionKind,
    options: &CompletionConfig,
) -> Vec<CompletionItem> {
    let (analysis, position) = if code.contains("//-") {
        analysis_and_position(code)
    } else {
        single_file_with_position(code)
    };
    let completion_items = analysis.completions(position, options).unwrap().unwrap();
    let mut kind_completions: Vec<CompletionItem> =
        completion_items.into_iter().filter(|c| c.completion_kind == kind).collect();
    kind_completions.sort_by_key(|c| c.label().to_owned());
    kind_completions
}
