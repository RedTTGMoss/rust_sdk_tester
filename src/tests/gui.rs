use crate::moss_definitions::functions::*;
use crate::moss_definitions::types::*;

pub unsafe fn check_icon(state: &MossState) {
    // Check for the test icon and invert it
    for icon in state.icons.iter() {
        if icon == "test_icon" {
            moss_em_config_set::<bool>("icon_loaded", true);

            moss_gui_invert_icon("test_icon".to_string(), "test_icon_inverted".to_string()).unwrap();

            moss_em_config_set::<bool>("icon_inverted", true);
            break;
        }
    }
}



pub unsafe fn run_all_gui_tests(state: &MossState) {
    check_icon(state);
}