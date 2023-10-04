use bevy::prelude::*;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiButton;

#[derive(Copy, Clone, Debug)]
pub struct UiButtonProps {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub border: UiRect,
    pub border_color: Color,
}

impl Default for UiButtonProps {
    fn default() -> Self {
        Self {
            width: Val::Px(270.0),
            height: Val::Px(90.0),
            color: Color::TEAL,
            border: UiRect::all(Val::Px(5.0)),
            border_color: Color::BLACK,
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiButtonBundle {
    pub child: UiWidgetBundle,
    pub button: Button,
    pub interaction: Interaction,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub image: UiImage,
    internal_tag: UiButton,
}

impl Default for UiButtonBundle {
    fn default() -> Self {
        UiButtonBundle::from(UiButtonProps::default())
    }
}

impl UiButtonBundle {
    pub fn from(props: UiButtonProps) -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    width: props.width,
                    height: props.height,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: props.border,
                    ..default()
                },
                ..default()
            },
            button: Default::default(),
            interaction: Default::default(),
            background_color: BackgroundColor::from(props.color),
            border_color: BorderColor(props.border_color),
            image: Default::default(),
            internal_tag: UiButton::default(),
        }
    }

    pub fn from_size(width: Val, height: Val) -> Self {
        Self::from(UiButtonProps {
            width,
            height,
            ..default()
        })
    }

    pub fn from_size_splat(val: Val) -> Self {
        Self::from(UiButtonProps {
            width: val,
            height: val,
            ..default()
        })
    }

    pub fn from_width(width: Val) -> Self {
        Self::from(UiButtonProps {
            width,
            ..default()
        })
    }

    pub fn from_height(height: Val) -> Self {
        Self::from(UiButtonProps {
            height,
            ..default()
        })
    }

    pub fn from_color(color: Color) -> Self {
        Self::from(UiButtonProps {
            color,
            ..default()
        })
    }
}
