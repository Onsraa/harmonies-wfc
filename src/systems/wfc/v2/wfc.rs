use crate::components::grid::hex::Hex;
use crate::systems::grid::tile::visualisation::UpdateGridVisuals;
use crate::wfc::v2::controller::WfcController;
use crate::wfc::v2::grid::WfcHexGrid;
use crate::wfc::v2::WfcError;
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct WfcTask {
    task: Task<Result<(), WfcError>>,
    shared_grid: Arc<Mutex<Option<WfcHexGrid>>>,
    is_complete: Arc<Mutex<bool>>,
    update_signal: Arc<Mutex<bool>>, // Signal pour déclencher la mise à jour
}

#[derive(Resource)]
pub struct WfcSharedState {
    pub grid: Arc<Mutex<Option<WfcHexGrid>>>,
    pub is_complete: Arc<Mutex<bool>>,
    pub update_signal: Arc<Mutex<bool>>,
}

#[derive(Resource)]
pub struct WfcStatus {
    pub has_run: bool,
    pub auto_restart: bool,
}

impl Default for WfcStatus {
    fn default() -> Self {
        Self {
            has_run: false,
            auto_restart: false, // Set to true if you want auto-restart behavior
        }
    }
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

    // Don't auto-restart unless explicitly enabled
    if wfc_status.has_run && !wfc_status.auto_restart {
        return;
    }

    let hex_vec = hexes.iter().copied().collect::<Vec<_>>();
    if hex_vec.is_empty() {
        return;
    }

    println!("Starting WFC async task with {} hexes", hex_vec.len());
    println!("Press R to restart WFC after completion");

    let shared_grid = Arc::new(Mutex::new(None));
    let is_complete = Arc::new(Mutex::new(false));
    let update_signal = Arc::new(Mutex::new(false));

    let shared_grid_clone = Arc::clone(&shared_grid);
    let is_complete_clone = Arc::clone(&is_complete);
    let update_signal_clone = Arc::clone(&update_signal);

    let task_pool = AsyncComputeTaskPool::get();
    let task = task_pool.spawn(async move {
        let mut controller = WfcController::from_hexes(hex_vec);
        let mut steps = 0;
        let start_time = std::time::Instant::now();

        loop {
            match controller.step() {
                Ok(_) => {
                    steps += 1;

                    const UPDATE_EVERY_N_STEPS: usize = 1000;

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

                    // Share final state
                    if let Ok(mut grid_guard) = shared_grid_clone.try_lock() {
                        *grid_guard = Some(controller.grid);
                    }
                    if let Ok(mut complete_guard) = is_complete_clone.try_lock() {
                        *complete_guard = true;
                    }
                    // Signal final update
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

    // Mark that we've started a task
    wfc_status.has_run = true;

    commands.insert_resource(WfcSharedState {
        grid: Arc::clone(&shared_grid),
        is_complete: Arc::clone(&is_complete),
        update_signal: Arc::clone(&update_signal),
    });

    commands.insert_resource(WfcTask {
        task,
        shared_grid,
        is_complete,
        update_signal,
    });
}

pub fn check_wfc_progress(
    mut commands: Commands,
    wfc_task: Option<ResMut<WfcTask>>,
    wfc_shared: Option<Res<WfcSharedState>>,
    mut update_grid_event: EventWriter<UpdateGridVisuals>,
) {
    let Some(mut task_resource) = wfc_task else {
        return;
    };

    if let Some(shared_state) = wfc_shared.as_ref() {
        if let Ok(mut signal_guard) = shared_state.update_signal.try_lock() {
            if *signal_guard {
                update_grid_event.write(UpdateGridVisuals);
                *signal_guard = false;
            }
        }
    }

    // Vérifier si la tâche est terminée
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
    }
}
