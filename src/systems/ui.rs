use crate::components::grid::tile::tile_type::TileType;
use crate::globals::MAX_HEIGHT;
use crate::resources::tile_weights::TileWeights;
use crate::systems::menu::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui::SliderClamping;

/// Condition pour vérifier si l'UI doit être mise à jour
pub fn should_update_ui(tile_weights: Res<TileWeights>) -> bool {
    tile_weights.show_ui
}

/// Condition pour vérifier si l'aide doit être affichée
pub fn should_show_help(keyboard: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard.pressed(KeyCode::KeyP)
}

/// Système pour l'interface utilisateur de gestion des poids
pub fn tile_weights_ui_system(
    mut contexts: EguiContexts,
    mut tile_weights: ResMut<TileWeights>,
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
) {
    match state.get() {
        GameState::InGame => {}
        _ => return,
    }

    // Toggle UI avec Tab
    if keyboard.just_pressed(KeyCode::Tab) {
        tile_weights.show_ui = !tile_weights.show_ui;
    }

    if !tile_weights.show_ui {
        return;
    }

    egui::Window::new("Gestion des poids des tuiles")
        .default_pos([10.0, 10.0])
        .default_width(400.0)
        .scroll(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Probabilités d'apparition");

            ui.separator();
            ui.label("Ajustez les poids pour chaque type de tuile par niveau.");
            ui.label("Plus le poids est élevé, plus la tuile apparaîtra souvent.");

            // Pour chaque niveau
            for height in 0..MAX_HEIGHT {
                ui.separator();
                ui.heading(format!("Niveau {}", height));

                ui.horizontal(|ui| {
                    if ui
                        .button("Normaliser")
                        .on_hover_text("Ajuste les poids pour que la somme = 100")
                        .clicked()
                    {
                        tile_weights.normalize_weights(height);
                    }
                });

                // Détermine quelles tuiles sont disponibles à ce niveau
                let tiles = match height {
                    0 => vec![
                        (TileType::City, "Ville", egui::Color32::from_rgb(255, 0, 0)),
                        (
                            TileType::River,
                            "Rivière",
                            egui::Color32::from_rgb(0, 128, 255),
                        ),
                        (
                            TileType::Rock,
                            "Roche",
                            egui::Color32::from_rgb(128, 128, 128),
                        ),
                        (
                            TileType::Trunk,
                            "Tronc",
                            egui::Color32::from_rgb(102, 51, 0),
                        ),
                        (
                            TileType::Field,
                            "Champs",
                            egui::Color32::from_rgb(255, 255, 0),
                        ),
                    ],
                    _ => vec![
                        (TileType::City, "Ville", egui::Color32::from_rgb(255, 0, 0)),
                        (
                            TileType::Rock,
                            "Roche",
                            egui::Color32::from_rgb(128, 128, 128),
                        ),
                        (
                            TileType::Trunk,
                            "Tronc",
                            egui::Color32::from_rgb(102, 51, 0),
                        ),
                        (
                            TileType::Leaves,
                            "Feuilles",
                            egui::Color32::from_rgb(0, 204, 0),
                        ),
                        (
                            TileType::Empty,
                            "Vide",
                            egui::Color32::from_rgb(200, 200, 200),
                        ),
                    ],
                };

                // Affiche un slider pour chaque tuile
                egui::Grid::new(format!("weights_grid_{}", height))
                    .num_columns(2)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        for (tile_type, name, color) in tiles {
                            ui.horizontal(|ui| {
                                // Carré coloré
                                let (rect, _) = ui.allocate_exact_size(
                                    egui::Vec2::new(16.0, 16.0),
                                    egui::Sense::hover(),
                                );
                                ui.painter().rect_filled(rect, 2.0, color);

                                ui.label(name);
                            });

                            let weight = tile_weights.get_weight_mut(tile_type, height);
                            ui.add(
                                egui::Slider::new(weight, 0.1..=100.0)
                                    .suffix("%")
                                    .clamping(SliderClamping::Always),
                            );
                            ui.end_row();
                        }
                    });
            }

            ui.separator();

            // Boutons d'action
            ui.horizontal(|ui| {
                if ui.button("🎲 Générer").clicked() {}

                if ui
                    .button("🔄 Réinitialiser")
                    .on_hover_text("Remettre les poids par défaut")
                    .clicked()
                {
                    tile_weights.reset_to_default();
                }

                ui.label("Tab pour masquer");
            });

            // Conseils
            ui.separator();
            ui.collapsing("Conseils", |ui| {
                ui.label(
                    "• Augmentez le poids des rivières au niveau 0 pour avoir plus de cours d'eau",
                );
                ui.label("• Réduisez le poids 'Vide' pour des constructions plus denses");
                ui.label("• Les poids sont relatifs : seul le rapport entre eux compte");
            });
        });
}

/// Système pour afficher les contrôles
pub fn controls_help_ui_system(
    mut contexts: EguiContexts,
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
) {
    match state.get() {
        GameState::InGame => {}
        _ => return,
    }

    if !keyboard.pressed(KeyCode::KeyP) {
        return;
    }

    egui::Window::new("Contrôles")
        .anchor(egui::Align2::RIGHT_TOP, [-10.0, 10.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Commandes");
            ui.separator();

            ui.label("🎮 Caméra:");
            ui.label("  • WASD : Déplacer");
            ui.label("  • Clic droit + souris : Tourner");
            ui.label("  • Molette : Zoom");

            ui.separator();
            ui.label("⌨️ Raccourcis:");
            ui.label("  • G : Nouvelle génération");
            ui.label("  • Tab : Interface des poids");
            ui.label("  • F1 : Cette aide");
        });
}
