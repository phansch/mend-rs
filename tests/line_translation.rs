use mend_rs::diff::*;
use mend_rs::lint_report::*;
use mend_rs::review_comment::*;

// Given a PR diff and a Clippy lint result
// All these tests make sure that the correct
// ReviewComment is created

#[test]
fn test_simple() {
    let lint_report = vec![LintReport::new(
        "this should not be hardcoded",
        52,
        "src/main.rs",
    )];

    let patchset = Diff::patchset_from_str(include_str!("auxiliary/simple.diff"));
    let diff = Diff {
        url: "abc".to_string(),
        patchset: patchset,
    };

    let review_comments = ReviewComment::from_lint_report(&lint_report, &diff);

    let expected = vec![ReviewComment::new(
        "this should not be hardcoded",
        "src/main.rs",
        10,
    )];

    assert_eq!(expected, review_comments);
}

// TODO: test with only created file
