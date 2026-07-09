kushal@fedora:~/src/golang/GoTunnels$ cd ~/src/mypython/myinkbird/; time bash export.sh > docs/llm/vendor/output.txt

real	0m0.103s
user	0m0.041s
sys	0m0.054s
kushal@fedora:~/src/mypython/myinkbird$ cd ~/src/mypython/myinkbird/; time bash export.sh > docs/llm/vendor/output.txt

real	0m0.131s
user	0m0.035s
sys	0m0.058s
kushal@fedora:~/src/mypython/myinkbird$ cd ~/src/mypython/myinkbird/; time bash export.sh > docs/llm/vendor/output.txt
export.sh: wrote 48 files to docs/llm/dump.txt

real	0m0.096s
user	0m0.040s
sys	0m0.050s
kushal@fedora:~/src/mypython/myinkbird$ cd ~/src/mypython/myinkbird/; time bash export.sh > docs/llm/vendor/output.txt

real	0m1.559s
user	0m0.757s
sys	0m0.926s
kushal@fedora:~/src/mypython/myinkbird$ cd ~/src/mypython/myinkbird/; time bash export.sh > docs/llm/vendor/output.txt

real	0m1.604s
user	0m0.758s
sys	0m1.058s
kushal@fedora:~/src/mypython/myinkbird$ cd ~/src/mypython/myinkbird/; time bash export.sh > docs/llm/vendor/output.txt

real	0m1.608s
user	0m0.791s
sys	0m1.008s
kushal@fedora:~/src/mypython/myinkbird$ time rustfmt



^C
real	6m38.924s
user	0m0.014s
sys	0m0.055s

kushal@fedora:~/src/mypython/myinkbird$ 
kushal@fedora:~/src/mypython/myinkbird$ cd ~/src/mypython/myinkbird/; time bash export.sh > docs/llm/vendor/output.txt

real	0m1.757s
user	0m0.782s
sys	0m1.136s
kushal@fedora:~/src/mypython/myinkbird$ cd ~/src/rust/myinkbird/; time bash export.sh > docs/llm/vendor/output.txt

real	0m1.670s
user	0m0.763s
sys	0m1.073s
kushal@fedora:~/src/rust/myinkbird$ cd ~/src/rust/myinkbird/; time bash export.sh > docs/llm/vendor/output.txt

real	0m1.768s
user	0m0.769s
sys	0m1.064s
kushal@fedora:~/src/rust/myinkbird$ cd ~/src/rust/myinkbird/; time bash export.sh > docs/llm/vendor/output.txt

real	0m1.676s
user	0m0.755s
sys	0m1.058s
kushal@fedora:~/src/rust/myinkbird$ cd ~/src/rust/myinkbird/; time bash export.sh > docs/llm/vendor/output.txt; time systemctl status bluetooth; time bash scripts/container-build.sh; time bash scripts/run.sh;

real	0m1.669s
user	0m0.771s
sys	0m1.046s
● bluetooth.service - Bluetooth service
     Loaded: loaded (/usr/lib/systemd/system/bluetooth.service; enabled; preset: enabled)
    Drop-In: /usr/lib/systemd/system/service.d
             └─10-timeout-abort.conf
     Active: active (running) since Mon 2026-07-06 03:48:22 EDT; 3 days ago
 Invocation: 071b37e9f5244feabadc219b998f1c06
       Docs: man:bluetoothd(8)
   Main PID: 1030 (bluetoothd)
     Status: "Running"
      Tasks: 1 (limit: 18226)
     Memory: 652K (peak: 4.5M, swap: 432K, swap peak: 432K)
        CPU: 260ms
     CGroup: /system.slice/bluetooth.service
             └─1030 /usr/libexec/bluetooth/bluetoothd

Jul 06 03:49:06 fedora bluetoothd[1030]: Endpoint registered: sender=:1.98 path=/MediaEndpoint/A2DPSink/opus_05_duplex
Jul 06 03:49:06 fedora bluetoothd[1030]: Endpoint registered: sender=:1.98 path=/MediaEndpoint/A2DPSource/opus_05_duplex
Jul 06 08:46:06 fedora bluetoothd[1030]: /org/bluez/hci0/dev_41_42_3C_F8_C4_8A/fd0: fd(41) ready
Jul 06 08:56:50 fedora bluetoothd[1030]: src/profile.c:ext_io_disconnected() Unable to get io data for Hands-Free Voice gateway: getpeername: Transport endpoint is not connected (107)
Jul 07 08:58:57 fedora bluetoothd[1030]: Controller resume with wake event 0x0
Jul 07 19:35:18 fedora bluetoothd[1030]: /org/bluez/hci0/dev_41_42_3C_F8_C4_8A/fd0: fd(41) ready
Jul 07 22:52:33 fedora bluetoothd[1030]: src/profile.c:ext_io_disconnected() Unable to get io data for Hands-Free Voice gateway: getpeername: Transport endpoint is not connected (107)
Jul 08 07:34:45 fedora bluetoothd[1030]: /org/bluez/hci0/dev_41_42_3C_F8_C4_8A/fd0: fd(40) ready
Jul 08 08:41:13 fedora bluetoothd[1030]: src/profile.c:ext_io_disconnected() Unable to get io data for Hands-Free Voice gateway: getpeername: Transport endpoint is not connected (107)
Jul 08 16:12:31 fedora bluetoothd[1030]: Controller resume with wake event 0x0

real	0m0.878s
user	0m0.007s
sys	0m0.263s
[myinkbird] building collector image (localhost/myinkbird-collector:latest)
[1/2] STEP 1/7: FROM registry.fedoraproject.org/fedora:latest AS build
Trying to pull registry.fedoraproject.org/fedora:latest...
Getting image source signatures
Copying blob 40a17f3afeec done   | 
Copying config 78649a001f done   | 
Writing manifest to image destination
[1/2] STEP 2/7: RUN dnf -y install gcc pkgconf-pkg-config dbus-devel rustup     && rustup-init -y --profile minimal --default-toolchain stable --no-modify-path     && dnf clean all
Updating and loading repositories:
 Fedora 44 openh264 (From Cisco) - x86_ 100% |   4.3 KiB/s |   5.3 KiB |  00m01s
 Fedora 44 - x86_64 - Updates           100% |   6.6 MiB/s |  10.1 MiB |  00m02s
 Fedora 44 - x86_64                     100% |   7.5 MiB/s |  36.4 MiB |  00m05s
Repositories loaded.
Total size of inbound packages is 71 MiB. Need to download 71 MiB.
After this operation, 212 MiB extra will be used (install 212 MiB, remove 0 B).
Package                     Arch   Version          Repository      Size
Installing:
 dbus-devel                 x86_64 1:1.16.2-1.fc44  fedora     131.7 KiB
 gcc                        x86_64 0:16.1.1-2.fc44  updates    120.8 MiB
 pkgconf-pkg-config         x86_64 0:2.5.1-1.fc44   fedora     989.0   B
 rustup                     x86_64 0:1.29.0-4.fc44  updates      9.4 MiB
Installing dependencies:
 binutils                   x86_64 0:2.46-3.fc44    updates     27.2 MiB
 cmake-filesystem           x86_64 0:4.3.0-1.fc44   fedora       0.0   B
 cpp                        x86_64 0:16.1.1-2.fc44  updates     42.1 MiB
 dbus-libs                  x86_64 1:1.16.2-1.fc44  fedora     357.4 KiB
 elfutils-debuginfod-client x86_64 0:0.195-1.fc44   updates     83.8 KiB
 glibc-devel                x86_64 0:2.43-7.fc44    updates      2.3 MiB
 jansson                    x86_64 0:2.14-4.fc44    fedora      88.9 KiB
 kernel-headers             x86_64 0:7.1.3-200.fc44 updates      6.9 MiB
 libatomic                  x86_64 0:16.1.1-2.fc44  updates     45.6 KiB
 libmpc                     x86_64 0:1.4.1-1.fc44   updates    168.7 KiB
 libpkgconf                 x86_64 0:2.5.1-1.fc44   fedora      90.1 KiB
 libxcrypt-devel            x86_64 0:4.5.2-3.fc44   fedora      31.0 KiB
 make                       x86_64 1:4.4.1-12.fc44  fedora       1.8 MiB
 pkgconf                    x86_64 0:2.5.1-1.fc44   fedora      92.7 KiB
 pkgconf-m4                 noarch 0:2.5.1-1.fc44   fedora      14.3 KiB
 systemd-devel              x86_64 0:259.7-1.fc44   updates    601.0 KiB
 xml-common                 noarch 0:0.6.3-68.fc44  fedora      78.4 KiB

Transaction Summary:
 Installing:        21 packages

[ 1/21] make-1:4.4.1-12.fc44.x86_64     100% | 674.3 KiB/s | 588.7 KiB |  00m01s
[ 2/21] pkgconf-pkg-config-0:2.5.1-1.fc 100% |  58.1 KiB/s |   9.5 KiB |  00m00s
[ 3/21] cpp-0:16.1.1-2.fc44.x86_64      100% |  12.8 MiB/s |  14.5 MiB |  00m01s
[ 4/21] pkgconf-0:2.5.1-1.fc44.x86_64   100% | 371.7 KiB/s |  48.7 KiB |  00m00s
[ 5/21] libpkgconf-0:2.5.1-1.fc44.x86_6 100% | 514.2 KiB/s |  42.7 KiB |  00m00s
[ 6/21] pkgconf-m4-0:2.5.1-1.fc44.noarc 100% |  86.0 KiB/s |  13.8 KiB |  00m00s
[ 7/21] dbus-devel-1:1.16.2-1.fc44.x86_ 100% | 440.9 KiB/s |  38.8 KiB |  00m00s
[ 8/21] cmake-filesystem-0:4.3.0-1.fc44 100% | 194.7 KiB/s |  15.0 KiB |  00m00s
[ 9/21] dbus-libs-1:1.16.2-1.fc44.x86_6 100% |   1.7 MiB/s | 155.1 KiB |  00m00s
[10/21] xml-common-0:0.6.3-68.fc44.noar 100% | 189.8 KiB/s |  31.1 KiB |  00m00s
[11/21] rustup-0:1.29.0-4.fc44.x86_64   100% |  11.9 MiB/s |   3.4 MiB |  00m00s
[12/21] gcc-0:16.1.1-2.fc44.x86_64      100% |  20.1 MiB/s |  43.1 MiB |  00m02s
[13/21] binutils-0:2.46-3.fc44.x86_64   100% |  14.0 MiB/s |   6.1 MiB |  00m00s
[14/21] glibc-devel-0:2.43-7.fc44.x86_6 100% |   7.9 MiB/s | 617.2 KiB |  00m00s
[15/21] systemd-devel-0:259.7-1.fc44.x8 100% | 915.0 KiB/s | 688.1 KiB |  00m01s
[16/21] jansson-0:2.14-4.fc44.x86_64    100% | 305.9 KiB/s |  47.1 KiB |  00m00s
[17/21] libatomic-0:16.1.1-2.fc44.x86_6 100% |   1.3 MiB/s |  50.2 KiB |  00m00s
[18/21] libmpc-0:1.4.1-1.fc44.x86_64    100% |   2.2 MiB/s |  75.5 KiB |  00m00s
[19/21] elfutils-debuginfod-client-0:0. 100% |   1.2 MiB/s |  46.2 KiB |  00m00s
[20/21] libxcrypt-devel-0:4.5.2-3.fc44. 100% | 191.2 KiB/s |  30.0 KiB |  00m00s
[21/21] kernel-headers-0:7.1.3-200.fc44 100% |  15.3 MiB/s |   1.7 MiB |  00m00s
--------------------------------------------------------------------------------
[21/21] Total                           100% |  29.1 MiB/s |  71.2 MiB |  00m02s
Running transaction
[ 1/23] Verify package files            100% | 122.0   B/s |  21.0   B |  00m00s
[ 2/23] Prepare transaction             100% | 381.0   B/s |  21.0   B |  00m00s
[ 3/23] Installing libmpc-0:1.4.1-1.fc4 100% |  55.5 MiB/s | 170.4 KiB |  00m00s
[ 4/23] Installing cpp-0:16.1.1-2.fc44. 100% | 353.6 MiB/s |  42.1 MiB |  00m00s
[ 5/23] Installing kernel-headers-0:7.1 100% |  64.5 MiB/s |   7.1 MiB |  00m00s
[ 6/23] Installing elfutils-debuginfod- 100% |   5.6 MiB/s |  86.1 KiB |  00m00s
[ 7/23] Installing libatomic-0:16.1.1-2 100% |  45.6 MiB/s |  46.7 KiB |  00m00s
[ 8/23] Installing jansson-0:2.14-4.fc4 100% |  44.1 MiB/s |  90.3 KiB |  00m00s
[ 9/23] Installing binutils-0:2.46-3.fc 100% | 349.5 MiB/s |  27.3 MiB |  00m00s
[10/23] Installing systemd-devel-0:259. 100% |  42.4 MiB/s | 737.7 KiB |  00m00s
[11/23] Installing xml-common-0:0.6.3-6 100% |  39.6 MiB/s |  81.1 KiB |  00m00s
[12/23] Installing dbus-libs-1:1.16.2-1 100% | 175.1 MiB/s | 358.5 KiB |  00m00s
[13/23] Installing cmake-filesystem-0:4 100% |   1.8 MiB/s |   9.4 KiB |  00m00s
[14/23] Installing libpkgconf-0:2.5.1-1 100% |  89.1 MiB/s |  91.3 KiB |  00m00s
[15/23] Installing pkgconf-0:2.5.1-1.fc 100% |   8.5 MiB/s |  95.2 KiB |  00m00s
[16/23] Installing pkgconf-m4-0:2.5.1-1 100% |  14.4 MiB/s |  14.7 KiB |  00m00s
[17/23] Installing pkgconf-pkg-config-0 100% | 197.0 KiB/s |   1.8 KiB |  00m00s
[18/23] Installing libxcrypt-devel-0:4. 100% |  16.3 MiB/s |  33.3 KiB |  00m00s
[19/23] Installing glibc-devel-0:2.43-7 100% |  53.0 MiB/s |   2.4 MiB |  00m00s
[20/23] Installing make-1:4.4.1-12.fc44 100% | 120.0 MiB/s |   1.8 MiB |  00m00s
[21/23] Installing gcc-0:16.1.1-2.fc44. 100% | 395.0 MiB/s | 120.9 MiB |  00m00s
[22/23] Installing dbus-devel-1:1.16.2- 100% |  33.3 MiB/s | 136.5 KiB |  00m00s
[23/23] Installing rustup-0:1.29.0-4.fc 100% |  97.8 MiB/s |   9.4 MiB |  00m00s
Complete!
warn: It looks like you have an existing rustup settings file at:
warn: /root/.rustup/settings.toml
warn: Rustup will install the default toolchain as specified in the settings file,
warn: instead of the one inferred from the default host triple.
info: profile set to minimal
info: default host triple is x86_64-unknown-linux-gnu
info: syncing channel updates for stable-x86_64-unknown-linux-gnu
info: latest update on 2026-06-30 for version 1.96.1 (31fca3adb 2026-06-26)
info: downloading 3 components

info: default toolchain set to stable-x86_64-unknown-linux-gnu
  stable-x86_64-unknown-linux-gnu installed - rustc 1.96.1 (31fca3adb 2026-06-26)


Rust is installed now. Great!

To get started you need Cargo's bin directory ($HOME/.cargo/bin) in your PATH
environment variable. This has not been done automatically.

To configure your current shell, you need to source
the corresponding env file under $HOME/.cargo.

This is usually done by running one of the following (note the leading DOT):
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
source "~/.cargo/env.nu"  # For nushell
source "$HOME/.cargo/env.tcsh"  # For tcsh
. "$HOME/.cargo/env.ps1"        # For pwsh
source "$HOME/.cargo/env.xsh"   # For xonsh
Removed 18 files, 11 directories (total of 97 MiB). 0 errors occurred.
--> 2cf0c0d4fa51
[1/2] STEP 3/7: ENV PATH="/root/.cargo/bin:${PATH}"
--> f45acc341346
[1/2] STEP 4/7: WORKDIR /src
--> 74741fe66b1c
[1/2] STEP 5/7: COPY Cargo.toml ./
--> b85927c1391c
[1/2] STEP 6/7: COPY crates ./crates
--> 0e589d74652d
[1/2] STEP 7/7: RUN cargo build --release -p inkbird-collector
    Updating crates.io index
     Locking 139 packages to latest Rust 1.85 compatible versions
      Adding time v0.3.45 (available: v0.3.53, requires Rust 1.88.0)
      Adding time-core v0.1.7 (available: v0.1.9, requires Rust 1.88.0)
      Adding time-macros v0.2.25 (available: v0.2.31, requires Rust 1.88.0)
 Downloading crates ...
  Downloaded bluez-async v0.8.2
  Downloaded futures-sink v0.3.32
  Downloaded clap v4.6.1
  Downloaded powerfmt v0.2.0
  Downloaded clap_lex v1.1.0
  Downloaded anstyle-query v1.1.5
  Downloaded futures-task v0.3.32
  Downloaded cfg-if v1.0.4
  Downloaded colorchoice v1.0.5
  Downloaded tokio-macros v2.7.0
  Downloaded heck v0.5.0
  Downloaded bluez-generated v0.4.0
  Downloaded lazy_static v1.5.0
  Downloaded strsim v0.11.1
  Downloaded futures-macro v0.3.32
  Downloaded errno v0.3.14
  Downloaded utf8parse v0.2.2
  Downloaded futures-io v0.3.32
  Downloaded num-conv v0.1.0
  Downloaded futures-core v0.3.32
  Downloaded time-core v0.1.7
  Downloaded matchers v0.2.0
  Downloaded quote v1.0.46
  Downloaded smallvec v1.15.2
  Downloaded anstyle-parse v1.0.0
  Downloaded slab v0.4.12
  Downloaded scopeguard v1.2.0
  Downloaded dbus-tokio v0.7.6
  Downloaded anstyle v1.0.14
  Downloaded anstream v1.0.0
  Downloaded thread_local v1.1.9
  Downloaded zmij v1.0.21
  Downloaded signal-hook-registry v1.4.8
  Downloaded thiserror-impl v2.0.18
  Downloaded lock_api v0.4.14
  Downloaded dashmap v6.2.1
  Downloaded thiserror v2.0.18
  Downloaded pkg-config v0.3.33
  Downloaded once_cell v1.21.4
  Downloaded static_assertions v1.1.0
  Downloaded nu-ansi-term v0.50.3
  Downloaded pin-project-lite v0.2.17
  Downloaded tracing-log v0.2.0
  Downloaded clap_derive v4.6.1
  Downloaded futures-channel v0.3.32
  Downloaded async-trait v0.1.89
  Downloaded tracing-attributes v0.1.31
  Downloaded tokio-stream v0.1.18
  Downloaded time-macros v0.2.25
  Downloaded sharded-slab v0.1.7
  Downloaded anyhow v1.0.103
  Downloaded uuid v1.23.4
  Downloaded xml v1.3.0
  Downloaded unicode-ident v1.0.24
  Downloaded tracing-core v0.1.36
  Downloaded bytes v1.12.1
  Downloaded socket2 v0.6.4
  Downloaded log v0.4.33
  Downloaded bitflags v2.13.0
  Downloaded serde_derive v1.0.228
  Downloaded serde v1.0.228
  Downloaded proc-macro2 v1.0.106
  Downloaded futures v0.3.32
  Downloaded memchr v2.8.3
  Downloaded serde_core v1.0.228
  Downloaded serde-xml-rs v0.8.2
  Downloaded mio v1.2.1
  Downloaded dbus v0.9.12
  Downloaded hashbrown v0.14.5
  Downloaded parking_lot_core v0.9.12
  Downloaded serde_json v1.0.150
  Downloaded tokio-util v0.7.18
  Downloaded clap_builder v4.6.0
  Downloaded tracing-subscriber v0.3.23
  Downloaded time v0.3.45
  Downloaded btleplug v0.12.0
  Downloaded futures-util v0.3.32
  Downloaded aho-corasick v1.1.4
  Downloaded itertools v0.14.0
  Downloaded syn v2.0.118
  Downloaded crossbeam-utils v0.8.22
  Downloaded is_terminal_polyfill v1.70.2
  Downloaded either v1.16.0
  Downloaded regex-syntax v0.8.11
  Downloaded deranged v0.5.8
  Downloaded itoa v1.0.18
  Downloaded futures-executor v0.3.32
  Downloaded tracing v0.1.44
  Downloaded regex-automata v0.4.14
  Downloaded libc v0.2.186
  Downloaded tokio v1.52.3
  Downloaded libdbus-sys v0.2.7
   Compiling proc-macro2 v1.0.106
   Compiling unicode-ident v1.0.24
   Compiling quote v1.0.46
   Compiling libc v0.2.186
   Compiling pin-project-lite v0.2.17
   Compiling futures-sink v0.3.32
   Compiling futures-core v0.3.32
   Compiling memchr v2.8.3
   Compiling futures-io v0.3.32
   Compiling pkg-config v0.3.33
   Compiling futures-task v0.3.32
   Compiling slab v0.4.12
   Compiling serde_core v1.0.228
   Compiling once_cell v1.21.4
   Compiling thiserror v2.0.18
   Compiling cfg-if v1.0.4
   Compiling serde v1.0.228
   Compiling log v0.4.33
   Compiling smallvec v1.15.2
   Compiling parking_lot_core v0.9.12
   Compiling futures-channel v0.3.32
   Compiling utf8parse v0.2.2
   Compiling crossbeam-utils v0.8.22
   Compiling anstyle-parse v1.0.0
   Compiling tracing-core v0.1.36
   Compiling regex-syntax v0.8.11
   Compiling is_terminal_polyfill v1.70.2
   Compiling colorchoice v1.0.5
   Compiling bytes v1.12.1
   Compiling anstyle v1.0.14
   Compiling xml v1.3.0
   Compiling scopeguard v1.2.0
   Compiling zmij v1.0.21
   Compiling anstyle-query v1.1.5
   Compiling either v1.16.0
   Compiling lock_api v0.4.14
   Compiling hashbrown v0.14.5
   Compiling serde_json v1.0.150
   Compiling anstream v1.0.0
   Compiling libdbus-sys v0.2.7
   Compiling itertools v0.14.0
   Compiling time-core v0.1.7
   Compiling lazy_static v1.5.0
   Compiling num-conv v0.1.0
   Compiling uuid v1.23.4
   Compiling bitflags v2.13.0
   Compiling anyhow v1.0.103
   Compiling heck v0.5.0
   Compiling clap_lex v1.1.0
   Compiling powerfmt v0.2.0
   Compiling strsim v0.11.1
   Compiling itoa v1.0.18
   Compiling syn v2.0.118
   Compiling time-macros v0.2.25
   Compiling tracing-log v0.2.0
   Compiling sharded-slab v0.1.7
   Compiling thread_local v1.1.9
   Compiling deranged v0.5.8
   Compiling static_assertions v1.1.0
   Compiling clap_builder v4.6.0
   Compiling nu-ansi-term v0.50.3
   Compiling errno v0.3.14
   Compiling socket2 v0.6.4
   Compiling mio v1.2.1
   Compiling signal-hook-registry v1.4.8
   Compiling dashmap v6.2.1
   Compiling regex-automata v0.4.14
   Compiling time v0.3.45
   Compiling matchers v0.2.0
   Compiling futures-macro v0.3.32
   Compiling tokio-macros v2.7.0
   Compiling thiserror-impl v2.0.18
   Compiling serde_derive v1.0.228
   Compiling tracing-attributes v0.1.31
   Compiling clap_derive v4.6.1
   Compiling async-trait v0.1.89
   Compiling tokio v1.52.3
   Compiling futures-util v0.3.32
   Compiling tracing v0.1.44
   Compiling inkbird-core v0.1.0 (/src/crates/inkbird-core)
   Compiling tracing-subscriber v0.3.23
   Compiling clap v4.6.1
   Compiling serde-xml-rs v0.8.2
   Compiling dbus v0.9.12
   Compiling futures-executor v0.3.32
   Compiling futures v0.3.32
   Compiling tokio-util v0.7.18
   Compiling tokio-stream v0.1.18
   Compiling bluez-generated v0.4.0
   Compiling dbus-tokio v0.7.6
   Compiling bluez-async v0.8.2
   Compiling btleplug v0.12.0
   Compiling inkbird-collector v0.1.0 (/src/crates/inkbird-collector)
    Finished `release` profile [optimized] target(s) in 18.88s
--> d6bb075e2199
[2/2] STEP 1/5: FROM registry.fedoraproject.org/fedora-minimal:latest AS runtime
Trying to pull registry.fedoraproject.org/fedora-minimal:latest...
Getting image source signatures
Copying blob 88fa1a5c282b done   | 
Copying config 3cb18f500d done   | 
Writing manifest to image destination
[2/2] STEP 2/5: RUN microdnf install -y dbus-libs && microdnf clean all
Updating and loading repositories:
 Fedora 44 openh264 (From Cisco) - x86_ 100% |   5.1 KiB/s |   5.3 KiB |  00m01s
 Fedora 44 - x86_64 - Updates           100% |   3.0 MiB/s |  11.4 MiB |  00m04s
 Fedora 44 - x86_64                     100% |   7.5 MiB/s |  36.4 MiB |  00m05s
Repositories loaded.
Total size of inbound packages is 155 KiB. Need to download 155 KiB.
After this operation, 357 KiB extra will be used (install 357 KiB, remove 0 B).
Package    Arch   Version         Repository      Size
Installing:
 dbus-libs x86_64 1:1.16.2-1.fc44 fedora     357.4 KiB

Transaction Summary:
 Installing:         1 package

[1/1] dbus-libs-1:1.16.2-1.fc44.x86_64  100% |   1.1 MiB/s | 155.1 KiB |  00m00s
--------------------------------------------------------------------------------
[1/1] Total                             100% |   1.1 MiB/s | 155.1 KiB |  00m00s
Running transaction
[1/3] Verify package files              100% |   0.0   B/s |   1.0   B |  00m00s
[2/3] Prepare transaction               100% |  76.0   B/s |   1.0   B |  00m00s
[3/3] Installing dbus-libs-1:1.16.2-1.f 100% |  23.3 MiB/s | 358.5 KiB |  00m00s
Complete!
Removed 18 files, 10 directories (total of 97 MiB). 0 errors occurred.
--> e6e1b9cbd58b
[2/2] STEP 3/5: COPY --from=build /src/target/release/inkbird-collector /usr/local/bin/inkbird-collector
--> 7f92595cd29a
[2/2] STEP 4/5: ENTRYPOINT ["/usr/local/bin/inkbird-collector"]
--> 1d0db46757b1
[2/2] STEP 5/5: CMD ["collect"]
[2/2] COMMIT localhost/myinkbird-collector:latest
--> 48bde1c58230
Successfully tagged localhost/myinkbird-collector:latest
48bde1c5823054aaf1fd27c05a60fde0e69c1db42aca0972897c07e67bee9e1e
[myinkbird] building committer image (localhost/myinkbird-committer:latest)
STEP 1/5: FROM registry.fedoraproject.org/fedora-minimal:latest
STEP 2/5: RUN microdnf install -y git && microdnf clean all
Updating and loading repositories:
 Fedora 44 openh264 (From Cisco) - x86_ 100% |   5.0 KiB/s |   5.3 KiB |  00m01s
 Fedora 44 - x86_64 - Updates           100% |   7.1 MiB/s |  10.1 MiB |  00m01s
 Fedora 44 - x86_64                     100% |  12.8 MiB/s |  36.4 MiB |  00m03s
Repositories loaded.
Package                     Arch   Version                   Repository      Size
Installing:
 git                        x86_64 0:2.55.0-1.fc44           updates     57.7 KiB
Installing dependencies:
 expat                      x86_64 0:2.8.1-1.fc44            updates    327.4 KiB
 git-core                   x86_64 0:2.55.0-1.fc44           updates     25.6 MiB
 git-core-doc               noarch 0:2.55.0-1.fc44           updates     18.9 MiB
 groff-base                 x86_64 0:1.23.0-12.fc44          fedora       3.9 MiB
 less                       x86_64 0:702-1.fc44              updates    477.5 KiB
 libedit                    x86_64 0:3.1-59.20260512cvs.fc44 updates    248.3 KiB
 libfdisk                   x86_64 0:2.41.5-1.fc44           updates    388.3 KiB
 liblastlog2                x86_64 0:2.41.5-1.fc44           updates     41.6 KiB
 mpdecimal                  x86_64 0:4.0.1-3.fc44            fedora     217.1 KiB
 ncurses                    x86_64 0:6.6-1.fc44              fedora     611.4 KiB
 oniguruma                  x86_64 0:6.9.10-4.fc44           fedora     770.9 KiB
 openssh                    x86_64 0:10.2p1-12.fc44          updates      1.4 MiB
 openssh-clients            x86_64 0:10.2p1-12.fc44          updates      2.7 MiB
 perl-AutoLoader            noarch 0:5.74-524.fc44           updates     20.6 KiB
 perl-B                     x86_64 0:1.89-524.fc44           updates    501.2 KiB
 perl-Carp                  noarch 0:1.54-521.fc44           fedora      46.6 KiB
 perl-Class-Struct          noarch 0:0.68-524.fc44           updates     25.4 KiB
 perl-Data-Dumper           x86_64 0:2.192-523.fc44          updates    115.5 KiB
 perl-Digest                noarch 0:1.20-521.fc44           fedora      35.3 KiB
 perl-Digest-MD5            x86_64 0:2.59-521.fc44           fedora      59.6 KiB
 perl-DynaLoader            x86_64 0:1.57-524.fc44           updates     32.1 KiB
 perl-Encode                x86_64 4:3.21-521.fc44           fedora       4.7 MiB
 perl-Errno                 x86_64 0:1.38-524.fc44           updates      8.4 KiB
 perl-Error                 noarch 1:0.17030-3.fc44          fedora      76.8 KiB
 perl-Exporter              noarch 0:5.79-521.fc44           fedora      54.3 KiB
 perl-Fcntl                 x86_64 0:1.20-524.fc44           updates     48.7 KiB
 perl-File-Basename         noarch 0:2.86-524.fc44           updates     14.0 KiB
 perl-File-Path             noarch 0:2.18-522.fc44           fedora      63.5 KiB
 perl-File-Temp             noarch 1:0.231.200-2.fc44        fedora     163.7 KiB
 perl-File-stat             noarch 0:1.14-524.fc44           updates     12.5 KiB
 perl-FileHandle            noarch 0:2.05-524.fc44           updates      9.4 KiB
 perl-Getopt-Long           noarch 1:2.58-521.fc44           fedora     144.5 KiB
 perl-Getopt-Std            noarch 0:1.14-524.fc44           updates     11.2 KiB
 perl-Git                   noarch 0:2.55.0-1.fc44           updates     64.4 KiB
 perl-HTTP-Tiny             noarch 0:0.094-1.fc44            updates    158.2 KiB
 perl-IO                    x86_64 0:1.55-524.fc44           updates    147.3 KiB
 perl-IO-Socket-IP          noarch 0:0.43-522.fc44           fedora     100.3 KiB
 perl-IO-Socket-SSL         noarch 0:2.098-2.fc44            fedora     723.5 KiB
 perl-IPC-Open3             noarch 0:1.24-524.fc44           updates     27.7 KiB
 perl-MIME-Base32           noarch 0:1.303-25.fc44           fedora      30.7 KiB
 perl-MIME-Base64           x86_64 0:3.16-521.fc44           fedora      41.9 KiB
 perl-Net-SSLeay            x86_64 0:1.94-12.fc44            fedora       1.3 MiB
 perl-POSIX                 x86_64 0:2.23-524.fc44           updates    229.4 KiB
 perl-PathTools             x86_64 0:3.94-521.fc44           fedora     179.9 KiB
 perl-Pod-Escapes           noarch 1:1.07-521.fc44           fedora      24.9 KiB
 perl-Pod-Perldoc           noarch 0:3.28.01-522.fc44        fedora     163.7 KiB
 perl-Pod-Simple            noarch 1:3.47-4.fc44             fedora     565.3 KiB
 perl-Pod-Usage             noarch 4:2.05-521.fc44           fedora      86.3 KiB
 perl-Scalar-List-Utils     x86_64 5:1.70-2.fc44             fedora     144.8 KiB
 perl-SelectSaver           noarch 0:1.02-524.fc44           updates      2.2 KiB
 perl-Socket                x86_64 4:2.041-1.fc44            updates    120.7 KiB
 perl-Storable              x86_64 1:3.37-522.fc44           fedora     235.1 KiB
 perl-Symbol                noarch 0:1.09-524.fc44           updates      6.8 KiB
 perl-Term-ANSIColor        noarch 0:5.01-522.fc44           fedora      97.5 KiB
 perl-Term-Cap              noarch 0:1.18-521.fc44           fedora      29.3 KiB
 perl-TermReadKey           x86_64 0:2.38-27.fc44            fedora      63.9 KiB
 perl-Text-ParseWords       noarch 0:3.31-521.fc44           fedora      13.6 KiB
 perl-Text-Tabs+Wrap        noarch 0:2024.001-521.fc44       fedora      22.6 KiB
 perl-Time-HiRes            x86_64 4:1.9778-521.fc44         fedora     115.6 KiB
 perl-Time-Local            noarch 2:1.350-521.fc44          fedora      69.0 KiB
 perl-URI                   noarch 0:5.35-1.fc44             updates    268.2 KiB
 perl-base                  noarch 0:2.27-524.fc44           updates     12.6 KiB
 perl-constant              noarch 0:1.33-522.fc44           fedora      26.2 KiB
 perl-if                    noarch 0:0.61.000-524.fc44       updates      5.8 KiB
 perl-interpreter           x86_64 4:5.42.2-524.fc44         updates    118.7 KiB
 perl-lib                   x86_64 0:0.65-524.fc44           updates      8.5 KiB
 perl-libnet                noarch 0:3.15-522.fc44           fedora     289.4 KiB
 perl-libs                  x86_64 4:5.42.2-524.fc44         updates     11.6 MiB
 perl-locale                noarch 0:1.13-524.fc44           updates      6.1 KiB
 perl-mro                   x86_64 0:1.29-524.fc44           updates     41.4 KiB
 perl-overload              noarch 0:1.40-524.fc44           updates     71.6 KiB
 perl-overloading           noarch 0:0.02-524.fc44           updates      4.9 KiB
 perl-parent                noarch 1:0.244-521.fc44          fedora      10.3 KiB
 perl-podlators             noarch 1:6.0.2-521.fc44          fedora     317.5 KiB
 perl-vars                  noarch 0:1.05-524.fc44           updates      3.9 KiB
 python-pip-wheel           noarch 0:26.0.1-2.fc44           fedora       1.2 MiB
 python3                    x86_64 0:3.14.6-1.fc44           updates     28.7 KiB
 python3-libs               x86_64 0:3.14.6-1.fc44           updates     43.9 MiB
 tzdata                     noarch 0:2026b-1.fc44            updates      1.2 MiB
 util-linux                 x86_64 0:2.41.5-1.fc44           updates      3.5 MiB
Installing weak dependencies:
 7zip                       x86_64 0:26.02-1.fc44            updates      3.3 MiB
 bat                        x86_64 0:0.26.1-1.fc44           fedora       5.9 MiB
 less-color                 x86_64 0:702-1.fc44              updates      2.3 KiB
 perl-NDBM_File             x86_64 0:1.18-524.fc44           updates     28.4 KiB
 python-unversioned-command noarch 0:3.14.6-1.fc44           updates     23.0   B
 python3-html2text          noarch 0:2025.4.15-6.fc44        fedora     289.4 KiB
 unzip                      x86_64 0:6.0-69.fc44             fedora     445.8 KiB

Transaction Summary:
 Installing:        88 packages

Total size of inbound packages is 37 MiB. Need to download 37 MiB.
After this operation, 139 MiB extra will be used (install 139 MiB, remove 0 B).
[ 1/88] git-0:2.55.0-1.fc44.x86_64      100% | 281.8 KiB/s |  41.1 KiB |  00m00s
[ 2/88] perl-Git-0:2.55.0-1.fc44.noarch 100% | 598.4 KiB/s |  37.7 KiB |  00m00s
[ 3/88] git-core-doc-0:2.55.0-1.fc44.no 100% |   6.5 MiB/s |   3.2 MiB |  00m00s
[ 4/88] perl-Getopt-Long-1:2.58-521.fc4 100% | 232.1 KiB/s |  63.6 KiB |  00m00s
[ 5/88] git-core-0:2.55.0-1.fc44.x86_64 100% |   9.8 MiB/s |   5.4 MiB |  00m01s
[ 6/88] perl-PathTools-0:3.94-521.fc44. 100% | 786.7 KiB/s |  87.3 KiB |  00m00s
[ 7/88] perl-Error-1:0.17030-3.fc44.noa 100% | 671.8 KiB/s |  40.3 KiB |  00m00s
[ 8/88] expat-0:2.8.1-1.fc44.x86_64     100% |   1.2 MiB/s | 129.7 KiB |  00m00s
[ 9/88] perl-Exporter-0:5.79-521.fc44.n 100% | 550.3 KiB/s |  30.8 KiB |  00m00s
[10/88] perl-TermReadKey-0:2.38-27.fc44 100% | 154.1 KiB/s |  35.5 KiB |  00m00s
[11/88] perl-Pod-Usage-4:2.05-521.fc44. 100% | 677.7 KiB/s |  40.7 KiB |  00m00s
[12/88] perl-Text-ParseWords-0:3.31-521 100% | 270.0 KiB/s |  16.5 KiB |  00m00s
[13/88] perl-Carp-0:1.54-521.fc44.noarc 100% | 523.3 KiB/s |  28.8 KiB |  00m00s
[14/88] perl-constant-0:1.33-522.fc44.n 100% | 112.3 KiB/s |  22.9 KiB |  00m00s
[15/88] perl-Scalar-List-Utils-5:1.70-2 100% | 628.3 KiB/s |  75.4 KiB |  00m00s
[16/88] perl-Pod-Perldoc-0:3.28.01-522. 100% |   1.2 MiB/s |  86.1 KiB |  00m00s
[17/88] perl-File-Temp-1:0.231.200-2.fc 100% | 960.7 KiB/s |  59.6 KiB |  00m00s
[18/88] perl-podlators-1:6.0.2-521.fc44 100% | 793.1 KiB/s | 128.5 KiB |  00m00s
[19/88] perl-parent-1:0.244-521.fc44.no 100% | 310.0 KiB/s |  14.9 KiB |  00m00s
[20/88] perl-Pod-Simple-1:3.47-4.fc44.n 100% |   1.7 MiB/s | 220.0 KiB |  00m00s
[21/88] groff-base-0:1.23.0-12.fc44.x86 100% |   5.3 MiB/s |   1.1 MiB |  00m00s
[22/88] perl-Term-ANSIColor-0:5.01-522. 100% | 836.4 KiB/s |  47.7 KiB |  00m00s
[23/88] perl-Term-Cap-0:1.18-521.fc44.n 100% | 458.3 KiB/s |  22.0 KiB |  00m00s
[24/88] perl-File-Path-0:2.18-522.fc44. 100% | 783.5 KiB/s |  35.3 KiB |  00m00s
[25/88] perl-Text-Tabs+Wrap-0:2024.001- 100% | 453.1 KiB/s |  21.8 KiB |  00m00s
[26/88] perl-Pod-Escapes-1:1.07-521.fc4 100% | 388.1 KiB/s |  19.8 KiB |  00m00s
[27/88] ncurses-0:6.6-1.fc44.x86_64     100% |   6.9 MiB/s | 429.8 KiB |  00m00s
[28/88] perl-File-Basename-0:2.86-524.f 100% | 434.2 KiB/s |  16.9 KiB |  00m00s
[29/88] perl-POSIX-0:2.23-524.fc44.x86_ 100% |   2.4 MiB/s |  96.8 KiB |  00m00s
[30/88] perl-interpreter-4:5.42.2-524.f 100% |   1.7 MiB/s |  72.1 KiB |  00m00s
[31/88] perl-Errno-0:1.38-524.fc44.x86_ 100% | 408.6 KiB/s |  14.7 KiB |  00m00s
[32/88] perl-MIME-Base64-0:3.16-521.fc4 100% | 595.4 KiB/s |  29.8 KiB |  00m00s
[33/88] perl-Encode-4:3.21-521.fc44.x86 100% |  13.8 MiB/s |   1.1 MiB |  00m00s
[34/88] perl-Storable-1:3.37-522.fc44.x 100% |   1.7 MiB/s | 100.9 KiB |  00m00s
[35/88] perl-DynaLoader-0:1.57-524.fc44 100% | 644.4 KiB/s |  25.8 KiB |  00m00s
[36/88] perl-vars-0:1.05-524.fc44.noarc 100% | 319.2 KiB/s |  12.8 KiB |  00m00s
[37/88] perl-Fcntl-0:1.20-524.fc44.x86_ 100% | 737.8 KiB/s |  29.5 KiB |  00m00s
[38/88] perl-libs-4:5.42.2-524.fc44.x86 100% |  11.9 MiB/s |   2.6 MiB |  00m00s
[39/88] perl-IO-0:1.55-524.fc44.x86_64  100% |   2.1 MiB/s |  81.9 KiB |  00m00s
[40/88] perl-Symbol-0:1.09-524.fc44.noa 100% | 358.4 KiB/s |  14.0 KiB |  00m00s
[41/88] perl-if-0:0.61.000-524.fc44.noa 100% | 393.8 KiB/s |  13.8 KiB |  00m00s
[42/88] perl-overload-0:1.40-524.fc44.n 100% |   1.2 MiB/s |  45.3 KiB |  00m00s
[43/88] perl-HTTP-Tiny-0:0.094-1.fc44.n 100% |   1.4 MiB/s |  57.1 KiB |  00m00s
[44/88] perl-IO-Socket-SSL-0:2.098-2.fc 100% |   4.2 MiB/s | 234.7 KiB |  00m00s
[45/88] perl-Time-HiRes-4:1.9778-521.fc 100% |   1.0 MiB/s |  57.2 KiB |  00m00s
[46/88] perl-Time-Local-2:1.350-521.fc4 100% | 821.6 KiB/s |  34.5 KiB |  00m00s
[47/88] perl-Net-SSLeay-0:1.94-12.fc44. 100% |   3.6 MiB/s | 373.4 KiB |  00m00s
[48/88] perl-IO-Socket-IP-0:0.43-522.fc 100% | 766.6 KiB/s |  42.2 KiB |  00m00s
[49/88] perl-IPC-Open3-0:1.24-524.fc44. 100% | 607.6 KiB/s |  23.7 KiB |  00m00s
[50/88] perl-AutoLoader-0:5.74-524.fc44 100% | 568.1 KiB/s |  21.0 KiB |  00m00s
[51/88] perl-Socket-4:2.041-1.fc44.x86_ 100% |   1.2 MiB/s |  55.0 KiB |  00m00s
[52/88] perl-URI-0:5.35-1.fc44.noarch   100% |   3.5 MiB/s | 148.9 KiB |  00m00s
[53/88] perl-MIME-Base32-0:1.303-25.fc4 100% | 466.4 KiB/s |  20.5 KiB |  00m00s
[54/88] perl-libnet-0:3.15-522.fc44.noa 100% |   2.5 MiB/s | 128.4 KiB |  00m00s
[55/88] perl-Digest-MD5-0:2.59-521.fc44 100% | 667.0 KiB/s |  36.0 KiB |  00m00s
[56/88] perl-Digest-0:1.20-521.fc44.noa 100% | 507.3 KiB/s |  24.9 KiB |  00m00s
[57/88] perl-base-0:2.27-524.fc44.noarc 100% | 499.6 KiB/s |  16.0 KiB |  00m00s
[58/88] perl-Getopt-Std-0:1.14-524.fc44 100% | 351.8 KiB/s |  15.5 KiB |  00m00s
[59/88] perl-mro-0:1.29-524.fc44.x86_64 100% | 672.7 KiB/s |  29.6 KiB |  00m00s
[60/88] perl-overloading-0:0.02-524.fc4 100% | 333.6 KiB/s |  12.7 KiB |  00m00s
[61/88] perl-Data-Dumper-0:2.192-523.fc 100% |   1.4 MiB/s |  56.3 KiB |  00m00s
[62/88] perl-locale-0:1.13-524.fc44.noa 100% | 349.5 KiB/s |  13.3 KiB |  00m00s
[63/88] perl-File-stat-0:1.14-524.fc44. 100% | 431.8 KiB/s |  16.8 KiB |  00m00s
[64/88] perl-SelectSaver-0:1.02-524.fc4 100% | 310.9 KiB/s |  11.5 KiB |  00m00s
[65/88] perl-Class-Struct-0:0.68-524.fc 100% | 546.6 KiB/s |  21.9 KiB |  00m00s
[66/88] perl-B-0:1.89-524.fc44.x86_64   100% |   4.0 MiB/s | 177.7 KiB |  00m00s
[67/88] less-0:702-1.fc44.x86_64        100% |   4.9 MiB/s | 227.1 KiB |  00m00s
[68/88] perl-lib-0:0.65-524.fc44.x86_64 100% | 342.1 KiB/s |  14.7 KiB |  00m00s
[69/88] perl-FileHandle-0:2.05-524.fc44 100% | 391.7 KiB/s |  15.3 KiB |  00m00s
[70/88] openssh-clients-0:10.2p1-12.fc4 100% |   5.4 MiB/s | 778.3 KiB |  00m00s
[71/88] libedit-0:3.1-59.20260512cvs.fc 100% |   2.6 MiB/s | 110.4 KiB |  00m00s
[72/88] openssh-0:10.2p1-12.fc44.x86_64 100% |   2.2 MiB/s | 355.7 KiB |  00m00s
[73/88] liblastlog2-0:2.41.5-1.fc44.x86 100% | 604.3 KiB/s |  24.2 KiB |  00m00s
[74/88] perl-NDBM_File-0:1.18-524.fc44. 100% | 637.0 KiB/s |  22.3 KiB |  00m00s
[75/88] util-linux-0:2.41.5-1.fc44.x86_ 100% |  11.4 MiB/s |   1.2 MiB |  00m00s
[76/88] libfdisk-0:2.41.5-1.fc44.x86_64 100% |   1.7 MiB/s | 168.6 KiB |  00m00s
[77/88] python3-html2text-0:2025.4.15-6 100% |   1.6 MiB/s |  81.7 KiB |  00m00s
[78/88] less-color-0:702-1.fc44.x86_64  100% | 270.2 KiB/s |  10.5 KiB |  00m00s
[79/88] python3-0:3.14.6-1.fc44.x86_64  100% | 758.5 KiB/s |  28.8 KiB |  00m00s
[80/88] unzip-0:6.0-69.fc44.x86_64      100% |   1.3 MiB/s | 199.6 KiB |  00m00s
[81/88] 7zip-0:26.02-1.fc44.x86_64      100% |   8.6 MiB/s |   1.3 MiB |  00m00s
[82/88] python-pip-wheel-0:26.0.1-2.fc4 100% |   7.7 MiB/s |   1.1 MiB |  00m00s
[83/88] mpdecimal-0:4.0.1-3.fc44.x86_64 100% | 483.6 KiB/s |  99.1 KiB |  00m00s
[84/88] tzdata-0:2026b-1.fc44.noarch    100% |   5.4 MiB/s | 713.4 KiB |  00m00s
[85/88] python3-libs-0:3.14.6-1.fc44.x8 100% |  27.5 MiB/s |  10.1 MiB |  00m00s
[86/88] oniguruma-0:6.9.10-4.fc44.x86_6 100% |   2.9 MiB/s | 220.0 KiB |  00m00s
[87/88] python-unversioned-command-0:3. 100% | 280.8 KiB/s |  11.0 KiB |  00m00s
[88/88] bat-0:0.26.1-1.fc44.x86_64      100% |   2.8 MiB/s |   2.8 MiB |  00m01s
--------------------------------------------------------------------------------
[88/88] Total                           100% |  10.6 MiB/s |  36.8 MiB |  00m03s
Running transaction
[ 1/90] Verify package files            100% | 916.0   B/s |  88.0   B |  00m00s
[ 2/90] Prepare transaction             100% |   1.5 KiB/s |  88.0   B |  00m00s
[ 3/90] Installing less-0:702-1.fc44.x8 100% |  33.6 MiB/s | 481.2 KiB |  00m00s
[ 4/90] Installing expat-0:2.8.1-1.fc44 100% |  32.2 MiB/s | 329.5 KiB |  00m00s
[ 5/90] Installing oniguruma-0:6.9.10-4 100% | 125.8 MiB/s | 773.1 KiB |  00m00s
[ 6/90] Installing tzdata-0:2026b-1.fc4 100% |  11.5 MiB/s |   1.5 MiB |  00m00s
[ 7/90] Installing python-pip-wheel-0:2 100% | 407.6 MiB/s |   1.2 MiB |  00m00s
[ 8/90] Installing mpdecimal-0:4.0.1-3. 100% |  42.7 MiB/s | 218.6 KiB |  00m00s
[ 9/90] Installing python3-libs-0:3.14. 100% | 167.7 MiB/s |  44.3 MiB |  00m00s
[10/90] Installing python3-0:3.14.6-1.f 100% |   2.7 MiB/s |  30.5 KiB |  00m00s
[11/90] Installing liblastlog2-0:2.41.5 100% |   2.8 MiB/s |  43.7 KiB |  00m00s
[12/90] Installing libfdisk-0:2.41.5-1. 100% | 126.8 MiB/s | 389.4 KiB |  00m00s
[13/90] Installing util-linux-0:2.41.5- 100% |  56.0 MiB/s |   3.6 MiB |  00m00s
[14/90] Installing openssh-0:10.2p1-12. 100% | 110.2 MiB/s |   1.4 MiB |  00m00s
[15/90] Installing libedit-0:3.1-59.202 100% | 122.0 MiB/s | 250.0 KiB |  00m00s
[16/90] Installing openssh-clients-0:10 100% |  84.1 MiB/s |   2.7 MiB |  00m00s
[17/90] Installing git-core-0:2.55.0-1. 100% | 388.0 MiB/s |  25.6 MiB |  00m00s
[18/90] Installing git-core-doc-0:2.55. 100% |   1.3 GiB/s |  19.1 MiB |  00m00s
[19/90] Installing ncurses-0:6.6-1.fc44 100% |  20.1 MiB/s | 618.0 KiB |  00m00s
[20/90] Installing groff-base-0:1.23.0- 100% |  77.9 MiB/s |   3.9 MiB |  00m00s
[21/90] Installing perl-Digest-0:1.20-5 100% |  36.2 MiB/s |  37.1 KiB |  00m00s
[22/90] Installing perl-FileHandle-0:2. 100% |   9.6 MiB/s |   9.8 KiB |  00m00s
[23/90] Installing perl-B-0:1.89-524.fc 100% | 123.2 MiB/s | 504.5 KiB |  00m00s
[24/90] Installing perl-Digest-MD5-0:2. 100% |  60.0 MiB/s |  61.5 KiB |  00m00s
[25/90] Installing perl-libnet-0:3.15-5 100% |  71.9 MiB/s | 294.7 KiB |  00m00s
[26/90] Installing perl-Data-Dumper-0:2 100% |  57.3 MiB/s | 117.4 KiB |  00m00s
[27/90] Installing perl-MIME-Base32-0:1 100% |  31.4 MiB/s |  32.2 KiB |  00m00s
[28/90] Installing perl-URI-0:5.35-1.fc 100% |  39.3 MiB/s | 282.0 KiB |  00m00s
[29/90] Installing perl-IO-Socket-IP-0: 100% |  99.8 MiB/s | 102.2 KiB |  00m00s
[30/90] Installing perl-AutoLoader-0:5. 100% |   0.0   B/s |  21.0 KiB |  00m00s
[31/90] Installing perl-IO-Socket-SSL-0 100% | 236.9 MiB/s | 727.6 KiB |  00m00s
[32/90] Installing perl-locale-0:1.13-5 100% |   6.4 MiB/s |   6.5 KiB |  00m00s
[33/90] Installing perl-Net-SSLeay-0:1. 100% | 122.4 MiB/s |   1.3 MiB |  00m00s
[34/90] Installing perl-Time-Local-2:1. 100% |  68.9 MiB/s |  70.6 KiB |  00m00s
[35/90] Installing perl-Time-HiRes-4:1. 100% | 114.9 MiB/s | 117.7 KiB |  00m00s
[36/90] Installing perl-if-0:0.61.000-5 100% |   0.0   B/s |   6.2 KiB |  00m00s
[37/90] Installing perl-Pod-Escapes-1:1 100% |  25.3 MiB/s |  25.9 KiB |  00m00s
[38/90] Installing perl-File-Path-0:2.1 100% |  63.0 MiB/s |  64.5 KiB |  00m00s
[39/90] Installing perl-Text-Tabs+Wrap- 100% |  23.3 MiB/s |  23.9 KiB |  00m00s
[40/90] Installing perl-HTTP-Tiny-0:0.0 100% | 156.5 MiB/s | 160.3 KiB |  00m00s
[41/90] Installing perl-Class-Struct-0: 100% |   0.0   B/s |  25.9 KiB |  00m00s
[42/90] Installing perl-IPC-Open3-0:1.2 100% |   0.0   B/s |  28.5 KiB |  00m00s
[43/90] Installing perl-File-Temp-1:0.2 100% | 161.6 MiB/s | 165.5 KiB |  00m00s
[44/90] Installing perl-POSIX-0:2.23-52 100% |  75.1 MiB/s | 230.6 KiB |  00m00s
[45/90] Installing perl-Pod-Simple-1:3. 100% | 112.3 MiB/s | 574.9 KiB |  00m00s
[46/90] Installing perl-Term-Cap-0:1.18 100% |  29.9 MiB/s |  30.6 KiB |  00m00s
[47/90] Installing perl-Term-ANSIColor- 100% |  96.9 MiB/s |  99.2 KiB |  00m00s
[48/90] Installing perl-Pod-Perldoc-0:3 100% |  15.0 MiB/s | 169.2 KiB |  00m00s
[49/90] Installing perl-podlators-1:6.0 100% |  28.5 MiB/s | 321.4 KiB |  00m00s
[50/90] Installing perl-File-stat-0:1.1 100% |  12.8 MiB/s |  13.1 KiB |  00m00s
[51/90] Installing perl-SelectSaver-0:1 100% |   2.6 MiB/s |   2.6 KiB |  00m00s
[52/90] Installing perl-Socket-4:2.041- 100% |  59.9 MiB/s | 122.7 KiB |  00m00s
[53/90] Installing perl-Symbol-0:1.09-5 100% |   0.0   B/s |   7.3 KiB |  00m00s
[54/90] Installing perl-Pod-Usage-4:2.0 100% |   8.6 MiB/s |  87.9 KiB |  00m00s
[55/90] Installing perl-IO-0:1.55-524.f 100% |  49.3 MiB/s | 151.5 KiB |  00m00s
[56/90] Installing perl-overloading-0:0 100% |   5.4 MiB/s |   5.6 KiB |  00m00s
[57/90] Installing perl-mro-0:1.29-524. 100% |  41.6 MiB/s |  42.6 KiB |  00m00s
[58/90] Installing perl-base-0:2.27-524 100% |   0.0   B/s |  13.0 KiB |  00m00s
[59/90] Installing perl-Fcntl-0:1.20-52 100% |  48.6 MiB/s |  49.8 KiB |  00m00s
[60/90] Installing perl-Text-ParseWords 100% |   0.0   B/s |  14.6 KiB |  00m00s
[61/90] Installing perl-Getopt-Long-1:2 100% | 143.8 MiB/s | 147.2 KiB |  00m00s
[62/90] Installing perl-Storable-1:3.37 100% | 115.6 MiB/s | 236.7 KiB |  00m00s
[63/90] Installing perl-overload-0:1.40 100% |   0.0   B/s |  72.0 KiB |  00m00s
[64/90] Installing perl-Getopt-Std-0:1. 100% |   0.0   B/s |  11.8 KiB |  00m00s
[65/90] Installing perl-vars-0:1.05-524 100% |   0.0   B/s |   4.3 KiB |  00m00s
[66/90] Installing perl-MIME-Base64-0:3 100% |  21.6 MiB/s |  44.2 KiB |  00m00s
[67/90] Installing perl-Errno-0:1.38-52 100% |   0.0   B/s |   8.8 KiB |  00m00s
[68/90] Installing perl-File-Basename-0 100% |   0.0   B/s |  14.6 KiB |  00m00s
[69/90] Installing perl-parent-1:0.244- 100% |   0.0   B/s |  11.0 KiB |  00m00s
[70/90] Installing perl-Scalar-List-Uti 100% |  48.3 MiB/s | 148.5 KiB |  00m00s
[71/90] Installing perl-constant-0:1.33 100% |  26.7 MiB/s |  27.4 KiB |  00m00s
[72/90] Installing perl-Encode-4:3.21-5 100% | 156.4 MiB/s |   4.7 MiB |  00m00s
[73/90] Installing perl-DynaLoader-0:1. 100% |  31.7 MiB/s |  32.5 KiB |  00m00s
[74/90] Installing perl-PathTools-0:3.9 100% |  60.0 MiB/s | 184.4 KiB |  00m00s
[75/90] Installing perl-Carp-0:1.54-521 100% |  46.6 MiB/s |  47.7 KiB |  00m00s
[76/90] Installing perl-Exporter-0:5.79 100% |  27.1 MiB/s |  55.6 KiB |  00m00s
[77/90] Installing perl-libs-4:5.42.2-5 100% | 139.0 MiB/s |  11.7 MiB |  00m00s
[78/90] Installing perl-interpreter-4:5 100% |  11.8 MiB/s | 120.4 KiB |  00m00s
[79/90] Installing perl-TermReadKey-0:2 100% |  32.3 MiB/s |  66.1 KiB |  00m00s
[80/90] Installing perl-Error-1:0.17030 100% |  78.1 MiB/s |  80.0 KiB |  00m00s
[81/90] Installing perl-lib-0:0.65-524. 100% |   0.0   B/s |   8.9 KiB |  00m00s
[82/90] Installing git-0:2.55.0-1.fc44. 100% |  57.6 MiB/s |  59.0 KiB |  00m00s
[83/90] Installing perl-Git-0:2.55.0-1. 100% |  63.8 MiB/s |  65.4 KiB |  00m00s
[84/90] Installing perl-NDBM_File-0:1.1 100% |  28.8 MiB/s |  29.5 KiB |  00m00s
[85/90] Installing python3-html2text-0: 100% |  22.4 MiB/s | 297.9 KiB |  00m00s
[86/90] Installing python-unversioned-c 100% |  51.8 KiB/s | 424.0   B |  00m00s
[87/90] Installing bat-0:0.26.1-1.fc44. 100% | 257.1 MiB/s |   5.9 MiB |  00m00s
[88/90] Installing less-color-0:702-1.f 100% | 236.9 KiB/s |   2.6 KiB |  00m00s
[89/90] Installing 7zip-0:26.02-1.fc44. 100% | 145.6 MiB/s |   3.3 MiB |  00m00s
[90/90] Installing unzip-0:6.0-69.fc44. 100% |   7.6 MiB/s | 449.3 KiB |  00m00s
Complete!
Removed 18 files, 11 directories (total of 97 MiB). 0 errors occurred.
--> 05ba993cf5b4
STEP 3/5: COPY scripts/commit-loop.sh /usr/local/bin/commit-loop.sh
--> 7d41e97a4205
STEP 4/5: RUN chmod +x /usr/local/bin/commit-loop.sh
--> 0debdc8eb092
STEP 5/5: ENTRYPOINT ["/usr/local/bin/commit-loop.sh"]
COMMIT localhost/myinkbird-committer:latest
--> 655b42b715d6
Successfully tagged localhost/myinkbird-committer:latest
655b42b715d628b3676c30d7346c343c6dd77c080994dd3d3c553d83a872a980
[myinkbird] images built

real	1m19.206s
user	1m53.012s
sys	0m24.790s
[myinkbird] building collector image (localhost/myinkbird-collector:latest)
[1/2] STEP 1/7: FROM registry.fedoraproject.org/fedora:latest AS build
[1/2] STEP 2/7: RUN dnf -y install gcc pkgconf-pkg-config dbus-devel rustup     && rustup-init -y --profile minimal --default-toolchain stable --no-modify-path     && dnf clean all
--> Using cache 2cf0c0d4fa5177707c2773b62a6db523b88711a7a064b4b95e01d7b08ce59b28
--> 2cf0c0d4fa51
[1/2] STEP 3/7: ENV PATH="/root/.cargo/bin:${PATH}"
--> Using cache f45acc341346a77f946cfba309a03cdc4793e67bab8dc544eb8943b3d96121f3
--> f45acc341346
[1/2] STEP 4/7: WORKDIR /src
--> Using cache 74741fe66b1c245235c4b20ded6fb257b742b75c7a682f55db5703459dfc9599
--> 74741fe66b1c
[1/2] STEP 5/7: COPY Cargo.toml ./
--> Using cache b85927c1391c0c472b247573e9d0e165309417f440b55942b45ba32d91eadeda
--> b85927c1391c
[1/2] STEP 6/7: COPY crates ./crates
--> Using cache 0e589d74652db5a1257fef16d79d0dce9f30f138692ec46cb904a51a853a43b9
--> 0e589d74652d
[1/2] STEP 7/7: RUN cargo build --release -p inkbird-collector
--> Using cache d6bb075e219969c7432b1623706c6798564060ea88776bfe90d23f347d1e6059
--> d6bb075e2199
[2/2] STEP 1/5: FROM registry.fedoraproject.org/fedora-minimal:latest AS runtime
[2/2] STEP 2/5: RUN microdnf install -y dbus-libs && microdnf clean all
--> Using cache e6e1b9cbd58bd16507bb7aaefa18343f7cba4dcda92648602efbc3fb91572a69
--> e6e1b9cbd58b
[2/2] STEP 3/5: COPY --from=build /src/target/release/inkbird-collector /usr/local/bin/inkbird-collector
--> Using cache 7f92595cd29a219811a138052b5be784106930a84b3d4ee11a17702b01dfd452
--> 7f92595cd29a
[2/2] STEP 4/5: ENTRYPOINT ["/usr/local/bin/inkbird-collector"]
--> Using cache 1d0db46757b13340652362293cad45edf2c08b621847a19764817cd3c281b25a
--> 1d0db46757b1
[2/2] STEP 5/5: CMD ["collect"]
--> Using cache 48bde1c5823054aaf1fd27c05a60fde0e69c1db42aca0972897c07e67bee9e1e
[2/2] COMMIT localhost/myinkbird-collector:latest
--> 48bde1c58230
Successfully tagged localhost/myinkbird-collector:latest
48bde1c5823054aaf1fd27c05a60fde0e69c1db42aca0972897c07e67bee9e1e
[myinkbird] building committer image (localhost/myinkbird-committer:latest)
STEP 1/5: FROM registry.fedoraproject.org/fedora-minimal:latest
STEP 2/5: RUN microdnf install -y git && microdnf clean all
--> Using cache 05ba993cf5b46084651a307c333724e173b29e235b9e037334dbaebff28b96a2
--> 05ba993cf5b4
STEP 3/5: COPY scripts/commit-loop.sh /usr/local/bin/commit-loop.sh
--> Using cache 7d41e97a42055ec3217aedb7483797fff2d8fced4769ade4bfb0a9b8bb802c06
--> 7d41e97a4205
STEP 4/5: RUN chmod +x /usr/local/bin/commit-loop.sh
--> Using cache 0debdc8eb09223f34bc0c124d0b740c323facd85b7bc584125564adf83449571
--> 0debdc8eb092
STEP 5/5: ENTRYPOINT ["/usr/local/bin/commit-loop.sh"]
--> Using cache 655b42b715d628b3676c30d7346c343c6dd77c080994dd3d3c553d83a872a980
COMMIT localhost/myinkbird-committer:latest
--> 655b42b715d6
Successfully tagged localhost/myinkbird-committer:latest
655b42b715d628b3676c30d7346c343c6dd77c080994dd3d3c553d83a872a980
[myinkbird] images built
[myinkbird] starting stack (rootful for BlueZ D-Bus access)
[sudo] password for kushal: 
>>>> Executing external compose provider "/usr/sbin/podman-compose". Please see podman-compose(1) for how to disable this message. <<<<

e4f355d226292648676bca12a5b58f14f93df5486171d25361d186f577da8c31
Trying to pull localhost/myinkbird-collector:latest...
WARN[0000] Failed, retrying in 1s ... (1/3). Error: initializing source docker://localhost/myinkbird-collector:latest: pinging container registry localhost: Get "https://localhost/v2/": dial tcp [::1]:443: connect: connection refused 
WARN[0001] Failed, retrying in 1s ... (2/3). Error: initializing source docker://localhost/myinkbird-collector:latest: pinging container registry localhost: Get "https://localhost/v2/": dial tcp [::1]:443: connect: connection refused 
WARN[0002] Failed, retrying in 1s ... (3/3). Error: initializing source docker://localhost/myinkbird-collector:latest: pinging container registry localhost: Get "https://localhost/v2/": dial tcp [::1]:443: connect: connection refused 
Error: unable to copy from source docker://localhost/myinkbird-collector:latest: initializing source docker://localhost/myinkbird-collector:latest: pinging container registry localhost: Get "https://localhost/v2/": dial tcp [::1]:443: connect: connection refused
Trying to pull localhost/myinkbird-committer:latest...
WARN[0000] Failed, retrying in 1s ... (1/3). Error: initializing source docker://localhost/myinkbird-committer:latest: pinging container registry localhost: Get "https://localhost/v2/": dial tcp [::1]:443: connect: connection refused 
WARN[0001] Failed, retrying in 1s ... (2/3). Error: initializing source docker://localhost/myinkbird-committer:latest: pinging container registry localhost: Get "https://localhost/v2/": dial tcp [::1]:443: connect: connection refused 
WARN[0002] Failed, retrying in 1s ... (3/3). Error: initializing source docker://localhost/myinkbird-committer:latest: pinging container registry localhost: Get "https://localhost/v2/": dial tcp [::1]:443: connect: connection refused 
Error: unable to copy from source docker://localhost/myinkbird-committer:latest: initializing source docker://localhost/myinkbird-committer:latest: pinging container registry localhost: Get "https://localhost/v2/": dial tcp [::1]:443: connect: connection refused
Error: no container with name or ID "myinkbird_collector_1" found: no such container
^C
real	8m34.054s
user	0m0.663s
sys	0m0.760s
kushal@fedora:~/src/rust/myinkbird$ cd ~/src/rust/myinkbird/; time bash scripts/stop.sh 
[sudo] password for kushal: 
>>>> Executing external compose provider "/usr/sbin/podman-compose". Please see podman-compose(1) for how to disable this message. <<<<

Error: no container with name or ID "myinkbird_committer_1" found: no such container
Error: no container with name or ID "myinkbird_collector_1" found: no such container
Error: no container with ID or name "myinkbird_committer_1" found: no such container
Error: no container with ID or name "myinkbird_collector_1" found: no such container
e4f355d226292648676bca12a5b58f14f93df5486171d25361d186f577da8c31
myinkbird_default
[myinkbird] stack stopped

real	0m5.789s
user	0m0.133s
sys	0m0.089s
kushal@fedora:~/src/rust/myinkbird$ 
