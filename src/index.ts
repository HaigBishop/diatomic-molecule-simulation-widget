// Import the WebAssembly initialization function from our Rust-generated module
import init from "../public/wasm/wasm_crate.js";

// Main class to handle the simulation UI and interactions
class SimulationUI {
    
    constructor() {
        // Set up all the UI controls
        this.setupControls();
        this.setupDropdowns();
    }

    // Private method to initialize and configure all UI controls
    private setupControls() {
        // List of IDs for all range input elements we want to handle
        const rangeInputs = ['duration', 'timestep', 'temperature'];
        
        // Loop through each input ID and set up its behavior
        rangeInputs.forEach(id => {
            // Get the range input element and its corresponding display element
            // 'as HTMLInputElement' tells TypeScript this is definitely an input element
            const input = document.getElementById(id) as HTMLInputElement; // e.g. duration
            const display = document.getElementById(`${id}-value`); // e.g. duration-value
            
            // Only proceed if both elements exist in the DOM
            if (input && display) {
                // Set the initial value display when the page loads
                display.textContent = input.value;
                
                // Update the display whenever the slider moves
                input.addEventListener('input', () => {
                    display.textContent = input.value;
                    // We'll add actual simulation logic later
                });
            }
        });
    }

    // Private method to initialize dropdowns
    private setupDropdowns() {
        // Get dropdown elements
        const modelSelect = document.getElementById('model') as HTMLSelectElement;
        const elementSelect = document.getElementById('element') as HTMLSelectElement;

        // Add event listeners for dropdowns
        if (modelSelect) {
            modelSelect.addEventListener('change', () => {
                console.log('Selected model:', modelSelect.value);
                // We'll add actual simulation logic later
            });
        }

        if (elementSelect) {
            elementSelect.addEventListener('change', () => {
                console.log('Selected element:', elementSelect.value);
                // We'll add actual simulation logic later
            });
        }
    }
}

// Main application initialization function
async function main() {
    try {
        // Initialize the WebAssembly module
        // This needs to be done before we can use any Rust functions
        await init();
        
        // Create a new instance of our UI class
        new SimulationUI();
    } catch (error) {
        // If anything goes wrong during initialization, log it to the console
        console.error("Failed to initialize:", error);
    }
}

// Start the application by calling main()
main();
