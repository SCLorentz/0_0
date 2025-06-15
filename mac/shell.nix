{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    llvm_15
    clang_15
    lld_15
    pkgsCross.aarch64-embedded.buildPackages.binutils
    pkgsCross.aarch64-embedded.buildPackages.gcc
  ];

  shellHook = ''
    export LD=aarch64-none-elf-ld
    export CC=aarch64-none-elf-gcc
  '';
}