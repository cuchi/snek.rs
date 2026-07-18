use std::fs;
use std::path::PathBuf;

/// Path to the high-scores file in the user's home directory.
/// Returns `None` if the home directory can't be determined.
fn scores_path() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        Some(PathBuf::from(home).join(".snek_highscores"))
    } else if let Ok(profile) = std::env::var("USERPROFILE") {
        Some(PathBuf::from(profile).join(".snek_highscores"))
    } else {
        None
    }
}

/// Load the top 5 high scores from disk.  Returns an empty list if the file
/// doesn't exist or can't be read.
pub fn load() -> Vec<u32> {
    let path = match scores_path() {
        Some(p) => p,
        None => return Vec::new(),
    };
    match fs::read_to_string(&path) {
        Ok(contents) => contents
            .lines()
            .filter_map(|line| line.trim().parse::<u32>().ok())
            .take(5)
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// Insert `new_score` into the top-5 list (descending order), persist to
/// disk, and return `true` if it made the cut.  Zero scores are ignored.
pub fn maybe_insert(new_score: u32, scores: &mut Vec<u32>) -> bool {
    if new_score == 0 {
        return false;
    }

    scores.push(new_score);
    scores.sort_unstable_by(|a, b| b.cmp(a)); // descending
    scores.truncate(5);

    if let Some(path) = scores_path() {
        let data: String = scores.iter().map(|s| s.to_string() + "\n").collect();
        fs::write(path, data).ok();
    }

    scores.contains(&new_score)
}
