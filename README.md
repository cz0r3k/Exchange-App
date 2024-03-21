# Exchange app

App converts amounts between different currencies using real-time exchange rate data fetched from an API.

## Usage

```
Usage: exchangeapp [OPTIONS] [COMMAND]

Commands:
  exchange         Exchange currency to another
  list-currencies  List all available currencies
  latest           List currencies with exchange rate
  help             Print this message or the help of the given subcommand(s)

Options:
      --show-connectors                Show all available connectors
      --set-connector <SET_CONNECTOR>  Set current connector [possible values: mock, currencybeacon]
      --set-apikey <SET_APIKEY>        Set api key for chosen connector
  -h, --help                           Print help
```

### exchange

```
Exchange currency to another

Usage: exchangeapp.exe exchange --source <SOURCE> --target <TARGET> <AMOUNT>

Arguments:
  <AMOUNT>  Amount to be converted

Options:
  -s, --source <SOURCE>  Source currency code
  -t, --target <TARGET>  Target currency code
  -h, --help             Print help
```

### list-currencies

```
List all available currencies

Usage: exchangeapp.exe list-currencies

Options:
  -h, --help  Print help
```

### latest

```
List currencies with exchange rate

Usage: exchangeapp.exe latest [OPTIONS] --base <BASE>

Options:
  -b, --base <BASE>      Base currency code
  -t, --target <TARGET>  Target currency code
  -h, --help             Print help
```

## .env

Rename `.env.copy` file to `.env`

## API

Application uses the api from `currencybeacon.com`

### API key

To pass API key to aplication, enter it to `.env` file after `EAPP_CURRENCYBEACON=`, or you can pass it to CLI with argument `--set-apikey  <SET_APIKEY>`

## Building

Nightly toolchain is require to build this app

```
cargo +nightly build --release 
```

## Building docker

```
docker compose build
```

## Running 

```
cargo run --release -- [OPTIONS] [COMMAND]
```


## Running docker

To run this app in docker, start container detached and attach to the the container

`.env` file is required on local machine

```
docker compose pull
docker compose up -d
docker attach CONTAINER
```


