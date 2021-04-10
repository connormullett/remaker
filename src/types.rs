pub type Rules = Vec<Rule>;

pub type Variables = Vec<VariableAssignment>;

#[derive(Debug)]
pub struct RemakeFile {
    pub rules: Rules,
    pub variables: Variables,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Target {
    pub targets: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VariableAssignment {
    pub symbol: String,
    pub value: String,
}

impl From<(&str, &str)> for Target {
    fn from(i: (&str, &str)) -> Self {
        Self {
            targets: i.0.split(' ').map(|target| target.to_string()).collect(),
            dependencies: i
                .1
                .trim()
                .split(' ')
                .map(|target| target.to_string())
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub targets: Vec<Box<String>>,
    pub dependencies: Vec<Box<String>>,
    pub build_steps: Vec<String>,
}
