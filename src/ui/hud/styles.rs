use bevy::prelude::*;
pub const HUD_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::SpaceBetween,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
    padding: UiRect::all(Val::Px(5.0)),
    ..Style::DEFAULT
};


pub fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/font.ttf"),
		font_size: 42.0,
		color: Color::rgb(1.0, 1.0, 1.0),
	}
}