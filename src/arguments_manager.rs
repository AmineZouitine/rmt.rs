pub struct ArgumentsManager {
    pub is_force: bool, // -f --force
    pub confirmation: Confirmation,
    pub is_recursive: bool,     // -r -R --recursive
    pub is_empty_dir: bool,     // -d --dir
    pub is_verbose: bool,       // -v --verbose
    pub is_help: bool,          // -h --help
    pub is_trash_display: bool, // rmt trash_display or rmt td
    pub is_trash_info: bool,    // rmt trash_info or rmt ti
    pub is_trash_flush: bool,   // rmt trash_flush or rmt tf
    pub is_destroy: bool,       // rmt --destroy
}

impl ArgumentsManager {
    pub fn new(arguments: &[String]) -> Self {
        Self {
            is_force: arguments.contains(&String::from("-f"))
                || arguments.contains(&String::from("--force")),
            confirmation: Confirmation::new(arguments),
            is_recursive: arguments.contains(&String::from("-r"))
                || arguments.contains(&String::from("--recursive"))
                || arguments.contains(&String::from("-R")),
            is_empty_dir: arguments.contains(&String::from("-d"))
                || arguments.contains(&String::from("--dir")),
            is_verbose: arguments.contains(&String::from("-v"))
                || arguments.contains(&String::from("--verbose")),
            is_help: arguments.contains(&String::from("--h"))
                || arguments.contains(&String::from("--help")),
            is_trash_display: arguments.contains(&String::from("trash_display"))
                || arguments.contains(&String::from("td")),
            is_trash_info: arguments.contains(&String::from("trash_info"))
                || arguments.contains(&String::from("ti")),
            is_trash_flush: arguments.contains(&String::from("trash_flush"))
                || arguments.contains(&String::from("tf")),
            is_destroy: arguments.contains(&String::from("--destroy")),
        }
    }

    pub fn filter_args(arguments: &mut [String]) {
        arguments.to_vec().retain(|arg| {
            arg != "-f"
                && arg != "--force"
                && arg != "-i"
                && arg != "--interactive[always]"
                && arg != "-I"
                && arg != "interactive[once]"
                && arg != "interactive[never]"
                && arg != "-r"
                && arg != "--recursive"
                && arg != "-R"
                && arg != "-d"
                && arg != "--dir"
                && arg != "-v"
                && arg != "--verbose"
                && arg != "--h"
                && arg != "--help"
                && arg != "trash_display"
                && arg != "td"
                && arg != "trash_info"
                && arg != "ti"
                && arg != "trash_flush"
                && arg != "tf"
                && arg != "--destroy"
        });
    }
}

pub enum Confirmation {
    Never,  // default
    Always, // -I
    Once,   // -I
}

impl Confirmation {
    pub fn new(arguments: &[String]) -> Self {
        if arguments.contains(&String::from("-i"))
            || arguments.contains(&String::from("--interactive[always]"))
        {
            return Self::Always;
        } else if arguments.contains(&String::from("-I"))
            || arguments.contains(&String::from("interactive[once]"))
        {
            return Self::Once;
        }
        Self::Never
    }
}
