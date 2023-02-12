A simple demonstration of the difference in build times between a large generic function, and a large polymorphic function with a small generic wrapper function.

Run `cargo build` for the monomorphic version and `cargo build --features polymorphic` for the polymorphic version

Some sample build times on my machine:

|Monomophic | Polymorphic|
|-----------|------------|
|2.70s | 1.87s |
|2.70s | 1.77s |
|2.73s | 1.66s |