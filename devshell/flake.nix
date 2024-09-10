{
  description = "Rust devshell";

  inputs = {
    # nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rustaceanvim.url = "github:mrcjkb/rustaceanvim";
  };

  outputs =
    { nixpkgs, rustaceanvim, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          pkgs.cargo
          pkgs.rustc
          pkgs.rustfmt
          pkgs.rustPackages.clippy
          pkgs.rust-analyzer

          pkgs.gdb
          pkgs.gdbgui
          rustaceanvim.outputs.packages.x86_64-linux.codelldb
        ];

        RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
      };
    };
}
