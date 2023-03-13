use clap::{ArgGroup, Parser, Subcommand, CommandFactory};
use std::path::PathBuf;

/// A Extraordinary Moe Global Project Manager
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None
)]
struct Arg { #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// add project root to known projects list
    Add {
        project: PathBuf,

        /// nickname for this project
        nick_name: Option<String>,

        /// target program language, defautl to autodetect language
        #[arg(short, long)]
        language: Option<String>,

        /// add category attribution
        #[arg(short, long)]
        category: Option<String>,

        /// add tag attribution, can be used multiple times
        // -t hello -t hi
        #[arg(short, long)]
        tags: Vec<String>,
    },

    /// Scan all subdirectories and automatically add projects
    Scan { path: PathBuf },

    /// change directory, but also check whether project existence and prompt you to add it
    // don't prompt if already prompted
    Cd { path: PathBuf },

    /// init project using project tempates
    Init {
        url: Option<String>,

        /// target program language, if language is not set, the range is all languages
        #[arg(short, long)]
        language: Option<String>,

        /// choose a template in corresponding language(or all languages if language is not set)
        #[arg(short, long)]
        template: Option<String>,

        /// don't use fzf, just list templates plainly
        #[arg(short = 'f', long)]
        no_fzf: bool,
    },

    /// Copy one or more files from a project template, when their is no filter appiled, auto detect the current(project)
    Cp {
        /// target program language, if language is not set, the range is all languages
        language: Option<String>,

        /// choose a template in corresponding language(or all languages if language is not set)
        #[arg(short, long)]
        template: Option<String>,

        /// don't use fzf, just list templates plainly
        #[arg(short = 'f', long)]
        no_fzf: bool,

        /// don't use auto-detector
        no_auto: bool,
    },

    /// List existing projects
    List {
        /// [filter] target program language, default to all languages
        #[arg(short, long)]
        language: Option<String>,

        /// [filter] certain category
        #[arg(short, long)]
        category: Option<String>,

        /// [filter] certain tags, can be used multiple times
        // -t hello -t hi
        #[arg(short, long)]
        tags: Vec<String>,

        /// list dependencies
        dependencies: bool,
    },

    /// jump into a project root in projects list, default using fzf
    Jump {
        /// [filter] target program language, default to all languages
        #[arg(short, long)]
        language: Option<String>,

        /// [filter] certain category
        #[arg(short, long)]
        category: Option<String>,

        /// [filter] certain tags, can be use multiple times
        // -t hello -t hi
        #[arg(short, long)]
        tags: Vec<String>,

        /// don't use fzf, just list templates plainly
        #[arg(short = 'f', long)]
        no_fzf: bool,
    },

    /// edit configuration or projects attribution using default editor
    #[command(group(
        ArgGroup::new("edit_file")
        .required(true)
        .args(["attribution", "configuration"]),
    ))]
    Edit {
        /// attribution file for projects
        #[arg(short)]
        attribution: bool,

        /// configuration
        #[arg(short)]
        configuration: bool,
    },
}

fn main() {
    let args = Arg::parse();
    // custom checks 
    match &args.command {
        Some(Commands::Init {url, language, template, no_fzf}) => {
            if url.is_some() && (language.is_some() || template.is_some() || no_fzf.to_owned() ) {
                let mut cmd = Arg::command();
                cmd.error(
                    clap::error::ErrorKind::ValueValidation,
                    "`url` can not be used with other arguments."
                ).exit();
            }
        },
        None => {},
        _ => {}
    }
    println!("{:#?}", args);
}
