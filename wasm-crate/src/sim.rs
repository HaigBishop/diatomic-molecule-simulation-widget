use wasm_bindgen::prelude::*;
use serde::Serialize;
use crate::log;

// Define parameter struct
#[wasm_bindgen]
pub struct SimulationParameters {
    model: String,
    element: String,
    duration: f64,
    timestep: f64,
    temperature: f64,
}

#[wasm_bindgen]
impl SimulationParameters {
    #[wasm_bindgen(constructor)]
    pub fn new(model: String, element: String, duration: f64, timestep: f64, temperature: f64) -> SimulationParameters {
        SimulationParameters {
            model,
            element,
            duration,
            timestep,
            temperature,
        }
    }
    
    // Getters for accessing the fields
    #[wasm_bindgen(getter)]
    pub fn model(&self) -> String {
        self.model.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn element(&self) -> String {
        self.element.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn duration(&self) -> f64 {
        self.duration
    }
    
    #[wasm_bindgen(getter)]
    pub fn timestep(&self) -> f64 {
        self.timestep
    }
    
    #[wasm_bindgen(getter)]
    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}

// Define result struct for time series data
#[derive(Serialize)]
pub struct SimulationResult {
    pub times: Vec<f64>,
    pub positions: Vec<f64>,
    pub potential_energies: Vec<f64>,
    pub kinetic_energies: Vec<f64>,
    pub total_energies: Vec<f64>,
}

// Placeholder function to generate synthetic simulation data
pub fn simulate_molecule(params: &SimulationParameters) -> SimulationResult {
    // Log the parameters we received
    log(&format!("Simulating with model: {}, element: {}, duration: {}, timestep: {}, temperature: {}",
        params.model, params.element, params.duration, params.timestep, params.temperature));
    
    // Create vectors to hold our synthetic data
    let mut times = Vec::with_capacity(20);
    let mut positions = Vec::with_capacity(20);
    let mut potential_energies = Vec::with_capacity(20);
    let mut kinetic_energies = Vec::with_capacity(20);
    let mut total_energies = Vec::with_capacity(20);
    
    // Generate synthetic data based on a simple sine wave
    // This will create oscillating position and energy values
    for i in 0..20 {
        let t = i as f64 * 0.5; // time in arbitrary units
        times.push(t);
        
        // Position follows a damped sine wave
        let amplitude = 1.0 * ((-0.05 * t).exp());
        let position = amplitude * (2.0 * std::f64::consts::PI * t).sin();
        positions.push(position);
        
        // Potential energy is proportional to position squared (like a spring)
        let potential = 0.5 * 2.0 * position.powi(2);
        potential_energies.push(potential);
        
        // Kinetic energy is higher when position is near zero (maximum velocity)
        let kinetic = 0.5 * (1.0 - position.powi(2) / amplitude.powi(2)) * 2.0;
        kinetic_energies.push(kinetic);
        
        // Total energy should be roughly constant for a proper simulation
        // but we'll add a slight decay to simulate energy loss
        let total = potential + kinetic;
        total_energies.push(total * ((-0.02 * t).exp()));
    }
    
    // Return the synthetic data
    SimulationResult {
        times,
        positions,
        potential_energies,
        kinetic_energies,
        total_energies,
    }
}
