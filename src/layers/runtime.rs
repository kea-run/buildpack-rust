use std::path::Path;

use libcnb::{
    data::layer_name,
    layer::{LayerRef, UncachedLayerDefinition},
};

use crate::{RustBuildpack, error::RustBuildPackError};

use super::cargo::CargoLayerRef;

pub fn handle(
    context: &libcnb::build::BuildContext<RustBuildpack>,
    cargo_layer: &CargoLayerRef,
) -> Result<LayerRef<RustBuildpack, (), ()>, libcnb::Error<RustBuildPackError>> {
    let runtime_layer = context.uncached_layer(
        layer_name!("runtime"),
        UncachedLayerDefinition {
            build: false,
            launch: true,
        },
    )?;

    std::fs::rename(
        cargo_layer.path().join("bin"),
        &runtime_layer.path().join("bin"),
    )
    .map_err(RustBuildPackError::RuntimePrepareBin)?;

    clear_dir(&context.app_dir)?;

    symlink_dir_content(&runtime_layer.path(), &context.app_dir)?;

    Ok(runtime_layer)
}

fn clear_dir(path: &Path) -> Result<(), libcnb::Error<RustBuildPackError>> {
    for item in path
        .read_dir()
        .map_err(RustBuildPackError::RuntimeCleanLayersDir)?
    {
        if let Ok(item) = item {
            if item.file_type().unwrap().is_dir() {
                std::fs::remove_dir_all(item.path())
                    .map_err(RustBuildPackError::RuntimeCleanLayersDir)?
            } else {
                std::fs::remove_file(item.path())
                    .map_err(RustBuildPackError::RuntimeCleanLayersDir)?
            }
        }
    }
    Ok(())
}

fn copy_dir(src_dir: &Path, dest: &Path) -> Result<(), libcnb::Error<RustBuildPackError>> {
    for item in src_dir
        .read_dir()
        .map_err(RustBuildPackError::RuntimeCreateBin)?
    {
        if let Ok(item) = item {
            let path = item.path();
            let suffix = path
                .strip_prefix(&src_dir)
                .map_err(RustBuildPackError::StripPrefix)?;

            let dest = dest.join(suffix);

            if item.file_type().unwrap().is_dir() {
                std::fs::create_dir(&dest).map_err(RustBuildPackError::RuntimeCreateBin)?;
                copy_dir(item.path().as_ref(), &dest)?;
            } else {
                std::fs::copy(&path, dest).map_err(RustBuildPackError::RuntimeCreateBin)?;
            }
        }
    }
    Ok(())
}

fn symlink_dir_content(
    src_dir: &Path,
    dest: &Path,
) -> Result<(), libcnb::Error<RustBuildPackError>> {
    for item in src_dir
        .read_dir()
        .map_err(RustBuildPackError::RuntimeCreateBin)?
    {
        if let Ok(item) = item {
            let path = item.path();
            let suffix = path
                .strip_prefix(&src_dir)
                .map_err(RustBuildPackError::StripPrefix)?;

            let dest = dest.join(suffix);

            std::os::unix::fs::symlink(path, dest).map_err(RustBuildPackError::Symlink)?;
        }
    }
    Ok(())
}
