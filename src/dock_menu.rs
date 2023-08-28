use bevy::prelude::*;

#[derive(Default)]
pub struct Card {
    pub person_name: String,
    pub task: String,
}

impl Card {
    pub fn spawn_node(&self, cmd: &mut ChildBuilder, font: Handle<Font>) {
        let text_style = TextStyle {
            font,
            font_size: 18.,
            color: Color::BLACK,
        };
        let color_paper = Color::rgb(1., 0.85, 0.63).into();
        cmd.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(40.),
                height: Val::Percent(80.),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Stretch,
                flex_direction: FlexDirection::Column,
                padding: UiRect::percent(2., 2., 2., 2.),
                margin: UiRect::percent(0.5, 0.5, 0.5, 0.5),
                ..default()
            },
            background_color: color_paper,
            ..default()
        })
        .with_children(|cmd| {
            cmd.spawn(TextBundle::from_section(
                self.person_name.clone(),
                text_style.clone(),
            ));
            cmd.spawn(TextBundle::from_section(
                self.task.clone(),
                text_style.clone(),
            ));
        });
    }
}