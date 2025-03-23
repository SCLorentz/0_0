fn main()
{
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    
    let mut targets = Vec::new();

    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    {
        targets.push(("exit_arml", "src/functions/core/exit_arml.s"));
        targets.push(("write_arml", "src/functions/core/write_arml.s"));
        targets.push(("read_arml", "src/functions/core/read_arml.s"));
    }

    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    {
        targets.push(("exit_amdl", "src/functions/core/exit_amdl.s"));
        // TODO: (possible problem) review this file:
        targets.push(("write_amdl", "src/functions/core/write_amdl.s"));
    }

    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    {
        targets.push(("exit_armx", "src/functions/core/exit_armx.s"));
        targets.push(("write_armx", "src/functions/core/write_armx.s"));
    }

    for (name, file) in targets
    {
        cc::Build::new()
            .file(file)
            .compile(name);
    }
}