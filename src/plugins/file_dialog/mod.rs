mod systems;

use bevy::input::common_conditions::{input_just_pressed, input_pressed};
use std::path::PathBuf;
pub use systems::*;

use bevy::prelude::*;
use bevy::tasks::Task;

pub struct FileDialogPlugin;

impl Plugin for FileDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ImportFileEvent>()
            .add_event::<ExportFileEvent>()
            .init_state::<FileState>()
            .add_systems(
                Update,
                (
                    (
                        open_file_dialog::<ImportFile>.run_if(
                            input_pressed(KeyCode::SuperLeft)
                                .and_then(input_just_pressed(KeyCode::KeyL)),
                        ),
                        open_file_dialog::<ExportFile>.run_if(
                            input_pressed(KeyCode::SuperLeft)
                                .and_then(input_just_pressed(KeyCode::KeyS)),
                        ),
                    )
                        .run_if(in_state(FileState::Inactive)),
                    poll_for_selected_file.run_if(not(in_state(FileState::Inactive))),
                ),
            )
            .add_systems(OnEnter(FileState::Inactive), cleanup_file_dialog);
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct SelectedFileTask(Task<Option<PathBuf>>);

#[derive(Event, Deref, DerefMut)]
pub struct ExportFileEvent(PathBuf);

#[derive(Event, Deref, DerefMut)]
pub struct ImportFileEvent(PathBuf);

#[derive(States, Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FileState {
    #[default]
    Inactive,
    Export,
    Import,
}
