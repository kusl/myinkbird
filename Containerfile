# Containerfile for the inkbird-collector binary (OCI image, built with podman).
#
# Multi-stage: compile with the full Fedora toolchain, then ship the stripped
# binary on fedora-minimal. Both stages use the current Fedora release so their
# glibc versions line up. See docs/adr/0004 and docs/adr/0009.

FROM registry.fedoraproject.org/fedora:latest AS build

# gcc + pkg-config + D-Bus headers are needed to build btleplug's Linux
# backend (it links libdbus); rustup provides the Rust toolchain.
RUN dnf -y install gcc pkgconf-pkg-config dbus-devel rustup \
    && rustup-init -y --profile minimal --default-toolchain stable --no-modify-path \
    && dnf clean all
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /src
# Copy the committed Cargo.lock too, so the build uses the exact, pinned
# dependency versions - in particular `time` >= 0.3.47, which avoids
# RUSTSEC-2026-0009. See docs/adr/0007 and docs/adr/0009.
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo build --release -p inkbird-collector

FROM registry.fedoraproject.org/fedora-minimal:latest AS runtime

# libdbus is the only runtime dependency (btleplug talks to the host
# bluetoothd over the D-Bus system bus).
RUN microdnf install -y dbus-libs && microdnf clean all

COPY --from=build /src/target/release/inkbird-collector /usr/local/bin/inkbird-collector

# BlueZ's D-Bus policy only lets root (or the bluetooth group) talk to
# org.bluez, so the collector runs as root and the stack is launched rootful.
# See docs/adr/0005 and docs/bluetooth.md.
ENTRYPOINT ["/usr/local/bin/inkbird-collector"]
CMD ["collect"]
