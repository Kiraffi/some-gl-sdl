rustc -C opt-level=0 -C target-cpu=skylake  main.rs -o main_sky_o0.exe
rustc -C opt-level=1 -C target-cpu=skylake  main.rs -o main_sky_o1.exe
rustc -C opt-level=2 -C target-cpu=skylake  main.rs -o main_sky_o2.exe
rustc -C opt-level=3 -C target-cpu=skylake  main.rs -o main_sky_o3.exe


rustc -C opt-level=0 -C target-cpu=ivybridge  main.rs -o main_ivy_o0.exe
rustc -C opt-level=1 -C target-cpu=ivybridge  main.rs -o main_ivy_o1.exe
rustc -C opt-level=2 -C target-cpu=ivybridge  main.rs -o main_ivy_o2.exe
rustc -C opt-level=3 -C target-cpu=ivybridge  main.rs -o main_ivy_o3.exe

