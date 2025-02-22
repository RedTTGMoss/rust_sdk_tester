use crate::moss_definitions;
use crate::moss_definitions::functions::*;
use crate::moss_definitions::types::*;
use extism_pdk;
use moss_macros::*;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize)]
pub struct ResultScreen {
    background_color: Color,
    progress_color: Color,
    title_text: TextRef,
    timer: i64,
    progress_rect: Rect,
}

#[moss_screen]
impl ResultScreen {
    pub fn r#loop() {
        let state = moss_em_get_state()?;
        let background_color = moss_pe_get_screen_value::<Color>("background_color")?.value;
        let progress_color = moss_pe_get_screen_value::<Color>("progress_color")?.value;
        let title_text = moss_pe_get_screen_value::<TextRef>("title_text")?.value;
        let timer = moss_pe_get_screen_value::<i64>("timer")?.value;
        let mut progress_rect = moss_pe_get_screen_value::<Rect>("progress_rect")?.value;
        let (completed, total_functions, called_functions) = ResultScreen::check_function_calls();

        moss_pe_draw_rect(
            &background_color,
            &Rect::new(0, 0, state.width as i64, state.height as i64),
            0,
            None,
        );
        title_text.display();

        let max_width = state.width as i64 - 60;
        let progress = called_functions as f64 / total_functions as f64;

        progress_rect.width = (max_width as f64 * progress) as i64;

        moss_pe_draw_rect(
            &progress_color,
            &progress_rect,
            0,
            Some(PygameExtraRectEdgeRounding::all(10)),
        );

        progress_rect.width = max_width;

        moss_pe_draw_rect(
            &moss_color!(0xffffff),
            &progress_rect,
            1,
            Some(PygameExtraRectEdgeRounding::all(10)),
        );
        if !completed {
            moss_pe_set_screen_value::<String>("new_value", "test".to_string());
            moss_em_config_set::<bool>("completed", true);
            moss_em_config_set::<i64>("total_functions", 0);
            moss_pe_close_screen()?;
            ResultScreen::ResultScreen_open_action();
        }

        if get_rm_time_now() - timer > 5000 {
            moss_em_export_statistical_data()?;
            moss_pe_close_screen()?;
        }
    }

    fn check_function_calls() -> (bool, i64, i64) {
        let completed = moss_em_config_get::<bool>("completed").unwrap().value;
        if !completed {
            return (false, 0, 0);
        }
        let total_functions = moss_em_config_get::<i64>("total_functions").unwrap().value;
        if total_functions != 0 {
            let called_functions = moss_em_config_get::<i64>("called_functions").unwrap().value;
            return (true, total_functions, called_functions);
        }
        let file = File::open("temp/extension_calls.json").unwrap();
        let reader = BufReader::new(file);
        let json_data: HashMap<String, HashMap<String, u64>> =
            serde_json::from_reader(reader).unwrap();

        if let Some(functions) = json_data.get("rust_sdk_tester") {
            let total_functions = functions.len() as i64;
            let called_functions = functions.iter().filter(|(_, &count)| count > 0).count() as i64;
            moss_em_config_set::<i64>("total_functions", total_functions);
            moss_em_config_set::<i64>("called_functions", called_functions);
            (true, total_functions, called_functions)
        } else {
            (true, 0, 0)
        }
    }

    pub fn open_action() {
        let font = moss_defaults_get::<String>("TITLE_FONT")?.value;
        let font2 = moss_defaults_get::<String>("LOGO_FONT")?.value;
        let title_text = TextRef::create_white("Rust SDK tester", font.as_str(), 24);
        let mut screen = ResultScreen {
            background_color: moss_color!(0x000000),
            progress_color: moss_color!(0x00ff65),
            title_text,
            timer: get_rm_time_now(),
            progress_rect: Rect::new(30, 0, 0, 20),
        };
        screen
            .title_text
            .update_font(font2.as_str(), screen.title_text.font_size);
        screen.title_text.update_text("Rust SDK tester :3");
        screen.title_text.rect.set_topleft(10, 10);
        screen.title_text.update_rect();
        screen.progress_rect.y = screen.title_text.rect.height + 20;
        ResultScreen::open_with_data(screen);
    }
}
