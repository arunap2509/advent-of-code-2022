use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum Files<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

#[derive(Debug)]
struct File<'a> {
    size: u32,
    name: &'a str,
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("qwertyuiopasdfghjklzcxvbnm."),
    )(input)?;
    Ok((input, Files::File { size, name }))
}

fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir(name)))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;
    Ok((input, Operation::Ls(files)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match dir {
        ".." => Cd::Up,
        "/" => Cd::Root,
        name => Cd::Down(name),
    };
    Ok((input, Operation::Cd(op)))
}

fn command(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmd) = separated_list1(newline, alt((cd, ls)))(input)?;
    Ok((input, cmd))
}

pub fn process_input(input: &str) -> String {
    let cmds = command(input).unwrap().1;
    let mut directories = BTreeMap::<String, Vec<File>>::new();
    let mut context: Vec<&str> = vec![];

    for command in cmds.iter() {
        match command {
            Operation::Cd(Cd::Root) => {
                context.push("");
            }
            Operation::Cd(Cd::Up) => {
                context.pop();
            }
            Operation::Cd(Cd::Down(name)) => context.push(&name),
            Operation::Ls(files) => {
                directories
                    .entry(
                        context
                            .iter()
                            .cloned()
                            .flat_map(|c| [c, "/"])
                            .collect::<String>(),
                    )
                    .or_insert(vec![]);
                for file in files.iter() {
                    match file {
                        Files::File { size, name } => {
                            directories
                                .entry(
                                    context
                                        .iter()
                                        .cloned()
                                        .flat_map(|f| [f, "/"])
                                        .collect::<String>(),
                                )
                                .and_modify(|vec| {
                                    vec.push(File { size: *size, name });
                                });
                        }
                        Files::Dir(_) => (),
                    }
                }
            }
        }
    }

    println!("{:?}", directories);

    let mut sizes = BTreeMap::<String, u32>::new();
    for (path, files) in directories.iter() {
        let dirs = path.split("/").collect::<Vec<&str>>();
        let size = files.iter().map(|File { size, .. }| size).sum::<u32>();

        for i in 0..dirs.len() {
            sizes
                .entry(
                    (dirs[0..=i])
                        .iter()
                        .cloned()
                        .flat_map(|f| [f, "/"])
                        .collect::<String>(),
                )
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }

    println!("{:?}", sizes);

    sizes
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, size)| size)
        .sum::<u32>()
        .to_string()
}
