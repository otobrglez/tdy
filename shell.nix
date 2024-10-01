let
  pkgs = import <nixpkgs> {};
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = [pkgs.rustfmt pkgs.rustc pkgs.cargo pkgs.rustPlatform.rustcSrc pkgs.clippy];
  };
in with pkgs;

mkShell {
  name = "tdy";
  buildInputs = [
    libiconv
    openssl
    pkg-config
    rust-analyzer
    rust-toolchain
  ] ++ 
  lib.optionals (!stdenv.isDarwin) [
    procps
  ] ++
  lib.optionals (stdenv.isDarwin) [
    darwin.apple_sdk.frameworks.Security
    darwin.libobjc
  ]
  ;

  NIX_ENFORCE_PURITY = 0;
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  #  export RUST_SRC_PATH=${pkgs.rustPlatform.rustcSrc}
  
  shellHook = ''
    echo "Setting tdy alias."
    alias tdy=./target/debug/tdy
  '';

}

