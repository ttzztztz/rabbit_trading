# Interactive Brokers (IBKR) Client Portal API Rust Implementation

## Disclaimer

This is an _Unofficial_ API Client implement for Interactive brokers (IBKR) Client Portal. Please use it at your own risk!

## Documents

- [doc.rs - ibkr_client_portal (latest)](https://docs.rs/ibkr_client_portal/latest/ibkr_client_portal)

## Tutorial

- Visit our [unit test cases](https://github.com/ttzztztz/rabbit_trading/tree/master/packages/ibkr_client_portal/src/test) to have a quick _tutorial_ of how to use this lib (really straightforward to use)

## IBKR Client Portal Docker image Usage

There's a self written (unofficial) docker image for ibkr client portal. [github.com/ttzztztz/rabbit_trading dockerfiles/ibkr_client_portal](https://github.com/ttzztztz/rabbit_trading/tree/master/dockerfiles/ibkr_client_portal)

- First, clone the repo and modify the configuration YAML file according to your needs

```bash
git clone git@github.com:ttzztztz/rabbit_trading.git
cd  dockerfiles/ibkr_client_portal
vim conf.yaml
```

- Then, start the docker-compose, waiting for the image build and container startup

```bash
docker-compose up -d
```

- Finally, visit the `http://localhost:5000` to finish the auth

## References

- This lib is built based on the official documents below
  - https://ibkrcampus.com/ibkr-api-page/cpapi-v1/
  - https://interactivebrokers.github.io/cpwebapi/

## Contributions

- Submit Pull Requests for the repo [Github ttzztztz/rabbit_trading](https://github.com/ttzztztz/rabbit_trading)
