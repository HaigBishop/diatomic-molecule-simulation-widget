/**
 * Molecular Dynamics Simulation Interface
 * 
 * This module provides a TypeScript interface for a molecular dynamics simulation
 * powered by Rust WebAssembly. It handles:
 * 
 * - User interface controls for simulation parameters (model, element, duration, etc.)
 * - Simulation parameter validation and management
 * - Execution of simulations via WebAssembly
 * - Visualization of simulation results through:
 *   - Energy plots (handled by Rust/WASM)
 *   - Displacement plots (handled by Rust/WASM)
 *   - Animated atom visualization (handled by JavaScript)
 * 
 * The simulation supports different potential energy models (Harmonic, Morse, Lennard-Jones)
 * and different elements (H, Hg, Ar) with validation to ensure valid combinations.
 */

// Import Rust WebAssembly for simulation and plotting (+ wasm initialization function)  
import init, { simulate_and_plot, SimulationParameters } from "../public/wasm/wasm_crate.js";

// Main class to handle the simulation UI and interactions
class SimulationUI {
    // Properties to store references to UI elements
    private modelSelect: HTMLSelectElement;
    private elementSelect: HTMLSelectElement;
    private durationInput: HTMLInputElement;
    private timestepInput: HTMLInputElement;
    private temperatureInput: HTMLInputElement;
    
    // Add a new property for the error message element
    private errorMessageElement: HTMLElement;
    
    // Animation properties
    private animationCanvas: HTMLCanvasElement;
    private animationContext: CanvasRenderingContext2D | null;
    private animationFrameId: number | null = null;
    private simulationResult: any = null;
    private animationStartTime: number = 0;
    
    constructor() {
        // Get references to all UI elements
        this.modelSelect = document.getElementById('model') as HTMLSelectElement;
        this.elementSelect = document.getElementById('element') as HTMLSelectElement;
        this.durationInput = document.getElementById('duration') as HTMLInputElement;
        this.timestepInput = document.getElementById('timestep') as HTMLInputElement;
        this.temperatureInput = document.getElementById('temperature') as HTMLInputElement;
        this.animationCanvas = document.getElementById('animation-canvas') as HTMLCanvasElement;
        this.animationContext = this.animationCanvas.getContext('2d');
        
        // Get reference to the error message element
        this.errorMessageElement = document.getElementById('error-message') as HTMLElement;
        
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
                this.validateModelElementCombination('model');
                this.runSimulation();
            });
        }

        if (this.elementSelect) {
            this.elementSelect.addEventListener('change', () => {
                console.log('Selected element:', this.elementSelect.value);
                this.validateModelElementCombination('element');
                this.runSimulation();
            });
        }
    }
    
    // Method to validate and adjust model-element combinations
    private validateModelElementCombination(changedInput: 'model' | 'element') {
        const model = this.modelSelect.value;
        const element = this.elementSelect.value;
        
        // Validation rules:
        // - harmonic: any element is valid
        // - morse: only H is valid
        // - LJ: only Hg and Ar are valid
        
        if (changedInput === 'model') {
            // User changed the model, adjust element if needed
            if (model === 'morse' && element !== 'H') {
                console.log('Morse model only supports H, adjusting element');
                this.elementSelect.value = 'H';
            } else if (model === 'lennard-jones' && element !== 'Hg' && element !== 'Ar') {
                console.log('LJ model only supports Hg and Ar, adjusting element');
                this.elementSelect.value = 'Ar'; // Default to Ar for LJ model
            }
            // For harmonic model, any element is valid, so no adjustment needed
        } else {
            // User changed the element, adjust model if needed
            if (element === 'H') {
                // H works with harmonic and morse, no need to change if model is already one of these
                if (model !== 'harmonic' && model !== 'morse') {
                    console.log('Element H requires harmonic or morse model, adjusting model');
                    this.modelSelect.value = 'harmonic'; // Default to harmonic
                }
            } else if (element === 'Hg' || element === 'Ar') {
                // Hg and Ar work with harmonic and LJ
                if (model !== 'harmonic' && model !== 'lennard-jones') {
                    console.log('Element Hg/Ar requires harmonic or LJ model, adjusting model');
                    this.modelSelect.value = 'harmonic'; // Default to harmonic
                }
            } else {
                // Any other element only works with harmonic
                if (model !== 'harmonic') {
                    console.log('This element only supports harmonic model, adjusting model');
                    this.modelSelect.value = 'harmonic';
                }
            }
        }
    }
    
    // Methods to show and hide error messages
    private showError(message: string): void {
        this.errorMessageElement.textContent = message;
        this.errorMessageElement.classList.remove('hidden');
    }
    
    private hideError(): void {
        this.errorMessageElement.textContent = '';
        this.errorMessageElement.classList.add('hidden');
    }
    
    // Method to run the simulation with current parameter values
    private runSimulation() {
        try {
            // Hide any previous error message
            this.hideError();
            
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
                'displacement-canvas'
            );
            
            console.log('Simulation completed:', result);         
            
            // Store the simulation result and start the animation
            this.simulationResult = result;
            this.startAnimation();
            
        } catch (error) {
            // Display the error message to the user
            const errorMessage = error instanceof Error ? error.message : String(error);
            this.showError(`Simulation failed: ${errorMessage}`);
            console.error('Error running simulation:', error);
            
            // Clear any previous simulation result
            this.simulationResult = null;
            
            // Cancel any ongoing animation
            if (this.animationFrameId !== null) {
                cancelAnimationFrame(this.animationFrameId);
                this.animationFrameId = null;
            }
        }
    }
    
    // Start the animation loop
    private startAnimation() {
        // Only start animation if we have simulation results
        if (!this.simulationResult) return;
        
        // Cancel any existing animation
        if (this.animationFrameId !== null) {
            cancelAnimationFrame(this.animationFrameId);
        }
        
        // Reset start time
        this.animationStartTime = performance.now();
        
        // Start the animation loop
        this.animateAtoms();
    }
    
    // Animation loop for atoms
    private animateAtoms() {
        if (!this.simulationResult || !this.animationContext) return;
        
        const ctx = this.animationContext;
        const canvas = this.animationCanvas;
        const distances = this.simulationResult.distances;
        const times = this.simulationResult.times;
        
        // Calculate the maximum absolute distance to scale the axis
        const maxAbsDistance = Math.max(...distances.map(Math.abs)) * 1.1; // Add 10% padding
        
        // Animation timing
        const animationDuration = 10000; // 10 seconds in ms
        const currentTime = performance.now();
        const elapsedTime = currentTime - this.animationStartTime;
        const animationProgress = (elapsedTime % animationDuration) / animationDuration;
        
        // Calculate index in the simulation data
        const dataIndex = Math.min(
            Math.floor(animationProgress * distances.length),
            distances.length - 1
        );
        
        // Get current distance and convert to positions
        const currentDistance = distances[dataIndex];
        const atom1Position = -currentDistance / 2;
        const atom2Position = currentDistance / 2;
        
        // Clear canvas
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        
        // Set canvas scaling and transformations
        const padding = 40; // Padding for axis labels
        const axisY = canvas.height / 2;
        const scaleX = (canvas.width - padding * 2) / (maxAbsDistance * 2);
        
        // Draw axis
        ctx.beginPath();
        ctx.moveTo(padding, axisY);
        ctx.lineTo(canvas.width - padding, axisY);
        ctx.strokeStyle = '#888';
        ctx.lineWidth = 1;
        ctx.stroke();
        
        // Draw tick marks and labels
        this.drawAxisTicks(ctx, axisY, padding, canvas.width - padding, maxAbsDistance, scaleX);
        
        // Draw atoms
        this.drawAtom(ctx, padding + (maxAbsDistance + atom1Position) * scaleX, axisY, '#646cff');
        this.drawAtom(ctx, padding + (maxAbsDistance + atom2Position) * scaleX, axisY, '#646cff');
        
        // Draw time indicator
        ctx.fillStyle = '#444';
        ctx.font = '12px Arial';
        ctx.textAlign = 'right';
        ctx.fillText(`Time: ${times[dataIndex].toFixed(2)} fs`, canvas.width - padding, 20);
        
        // Continue animation loop
        this.animationFrameId = requestAnimationFrame(() => this.animateAtoms());
    }
    
    // Helper to draw an atom
    private drawAtom(ctx: CanvasRenderingContext2D, x: number, y: number, color: string) {
        const radius = 6;
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, Math.PI * 2);
        ctx.fillStyle = color;
        ctx.fill();
    }
    
    // Helper to draw axis ticks and labels
    private drawAxisTicks(
        ctx: CanvasRenderingContext2D, 
        axisY: number, 
        leftX: number, 
        rightX: number, 
        maxValue: number,
        scaleX: number
    ) {
        const axisWidth = rightX - leftX;
        const centerX = leftX + axisWidth / 2;
        
        // Draw center line (position 0)
        ctx.beginPath();
        ctx.moveTo(centerX, axisY - 5);
        ctx.lineTo(centerX, axisY + 5);
        ctx.strokeStyle = '#888';
        ctx.stroke();
        
        ctx.fillStyle = '#444';
        ctx.font = '12px Arial';
        ctx.textAlign = 'center';
        ctx.fillText('0', centerX, axisY + 20);
        
        // Calculate reasonable tick intervals based on max value
        const tickInterval = this.calculateTickInterval(maxValue);
        
        // Draw positive ticks
        for (let value = tickInterval; value <= maxValue; value += tickInterval) {
            const x = centerX + value * scaleX;
            ctx.beginPath();
            ctx.moveTo(x, axisY - 5);
            ctx.lineTo(x, axisY + 5);
            ctx.stroke();
            ctx.fillText(value.toFixed(1), x, axisY + 20);
        }
        
        // Draw negative ticks
        for (let value = -tickInterval; value >= -maxValue; value -= tickInterval) {
            const x = centerX + value * scaleX;
            ctx.beginPath();
            ctx.moveTo(x, axisY - 5);
            ctx.lineTo(x, axisY + 5);
            ctx.stroke();
            ctx.fillText(value.toFixed(1), x, axisY + 20);
        }
        
        // Draw axis label
        ctx.fillText('Position (Å)', centerX, axisY + 40);
    }
    
    // Helper to calculate reasonable tick intervals
    private calculateTickInterval(maxValue: number): number {
        const idealTickCount = 5; // We want about 5 ticks on each side
        let interval = maxValue / idealTickCount;
        
        // Round to a nice number
        const magnitude = Math.pow(10, Math.floor(Math.log10(interval)));
        const normalized = interval / magnitude;
        
        if (normalized < 1.5) {
            interval = magnitude;
        } else if (normalized < 3.5) {
            interval = 2 * magnitude;
        } else {
            interval = 5 * magnitude;
        }
        
        return interval;
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
