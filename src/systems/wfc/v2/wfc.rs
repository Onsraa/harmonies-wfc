use crate::components::grid::hex::Hex;
use crate::systems::grid::tile::visualisation::{ResetGridVisuals, UpdateGridVisuals};
use crate::wfc::v2::controller::WfcController;
use crate::wfc::v2::grid::WfcHexGrid;
use crate::wfc::v2::WfcError;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct WfcSharedState {
    pub grid: Arc<Mutex<Option<WfcHexGrid>>>,
}

#[derive(Resource)]
pub struct WfcTask {
    task: Task<Result<(), WfcError>>,
    update_signal: Arc<Mutex<bool>>,
    reset_signal: Arc<Mutex<bool>>,
}

#[derive(Resource)]
pub struct WfcStatus {
    pub should_start: bool,
    pub is_running: bool,
    pub auto_restart: bool,
    pub should_reset: bool,
}

impl Default for WfcStatus {
    fn default() -> Self {
        Self {
            should_start: false,
            is_running: false,
            auto_restart: false,
            should_reset: false,
        }
    }
}

pub fn start_wfc_task(
    mut commands: Commands,
    mut wfc_status: ResMut<WfcStatus>,
    mut reset_grid_visuals: EventWriter<ResetGridVisuals>,
) {
    println!("Starting WFC task...");
    wfc_status.should_start = true;

    // Remove existing resources to ensure clean state
    commands.remove_resource::<WfcTask>();
    commands.remove_resource::<WfcSharedState>();

    // Reset visuals first
    reset_grid_visuals.write(ResetGridVisuals);
}

pub fn setup_wfc_task_when_ready(
    mut commands: Commands,
    hexes: Query<&Hex>,
    wfc_task: Option<Res<WfcTask>>,
    mut wfc_status: ResMut<WfcStatus>,
) {
    // Don't start if task already exists
    if wfc_task.is_some() {
        return;
    }

    // Only start if we should start
    if !wfc_status.should_start {
        return;
    }

    let hex_vec = hexes.iter().copied().collect::<Vec<_>>();
    if hex_vec.is_empty() {
        println!("No hexes found, cannot start WFC");
        return;
    }

    println!("Starting WFC async task with {} hexes", hex_vec.len());

    // Reset flags
    wfc_status.should_start = false;
    wfc_status.is_running = true;

    let shared_grid = Arc::new(Mutex::new(None));
    let update_signal = Arc::new(Mutex::new(false));

    let shared_grid_clone = Arc::clone(&shared_grid);
    let update_signal_clone = Arc::clone(&update_signal);

    let task_pool = AsyncComputeTaskPool::get();
    let task = task_pool.spawn(async move {
        let mut controller = WfcController::from_hexes(hex_vec.clone());

        let mut steps = 0;
        let start_time = std::time::Instant::now();
        const UPDATE_EVERY_N_STEPS: usize = 10;

        loop {
            match controller.step() {
                Ok(_) => {
                    steps += 1;

                    if steps % UPDATE_EVERY_N_STEPS == 0 {
                        if let Ok(mut grid_guard) = shared_grid_clone.try_lock() {
                            *grid_guard = Some(controller.grid.clone());
                        }

                        if let Ok(mut signal_guard) = update_signal_clone.try_lock() {
                            *signal_guard = true;
                        }
                    }
                }
                Err(WfcError::Complete) => {
                    println!(
                        "WFC completed after {} steps in {:.2}s",
                        steps,
                        start_time.elapsed().as_secs_f32()
                    );

                    // Final update with complete grid
                    if let Ok(mut grid_guard) = shared_grid_clone.try_lock() {
                        *grid_guard = Some(controller.grid);
                    }

                    if let Ok(mut signal_guard) = update_signal_clone.try_lock() {
                        *signal_guard = true;
                    }

                    return Ok(());
                }
                Err(e) => {
                    eprintln!("WFC Error after {} steps: {:?}", steps, e);
                    return Err(e);
                }
            }
        }
    });

    // Insert shared state for visualization
    commands.insert_resource(WfcSharedState {
        grid: Arc::clone(&shared_grid),
    });

    // Insert task resource
    commands.insert_resource(WfcTask {
        task,
        update_signal,
        reset_signal: Arc::new(Mutex::new(false)),
    });
}

pub fn check_wfc_progress(
    mut commands: Commands,
    mut wfc_task: Option<ResMut<WfcTask>>,
    mut update_grid_event: EventWriter<UpdateGridVisuals>,
    mut wfc_status: ResMut<WfcStatus>,
) {
    let Some(mut task_resource) = wfc_task else {
        return;
    };

    // Check for updates
    if let Ok(mut signal_guard) = task_resource.update_signal.try_lock() {
        if *signal_guard {
            update_grid_event.write(UpdateGridVisuals);
            *signal_guard = false;
        }
    }

    // Check if task is complete
    if let Some(result) = future::block_on(future::poll_once(&mut task_resource.task)) {
        match result {
            Ok(_) => {
                println!("WFC async task completed successfully!");
                update_grid_event.write(UpdateGridVisuals);
            }
            Err(e) => {
                eprintln!("WFC async task failed: {:?}", e);
            }
        }

        commands.remove_resource::<WfcTask>();
        wfc_status.is_running = false;

        if wfc_status.auto_restart {
            wfc_status.should_start = true;
        }
    }
}

pub fn cancel_wfc_task(mut commands: Commands, mut wfc_status: ResMut<WfcStatus>) {
    println!("Cancelling WFC task...");
    commands.remove_resource::<WfcTask>();
    wfc_status.is_running = false;
    wfc_status.should_start = false;
}

#[derive(Event)]
pub struct ResetGrid;

pub fn reset_grid_system(
    mut commands: Commands,
    mut reset_grid_events: EventReader<ResetGrid>,
    mut reset_grid_visuals: EventWriter<ResetGridVisuals>,
    mut wfc_status: ResMut<WfcStatus>,
    wfc_task: Option<ResMut<WfcTask>>,
) {
    for _ in reset_grid_events.read() {
        println!("Resetting grid system...");

        // Clean up all WFC resources
        commands.remove_resource::<WfcTask>();
        commands.remove_resource::<WfcSharedState>();

        // Reset status
        wfc_status.should_start = false;
        wfc_status.is_running = false;
        wfc_status.should_reset = true;

        // Trigger visual reset
        reset_grid_visuals.write(ResetGridVisuals);
    }
}

pub fn trigger_grid_reset(mut reset_events: EventWriter<ResetGrid>) {
    reset_events.write(ResetGrid);
}
