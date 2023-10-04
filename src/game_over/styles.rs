use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgba(0.1, 0.1, 0.1, 0.0);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgba(0.1, 0.1, 0.1, 0.5);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgba(0.1, 0.1, 0.1, 1.0);
pub const BUTTON_STYLE: Style = Style {
					size: Size::new(Val::Px(420.0), Val::Px(80.0)),
					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					..Style::DEFAULT
				};