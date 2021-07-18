use std::error::Error;
use std::process::{Command, Stdio};
use std::io::Read;

use serde_json::Value;
use std::result::Result;

/// A Youtube search entry from youtube-dl
pub struct YouTubeSearchEntry {
    pub id: String,
    pub description: String,
    pub title: String,
}

pub fn search(query: &str, limit: i32) 
    -> Result<Vec<YouTubeSearchEntry>, Box<dyn Error>> 
{
    println!("Searching YouTube, please wait..");

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
            id: entry.get("id").unwrap().to_string(),
            description: entry.get("description").unwrap().to_string(),
            title: entry.get("title").unwrap().to_string(),
        });
    }

    Ok(results)
}
