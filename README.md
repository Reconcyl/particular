# `particular`

Particular is a Rust library that serves one function: displaying a window of a specified size containing circles of specified radius and color at specified positions over a background of specified color. That's it.

## API

The API (contained in `src/lib.rs`) can be summarized as follows:

    struct Circle {
        x: f32,
        y: f32,
        r: f32,
        color: [u8; 3],
    }

    fn display(
        name: &str,
        bg: [u8; 3],
        width: u32,
        height: u32,
        frames: impl Iterator<Item=impl Iterator<Item=Circle>>
    );

See `main.rs` for a very simple example. The library uses `ggez` game engine internally.