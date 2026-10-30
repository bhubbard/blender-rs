//! Blender Desktop GUI Window using eframe/egui (0.37+).
//! Renders native 3D Viewport with arcball orbit, Blender Outliner, Properties Inspector,
//! and real-time BMesh topology modification controls.

use crate::scene::SceneContext;
use blender_math::Vec3;
use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Ui};

pub struct BlenderAppGui {
    scene: SceneContext,
    primitive_type: String,
    subdivisions: usize,
    camera_yaw: f32,
    camera_pitch: f32,
    camera_dist: f32,
    wireframe: bool,
    show_grid: bool,
    fps_time: std::time::Instant,
    frame_count: u64,
    fps: f32,
    status_message: String,
}

impl BlenderAppGui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut scene = SceneContext::new("Scene");
        let _ = scene.create_default_cube();

        Self {
            scene,
            primitive_type: "cube".to_string(),
            subdivisions: 0,
            camera_yaw: 0.65,
            camera_pitch: 0.45,
            camera_dist: 4.5,
            wireframe: true,
            show_grid: true,
            fps_time: std::time::Instant::now(),
            frame_count: 0,
            fps: 60.0,
            status_message: "Ready. Safe Rust BMesh kernel initialized.".to_string(),
        }
    }

    fn rebuild_mesh(&mut self) {
        let t0 = std::time::Instant::now();
        let res = match self.primitive_type.as_str() {
            "sphere" => self.scene.create_uv_sphere(1.2, 16, 12),
            _ => self.scene.create_default_cube(),
        };

        if let Ok(()) = res {
            for _ in 0..self.subdivisions {
                let _ = self.scene.subdivide_edges(0.5);
            }
            let dur = t0.elapsed();
            self.status_message = format!(
                "Rebuilt {} (subdiv {}) in {:.2?} | Verts: {} | Edges: {} | Faces: {}",
                self.primitive_type,
                self.subdivisions,
                dur,
                self.scene.bmesh.totvert(),
                self.scene.bmesh.totedge(),
                self.scene.bmesh.totface(),
            );
        }
    }

    fn render_topbar(&mut self, ui: &mut Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    self.primitive_type = "cube".to_string();
                    self.subdivisions = 0;
                    self.rebuild_mesh();
                    ui.close();
                }
                if ui.button("Save As (.obj)...").clicked() {
                    let obj = blender_io::export_obj(&self.scene.bmesh);
                    let _ = std::fs::write("/tmp/blender_export.obj", obj);
                    self.status_message = "Exported /tmp/blender_export.obj".to_string();
                    ui.close();
                }
                ui.separator();
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });

            ui.menu_button("Add", |ui| {
                if ui.button("Cube").clicked() {
                    self.primitive_type = "cube".to_string();
                    self.rebuild_mesh();
                    ui.close();
                }
                if ui.button("UV Sphere").clicked() {
                    self.primitive_type = "sphere".to_string();
                    self.rebuild_mesh();
                    ui.close();
                }
            });

            ui.menu_button("Render", |ui| {
                if ui.button("Viewport Benchmark").clicked() {
                    let t0 = std::time::Instant::now();
                    for _ in 0..100 {
                        let _ = self.scene.create_default_cube();
                        let _ = self.scene.subdivide_edges(0.5);
                    }
                    self.status_message = format!("100 mesh rebuild cycles in {:.2?}", t0.elapsed());
                    ui.close();
                }
            });

            ui.separator();
            ui.label(egui::RichText::new("Blender 4.2.0 (Rust Edition)").strong().color(Color32::from_rgb(235, 120, 35)));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("FPS: {:.0}", self.fps));
            });
        });
    }

    fn render_viewport(&mut self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter_at(rect);
        // Dark viewport background (Blender default slate)
        painter.rect_filled(rect, 0.0, Color32::from_rgb(52, 54, 58));

        // Handle mouse drag for camera orbit
        let response = ui.allocate_rect(rect, egui::Sense::drag());
        if response.dragged() {
            let delta = response.drag_delta();
            self.camera_yaw += delta.x * 0.01;
            self.camera_pitch = (self.camera_pitch + delta.y * 0.01).clamp(-1.5, 1.5);
        }

        // Handle mouse scroll for zoom
        let scroll = ui.input(|i| i.smooth_scroll_delta.y);
        if rect.contains(ui.input(|i| i.pointer.hover_pos().unwrap_or_default())) && scroll != 0.0 {
            self.camera_dist = (self.camera_dist - scroll * 0.01).clamp(1.0, 20.0);
        }

        let center = rect.center();
        let scale = (rect.width().min(rect.height()) / 3.0) * (3.5 / self.camera_dist);

        let cos_y = self.camera_yaw.cos();
        let sin_y = self.camera_yaw.sin();
        let cos_p = self.camera_pitch.cos();
        let sin_p = self.camera_pitch.sin();

        let project = |v: Vec3| -> Pos2 {
            // Yaw rotation
            let x1 = v.x * cos_y - v.y * sin_y;
            let y1 = v.x * sin_y + v.y * cos_y;
            let z1 = v.z;

            // Pitch rotation
            let y2 = y1 * cos_p - z1 * sin_p;
            let z2 = y1 * sin_p + z1 * cos_p;

            // Perspective division
            let depth = 5.0 + y2;
            let f = if depth > 0.1 { 5.0 / depth } else { 1.0 };

            Pos2::new(center.x + x1 * scale * f, center.y - z2 * scale * f)
        };

        // Draw ground grid
        if self.show_grid {
            let grid_color = Color32::from_rgba_unmultiplied(85, 87, 92, 180);
            let grid_size = 4;
            for i in -grid_size..=grid_size {
                let f = i as f32;
                let p1 = project(Vec3::new(f, -grid_size as f32, 0.0));
                let p2 = project(Vec3::new(f, grid_size as f32, 0.0));
                painter.line_segment([p1, p2], Stroke::new(1.0, grid_color));

                let p3 = project(Vec3::new(-grid_size as f32, f, 0.0));
                let p4 = project(Vec3::new(grid_size as f32, f, 0.0));
                painter.line_segment([p3, p4], Stroke::new(1.0, grid_color));
            }

            // World axes: X (red), Y (green)
            let origin = project(Vec3::ZERO);
            let x_axis = project(Vec3::new(1.5, 0.0, 0.0));
            let y_axis = project(Vec3::new(0.0, 1.5, 0.0));
            painter.line_segment([origin, x_axis], Stroke::new(2.0, Color32::from_rgb(220, 60, 60)));
            painter.line_segment([origin, y_axis], Stroke::new(2.0, Color32::from_rgb(60, 200, 60)));
        }

        // Draw BMesh edges
        let wire_color = Color32::from_rgb(240, 240, 240);
        let lines = self.scene.extract_edges_lines();
        for (p_start, p_end) in lines {
            let s2d = project(p_start);
            let e2d = project(p_end);
            painter.line_segment([s2d, e2d], Stroke::new(1.5, wire_color));
        }

        // Overlay viewport stats in top-left
        let stats_rect = Rect::from_min_size(rect.min + egui::vec2(12.0, 12.0), egui::vec2(220.0, 80.0));
        painter.rect_filled(stats_rect, 4.0, Color32::from_black_alpha(160));
        let text_color = Color32::from_rgb(220, 220, 220);
        painter.text(
            stats_rect.min + egui::vec2(8.0, 8.0),
            egui::Align2::LEFT_TOP,
            format!("Mesh: {}", self.primitive_type),
            egui::FontId::proportional(13.0),
            text_color,
        );
        painter.text(
            stats_rect.min + egui::vec2(8.0, 26.0),
            egui::Align2::LEFT_TOP,
            format!("Vertices: {}", self.scene.bmesh.totvert()),
            egui::FontId::proportional(12.0),
            Color32::from_rgb(180, 200, 220),
        );
        painter.text(
            stats_rect.min + egui::vec2(8.0, 42.0),
            egui::Align2::LEFT_TOP,
            format!("Edges: {}", self.scene.bmesh.totedge()),
            egui::FontId::proportional(12.0),
            Color32::from_rgb(180, 200, 220),
        );
        painter.text(
            stats_rect.min + egui::vec2(8.0, 58.0),
            egui::Align2::LEFT_TOP,
            format!("Faces: {}", self.scene.bmesh.totface()),
            egui::FontId::proportional(12.0),
            Color32::from_rgb(180, 200, 220),
        );
    }
}

impl eframe::App for BlenderAppGui {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        // Track FPS
        self.frame_count += 1;
        let elapsed = self.fps_time.elapsed().as_secs_f32();
        if elapsed >= 0.5 {
            self.fps = self.frame_count as f32 / elapsed;
            self.frame_count = 0;
            self.fps_time = std::time::Instant::now();
        }

        // Custom Blender Dark Styling
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = Color32::from_rgb(40, 42, 45);
        visuals.window_fill = Color32::from_rgb(46, 48, 52);
        visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(32, 34, 37);
        ui.ctx().set_visuals(visuals);

        // Topbar
        egui::Panel::top("top_bar").show(ui, |ui| {
            self.render_topbar(ui);
        });

        // Bottom Status Bar
        egui::Panel::bottom("status_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&self.status_message).size(11.0).color(Color32::from_rgb(170, 175, 180)));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("Memory: 100% Rust Safe Heap").size(11.0).color(Color32::from_rgb(110, 180, 120)));
                });
            });
        });

        // Right Side: Outliner & Properties
        egui::Panel::right("properties_panel")
            .resizable(true)
            .default_size(280.0)
            .size_range(220.0..=450.0)
            .show(ui, |ui| {
                ui.heading("Outliner");
                ui.separator();
                egui::ScrollArea::vertical().id_salt("outliner_scroll").max_height(140.0).show(ui, |ui| {
                    ui.label("▾ 📁 Collection");
                    ui.indent("collection_indent", |ui| {
                        ui.label("  📷 Camera");
                        ui.label(format!("  📦 {} (Mesh)", self.primitive_type));
                        ui.label("  💡 Point Light");
                    });
                });

                ui.add_space(8.0);
                ui.heading("Properties & Modifiers");
                ui.separator();

                ui.collapsing("Mesh Primitive", |ui| {
                    let mut prim_changed = false;
                    ui.horizontal(|ui| {
                        if ui.selectable_label(self.primitive_type == "cube", "Cube").clicked() {
                            self.primitive_type = "cube".to_string();
                            prim_changed = true;
                        }
                        if ui.selectable_label(self.primitive_type == "sphere", "UV Sphere").clicked() {
                            self.primitive_type = "sphere".to_string();
                            prim_changed = true;
                        }
                    });
                    if prim_changed {
                        self.rebuild_mesh();
                    }
                });

                ui.collapsing("Subdivision Surface", |ui| {
                    let prev_sub = self.subdivisions;
                    ui.add(egui::Slider::new(&mut self.subdivisions, 0..=3).text("Levels Viewport"));
                    if self.subdivisions != prev_sub {
                        self.rebuild_mesh();
                    }
                });

                ui.collapsing("Viewport Display", |ui| {
                    ui.checkbox(&mut self.wireframe, "Wireframe Overlay");
                    ui.checkbox(&mut self.show_grid, "Floor Grid");
                    if ui.button("Reset Camera View").clicked() {
                        self.camera_yaw = 0.65;
                        self.camera_pitch = 0.45;
                        self.camera_dist = 4.5;
                    }
                });

                ui.collapsing("Wavefront Export", |ui| {
                    if ui.button("Export OBJ to /tmp/mesh.obj").clicked() {
                        let obj_content = blender_io::export_obj(&self.scene.bmesh);
                        let path = "/tmp/blender_app_mesh.obj";
                        if let Ok(()) = std::fs::write(path, obj_content) {
                            self.status_message = format!("Exported OBJ to {}", path);
                        }
                    }
                });
            });

        // Center Viewport
        egui::CentralPanel::default().show(ui, |ui| {
            let available = ui.available_rect_before_wrap();
            self.render_viewport(ui, available);
        });

        // Keep continuous repaint for smooth interaction
        ui.ctx().request_repaint();
    }
}
