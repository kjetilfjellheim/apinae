# Apinae

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#build-daemon">Build daemon</a></li>        
        <li><a href="#build-ui">Build UI</a></li>        
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>

## About The Project

This is an application for helping testing apis. It is still very much in an early phase of development, but the basics
have been implemented. It currently allows for mocking rest responses, timeout and testing tcp issues. It also allows 
for routing request to further to other apis.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

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

## Usage

| Parameter  | Description |
| ------------- | ------------- |
| --file | Configuration file |
| --id | ID of the test to start from the configuration file. |
| --list | Lists all tests from the configuration file. |
| --help | Help. |
| --version | Shows application version. |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Roadmap

Roadmap to 0.1.0
- [x] Improve readme docs
- [x] Show validation status on edit fields
- [x] Editable http headers 
- [x] Reintroduce outdated to build process
- [x] Reintroduce clippy to build process
- [x] Settings component

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## License

Distributed under the GNU General Public License v2.0. See LICENSE.txt for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Acknowledgments
