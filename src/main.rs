use clap::arg_enum;
use structopt::StructOpt;
use std::process::Command;
use std::io::{self, Write};

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

    println!("type: {}", opt.commit_type.key());
    println!("message: {}", opt.message);
    println!("body exists: {}", opt.body != None);
    println!("urgent: {}", opt.urgent);

    println!("---------");

    let commit_message: String;

    if let Some(body) = opt.body {

        commit_message = format!("{}: {}\n\n{}",opt.commit_type.key(), opt.message, body);
    } else {
        commit_message = format!("{}: {}", opt.commit_type.key(), opt.message);
    }

    println!("{}", commit_message);

    let output = Command::new("git")
                         .arg("commit")
                         .arg("-m")
                         .arg(commit_message)
                         .output()
                         .expect("Failed to execute git commit process"); 

    io::stdout().write_all(&output.stdout).unwrap(); 
    io::stderr().write_all(&output.stderr).unwrap(); 
}
