{ lib
, path
, stdenv
, rustPlatform
}:

let
  cargo_toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
in
rustPlatform.buildRustPackage rec {
  pname = cargo_toml.package.name;
  version = cargo_toml.package.version;
  src = lib.cleanSource ./.;
  cargoHash = "sha256-RQe1B43/EHMYAuihwgzupzYWCikJawTJl0gKF55zxtY=";
  doCheck = false;

  passthru = {
    inherit rustPlatform;
  };
}
