mod btctl;
mod menu;

use inquire::ui::{Color, RenderConfig, Styled};

/// `main` is the entry point of the program, and it sets the global render config and then runs the
/// menu
fn main() {
    inquire::set_global_render_config(get_render_config());
    menu::run();
}

/// `get_render_config` returns a `RenderConfig` with a few customizations
///
/// Returns:
///
/// A RenderConfig struct
fn get_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("•").with_fg(Color::LightBlue);
    render_config.highlighted_option_prefix = Styled::new("⤇").with_fg(Color::LightYellow);
    render_config.selected_checkbox = Styled::new("•").with_fg(Color::DarkGreen);
    render_config.unselected_checkbox = Styled::new(" ").with_fg(Color::Grey);
    render_config.scroll_up_prefix = Styled::new("⬆").with_fg(Color::Grey);
    render_config.scroll_down_prefix = Styled::new("⬇").with_fg(Color::Grey);
    render_config
}
