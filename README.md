
# 1D Diatomic Molecule Simulation
*(diatomic-molecule-simulation-widget)*


### Rust Components

 - simulate function 
     - inputs: 
        - duration
        - element parameters (e.g. mass, force constant)
        - model
        - time step
        - temperature
     - outputs: 
        - displacements over time
        - energies over time
        - plots of energies and displacements over time
            - axis labels
            - sensible axis ticks
            - automatic title
            - legend for energies

### UI Inputs

 - Dropdown for model
 - Dropdown for element
 - Slider for duration (1,000 - 1,000,000)
 - Slider for time-step (1 - 100)
 - Slider for temperature (50 - 1,000)

### UI Outputs

  - plot of energie over time
     - axis labels
     - sensible axis ticks
     - automatic title
     - legend for potential, kinetic and total energies
  - plot of bond length over time
     - axis labels
     - sensible axis ticks
     - automatic title
  - looping animation of atoms
     - two solid circles moving according to displacement over time
     - total duration is played in 10 seconds
     - axis label & ticks


### Useful Sources

 - https://plotters-rs.github.io/wasm-demo/www/index.html
 - https://crates.io/crates/plotters
 - https://github.com/plotters-rs/plotters

