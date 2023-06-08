use ratatui::{backend::Backend, Frame, widgets::Paragraph};

use crate::app::App;

pub fn render<B: Backend>(frame: &mut Frame<B>, _app: &mut App) {
    frame.render_widget(Paragraph::new("Hello world!"), frame.size());
}

#[allow(dead_code)]
mod colors {
    use ratatui::style::Color;

    pub const NORD0: Color = Color::Rgb(46, 52, 64);
    pub const NORD1: Color = Color::Rgb(59, 66, 82);
    pub const NORD2: Color = Color::Rgb(67, 76, 94);
    pub const NORD3: Color = Color::Rgb(76, 86, 106);
    pub const NORD4: Color = Color::Rgb(216, 222, 233);
    pub const NORD5: Color = Color::Rgb(229, 233, 240);
    pub const NORD6: Color = Color::Rgb(236, 239, 244);
    pub const NORD7: Color = Color::Rgb(143, 188, 187);
    pub const NORD8: Color = Color::Rgb(136, 192, 208);
    pub const NORD9: Color = Color::Rgb(129, 161, 193);
    pub const NORD10: Color = Color::Rgb(94, 129, 172);
    pub const NORD11: Color = Color::Rgb(191, 97, 106);
    pub const NORD12: Color = Color::Rgb(208, 135, 112);
    pub const NORD13: Color = Color::Rgb(235, 203, 139);
    pub const NORD14: Color = Color::Rgb(163, 190, 140);
    pub const NORD15: Color = Color::Rgb(180, 142, 173);
}
