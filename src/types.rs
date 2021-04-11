use std::{env, ffi::CString, path::PathBuf};

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

    pub fn target_as_path(&self) -> PathBuf {
        let mut current_directory = env::current_dir().unwrap();
        current_directory.push(self.target.clone());
        current_directory
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

        for mut command in self.build_commands.clone().into_iter() {
            for wildcard in wildcards {
                command = command.replace(
                    wildcard.symbol.as_str(),
                    wildcard.values_as_string().as_str(),
                );
            }
            commands.push(command.clone())
        }
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
                if let Err(e) = execvp(&args[0], &args) {
                    println!("error during command '{}': {}", command, e);
                    panic!();
                }
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
}
