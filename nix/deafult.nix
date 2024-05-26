{
  lib,
  fetchFromGitHub,
  buildNpmPackage,
  openssl,
  pkg-config,
  freetype,
  libsoup_3,
  gtk3,
  webkitgtk_4_1,
  nodejs-slim,
  cargo-tauri,
  cargo,
  rustPlatform,
  rustc,
  bun,
}:

buildNpmPackage rec {

  pname = "TeaLauncherMC";
  version = "0.0.1-dev";

  src = ./..;

  npmDepsHash = "sha256-dSNAfRntOemtEl/b0zcu22nfGjE6KMHJuh5zYiR79fE=";

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = src + "/src-tauri/Cargo.lock";
    outputHashes = {
      # "fix-path-env-0.0.0" = "sha256-kSpWO2qMotpsYKJokqUWCUzGGmNOazaREDLjke4/CtE=";
      "tauri-plugin-clipboard-manager-2.1.0-beta.1" = "sha256-2F+OkX92B2/aJva86orotHc7mYUZuaYAmKx50dDp2Sc=";
    };
  };

  configurePhase = ''
    export HOME=$(mktemp -d)
  '';

  preBuild = ''
    cargo tauri build -b deb
  '';

  cargoRoot = "src-tauri/";

  preInstall = ''
    mv src-tauri/target/release/bundle/deb/*/data/usr/ "$out"
  '';

  nativeBuildInputs = [
    pkg-config
    rustPlatform.cargoSetupHook
    cargo
    rustc
    cargo-tauri
    nodejs-slim
    openssl
    bun
  ];

  buildInputs = [
    openssl
    freetype
    libsoup_3
    gtk3
    webkitgtk_4_1
  ];

  meta = {
    description = "A Nice Opensourced Client.";
    homepage = "https://teaclient.net";
    license = with lib.licenses; [ epl20 ];
    mainProgram = "teaclient";
    maintainers = with lib.maintainers; [ eveeifyeve ];
  };
}
