mod gui;
use gui::*;
mod api;
use api::*;
mod defaults;
pub use defaults::*;
use crate::MossState;

mod context_menu;
pub use context_menu::*;

mod colors;

pub unsafe fn run_all_tests(state: &MossState) {
    run_all_gui_tests(state);
    run_all_api_tests();
}