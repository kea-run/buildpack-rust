
.PHONY: package
package:
	cargo libcnb package --no-cross-compile-assistance

.PHONY: package-arm
package-arm:
	cargo libcnb package --no-cross-compile-assistance --target aarch64-unknown-linux-musl

.PHONY: pack-build
pack-build:
	pack build my-image-name \
	--buildpack packaged/x86_64-unknown-linux-musl/debug/kea-run_buildpack-run \
	--trust-extra-buildpacks \
	--path /path/to/application
