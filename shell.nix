{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
  ];
  buildInputs = with pkgs; [
    rustc
    rust-analyzer
    rustfmt
  ] ++ lib.optional stdenv.isDarwin libiconv;
}
