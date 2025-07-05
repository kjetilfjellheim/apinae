---
title: Introduction
type: docs
---

# Apinae

Mock and test utility for apis.

## Getting Started

### Prerequisites

The application requires at least Rust 1.84. It might work on earlier versions but this is untested.

Rust language
- https://www.rust-lang.org

The UI is written using the Tauri library with Vue 3 as frontend.

Vue 3 frontend documentation.
- https://vuejs.org/

Tauri documentation
- https://v2.tauri.app/

Currently only Linux is supported, later Windows and Mac might get support.

The following is recommended.

For finding outdated dependencies.
Install by ``` cargo install cargo-outdated ```
Run ```cargo outdated -w -R```

For improving code.
Run ```cargo clippy --all-targets --all-features -- -D warnings```

The prerequisites for Tauri can be found here.
- https://v1.tauri.app/v1/guides/getting-started/prerequisites


<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Build daemon

For testing
```
cargo build
```

For release
```
cargo build --release
```

### Build UI

For testing run the following from the apinae-ui directory.
```
npm run tauri dev
```

For release
```
cargo tauri build --no-bundle   
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Installation

The versions built is found either in target/debug or target/release

Copy the apinae-daemon or apinae-ui to /usr/bin. The UI application expects the daemon application to be called apinae. 

