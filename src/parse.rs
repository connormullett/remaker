#![allow(clippy::clippy::upper_case_acronyms)]

use crate::types::{RemakeFile, RemakeRule, RemakeWildcard};
use pest::Parser;

#[derive(Parser)]
#[grammar = "remake.pest"]
struct RemakeParser;

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
                current_wildcard.symbol = String::from(symbol);

                for (i, wildcard) in wildcards.clone().into_iter().enumerate() {
                    if wildcard.symbol == current_wildcard.symbol {
                        current_wildcard = wildcard;
                        wildcards.remove(i);
                        break;
                    }
                }

                for value in inner_rules {
                    current_wildcard.values.push(String::from(value.as_str()));
                }

                wildcards.push(current_wildcard.clone());
                current_wildcard.clear();
            }
            Rule::target_line => {
                if !first_rule {
                    rules.push(current_rule.clone());
                }
                current_rule.clear();
                let mut inner_rules = line.into_inner();
                let target = inner_rules.next().unwrap().as_str();
                let mut dependencies: Vec<String> = Vec::new();
                for value in inner_rules {
                    dependencies.push(value.as_str().to_string());
                }

                let is_phony = if ".PHONY" == target { true } else { false };

                current_rule = RemakeRule {
                    target: String::from(target),
                    dependencies,
                    build_commands: vec![],
                    is_phony,
                };
                first_rule = false;
            }
            Rule::build_command => current_rule.build_commands.push(line.as_str().to_string()),
            Rule::EOI => (),
            _ => (),
        }
    }

    if !current_rule.is_empty() {
        rules.push(current_rule);
    }

    RemakeFile { rules, wildcards }
}
