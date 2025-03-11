# COSC 473 – Decentralised Applications on the Web  
## Assignment 1 - Wasm Widget  

### Introduction  
The goal of this assignment is to create a Rust-based WebAssembly (Wasm) application that interacts with JavaScript in a web environment. This assignment will demonstrate an understanding of Rust-Wasm interop, structured state management, and Rust programming for a computationally intensive web task.

### Suggested Widget Ideas  
- A physics-based simulation (e.g., bouncing balls) using `nalgebra`.  
- A cryptographic tool for message signing using `rsa` or `ring`.  
- A Monte Carlo simulation using `rand`.  
- A fractal generator using Rust for high-performance computation.  
- Your own idea.  

### Due Date  
**Friday, March 21, end of day.**  

### Basic Requirements  
- Implement a simple web-based widget where the core logic runs in Rust and is compiled to WebAssembly.  
- Expose Rust functions to JavaScript using `wasm-bindgen`.  
- Utilize at least one external Rust crate to enhance functionality (e.g., `rand`, `serde`, `nalgebra`, `rsa`, `image`).  
  - Not all crates are Wasm-compatible, especially if they interact with the OS or external C libraries.  
- Create a basic frontend using HTML and JavaScript that interacts with the Rust module.  
- Implement user interactions (e.g., buttons or input elements triggering Rust logic through JavaScript).  
- Ensure proper error handling within the Rust module.  
- Provide documentation explaining how to build and run the project, including dependencies and command-line instructions.  

### Sample Bonus Features  
A crate must require setup and thoughtful integration to count as a bonus. Examples:

| Crate | Basic Usage (Not Bonus) | Bonus-Worthy Usage |
|------|-------------------|-------------------|
| `rand` | Calling `rand::random()` once | Generating a simulated dataset, exposing it via WebAssembly, and visualizing it dynamically in JS |
| `nalgebra` | Using `nalgebra::Vector2` for a simple calculation | Implementing a physics engine (e.g., simulating bouncing balls) using `nalgebra` |
| `rsa` / `ring` | Generating a key pair and logging output | Encrypting messages with RSA, passing them through JavaScript, and decrypting in Rust |
| `serde` | Serializing a Rust struct into JSON once | Building a structured WebAssembly API where Rust objects are serialized/deserialized dynamically between Rust and JS |
| `wasm-bindgen-futures` | Using `async/await` for one function | Implementing a WebSocket-based chat where Rust asynchronously processes messages |
| `image` | Loading an image and printing its dimensions | Implementing an image filter where Rust-Wasm processes an image and returns a modified version |

Other bonus ideas:
- Create a more advanced UI using React, Svelte, or Vue.js.  
- Any creative idea that requires significant effort (check with the instructor).  

### Submission Requirements  
- Submit your code as a repository on **eng-git** and add user `bta47` to the project.  
- Include a `README.md` with build and run instructions.  
- Upload a **1-2 page report** to Learn, covering:
  - How the application works  
  - Challenges faced  
  - Approach to solving them  
- Include a **self-assessed grade estimation** with an explanation based on the grading criteria.  

### Grading  

| Category | Weight |
|----------|--------|
| Functionality | 40% |
| Rust Code Quality | 20% |
| JavaScript Interop | 20% |
| Creativity & Usability | 20% |

#### Rough Guidelines  
- **C range (or lower):** Application does not meet all basic requirements or does not function as expected.  
- **B range:** Meets basic requirements with minimal documentation.  
- **A range:** Includes bonus features, significant creativity, or demonstrates additional effort beyond core requirements.  

[Awesome Rust - External Libraries](https://github.com/rust-unofficial/awesome-rust)
