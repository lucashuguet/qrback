{
  description = "cli tool to backup files as qr codes";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
  };

  outputs = { nixpkgs, ... }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        openssl pkg-config cargo-nextest cargo-expand
        zbar qrencode jabcode-reader jabcode-writer
        grim slurp scrot
      ];
    };
  };
}
