rustc -C opt-level=0 -C target-cpu=skylake  main.rs -o main_sky_o0
rustc -C opt-level=1 -C target-cpu=skylake  main.rs -o main_sky_o1
rustc -C opt-level=2 -C target-cpu=skylake  main.rs -o main_sky_o2
rustc -C opt-level=3 -C target-cpu=skylake  main.rs -o main_sky_o3


rustc -C opt-level=0 -C target-cpu=znver3  main.rs -o main_zen_o0
rustc -C opt-level=1 -C target-cpu=znver3  main.rs -o main_zen_o1
rustc -C opt-level=2 -C target-cpu=znver3  main.rs -o main_zen_o2
rustc -C opt-level=3 -C target-cpu=znver3  main.rs -o main_zen_o3

rustc -C opt-level=0 -C target-cpu=cannonlake  main.rs -o main_can_o0
rustc -C opt-level=1 -C target-cpu=cannonlake  main.rs -o main_can_o1
rustc -C opt-level=2 -C target-cpu=cannonlake  main.rs -o main_can_o2
rustc -C opt-level=3 -C target-cpu=cannonlake  main.rs -o main_can_o3

rustc -C opt-level=0 -C target-cpu=ivybridge  main.rs -o main_ivy_o0
rustc -C opt-level=1 -C target-cpu=ivybridge  main.rs -o main_ivy_o1
rustc -C opt-level=2 -C target-cpu=ivybridge  main.rs -o main_ivy_o2
rustc -C opt-level=3 -C target-cpu=ivybridge  main.rs -o main_ivy_o3

