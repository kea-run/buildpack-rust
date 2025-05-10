{
  outputs =
    { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
      pkgsStatic = pkgs.pkgsStatic;
      pkgsCross = pkgs.pkgsCross;
      inherit (pkgs) stdenv;

      toSystem =
        flakeObj:
        let
          createAttr = (parts: attr: parts // { ${attr}.${system} = flakeObj.${attr}; });
        in
        builtins.foldl' createAttr { } (builtins.attrNames flakeObj);

      NIX_LD_LIBRARY_PATH = [ stdenv.cc.cc ];
    in
    toSystem {

      devShells.default = pkgs.mkShell {
        name = "buildpack-rust";

        buildInputs = with pkgs; [
          rustup
          pkg-config
          pkgsStatic.gccStdenv
          pkgsCross.aarch64-multiplatform-musl.zlib
          # pkgsCross.aarch64-unknown-linux-gnu.gcc
          #pkgsStatic.gcc
        ];

        shellHook = ''
          export LD_LIBRARY_PATH=NIX_LD_LIBRARY_PATH
          export PKG_CONFIG_PATH="${pkgsStatic.openssl.dev}/lib/pkgconfig"
          #export OPENSSL_NO_VENDOR=1
          #export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER="${pkgsCross.aarch64-multiplatform-musl.gcc}/bin/ld"
        '';
      };
    };
}

