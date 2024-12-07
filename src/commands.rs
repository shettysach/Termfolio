mod fetch;
mod texts;
pub use fetch::get_prompt;

pub async fn command(inp0: &str, inp1: &str) -> String {
    let result = match inp0 {
        "help" | "termfolio" => texts::HELP,
        "about" => &fetch::get_about(),
        "github" | "neofetch" | "fastfetch" => &fetch::get_github().await,
        "repos" | "onefetch" => &fetch::get_repos().await,
        "links" => fetch::get_contacts(),
        "credits" => texts::CREDITS,

        "cd" => "Nowhere to go.",
        "mkdir" | "touch" => "Nowhere to create.",
        "rm" | "rmdir" => "Nothing to destroy.",
        "cp" => "Nothing to duplicate.",
        "mv" => "Nowhere to move.",
        "ls" | "cat" => "Nothing to see.",
        "grep" | "which" | "find" => "Nowhere to search.",
        "pwd" => "You are here.",
        "nano" | "vi" | "vim" | "nvim" | "hx" => "Great editor.",
        "emacs" => "Great mail client",
        "su" | "sudo" | "chmod" => "With great power comes great responsibility.",
        "whoami" => "Despite everything, it's still you.",
        "exit" => "Hasta la vista.",
        "echo" => inp1.trim(),
        "" => "",
        _ => &format!("{inp0}: command not found"),
    };

    result.to_string()
}

pub fn autocomplete(inp: &str) -> &str {
    let inp = inp.trim();

    let comms = [
        "help",
        "history",
        "about",
        "github",
        "repos",
        "links",
        "theme",
        "wal",
        "credits",
        "onefetch",
        "neofetch",
        "fastfetch",
    ];

    if !inp.is_empty() {
        for &c in comms.iter() {
            if c.starts_with(inp) {
                return c;
            }
        }
    }

    inp
}

pub fn banner() -> String {
    String::from(texts::HELP)
}
