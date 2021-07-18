use std::error::Error;
use std::process::{Command, Stdio};
use std::io::Read;

use serde_json::Value;
use spinners::{Spinner, Spinners};
use std::result::Result;

/// A Youtube search entry from youtube-dl
#[derive(Clone)]
pub struct YouTubeSearchEntry {
    pub id: String,
    pub description: String,
    pub title: String,
    pub likes: i32,
    pub dislikes: i32,
}

fn strip_quotes(s: String) -> String {
    s.replace('"', "")
}

/// Searches YouTube for the given query string. The limit
/// specifies how many results to include in the search, the higher the limit
/// the more time it takes for youtube-dl to search however.
pub fn search(query: &str, limit: i32) 
    -> Result<Vec<YouTubeSearchEntry>, Box<dyn Error>> 
{
    let sp = Spinner::new(Spinners::Dots, "Searching YouTube, please wait..".into());

    let mut results: Vec<YouTubeSearchEntry> = Vec::new();
    let query = format!("ytsearch{}:{}", limit, query);

    let mut child = Command::new("youtube-dl")
        .args(&[query, "--skip-download".to_string(), "-J".to_string()])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run youtube-dl");

    let mut stdout = String::new();
    child.stdout.take().unwrap().read_to_string(&mut stdout)?;
    let json: Value = serde_json::from_str(&stdout)?;

    let entries = json["entries"].as_array().unwrap();
    for entry in entries {
        results.push(YouTubeSearchEntry {
            id: strip_quotes(entry.get("id").unwrap().to_string()),
            description: strip_quotes(entry.get("description").unwrap().to_string()),
            title: strip_quotes(entry.get("title").unwrap().to_string()),
            likes: strip_quotes(entry.get("like_count").unwrap().to_string())
                .parse::<i32>()?,
            dislikes: strip_quotes(entry.get("dislike_count").unwrap().to_string())
                .parse::<i32>()?,
        });
    }

    sp.stop();
    Ok(results)
}

pub fn search_id(video_id: &str)
    -> Result<YouTubeSearchEntry, Box<dyn Error>>
{
    let results = search(video_id, 1)?;
    let result = &results[0];

    Ok(result.clone())
}

/// Spawn a mpv/mplayer instance to handle the YouTube stream
/// 
/// NOTE: `entry` is a reference since otherwise the n'th item of the vector would be moved
/// while the rest of the vector *isn't* moved and Rust screams at me :(
pub fn spawn_player(entry: &YouTubeSearchEntry, player: &str) 
    -> Result<(), Box<dyn Error>>
{
    println!("Opening https://youtube.com/watch?v={} in mpv", entry.id.replace('"', ""));

    let _child = Command::new(player)
        .arg(format!("https://youtube.com/watch?v={}", entry.id.replace('"', "")))
        .spawn()?;

    Ok(())
}