# Interactive Diatomic Molecule Simulation (Web Widget in with Rust Wasm)
**Author:** Haig Bishop

**Date:** 24-03-2025

---

### Project Overview

This project is a web-based interactive simulation of a 1D diatomic molecule. It allows users to explore the dynamics of molecular vibrations under different potential energy models and for various elements. The simulation is built using Rust for the computational backend, compiled to WebAssembly, and TypeScript for the frontend user interface.

![Screen Capture](res/screen_capture.png)

---

### Features

- **Interactive Controls**: Users can adjust simulation parameters such as:
    - **Potential Model**: Choose between Harmonic Oscillator, Morse Potential, and Lennard-Jones models.
    - **Element Selection**: Simulate molecules of Hydrogen (H), Argon (Ar), or Mercury (Hg).
    - **Simulation Duration**: Control the length of the simulation.
    - **Time Step**: Adjust the granularity of the simulation.
    - **Temperature**: Set the temperature of the simulation environment.

- **Real-time Visualizations**: The application provides dynamic visualizations of the simulation:
    - **Animated Atom Display**: A canvas animation showing the movement of the two atoms in the diatomic molecule.
    - **Energy Plot**: A graph displaying the potential, kinetic, and total energy of the system over time.
    - **Displacement Plot**: A graph showing the displacement of the atoms from their equilibrium position over time.

- **Rust-Powered Simulation**: The core simulation logic is implemented in Rust, ensuring high performance and efficiency. WebAssembly allows seamless integration of Rust code into the web browser environment.

- **Responsive User Interface**: The user interface is designed to be intuitive and responsive, providing a smooth user experience.

- **Parameter Validation**: The application includes validation to ensure that selected models and elements are compatible, guiding users to valid simulation configurations.

---

### Try the following:

1.  Select the **Lennard-Jones Model** from the "Model" dropdown.
2.  Increase the **Duration** slider to a high value (e.g. 300,000)
3.  Experiment with the **Temperature** slider. 

Observe that as you increase the temperature above a certain threshold, the atoms will only move further and further apart from each other, dissociating completely. This demonstrates the temperature-dependent behavior of the Lennard-Jones potential, where high temperatures can overcome the attractive forces between atoms.

---


## How to Build and Run

This project is built using Rust for the simulation logic compiled to WebAssembly (wasm), and TypeScript for the user interface. To build and run this project, you will need to have Node.js/npm and Rust with wasm-pack installed on your system. Follow these steps to get started:

### Prerequisites

1.  **Install Node.js and npm:**
    -   If you don't have Node.js and npm (Node Package Manager) installed, you'll need to download and install them.
    -   Go to the official Node.js website: [https://nodejs.org/](https://nodejs.org/) and download the installer for your operating system. npm is included with Node.js.
    -   Follow the installation instructions provided on the website.
    -   Once installed, you can verify the installation by opening a terminal or command prompt and running the following commands:
        ```bash
        node -v
        npm -v
        ```
        These commands should display the versions of Node.js and npm installed on your system.

2.  **Install Rust and wasm-pack:**
    -   If you don't have Rust installed, you can install it by following the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
    -   For most Unix-like systems (Linux, macOS), you can install Rust by running the following command in your terminal:
        ```bash
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        ```
        For Windows, download and run the `rustup-init.exe` installer from the Rust website.
    -   After installing Rust, you need to install `wasm-pack`, which is a tool for building and working with WebAssembly in Rust.
    -   Open your terminal or command prompt and run:
        ```bash
        cargo install wasm-pack
        ```
    -   Ensure that Rust's Cargo package manager is in your system's PATH environment variable so that `wasm-pack` can be found.
    -   You can verify the installation of `wasm-pack` by running:
        ```bash
        wasm-pack --version
        ```

### Building and Running the Project

1.  **Clone the project repository:**
    -   If you haven't already, clone the project repository to your local machine using Git:
        ```bash
        git clone https://github.com/HaigBishop/diatomic-molecule-simulation-widget.git
        cd diatomic-molecule-simulation-widget
        ```

2.  **Navigate to the project directory:**
    -   If you just cloned the repository, you are already in the project directory (`diatomic-molecule-simulation-widget`). If not, navigate to it using the `cd` command in your terminal.

3.  **Install npm dependencies:**
    -   In the project directory, run the following command to install the required npm packages:
        ```bash
        npm install
        ```
        This command reads the `package.json` file and installs all the dependencies listed there, which are necessary for the TypeScript and Vite parts of the project.

4.  **Build the Rust WebAssembly module:**
    -   You can build the Rust code into a WebAssembly module using the npm script defined in `package.json`:
        ```bash
        npm run build-wasm
        ```
        This command compiles the Rust code in `wasm-crate/src/lib.rs` to WebAssembly and places the output files in the `public/wasm` directory in the project's root. The `--target web` option specifies that the wasm module is intended to run in a web browser environment.

5.  **Start the development server:**
    -   To start the Vite development server, use the following npm script:
        ```bash
        npm run dev
        ```
        This command starts a local development server using Vite. It will typically provide you with a URL (usually `http://localhost:5173/`) to access the application in your web browser. Vite will watch for file changes and automatically reload the browser, making development easier.

6.  **Open in your browser:**
    -   Open your web browser and go to the URL provided by Vite (e.g., `http://localhost:5173/`). You should see the Diatomic Molecule Simulation running in your browser.

### Building and Running with a Single Command

For convenience, you can use the `npm run start` command from the project root to both build the Rust WebAssembly module and start the development server. This command is set up in the `package.json` file to execute both the wasm build and the Vite development server in sequence.

Simply run:

```bash
npm run start
```

This will first compile the Rust code, and then start the Vite development server, making it a quick way to get the application up and running during development.

### Useful Resources

 - https://plotters-rs.github.io/wasm-demo/www/index.html
 - https://crates.io/crates/plotters
 - https://github.com/plotters-rs/plotters
 - https://github.com/plotters-rs/plotters/blob/a212c30a17f0c44f683b44adb096bba3bae21ae5/README.md
