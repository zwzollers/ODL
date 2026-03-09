{
  description = "Rust flake";
  inputs =
    {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # or whatever vers
    };
  
  outputs = { self, nixpkgs, ... }@inputs:
    let
      system = "x86_64-linux"; # your version
      pkgs = nixpkgs.legacyPackages.${system}; 
      libPath = with pkgs; lib.makeLibraryPath [
           libGL
           libxkbcommon
           wayland
    ];   
    in
    {
      devShells.${system}.default = pkgs.mkShell
      {
        packages = with pkgs; [ rustc cargo wayland ]; # whatever you need
        
        RUST_LOG = "debug";
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        LD_LIBRARY_PATH = libPath;
      };
    };
}