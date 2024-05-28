use super::{ExportFileEvent, FileState, ImportFileEvent, SelectedFileTask};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::tasks::futures_lite::future;
use bevy::tasks::AsyncComputeTaskPool;
use rfd::FileDialog;
use std::path::PathBuf;

pub trait FileDialogOpener {
    fn open(dialog: FileDialog) -> Option<PathBuf>;

    fn file_state() -> FileState;
}

pub struct ImportFile;
pub struct ExportFile;

impl FileDialogOpener for ImportFile {
    fn open(dialog: FileDialog) -> Option<PathBuf> {
        dialog.pick_file()
    }

    fn file_state() -> FileState {
        FileState::Import
    }
}

impl FileDialogOpener for ExportFile {
    fn open(dialog: FileDialog) -> Option<PathBuf> {
        dialog.save_file()
    }

    fn file_state() -> FileState {
        FileState::Export
    }
}

pub fn open_file_dialog<F: FileDialogOpener>(
    mut commands: Commands,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut mouse_wheel: ResMut<Events<MouseWheel>>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    mut next_file_state: ResMut<NextState<FileState>>,
) {
    mouse.reset_all();
    mouse_wheel.clear();
    keyboard.reset_all();

    let thread_pool = AsyncComputeTaskPool::get();
    let task = thread_pool
        .spawn(async move { F::open(FileDialog::new().add_filter("valid_formats", &["json"])) });
    commands.insert_resource(SelectedFileTask(task));

    next_file_state.set(F::file_state());
}

pub fn poll_for_selected_file(
    mut task: ResMut<SelectedFileTask>,
    state: Res<State<FileState>>,
    mut next_state: ResMut<NextState<FileState>>,
    mut export_file_writer: EventWriter<ExportFileEvent>,
    mut import_file_writer: EventWriter<ImportFileEvent>,
) {
    if let Some(result) = future::block_on(future::poll_once(&mut **task)) {
        if let Some(path_buf) = result {
            match **state {
                FileState::Import => {
                    import_file_writer.send(ImportFileEvent(path_buf));
                }
                FileState::Export => {
                    export_file_writer.send(ExportFileEvent(path_buf));
                }
                _ => unreachable!(),
            }
        }

        next_state.set(FileState::Inactive);
    }
}

pub fn cleanup_file_dialog(
    mut commands: Commands,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut mouse_wheel: ResMut<Events<MouseWheel>>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
) {
    commands.remove_resource::<SelectedFileTask>();
    mouse.reset_all();
    mouse_wheel.clear();
    keyboard.reset_all();
}
