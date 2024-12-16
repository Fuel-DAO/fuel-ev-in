# [FuelEV.in](https://fuelev.in) : Car Rental Platform

This project is a decentralized application (dApp) for renting cars on the Internet Computer (ICP).

## Prerequisites

To run this project locally, make sure you have the following installed:

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Trunk**: [Install Trunk](https://trunkrs.dev/)
- **DFX (Dfinity SDK)**: [Install DFX](https://internetcomputer.org/docs/current/developer-docs/setup/install/)

## Project Structure

### Canister Configuration (`dfx.json`)

```json
{
  "canisters": {
    "frontend": {
      "source": ["dist", "static"],
      "type": "assets"
    }
  }
}
```

# Running Project Locally

## Local Deployment

To deploy the canisters locally using the DFX (Dfinity SDK), run the following script:

```bash
dfx start --background
./scripts/build.sh
./scripts/local_deploy.sh
```
