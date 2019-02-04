pub trait MpcAPI {
    fn current_song(&self) -> String;
    fn play(&self);
    fn pause(&self);
    fn toggle(&self);
    fn next(&self);
    fn previous(&self);
}

pub fn new_mpc() -> impl MpcAPI {
    if cfg!(target_os = "windows") {
        unimplemented!()
    }

    MpcUnix::new()
}

pub struct MpcUnix;

impl MpcUnix {
    pub fn new() -> MpcUnix {
        MpcUnix {}
    }
}

impl MpcAPI for MpcUnix {
    // Retrieve current song.
    fn current_song(&self) -> String {
        // Mpc returns other information such as volume, current seek, etc.
        // This get's just the first line with the song title and artist.
        let status = run_mpccmd(&[]);
        status.splitn(2, '\n').next().unwrap().to_owned()
    }

    // Play if paused, and vice versa.
    fn toggle(&self) {
        run_mpccmd(&["toggle"]);
    }

    // Pause if playing.
    fn pause(&self) {
        run_mpccmd(&["pause"]);
    }

    // Play if paused.
    fn play(&self) {
        run_mpccmd(&["play"]);
    }

    // Change to next track.
    fn next(&self) {
        run_mpccmd(&["next"]);
    }

    // Change to next track.
    fn previous(&self) {
        run_mpccmd(&["prev"]);
    }
}

// Run an AppleScript command.
fn run_mpccmd(args: &[&str]) -> String {
    use std::process::Command;

    let mut process = Command::new("mpc");

    for arg in args {
        process.arg(arg);
    }

    let output = process.output().unwrap();

    std::str::from_utf8(&output.stdout[..]).unwrap().to_owned()
}
