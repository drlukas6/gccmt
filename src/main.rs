use clap::arg_enum;
use std::io::{self, Write};
use std::process::Command;
use structopt::StructOpt;

const GIT_ARG: &str = "git";
const COMMIT_ARG: &str = "commit";
const MESSAGE_ARG: &str = "-m";

arg_enum! {

    #[derive(Debug)]
    enum CommitType {
        Feature, Fix, Style, Refactor, Improvement, Docs, Internal, Bump, Revert
    }
}

impl CommitType {
    fn key(&self) -> &str {
        match self {
            CommitType::Feature => "feature",
            CommitType::Fix => "fix",
            CommitType::Style => "style",
            CommitType::Refactor => "refactor",
            CommitType::Improvement => "improvement",
            CommitType::Docs => "docs",
            CommitType::Internal => "internal",
            CommitType::Bump => "bump",
            CommitType::Revert => "revert",
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Commit options", about = "Commit type, message and body")]
struct Opt {
    /// Commit type for current changes.
    #[structopt(possible_values = &CommitType::variants(), case_insensitive = true, short = "t", long = "type")]
    commit_type: CommitType,

    /// Adds and exlamation point to commit type making note of a breaking change
    #[structopt(short, long)]
    urgent: bool,

    /// Commit message
    #[structopt(case_insensitive = true, short, long)]
    message: String,

    /// Commit body
    #[structopt(short, long)]
    body: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let output = Command::new(GIT_ARG)
        .args([COMMIT_ARG, MESSAGE_ARG, &make_commit_message(&opt)])
        .output()
        .expect("Failed to execute git commit process");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn make_commit_message(opt: &Opt) -> String {
    let urgent = if opt.urgent { "!" } else { "" };

    match &opt.body {
        None => format!("{}{}: {}", opt.commit_type.key(), urgent, opt.message),
        Some(body) => format!(
            "{}{}: {}\n\n{}",
            opt.commit_type.key(),
            urgent,
            opt.message,
            body
        ),
    }
}
