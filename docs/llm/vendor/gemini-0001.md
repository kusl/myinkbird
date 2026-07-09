please review this code. please check specifically for consistency, defects, and pay special attention to whether this code causes battery drain in the inkbird. go slow and please review the full dump methodically and cross reference everything because the dump contains all the code 

This is an incredibly thorough and well-documented repository. Reviewing a codebase that relies so heavily on Architecture Decision Records (ADRs) makes it much easier to trace the *why* behind the *how*.

I have methodically reviewed the provided context dump, focusing strictly on your concerns regarding battery drain, architectural consistency, and potential defects.

### **Battery Drain Analysis**

You explicitly asked if this code will cause battery drain on the INKBIRD ITH-13-B. The short answer is: **It avoids the massive battery drain of a GATT connection, but it does incur the minor overhead of an active scan.**

* 
**The Good (No Connection Overhead):** Your code perfectly respects the "listen-only" mandate outlined in ADR 0003. Looking at `crates/inkbird-collector/src/scanner.rs`, the logic never invokes a `connect()` method. It exclusively monitors `CentralEvent::DeviceDiscovered` and `CentralEvent::DeviceUpdated` , reads the advertised `properties()` , and extracts the `manufacturer_data`. By never establishing a GATT connection, you prevent the sensor from being forced into a power-hungry connection interval schedule.


* 
**The Nuance (Active Scanning):** You correctly identified and documented this limitation, and it bears repeating for the code review. The `btleplug` library utilizes BlueZ's `StartDiscovery` method, which initiates an *active* scan. During an active scan, your host's Bluetooth adapter broadcasts `SCAN_REQ` (Scan Request) packets. The ITH-13-B, upon hearing these, will respond with `SCAN_RSP` (Scan Response) packets. While this uses exponentially less power than maintaining a full connection, transmitting these responses *does* consume marginally more battery than a pure passive scan (like BlueZ's `AdvertisementMonitor` API).



### **Consistency Check**

The implementation aligns remarkably well with your stated rules and ADRs.

* 
**Rootful Execution:** The choice to use `sudo` and run rootful is consistently applied across `compose.yaml` , `scripts/run.sh` , and `scripts/container-build.sh`. You correctly documented that rootless Podman uses a separate image store , and your scripts account for this by building and running in the same rootful context.


* 
**Empty Address Handling:** Compose passes an unset `INKBIRD_ADDRESS` as an empty string (`""`). In `crates/inkbird-collector/src/config.rs`, you elegantly handle this via `.filter(|a| !a.is_empty())`. This cleanly normalizes the empty string to `None`, forcing the scanner to fall back to the `name_match` heuristic as intended.


* 
**Git Sidecar Logic:** ADR 0006 dictates that data is kept locally without outbound pushes. The `commit-loop.sh` perfectly executes this by strictly running `git add -A` and `git commit` inside a loop without any `git push` commands or remote configurations.


* 
**Vulnerability Mitigation:** You documented a goal to avoid `RUSTSEC-2026-0009` by pinning the `time` crate to `>= 0.3.47`. The `Cargo.lock` file confirms you have pinned `time` to version `0.3.53`, successfully mitigating the stack-exhaustion vulnerability.



### **Code Quality & Potential Defects**

The Rust code is highly defensive and well-structured, but here are a few implementation details to be aware of:

* 
**File I/O Overhead:** In `crates/inkbird-collector/src/ndjson_sink.rs`, the `record` method opens the file, appends a single line, and drops the file handle for every reading. Given the default 60-second throttle, this is not a performance defect for a single sensor. However, it is an I/O-inefficient design. If you eventually deploy multiple sensors broadcasting rapidly, opening and closing the file descriptor per line could cause unnecessary disk churn.


* 
**Data Validation:** The logic in `crates/inkbird-core/src/parser.rs` strictly drops the *entire* reading if `humidity > 100.0` or `battery > 100`. This is an excellent defense mechanism against corrupt BLE packets, ensuring garbage values do not permanently pollute your NDJSON logs.


* 
**Negative Temperatures:** The decoder handles signed temperatures correctly using `i16::from_le_bytes`, which properly honors two's complement. As shown in your tests, `-55` correctly decodes to `-5.5°C`.


* 
**Time Formatting Fallback:** In `scanner.rs`, the `now_rfc3339()` function uses `now.format(&Rfc3339).unwrap_or_else(|_| now.to_string())`. This is a great piece of defensive programming, ensuring that if the rigorous RFC3339 formatter fails, the scanner loop will not panic and crash.



---

Summarize architectural changes of the BLE collector

The architecture establishes a passive BLE listening stack for the INKBIRD ITH-13-B to prevent sensor battery drain. It implements a decoupled containerized environment utilizing Podman, separating the core decoding library (`inkbird-core`) from the BLE collection and NDJSON storage binary (`inkbird-collector`). Data persistence is handled via a host-mounted directory with a dedicated sidecar container ensuring periodic local git commits without outbound network dependencies.

Are you planning to deploy this collector continuously on a low-power host like a Raspberry Pi, or will it primarily run alongside other services on a standard workstation?
