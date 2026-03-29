use discord_rpc_client::models::{Activity, ActivityAssets};
use discord_rpc_client::Client;
use regex::Regex;
use reqwest::blocking::Client as HttpClient;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn get_cover_url(artist: &str, album: &str) -> Option<String> {
    let http = HttpClient::new();

    let search_url = format!(
        "https://musicbrainz.org/ws/2/release/?query=artist:{} release:{}&fmt=json&limit=1",
        urlencoding::encode(artist),
        urlencoding::encode(album)
    );

    let resp: serde_json::Value = http
        .get(&search_url)
        .header("User-Agent", "ElisaRPC/0.1")
        .send()
        .ok()?
        .json()
        .ok()?;

    let mbid = resp["releases"][0]["id"].as_str()?;

    Some(format!("https://coverartarchive.org/release/{}/front", mbid))
}

fn set_discord_activity(
    drpc: &mut Client,
    title: &str,
    artist: Option<&str>,
    album: Option<&str>,
) {
    let details_text = match artist {
        Some(a) => format!("{} - {}", title, a),
        None => title.to_string(),
    };
    let state_text = album.unwrap_or("Unbekanntes Album").to_string();

    let cover_url = match (artist, album) {
        (Some(ar), Some(al)) => get_cover_url(ar, al).unwrap_or_else(|| "elisalogo".to_string()),
        _ => "elisalogo".to_string(),
    };

    println!("Setting activity: {} | {} | cover: {}", details_text, state_text, cover_url);

    let result = drpc.set_activity(|act: Activity| {
        act.details(&details_text)
            .state(&state_text)
            .assets(|ass: ActivityAssets| ass.large_image(&cover_url))
    });

    match result {
        Ok(_) => println!("Activity updated!"),
        Err(e) => eprintln!("set_activity error: {:?}", e),
    }
}

fn main() {
    let metadata_pattern = Regex::new(r"elisa\s+(\S+)\s+(.+)").unwrap();

    let mut drpc = Client::new(1230850847345348669);
    drpc.on_ready(|_ctx| println!("Discord Ready"));
    drpc.start();

    thread::sleep(Duration::from_secs(2));

    println!("Monitoring metadata...");

    let mut title: Option<String> = None;
    let mut artist: Option<String> = None;
    let mut album: Option<String> = None;
    let mut last_title: Option<String> = None;

    let process = Command::new("playerctl")
        .args(["-p", "elisa", "-F", "metadata"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start playerctl");

    let stdout = process.stdout.expect("Failed to get stdout");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        };

        println!("LINE: {:?}", line);

        if line.trim().is_empty() {
            if let Some(ref t) = title {
                if last_title.as_deref() != Some(t.as_str()) {
                    set_discord_activity(&mut drpc, t, artist.as_deref(), album.as_deref());
                    last_title = title.clone();
                }
            }
            title = None;
            artist = None;
            album = None;
            continue;
        }

        if let Some(caps) = metadata_pattern.captures(&line) {
            let key = caps.get(1).map(|m| m.as_str());
            let value = caps.get(2).map(|m| m.as_str()).map(String::from);

            match key {
                Some("xesam:title") => {
                    let new_title = value.clone();
                    if last_title != new_title {
                        if let Some(ref t) = new_title {
                            set_discord_activity(&mut drpc, t, artist.as_deref(), album.as_deref());
                            last_title = new_title.clone();
                        }
                    }
                    title = new_title;
                }
                Some("xesam:artist") => artist = value,
                Some("xesam:album") => album = value,
                _ => {}
            }
        }
    }
}