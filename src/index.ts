// Import the WebAssembly initialization function and exported types/functions
import init, { simulate_and_plot, SimulationParameters } from "../public/wasm/wasm_crate.js";

// Main class to handle the simulation UI and interactions
class SimulationUI {
    // Properties to store references to UI elements
    private modelSelect: HTMLSelectElement;
    private elementSelect: HTMLSelectElement;
    private durationInput: HTMLInputElement;
    private timestepInput: HTMLInputElement;
    private temperatureInput: HTMLInputElement;
    
    constructor() {
        // Get references to all UI elements
        this.modelSelect = document.getElementById('model') as HTMLSelectElement;
        this.elementSelect = document.getElementById('element') as HTMLSelectElement;
        this.durationInput = document.getElementById('duration') as HTMLInputElement;
        this.timestepInput = document.getElementById('timestep') as HTMLInputElement;
        this.temperatureInput = document.getElementById('temperature') as HTMLInputElement;
        
        // Set up all the UI controls
        this.setupControls();
        this.setupDropdowns();
        
        // Run initial simulation
        this.runSimulation();
    }

    // Private method to initialize and configure all UI controls
    private setupControls() {
        // List of IDs for all range input elements we want to handle
        const rangeInputs = ['duration', 'timestep', 'temperature'];
        
        // Loop through each input ID and set up its behavior
        rangeInputs.forEach(id => {
            // Get the range input element and its corresponding display element
            const input = document.getElementById(id) as HTMLInputElement;
            const display = document.getElementById(`${id}-value`);
            
            // Only proceed if both elements exist in the DOM
            if (input && display) {
                // Set the initial value display when the page loads
                display.textContent = input.value;
                
                // Update the display whenever the slider moves
                input.addEventListener('input', () => {
                    display.textContent = input.value;
                });
                
                // Run simulation when slider value changes (on release)
                input.addEventListener('change', () => {
                    this.runSimulation();
                });
            }
        });
    }

    // Private method to initialize dropdowns
    private setupDropdowns() {
        // Add event listeners for dropdowns
        if (this.modelSelect) {
            this.modelSelect.addEventListener('change', () => {
                console.log('Selected model:', this.modelSelect.value);
                this.runSimulation();
            });
        }

        if (this.elementSelect) {
            this.elementSelect.addEventListener('change', () => {
                console.log('Selected element:', this.elementSelect.value);
                this.runSimulation();
            });
        }
    }
    
    // Method to run the simulation with current parameter values
    private runSimulation() {
        try {
            console.log('Running simulation with current parameters...');
            
            // Get current values from UI controls
            const model = this.modelSelect.value;
            const element = this.elementSelect.value;
            const duration = parseFloat(this.durationInput.value);
            const timestep = parseFloat(this.timestepInput.value);
            const temperature = parseFloat(this.temperatureInput.value);
            
            // Create parameters object for the Rust function
            const params = new SimulationParameters(
                model,
                element,
                duration,
                timestep,
                temperature
            );
            
            // Call the Rust function with parameters and canvas IDs
            const result = simulate_and_plot(
                params, 
                'energy-canvas',
                'displacement-canvas'//,
                // 'animation-canvas'
            );
            
            console.log('Simulation completed:', result);
            
            // Additional JS handling of the result could go here
            // For example, if you want to do additional processing of the data
            
        } catch (error) {
            console.error('Error running simulation:', error);
        }
    }
}

// Main application initialization function
async function main() {
    try {
        // Initialize the WebAssembly module
        // This needs to be done before we can use any Rust functions
        await init();
        console.log('WebAssembly module initialized successfully');
        
        // Create a new instance of our UI class
        new SimulationUI();
    } catch (error) {
        // If anything goes wrong during initialization, log it to the console
        console.error("Failed to initialize:", error);
    }
}

// Start the application by calling main()
main();
