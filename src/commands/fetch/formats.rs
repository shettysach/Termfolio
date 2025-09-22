use super::{About, Links, Profile, Repository};
use std::collections::HashMap;

// Ascii art used for Github
const NEOFETCH: &str = include_str!("../../../configs/neofetch.txt");

// Language icons for repos
const RUST: &str = include_str!("../../../configs/lang_icons/rust.txt");
const PYTHON: &str = include_str!("../../../configs/lang_icons/python.txt");
const GITHUB: &str = include_str!("../../../configs/lang_icons/github.txt");

pub fn format_about(about: About) -> String {
    let exp_string: String = about
        .experience
        .iter()
        .map(|exp| {
            format!(
                r#"<span class="blu semibold">Title:</span> {}
<span class="blu semibold">Description:</span> 
{}"#,
                exp.title,
                exp.description
                    .iter()
                    .map(|s| format!(r#"<span class="blu semibold">*</span> {}"#, s))
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let edu_string: String = about
        .education
        .iter()
        .map(|edu| {
            format!(
                r#"<span class="blu semibold">Institute: </span>{}
<span class="blu semibold">Course:</span> {}
<span class="blu semibold">Duration:</span> {}
"#,
                edu.institute, edu.course, edu.duration
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let text = format!(
        r#"<center class="grn semibold">{}</center>
{}

<u class="rd semibold">Languages</u>

{}

<u class="rd semibold">Experience</u>

{}

<u class="rd semibold">Education</u>

{}
"#,
        about.name.to_uppercase(),
        about.intro,
        format_langs(about.langs),
        exp_string,
        edu_string
    );

    format!(
        r#"


<div class="row" style="display: flex; flex-direction: row; align-items: center; justify-content: center;"> 
<div class="about">{}</div>
</div>
"#,
        text
    )
}

pub fn format_profile(profile: Profile) -> String {
    let name = profile.info.name.unwrap_or(String::from("-"));
    let bio = profile.info.bio.unwrap_or(String::from("-"));
    let repos = profile.info.public_repos;
    let stars = profile.stats.stars;
    let forks = profile.stats.forks;
    let company = profile.info.company.unwrap_or(String::from("-"));
    let location = profile.info.location.unwrap_or(String::from("-"));
    let followers = profile.info.followers;
    let following = profile.info.following;
    let created_on = &profile.info.created_at[..10];

    let text = format!(
        r#"<a href="https://www.github.com/{username}" style="text-decoration:none" target="_blank"><span class="grn semibold">{username}</span><span class="grn semibold">@termfolio</span></a>
----------------------
<span class="grn semibold">Name:</span> {name}
<span class="grn semibold">Bio:</span> {bio}
<span class="grn semibold">Repos:</span> {repos}
<span class="grn semibold">Langs:</span> {langs}
<span class="grn semibold">Stars:</span> {stars}
<span class="grn semibold">Forks:</span> {forks}
<span class="grn semibold">Company:</span> {company}
<span class="grn semibold">Location:</span> {location}
<span class="grn semibold">Followers:</span> {followers}
<span class="grn semibold">Following:</span> {following}
<span class="grn semibold">Created on:</span> {created_on}

{BLOCKS}"#,
        username = profile.username,
        langs = format_langs(profile.langs),
    );

    format!(
        r#"<div class="row">
<div class="ascii">{}</div>
<div class="text">{}</div>
</div>"#,
        NEOFETCH, text
    )
}

pub fn format_repos(repos: &[Repository]) -> String {
    let res: Vec<String> = repos
        .iter()
        .map(|repo| {
            let text = format!(
                r#"<a href="https://github.com/{}/{}" target="_blank" class="blu semibold">{}</a>

<span class="rd semibold">Description:</span> {}
<span class="rd semibold">Language:</span> <span class="blu">{}</span>
<span class="rd semibold">Stars:</span> <span class="ylw">{}</span>
<span class="rd semibold">Forks:</span> <span class="ylw">{}</span>

        "#,
                repo.author,
                repo.name,
                repo.name,
                repo.description,
                repo.language,
                repo.stars,
                repo.forks
            );

            format!(
                r#"<div class="row">
<div class="ascii">{}</div>
<div class="text">{}</div>
</div>"#,
                lang_icon(&repo.language),
                text
            )
        })
        .collect();

    res.join("\n")
}

pub fn format_links(links: Links) -> String {
    let mut result = String::new();

    result += &format!(
        r#"  <a href="https://github.com/{}" target="_blank" class="semibold"  style="color:var(--purple);">Github</a>: github.com/{}
"#,
        links.github, links.github
    );

    if let Some(email) = &links.email {
        result += &format!(
            r#"
  <a href="mailto:{}" target="_blank" class="semibold" style="color:var(--orange);">Email</a>: {}
  "#,
            email, email
        );
    }

    if let Some(linkedin) = &links.linkedin {
        result += &format!(
            r#"
  <a href="https://www.linkedin.com/{}" target="_blank" class="semibold" style="color:var(--dblue);">LinkedIn</a>: linkedin.com/{}
  "#,
            linkedin, linkedin
        );
    }

    if let Some(twitter) = &links.twitter {
        result += &format!(
            r#"
  <a href="https://www.twitter.com/{}" target="_blank" class="blu semibold">Twitter/X</a>: @{}
  "#,
            twitter, twitter
        );
    }

    result
}

pub fn format_langs(langs: Vec<String>) -> String {
    let color_map: HashMap<&str, &str> = [
        ("Rust", "orange"),
        ("Python", "blue"),
        ("C", "dblue"),
        ("C++", "dblue"),
        ("Java", "red"),
        ("Haskell", "purple"),
        ("Zig", "orange"),
        ("Go", "blue"),
        ("Dart", "dblue"),
        ("JavaScript", "yellow"),
        ("TypeScript", "blue"),
        ("Bash", "dgreen"),
    ]
    .into();

    let formatted_langs: Vec<String> = langs
        .into_iter()
        .map(|lang| {
            color_map.get(lang.as_str()).map_or_else(
                || format!(r#"<span>{}</span>"#, lang),
                |color| format!(r#"<span style="color:var(--{});">{}</span>"#, color, lang),
            )
        })
        .collect();

    formatted_langs.join(" ")
}

fn lang_icon(lang: &str) -> &str {
    match lang {
        "Rust" => RUST,
        "Python" | "Jupyter Notebook" => PYTHON,
        _ => GITHUB,
    }
}

const BLOCKS: &str = r#"<span class="blocks" style="color:var(--black)">█</span><span class="rd blocks">█</span><span class="grn blocks">█</span><span class="ylw blocks">█</span><span class="blu blocks">█</span><span class="blocks" style="color:var(--orange)">█</span><span class="blocks" style="color:var(--purple)">█</span><span class="blocks">█</span>"#;
