{
  pkgs ? import <nixpkgs> { },
}:
let
  overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
in
pkgs.callPackage (
  {
    stdenv,
    mkShell,
    rustup,
    rustPlatform
  }:
  mkShell {
    strictDeps = true;
    nativeBuildInputs = [
      rustup
      rustPlatform.bindgenHook
    ];
    buildInputs =
      [
        
      ];
    RUSTC_VERSION = overrides.toolchain.channel;
    shellHook = ''
      export PATH="''${CARGO_HOME:-~/.cargo}/bin":"$PATH"
      export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"
    '';
  }
) {}
