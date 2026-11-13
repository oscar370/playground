{ pkgs, lib, ... }:

{
  languages = {
    rust = {
      enable = true;
      channel = "stable";
      targets = [ "wasm32-unknown-unknown" ];
      components = [
        "rustc"
        "cargo"
        "clippy"
        "rustfmt"
        "rust-analyzer"
        "rust-src"
      ];
    };

    javascript = {
      enable = true;
      pnpm.enable = true;
    };
  };

  packages = with pkgs; [
    dioxus-cli
  ];
}
