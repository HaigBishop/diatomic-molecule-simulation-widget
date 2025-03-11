mod sim;
mod plt;

use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Re-export the SimulationParameters struct
pub use sim::SimulationParameters;

// Main simulation function
#[wasm_bindgen]
pub fn simulate_and_plot(
    params: SimulationParameters,
    energy_canvas_id: &str,
    displacement_canvas_id: &str//,
    // animation_canvas_id: &str
) -> Result<JsValue, JsValue> {
    // 1. Run simulation based on parameters
    let result = sim::simulate_molecule(&params);
    
    // 2. Render energy plot
    plt::render_energy_plot(&result, energy_canvas_id)?;
    
    // 3. Render displacement plot
    plt::render_displacement_plot(&result, displacement_canvas_id)?;
    
    // 4. Prepare animation data
    // Either render animation frames or return data for JS to animate
    
    // 5. Return simulation data to JavaScript
    Ok(to_value(&result)?)
}

