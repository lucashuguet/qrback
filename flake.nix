{
  description = "cli tool to backup files as qr codes";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
  };

  outputs = { nixpkgs, ... }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    qrback = pkgs.rustPlatform.buildRustPackage rec {
      pname = "qrback";
      version = "0.1.0";
      cargoLock.lockFile = ./Cargo.lock;
      src = pkgs.lib.cleanSource ./.;
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        openssl pkg-config cargo-nextest cargo-expand
        zbar qrencode jabcode-reader jabcode-writer
        grim slurp scrot
        qrback
        (pkgs.writeScriptBin "qregion" (builtins.readFile ./scripts/qregion.sh))
        (pkgs.writeScriptBin "qregion4" (builtins.readFile ./scripts/qregion4.sh))
        (pkgs.writeScriptBin "qregion8" (builtins.readFile ./scripts/qregion8.sh))
      ];
    };
  };
}
