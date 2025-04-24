fn main()
{
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-arg=-Wl,--gc-sections");
    println!("cargo:rustc-link-arg=-Wl,--strip-all");

    let targets = configure_targets();

    for (name, file) in targets
    {
        cc::Build::new()
            .file(file)
            .compile(name);
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

        targets
    }
}