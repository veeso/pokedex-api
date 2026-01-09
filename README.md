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

The project comes with a Dockerfile for easy setup and deployment.

```bash
docker build -t pokedex-api .
docker run --name pokedex-api -p 127.0.0.1:5000:5000 pokedex-api
```

### Setup with Rust

Install Rust and Cargo by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

The build the project and run the server locally:

```bash
cargo run --release
```

The server takes the following arguments:

```txt
Options:
  --adapter-timeout-secs
                    adapter timeout in seconds [default: 30]
  -b, --bind-address
                    bind address for the web server [default: 127.0.0.1]
  --funtranslation-endpoint
                    fun translation custom endpoint URL [default:
                    https://api.funtranslations.com/]
  -L, --log-file    the path to the log file [default: log only to stdout]
  --log-filter      apply log filtering to target matching the given string
                    [default: no filtering]
  -l, --log-level   the log level [default: info]
  --pokeapi-endpoint
                    pokeapi custom endpoint URL [default:
                    https://pokeapi.co/api/v2/]
  -p, --port        the port the web server will listen on [default: 5000]
  --help, help      display usage information
```

## API Endpoints

### Get Pokemon Information

GET `/pokemon/{name}`

Returns basic information about the specified Pokémon.

```json
{
  "name": "miltank",
  "description": "MILTANK gives over five gallons of milk on a daily basis. Its sweet milk is enjoyed by children and grown-ups alike. People who can’t drink milk turn it into yogurt and eat it instead.",
  "habitat": "grassland",
  "isLegendary": false
}
```

Where

- `name`: The name of the Pokémon.
- `description`: A brief description of the Pokémon.
- `habitat`: The habitat where the Pokémon can be found. Optional: it is provided only for Pokémon that belong to 1st to
  3rd generation due to being a thing of 3rd gen games.
- `isLegendary`: A boolean indicating whether the Pokémon is legendary.

### Get translated Pokemon Information

GET `/pokemon/translated/{name}`

Returns the Pokémon information with the description translated according to the rules specified above.

```json
{
  "name": "miltank",
  "description": "...",
  "habitat": "grassland",
  "isLegendary": false
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.