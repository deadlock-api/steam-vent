{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.11";
    flakelight = {
      url = "github:nix-community/flakelight";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    mill-scale = {
      url = "git+https://codeberg.org/icewind/mill-scale.git";
      inputs.flakelight.follows = "flakelight";
    };
  };
  outputs = {mill-scale, ...}:
    mill-scale ./. {
      extraFiles = ["system.pem"];
      packages = {
        proto-builder = {craneLib, ...}:
          craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./protobuf/build;
            doCheck = false;
            strictDeps = true;
          };
      };
    };
}
