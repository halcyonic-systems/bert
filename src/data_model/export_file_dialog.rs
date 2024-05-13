use std::path::PathBuf;
use rfd::FileDialog;
use bevy::{prelude::*,
    tasks::{
        futures_lite::future, AsyncComputeTaskPool, Task
    },
};

#[derive(States, Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FileExportState {
    #[default]
    Inactive,
    Select,
    Poll,
    Save,
    CleanUp,
}

impl FileExportState  {
    pub fn next(&self) -> Self {
        type S = FileExportState;
        match *self {
            S::Inactive => S::Select,
            S::Select   => S::Poll,
            S::Poll     => S::Save,
            S::Save     => S::CleanUp,
            S::CleanUp  => S::Inactive,
        }
    }
    pub fn reset(&self) -> Self {
        type S = FileExportState;
        match *self { _ => S::Inactive }
    }
}

#[derive(Component)]
pub struct ExportFileTask(Task<Option<PathBuf>>);

#[derive(Component)]
pub struct SaveFile {
    pub path_buf: PathBuf,
}

pub fn export_file(
    state: Res<State<FileExportState>>,
    mut next_state: ResMut<NextState<FileExportState>>,
) {
        info!("state: {:?}", state.get());
        next_state.set(state.get().next());
}

pub fn open_export_dialog(
    mut commands: Commands,
    state: Res<State<FileExportState>>,
    mut next_state: ResMut<NextState<FileExportState>>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    info!("Start Polling");

    let task = thread_pool.spawn(async move {
        FileDialog::new()
                .add_filter("valid_formats", &["json"])
                .save_file()
    });
    commands.spawn(ExportFileTask(task));
    next_state.set(state.get().next());
}
    /* Polls the async task for its completion to get the resulting file */
pub fn poll_for_export_file(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ExportFileTask)>,
    state: Res<State<FileExportState>>,
    mut next_state: ResMut<NextState<FileExportState>>,
) {
    println!("Export Polling");
    for (entity, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = future::block_on(
            future::poll_once(&mut selected_file.0)
        ) {
            info!("{:?}", result);
            if let Some(path_buf) = result {
                commands.spawn(SaveFile { path_buf });
                next_state.set(state.get().next());
            } else {
                next_state.set(state.get().reset());
            }
            commands.entity(entity).remove::<ExportFileTask>();
        }
    }
}

pub fn export_clean_up(
    mut commands: Commands,
    selected_file_query: Query<Entity, With<SaveFile>>,
    state: Res<State<FileExportState>>,
    mut next_state: ResMut<NextState<FileExportState>>,
) {
    let entity = selected_file_query
        .get_single()
        .expect("there should only be 1 selected file");
    commands.entity(entity).remove::<SaveFile>();
    next_state.set(state.get().next());
}
