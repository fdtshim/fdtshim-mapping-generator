{ pkgs ? (import (import ./npins).nixpkgs {}) }:
pkgs.callPackage ./package.nix {}
