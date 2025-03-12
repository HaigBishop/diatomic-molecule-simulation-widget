/*
Main library module for the WebAssembly simulation and plotting of diatomic molecules.

Contains:
 - Re-exports:
    - SimulationParameters struct from the sim module for use in JavaScript
 - Main function:
    - simulate_and_plot: orchestrates the simulation and plotting process
        - Takes simulation parameters and canvas IDs for energy and displacement plots
        - Runs the simulation using the sim module
        - Renders energy and displacement plots using the plt module
        - Returns simulation results to JavaScript for further use
*/

use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;

// Module for simulation
mod sim;
// Module for plotting
mod plt;

// Re-export the SimulationParameters struct to be used from JavaScript
pub use sim::SimulationParameters;

// Main simulation function called from JavaScript
#[wasm_bindgen]
pub fn simulate_and_plot(
    params: SimulationParameters,
    energy_canvas_id: &str,
    displacement_canvas_id: &str
) -> Result<JsValue, JsValue> {
    // 1. Run simulation based on parameters
    let result = sim::simulate_molecule(&params);
    
    // 2. Render energy plot
    plt::render_energy_plot(&result, energy_canvas_id)?;
    
    // 3. Render displacement plot
    plt::render_displacement_plot(&result, displacement_canvas_id)?;
    
    // 4. Return simulation data to JavaScript for animation
    Ok(to_value(&result)?)
}
