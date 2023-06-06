
<div align="center">

  <img src="assets/aster_logo.png" alt="logo" width="300" height="auto" />
  <h1>Aster</h1>
  
  <p>
    A seamless and customizable metrics collector.
  </p>
  
  
<!-- Badges -->
<p>
  <a href="https://github.com/aster-do/aster/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/aster-do/aster" alt="contributors" />
  </a>
  <a href="https://github.com/aster-do/aster/commits/main">
    <img src="https://img.shields.io/github/last-commit/aster-do/aster" alt="last update" />
  </a>
  <a href="https://github.com/aster-do/aster/network/members">
    <img src="https://img.shields.io/github/forks/aster-do/aster" alt="forks" />
  </a>
  <a href="https://github.com/Louis3797/aster-do/aster">
    <img src="https://img.shields.io/github/stars/aster-do/aster" alt="stars" />
  </a>
  <a href="https://github.com/aster-do/aster/issues/">
    <img src="https://img.shields.io/github/issues/aster-do/aster" alt="open issues" />
  </a>
  <a href="https://github.com/aster-do/aster/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/aster-do/aster.svg" alt="license" />
  </a>
</p>
   
<h4>
    <a href="https://github.com/aster-do/aster">Documentation</a>
  <span> · </span>
    <a href="https://github.com/aster-do/aster/issues/">Report Bug</a>
  <span> · </span>
    <a href="https://github.com/aster-do/aster/issues/">Request Feature</a>
  </h4>
</div>

<br />

<!-- Table of Contents -->
# :notebook_with_decorative_cover: Table of Contents

- [:notebook\_with\_decorative\_cover: Table of Contents](#notebook_with_decorative_cover-table-of-contents)
  - [:star2: About the Project](#star2-about-the-project)
    - [:dart: Features](#dart-features)
  - [:building\_construction: Architecture](docs/architecture.md)
  - [:toolbox: Getting Started](#toolbox-getting-started)
    - [:bangbang: Prerequisites](#bangbang-prerequisites)
    - [:test\_tube: Running Tests](#test_tube-running-tests)
    - [:running: Run Locally](#running-run-locally)
    - [:rocket: Build](#rocket-build)
  - [:wave: Contributing](#wave-contributing)
  - [:warning: License](#warning-license)
  - [:handshake: Authors](#handshake-authors)

  

<!-- About the Project -->
## :star2: About the Project

Aster is designed to efficiently collect metrics from a REST API, transform them into chargeable data, expose the API and SDKs, create invoices and bills, and generate alerts based on specified rules.


<!-- Features -->
### :dart: Features

- Collect metrics from a REST API
- Transform metrics into chargeable
- Expose the API and SDKs
- Create invoices and bills
- Alerts based on rules


<!-- Getting Started -->
## 	:toolbox: Getting Started

<!-- Prerequisites -->
### :bangbang: Prerequisites

This project uses Cargo as RUST package manager

```bash
curl https://sh.rustup.rs -sSf | sh
```

<!-- Running Tests -->
### :test_tube: Running Tests

To run tests, run the following command

```bash
  cargo test --workspace --bins --lib
```

<!-- Run Locally -->
### :running: Run Locally

Clone the project

```bash
  git clone https://github.com/aster-do/aster.git
```

Go to the project directory

```bash
  cd aster
```

Run the application

```bash
  cargo run
```

<!-- Build -->
### :rocket: Build

Build Aster with Cargo

```bash
  cargo build
```


### debug async with tokio console

see [tokio-console](https://github.com/tokio-rs/console)

```sh
./debug-aster.sh
```

then open the tokio console
```sh
tokio-console http://localhost:5555
```


<!-- Contributing -->
## :wave: Contributing

<a href="https://github.com/aster-do/aster/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=aster-do/aster" />
</a>


Contributions are always welcome!

See `CONTRIBUTING.md` for ways to get started.



<!-- License -->
## :warning: License

Distributed under the TODO License. See `LICENSE.md` for more information.


<!-- Authors -->
## :handshake: Authors

This project was developed by the Polytech DO teams in June 2023. 

Project Link: [https://github.com/aster-do/aster](https://github.com/aster-do/aster)


