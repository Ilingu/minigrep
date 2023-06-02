use colored::{Color, Colorize};
use std::{env::Args, fs, process};

pub fn fatal(msg: &str) -> ! {
    eprintln!("{}", msg.red().bold());
    process::exit(1)
}

pub trait ColorJoiner {
    fn color_join(&self, join: &str, color_join: Color, color_other: Color) -> String;
}

impl ColorJoiner for Vec<&str> {
    fn color_join(&self, join: &str, color_join: Color, color_other: Color) -> String {
        let mut joined = String::new();
        for (i, sp) in self.iter().enumerate() {
            match i == self.len() - 1 {
                true => joined.push_str(format!("{}", sp.color(color_other)).as_str()),
                false => joined.push_str(
                    format!("{}{}", sp.color(color_other), join.color(color_join).bold()).as_str(),
                ),
            }
        }
        joined
    }
}

pub struct Grepper {
    // filepath: String,
    pattern: String,
    file_content: String,
}

impl Grepper {
    pub fn from_args(args: Args) -> Self {
        let mut user_args = args.skip(1);
        let filepath = user_args
            .next()
            .unwrap_or_else(|| fatal("where is my file?"));
        let pattern = user_args
            .next()
            .unwrap_or_else(|| fatal("where is my pattern?"));
        let case_sensitive = match user_args.next() {
            Some(b) => b.parse().unwrap_or(true),
            None => true,
        };
        let file_content =
            fs::read_to_string(filepath).unwrap_or_else(|_| fatal("cannot read file"));

        Self {
            // filepath,
            pattern: if case_sensitive {
                pattern
            } else {
                pattern.to_lowercase()
            },
            file_content: if case_sensitive {
                file_content
            } else {
                file_content.to_lowercase()
            },
        }
    }

    pub fn grep(&self) -> Vec<String> {
        self.file_content
            .lines()
            .enumerate()
            .filter(|(_, line)| line.contains(&self.pattern))
            .map(|(ln, line)| {
                (
                    ln,
                    line.split(&self.pattern).collect::<Vec<_>>().color_join(
                        &self.pattern,
                        Color::Red,
                        Color::BrightWhite,
                    ),
                )
            })
            .map(|(ln, l)| format!("{} {l}", format!("{}|", ln + 1).blue()))
            .collect()
    }
}
