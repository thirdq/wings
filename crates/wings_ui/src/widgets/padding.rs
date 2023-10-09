use bevy::prelude::*;
use crate::classes::edge_insets::EdgeInsets;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiPadding(pub EdgeInsets);

#[derive(Copy, Clone, Debug, Default)]
pub struct UiPaddingProps {
    pub padding: EdgeInsets,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiPaddingBundle {
    pub child: UiWidgetBundle,
    pub padding: UiPadding,
}

impl Default for UiPaddingBundle {
    #[inline]
    fn default() -> Self {
        UiPaddingBundle::from(UiPaddingProps::default())
    }
}

impl UiPaddingBundle {
    #[inline]
    pub fn from(props: UiPaddingProps) -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    padding: props.padding.to_ui_rect(),
                    ..default()
                },
                ..default()
            },
            padding: UiPadding(props.padding),
        }
    }

    #[inline]
    pub fn from_edge(padding: EdgeInsets) -> Self {
        Self::from(UiPaddingProps { padding })
    }
}
