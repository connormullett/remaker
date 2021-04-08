use std::path::Path;

#[derive(Debug)]
pub struct Rule {
    targets: Vec<Box<Path>>,
    dependencies: Vec<Box<Path>>,
    build_steps: Vec<String>,
}

pub type Rules = Vec<Rule>;

impl Rule {
    pub fn from(
        targets: Vec<Box<Path>>,
        dependencies: Vec<Box<Path>>,
        build_steps: Vec<String>,
    ) -> Self {
        Self {
            targets,
            dependencies,
            build_steps,
        }
    }
}
