//! Blender Automated UI Test Suite
//! Ported directly from the upstream Blender UI testing specifications:
//! https://developer.blender.org/docs/handbook/testing/ui/
//!
//! Enforces Zero Python Policy (100% Pure Safe Rust execution).
//! Tests:
//! 1. `test_ui_workspaces`: All 11 canonical workspace transitions and modes.
//! 2. `test_ui_panels_properties`: All 8 Properties Editor tabs and modifier stack.
//! 3. `test_ui_outliner_hierarchy`: Object selection and visibility toggling.
//! 4. `test_ui_timeline_transport`: Play, pause, scrub, step, and frame clamping.
//! 5. `test_ui_navigation_views`: Top, Front, and Right camera alignment snapping.
//! 6. `test_ui_tools_shelf`: T-shelf tools and 3D cursor placement.
//! 7. `test_ui_operators_primitive_generation`: Adding Cube, UV Sphere, Cylinder, Plane, Torus.
//! 8. `test_ui_sculpt_brush`: Dynamic BMesh vertex deformation.
//! 9. `test_ui_scripting_execution`: Python/Rust console runner.
//! 10. `test_ui_headless_frame_pipeline`: Full headless egui render pass verification.

use blender_app::{
    ActiveTool, BlenderAppGui, EditMode, PropertyTab, ShadingMode,
};
use blender_math::Vec3;
use eframe::egui;

#[test]
fn test_ui_workspaces() {
    let mut gui = BlenderAppGui::new_headless();

    // 1. Initial workspace should be Layout
    assert_eq!(gui.active_workspace(), "Layout");
    assert_eq!(gui.edit_mode(), EditMode::Object);

    // 2. Modeling workspace switches to Edit mode
    gui.set_workspace("Modeling");
    assert_eq!(gui.active_workspace(), "Modeling");
    assert_eq!(gui.edit_mode(), EditMode::Edit);

    // 3. Sculpting workspace switches to Sculpt mode
    gui.set_workspace("Sculpting");
    assert_eq!(gui.active_workspace(), "Sculpting");
    assert_eq!(gui.edit_mode(), EditMode::Sculpt);
    assert_eq!(gui.shading_mode(), ShadingMode::Solid);

    // 4. Shading workspace switches to Material Preview
    gui.set_workspace("Shading");
    assert_eq!(gui.active_workspace(), "Shading");
    assert_eq!(gui.shading_mode(), ShadingMode::MaterialPreview);

    // 5. Test remaining standard workspaces
    let all_workspaces = [
        "UV Editing",
        "Texture Paint",
        "Animation",
        "Rendering",
        "Compositing",
        "Geometry Nodes",
        "Scripting",
        "Layout",
    ];
    for ws in all_workspaces {
        gui.set_workspace(ws);
        assert_eq!(gui.active_workspace(), ws);
    }
}

#[test]
fn test_ui_panels_properties() {
    let mut gui = BlenderAppGui::new_headless();

    // Verify all 8 property category tabs can be set and inspected
    let tabs = [
        PropertyTab::Render,
        PropertyTab::Output,
        PropertyTab::Scene,
        PropertyTab::World,
        PropertyTab::Object,
        PropertyTab::Modifiers,
        PropertyTab::Data,
        PropertyTab::Material,
    ];
    for tab in tabs {
        gui.set_property_tab(tab);
        assert_eq!(gui.active_prop_tab(), tab);
    }

    // Test Object Transform updates
    gui.set_property_tab(PropertyTab::Object);
    let new_pos = Vec3::new(1.5, -2.0, 3.25);
    let new_rot = Vec3::new(45.0, 90.0, 0.0);
    let new_scale = Vec3::new(2.0, 2.0, 2.0);
    gui.set_object_transform(new_pos, new_rot, new_scale);
    assert_eq!(gui.object_pos(), new_pos);
    assert_eq!(gui.object_rot(), new_rot);
    assert_eq!(gui.object_scale(), new_scale);

    // Test Material Color updates
    gui.set_property_tab(PropertyTab::Material);
    gui.set_material_color([0.9, 0.2, 0.1]);
    assert_eq!(gui.material_color(), [0.9, 0.2, 0.1]);

    // Test Modifier Stack: Subdivision Surface
    gui.set_property_tab(PropertyTab::Modifiers);
    assert_eq!(gui.subdivisions(), 0);
    assert_eq!(gui.scene().bmesh.totvert(), 8);

    gui.set_subdivisions(1);
    assert_eq!(gui.subdivisions(), 1);
    assert!(gui.scene().bmesh.totvert() > 8);
}

#[test]
fn test_ui_outliner_hierarchy() {
    let mut gui = BlenderAppGui::new_headless();

    // Default selection
    assert_eq!(gui.selected_object(), "Cube");
    assert!(gui.is_object_visible("Cube"));
    assert!(gui.is_object_visible("Camera"));
    assert!(gui.is_object_visible("Light"));

    // Select Camera
    gui.select_object("Camera");
    assert_eq!(gui.selected_object(), "Camera");

    // Select Light
    gui.select_object("Light");
    assert_eq!(gui.selected_object(), "Light");

    // Toggle Camera visibility
    gui.toggle_object_visibility("Camera");
    assert!(!gui.is_object_visible("Camera"));
    gui.toggle_object_visibility("Camera");
    assert!(gui.is_object_visible("Camera"));

    // Toggle Cube visibility
    gui.toggle_object_visibility("Cube");
    assert!(!gui.is_object_visible("Cube"));
}

#[test]
fn test_ui_timeline_transport() {
    let mut gui = BlenderAppGui::new_headless();

    // Initial frame
    assert_eq!(gui.current_frame(), 1);
    assert!(!gui.is_playing());

    // Play/Pause
    gui.timeline_play_pause();
    assert!(gui.is_playing());
    gui.timeline_play_pause();
    assert!(!gui.is_playing());

    // Step Forward
    gui.timeline_step(10);
    assert_eq!(gui.current_frame(), 11);

    // Step Back
    gui.timeline_step(-5);
    assert_eq!(gui.current_frame(), 6);

    // Clamping to lower bound
    gui.timeline_step(-100);
    assert_eq!(gui.current_frame(), 1);

    // Jump to End
    gui.timeline_jump_end();
    assert_eq!(gui.current_frame(), 250);

    // Clamping to upper bound
    gui.timeline_step(50);
    assert_eq!(gui.current_frame(), 250);

    // Jump to Start
    gui.timeline_jump_start();
    assert_eq!(gui.current_frame(), 1);
}

#[test]
fn test_ui_navigation_views() {
    let mut gui = BlenderAppGui::new_headless();

    // Front View (Numpad 1)
    gui.set_camera_view("front");
    assert_eq!(gui.camera_yaw(), 0.0);
    assert_eq!(gui.camera_pitch(), 0.0);

    // Right View (Numpad 3)
    gui.set_camera_view("right");
    assert!((gui.camera_yaw() - std::f32::consts::FRAC_PI_2).abs() < 1e-4);
    assert_eq!(gui.camera_pitch(), 0.0);

    // Top View (Numpad 7)
    gui.set_camera_view("top");
    assert_eq!(gui.camera_yaw(), 0.0);
    assert!(gui.camera_pitch() > 1.5);
}

#[test]
fn test_ui_tools_shelf() {
    let mut gui = BlenderAppGui::new_headless();

    let all_tools = [
        ActiveTool::SelectBox,
        ActiveTool::Cursor,
        ActiveTool::Move,
        ActiveTool::Rotate,
        ActiveTool::Scale,
        ActiveTool::Transform,
        ActiveTool::Annotate,
        ActiveTool::Measure,
        ActiveTool::AddCube,
    ];
    for tool in all_tools {
        gui.set_tool(tool);
        assert_eq!(gui.active_tool(), tool);
    }
}

#[test]
fn test_ui_operators_primitive_generation() {
    let mut gui = BlenderAppGui::new_headless();

    // 1. Cube
    gui.add_primitive_mesh("cube");
    assert_eq!(gui.scene().bmesh.totvert(), 8);
    assert_eq!(gui.scene().bmesh.totface(), 6);

    // 2. UV Sphere
    gui.add_primitive_mesh("sphere");
    assert!(gui.scene().bmesh.totvert() > 50);
    assert!(gui.scene().bmesh.totface() > 50);

    // 3. Cylinder
    gui.add_primitive_mesh("cylinder");
    assert_eq!(gui.scene().bmesh.totvert(), 34); // 16 top + 16 bot + 2 centers
    assert_eq!(gui.scene().bmesh.totface(), 48); // 16 sides + 16 top + 16 bot

    // 4. Plane
    gui.add_primitive_mesh("plane");
    assert_eq!(gui.scene().bmesh.totvert(), 4);
    assert_eq!(gui.scene().bmesh.totface(), 1);

    // 5. Torus
    gui.add_primitive_mesh("torus");
    assert_eq!(gui.scene().bmesh.totvert(), 240); // 20 * 12
    assert_eq!(gui.scene().bmesh.totface(), 240);
}

#[test]
fn test_ui_sculpt_brush() {
    let mut gui = BlenderAppGui::new_headless();
    gui.add_primitive_mesh("sphere");

    let count = gui.sculpt(Vec3::new(0.0, 0.0, 1.2), 0.8, 0.2);
    assert!(count > 0, "Sculpt brush must deform vertices within radius");
}

#[test]
fn test_ui_scripting_execution() {
    let mut gui = BlenderAppGui::new_headless();
    let script = "import bpy\nbpy.ops.mesh.primitive_cube_add()";
    gui.run_script(script);
    assert!(gui.script_output().contains("Executed script"));
    assert!(gui.script_output().contains("0 errors"));
}

#[test]
fn test_ui_headless_frame_pipeline() {
    let mut gui = BlenderAppGui::new_headless();
    let ctx = egui::Context::default();

    // Render frame across multiple workspace layouts to ensure zero panics
    let workspaces = ["Layout", "Shading", "UV Editing", "Modeling", "Scripting"];
    for ws in workspaces {
        gui.set_workspace(ws);
        gui.test_render_frame(&ctx);
    }
}
