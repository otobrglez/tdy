{ pkgs, lib, config, inputs, ... }:

{
  env.NIX_ENFORCE_PURITY = 0;
  env.RUST_BACKTRACE = "full";

  packages = [ 
    pkgs.dejavu_fonts
    pkgs.dina-font
    pkgs.fira-code
    pkgs.fira-code-symbols
    pkgs.git
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
    version = "1.91.1";
  };

  enterShell = ''
    echo "~~~ tdy ~~~"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo "RUST_SRC_PATH: $RUST_SRC_PATH"
  '';

  enterTest = ''
    cargo test -- --nocapture
  '';
}
