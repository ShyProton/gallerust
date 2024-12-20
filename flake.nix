{
  description = "Template cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    nix-filter.url = "github:numtide/nix-filter";
    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    fenix,
    nix-filter,
    flake-utils,
    advisory-db,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };

      inherit (pkgs) lib;

      craneLib = crane.mkLib nixpkgs.legacyPackages.${system};
      src = nix-filter.lib.filter {
        root = craneLib.path ./.;
        include = [
          "src"
          "Cargo.lock"
          (nix-filter.lib.matchExt "rs")
          (nix-filter.lib.matchExt "toml")
        ];
      };

      # Common arguments can be set here to avoid repeating them later
      commonArgs = {
        inherit src;

        buildInputs =
          [
            # Add additional build inputs here
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];

        # Additional environment variables can be set directly
        # MY_CUSTOM_VAR = "some value";
      };

      craneLibLLvmTools =
        craneLib.overrideToolchain
        (fenix.packages.${system}.complete.withComponents [
          "cargo"
          "llvm-tools"
          "rustc"
        ]);

      # Build *just* the cargo dependencies, so we can reuse
      # all of that work (e.g. via cachix) when running in CI
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      # Build the actual crate itself, reusing the dependency
      # artifacts from above.
      crate = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
        });

      dockerImage = pkgs.dockerTools.buildImage {
        name = "template";
        tag = "latest";
        copyToRoot = [crate];
        config = {
          Cmd = ["${crate}/bin/template"];
        };
      };
    in {
      checks =
        {
          # Build the crate as part of `nix flake check` for convenience
          inherit crate;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          crate-clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = ''                --all-targets -- --deny warnings \
                              -W clippy::pedantic \
                              -W clippy::nursery \
                              -W clippy::unwrap_used \
                              -W clippy::expect_used
              '';
            });

          crate-doc = craneLib.cargoDoc (commonArgs
            // {
              inherit cargoArtifacts;
            });

          # Check formatting
          crate-fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Audit dependencies
          crate-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `crate` if you do not want
          # the tests to run twice
          crate-nextest = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            });
        }
        // lib.optionalAttrs (system == "x86_64-linux") {
          # NB: cargo-tarpaulin only supports x86_64 systems
          # Check code coverage (note: this will not upload coverage anywhere)
          crate-coverage = craneLib.cargoTarpaulin (commonArgs
            // {
              inherit cargoArtifacts;
            });
        };

      packages = {
        inherit crate dockerImage;
        default = crate;
        crate-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs
          // {
            inherit cargoArtifacts;
          });
      };

      apps.default = flake-utils.lib.mkApp {
        drv = crate;
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks.${system};

        # Additional dev-shell environment variables can be set directly
        # MY_CUSTOM_DEVELOPMENT_VAR = "something else";
        RUST_LOG = "debug";

        # Extra inputs can be added here
        nativeBuildInputs = with pkgs; [
          rustc
          bacon
          cargo-modules
          dive
          pkg-config
          glib.dev
          gtk4.dev
          cairo.dev
          librsvg.dev
          libadwaita.dev
          adwaita-icon-theme
        ];
      };
    });
}
