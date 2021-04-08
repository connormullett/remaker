use crate::types::{ParseResult, ParseStatus, Rule, Rules};

// TODO: expand and handle wildcards

fn parse_target(_input: &str) -> Vec<String> {
    vec![]
}

fn parse_rules(input: &str) -> Rules {
    let lines = input.split('\n');
    let output = Vec::new();

    for line in lines {
        parse_rule(line);
    }

    output
}

fn parse_rule(_input: &str) -> Rule {
    let _ = parse_target("");
    let _ = parse_build_steps("");
    Rule::from(vec![], vec![], vec![])
}

fn is_valid_rule_header(input: &str) -> bool {
    input.split(':').collect::<Vec<&str>>().len() == 2
}

fn parse_build_steps(input: &str) -> ParseResult<Vec<String>, ParseStatus> {
    let mut lines = input.split('\n');
    let mut build_steps = Vec::new();

    for line in &mut lines {
        if !line.starts_with('\t') {
            if is_valid_rule_header(lines.next().unwrap()) {
                return ParseResult::Err(ParseStatus::Incomplete);
            }
            return ParseResult::Ok(build_steps, ParseStatus::Complete);
        }

        build_steps.push(line.to_string())
    }

    ParseResult::Ok(build_steps, ParseStatus::Complete)
}

pub fn parse_remake_file(_remake_file_contents: &str) -> Rules {
    parse_rules("")
}
