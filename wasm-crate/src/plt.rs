use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use crate::sim::SimulationResult;
use crate::log;

// Function to render the energy plot
pub fn render_energy_plot(result: &SimulationResult, canvas_id: &str) -> Result<(), JsValue> {
    // Get the canvas element
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id)
        .ok_or_else(|| JsValue::from_str(&format!("Cannot find canvas with id {}", canvas_id)))?;
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;
    
    // Create a drawing backend using the canvas
    let backend = CanvasBackend::with_canvas_object(canvas)
        .ok_or_else(|| JsValue::from_str("Cannot create canvas backend"))?;
    
    // Create a drawing area on the backend
    let root = backend.into_drawing_area();
    
    // Clear any previous drawing
    root.fill(&WHITE)
        .map_err(|e| JsValue::from_str(&format!("Cannot fill background: {}", e)))?;
    
    // Find min and max values for setting up chart scales
    let max_time = result.times.iter().fold(0.0, |a, &b| f64::max(a, b));
    let min_energy = result.total_energies.iter()
        .chain(result.potential_energies.iter())
        .chain(result.kinetic_energies.iter())
        .fold(0.0, |a, &b| f64::min(a, b));
    let max_energy = result.total_energies.iter()
        .chain(result.potential_energies.iter())
        .chain(result.kinetic_energies.iter())
        .fold(0.0, |a, &b| f64::max(a, b));
    
    // Add a bit of padding to the min/max values
    let y_range = max_energy - min_energy;
    let y_min = min_energy - y_range * 0.1;
    let y_max = max_energy + y_range * 0.1;
    
    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .caption("Energy Over Time", ("sans-serif", 20).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..max_time, y_min..y_max)
        .map_err(|e| JsValue::from_str(&format!("Cannot build chart: {}", e)))?;
    
    // Configure mesh and axes
    chart.configure_mesh()
        .x_desc("Time")
        .y_desc("Energy")
        .x_labels(20)
        .x_label_formatter(&|x| format!("{}", x.floor() as i32))
        .draw()
        .map_err(|e| JsValue::from_str(&format!("Cannot draw mesh: {}", e)))?;
    
    // Draw the potential energy data
    chart.draw_series(LineSeries::new(
        result.times.iter().zip(&result.potential_energies).map(|(&x, &y)| (x, y)),
        RED.filled()
    ))
    .map_err(|e| JsValue::from_str(&format!("Cannot draw potential energy series: {}", e)))?
    .label("Potential Energy")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
    
    // Draw the kinetic energy data
    chart.draw_series(LineSeries::new(
        result.times.iter().zip(&result.kinetic_energies).map(|(&x, &y)| (x, y)),
        BLUE.filled()
    ))
    .map_err(|e| JsValue::from_str(&format!("Cannot draw kinetic energy series: {}", e)))?
    .label("Kinetic Energy")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));
    
    // Draw the total energy data
    chart.draw_series(LineSeries::new(
        result.times.iter().zip(&result.total_energies).map(|(&x, &y)| (x, y)),
        GREEN.filled()
    ))
    .map_err(|e| JsValue::from_str(&format!("Cannot draw total energy series: {}", e)))?
    .label("Total Energy")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));
    
    // Draw the legend
    chart.configure_series_labels()
        .background_style(WHITE.filled())
        .border_style(BLACK)
        .draw()
        .map_err(|e| JsValue::from_str(&format!("Cannot draw legend: {}", e)))?;
    
    // Present the drawing
    root.present()
        .map_err(|e| JsValue::from_str(&format!("Cannot present chart: {}", e)))?;
    
    log(&format!("Energy plot rendered to canvas: {}", canvas_id));
    Ok(())
}

// Function to render the displacement plot
pub fn render_displacement_plot(result: &SimulationResult, canvas_id: &str) -> Result<(), JsValue> {
    // Get the canvas element
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id)
        .ok_or_else(|| JsValue::from_str(&format!("Cannot find canvas with id {}", canvas_id)))?;
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;
    
    // Create a drawing backend using the canvas
    let backend = CanvasBackend::with_canvas_object(canvas)
        .ok_or_else(|| JsValue::from_str("Cannot create canvas backend"))?;
    
    // Create a drawing area on the backend
    let root = backend.into_drawing_area();
    
    // Clear any previous drawing
    root.fill(&WHITE)
        .map_err(|e| JsValue::from_str(&format!("Cannot fill background: {}", e)))?;
    
    // Find min and max values for setting up chart scales
    let max_time = result.times.iter().fold(0.0, |a, &b| f64::max(a, b));
    let min_position = result.displacements.iter().fold(0.0, |a, &b| f64::min(a, b));
    let max_position = result.displacements.iter().fold(0.0, |a, &b| f64::max(a, b));
    
    // Add a bit of padding to the min/max values
    let y_range = max_position - min_position;
    let y_min = min_position - y_range * 0.1;
    let y_max = max_position + y_range * 0.1;
    
    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .caption("Displacement Over Time", ("sans-serif", 20).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..max_time, y_min..y_max)
        .map_err(|e| JsValue::from_str(&format!("Cannot build chart: {}", e)))?;
    
    // Configure mesh and axes
    chart.configure_mesh()
        .x_desc("Time")
        .y_desc("Displacement")
        .x_labels(20)
        .x_label_formatter(&|x| format!("{}", x.floor() as i32))
        .draw()
        .map_err(|e| JsValue::from_str(&format!("Cannot draw mesh: {}", e)))?;
    
    // Draw the position data
    chart.draw_series(LineSeries::new(
        result.times.iter().zip(&result.displacements).map(|(&x, &y)| (x, y)),
        BLUE.filled()
    ))
    .map_err(|e| JsValue::from_str(&format!("Cannot draw position series: {}", e)))?;
    
    // Present the drawing
    root.present()
        .map_err(|e| JsValue::from_str(&format!("Cannot present chart: {}", e)))?;
    
    log(&format!("Displacement plot rendered to canvas: {}", canvas_id));
    Ok(())
}
