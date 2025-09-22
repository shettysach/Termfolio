pub const HELP: &str = r#"<span class="grn"> _____________  __  ___________  __   ________
/_  __/ __/ _ \/  |/  / __/ __ \/ /  /  _/ __ \
 / / / _// , _/ /|_/ / _// /_/ / /___/ // /_/ /
/_/ /___/_/|_/_/  /_/_/  \____/____/___/\____/ 
</span>
Hello, welcome to <u class="blu semibold">Termfolio</u> [WIP]. Type one of these commands -

  <span class="rd semibold">about</span> - View about me
  <span class="rd semibold">neofetch / fastfetch / github</span> - View about Github profile 
  <span class="rd semibold">onefetch / repos</span> - View about my pinned repos/projects
  <span class="rd semibold">links</span> - View contact info and links
  <span class="rd semibold">help</span> - View this help section
  <span class="rd semibold">theme / wal</span> - Cycle through themes
  <span class="rd semibold">credits</span> - View credits and repo
  <span class="rd semibold">history</span> - View command history
  <span class="rd semibold">clear</span> - Clear screen

You can use <i>arrow keys</i> to scroll through history,
and also use <i>Ctrl+L</i> to clear the screen
If you prefer a static site, visit <a href="https://shettysach.github.io" target="_blank" class="blu semibold">shettysach.github.io</a>
"#;

pub const CREDITS: &str = r#"<span class="grn"> _____________  __  ___________  __   ________
/_  __/ __/ _ \/  |/  / __/ __ \/ /  /  _/ __ \
 / / / _// , _/ /|_/ / _// /_/ / /___/ // /_/ /
/_/ /___/_/|_/_/  /_/_/  \____/____/___/\____/ 
</span>
Terminal style portfolio website. 
 
  <a href="https://github.com/shettysach" target="_blank" class="blu semibold">Github</a>: github.com/shettysach

  <a href="https://github.com/shettysach/termfolio" target="_blank" class="blu semibold">Repo</a>: github.com/shettysach/termfolio

<span class="rd semibold">APIs used -</span>

* <a 
    href="https://docs.github.com/en/rest/about-the-rest-api"
    target="_blank"
    class="blu semibold">Github REST API</a>

* <a 
    href="https://pinned.berrysauce.me"
    target="_blank" 
    class="blu semibold">Pinned repos</a> - berrysauce/pinned

* <a 
    href="https://github.com/idealclover/GitHub-Star-Counter"
    target="_blank"
    class="blu semibold">Total stars and forks</a> - idealclover/GitHub-Star-Counter

"#;

pub const READ_JSON_ERROR: &str = r#"<span class="rd semibold">Error reading config.json</span>"#;
pub const FETCH_GITHUB_ERROR: &str =
    r#"<span class="rd semibold">Error fetching data from Github.</span>"#;
