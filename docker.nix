{ pkgs ? import <nixpkgs> { } }:
let
  toml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
  package = pkgs.callPackage ./package.nix { };
in pkgs.dockerTools.buildImage {
  name = toml.package.name;
  tag = "${toml.package.version}-amd64";

  runAsRoot = ''
    #!${pkgs.runtimeShell}
    mkdir -p /data
  '';

  config = {
    Entrypoint = [ "${package}/bin/${toml.package.name}" ];
    Volumes = { "/data" = { }; };
  };
}
