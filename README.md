# Game of Life in Rust

This is a simple implementation of Conway's Game of Life in Rust, running in the terminal.

## Example

Here's an example of what the game looks like when running:

![Game of Life Example](gol.gif)

## How to Run

1. Ensure you have Rust and Cargo installed on your system. If not, you can install them from [rustup.rs](https://rustup.rs/).

2. Clone this repository:
   ```
   git clone [your-repository-url]
   cd [your-repository-name]
   ```

3. Run the game:
   ```
   cargo run
   ```

4. To exit the game, press `Ctrl+C`.

## Features

- Random initialization of the grid
- Continuous updates of the game state
- Terminal-based display

## Customization

You can customize the game by modifying the following constants in `src/main.rs`:
- `WIDTH`: The width of the game grid
- `HEIGHT`: The height of the game grid

You can also adjust the probability of cells being initially alive by changing the argument to `gen_bool()` in the `initialize_random_grid` function.

## Contributing

Feel free to fork this repository and submit pull requests with improvements or additional features!
