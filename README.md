# RUSTY KRUNKER

A small, hand-built 3D shooting game written in **Rust** using the **macroquad** game library. This project is meant as a learning experiment and demo of basic game systems implemented from scratch: shooting, collision detection, player movement, first/third person camera, a simple spatial grid for efficient collision/hit detection, and a starfield background.

---

## Background

This codebase implements a minimal shooting experience inspired by arena shooters. It focuses on core systems implemented by hand rather than relying on third‑party physics engines: ray-like shooting, bounding-box collision tests, a grid spatial partitioning to reduce checks, and two camera modes (first- and third-person).

---

## Features

* Shoot targets with a simple ray-step approach.
* Collision detection between player and enemies using axis-aligned bounding box projections.
* Player movement (W/A/S/D), jump, mouse-look (yaw/pitch) and camera control.
* Camera modes: First-Person and Third-Person (press `V` to toggle while held).
* Simple spatial grid to partition enemies into cells for efficient collision & hit detection.
* Starfield / infinity walls to create an outer environment.
* Score, best score tracking and a timed round (30s by default).

---

## Demo

![Gameplay Demo](./assets/gameplay/demo.gif)


---

## Built with

* Rust (stable toolchain)
* [macroquad](https://crates.io/crates/macroquad) (game framework)

> Note: No external physics or collision libraries are used — collision and shooting logic are handwritten for clarity and learning value.

---

## Prerequisites

* Rust toolchain (install from [https://rustup.rs](https://rustup.rs))
* `cargo` build tool (installed with rustup)

---

## Getting started (Run locally)

1. Clone this repository:

```bash
git clone https://github.com/OneFriendlyCoder/rustyKrunker.git
cd rustyKrunker
```

2. Ensure assets exist in the `assets/` folder. The game calls `set_pc_assets_folder("./assets/")` so the folder should be present in the project root. At minimum include `assets/textures/crosshair.png`.

3. Build & run:

```bash
cargo run --release
```

`--release` is recommended for better performance while playing.

> If you want to experiment from the editor, `cargo run` without `--release` works as well.

---

## Controls

* `W` `A` `S` `D` — Move forward / left / back / right
* Mouse — Look around (yaw & pitch)
* `Space` — Jump
* Hold **Right Mouse Button** — Zoom (aim down sights)
* Left Mouse Button (while aiming) — Fire shot
* `V` — Switch camera view (First <-> Third person; [note: in code it uses `is_key_down(KeyCode::V)` to hold toggle])
* `R` — Reset the game (respawn enemies and reset score)

UI: There is a simple HUD showing `Score`, `Best Score`, `Time Remaining` and reset hint.

---

## Project layout / code map

```
src/
├─ main.rs        # game entry, initialization & main loop
├─ utilis.rs      # utility functions (input, board size, enums)
├─ player.rs      # Player struct, movement, shooting logic & camera updates
├─ enemy.rs       # Enemy data, spawning, drawing and TTL update logic
├─ collision.rs   # Collision helpers, hit testing and ray-step bullet checks
├─ grid.rs        # Spatial grid for partitioning enemies (reduce collision checks)
├─ camera.rs      # CameraView enum
├─ infinity.rs    # Starfield / star-wall generator and drawing
assets/
└─ textures/
   └─ crosshair.png
```

### File responsibilities (summary)

* **main.rs**: Initializes game world, cameras, loads textures, game loop, UI drawing, and overall orchestration.
* **utilis.rs**: Input helpers, movement enum, screen/board size helpers.
* **player.rs**: Handles player state, movement updates, camera assignment for first/third person, shot creation and drawing, and scoring when hits are detected.
* **enemy.rs**: Enemy struct with simple TTL/health, drawing, and enemy vector management (including swap-remove behaviour to keep indices tightly packed).
* **grid.rs**: Uniform grid spatial partitioning. Each cell stores indices of enemies currently overlapping that cell. Provides helper methods to map world coordinates to cell indices and to query a cell.
* **collision.rs**: Projects bounding boxes to XZ plane for AABB checks and performs a ray-step search for bullets — looks up enemies cell-by-cell along a shot path.
* **infinity.rs**: Generates and renders a wall-of-stars for surround/background.
* **camera.rs**: Small CameraView enum used to switch camera behaviour.

---

## How the core mechanics work (technical)

### Shooting

* When player aims (right mouse button), the code constructs an extremely long ray starting at the camera `position` and pointed toward the camera `target` (a very large `md` multiplier used to make the endpoint far away).
* On left-click while aiming, the `shots` list gains a `Shot` with `start`, `end`, `lifetime`, and `hit` fields.
* `collision::check_bullet_hit_grid` iterates along that ray in integer step increments, checks the grid cell for enemies, and performs simple AABB containment tests for each candidate enemy.
* When an enemy is hit it is `swap_remove`-ed out of the `Enemies` vector and the grid cells are updated to remove the index and (if needed) patch indices that were moved due to `swap_remove`.

### Collision detection (player vs enemies)

* To detect player collisions, the code projects an enemy's bounding box corners onto the XZ plane and does 2D range checks against the player's projected bounds. That reduces the collision problem to a simple 2D intersection test.
* The `Grid` is used to only check nearby enemies by querying the player's cell and its adjacent cells.

### Spatial Grid

* The world is split into a uniform `xcells` by `zcells` grid. Each cell keeps a `Vec<usize>` of indices into the `Enemies.enemies` vector.
* Enemies register themselves into every cell they overlap using `add_enemy` during grid initialization.
* When enemies die and are removed via `swap_remove`, the grid is updated to reflect new indices (the code patches any cell entries that pointed to the last index, replacing them with the moved index).

### Camera modes

* Two `Camera3D` objects are maintained: `camera` (used for first-person) and `camera1` (for third-person). The code sets camera position & target based on `player.position`, `yaw`, `pitch`, and whether third person is active.

---

## Configuration & tuning

You can tweak gameplay by modifying constants and parameters in `main.rs` and other modules, for example:

* Number of enemies: `Enemies::init_enemies(100, ...)`
* Game duration: `game_duration` (currently 30 seconds)
* Player speed, jump velocity and gravity: in `player.rs` there are `speed`, `velocity_y`, and gravity constant used in the update.
* Grid resolution: `init_grid(..., xcells: 10, zcells: 10)` — increasing resolution reduces per-cell enemy count but increases overhead.

---

## Performance tips

* Run with `--release` for better frame rates.
* If your machine runs slowly, reduce enemy count or increase grid cells to reduce per-cell checks.
* The bullet hit detection currently steps integer units along the ray. Consider switching to a continuous ray-AABB intersection (analytical tests) for both correctness and performance.

---

## Known limitations & possible improvements

* **Shot stepping**: Using integer steps along the ray is simple but can miss very small targets or be inefficient. Replace with ray-AABB intersection tests (e.g. slab method) for robust collision detection.
* **Physics**: No actual physics engine; movement and collisions are manual and simplistic.
* **Enemy behavior**: Enemies are static (only TTL). Add movement, pathfinding, or behaviors.
* **Networking / Multiplayer**: Not implemented — single-player local experience only.
* **Audio**: No sound effects; add macroquad audio calls for shooting/hits/music.
* **Input smoothing**: Mouse sensitivity and smoothing can be tuned; consider clamping and configurable sensitivity.
* **Cross-platform builds**: Add a `wasm32` target or desktop packaging instructions.

---

## Troubleshooting

* **Black screen / texture not found**: Ensure `assets/textures/crosshair.png` exists. The project calls `set_pc_assets_folder("./assets/")` so place the `assets` directory beside `Cargo.toml`.
* **Macroquad compile errors**: Ensure Rust toolchain is up to date and `cargo` can fetch crates. If using an older Rust version, update using `rustup update`.
* **Low FPS**: Run `cargo run --release`, reduce enemy count, or profile to find hotspots.

---

## License

This project is provided under the **MIT License**. See `LICENSE` for details

---

## Acknowledgements

* `macroquad` for a compact and ergonomic game framework in Rust.
* Inspiration from classic arena shooters and educational demos of collision and ray casting.

---

If you want, I can:

* Generate a `Cargo.toml` snippet showing the dependency for `macroquad`.
* Add an example crosshair image or small assets.
* Convert the bullet stepping to an analytical ray-AABB method and update `collision.rs` with that implementation.

Tell me which follow-up you want and I will update the README or the code accordingly.
