//! Windows 98 component library. Styles live in `style.css` and use design tokens.

mod button;
mod label;
mod text_input;
mod title_bar;
mod window;

pub use button::Button;
pub use label::Label;
pub use text_input::TextInput;
pub use title_bar::TitleBar;
pub use window::Window;
