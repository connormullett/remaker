use pest::Parser;
#[derive(Parser)]
#[grammar = "remake.pest"]
struct RemakeParser;
use crate::types::{RemakeFile, RemakeRule, RemakeWildcard};

pub fn parse(remake_file_contents: &str) -> RemakeFile {
    let file = RemakeParser::parse(Rule::remake_file, &&remake_file_contents)
        .expect("bad parse")
        .next()
        .unwrap();

    let mut wildcards = Vec::<RemakeWildcard>::new();

    let mut rules = Vec::<RemakeRule>::new();
    let mut current_rule = RemakeRule::new();
    let mut first_rule = true;

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::wildcard => {
                let mut inner_rules = line.into_inner();
                let symbol: &str = inner_rules.next().unwrap().as_str();
                let mut current_wildcard = RemakeWildcard::new();
                current_wildcard.symbol = symbol;

                while let Some(value) = inner_rules.next() {
                    current_wildcard.values.push(value.as_str());
                }
                wildcards.push(current_wildcard.clone());
                current_wildcard.clear();
            }
            Rule::target_line => {
                if !first_rule {
                    // if it is not the first rule, push the rule
                    rules.push(current_rule.clone());
                }
                current_rule.clear();
                let mut inner_rules = line.into_inner();
                let target = inner_rules.next().unwrap().as_str();
                let dependencies = inner_rules.next().unwrap().as_str();
                current_rule = RemakeRule {
                    target,
                    dependencies: vec![dependencies],
                    build_commands: vec![],
                };
                first_rule = false;
            }
            Rule::build_command => current_rule.build_commands.push(line.as_str()),
            Rule::EOI => (),
            _ => (),
        }
    }

    if !current_rule.is_empty() {
        rules.push(current_rule);
    }

    RemakeFile {
        rules,
        wildcards: wildcards.clone(),
    }
}
