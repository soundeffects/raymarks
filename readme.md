# Raymarks
Empirical benchmarks for ray tracing.

The goal is to explore the feasibility and performance characteristics of
various ray tracing techniques and data structures for real-time rendering.
I limit my approaches to those which use widely available hardware, using
the WebGPU API (and possibly some extensions provided by the `wgpu` crate)
as baseline for support. I plan to test a variety of software techniques on
top of that baseline.