extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session};

use crate::mpc::{new_mpc, MpcAPI};

enum Messages {
    CurrentSong,
    Toggle,
    Play,
    Pause,
    Next,
    Previous,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "current_song" => Messages::CurrentSong,
            "toggle" => Messages::Toggle,
            "play" => Messages::Play,
            "pause" => Messages::Pause,
            "next" => Messages::Next,
            "previous" => Messages::Previous,
            _ => Messages::Unknown(event),
        }
    }
}

/// EventHandler receives RPC requests, and maps them to right Mpc and Neovim commands.
pub struct EventHandler {
    nvim: Neovim,
    mpc: Box<MpcAPI>,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        let mut session = Session::new_parent().unwrap();
        session.set_infinity_timeout();
        let nvim = Neovim::new(session);
        let mpc = new_mpc();

        EventHandler {
            nvim,
            mpc: Box::new(mpc),
        }
    }

    pub fn handle_events(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, _values) in receiver {
            match Messages::from(event) {
                Messages::CurrentSong => {
                    let song = self.mpc.current_song();

                    self.nvim.command(&format!("echo \"{}\"", song)).unwrap();
                }

                Messages::Toggle => {
                    self.mpc.toggle();
                }

                Messages::Play => {
                    self.mpc.play();
                }

                Messages::Pause => {
                    self.mpc.pause();
                }

                Messages::Next => {
                    self.mpc.next();
                }

                Messages::Previous => {
                    self.mpc.previous();
                }

                // Handle any "Unknown" messages.
                Messages::Unknown(ev) => {
                    self.nvim
                        .command(&format!("echoerr \"{}\" Unknown command", ev))
                        .unwrap();
                }
            }
        }
    }
}
