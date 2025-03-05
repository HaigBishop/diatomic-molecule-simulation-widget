use wasm_bindgen::prelude::*;
use serde::Serialize;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    log(format!("Adding {} and {}", a, b).as_str());
    return a + b;
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}.", name)
}


#[wasm_bindgen]
#[derive(Serialize)]
pub struct SimulationResultState {
    time: f32,
    displacement: f32,
    potential_energy: f32,
    kinetic_energy: f32,
}

// Temporary placeholder function to "simulate" data
#[wasm_bindgen]
pub fn simulate() -> JsValue {
    let data = vec![
        SimulationResultState { time: 0.0, displacement: 1.0, potential_energy: 2.0, kinetic_energy: 3.0 },
        SimulationResultState { time: 1.0, displacement: 1.5, potential_energy: 2.5, kinetic_energy: 3.5 },
        SimulationResultState { time: 2.0, displacement: 1.8, potential_energy: 2.8, kinetic_energy: 3.8 },
        SimulationResultState { time: 3.0, displacement: 2.0, potential_energy: 3.0, kinetic_energy: 4.0 },
        SimulationResultState { time: 4.0, displacement: 2.2, potential_energy: 3.2, kinetic_energy: 4.2 },
        SimulationResultState { time: 5.0, displacement: 2.4, potential_energy: 3.4, kinetic_energy: 4.4 },
        SimulationResultState { time: 6.0, displacement: 2.6, potential_energy: 3.6, kinetic_energy: 4.6 },
        SimulationResultState { time: 7.0, displacement: 2.8, potential_energy: 3.8, kinetic_energy: 4.8 },
        SimulationResultState { time: 8.0, displacement: 3.0, potential_energy: 4.0, kinetic_energy: 5.0 },
        SimulationResultState { time: 9.0, displacement: 3.2, potential_energy: 4.2, kinetic_energy: 5.2 },
        SimulationResultState { time: 10.0, displacement: 3.4, potential_energy: 4.4, kinetic_energy: 5.4 },
        SimulationResultState { time: 11.0, displacement: 3.6, potential_energy: 4.6, kinetic_energy: 5.6 },
        SimulationResultState { time: 12.0, displacement: 3.8, potential_energy: 4.8, kinetic_energy: 5.8 },
    ];
    
    return to_value(&data).unwrap();
}

