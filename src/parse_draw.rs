use bevy::prelude::*;
use bevy::asset::RenderAssetUsages;
use std::f32::consts::PI;
use bevy_fontmesh::prelude::*;

use crate::*;  // for AppState, ColorMap, AssetHandels, ClassValueAsset, etc.
pub use loading_plugin::*;

// Our modules, adjacent directories
mod components;
mod spawn;
mod layout;
mod associations;
mod collapse;
// Re-export everything
pub use components::*; // so crate::parse_draw::ResolvedAssociation still works
use spawn::*;
use layout::*;
pub use associations::*;
use collapse::*;

pub struct ParseDrawPlugin;
impl Plugin for ParseDrawPlugin {
    fn build(&self, app: &mut App) {
        app
            // initialize the registry
            .init_resource::<KeyRegistry>()
            .add_systems(
                Update,
                (
                    sync_leaf_visuals, // run before update_subclass_positions
                    update_subclass_positions,
                    animate_translation,
                    animate_scale,
                    animate_alpha,
                    toggle_collapse_visuals,
                )
                    .chain()
                    .run_if(in_state(AppState::Loaded)),
            )
            // parse the classes.json Value and draw the classes
            .add_systems(OnEnter(AppState::Loaded), parse_draw_classes)
            .add_systems(
                PostUpdate,
                (resolve_pending_associations, update_association_beams)
                    .chain()
                    .after(TransformSystems::Propagate)
                    .run_if(in_state(AppState::Loaded)),
            )
            .add_observer(on_click);
    }
}

