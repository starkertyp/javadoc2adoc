let
  # this will use aarch64 binaries from binary cache, so no need to build those
  pkgsArm = import <nixpkgs> {
    config = { };
    overlays = [ ];
    system = "aarch64-linux";
  };

  # these will be your cross packages
  pkgsCross = import <nixpkgs> {

    overlays =
      [ (self: super: { inherit (pkgsArm) rustPlatform dockerTools; }) ];
    crossSystem = { config = "aarch64-unknown-linux-gnu"; };
  };
  toml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
  package = pkgsCross.callPackage ./package.nix { };

in pkgsCross.dockerTools.buildImage {
  name = toml.package.name;
  tag = "${toml.package.version}-arm";

  runAsRoot = ''
    #!${pkgsCross.runtimeShell}
    mkdir -p /data
  '';

  config = {
    Entrypoint = [ "${package}/bin/${toml.package.name}" ];
    Volumes = { "/data" = { }; };
  };

}
