use nom::{
    bytes::{complete::tag, streaming::take_until},
    error::{context, VerboseError},
    multi::many_till,
    sequence::{delimited, separated_pair},
    IResult,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
struct Target {
    targets: Vec<String>,
    dependencies: Vec<String>,
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

fn parse_target_line(input: &str) -> Res<&str, Target> {
    context(
        "target_line",
        separated_pair(take_until(":"), tag(":"), take_until("\n")),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

fn parse_build_commands(input: &str) -> Res<&str, (Vec<&str>, &str)> {
    context(
        "parse_build_commands",
        many_till(delimited(tag("\t"), take_until("\n"), tag("\n")), tag("\n")),
    )(input)
}

fn parse_build_command(input: &str) -> Res<&str, &str> {
    context(
        "parse_build_command",
        delimited(tag("\t"), take_until("\n"), tag("\n")),
    )(input)
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
        let build_commands = "\tgcc foo.c -o foo.o\n\techo it worked\n\n";
        let actual = parse_build_commands(build_commands);
        println!("actual {:?}", actual);

        assert!(actual.is_ok())
    }

    #[test]
    fn test_parse_build_command() {
        let build_command = "\tgcc foo.c -o foo.o\n";
        let actual = parse_build_command(build_command);

        assert_eq!(actual.unwrap(), ("", "gcc foo.c -o foo.o"));
    }
}
