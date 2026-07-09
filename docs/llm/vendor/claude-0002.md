14
87
100

Github actions are failing 
in addition please review the entire dump.txt to make sure the copy paste was correct and there are no defects 
and please give me FULL files and file paths for all files that need to change 

lint, build, test
failed now in 16s 
Run ./scripts/ci.sh
[myinkbird] installing build dependencies (C compiler, pkg-config, D-Bus headers) via apt-get
Get:1 file:/etc/apt/apt-mirrors.txt Mirrorlist [144 B]
Hit:2 http://azure.archive.ubuntu.com/ubuntu noble InRelease
Get:3 http://azure.archive.ubuntu.com/ubuntu noble-updates InRelease [126 kB]
Get:6 https://packages.microsoft.com/repos/azure-cli noble InRelease [3564 B]
Get:4 http://azure.archive.ubuntu.com/ubuntu noble-backports InRelease [126 kB]
Get:7 https://packages.microsoft.com/ubuntu/24.04/prod noble InRelease [3600 B]
Get:5 http://azure.archive.ubuntu.com/ubuntu noble-security InRelease [126 kB]
Get:8 https://packages.microsoft.com/repos/azure-cli noble/main amd64 Packages [2314 B]
Get:9 https://dl.google.com/linux/chrome-stable/deb stable InRelease [1825 B]
Get:10 http://azure.archive.ubuntu.com/ubuntu noble-updates/main amd64 Packages [1079 kB]
Get:11 http://azure.archive.ubuntu.com/ubuntu noble-updates/main Translation-en [267 kB]
Get:12 http://azure.archive.ubuntu.com/ubuntu noble-updates/main amd64 Components [180 kB]
Get:13 http://azure.archive.ubuntu.com/ubuntu noble-updates/main amd64 c-n-f Metadata [17.7 kB]
Get:14 http://azure.archive.ubuntu.com/ubuntu noble-updates/universe amd64 Packages [1659 kB]
Get:15 http://azure.archive.ubuntu.com/ubuntu noble-updates/universe Translation-en [327 kB]
Get:16 http://azure.archive.ubuntu.com/ubuntu noble-updates/universe amd64 Components [388 kB]
Get:17 http://azure.archive.ubuntu.com/ubuntu noble-updates/universe amd64 c-n-f Metadata [34.9 kB]
Get:18 http://azure.archive.ubuntu.com/ubuntu noble-updates/restricted amd64 Packages [1196 kB]
Get:19 http://azure.archive.ubuntu.com/ubuntu noble-updates/restricted Translation-en [271 kB]
Get:20 http://azure.archive.ubuntu.com/ubuntu noble-updates/multiverse Translation-en [11.8 kB]
Get:21 http://azure.archive.ubuntu.com/ubuntu noble-updates/multiverse amd64 Components [940 B]
Get:22 http://azure.archive.ubuntu.com/ubuntu noble-backports/main amd64 Components [5768 B]
Get:23 http://azure.archive.ubuntu.com/ubuntu noble-backports/universe amd64 Components [10.5 kB]
Get:24 https://packages.microsoft.com/ubuntu/24.04/prod noble/main amd64 Packages [207 kB]
Get:25 https://packages.microsoft.com/ubuntu/24.04/prod noble/main arm64 Packages [174 kB]
Get:26 https://packages.microsoft.com/ubuntu/24.04/prod noble/main armhf Packages [11.7 kB]
Get:27 http://azure.archive.ubuntu.com/ubuntu noble-security/main amd64 Packages [827 kB]
Get:28 http://azure.archive.ubuntu.com/ubuntu noble-security/main Translation-en [187 kB]
Get:29 http://azure.archive.ubuntu.com/ubuntu noble-security/main amd64 Components [44.9 kB]
Get:30 http://azure.archive.ubuntu.com/ubuntu noble-security/main amd64 c-n-f Metadata [11.9 kB]
Get:31 http://azure.archive.ubuntu.com/ubuntu noble-security/universe amd64 Packages [1173 kB]
Get:32 http://azure.archive.ubuntu.com/ubuntu noble-security/universe Translation-en [230 kB]
Get:33 http://azure.archive.ubuntu.com/ubuntu noble-security/universe amd64 Components [76.3 kB]
Get:34 http://azure.archive.ubuntu.com/ubuntu noble-security/universe amd64 c-n-f Metadata [24.2 kB]
Get:35 http://azure.archive.ubuntu.com/ubuntu noble-security/restricted amd64 Packages [1133 kB]
Get:36 http://azure.archive.ubuntu.com/ubuntu noble-security/restricted Translation-en [259 kB]
Get:37 http://azure.archive.ubuntu.com/ubuntu noble-security/multiverse Translation-en [10.1 kB]
Get:38 https://dl.google.com/linux/chrome-stable/deb stable/main amd64 Packages [1220 B]
Fetched 10.2 MB in 1s (8022 kB/s)
Reading package lists...
Reading package lists...
Building dependency tree...
Reading state information...
gcc is already the newest version (4:13.2.0-7ubuntu1).
pkg-config is already the newest version (1.8.1-2build1).
The following NEW packages will be installed:
  libdbus-1-dev
0 upgraded, 1 newly installed, 0 to remove and 52 not upgraded.
Need to get 190 kB of archives.
After this operation, 1007 kB of additional disk space will be used.
Get:1 file:/etc/apt/apt-mirrors.txt Mirrorlist [144 B]
Get:2 http://azure.archive.ubuntu.com/ubuntu noble-updates/main amd64 libdbus-1-dev amd64 1.14.10-4ubuntu4.1 [190 kB]
Fetched 190 kB in 0s (2737 kB/s)
Selecting previously unselected package libdbus-1-dev:amd64.
(Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
(Reading database ... 20%
(Reading database ... 25%
(Reading database ... 30%
(Reading database ... 35%
(Reading database ... 40%
(Reading database ... 45%
(Reading database ... 50%
(Reading database ... 55%
(Reading database ... 60%
(Reading database ... 65%
(Reading database ... 70%
(Reading database ... 75%
(Reading database ... 80%
(Reading database ... 85%
(Reading database ... 90%
(Reading database ... 95%
(Reading database ... 100%
(Reading database ... 202513 files and directories currently installed.)
Preparing to unpack .../libdbus-1-dev_1.14.10-4ubuntu4.1_amd64.deb ...
Unpacking libdbus-1-dev:amd64 (1.14.10-4ubuntu4.1) ...
Setting up libdbus-1-dev:amd64 (1.14.10-4ubuntu4.1) ...
Processing triggers for sgml-base (1.31) ...

Running kernel seems to be up-to-date.

No services need to be restarted.

No containers need to be restarted.

No user sessions are running outdated binaries.

No VM guests are running outdated hypervisor (qemu) binaries on this host.
[myinkbird] system dependencies installed
[myinkbird] checking formatting (cargo fmt --check)
Diff in /home/runner/work/myinkbird/myinkbird/crates/inkbird-collector/src/main.rs:76:
 
 /// Initialise tracing/logging. Honours `RUST_LOG`; defaults to `info`.
 fn init_tracing() {
-    let filter = EnvFilter::try_from_default_env()
-        .unwrap_or_else(|_| EnvFilter::new("info"));
+    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
     tracing_subscriber::fmt()
         .with_env_filter(filter)
         .with_target(false)
Diff in /home/runner/work/myinkbird/myinkbird/crates/inkbird-collector/src/ndjson_sink.rs:37:
 
 impl ReadingSink for NdjsonSink {
     fn record(&mut self, reading: &StoredReading) -> anyhow::Result<()> {
-        fs::create_dir_all(&self.readings_dir).with_context(|| {
-            format!("creating readings dir {}", self.readings_dir.display())
-        })?;
+        fs::create_dir_all(&self.readings_dir)
+            .with_context(|| format!("creating readings dir {}", self.readings_dir.display()))?;
 
         let path = self.file_for(reading.date_key());
         let mut line = serde_json::to_string(reading).context("serializing reading")?;
Diff in /home/runner/work/myinkbird/myinkbird/crates/inkbird-collector/src/scanner.rs:17:
 use std::fmt::Write as _;
 use std::time::{Duration, Instant};
 
-use anyhow::{anyhow, Context, Result};
+use anyhow::{Context, Result, anyhow};
 use btleplug::api::{Central as _, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
 use btleplug::platform::{Adapter, Manager, PeripheralId};
 use futures::StreamExt;
Diff in /home/runner/work/myinkbird/myinkbird/crates/inkbird-collector/src/scanner.rs:24:
-use time::format_description::well_known::Rfc3339;
 use time::OffsetDateTime;
+use time::format_description::well_known::Rfc3339;
 use tokio::time::sleep;
 use tracing::{debug, info, warn};
 
Diff in /home/runner/work/myinkbird/myinkbird/crates/inkbird-collector/src/scanner.rs:29:
-use inkbird_core::{build_message, parse_ith_13_b, SensorReading, ITH_13_B_MESSAGE_LEN};
+use inkbird_core::{ITH_13_B_MESSAGE_LEN, SensorReading, build_message, parse_ith_13_b};
 
 use crate::config::Config;
 use crate::record::StoredReading;
Diff in /home/runner/work/myinkbird/myinkbird/crates/inkbird-collector/src/scanner.rs:42:
 /// running or the D-Bus system socket is not reachable from inside the
 /// container) or if the host has no Bluetooth adapter.
 pub async fn get_central() -> Result<Adapter> {
-    let manager = Manager::new()
-        .await
-        .context("creating BLE manager (is bluetoothd running and the D-Bus system socket mounted?)")?;
+    let manager = Manager::new().await.context(
+        "creating BLE manager (is bluetoothd running and the D-Bus system socket mounted?)",
+    )?;
     let adapters = manager
         .adapters()
         .await
Diff in /home/runner/work/myinkbird/myinkbird/crates/inkbird-collector/src/scanner.rs:209:
         .start_scan(ScanFilter::default())
         .await
         .context("starting BLE scan")?;
-    info!(seconds, "discovering BLE devices; press Ctrl-C to stop early");
+    info!(
+        seconds,
+        "discovering BLE devices; press Ctrl-C to stop early"
+    );
 
     let mut seen: HashSet<String> = HashSet::new();
     let deadline = sleep(Duration::from_secs(seconds));
Diff in /home/runner/work/myinkbird/myinkbird/crates/inkbird-collector/src/shutdown.rs:8:
 /// Completes on the first shutdown signal (Ctrl-C or, on Unix, SIGTERM).
 #[cfg(unix)]
 pub async fn shutdown_signal() {
-    use tokio::signal::unix::{signal, SignalKind};
+    use tokio::signal::unix::{SignalKind, signal};
 
     let mut sigterm = match signal(SignalKind::terminate()) {
         Ok(s) => s,
Diff in /home/runner/work/myinkbird/myinkbird/crates/inkbird-core/src/lib.rs:48:
 
 pub use model::{ParseError, SensorReading};
 pub use parser::{
-    build_message, looks_like_ith_13_b, parse_ith_13_b, parse_manufacturer_entry,
     INKBIRD_SERVICE_UUID_16, ITH_13_B_LOCAL_NAME, ITH_13_B_MESSAGE_LEN, MIN_MESSAGE_LEN,
+    build_message, looks_like_ith_13_b, parse_ith_13_b, parse_manufacturer_entry,
 };
 
Diff in /home/runner/work/myinkbird/myinkbird/crates/inkbird-core/src/parser.rs:237:
         let le = 1000u16.to_le_bytes(); // exactly 100.0 %
         ok[8] = le[0];
         ok[9] = le[1];
-        assert!(approx(parse_ith_13_b(&ok).unwrap().humidity_pct.unwrap(), 100.0));
+        assert!(approx(
+            parse_ith_13_b(&ok).unwrap().humidity_pct.unwrap(),
+            100.0
+        ));
 
         let mut bad = good_message();
         let le = 1001u16.to_le_bytes(); // 100.1 %
Error: Process completed with exit code 1.


cargo-deny
failed 1 minute ago in 2m 14s 
Run ./scripts/deny.sh
[myinkbird] installing cargo-deny
    Updating crates.io index
 Downloading crates ...
  Downloaded cargo-deny v0.19.9
  Installing cargo-deny v0.19.9
warning: default toolchain implicitly overridden with `stable-x86_64-unknown-linux-gnu` by rustup toolchain file
  |
  = help: use `cargo +stable install` if you meant to use the stable toolchain
  = note: rustup selects the toolchain based on the parent environment and not the environment of the package being installed
    Updating crates.io index
    Updating crates.io index
 Downloading crates ...
  Downloaded zstd v0.13.3
  Downloaded termcolor v1.4.1
  Downloaded walkdir v2.5.0
  Downloaded typenum v1.20.0
  Downloaded zerovec-derive v0.11.3
  Downloaded zmij v1.0.21
  Downloaded yoke-derive v0.8.2
  Downloaded untrusted v0.9.0
  Downloaded utf8parse v0.2.2
  Downloaded globset v0.4.18
  Downloaded zerofrom v0.1.8
  Downloaded tinystr v0.8.3
  Downloaded unicode-ident v1.0.24
  Downloaded version_check v0.9.5
  Downloaded zerotrie v0.2.4
  Downloaded zstd-safe v7.2.4
  Downloaded thiserror v2.0.18
  Downloaded unicode-width v0.2.2
  Downloaded zerofrom-derive v0.1.7
  Downloaded zerovec v0.11.6
  Downloaded twox-hash v2.1.2
  Downloaded yoke v0.8.2
  Downloaded krates v0.21.2
  Downloaded indexmap v2.14.0
  Downloaded crossbeam-queue v0.3.12
  Downloaded serde v1.0.228
  Downloaded icu_normalizer v2.2.0
  Downloaded http v1.4.0
  Downloaded digest v0.10.7
  Downloaded regex v1.12.3
  Downloaded serde_derive v1.0.228
  Downloaded serde_json v1.0.150
  Downloaded same-file v1.0.6
  Downloaded syn v2.0.117
  Downloaded heck v0.5.0
  Downloaded parking_lot v0.12.5
  Downloaded percent-encoding v2.3.2
  Downloaded codespan-reporting v0.13.1
  Downloaded proc-macro2 v1.0.106
  Downloaded zstd-sys v2.0.16+zstd.1.5.7
  Downloaded scroll_derive v0.13.1
  Downloaded anyhow v1.0.102
  Downloaded nu-ansi-term v0.50.3
  Downloaded semver v1.0.28
  Downloaded scroll v0.13.0
  Downloaded serde_core v1.0.228
  Downloaded icu_provider v2.2.0
  Downloaded icu_normalizer_data v2.2.0
  Downloaded borsh v1.6.1
  Downloaded icu_locale_core v2.2.0
  Downloaded getrandom v0.2.17
  Downloaded camino v1.2.2
  Downloaded rayon v1.12.0
  Downloaded idna v1.1.0
  Downloaded clap_builder v4.6.0
  Downloaded hashbrown v0.15.5
  Downloaded clap_derive v4.6.1
  Downloaded icu_collections v2.2.0
  Downloaded either v1.16.0
  Downloaded block-buffer v0.10.4
  Downloaded regex-automata v0.4.14
  Downloaded fern v0.7.1
  Downloaded regex-syntax v0.8.10
  Downloaded scopeguard v1.2.0
  Downloaded fixedbitset v0.5.7
  Downloaded litemap v0.8.2
  Downloaded home v0.5.12
  Downloaded bstr v1.12.1
  Downloaded codespan v0.13.1
  Downloaded goblin v0.10.5
  Downloaded cfg-expr v0.20.7
  Downloaded bitvec v1.0.1
  Downloaded sha2 v0.10.9
  Downloaded potential_utf v0.1.5
  Downloaded parking_lot_core v0.9.12
  Downloaded jobserver v0.1.34
  Downloaded idna_adapter v1.2.2
  Downloaded libc v0.2.186
  Downloaded plain v0.2.3
  Downloaded is_terminal_polyfill v1.70.2
  Downloaded equivalent v1.0.2
  Downloaded memchr v2.8.0
  Downloaded itoa v1.0.18
  Downloaded icu_properties_data v2.2.0
  Downloaded rayon-core v1.13.0
  Downloaded generic-array v0.14.7
  Downloaded jiff v0.2.24
  Downloaded petgraph v0.8.1
  Downloaded displaydoc v0.2.5
  Downloaded anstyle v1.0.14
  Downloaded hashbrown v0.17.1
  Downloaded crossbeam v0.8.4
  Downloaded allocator-api2 v0.2.21
  Downloaded tinyvec v1.11.0
  Downloaded smallvec v1.15.1
  Downloaded funty v2.0.0
  Downloaded toml-span v0.7.1
  Downloaded log v0.4.29
  Downloaded pkg-config v0.3.33
  Downloaded crossbeam-channel v0.5.15
  Downloaded lock_api v0.4.14
  Downloaded cc v1.2.62
  Downloaded anstyle-parse v1.0.0
  Downloaded synstructure v0.13.2
  Downloaded icu_properties v2.2.0
  Downloaded foldhash v0.1.5
  Downloaded url v2.5.8
  Downloaded unicode-normalization v0.1.25
  Downloaded target-lexicon v0.13.3
  Downloaded radium v0.7.0
  Downloaded memmap2 v0.9.10
  Downloaded cfg-if v1.0.4
  Downloaded crypto-common v0.1.7
  Downloaded strum_macros v0.28.0
  Downloaded spdx v0.13.4
  Downloaded colorchoice v1.0.5
  Downloaded utf8_iter v1.0.4
  Downloaded cpufeatures v0.2.17
  Downloaded tame-index v0.26.3
  Downloaded ring v0.17.14
  Downloaded quote v1.0.45
  Downloaded cfg_aliases v0.2.1
  Downloaded wyz v0.5.1
  Downloaded strum v0.28.0
  Downloaded crossbeam-utils v0.8.21
  Downloaded form_urlencoded v1.2.2
  Downloaded find-msvc-tools v0.1.9
  Downloaded shlex v1.3.0
  Downloaded crossbeam-epoch v0.9.18
  Downloaded writeable v0.6.3
  Downloaded thiserror-impl v2.0.18
  Downloaded smol_str v0.3.2
  Downloaded clap_lex v1.1.0
  Downloaded anstyle-query v1.1.5
  Downloaded anstream v1.0.0
  Downloaded aho-corasick v1.1.4
  Downloaded tinyvec_macros v0.1.1
  Downloaded tap v1.0.1
  Downloaded strsim v0.11.1
  Downloaded bytes v1.11.1
  Downloaded clap v4.6.1
  Downloaded rustc-stable-hash v0.1.2
  Downloaded crossbeam-deque v0.8.6
  Downloaded stable_deref_trait v1.2.1
   Compiling proc-macro2 v1.0.106
   Compiling quote v1.0.45
   Compiling unicode-ident v1.0.24
   Compiling libc v0.2.186
   Compiling stable_deref_trait v1.2.1
   Compiling serde_core v1.0.228
   Compiling shlex v1.3.0
   Compiling smallvec v1.15.1
   Compiling find-msvc-tools v0.1.9
   Compiling crossbeam-utils v0.8.21
   Compiling memchr v2.8.0
   Compiling jobserver v0.1.34
   Compiling syn v2.0.117
   Compiling cc v1.2.62
   Compiling version_check v0.9.5
   Compiling serde v1.0.228
   Compiling generic-array v0.14.7
   Compiling pkg-config v0.3.33
   Compiling writeable v0.6.3
   Compiling litemap v0.8.2
   Compiling icu_properties_data v2.2.0
   Compiling zstd-sys v2.0.16+zstd.1.5.7
   Compiling icu_normalizer_data v2.2.0
   Compiling typenum v1.20.0
   Compiling cfg-if v1.0.4
   Compiling utf8_iter v1.0.4
   Compiling crossbeam-epoch v0.9.18
   Compiling zmij v1.0.21
   Compiling crossbeam-deque v0.8.6
   Compiling synstructure v0.13.2
   Compiling aho-corasick v1.1.4
   Compiling itoa v1.0.18
   Compiling utf8parse v0.2.2
   Compiling target-lexicon v0.13.3
   Compiling heck v0.5.0
   Compiling equivalent v1.0.2
   Compiling unicode-width v0.2.2
   Compiling camino v1.2.2
   Compiling zerofrom-derive v0.1.7
   Compiling yoke-derive v0.8.2
   Compiling zerofrom v0.1.8
   Compiling zerovec-derive v0.11.3
   Compiling yoke v0.8.2
   Compiling displaydoc v0.2.5
   Compiling serde_derive v1.0.228
   Compiling zerotrie v0.2.4
   Compiling zerovec v0.11.6
   Compiling zstd-safe v7.2.4
   Compiling regex-syntax v0.8.10
   Compiling tinystr v0.8.3
   Compiling icu_locale_core v2.2.0
   Compiling potential_utf v0.1.5
   Compiling icu_collections v2.2.0
   Compiling icu_provider v2.2.0
   Compiling serde_json v1.0.150
   Compiling termcolor v1.4.1
   Compiling rayon-core v1.13.0
   Compiling icu_normalizer v2.2.0
   Compiling codespan-reporting v0.13.1
   Compiling icu_properties v2.2.0
   Compiling regex-automata v0.4.14
   Compiling anstyle-parse v1.0.0
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.7
   Compiling hashbrown v0.17.1
   Compiling is_terminal_polyfill v1.70.2
   Compiling foldhash v0.1.5
   Compiling log v0.4.29
   Compiling radium v0.7.0
   Compiling anstyle v1.0.14
   Compiling colorchoice v1.0.5
   Compiling tinyvec_macros v0.1.1
   Compiling thiserror v2.0.18
   Compiling allocator-api2 v0.2.21
   Compiling anstyle-query v1.1.5
   Compiling parking_lot_core v0.9.12
   Compiling hashbrown v0.15.5
   Compiling anstream v1.0.0
   Compiling tinyvec v1.11.0
   Compiling indexmap v2.14.0
   Compiling idna_adapter v1.2.2
   Compiling digest v0.10.7
   Compiling scroll_derive v0.13.1
   Compiling thiserror-impl v2.0.18
   Compiling ring v0.17.14
   Compiling semver v1.0.28
   Compiling clap_lex v1.1.0
   Compiling bytes v1.11.1
   Compiling percent-encoding v2.3.2
   Compiling anyhow v1.0.102
   Compiling tap v1.0.1
   Compiling fixedbitset v0.5.7
   Compiling cpufeatures v0.2.17
   Compiling strsim v0.11.1
   Compiling either v1.16.0
   Compiling scopeguard v1.2.0
   Compiling lock_api v0.4.14
   Compiling rayon v1.12.0
   Compiling clap_builder v4.6.0
   Compiling sha2 v0.10.9
   Compiling http v1.4.0
   Compiling petgraph v0.8.1
   Compiling wyz v0.5.1
   Compiling form_urlencoded v1.2.2
   Compiling scroll v0.13.0
   Compiling cfg-expr v0.20.7
   Compiling idna v1.1.0
   Compiling unicode-normalization v0.1.25
   Compiling regex v1.12.3
   Compiling toml-span v0.7.1
   Compiling smol_str v0.3.2
   Compiling strum_macros v0.28.0
   Compiling clap_derive v4.6.1
   Compiling getrandom v0.2.17
   Compiling crossbeam-queue v0.3.12
   Compiling crossbeam-channel v0.5.15
   Compiling bstr v1.12.1
   Compiling funty v2.0.0
   Compiling twox-hash v2.1.2
   Compiling rustc-stable-hash v0.1.2
   Compiling plain v0.2.3
   Compiling untrusted v0.9.0
   Compiling same-file v1.0.6
   Compiling walkdir v2.5.0
   Compiling goblin v0.10.5
   Compiling bitvec v1.0.1
   Compiling tame-index v0.26.3
   Compiling clap v4.6.1
   Compiling globset v0.4.18
   Compiling crossbeam v0.8.4
   Compiling strum v0.28.0
   Compiling parking_lot v0.12.5
   Compiling url v2.5.8
   Compiling krates v0.21.2
   Compiling fern v0.7.1
   Compiling codespan v0.13.1
   Compiling memmap2 v0.9.10
   Compiling jiff v0.2.24
   Compiling nu-ansi-term v0.50.3
   Compiling home v0.5.12
   Compiling zstd v0.13.3
   Compiling spdx v0.13.4
   Compiling cargo-deny v0.19.9
    Finished `release` profile [optimized] target(s) in 2m 05s
  Installing /home/runner/.cargo/bin/cargo-deny
   Installed package `cargo-deny v0.19.9` (executable `cargo-deny`)
[myinkbird] running cargo-deny checks (advisories, bans, licenses, sources)
warning[license-not-encountered]: license was not encountered
   ┌─ /home/runner/work/myinkbird/myinkbird/deny.toml:29:6
   │
29 │     "0BSD",
   │      ━━━━ unmatched license allowance
warning[license-not-encountered]: license was not encountered
   ┌─ /home/runner/work/myinkbird/myinkbird/deny.toml:20:6
   │
20 │     "AGPL-3.0",
   │      ━━━━━━━━ unmatched license allowance
warning[license-not-encountered]: license was not encountered
   ┌─ /home/runner/work/myinkbird/myinkbird/deny.toml:25:6
   │
25 │     "BSD-2-Clause",
   │      ━━━━━━━━━━━━ unmatched license allowance
warning[license-not-encountered]: license was not encountered
   ┌─ /home/runner/work/myinkbird/myinkbird/deny.toml:31:6
   │
31 │     "CC0-1.0",
   │      ━━━━━━━ unmatched license allowance
warning[license-not-encountered]: license was not encountered
   ┌─ /home/runner/work/myinkbird/myinkbird/deny.toml:27:6
   │
27 │     "ISC",
   │      ━━━ unmatched license allowance
warning[license-not-encountered]: license was not encountered
   ┌─ /home/runner/work/myinkbird/myinkbird/deny.toml:30:6
   │
30 │     "MPL-2.0",
   │      ━━━━━━━ unmatched license allowance
warning[license-not-encountered]: license was not encountered
   ┌─ /home/runner/work/myinkbird/myinkbird/deny.toml:33:6
   │
33 │     "Unicode-DFS-2016",
   │      ━━━━━━━━━━━━━━━━ unmatched license allowance
warning[license-not-encountered]: license was not encountered
   ┌─ /home/runner/work/myinkbird/myinkbird/deny.toml:28:6
   │
28 │     "Zlib",
   │      ━━━━ unmatched license allowance
bug[unresolved-workspace-dependency]: failed to resolve a workspace dependency
   ┌─ /home/runner/work/myinkbird/myinkbird/crates/inkbird-collector/Cargo.toml:22:26
   │
22 │ inkbird-core.workspace = true
   │                          ━━━━
   │                          │
   │                          usage of workspace dependency
   │
   ├ inkbird-collector v0.1.0
warning[wildcard]: found 1 wildcard dependency for crate 'inkbird-collector'
   ┌─ /home/runner/work/myinkbird/myinkbird/crates/inkbird-collector/Cargo.toml:22:26
   │
22 │ inkbird-core.workspace = true
   │                          ━━━━ wildcard dependency
   │
   ├ inkbird-collector v0.1.0
warning[duplicate]: found 2 duplicate entries for crate 'thiserror'
   ┌─ /home/runner/work/myinkbird/myinkbird/Cargo.lock:96:1
   │  
96 │ ╭ thiserror 1.0.69 registry+https://github.com/rust-lang/crates.io-index
97 │ │ thiserror 2.0.18 registry+https://github.com/rust-lang/crates.io-index
   │ ╰──────────────────────────────────────────────────────────────────────┘ lock entries
   │  
   ├ thiserror v1.0.69
     └── jni v0.19.0
         └── btleplug v0.12.0
             └── inkbird-collector v0.1.0
   ├ thiserror v2.0.18
     ├── bluez-async v0.8.2
     │   └── btleplug v0.12.0
     │       └── inkbird-collector v0.1.0
     ├── btleplug v0.12.0 (*)
     ├── inkbird-core v0.1.0
     │   └── inkbird-collector v0.1.0 (*)
     └── serde-xml-rs v0.8.2
         └── bluez-async v0.8.2 (*)
warning[duplicate]: found 2 duplicate entries for crate 'thiserror-impl'
   ┌─ /home/runner/work/myinkbird/myinkbird/Cargo.lock:98:1
   │  
98 │ ╭ thiserror-impl 1.0.69 registry+https://github.com/rust-lang/crates.io-index
99 │ │ thiserror-impl 2.0.18 registry+https://github.com/rust-lang/crates.io-index
   │ ╰───────────────────────────────────────────────────────────────────────────┘ lock entries
   │  
   ├ thiserror-impl v1.0.69
     └── thiserror v1.0.69
         └── jni v0.19.0
             └── btleplug v0.12.0
                 └── inkbird-collector v0.1.0
   ├ thiserror-impl v2.0.18
     └── thiserror v2.0.18
         ├── bluez-async v0.8.2
         │   └── btleplug v0.12.0
         │       └── inkbird-collector v0.1.0
         ├── btleplug v0.12.0 (*)
         ├── inkbird-core v0.1.0
         │   └── inkbird-collector v0.1.0 (*)
         └── serde-xml-rs v0.8.2
             └── bluez-async v0.8.2 (*)
error[vulnerability]: Denial of Service via Stack Exhaustion
    ┌─ /home/runner/work/myinkbird/myinkbird/Cargo.lock:101:1
    │
101 │ time 0.3.45 registry+https://github.com/rust-lang/crates.io-index
    │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ security vulnerability detected
    │
    ├ ID: RUSTSEC-2026-0009
    ├ Advisory: https://rustsec.org/advisories/RUSTSEC-2026-0009
    ├ ## Impact
      
      When user-provided input is provided to any type that parses with the RFC 2822 format, a denial of
      service attack via stack exhaustion is possible. The attack relies on formally deprecated and
      rarely-used features that are part of the RFC 2822 format used in a malicious manner. Ordinary,
      non-malicious input will never encounter this scenario.
      
      ## Patches
      
      A limit to the depth of recursion was added in v0.3.47. From this version, an error will be returned
      rather than exhausting the stack.
      
      ## Workarounds
      
      Limiting the length of user input is the simplest way to avoid stack exhaustion, as the amount of
      the stack consumed would be at most a factor of the length of the input.
    ├ Announcement: https://github.com/time-rs/time/blob/main/CHANGELOG.md#0347-2026-02-05
    ├ Solution: Upgrade to >=0.3.47 (try `cargo update -p time`)
    ├ time v0.3.45
      └── inkbird-collector v0.1.0
