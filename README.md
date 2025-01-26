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
        <li><a href="#build">Build</a></li>        
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>

## About The Project

The goal of the project is to create an application where the following should be possible.
* Mock api responses.
* Route requests to other apis.
* +++

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Getting Started

### Prerequisites

The application is written in Rust.
- https://www.rust-lang.org/learn/get-started

The application requires at least Rust 1.84

Currently only Linux is supported, later Windows and Mac might get support.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Build

For debugging
```
cargo build
```

For release
```
cargo build --release
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Installation

The version build is found either in target/debug/apinae-daemon or target/release/apinae-daemon
Copy the file to /usr/bin

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
<!--
- [x] Add Changelog
- [x] Add back to top links
- [ ] Add Additional Templates w/ Examples
- [ ] Add "components" document to easily copy & paste sections of the readme
- [ ] Multi-language Support
    - [ ] Chinese
    - [ ] Spanish
-->
<p align="right">(<a href="#readme-top">back to top</a>)</p>

## License

Distributed under the GNU General Public License v2.0. See LICENSE.txt for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Acknowledgments
