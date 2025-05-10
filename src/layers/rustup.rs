use std::{io::Read, process::Command};
use ureq;

use libcnb::{
    data::layer_name,
    generic::GenericMetadata,
    layer::{
        CachedLayerDefinition, InvalidMetadataAction, LayerRef, LayerState, RestoredLayerAction,
    },
    layer_env::{LayerEnv, ModificationBehavior, Scope},
};

use crate::{RustBuildpack, error::RustBuildPackError};

pub type RustupLayerRef = LayerRef<RustBuildpack, (), ()>;

pub fn handle(
    context: &libcnb::build::BuildContext<RustBuildpack>,
) -> Result<RustupLayerRef, libcnb::Error<RustBuildPackError>> {
    let rustup_layer = context.cached_layer(
        layer_name!("rustup"),
        CachedLayerDefinition {
            build: true,
            launch: false,
            invalid_metadata_action: &|_| {
                println!("rustup metadata is invalid.");
                InvalidMetadataAction::DeleteLayer
            },
            restored_layer_action: &|_: &GenericMetadata, _| {
                println!("restoring rustup");
                RestoredLayerAction::KeepLayer
            },
        },
    )?;

    println!("Downloading rustup....");
    match rustup_layer.state {
        LayerState::Restored { .. } => {}
        LayerState::Empty { .. } => {
            let rustup_path = {
                let rustup_path = rustup_layer.path().join("rustup.sh");

                let req = ureq::get("https://sh.rustup.rs")
                    .call()
                    .map_err(RustBuildPackError::RustupDownload)?
                    .into_body();

                let mut fs =
                    std::fs::File::create(&rustup_path).map_err(RustBuildPackError::RustupSave)?;
                let mut web = req.into_reader();

                std::io::copy(&mut web as &mut dyn Read, &mut fs)
                    .map_err(RustBuildPackError::RustupSave)?;

                rustup_path
            };

            println!("Installing rust with rustup");

            let rustup_home = rustup_layer.path().join("rustup");
            let cargo_home = rustup_layer.path().join("cargo");

            Command::new("/bin/sh")
                .arg(rustup_path)
                .arg("--profile=minimal")
                .arg("--default-toolchain=stable")
                .arg("-y")
                .env("RUSTUP_HOME", &rustup_home)
                .env("CARGO_HOME", &cargo_home)
                .spawn()
                .map_err(RustBuildPackError::RustupExec)?
                .wait()
                .map_err(RustBuildPackError::RustupExec)?;

            rustup_layer.write_env(
                LayerEnv::new()
                    .chainable_insert(Scope::Build, ModificationBehavior::Delimiter, "PATH", ":")
                    .chainable_insert(
                        Scope::Build,
                        ModificationBehavior::Append,
                        "PATH",
                        cargo_home.join("bin"),
                    )
                    .chainable_insert(
                        Scope::Build,
                        ModificationBehavior::Override,
                        "CARGO_HOME",
                        &cargo_home,
                    )
                    .chainable_insert(
                        Scope::Build,
                        ModificationBehavior::Override,
                        "RUSTUP_HOME",
                        &rustup_home,
                    ),
            )?;
        }
    }

    Ok(rustup_layer)
}
