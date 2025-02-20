use crate::moss_definitions;
use crate::moss_definitions::functions::*;
use crate::moss_definitions::types::*;
use extism_pdk;
use moss_macros::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResultScreen {
    background_color: Color,
    title_text: TextRef,
}

#[moss_screen]
impl ResultScreen {
    pub fn r#loop() {
        let state = moss_em_get_state()?;
        let background_color = moss_pe_get_screen_value::<Color>("background_color")?.value;
        let title_text = moss_pe_get_screen_value::<TextRef>("title_text")?.value;

        moss_pe_draw_rect(
            background_color,
            Rect::new(0, 0, state.width as i64, state.height as i64),
            0,
            None,
        );
        title_text.display();
    }
    pub fn open_action() {
        let font = moss_defaults_get::<String>("TITLE_FONT").unwrap().value;
        let mut screen = ResultScreen {
            background_color: moss_color!(0x000000),
            title_text: TextRef::create_white("Results", font.as_str(), 24),
        };
        screen.title_text.rect.set_topleft(10, 10);
        screen.title_text.update_rect();
        ResultScreen::open_with_data(screen);
    }
}
