use std::{env, fs, io};

use serde::{Deserialize, Serialize};

use std::process::Command;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemakeRule {
    pub target: String,
    pub dependencies: Vec<String>,
    pub build_commands: Vec<String>,
    pub is_phony: bool,
}

impl RemakeRule {
    pub fn new() -> Self {
        Self {
            target: String::new(),
            dependencies: vec![],
            build_commands: vec![],
            is_phony: false,
        }
    }

    pub fn clear(&mut self) {
        self.target = String::new();
        self.dependencies = vec![];
        self.build_commands = vec![];
    }

    pub fn is_empty(&self) -> bool {
        if self.target.is_empty() && self.dependencies.is_empty() && self.build_commands.is_empty()
        {
            return true;
        }
        false
    }

    fn get_all_matches_by_pattern(&self, command: &String) -> io::Result<String> {
        let mut pattern = None;
        for part in command.split(' ') {
            if part.contains('*') {
                pattern = Some(part.replace("*", ""));
            }
        }

        if pattern.is_none() {
            crate::error_and_die(String::new());
        }

        let pattern = pattern.unwrap();

        let mut matches = Vec::new();
        for entry in fs::read_dir(env::current_dir()?)? {
            let entry = entry?;
            if entry.file_name().to_string_lossy().contains(&pattern) {
                let entry_as_string = entry.file_name().clone();
                matches.push(entry_as_string);
            }
        }

        let matches_as_string = matches
            .iter()
            .map(|value| value.to_string_lossy().to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let mut new_command = String::new();
        for part in command.split(' ') {
            if part.contains('*') {
                new_command.push_str(&matches_as_string);
            } else {
                new_command.push_str(part);
            }
            new_command.push(' ');
        }

        Ok(new_command.trim().to_string())
    }

    pub fn expand_wildcards(&mut self, wildcards: &[RemakeWildcard]) -> Self {
        let mut commands = Vec::new();

        for mut command in self.build_commands.clone() {
            for wildcard in wildcards {
                command = command.replace(
                    wildcard.symbol.as_str(),
                    wildcard.values_as_string().as_str(),
                );

                command = command.replace("$@", self.target.as_str());
                command = command.replace("$^", &self.dependencies_as_string());

                if command.contains('*') {
                    let result = self.get_all_matches_by_pattern(&command);
                    if result.is_err() {
                        crate::error_and_die(String::from("an error occurred"));
                    }
                    command = result.unwrap();
                }

                self.target = self.target.replace(
                    wildcard.symbol.as_str(),
                    wildcard.values_as_string().as_str(),
                );
            }

            commands.push(command.clone())
        }

        let mut dependencies = Vec::new();
        for mut dependency in self.dependencies.clone() {
            for wildcard in wildcards {
                dependency = dependency.replace(
                    wildcard.symbol.as_str(),
                    wildcard.values_as_string().as_str(),
                );
            }
            for split_dep in dependency.split(' ') {
                dependencies.push(split_dep.to_string());
            }
        }

        self.dependencies = dependencies;
        self.build_commands = commands;
        self.clone()
    }

    pub fn run_build_commands(&self, mut disable_output: bool) {
        for mut command in self.build_commands.clone() {
            if command.starts_with("@") {
                disable_output = true;
                command = command.replace("@", "");
            }

            if !disable_output {
                println!("{}", command);
            }

            let command: Vec<String> = command.split(" ").map(|value| value.to_string()).collect();

            let output = Command::new(&command[0]).args(&command[1..]).output();
            if let Err(error) = output {
                crate::error_and_die(error.to_string())
            }
        }
    }

    fn dependencies_as_string(&self) -> String {
        self.dependencies.join(" ")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemakeWildcard {
    pub symbol: String,
    pub values: Vec<String>,
}

impl RemakeWildcard {
    pub fn new() -> Self {
        Self {
            symbol: String::new(),
            values: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.symbol = String::new();
        self.values = vec![];
    }

    pub fn values_as_string(&self) -> String {
        self.values.join(" ")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemakeFile {
    pub rules: Vec<RemakeRule>,
    pub wildcards: Vec<RemakeWildcard>,
}

impl RemakeFile {
    pub fn handle_wildcards(&mut self) {
        let mut new_rules = Vec::new();
        for mut rule in self.rules.clone() {
            new_rules.push(rule.expand_wildcards(&self.wildcards));
        }
        self.rules = new_rules
    }

    pub fn create_new_rules_from_placeholders(&mut self) -> io::Result<()> {
        for (i, rule) in self.rules.clone().into_iter().enumerate() {
            if rule.dependencies.is_empty() {
                continue;
            }

            let dependency = &rule.dependencies[0];

            if dependency.contains('%') {
                let current_dir = env::current_dir()?;
                for entry in fs::read_dir(current_dir)? {
                    let entry = entry?;

                    if entry
                        .file_name()
                        .to_string_lossy()
                        .contains(&dependency.replace('%', ""))
                    {
                        let mut new_rule = rule.clone();
                        let new_dep = entry.file_name().clone();
                        new_rule.dependencies = vec![new_dep.to_string_lossy().into()];

                        if rule.target.contains('%') {
                            let file_match = entry.file_name().clone();
                            let file_match = file_match
                                .to_string_lossy()
                                .replace(&dependency.replace("%", ""), "");
                            let new_target = rule.target.replace("%", &file_match);

                            new_rule.target = new_target;
                        }

                        self.rules.push(new_rule);
                        self.rules.remove(i);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn handle_phony_rules(&mut self) {
        let rules = self.rules.clone();
        let phony_rules: Vec<&RemakeRule> = rules.iter().filter(|&rule| rule.is_phony).collect();

        for (i, rule) in rules.iter().enumerate() {
            for &phony in phony_rules.iter() {
                if phony.dependencies[0] == rule.target {
                    let mut new_rule = rule.clone();
                    new_rule.is_phony = true;
                    self.rules.remove(i);
                    self.rules.insert(i, new_rule);
                }
            }
        }

        self.rules = self
            .rules
            .iter()
            .filter(|&rule| rule.target != ".PHONY")
            .map(|rule| rule.to_owned())
            .collect();
    }
}
