use crate::types::{Rule, Rules};

fn parse_rule(_input: String) -> Rule {
    Rule::from(vec![], vec![], vec![])
}

pub fn parse_remake_file(_remake_file_contents: String) -> Rules {
    parse_rule(String::new());
    Rules::new()
}
