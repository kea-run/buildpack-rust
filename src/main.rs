mod error;
mod layers;
mod metatada;

use error::RustBuildPackError;

use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::data::launch::{LaunchBuilder, ProcessBuilder, WorkingDirectory};
use libcnb::data::process_type;
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::{GenericMetadata, GenericPlatform};
use libcnb::{Buildpack, buildpack_main};
use metatada::CargoToml;

const CARGO_METATDATA_FILE: &str = "Cargo.toml";

pub(crate) struct RustBuildpack;

impl Buildpack for RustBuildpack {
    type Platform = GenericPlatform;
    type Metadata = GenericMetadata;
    type Error = RustBuildPackError;

    fn detect(&self, context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        if std::fs::exists(context.app_dir.join(&CARGO_METATDATA_FILE))
            .map_err(RustBuildPackError::BuildpackDetection)?
        {
            DetectResultBuilder::pass().build()
        } else {
            DetectResultBuilder::fail().build()
        }
    }

    fn build(&self, context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        let metadata: CargoToml = {
            let cargo_toml = std::fs::read_to_string(&CARGO_METATDATA_FILE)
                .map_err(RustBuildPackError::MetadataRead)?;
            toml::from_str(&cargo_toml).map_err(RustBuildPackError::MetadataParse)?
        };

        println!("Started build for {}", metadata.package_name());
        println!(
            "The build is running on: {} ({})!",
            context.target.os, context.target.arch
        );

        let rustup_layer = layers::rustup::handle(&context)?;
        let cargo_layer = layers::cargo::handle(&context, &rustup_layer, &metadata)?;
        let _runtime_layer = layers::runtime::handle(&context, &cargo_layer)?;

        let process = ProcessBuilder::new(
            process_type!("app"),
            [context
                .app_dir
                .join("bin")
                .join(metadata.package_name())
                .as_os_str()
                .to_str()
                .unwrap()],
        )
        .default(true)
        .build();

        let launch_builder = LaunchBuilder::new().process(process).build();
        BuildResultBuilder::new().launch(launch_builder).build()
    }
}

buildpack_main!(RustBuildpack);
