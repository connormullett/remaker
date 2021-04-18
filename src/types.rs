use std::{env, ffi::CString, fs};

use nix::{
    sys::wait::{waitpid, WaitStatus},
    unistd::{execvp, fork, ForkResult},
};

#[derive(Debug, Clone)]
pub struct RemakeRule {
    pub target: String,
    pub dependencies: Vec<String>,
    pub build_commands: Vec<String>,
}

impl RemakeRule {
    pub fn new() -> Self {
        Self {
            target: String::new(),
            dependencies: vec![],
            build_commands: vec![],
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

    pub fn run_build_commands(&self) {
        for command in self.build_commands.clone() {
            println!("{}", command);
            let fork_result = unsafe { fork() };

            if let Ok(ForkResult::Child) = fork_result {
                let args: Vec<CString> = command
                    .split(' ')
                    .map(|item| CString::new(item.as_bytes()).unwrap())
                    .collect();
                let _ = execvp(&args[0], &args);
            }

            if let Ok(ForkResult::Parent { child, .. }) = fork_result {
                loop {
                    match waitpid(child, None) {
                        Ok(WaitStatus::Exited(_, _)) => break,
                        Ok(WaitStatus::Signaled(_, _, _)) => break,
                        _ => {}
                    };
                }
            };
        }
    }

    fn dependencies_as_string(&self) -> String {
        self.dependencies.join(" ")
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
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

    pub fn create_new_rules_from_placeholders(&mut self) {
        let mut i = 0;
        for rule in &self.rules.clone() {
            if rule.dependencies.is_empty() {
                continue;
            }

            let dependency = &rule.dependencies[0];

            if dependency.contains('%') {
                for entry in fs::read_dir(env::current_dir().unwrap()).unwrap() {
                    let entry = entry.unwrap();

                    if entry
                        .file_name()
                        .to_string_lossy()
                        .contains(&dependency.replace('%', ""))
                    {
                        let mut new_rule = rule.clone();
                        let new_dep = entry.file_name().clone();
                        new_rule.dependencies = vec![new_dep.to_string_lossy().into()];

                        if rule.target.contains('%') {
                            // foo.o -> foo -> foo.c
                            // match = foo.o
                            // dependency = %.o
                            // target = %.c
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
            i += 1;
        }
    }
}
