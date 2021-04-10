pub type Rules = Vec<Rule>;
#[derive(Debug, PartialEq, Eq)]
pub struct Target {
    pub targets: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct Rule {
    pub targets: Vec<Box<String>>,
    pub dependencies: Vec<Box<String>>,
    pub build_steps: Vec<String>,
}
