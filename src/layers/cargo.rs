use std::process::Command;

use libcnb::{
    Env,
    data::layer_name,
    layer::{LayerRef, UncachedLayerDefinition},
};

use crate::{
    RustBuildpack,
    error::RustBuildPackError,
    metatada::{CargoDeps, CargoToml},
};

use super::rustup::RustupLayerRef;

pub type CargoLayerRef = LayerRef<RustBuildpack, (), ()>;
pub type DepsCacheMetadata = CargoDeps;

#[derive(Debug)]
pub enum CacheMissCause {
    NoMiss,
    DepsChanged,
}

pub fn handle(
    context: &libcnb::build::BuildContext<RustBuildpack>,
    rustup_layer: &RustupLayerRef,
    _metadata: &CargoToml,
) -> Result<CargoLayerRef, libcnb::Error<RustBuildPackError>> {
    build_app(context, rustup_layer)
}
pub fn build_app(
    context: &libcnb::build::BuildContext<RustBuildpack>,
    rustup_layer: &RustupLayerRef,
) -> Result<CargoLayerRef, libcnb::Error<RustBuildPackError>> {
    let cargo_layer = context.uncached_layer(
        layer_name!("cargo-publish"),
        UncachedLayerDefinition {
            build: true,
            launch: false,
        },
    )?;

    let env = rustup_layer
        .read_env()?
        .apply(libcnb::layer_env::Scope::Build, &Env::from_current());

    Command::new("cargo")
        .arg("install")
        .arg("--path=.")
        .arg("--root")
        .arg(cargo_layer.path())
        .envs(&env)
        .spawn()
        .map_err(RustBuildPackError::CargoExec)?
        .wait()
        .map_err(RustBuildPackError::CargoExec)?;

    Ok(cargo_layer)
}

// pub fn cache_deps(
//     context: &libcnb::build::BuildContext<RustBuildpack>,
//     rustup_layer: &RustupLayerRef,
//     metadata: &CargoToml,
// ) -> Result<LayerRef<RustBuildpack, (), CacheMissCause>, libcnb::Error<RustBuildPackError>> {
//     let cargo_cache_layer = context.cached_layer(
//         layer_name!("cargo-cache-deps"),
//         CachedLayerDefinition {
//             build: true,
//             launch: false,
//             invalid_metadata_action: &|_| {
//                 println!("cargo dep metadata is invalid.");
//                 InvalidMetadataAction::DeleteLayer
//             },
//             restored_layer_action: &|deps_cache: &DepsCacheMetadata, _| {
//                 if deps_cache == &metadata.dependencies {
//                     (RestoredLayerAction::KeepLayer, CacheMissCause::NoMiss)
//                 } else {
//                     (
//                         RestoredLayerAction::DeleteLayer,
//                         CacheMissCause::DepsChanged,
//                     )
//                 }
//             },
//         },
//     )?;
//
//     match &cargo_cache_layer.state {
//         LayerState::Restored { .. } => {
//             println!("dependencies have not changed. keeping build cache.");
//         }
//         LayerState::Empty { cause } => {
//             println!("rebuilding deps. {:?}", cause);
//
//             let env = rustup_layer
//                 .read_env()?
//                 .apply(libcnb::layer_env::Scope::Build, &Env::from_current());
//
//             Command::new("cargo")
//                 .arg("build")
//                 .arg("--release")
//                 .envs(&env)
//                 .spawn()
//                 .map_err(RustBuildPackError::CargoExec)?
//                 .wait()
//                 .map_err(RustBuildPackError::CargoExec)?;
//         }
//     }
//
//     Ok(cargo_cache_layer)
// }
