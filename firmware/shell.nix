{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    # Build toolchain.
    rustup

    # Debugging tools.
    gcc-arm-embedded
  ];
}
