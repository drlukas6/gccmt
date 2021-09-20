use clap::arg_enum;
use structopt::StructOpt;

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

    #[structopt(possible_values = &CommitType::variants(), 
                case_insensitive = true,
                short = "t",
                long = "type")]
    commit_type: CommitType,

    #[structopt(case_insensitive = true,
                short = "m",
                long = "message")]
    message: String,

    #[structopt(short = "b", long = "body")]
    body: Option<String>
}

fn main() {

    let opt = Opt::from_args();

    println!("type: {}", opt.commit_type.key());
    println!("message: {}", opt.message);
    println!("body exists: {}", opt.body != None);
}
