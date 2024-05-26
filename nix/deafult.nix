{
  lib,
  stdenv,
  rustPlatform,
  fetchFromGitHub,
  openssl,
  pkg-config,
  glibc,
  libsoup,
  cairo,
  gtk3,
  webkitgtk,
  darwin,
  pkgs,
}:

let
  inherit (darwin.apple_sdk.frameworks) CoreServices Security SystemConfiguration;
in
rustPlatform.buildRustPackage rec {
  pname = "TeaClient";
  version = "0.1.0-beta";

  src = fetchFromGitHub {
    owner = "TeaClientMC";
    repo = "launcher";
    rev = version;
    hash = lib.misc.fakeHash;
  };

  # Manually specify the sourceRoot since this crate depends on other crates in the workspace. Relevant info at
  # https://discourse.nixos.org/t/difficulty-using-buildrustpackage-with-a-src-containing-multiple-cargo-workspaces/10202
  # sourceRoot = "";

  nativeBuildInputs = [ pkg-config ];

  buildInputs =
    [
      openssl
      pkgs.bun
    ]
    ++ lib.optionals stdenv.isLinux [
      glibc
      libsoup
      cairo
      gtk3
      webkitgtk
    ]
    ++ lib.optionals stdenv.isDarwin [
      CoreServices
      Security
      SystemConfiguration
    ];

  strictDeps = true;

  meta = {
    description = "A Nice Opensourced Client.";
    homepage = "https://teaclient.net";
    license = with lib.licenses; [ epl20 ];
    mainProgram = "teaclient";
    maintainers = with lib.maintainers; [ eveeifyeve ];
  };
}
