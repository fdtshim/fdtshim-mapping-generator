{ pkgs ? (import (import ./npins).nixpkgs {}) }:
(pkgs.callPackage ./package.nix {}).overrideAttrs(oldAttrs: {
  nativeBuildInputs = with pkgs; [
    # For Rust development
    cargo
    rustfmt
    clippy

    # For manipulating output
    dtc
  ];
})
