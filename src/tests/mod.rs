mod gui;
use gui::*;
mod defaults;
use defaults::*;
use crate::MossState;

mod colors;

pub unsafe fn run_all_tests(state: MossState) {
    run_all_defaults_tests();
    run_all_gui_tests(state);
}