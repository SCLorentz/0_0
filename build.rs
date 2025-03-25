fn main()
{
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-arg=-Wl,--gc-sections");
    println!("cargo:rustc-link-arg=-Wl,--strip-all");

    configure_linker();
    let targets = configure_targets();

    for (name, file) in targets
    {
        cc::Build::new()
            .file(file)
            .compile(name);
    }

    fn configure_linker()
    {
        #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
        {
            println!("cargo:rerun-if-changed=src/linkers/aarch64.ld");
            println!("cargo:rustc-link-arg=-Tsrc/linkers/aarch64.ld");
        }

        #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
        {
            println!("cargo:rerun-if-changed=linkers/x86_64.ld");
            println!("cargo:rustc-link-arg=-Tlinkers/x86_64.ld");
        }
    }

    fn configure_targets() -> Vec<(&'static str, &'static str)>
    {
        let mut targets = Vec::new();

        #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
        {
            targets.push(("exit", "src/functions/core/linux/aarch64/exit.s"));
            targets.push(("write", "src/functions/core/linux/aarch64/write.s"));
            targets.push(("read", "src/functions/core/linux/aarch64/read.s"));
            targets.push(("fork", "src/functions/core/linux/aarch64/fork.s"));
            targets.push(("exec", "src/functions/core/linux/aarch64/exec.s"));
        }

        #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
        {
            targets.push(("exit", "src/functions/core/linux/x86_64/exit.s"));
            // TODO: (possible problem) review this file:
            targets.push(("write", "src/functions/core/linux/x86_64/write.s"));
            targets.push(("read", "src/functions/core/linux/x86_64/read.s"));
        }

        #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
        {
            targets.push(("exit", "src/functions/core/mac/exit.s"));
            targets.push(("write", "src/functions/core/mac/write.s"));
        }

        targets
    }
}