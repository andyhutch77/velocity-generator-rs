//! Windows 98 component library. Styles live in `style.css` and use design tokens.
// Library pieces such as `Button` may not be used by a screen yet.
#![allow(dead_code, unused_imports)]

mod button;
mod heading;
mod label;
mod result_display;
mod text_input;
mod title_bar;
mod window;

pub use button::Button;
pub use heading::Heading;
pub use label::Label;
pub use result_display::ResultDisplay;
pub use text_input::TextInput;
pub use title_bar::TitleBar;
pub use window::Window;
