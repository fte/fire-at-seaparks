# Fire at Seapark

This project simulates a fire effect in the terminal using Rust. The fire effect is generated using a simple cellular automaton and displayed with ANSI escape codes for color.

## How to Compile and Run

1. Ensure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).

2. Clone the repository or download the source code.

3. Navigate to the project directory:
    ```sh
    cd fire-at-seapark
    ```

4. Compile the project using Cargo:
    ```sh
    cargo build --release
    ```

5. Run the compiled binary:
    ```sh
    cargo run --release
    ```

## Screenshot

![Fire Simulation](screenshot.png)

## Code Explanation

- The code uses the `rand` crate to generate random numbers for the fire intensity and spread.
- The fire effect is represented by a 2D vector `fire_pixels` where each element represents the intensity of the fire at that point.
- The fire is propagated upwards by decreasing the intensity and spreading it to neighboring pixels.
- The fire is displayed using ANSI escape codes to color the terminal output.

## Dependencies

- `rand` crate for random number generation.

## License

This project is licensed under the MIT License.
