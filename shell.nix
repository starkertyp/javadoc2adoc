{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ cargo rust-analyzer rustfmt clippy rustc sqlite pkg-config ];

  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  RUST_LOG = "trace";
}
