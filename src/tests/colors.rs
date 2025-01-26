use crate::moss_definitions::functions::*;
use crate::moss_definitions::types::*;

pub struct InitialColors {
    pub background: Color,
    pub text_colors: TextColors,
}

impl InitialColors {
    pub unsafe fn get() -> Self {
        Self {
            background: moss_defaults_get_color(BACKGROUND.to_string()).unwrap(),
            text_colors: moss_defaults_get_text_color(TEXT_COLOR.to_string()).unwrap(),
        }
    }
    pub unsafe fn revert(&self) {
        moss_defaults_set_color(BACKGROUND.to_string(), self.background).unwrap();
        moss_defaults_set_text_color(TEXT_COLOR.to_string(), self.text_colors).unwrap();
    }
}

const BACKGROUND: &str = "BACKGROUND";
const TEXT_COLOR: &str = "TEXT_COLOR";

pub struct TestColors {
    pub no_alpha_color: Color,
    pub alpha_color: Color,
    pub text_colors_no_alpha_no_background: TextColors,
    pub text_colors_no_alpha_with_background_no_alpha: TextColors,
    pub text_colors_no_alpha_with_background_with_alpha: TextColors,
    pub text_colors_with_alpha_no_background: TextColors,
    pub text_colors_with_alpha_with_background_no_alpha: TextColors,
    pub text_colors_with_alpha_with_background_with_alpha: TextColors
}

impl TestColors {
    pub unsafe fn new() -> Self {
        let no_alpha_color = Color::new_monochrome(100, None);
        let alpha_color = Color::new_monochrome(100, Some(100));
        Self {
            no_alpha_color,
            alpha_color,
            text_colors_no_alpha_no_background: TextColors {
                foreground: no_alpha_color,
                background: None,
            },
            text_colors_no_alpha_with_background_no_alpha: TextColors {
                foreground: no_alpha_color,
                background: Some(no_alpha_color),
            },
            text_colors_no_alpha_with_background_with_alpha: TextColors {
                foreground: no_alpha_color,
                background: Some(alpha_color),
            },

            text_colors_with_alpha_no_background: TextColors {
                foreground: alpha_color,
                background: None,
            },
            text_colors_with_alpha_with_background_no_alpha: TextColors {
                foreground: alpha_color,
                background: Some(no_alpha_color),
            },
            text_colors_with_alpha_with_background_with_alpha: TextColors {
                foreground: alpha_color,
                background: Some(alpha_color),
            },
        }
    }
}

pub unsafe fn test_colors(c: &TestColors) {
    moss_defaults_set_color(BACKGROUND.to_string(), c.no_alpha_color).unwrap();
    let result_color = moss_defaults_get_color(BACKGROUND.to_string()).unwrap();
    moss_em_config_set::<bool>(
        "test_defaults_set_color_no_alpha",
        c.no_alpha_color == result_color,
    );

    moss_defaults_set_color(BACKGROUND.to_string(), c.alpha_color).unwrap();
    let result_color = moss_defaults_get_color(BACKGROUND.to_string()).unwrap();
    moss_em_config_set::<bool>(
        "test_defaults_set_color_with_alpha",
        c.alpha_color == result_color,
    );
}

pub unsafe fn test_text_colors(c: &TestColors) {
    moss_defaults_set_text_color(TEXT_COLOR.to_string(), c.text_colors_no_alpha_no_background).unwrap();
    let result_color = moss_defaults_get_text_color(TEXT_COLOR.to_string()).unwrap();
    moss_em_config_set::<bool>(
        "test_defaults_set_text_color_no_alpha_no_background",
        c.text_colors_no_alpha_no_background == result_color,
    );

    moss_defaults_set_text_color(
        TEXT_COLOR.to_string(),
        c.text_colors_no_alpha_with_background_no_alpha,
    ).unwrap();
    let result_color = moss_defaults_get_text_color(TEXT_COLOR.to_string()).unwrap();
    moss_em_config_set::<bool>(
        "test_defaults_set_text_color_no_alpha_with_background_no_alpha",
        c.text_colors_no_alpha_with_background_no_alpha == result_color,
    );

    moss_defaults_set_text_color(
        TEXT_COLOR.to_string(),
        c.text_colors_no_alpha_with_background_with_alpha,
    ).unwrap();
    let result_color = moss_defaults_get_text_color(TEXT_COLOR.to_string()).unwrap();
    moss_em_config_set::<bool>(
        "test_defaults_set_text_color_no_alpha_with_background_with_alpha",
        c.text_colors_no_alpha_with_background_with_alpha == result_color,
    );

    moss_defaults_set_text_color(
        TEXT_COLOR.to_string(),
        c.text_colors_with_alpha_no_background,
    ).unwrap();
    let result_color = moss_defaults_get_text_color(TEXT_COLOR.to_string()).unwrap();
    moss_em_config_set::<bool>(
        "test_defaults_set_text_color_with_alpha_no_background",
        c.text_colors_with_alpha_no_background == result_color,
    );

    moss_defaults_set_text_color(
        TEXT_COLOR.to_string(),
        c.text_colors_with_alpha_with_background_no_alpha,
    ).unwrap();
    let result_color = moss_defaults_get_text_color(TEXT_COLOR.to_string()).unwrap();
    moss_em_config_set::<bool>(
        "test_defaults_set_text_color_with_alpha_with_background_no_alpha",
        c.text_colors_with_alpha_with_background_no_alpha == result_color,
    );

    moss_defaults_set_text_color(
        TEXT_COLOR.to_string(),
        c.text_colors_with_alpha_with_background_with_alpha,
    ).unwrap();
    let result_color = moss_defaults_get_text_color(TEXT_COLOR.to_string()).unwrap();
    moss_em_config_set::<bool>(
        "test_defaults_set_text_color_with_alpha_with_background_with_alpha",
        c.text_colors_with_alpha_with_background_with_alpha == result_color,
    );
}

pub unsafe fn run_all_color_tests() {
    let initial = InitialColors::get();
    let colors = TestColors::new();

    // Run tests
    test_colors(&colors);
    test_text_colors(&colors);

    // Reset defaults
    initial.revert();
}