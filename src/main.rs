///! 📝📝📝📝📝📝
use serde_derive::*;
use std::process::{Command, Stdio};
use std::borrow::Cow;

type Error = Box<std::error::Error>;

// From the gitmojis repo
const GITMOJIS_JSON: &str = include_str!("./gitmojis.json");

#[derive(Debug, Deserialize)]
struct GitmojisJson<'a> {
    gitmojis: Vec<Gitmoji<'a>>,
}

#[derive(Debug, Deserialize)]
struct Gitmoji<'a> {
    emoji: Cow<'a, str>,
    code: Cow<'a, str>,
    description: Cow<'a, str>,
    name: Cow<'a, str>,
}

fn main() -> Result<(), Error> {
    use std::io::{Read, Write};

    let gitmojis: GitmojisJson = serde_json::from_str(GITMOJIS_JSON)?;
    let mut child_process = Command::new("fzf-tmux")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child_process.stdin.take().expect("no access to fzf stdin");
    for gitmoji in &gitmojis.gitmojis {
        write!(
            stdin,
            "{} - {} - {}\n",
            gitmoji.emoji, gitmoji.code, gitmoji.description
        )?;
    }

    child_process.wait()?;

    let mut stdout = child_process
        .stdout
        .take()
        .expect("no access to fzf stdout");

    // Regex to split the line in space-separated groups. We are only interested in the first one
    // (the emoji).
    let emoji_regex = regex::Regex::new(r"[^ ]*")?;

    let mut stdout_string: String = String::with_capacity(300);
    stdout.read_to_string(&mut stdout_string)?;

    print!(
        "{}",
        emoji_regex
            .captures_iter(&stdout_string)
            // get the first captured match
            .next()
            // get the first capture group (there is only one)
            .and_then(|cap| cap.get(0))
            .map(|m| m.as_str())
            .unwrap_or("ø")
    );

    Ok(())
}
