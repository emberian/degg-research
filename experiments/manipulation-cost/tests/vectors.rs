//! The corpus is byte-stable and its own header states its ceiling.

use degg_manipulation_cost::MODEL;
use degg_manipulation_cost::table::{grid_rows, render_row, render_vectors_v1};

const CORPUS: &str = include_str!("../vectors/v1.txt");

#[test]
fn the_rendered_corpus_matches_the_checked_in_file_byte_for_byte() {
    assert_eq!(render_vectors_v1().expect("render"), CORPUS);
}

#[test]
fn rendering_is_idempotent_across_runs() {
    assert_eq!(
        render_vectors_v1().expect("render"),
        render_vectors_v1().expect("render")
    );
}

#[test]
fn the_header_states_the_model_the_recovery_rule_and_the_bound() {
    let mut lines = CORPUS.lines();
    assert_eq!(lines.next(), Some("manipulation-cost-vectors-v1"));
    assert_eq!(lines.next(), Some(&format!("model={MODEL}")[..]));
    assert!(CORPUS.contains("recovery=self-reversal-into-the-same-pool"));
    assert!(CORPUS.contains("bound=lower-bound-under-stated-assumptions"));
    assert!(CORPUS.contains("data=synthetic-only"));
}

#[test]
fn every_row_renders_on_one_line_with_the_declared_columns() {
    let column_line = CORPUS
        .lines()
        .find(|line| line.starts_with("columns="))
        .expect("column header");
    let columns = column_line
        .trim_start_matches("columns=")
        .split(',')
        .count();
    for row in grid_rows().expect("grid") {
        let rendered = render_row(&row);
        assert!(!rendered.contains('\n'));
        let fields: Vec<&str> = rendered.split('|').collect();
        assert_eq!(fields.len(), columns);
        for field in fields {
            assert!(field.contains('='), "field without a key: {field}");
        }
    }
}

#[test]
fn the_corpus_carries_one_line_per_row_plus_its_header() {
    let header_lines = 6;
    assert_eq!(
        CORPUS.lines().count(),
        header_lines + grid_rows().expect("grid").len()
    );
}

#[test]
fn the_row_quoted_in_the_memo_addendum_is_the_row_the_table_holds() {
    // The 2026-08-18 addendum to
    // docs/regulatory/research-memos/definitions-q15-reference-integrity.md
    // quotes these five figures in prose. If the model changes, this fails
    // before the memo goes stale.
    let row = grid_rows()
        .expect("grid")
        .into_iter()
        .find(|row| {
            row.depth == 1_000_000_000
                && row.fee_bps == 30
                && row.window_seconds == 3600
                && row.buckets == 12
                && row.hold == 1
                && row.boundary_bps == 10
        })
        .expect("the quoted row");
    assert_eq!(row.bucket_bps, 120);
    assert_eq!(row.capital, 5_991_095);
    assert_eq!(row.contiguous, 35_681);
    assert_eq!(row.naive_spot, 2_998);
    assert_eq!(row.ceiling, 6_027_108);
    // "a factor of one hundred and sixty-eight" and "eleven point nine".
    assert_eq!(row.capital / row.contiguous, 167);
    assert_eq!(row.capital * 10 / row.contiguous, 1_679);
    assert_eq!(row.contiguous * 10 / row.naive_spot, 119);
}
