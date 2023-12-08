# Advent of Code template
This template is largely inspired by https://github.com/fspoettel/advent-of-code-rust

However, it is written from scratch as the original template creates a new binary for each day.
This, unfortunately, makes it [impossible](https://bheisler.github.io/criterion.rs/book/user_guide/known_limitations.html) to benchmark using `criterion.rs`.

Because of this, I decided to create this template instead.
Internally, this uses some macro trickery to make each day accessible dynamically.

> [!NOTE]
> Although this template should largely be functional, there might still be some rough edges. Feel free to open an Issue or create a Pull Request if you notice anything.