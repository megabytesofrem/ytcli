mod youtube;
use std::io::Write;
use std::error::Error;
use clap::{AppSettings, Clap};

/// Command line interface for YouTube written in Rust
#[derive(Clap)]
#[clap(version = "1.0",author = "github.com/bimorphism/ytcli")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Search YouTube for a given query string
    #[clap(short = 's', long = "search")]
    search: Option<String>,

    /// The amount of videos (the limit) that ytcli will show in the search results.
    /// Defaults to 10.
    #[clap(short = 'L', long = "limit", default_value = "10")]
    limit: i32,

    /// View information about a specific upload
    #[clap(short = 'i', long = "info")]
    info: Option<String>,
}

fn show_results(query: &str, limit: i32)
     -> Result<(), Box<dyn Error>>
{
    let results = youtube::search(query, limit)?;

    println!();
    for (i, result) in results.iter().enumerate() {
        println!("{}\tID: {}\t\tTitle: {}", i, result.id, result.title);
        println!("-------------------------------------------");
    }

    println!("Select a video to view (by its number)");
    print!(">");
    std::io::stdout().flush()?;


    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    let line = line.strip_suffix('\n').unwrap();
    let choice = line.parse::<usize>()?;
    youtube::spawn_player(&results[choice], "mpv")?;

    Ok(())
}

fn show_info(video_id: &str) -> Result<(), Box<dyn Error>> {
    let result = youtube::search_id(video_id)?;

    println!();
    println!("-----------------------------------");
    println!("Title: {}", result.title);
    println!("ID: {}", result.id);
    println!("Description: {}", result.description);
    println!();
    println!("Likes, dislikes: {}, {}", result.likes, result.dislikes);
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    if let Some(search) = opts.search {
        show_results(&search, opts.limit)?;
    }

    if let Some(info) = opts.info {
        show_info(&info)?;
    }

    Ok(())
}
