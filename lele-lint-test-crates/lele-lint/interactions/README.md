# Interaction test crates

One dir per declared `Rule::relations` entry: the file names and folders show the
situation, and the crate lints clean only because the first rule defers to the second.

- `atomic_file__bevy_folder`: `atomic_file` defers to `bevy_folder`; `widget_poll.rs` starts with the `Widget` stem but is not a method file.
- `method_file_co_location__bevy_folder`: `method_file_co_location` defers to `bevy_folder`; `sensor_read.rs` sits apart from `sensor.rs`.
- `no_free_functions__bevy_folder`: `no_free_functions` defers to `bevy_folder`; `spin` is a free function by design.
- `constants_placement__flat_dir`: `constants_placement` defers to `flat_dir`; `lele.toml` declares the flat dir (clean), `lele.without.toml` drops it (`LELE019` fires).
- `constants_placement__testing_layout`: `constants_placement` defers to `testing_layout`; `testing/__fixtures__/sample.rs` uses the const without forcing a lift.
- `atomic_delegates__inline_trait_methods`: `atomic_delegates` defers to `inline_trait_methods`; `lele.toml` lists `Greet::greet` (clean), `lele.without.toml` drops it (`LELECORE009` fires).
