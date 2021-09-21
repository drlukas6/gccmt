use clap::arg_enum;
use structopt::StructOpt;
use std::process::Command;
use std::io::{self, Write};

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
            CommitType::Revert => "revert"
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Commit options", about = "Commit type, message and body")]
struct Opt {

    /// Commit type for current changes.
    #[structopt(possible_values = &CommitType::variants(), 
                case_insensitive = true,
                short = "t",
                long = "type")]
    commit_type: CommitType,

    /// Adds and exlamation point to commit type making note of a breaking change
    #[structopt(short, long)]
    urgent: bool,

    /// Commit message
    #[structopt(case_insensitive = true, short, long)]
    message: String,

    /// Commit body
    #[structopt(short, long)]
    body: Option<String>
}

fn main() {

    let opt = Opt::from_args();

    let output = Command::new(GIT_ARG)
                         .arg(COMMIT_ARG)
                         .arg(MESSAGE_ARG)
                         .arg(make_commit_message(opt.commit_type, opt.message, opt.body, opt.urgent))
                         .output()
                         .expect("Failed to execute git commit process"); 

    io::stdout().write_all(&output.stdout).unwrap(); 
    io::stderr().write_all(&output.stderr).unwrap(); 
}

fn make_commit_message(commit_type: CommitType, message: String, body: Option<String>, urgent: bool) -> String {

    let urgent = if urgent { "!" } else { "" };

    match body {
        None => format!("{}{}: {}", urgent, commit_type.key(), message),
        Some(body) => format!("{}{}: {}\n\n{}", urgent, commit_type.key(), message, body)
    }
}