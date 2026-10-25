---
name: bevy-ui-preview
description: Use when a Bevy file spawns UI and needs a headless PNG/MP4 preview. Covers the lele_bevy_lint rules (LELEBEVY001/LELEBEVY002/LELEBEVY003/LELEBEVY004/LELEBEVY005/LELEBEVY006/LELEBEVY007/LELEBEVY008), the lele_bevy_preview scene DSL and test template, the change gate, and how to find preview gaps with --ui-inventory.
---

# Bevy UI previews

Every UI file must ship a preview test that routes through
`lele_bevy_preview::run(...)`. Previews run in the default test suite
(`just test`) and changed artifacts are sent to Telegram. This skill tells you where your UI lives, which
preview it needs, and how to write the test. The rules below are generated from
`lele_bevy_lint`, so they cannot drift from the linter.

## 1. Where is my UI defined?

Run the inventory for the crate:

```bash
lele-bevy-lint --scan-folder src,examples --ui-inventory
```

Each row is a file with UI. `markers` lists the components declared in that
file, `visual` the UI idents it spawns, `anim driver` the time/input signals it
reads, and `png`/`mp4`/`scene` whether a preview exists. Every `-` is a gap.

## 2. Which preview do I need?

- spawns UI (`Node`, `Text`, `ImageNode`, `Sprite`, `Text2d`, `Mesh2d`,
  `MeshMaterial2d`, `Button`) -> a `*_ui_png_preview`
- also reads a time driver (`Res<Time>`, `delta_secs`, `elapsed_secs`, `delta`,
  `Timer`, `Animatable`, `AnimatableUi`, `AnimationClip`, `AnimationPlayer`,
  `tween`, `keyframe`, `keyframes`) -> a `*_ui_mp4_preview`
- restyles UI with `&mut` on a styling component (`BackgroundColor`,
  `BorderColor`, `TextColor`, `Text`, `TextFont`, `Node`, `Visibility`,
  `ImageNode`, `Sprite`, `Outline`) without spawning: an input/state change
  (`Interaction`, `ButtonInput`, `MouseButton`, `KeyCode`) needs a
  `*_ui_png_preview` with two `State`s (before, after); a time-driven change
  needs a `*_ui_mp4_preview`
- defines `impl Plugin` and its domain spawns UI -> a
  `*_ui_scene_preview`

Preview tests are plain `#[test]`s: do not `#[ignore]` them. They render
headless (lavapipe) inside `just test`, so every run re-checks them.

## 3. The test template

Copy this into the file that owns the UI, next to the systems:

```rust
#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use lele_bevy_preview::preview::Config;
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::Kind;
    use lele_bevy_preview::scene::Scene;
    use lele_bevy_preview::scene::State;

    fn out() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui_preview")
    }

    fn build_scene(app: &mut App) {
        // spawn the real UI, or add the real plugin
    }

    fn empty_lobby(_world: &mut World) {}

    #[test]
    fn spawn_root_ui_png_preview() {
        run(
            &Scene {
                name: String::from("spawn_root"),
                kind: Kind::System,
                build: build_scene,
                states: vec![State {
                    label: String::from("empty lobby"),
                    apply: empty_lobby,
                }],
                timeline: None,
            },
            &Config { out_dir: out(), ..Config::default() },
            env!("CARGO_PKG_NAME"),
        )
        .expect("preview");
    }
}
```

## 4. The scene DSL

- `Kind` = `System` | `Component` | `Plugin` | `App`; required, it prefixes the Telegram
  message (`[system] spawn_root`).
- `State` = one PNG; `apply: fn(&mut World)` mutates the world before capture.
- `Timeline` = N frames -> one MP4; `frames`, `fps`, `apply`. `Config::lead_in_frames`
  baseline frames are captured before `apply`, and a clip whose frames are all
  identical fails with `Error::StaticClip`.
- A change that lands in one tick (`is_changed`, `Changed<T>`, `Local`) is not motion:
  show it as two `State`s (before, after) instead of an mp4.
- `build: fn(&mut App)` assembles the scene with typed code — no reflection, no JSON.
- `Config`: `width`, `height`, `warmup_frames`, `lead_in_frames`, `max_capture_frames`,
  `min_distinct_colors`, `out_dir`.

`run` renders every state, encodes the timeline if present, and fails with
`Error::NoVisual` when a frame has fewer than `min_distinct_colors` distinct
colours — a blank frame is an error, not a green test.

## 5. The change gate

`run` hashes every frame and persists a manifest. On a re-run, `FirstRun` /
`New` / `Changed` are sendable and `Same` is not. Telegram delivery fires only
for sendable artifacts when `TELEGRAM_BOT_TOKEN` / `TELEGRAM_CHAT_ID` are set;
missing credentials are not an error. When anything in a scene is sendable, the
whole scene (every state PNG plus the clip) goes out as one Telegram album with a
single numbered caption, so before/after images stay together.

## 6. Run commands

```bash
lele-bevy-lint                    # the rules (installed from github.com/Uomocosa/lele-rust-linter)
just test                         # all targets (or: cargo nextest run --all-targets --all-features)
cargo nextest run ui_png_preview ui_mp4_preview ui_scene_preview -- --nocapture
```

## Rules

### LELEBEVY001 `bevy_export`

A domain never re-exports its `bevy_systems` items at the domain root; callers write `domain::bevy_systems::system`.

**Why:** Consumers reach systems as `domain::bevy_systems::system`, so the folder is always visible in the path.

**Fix:** Delete the `pub use bevy_systems::<system>;` lines from the domain's `mod.rs` and make callers write `<domain>::bevy_systems::<system>`; keep the `pub use` inside `bevy_systems/mod.rs`.

**Bad** (reports LELEBEVY001):

`src/inventory/mod.rs`

```rust
pub mod bevy_systems;

pub use bevy_systems::poll_inv;
```

`src/inventory/bevy_systems/mod.rs`

```rust
pub mod poll_inv;
pub use poll_inv::poll_inv;
```

**Good:**

`src/inventory/mod.rs`

```rust
pub mod bevy_systems;
```

`src/inventory/bevy_systems/mod.rs`

```rust
pub mod poll_inv;
pub use poll_inv::poll_inv;
```


### LELEBEVY002 `bevy_folder`

A `pub fn` registered with `app.add_systems()` whose parameters read Bevy system types must live in the domain's `bevy_systems/` folder.

**Why:** Systems are the domain's Bevy edge; keeping them in one folder makes the plugin surface readable.

**Fix:** Move the system into `<domain>/bevy_systems/<system>.rs`, re-export it from `bevy_systems/mod.rs` only, and register it as `<domain>::bevy_systems::<system>`.

**Bad** (reports LELEBEVY002):

`src/enemy/tick_enemies.rs`

```rust
pub struct Query;

pub fn tick_enemies(_q: Query) {}

pub fn register_enemies() {
    let app = App;
    app.add_systems(Update, tick_enemies);
}
```

**Good:**

`src/enemy/mod.rs`

```rust
pub mod bevy_systems;
```

`src/enemy/bevy_systems/mod.rs`

```rust
pub mod tick_enemies;
pub use tick_enemies::tick_enemies;
```

`src/enemy/bevy_systems/tick_enemies.rs`

```rust
pub struct Query;

pub fn tick_enemies(_q: Query) {}
```


### LELEBEVY003 `bevy_ui`

A file whose visual spawn is reachable from production must ship a `*_ui_png_preview` test whose `Scene::build` renders that spawn (or the file's `Plugin`) through `lele_bevy_preview::run(...)`.

**Why:** UI that is never rendered cannot be reviewed; the named preview test is the contract that forces an artifact for every production visual, and naming the real spawner keeps it from rendering a copy.

**Fix:** Add a `*_ui_png_preview` `#[test]` to the file whose `Scene::build` spawns that UI (or adds its `Plugin`) and runs it through `lele_bevy_preview::run(...)`; see the bevy-ui-preview skill.

**Bad** (reports LELEBEVY003):

`src/discovery/ui/spawn_root.rs`

```rust
pub struct Sprite;
pub struct Spawner;

pub fn setup(spawner: &mut Spawner) {
    spawner.spawn((Sprite,));
}
```

**Good:**

`src/discovery/ui/spawn_root.rs`

```rust
pub struct Sprite;
pub struct Spawner;

pub fn setup(spawner: &mut Spawner) {
    spawner.spawn((Sprite,));
}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn build(app: &mut App) {
        app.add_systems(Startup, super::setup);
        app.add_systems(PostStartup, lele_bevy_preview::scene::bevy_systems::center_roots);
    }

    fn rest(_world: &mut World) {}

    #[test]
    fn spawn_root_ui_png_preview() {
        let scene = Scene {
            name: String::from("spawn_root"),
            kind: Kind::System,
            build,
            states: vec![State { label: String::from("rest"), apply: rest }],
            timeline: None,
        };
        run(&scene, &Config::default(), "x").expect("preview");
    }
}
```


### LELEBEVY004 `bevy_ui_mp4`

A file that spawns UI reachable from production and drives it over time (`Time`, `Timer`, animation) must ship a `*_ui_mp4_preview` test whose `Scene` has a `Timeline`.

**Why:** Static frames cannot show motion; a time-driven UI needs a recorded clip to be reviewable. One-tick input changes are covered by `bevy_ui_restyle`'s before/after states instead.

**Fix:** Add a `*_ui_mp4_preview` `#[test]` whose `Scene` has a `Timeline` that drives the time-based UI; see the bevy-ui-preview skill.

**Bad** (reports LELEBEVY004):

`src/discovery/ui/sync_room_list.rs`

```rust
pub fn setup(s: &mut S) { s.spawn((Node,)); }

pub fn tick(time: Res<Time>) {
    let _ = time.delta_secs();
}
```

**Good:**

`src/discovery/ui/sync_room_list.rs`

```rust
pub fn setup(s: &mut S) { s.spawn((Node,)); }

pub fn tick(time: Res<Time>) { let _ = time.delta_secs(); }

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State, Timeline};

    fn build(app: &mut App) {
        app.add_systems(Startup, super::setup).add_systems(Update, super::tick);
        app.add_systems(PostStartup, lele_bevy_preview::scene::bevy_systems::center_roots);
    }

    fn rest(_world: &mut World) {}

    fn scene(timeline: Option<Timeline>) -> Scene {
        Scene {
            name: String::from("sync_room_list"),
            kind: Kind::System,
            build,
            states: vec![State { label: String::from("rest"), apply: rest }],
            timeline,
        }
    }

    #[test]
    fn sync_room_list_ui_png_preview() {
        run(&scene(None), &Config::default(), "x").expect("preview");
    }

    #[test]
    fn sync_room_list_ui_mp4_preview() {
        let clip = Timeline { label: String::from("tick"), frames: 30, fps: 30, apply: rest };
        run(&scene(Some(clip)), &Config::default(), "x").expect("preview");
    }
}
```


### LELEBEVY005 `bevy_ui_restyle`

A file with a system that restyles UI (`&mut` on a styling component) must ship a preview that renders that system: a `*_ui_png_preview` with at least two `State`s when it is driven by input or state, or a `*_ui_mp4_preview` when it is driven by time.

**Why:** A restyle never changes the set of entities, so a spawn-only preview cannot see it; only a rendering that names the restyle system (with a before/after pair, or a clip for motion) proves the change lands.

**Fix:** Add a preview that runs the restyling system: a `*_ui_png_preview` with two `State`s (before, after) for input or state changes, or a `*_ui_mp4_preview` for time-driven changes.

**Bad** (reports LELEBEVY005):

`src/discovery/ui/bevy_systems/gate_create.rs`

```rust
pub fn build(app: &mut App) {
    app.add_systems(Update, gate_create);
}

pub fn gate_create(button: Query<&mut BackgroundColor>) {
    let _ = button;
}
```

**Good:**

`src/discovery/ui/bevy_systems/gate_create.rs`

```rust
pub fn build(app: &mut App) {
    app.add_systems(Update, gate_create);
}

pub fn gate_create(button: Query<&mut BackgroundColor>) {
    let _ = button;
}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    use super::gate_create;

    fn preview_build(app: &mut App) {
        app.add_systems(Update, gate_create);
        app.add_systems(PostStartup, lele_bevy_preview::scene::bevy_systems::center_roots);
    }

    fn rest(_world: &mut World) {}

    #[test]
    fn gate_create_ui_png_preview() {
        let scene = Scene {
            name: String::from("gate_create"),
            kind: Kind::System,
            build: preview_build,
            states: vec![
                State { label: String::from("blank"), apply: rest },
                State { label: String::from("typed"), apply: rest },
            ],
            timeline: None,
        };
        run(&scene, &Config::default(), "x").expect("preview");
    }
}
```


### LELEBEVY006 `bevy_plugin_scene`

A file defining a `Plugin` whose build spawns UI (in its own folder, or through a system it registers directly or via its `methods/<plugin>/` delegate) must ship a `*_ui_scene_preview` test whose `Scene::build` adds that plugin.

**Why:** A plugin is the assembly point for a whole screen; its scene preview proves the assembled screen renders.

**Fix:** Add a `*_ui_scene_preview` `#[test]` whose `Scene::build` adds the plugin and runs it through `lele_bevy_preview::run(...)`.

**Bad** (reports LELEBEVY006):

`src/discovery/ui/default_ui_plugin.rs`

```rust
pub fn spawn_ui(s: &mut S) { s.spawn((Node,)); }

pub struct DefaultUiPlugin;

impl Plugin for DefaultUiPlugin {
    fn build(&self, app: &mut A) { spawn_ui(app); }
}
```

**Good:**

`src/discovery/ui/default_ui_plugin.rs`

```rust
pub fn spawn_ui(s: &mut S) { s.spawn((Node,)); }

pub struct DefaultUiPlugin;

impl Plugin for DefaultUiPlugin {
    fn build(&self, app: &mut A) { spawn_ui(app); }
}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn build(app: &mut App) {
        app.add_plugins(super::DefaultUiPlugin);
    }

    fn rest(_world: &mut World) {}

    fn scene() -> Scene {
        Scene {
            name: String::from("default_ui_plugin"),
            kind: Kind::Plugin,
            build,
            states: vec![State { label: String::from("rest"), apply: rest }],
            timeline: None,
        }
    }

    #[test]
    fn default_ui_plugin_ui_png_preview() {
        run(&scene(), &Config::default(), "x").expect("preview");
    }

    #[test]
    fn default_ui_plugin_ui_scene_preview() {
        run(&scene(), &Config::default(), "x").expect("preview");
    }
}
```


### LELEBEVY007 `preview_routing`

Every `*_ui_png_preview`, `*_ui_mp4_preview` and `*_ui_scene_preview` fn must be a `#[test]` that calls `lele_bevy_preview::run(...)` and checks its result with `.expect(..)`, `.unwrap()` or `?`.

**Why:** A preview test that renders and asserts nothing can pass while producing a blank frame; routing through the harness and checking its `Result` makes `NoVisual`/`StaticClip` fail the test.

**Fix:** Make the preview fn a `#[test]` that calls `lele_bevy_preview::run(...)` and checks the result with `?`, `.unwrap()` or `.expect(..)`.

**Bad** (reports LELEBEVY007):

`src/discovery/ui/room_row.rs`

```rust
pub fn setup(s: &mut S) { s.spawn((Node,)); }

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;

    #[test]
    fn room_row_ui_png_preview() {
        let _ = run(&scene(), &Config::default(), "x");
    }
}
```

`src/discovery/ui/spawn_root.rs`

```rust
pub fn setup(s: &mut S) { s.spawn((Node,)); }

#[cfg(test)]
mod tests {
    #[test]
    fn spawn_root_ui_png_preview() {
        let shot = std::path::PathBuf::from("spawn_root.png");
        assert!(shot.exists());
    }
}
```

**Good:**

`src/discovery/ui/spawn_root.rs`

```rust
pub fn setup(s: &mut S) { s.spawn((Node,)); }

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn build(app: &mut App) {
        app.add_systems(Startup, super::setup);
        app.add_systems(PostStartup, lele_bevy_preview::scene::bevy_systems::center_roots);
    }

    fn rest(_world: &mut World) {}

    #[test]
    fn spawn_root_ui_png_preview() {
        let scene = Scene {
            name: String::from("spawn_root"),
            kind: Kind::System,
            build,
            states: vec![State { label: String::from("rest"), apply: rest }],
            timeline: None,
        };
        run(&scene, &Config::default(), "x").expect("preview");
    }
}
```


### LELEBEVY008 `preview_centered`

A preview whose scene is `Kind::System` or `Kind::Component` must centre its UI: its `build` (or a sibling fn it reaches) must call `lele_bevy_preview::scene::centered_root()` or add `lele_bevy_preview::scene::bevy_systems::center_roots`.

**Why:** Parentless UI nodes lay out at the window's (0,0), so isolated system/component previews pile into the top-left corner, overlap each other and hide state changes; `Plugin`/`App` previews keep their real layout and are exempt.

**Fix:** Call `lele_bevy_preview::scene::centered_root()` in the scene's `build`, or add the `lele_bevy_preview::scene::bevy_systems::center_roots` system.

**Bad** (reports LELEBEVY008):

`src/discovery/ui/room_row.rs`

```rust
pub fn highlight(s: &mut S) {}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn spawn_rows(mut commands: Commands) {
        commands.spawn((Node::default(),));
    }

    fn build(app: &mut App) {
        app.add_systems(Startup, spawn_rows);
        app.add_systems(Update, super::highlight);
    }

    fn rest(_world: &mut World) {}

    #[test]
    fn room_row_ui_png_preview() {
        let scene = Scene {
            name: String::from("room_row"),
            kind: Kind::System,
            build,
            states: vec![State { label: String::from("rest"), apply: rest }],
            timeline: None,
        };
        run(&scene, &Config::default(), "x").expect("preview");
    }
}
```

**Good:**

`src/discovery/ui/room_row.rs`

```rust
pub fn highlight(s: &mut S) {}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn spawn_rows(mut commands: Commands) {
        let root = commands.spawn(lele_bevy_preview::scene::centered_root()).id();
        commands.spawn((ChildOf(root), Node::default()));
    }

    fn build(app: &mut App) {
        app.add_systems(Startup, spawn_rows);
        app.add_systems(Update, super::highlight);
    }

    fn rest(_world: &mut World) {}

    #[test]
    fn room_row_ui_png_preview() {
        let scene = Scene {
            name: String::from("room_row"),
            kind: Kind::System,
            build,
            states: vec![State { label: String::from("rest"), apply: rest }],
            timeline: None,
        };
        run(&scene, &Config::default(), "x").expect("preview");
    }
}
```


