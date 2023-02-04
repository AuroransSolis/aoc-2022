# AoC 2022

Christmas in... whenever I feel like it, I guess? I've been working on this on and off since
December since this year's challenges were pretty fun. I'm also trying to challenge myself to do the
problems in SystemVerilog too, so let's see how that goes. Here's some info on how to run my
solutions.

### Rust

The Rust solutions are organised into binaries and libraries for each day, each one with its own
```toml
[[bin]]
name = "dayN"
path = "src/dNbin.rs" # haha gottem
```
in `rust/Cargo.toml`. I also have a couple feature flags for doing some extra things:

 - `large`: Enables use of large inputs. These are included as compressed (usually `.zip`) files in
 `inputs/`. Due to GitHub filesize limitations, you will need to decompress these yourself.
 - `d7large`: Selects `inputs/d7large.txt` as the input file for the `day7` binary.
 - `d7deep1`: Selects `inputs/d7deep1.txt` as the input file for the `day7` binary.
 - `d7deep2`: Selects `inputs/d7deep2.txt` as the input file for the `day7` binary.

There's also a benchmark program set up through `criterion` that can be run with `cargo criterion`,
should you have that installed. The feature flags listed above also change the input files provided
to the benchmarks.

### SystemVerilog

So this one I'm honestly just leaving up to you to decide how to run it. I'm using ModelSim,
Quartus, and Vivado just to mess around with simulating and synthesising my stuff in order to learn
more about each tool, since I haven't used them much outside the basics in a single class (that
honestly really didn't teach me very much). I'll report my results for each problem/part solved in a
results file in that problem's subdir.

Also, to fill the ROM for each problem, a `dN-hex.txt` must exist in `inputs/`. `gen-hex-inputs.sh`
is the script I wrote to create these, including the test files for easier debugging.
