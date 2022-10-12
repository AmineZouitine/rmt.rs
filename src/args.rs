use clap::Parser;


#[derive(Parser,Default,Debug)]
pub struct Arguments {
    pub files: Vec<String>,
    #[clap(short, long)]
    pub is_force: Option<>, // -f --force
    // pub confirmation: Confirmation,
    // pub is_recursive: Option,     // -r -R --recursive
    // #[clap(short, long)]
    // pub is_empty_dir: Option,     // -d --dir
    // #[clap(short, long)]
    // pub is_verbose: OPTION,       // -v --verbose
    // // pub is_help: Option,          // -h --help
    // #[clap(short, long)]
    // pub is_trash_display: Option, // rmt trash_display or rmt td
    // #[clap(short, long)]
    // pub is_trash_info: Option,    // rmt trash_info or rmt ti
    // #[clap(short, long)]
    // pub is_trash_flush: Option,   // rmt trash_flush or rmt tf
    // #[clap(short, long)]
    // pub is_destroy: Option,       
}