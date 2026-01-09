# pokedex-api

[![conventional-commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)
[![build-test](https://github.com/veeso/pokedex-api/actions/workflows/cargo.yml/badge.svg)](https://github.com/veeso/pokedex-api/actions/workflows/cargo.yml)

## Introduction

This project has been implemented as part of a technical interview process.
It is a RESTful API that provides very basic information about Pokémon.

It also exposes an endpoint to retrieve the pokemon data with the translated description.

The description is translated based on the following rules:

- If the pokemon is legendary or its `habitat` is `cave` or it's legendary, the description is translated to Yoda speak.
- For all other cases, the description is translated to Shakespearean English.

## Setup

### Setup with Docker

```bash

```

### Setup with Rust

```bash

```

## API Endpoints

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.