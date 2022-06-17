{ pkgs ? import <nixpkgs> {}}:

pkgs.stdenv.mkDerivation {
  pname = "temp-prometheus";
  version = "1.0";
  src = ./.;
  nativeBuildInputs = [ pkgs.rustc ];
  makeFlags = [ "PREFIX=$(out)" ];
}
