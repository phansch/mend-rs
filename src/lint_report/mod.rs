use serde_derive::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct LintCode {
    pub code: String,
    pub explanation: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct LintSpan {
    pub file_name: String,
    /// The line where the lint should be reported
    ///
    /// GitHub provides a line_start and a line_end.
    /// We should use the line_start in case of multi-line lints.
    /// (Why?)
    pub line_start: usize,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct LintReport {
    /// The lint message
    ///
    /// Example:
    ///
    /// unused variable: `count`
    pub message: String,
    pub spans: Vec<LintSpan>,
}

impl LintReport {
    pub fn new(message: &str, line: usize, file_name: &str) -> Self {
        LintReport {
            message: message.to_string(),
            spans: vec![LintSpan {
                line_start: line,
                file_name: file_name.to_string(),
            }],
        }
    }
}

pub fn parse_json(input: &str) -> Vec<LintReport> {
    input
        .lines()
        .filter(|l| l.starts_with('{'))
        .map(|line| serde_json::from_str(line).unwrap())
        .collect::<Vec<LintReport>>()
}

#[test]
fn test_translate_json() {
    let input = r##"
    Checking mend-rs v0.1.0 (file:///home/phansch/code/mend-rs)
{"message":"unused variable: `pulls`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"src/main.rs","byte_start":492,"byte_end":497,"line_start":19,"line_end":19,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let pulls = repo.pulls();","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_pulls` instead","code":null,"level":"help","spans":[{"file_name":"src/main.rs","byte_start":492,"byte_end":497,"line_start":19,"line_end":19,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let pulls = repo.pulls();","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":"_pulls","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `pulls`\n  --> src/main.rs:19:9\n   |\n19 |     let pulls = repo.pulls();\n   |         ^^^^^ help: consider using `_pulls` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
{"message":"function is never used: `translate_json`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"src/lib.rs","byte_start":1235,"byte_end":1284,"line_start":50,"line_end":50,"column_start":1,"column_end":50,"is_primary":true,"text":[{"text":"fn translate_json(input: &str) -> Vec<LintReport> {","highlight_start":1,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: function is never used: `translate_json`\n  --> src/lib.rs:50:1\n   |\n50 | fn translate_json(input: &str) -> Vec<LintReport> {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
    "##;

    let expected = vec![
        LintReport::new("unused variable: `pulls`", 19, "src/main.rs"),
        LintReport::new("function is never used: `translate_json`", 50, "src/lib.rs"),
    ];

    assert_eq!(expected, parse_json(input));
}
