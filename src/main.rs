mod youtube;
use std::io::Write;
use std::error::Error;
use clap::{AppSettings, Clap};

use colored::*;

/// Command line interface for YouTube written in Rust
#[derive(Clap)]
#[clap(version = "1.0",author = "github.com/bimorphism/ytcli")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Search YouTube for a given query string
    #[clap(short = 's', long = "search")]
    search: Option<String>,

    /// Watch a specific video in mpv
    #[clap(short = 'w', long = "watch")]
    watch: Option<String>,

    /// The amount of videos (the limit) that ytcli will show in the search results.
    /// Defaults to 10.
    #[clap(short = 'L', long = "limit", default_value = "10")]
    limit: i32,

    /// View information about a specific upload
    #[clap(short = 'i', long = "info")]
    info: Option<String>,

    /// Display no output when downloading
    #[clap(short = 'q', long = "quiet")]
    quiet: bool,
}

fn watch_link(opts: &Opts, video_id: &str) -> Result<(), Box<dyn Error>> {
    // Spawn an mpv to handle the link
    let result = youtube::search_id(video_id, opts.quiet)?;
    println!();
    
    youtube::spawn_player(&result, "mpv")?;
    
    Ok(())
}

fn show_results(opts: &Opts, query: &str, limit: i32)
     -> Result<(), Box<dyn Error>>
{
    let results = youtube::search(query, limit, opts.quiet)?;

    println!();

    let width = results.iter().map(|it| it.title.len()).max().unwrap_or_default();
    for (i, result) in results.iter().enumerate() {
        println!("▎{}. {: <0width$} {}", i, 
            result.title.red(),
            result.uploader.white().dimmed(),
            width = width);
    }

    println!("{}", "Select a video to view: ".white().dimmed());
    print!("░ ");
    std::io::stdout().flush()?;


    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    let line = line.strip_suffix('\n').unwrap();
    let choice = line.parse::<usize>()?;
    youtube::spawn_player(&results[choice], "mpv")?;

    Ok(())
}

fn show_info(opts: &Opts, video_id: &str) -> Result<(), Box<dyn Error>> {
    let result = youtube::search_id(video_id, opts.quiet)?;

    println!();
    println!("{}: {}", "Title".red().underline(), 
                        result.title.white().underline());
    println!("{}: {} ({})", "VidId".red(), result.id, 
                        "https://youtube.com/watch?v=".to_string() + &result.id);
    println!("{}: {}", "Descr".red().underline(), result.description.replace('\n', ".").white().dimmed());
    println!();
    println!("{}: {} {}",
        "Ratio".red().underline(),
        result.likes.to_string().green(), 
        result.dislikes.to_string().red());    
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    if let Some(ref search) = opts.search {
        show_results(&opts, &search, opts.limit)?;
    }

    if let Some(ref watch) = opts.watch {
        watch_link(&opts, &watch)?;
    }

    if let Some(ref info) = opts.info {
        show_info(&opts, &info)?;
    }

    Ok(())
}
