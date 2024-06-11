{ pkgs }:

let
  overlays = pkgs.appendOverlays [(final: super: {
  })];
in
rec {
  inherit overlays;
  #shell = overlays.callPackage (
  #  { mkShell
  #  , rustToolchain
  #  , rustTarget
  #  , OVMF
  #  , uefi-run
  #  , qemu
  #  , dtc
  #  }:
  #  mkShell {
  #    RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/";
  #    OVMF = OVMF.fd;
  #    RUST_TARGET = rustTarget;

  #    depsBuildBuild = [
  #      rustToolchain
  #      uefi-run
  #      qemu
  #      dtc
  #    ];
  #  }
  #) {
  #  inherit rustToolchain;
  #  inherit rustTarget;
  #};
  fdtshim-mapping-generator = overlays.callPackage ./package.nix { };
}
