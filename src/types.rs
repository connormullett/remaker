#[derive(Debug, Clone)]
pub struct RemakeRule<'a> {
    pub targets: Vec<&'a str>,
    pub dependencies: Vec<&'a str>,
    pub build_commands: Vec<&'a str>,
}

impl<'a> RemakeRule<'a> {
    pub fn new() -> Self {
        Self {
            targets: vec![],
            dependencies: vec![],
            build_commands: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.targets = vec![];
        self.dependencies = vec![];
        self.build_commands = vec![];
    }

    pub fn is_empty(&self) -> bool {
        if let false = self.targets.is_empty() {
            return false;
        }
        if let false = self.dependencies.is_empty() {
            return false;
        }
        if let false = self.build_commands.is_empty() {
            return false;
        }

        true
    }
}

#[derive(Debug, Clone)]
pub struct RemakeWildcard<'a> {
    pub symbol: &'a str,
    pub values: Vec<&'a str>,
}

impl<'a> RemakeWildcard<'a> {
    pub fn new() -> Self {
        Self {
            symbol: "",
            values: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.symbol = "";
        self.values = vec![];
    }
}

#[derive(Debug)]
pub struct RemakeFile<'a> {
    pub rules: Vec<RemakeRule<'a>>,
    pub wildcards: Vec<RemakeWildcard<'a>>,
}
