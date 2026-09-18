{ pkgs, ... }:

{
  name = "tdy";

  packages = [
    pkgs.git

    # Fonts and TeX used by convert.sh (pandoc -> PDF via xelatex).
    pkgs.dejavu_fonts
    pkgs.dina-font
    pkgs.fira-code
    pkgs.fira-code-symbols
    pkgs.liberation_ttf
    pkgs.mplus-outline-fonts.githubRelease
    pkgs.noto-fonts
    pkgs.noto-fonts-cjk-sans
    pkgs.pandoc
    pkgs.proggyfonts

    # https://github.com/Ptival/config/blob/bd89aed366de07c8ec683b8aefbd84cc21312519/nixos/nixpkgs/texlive.nix#L34
    (pkgs.texlive.combine {
      inherit (pkgs.texlive) scheme-small soul dejavu
      collection-xetex fncychap titlesec tabulary varwidth multirow
      hanging adjustbox collectbox stackengine sectsty tocloft
      newunicodechar etoc framed capt-of wrapfig needspace dejavu-otf helvetic upquote
      xcolor xifthen ifmtarg datetime extsizes memorygraphs currvita cbfonts xetex xstring
      inconsolata lato latexmk lm lualatex-math xurl twemojis
      luatex luatex85 luatexbase fontspec graphics stix
      ;
    })
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    version = "1.97.1";
    lsp.enable = true;
  };

  env = {
    RUST_BACKTRACE = "full";
    NIX_ENFORCE_PURITY = 0;
  };

  enterShell = ''
    export PATH="$DEVENV_ROOT/target/release:$PATH"

    # Socket Firewall (`sfw cargo ...`) routes package traffic through a local
    # MITM proxy and points SSL_CERT_FILE at its own CA. The rust-overlay cargo
    # wrapper loads nixpkgs' libcurl, whose OpenSSL prefers NIX_SSL_CERT_FILE
    # over SSL_CERT_FILE, so cargo verified the proxied connection against the
    # nix bundle and failed with "unable to get local issuer certificate".
    # Hand the nix bundle over via SSL_CERT_FILE and drop the nix-specific
    # variable so sfw's override is honoured when present.
    if [ -n "''${NIX_SSL_CERT_FILE:-}" ]; then
      export SSL_CERT_FILE="''${SSL_CERT_FILE:-$NIX_SSL_CERT_FILE}"
      unset NIX_SSL_CERT_FILE
    fi

    echo "~~~ tdy ~~~"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo "RUST_SRC_PATH: $RUST_SRC_PATH"
  '';
}
