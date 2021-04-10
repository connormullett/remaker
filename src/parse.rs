use nom::{
    branch::permutation,
    bytes::{
        complete::{tag, take_till},
        streaming::take_until,
    },
    combinator::opt,
    error::{context, VerboseError},
    multi::{many0, separated_list1},
    sequence::separated_pair,
    IResult,
};

use crate::types::{RemakeFile, Rule, Target, VariableAssignment};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn parse_variable_assignment(input: &str) -> Res<&str, VariableAssignment> {
    context(
        "parse_variable_assignment",
        separated_pair(take_until("="), tag("="), take_till(|c| c == '\n')),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            VariableAssignment {
                symbol: res.0.trim().to_string(),
                value: res.1.trim().to_string(),
            },
        )
    })
}

fn parse_target_line(input: &str) -> Res<&str, Target> {
    context(
        "target_line",
        separated_pair(take_until(":"), tag(":"), take_until("\n")),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

fn parse_build_commands(input: &str) -> Res<&str, Vec<&str>> {
    let (next_input, output) = context(
        "parse_build_commands",
        separated_list1(tag("\n\t"), take_till(|c| c == '\n')),
    )(input)?;

    Ok((
        next_input,
        output
            .into_iter()
            .filter(|&item| !item.is_empty())
            .collect(),
    ))
}

fn parse_rule(input: &str) -> Res<&str, Rule> {
    context(
        "parse_rule",
        permutation((parse_target_line, parse_build_commands)),
    )(input)
    .map(|(next_input, res)| {
        (
            next_input,
            Rule {
                targets: res
                    .0
                    .targets
                    .iter()
                    .map(|i| Box::new(i.to_string()))
                    .collect(),
                dependencies: res
                    .0
                    .dependencies
                    .iter()
                    .map(|i| Box::new(i.to_string()))
                    .collect(),
                build_steps: res.1.iter().map(|&i| i.to_string()).collect(),
            },
        )
    })
}

pub fn parse_remake_file(input: &str) -> RemakeFile {
    let mut remake_file = RemakeFile {
        rules: vec![],
        variables: vec![],
    };
    let _ = context(
        "parse_remake_file",
        many0(permutation((
            opt(parse_variable_assignment),
            opt(parse_rule),
        ))),
    )(input)
    .map(|(_, tuple_vec)| {
        tuple_vec
            .iter()
            .map(|(variable, rule)| {
                if let Some(value) = variable {
                    remake_file.variables.push(value.clone());
                }
                if let Some(value) = rule {
                    remake_file.rules.push(value.clone());
                }
            })
            .collect::<()>()
    });

    remake_file
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_target_line() {
        let target_line = "foo.c: foo.o\n";
        let actual = parse_target_line(target_line);

        assert_eq!(
            Target {
                targets: vec!["foo.c".to_string()],
                dependencies: vec!["foo.o".to_string()]
            },
            actual.unwrap().1
        );
    }

    #[test]
    fn test_parse_build_commands() {
        let build_commands = "\n\tgcc foo.c -o foo.o\n\techo it worked";
        let actual = parse_build_commands(build_commands);

        assert!(actual.is_ok())
    }

    #[test]
    fn test_parse_variable_assignment() {
        let input = "foo = value";
        let actual = parse_variable_assignment(input);

        assert!(actual.is_ok());
    }

    #[test]
    fn test_parse_rule() {
        let input = "foo.c: foo.o\n\tgcc -o foo.c -c\n\techo it works";
        let actual = parse_rule(input);

        assert!(actual.is_ok());
    }

    #[test]
    fn test_parse_remake_file() {
        let input = include_str!("../remaker");
        let actual = parse_remake_file(input);

        println!("file {:?}", actual);
    }
}
