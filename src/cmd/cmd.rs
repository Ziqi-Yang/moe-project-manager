use std::path::PathBuf;
use anyhow::Result;
use clap::{ArgGroup, Parser, Subcommand, CommandFactory};

const ENV_HELP: &str = "Environment variables:
  _MPM_DATA_DIR          Path for MPM data files";

/// A Extraordinary Moe Global Project Manager
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    after_help = ENV_HELP,
    about,
    long_about = None
)]
pub struct Arg { #[command(subcommand)]
    pub command: Option<Commands>,

    /// don't use fzf
    #[arg(short = 'f', long)]
    no_fzf: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Add(Add),
    Scan(Scan),
    Cd(Cd),
    Init(Init),
    Cp(Cp),
    List(List),
    Jump(Jump),
    Edit(Edit)
}

/// add project root to known projects list
#[derive(Debug, Parser)]
pub struct Add {
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
}


/// Scan all subdirectories and automatically add projects
#[derive(Debug, Parser)]
pub struct Scan {
    /// root path for scan
    path: PathBuf
}

/// change directory while executing automatical project adding function(prompt, directly add, do nothing)
#[derive(Debug, Parser)]
pub struct Cd {
    path: PathBuf
}

/// init project using project tempates
#[derive(Debug, Parser)]
pub struct Init {
    url: Option<String>,

    /// [filter] target program language
    #[arg(short, long, default_value="all")]
    language: Option<String>,

    /// [filter] certain category
    #[arg(short, long)]
    category: Option<String>,

    /// [filter] certain tags, can be used multiple times
    // -t hello -t hi
    #[arg(short, long)]
    tags: Vec<String>,

    /// [filter] template name prefix, if the name exactly matches, then directly use it
    #[arg(short, long)]
    template: Option<String>,
}

/// Copy one or more files from a project template, when their is no filter appiled, auto detect the current(project)
#[derive(Debug, Parser)]
pub struct Cp {
    /// target program language
    #[arg(short, long, default_value="all")]
    language: Option<String>,

    /// [filter] certain category
    #[arg(short, long)]
    category: Option<String>,

    /// [filter] certain tags, can be used multiple times
    // -t hello -t hi
    #[arg(short, long)]
    tags: Vec<String>,

    /// [filter] fuzzy template name, if the name exactly matches, then directly use it
    #[arg(short, long)]
    template: Option<String>,

    /// don't use auto-detector
    no_auto: bool,
}

/// List existing projects
#[derive(Debug, Parser)]
pub struct List {
    /// fuzzy project name
    p_name: Option<String>,
    
    /// [filter] target program language, default to all languages
    #[arg(short, long, default_value="all")]
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
}

/// jump into a project root in projects list, default using fzf
#[derive(Debug, Parser)]
pub struct Jump {
    /// fuzzy project name
    p_name: Option<String>,

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
}

/// edit configuration or projects attribution using default editor
#[derive(Debug, Parser)]
#[command(group(
    ArgGroup::new("edit_file")
    .required(true)
    .args(["attribution", "configuration"]),
))]
pub struct Edit {
    /// attribution file for projects
    #[arg(short)]
    attribution: bool,

    /// configuration
    #[arg(short)]
    configuration: bool,
}


impl Arg {
    pub fn check_arg(&self) -> Result<()> {
        match &self.command {
            Some(Commands::Init(Init{url, language, category, tags, template})) => {
                if url.is_some() && (language.is_some() || category.is_some() || !tags.is_empty() || template.is_some() ) {
                    let mut cmd = Arg::command();
                    cmd.error(
                        clap::error::ErrorKind::ValueValidation,
                        "`url` can not be used with other arguments."
                    ).exit();
                } else {
                    Ok(())
                }
            },
            None => Ok(()),
            _ => Ok(())
        }
    }
}
