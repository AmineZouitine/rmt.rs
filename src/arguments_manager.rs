use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author = "Amine Zouitine", version, about)]
pub struct ArgumentsManager {
    pub elements: Vec<String>,

    /// remove the element without placing it in the trash
    #[arg(long = "destroy")]
    pub is_destroy: bool,

    /// ignore nonexistent files and arguments, never prompt
    #[arg(short = 'f', long = "force")]
    pub is_force: bool,

    /// prompt before every removal
    #[arg(short = 'i')]
    pub confirmation_always: bool,

    ///  prompt once before removing more than three files, or  when removing  recursively;
    #[arg(short = 'I')]
    pub confirmation_once: bool,

    /// remove directories and their contents recursively
    #[arg(short = 'r', long = "recursive")]
    pub is_recursive: bool, // -r -R --recursive

    /// remove empty directories
    #[arg(short = 'd', long = "dir")]
    pub is_empty_dir: bool, // -d --dir

    /// Explain what is being done
    #[arg(short = 'v', long = "verbose")]
    pub is_verbose: bool,

    /// Open trash manager CLI, use to restore or delete element from the trash
    #[arg(long = "td")]
    pub is_trash_display: bool, // rmt trash_display or rmt td

    /// Show some informations about the trash
    #[arg(long = "ti")]
    pub is_trash_info: bool, // rmt trash_info or rmt ti

    /// Flush all the elements present in the trash
    #[arg(long = "tf")]
    pub is_trash_flush: bool, // rmt trash_flush or rmt tf
}
