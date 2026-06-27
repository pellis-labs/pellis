# Licensing

Pellis is a platform spanning hardware, firmware, software, models, datasets, and
documentation. No single license fits all of those, so Pellis uses a **per-artifact
license scheme**. This is deliberate and is the same approach used by other open
hardware platforms. (GitHub's license detector expects one license per repo and will
not auto-detect this scheme — that's expected.)

## The scheme

| Artifact type | License | Full text |
|---|---|---|
| **Hardware** design files (schematics, PCB, CAD, mechanical) | CERN-OHL-S v2 (strongly reciprocal) | [`licenses/CERN-OHL-S-v2.txt`](licenses/CERN-OHL-S-v2.txt) |
| **Firmware**, the device **library**, and **reference models** | Apache License 2.0 | [`licenses/Apache-2.0.txt`](licenses/Apache-2.0.txt) |
| **Datasets** | CC-BY-SA 4.0 | (added when the first dataset lands) |
| **Documentation** (including this repo's current contents) | CC-BY 4.0 | [`licenses/CC-BY-4.0.txt`](licenses/CC-BY-4.0.txt) |

## What applies right now

This repository currently contains **documentation only** (vision, governance,
licensing). All of it is licensed **CC-BY 4.0** unless a file or directory states
otherwise. As hardware, firmware, and datasets land, each will carry its own license
per the table above, and the relevant license texts will be added under
[`licenses/`](licenses/).

## Why these choices

- **CERN-OHL-S v2 (hardware)** — strongly reciprocal: improvements to the open hardware
  designs flow back to the commons. The right default for a foundation primitive.
- **Apache 2.0 (firmware / library / models)** — permissive with an explicit patent
  grant, so the library is safe to build commercial products on top of. Maximizes
  adoption, which is the entire point.
- **CC-BY (docs)** — share and adapt freely, with attribution.

## What is *not* covered by these licenses

The **PELLIS** trademark is reserved (USPTO Class 9, intent-to-use, filed 2026-05-05).
Open licenses cover copyright and (for Apache/CERN) patents — they do **not** grant
trademark rights. You can use, modify, and build on the open artifacts; you cannot use
the Pellis name or marks to brand your own product without permission.
