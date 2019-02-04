extern crate neovim_lib;

mod mpc;
mod neovim;

fn main() {
    let mut nvim = neovim::EventHandler::new();

    // Block
    nvim.handle_events();
}
