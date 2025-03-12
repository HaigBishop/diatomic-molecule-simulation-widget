/*
Modules for simulation of diatomic molecules

Contains:
 - ElementProperties struct: 
    - predefined elements (e.g. H, Hg, Ar) and their properties
 - SimulationParameters struct:
    - parameters for running a simulation, such as model type, element, duration, timestep, and temperature
 - SimulationState struct:
    - current state of the simulation, including time, displacement, force, acceleration, velocity, and energies
 - SimulationResult struct:
    - results of the simulation, including time series data for displacements, distances, and energies
 - simulate_molecule function:
    - orchestrates the simulation process by selecting the appropriate model based on parameters
    - calls one of:
        - simulate_harmonic_oscillator function
        - simulate_morse_potential function
        - simulate_lennard_jones function
*/

use wasm_bindgen::prelude::*;
use serde::Serialize;

// Conversion factors and constants
const KB: f32 = 1.3806488E-23;
const A0_TO_M: f32 = 5.2917721092E-11;



// Structure to hold physical constants for each element
#[derive(Clone, Copy)]
pub struct ElementProperties {
    m_au: f32,      // Mass (atomic units)
    k_au: f32,      // Force constant (atomic units)
    k_si: f32,      // Force constant (SI)
    d_au: f32,      // Dissociation energy (atomic units)
    d_si: f32,      // Dissociation energy (SI)
    alpha_au: f32,  // Bond strength (atomic units)
    alpha_si: f32,  // Bond strength (SI)
    rstr_au: f32,
    eps_au: f32,
}

// Define constants for all supported elements
const ELEMENT_PROPERTIES: &[(&str, ElementProperties)] = &[
    // Hydrogen
    ("H", ElementProperties {
        m_au: 9.114400E+02,
        k_au: 3.665358E-01,
        k_si: 5.706570E+02,
        d_au: 1.818446E-01,
        d_si: 7.928147E-19,
        alpha_au: 1.003894E+00,
        alpha_si: 1.897085E+10,
        rstr_au: 0.0,
        eps_au: 0.0,
    }),
    // Mercury
    ("Hg", ElementProperties {
        m_au: 1.840841E+05,
        k_au: 1.374407E-03,
        k_si: 2.139865E+00,
        d_au: 0.0,
        d_si: 0.0,
        alpha_au: 0.0,
        alpha_si: 0.0,
        rstr_au: 6.952302E+00,
        eps_au: 1.845314E-03,
    }),
    // Argon
    ("Ar", ElementProperties {
        m_au: 3.641021E+04,
        k_au: 3.232914E-04,
        k_si: 5.033442E-01,
        d_au: 0.0,
        d_si: 0.0,
        alpha_au: 0.0,
        alpha_si: 0.0,
        rstr_au: 7.107260E+00,
        eps_au: 4.536240E-04,
    }),
];

// Helper function to get element properties
fn get_element_properties(element: &str) -> Option<ElementProperties> {
    ELEMENT_PROPERTIES
        .iter()
        .find(|(symbol, _)| *symbol == element)
        .map(|(_, props)| *props)
}

// Define parameter struct for simulation settings
#[wasm_bindgen]
pub struct SimulationParameters {
    model: String,     // Model type (e.g., "harmonic", "morse", "lennard-jones")
    element: String,   // Element symbol (e.g., "H", "Hg", "Ar")
    duration: f64,     // Duration of the simulation
    timestep: f64,     // Time step for the simulation
    temperature: f64,  // Temperature for the simulation
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

// Structure to represent the current state of the simulation
#[derive(Clone)]
pub struct SimulationState {
    pub time: f32,          // Current time in the simulation
    pub displacement: f32,  // Current displacement from equilibrium
    pub force: f32,         // Current force acting on the molecule
    pub acceleration: f32,  // Current acceleration of the molecule
    pub velocity: f32,      // Current velocity of the molecule
    pub kinetic_e: f32,     // Current kinetic energy
    pub potential_e: f32,   // Current potential energy
    pub total_e: f32,       // Total energy (kinetic + potential)
}

impl SimulationState {
    // Initialize state for harmonic oscillator model
    pub fn init_harmonic_oscillator(properties: ElementProperties, temperature: f64) -> SimulationState {
        // Calculate the initial displacement based on temperature
        let r0_si_harm: f32 = ((2.0 * KB * temperature as f32) / properties.k_si).sqrt();
        let r0_a0_harm: f32 = r0_si_harm / A0_TO_M;
        
        SimulationState {
            time: 0.0,
            displacement: r0_a0_harm,
            force: -properties.k_au * r0_a0_harm,
            acceleration: -properties.k_au * r0_a0_harm / properties.m_au,
            velocity: 0.0,
            kinetic_e: 0.0,
            potential_e: 0.5 * properties.k_au * r0_a0_harm.powi(2),
            total_e: 0.5 * properties.k_au * r0_a0_harm.powi(2),
        }
    }

    // Initialize state for Morse potential model
    pub fn init_morse_potential(properties: ElementProperties, temperature: f64) -> SimulationState {
        // Calculate initial displacements
        let r0_si_harm: f32 = ((2.0 * KB * temperature as f32) / properties.k_si).sqrt();
        let r0_si_morse: f32 = (1.0 - (properties.k_si * r0_si_harm * r0_si_harm / 
                              (2.0 * properties.d_si)).sqrt()).ln() / (-properties.alpha_si);
        let r0_a0_morse: f32 = r0_si_morse / A0_TO_M;
        
        let exp_alpha_r0 = f32::exp(-properties.alpha_au * r0_a0_morse);
        let init_force = -2.0 * properties.d_au * properties.alpha_au * exp_alpha_r0 * (1.0 - exp_alpha_r0);
        let exp_alpha_r0_sq = (1.0 - exp_alpha_r0).powi(2);
        
        SimulationState {
            time: 0.0,
            displacement: r0_a0_morse,
            force: init_force,
            acceleration: init_force / properties.m_au,
            velocity: 0.0,
            kinetic_e: 0.0,
            potential_e: properties.d_au * exp_alpha_r0_sq,
            total_e: properties.d_au * exp_alpha_r0_sq,
        }
    }

    // Initialize state for Lennard-Jones potential model
    pub fn init_lennard_jones(properties: ElementProperties, temperature: f64) -> SimulationState {
        // Calculate initial displacements
        let r0_si_harm: f32 = ((2.0 * KB * temperature as f32) / properties.k_si).sqrt();
        let r0_a0_harm: f32 = r0_si_harm / A0_TO_M;
        
        // Calculate LJ initial displacement from harmonic displacement
        let r0_a0_lj: f32 = properties.rstr_au * (((2.0 * properties.eps_au).powf(1.0 / 12.0) * 
                           ((properties.k_au).sqrt() * r0_a0_harm + 
                           (2.0 * properties.eps_au).sqrt()).powf(-1.0 / 6.0)) - 1.0);
        
        let rstar_over = properties.rstr_au / (r0_a0_lj + properties.rstr_au);
        let init_force = (12.0 / (r0_a0_lj + properties.rstr_au)) * 
                        properties.eps_au * (rstar_over.powi(12) - rstar_over.powi(6));
        
        SimulationState {
            time: 0.0,
            displacement: r0_a0_lj,
            force: init_force,
            acceleration: init_force / properties.m_au,
            velocity: 0.0,
            kinetic_e: 0.0,
            potential_e: properties.eps_au * (rstar_over.powi(12) - 2.0 * rstar_over.powi(6) + 1.0),
            total_e: properties.eps_au * (rstar_over.powi(12) - 2.0 * rstar_over.powi(6) + 1.0),
        }
    }
}

// Define result struct for time series data
#[derive(Serialize)]
pub struct SimulationResult {
    pub times: Vec<f64>,             // Time points of the simulation
    pub displacements: Vec<f64>,     // Displacements at each time point
    pub distances: Vec<f64>,         // Distances at each time point
    pub potential_energies: Vec<f64>,// Potential energies at each time point
    pub kinetic_energies: Vec<f64>,  // Kinetic energies at each time point
    pub total_energies: Vec<f64>,    // Total energies at each time point
}

// Function to generate synthetic simulation data
pub fn simulate_molecule(params: &SimulationParameters) -> SimulationResult {
    // Get properties for the selected element
    let properties = get_element_properties(&params.element())
        .expect("Element not supported");

    // Get the model and run the appropriate simulation
    let model = params.model();
    
    let sim_result = match model.as_str() {
        "harmonic" => {
            let initial_sim_state = SimulationState::init_harmonic_oscillator(properties, params.temperature());
            simulate_harmonic_oscillator(initial_sim_state, params)
        },
        "morse" => {
            let initial_sim_state = SimulationState::init_morse_potential(properties, params.temperature());
            simulate_morse_potential(initial_sim_state, params)
        },
        "lennard-jones" => {
            let initial_sim_state = SimulationState::init_lennard_jones(properties, params.temperature());
            simulate_lennard_jones(initial_sim_state, params)
        },
        _ => panic!("Unsupported model: {}", model),
    };
    
    sim_result
}

// Function to simulate the harmonic oscillator model
fn simulate_harmonic_oscillator(mut state: SimulationState, params: &SimulationParameters) -> SimulationResult {
    // Initialize vectors to store simulation data
    let mut times = Vec::new();
    let mut displacements = Vec::new();
    let mut distances = Vec::new();
    let mut potential_energies = Vec::new();
    let mut kinetic_energies = Vec::new();
    let mut total_energies = Vec::new();
    
    // Get element properties
    let properties = get_element_properties(&params.element())
        .expect("Element not supported");
    
    // Calculate number of steps
    let duration = params.duration() as f32;
    let dt = params.timestep() as f32;
    let steps = (duration / dt) as usize;
    
    // Store initial state
    times.push(state.time as f64);
    displacements.push(state.displacement as f64);
    distances.push(state.displacement as f64);
    potential_energies.push(state.potential_e as f64);
    kinetic_energies.push(state.kinetic_e as f64);
    total_energies.push(state.total_e as f64);
    
    // Time integration loop (Velocity Verlet algorithm)
    for _ in 0..steps {
        // Update position using current velocity and acceleration
        let r_half = state.displacement + state.velocity * dt * 0.5;
        
        // Calculate new force and acceleration at half-step position
        let force = -properties.k_au * r_half;
        let accel = force / properties.m_au;
        
        // Update velocity and position
        state.velocity += accel * dt;
        state.displacement = r_half + state.velocity * dt * 0.5;
        
        // Update force and acceleration at new position
        state.force = -properties.k_au * state.displacement;
        state.acceleration = state.force / properties.m_au;
        
        // Update energies
        state.kinetic_e = 0.5 * properties.m_au * state.velocity * state.velocity;
        state.potential_e = 0.5 * properties.k_au * state.displacement * state.displacement;
        state.total_e = state.kinetic_e + state.potential_e;
        
        // Update time
        state.time += dt;
        
        // Store data
        times.push(state.time as f64);
        displacements.push(state.displacement as f64);
        distances.push(state.displacement as f64);
        potential_energies.push(state.potential_e as f64);
        kinetic_energies.push(state.kinetic_e as f64);
        total_energies.push(state.total_e as f64);
    }

    // Temporary fix to ensure distances are positive (add 1.1 * abs(min_distance) to all distances)
    let min_distance = distances.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let offset = if min_distance < 0.0 { 1.1 * min_distance.abs() } else { 0.0 };
    distances.iter_mut().for_each(|d| *d += offset);
    
    SimulationResult {
        times,
        displacements,
        distances,
        potential_energies,
        kinetic_energies,
        total_energies,
    }
}

// Function to simulate the Morse potential model
fn simulate_morse_potential(mut state: SimulationState, params: &SimulationParameters) -> SimulationResult {
    // Initialize vectors to store simulation data
    let mut times = Vec::new();
    let mut displacements = Vec::new();
    let mut distances = Vec::new();
    let mut potential_energies = Vec::new();
    let mut kinetic_energies = Vec::new();
    let mut total_energies = Vec::new();
    
    // Get element properties
    let properties = get_element_properties(&params.element())
        .expect("Element not supported");
    
    // Calculate number of steps
    let duration = params.duration() as f32;
    let dt = params.timestep() as f32;
    let steps = (duration / dt) as usize;
    
    // Store initial state
    times.push(state.time as f64);
    displacements.push(state.displacement as f64);
    distances.push(state.displacement as f64);
    potential_energies.push(state.potential_e as f64);
    kinetic_energies.push(state.kinetic_e as f64);
    total_energies.push(state.total_e as f64);
    
    // Time integration loop (Velocity Verlet algorithm)
    for _ in 0..steps {
        // Update position using current velocity and acceleration
        let r_half = state.displacement + state.velocity * dt * 0.5;
        
        // Calculate new force at half-step position (Morse potential)
        let exp_alpha_r = f32::exp(-properties.alpha_au * r_half);
        let force = -2.0 * properties.d_au * properties.alpha_au * exp_alpha_r * (1.0 - exp_alpha_r);
        let accel = force / properties.m_au;
        
        // Update velocity and position
        state.velocity += accel * dt;
        state.displacement = r_half + state.velocity * dt * 0.5;
        
        // Update force and acceleration at new position
        let exp_alpha_r = f32::exp(-properties.alpha_au * state.displacement);
        state.force = -2.0 * properties.d_au * properties.alpha_au * exp_alpha_r * (1.0 - exp_alpha_r);
        state.acceleration = state.force / properties.m_au;
        
        // Update energies
        state.kinetic_e = 0.5 * properties.m_au * state.velocity * state.velocity;
        let exp_alpha_r_sq = (1.0 - exp_alpha_r).powi(2);
        state.potential_e = properties.d_au * exp_alpha_r_sq;
        state.total_e = state.kinetic_e + state.potential_e;
        
        // Update time
        state.time += dt;
        
        // Store data
        times.push(state.time as f64);
        displacements.push(state.displacement as f64);
        distances.push(state.displacement as f64);
        potential_energies.push(state.potential_e as f64);
        kinetic_energies.push(state.kinetic_e as f64);
        total_energies.push(state.total_e as f64);
    }

    // Temporary fix to ensure distances are positive (add 1.1 * abs(min_distance) to all distances)
    let min_distance = distances.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let offset = if min_distance < 0.0 { 1.1 * min_distance.abs() } else { 0.0 };
    distances.iter_mut().for_each(|d| *d += offset);
    
    SimulationResult {
        times,
        displacements,
        distances,
        potential_energies,
        kinetic_energies,
        total_energies,
    }
}

// Function to simulate the Lennard-Jones potential model
fn simulate_lennard_jones(mut state: SimulationState, params: &SimulationParameters) -> SimulationResult {
    // Initialize vectors to store simulation data
    let mut times = Vec::new();
    let mut displacements = Vec::new();
    let mut distances = Vec::new();
    let mut potential_energies = Vec::new();
    let mut kinetic_energies = Vec::new();
    let mut total_energies = Vec::new();
    
    // Get element properties
    let properties = get_element_properties(&params.element())
        .expect("Element not supported");
    
    // Calculate number of steps
    let duration = params.duration() as f32;
    let dt = params.timestep() as f32;
    let steps = (duration / dt) as usize;
    
    // Store initial state
    times.push(state.time as f64);
    displacements.push(state.displacement as f64);
    distances.push(state.displacement as f64);
    potential_energies.push(state.potential_e as f64);
    kinetic_energies.push(state.kinetic_e as f64);
    total_energies.push(state.total_e as f64);
    
    // Time integration loop (Velocity Verlet algorithm)
    for _ in 0..steps {
        // Update position using current velocity and acceleration
        let r_half = state.displacement + state.velocity * dt * 0.5;
        
        // Calculate new force at half-step position (Lennard-Jones potential)
        let rstar_over = properties.rstr_au / (r_half + properties.rstr_au);
        let force = (12.0 / (r_half + properties.rstr_au)) * 
                    properties.eps_au * (rstar_over.powi(12) - rstar_over.powi(6));
        let accel = force / properties.m_au;
        
        // Update velocity and position
        state.velocity += accel * dt;
        state.displacement = r_half + state.velocity * dt * 0.5;
        
        // Update force and acceleration at new position
        let rstar_over = properties.rstr_au / (state.displacement + properties.rstr_au);
        state.force = (12.0 / (state.displacement + properties.rstr_au)) * 
                      properties.eps_au * (rstar_over.powi(12) - rstar_over.powi(6));
        state.acceleration = state.force / properties.m_au;
        
        // Update energies
        state.kinetic_e = 0.5 * properties.m_au * state.velocity * state.velocity;
        state.potential_e = properties.eps_au * (rstar_over.powi(12) - 2.0 * rstar_over.powi(6) + 1.0);
        state.total_e = state.kinetic_e + state.potential_e;
        
        // Update time
        state.time += dt;
        
        // Store data
        times.push(state.time as f64);
        displacements.push(state.displacement as f64);
        distances.push(state.displacement as f64);
        potential_energies.push(state.potential_e as f64);
        kinetic_energies.push(state.kinetic_e as f64);
        total_energies.push(state.total_e as f64);
    }

    // Temporary fix to ensure distances are positive (add 1.1 * abs(min_distance) to all distances)
    let min_distance = distances.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let offset = if min_distance < 0.0 { 1.1 * min_distance.abs() } else { 0.0 };
    distances.iter_mut().for_each(|d| *d += offset);
    
    SimulationResult {
        times,
        displacements,
        distances,
        potential_energies,
        kinetic_energies,
        total_energies,
    }
}


