extern crate quote;
extern crate serde;

use std::fs;
use std::path::Path;
use quote::quote;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VecTrajectory {
    pub total_time: f32,
    pub step_time: f32,
    pub points: Vec<(f32, f32, f32, f32, f32)>
}

fn main() {
    let trajectory: VecTrajectory = serde_json::from_str(fs::read_to_string("trajectories/trajectory.json").unwrap().as_str()).unwrap();

    let trajectory_total_time = trajectory.total_time;
    let trajectory_step_time = trajectory.step_time;
    let trajectory_len = trajectory.points.len();
    let trajectory_points = trajectory.points;

    let points_code = trajectory_points.iter().map(|(a, b, c, d, e)| {
        quote! { (#a, #b, #c, #d, #e) }
    });
    
    let code = quote! {
        use crate::trajectory::Trajectory; 
        
        pub static TRAJECTORY: Trajectory<#trajectory_len> = Trajectory {
            total_time: #trajectory_total_time,
            step_time: #trajectory_step_time,
            points: [
                #(#points_code),*
            ]
        };
    };

    // Write the generated code to src/trajectories.rs
    let dest_path = Path::new("src/trajectories.rs");
    fs::write(dest_path, code.to_string()).expect("Failed to write trajectories.rs");

    println!("cargo:rerun-if-changed=trajectories/*");
}
