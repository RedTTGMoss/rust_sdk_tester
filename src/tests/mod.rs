mod gui;
use gui::*;
mod api;
use api::*;
mod defaults;
use crate::MossState;
pub use defaults::*;

mod context_menu;
use crate::result_screen::ResultScreen;
pub use context_menu::*;

mod colors;

pub unsafe fn run_all_tests(state: &MossState) {
    run_all_gui_tests(state);
    run_all_api_tests();
    ResultScreen::ResultScreen_open_action();
}
