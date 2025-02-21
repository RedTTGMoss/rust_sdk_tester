mod gui;
use gui::*;
mod api;
use api::*;
mod defaults;
use crate::{moss_em_export_statistical_data, MossState};
pub use defaults::*;

mod context_menu;
pub use context_menu::*;

mod colors;

pub unsafe fn run_all_tests(state: &MossState) {
    run_all_gui_tests(state);
    run_all_api_tests();
    moss_em_export_statistical_data().unwrap();
}
