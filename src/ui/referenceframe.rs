use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        query::{Changed, With},
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    log::info,
    ui::{FlexDirection, Interaction},
};

use crate::{
    objects::systemsets::ObjectSets,
    orbits::history::{OrbitHistoryEntity, SelectedReferenceFrame},
};

use super::{
    super::objects::planet::Planet,
    button::{UiButtonBuilder, UiButtonStyle},
    container::UiContainerBuilder,
    systemsets::UiSets,
    window::UiWindowBuilder,
};

#[derive(Component)]
pub struct ReferenceChangeInteraction(Entity);

pub struct UiReferenceFramePlugin;

impl Plugin for UiReferenceFramePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            build_ui
                .in_set(UiSets::UiStartupAll)
                .after(ObjectSets::SpawnPlanet),
        )
        .add_systems(Update, change_frame.in_set(UiSets::UiUpdateAll));
    }
}

pub fn build_ui(
    mut commands: Commands,
    planets: Query<(&Planet, &OrbitHistoryEntity)>,
    asset_server: Res<AssetServer>,
) {
    let buttons: Vec<Entity> = planets
        .iter()
        .map(|(p, oh)| {
            UiButtonBuilder::build(
                &mut commands,
                &asset_server,
                ReferenceChangeInteraction(oh.0),
                p.name.to_owned(),
                UiButtonStyle::default(),
            )
        })
        .collect();

    let container =
        UiContainerBuilder::build(&mut commands, FlexDirection::Row, buttons.as_slice());

    UiWindowBuilder::build(
        &mut commands,
        &asset_server,
        "Reference Frame".into(),
        container,
        (30.0, 30.0),
    );
}

fn change_frame(
    interaction_query: Query<
        (&Interaction, &ReferenceChangeInteraction),
        (Changed<Interaction>, With<ReferenceChangeInteraction>),
    >,
    mut frame: ResMut<SelectedReferenceFrame>,
) {
    for (interaction, reference_change) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                info!("Setting reference frame to {:?}", reference_change.0);
                frame.target = reference_change.0;
            }
            _ => {}
        }
    }
}
