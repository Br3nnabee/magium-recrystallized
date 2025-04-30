# Magium Recrystallized

A community recreation & continuation of Magium, a popular choose-your-own-adventure (CYOA) game by author Cristian Mihailescu.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Introduction

Magium Recrystallized revives the highly loved narrative-driven experience, allowing players to create and explore meaningful decisions in a fantasy world. Driven by its passionate community, this version not only aims to faithfully recreate the original experience, but modernize its platform and continue the story past the 3 books originally written.

## Features

- **Engaging Narrative:** Experience deep, immersive storytelling with branching plotlines.
- **Community Created Content:** Regular updates made by the writing team continuing the story of Magium.
- **Cross-Platform:** Packaged via Tauri, supporting most desktop and mobile platforms with fast performance and low footprint.
- **Modern Stack:** Built using Svelte, Typescript, and WebAssembly for a responsive and lightweight experience.

## Installation

### Prerequisites

- Node.js v18+
- Rust 2024 (with `wasm32-unknown-unknown target`)
- Tauri CLI (`cargo install tauri-cli`)
- Wasm Pack (`cargo install wasm-pack`)
- Git

### Steps

```bash
# Clone the repository
git clone https://github.com/Br3nnabee/magium-recrystallized.git

# Navigate to project directory
cd magium-recrystallized

# Install Node dependencies
npm install

# Build the Rust/WASM backend
npm run tauri build
```

## Usage

To launch the development version of _Magium Recrystallized_:

```bash
# Start development server with Tauri
npm run tauri dev
```

To build a release version:

```bash
# Build production release
npm run tauri build
```

## Contributing

We welcome contributions! Here's how you can help:

1. Fork the repository.
2. Create your Feature Branch from Dev (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add AmazingFeature'`)
4. Push to the Branch (`git push origin feature/Amazingfeature`)
5. Open a Pull Request.

Please follow our coding guidelines and check for open issues before starting major changes.  
See the [CONTRIBUTING.md](CONTRIBUTING.md) file for detailed instructions and standards.

## License

Distributed under the AGPL 3.0 License. See [LICENSE](LICENSE) for more information.

## Acknowledgments

- Cristian Mihailescu – original creator of Magium
- Magium Recrystallized contributors – supporting and improving the app
- Writer Team - continuing the story of Magium
- [Magium-SDL](https://github.com/Colaboi2009/Magium-SDL) and [magium-dev](https://github.com/thuiop/magium-dev) - inspiration and reference for this implementation
