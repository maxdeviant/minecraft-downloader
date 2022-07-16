with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "minecraft-downloader";

  buildInputs = [
      stdenv
      pkg-config
  ];
}
