use crate::types::{ParseResult, ParseStatus, Rule, Rules};

// TODO: expand and handle wildcards

fn parse_target(input: &str) -> Vec<String> {
    vec![]
}

fn parse_rules(input: &str) -> Rules {
    let lines = input.split('\n');
    let mut output = Vec::new();

    for line in lines {}

    output
}

fn is_valid_rule_header(input: &str) -> bool {
    input.split(':').collect::<Vec<&str>>().len() == 2
}

fn parse_build_steps(input: &str) -> ParseResult<Vec<String>, ParseStatus> {
    let lines = input.split('\n');
    let mut build_steps = Vec::new();

    for line in lines {
        if !line.starts_with('\t') {
            // todo: check if next line is a valid target
            return ParseResult::Ok(build_steps, ParseStatus::Complete);
        }

        build_steps.push(line.to_string())
    }

    ParseResult::Ok(build_steps, ParseStatus::Complete)
}

fn parse_rule(_input: &str) -> Rule {
    Rule::from(vec![], vec![], vec![])
}

pub fn parse_remake_file(_remake_file_contents: &str) -> Rules {
    parse_rules("");
    Rules::new()
}
