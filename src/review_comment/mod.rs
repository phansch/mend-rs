use crate::diff::Diff;
use crate::lint_report::LintReport;

/// Represents a review comment on GitHub
#[derive(PartialEq, Debug)]
pub struct ReviewComment {
    pub body: String,
    pub path: String,
    pub position: usize,
}

impl ReviewComment {
    pub fn new(body: &str, path: &str, position: usize) -> ReviewComment {
        ReviewComment {
            body: body.to_string(),
            path: path.to_string(),
            position,
        }
    }

    /// Determines what comments to create from a LintReport
    ///
    /// This is the core of the whole project. Given the different
    /// lint results and a diff from a Pull Request, this function
    ///
    /// a) Determines what lint reports are relevant for the diff
    /// b) Determines the diff-relative position to send to GitHub
    pub fn from_lint_report(lint_report: &[LintReport], diff: &Diff) -> Vec<ReviewComment> {
        let mut result = vec![];
        // TODO: #8 also created files (test first)
        // NOTE: this is not an efficient implementation, probably
        for report in lint_report {
            if let Some(file) = diff
                .patchset
                .modified_files()
                .iter()
                .find(|f| f.target_file.ends_with(&report.spans[0].file_name))
            {
                for hunk in file.clone() {
                    // hunk line index has to start at 1
                    // TODO: possible to split this off?
                    let mut line_index = 1;
                    for line in hunk {
                        if let Some(target_line_no) = line.target_line_no {
                            if target_line_no == report.spans[0].line_start {
                                result.push(ReviewComment::new(
                                    &report.message,
                                    &report.spans[0].file_name,
                                    line_index,
                                ))
                            }
                        }
                        line_index += 1;
                    }
                }
            }
        }
        result
    }
}
