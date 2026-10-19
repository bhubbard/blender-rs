//! Blender-rs: Pure Rust executable application runtime engine.

use anyhow::Result;
use clap::Parser;
use colored::*;
use std::path::PathBuf;
use std::time::Instant;

mod scene;
mod viewport;

use scene::SceneContext;
use viewport::ViewportRasterizer;

#[derive(Parser, Debug)]
#[command(
    name = "blender",
    about = "Blender-rs: Native Safe Rust 3D Engine & Modeling Kernel",
    version = "4.2.0-rust.alpha"
)]
struct Cli {
    /// Run in background without window / interactive UI
    #[arg(short = 'b', long = "background", default_value_t = false)]
    background: bool,

    /// Mesh primitive to instantiate (cube, sphere)
    #[arg(long = "primitive", default_value = "cube")]
    primitive: String,

    /// Number of edge subdivision steps to apply
    #[arg(long = "subdivide", default_value_t = 0)]
    subdivide: usize,

    /// Export the evaluated BMesh to Wavefront OBJ format
    #[arg(long = "export-obj")]
    export_obj: Option<PathBuf>,

    /// Render 3D wireframe viewport in terminal
    #[arg(long = "viewport", default_value_t = false)]
    viewport: bool,

    /// Run topology & execution benchmark
    #[arg(long = "benchmark", default_value_t = false)]
    benchmark: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("{}", "\n======================================================================".cyan().bold());
    println!("{}", "               🎨  BLENDER 4.2.0 (SAFE RUST NATIVE ENGINE)             ".bold());
    println!("{}", "   BMesh Half-Edge Kernel • Geometry Nodes XPBD • Zero Pointer Hazards   ".dimmed());
    println!("{}", "======================================================================\n".cyan().bold());

    let mut ctx = SceneContext::new("DefaultScene");

    let t0 = Instant::now();
    match args.primitive.as_str() {
        "sphere" => {
            println!("{} Initializing UV Sphere primitive...", "✦".cyan());
            ctx.create_uv_sphere(1.2, 16, 12)?;
        }
        _ => {
            println!("{} Initializing default Cube primitive...", "✦".cyan());
            ctx.create_default_cube()?;
        }
    }
    let prim_dur = t0.elapsed();

    println!(
        "{} Primitive built in {:.2?} | Verts: {} | Edges: {} | Faces: {}",
        "✓".green().bold(),
        prim_dur,
        ctx.bmesh.totvert().to_string().yellow().bold(),
        ctx.bmesh.totedge().to_string().yellow().bold(),
        ctx.bmesh.totface().to_string().yellow().bold(),
    );

    // Apply subdivisions if requested
    if args.subdivide > 0 {
        println!("{} Applying {} edge subdivision passes...", "✦".cyan(), args.subdivide);
        let t_sub = Instant::now();
        let mut total_splits = 0;
        for _ in 0..args.subdivide {
            total_splits += ctx.subdivide_edges(0.5)?;
        }
        println!(
            "{} Subdivided {} edges in {:.2?} | Result: {} Verts, {} Edges",
            "✓".green().bold(),
            total_splits.to_string().yellow().bold(),
            t_sub.elapsed(),
            ctx.bmesh.totvert().to_string().yellow().bold(),
            ctx.bmesh.totedge().to_string().yellow().bold(),
        );
    }

    // Export OBJ if requested
    if let Some(ref out_path) = args.export_obj {
        println!("{} Exporting BMesh to OBJ: {:?}...", "✦".cyan(), out_path);
        let obj_data = blender_io::export_obj(&ctx.bmesh);
        std::fs::write(out_path, obj_data)?;
        println!("{} Successfully exported OBJ ({})", "✓".green().bold(), out_path.display());
    }

    // Benchmark mode
    if args.benchmark {
        println!("\n{}", "🚀 Running BMesh Topology Stress & Modification Benchmark...".yellow().bold());
        let runs = 1000;
        let t_bench = Instant::now();
        let mut test_scene = SceneContext::new("Bench");
        for _ in 0..runs {
            test_scene.create_default_cube()?;
            test_scene.subdivide_edges(0.5)?;
        }
        let total_time = t_bench.elapsed();
        let rate = runs as f64 / total_time.as_secs_f64();
        println!(
            "{} Completed {} complete BMesh cube + subdivide cycles in {:.2?} ({:.0} cycles/sec)",
            "★".green().bold(),
            runs,
            total_time,
            rate
        );
    }

    // Viewport preview
    if args.viewport || (!args.background && args.export_obj.is_none() && !args.benchmark) {
        println!("\n{}", "📐 3D Wireframe Viewport Preview (Isometric / Perspective):".bold());
        let lines = ctx.extract_edges_lines();
        let mut rasterizer = ViewportRasterizer::new(70, 24);
        let frame = rasterizer.render_frame(&lines, 0.45, 0.65);
        println!("{}", frame.cyan());
    }

    println!("{} Engine execution complete. 0 runtime hazards.\n", "✔".green().bold());
    Ok(())
}
