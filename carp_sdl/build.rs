fn main() 
{
    /*
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");
    let target_os = get_os_from_triple(target.as_str()).unwrap();
*/

    println!("cargo:rustc-flags=-l SDL2");
}