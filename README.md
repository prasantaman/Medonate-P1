# Medonate Soroban Contract

Medonate is a Soroban smart contract designed to manage accounts and facilitate donations. This contract allows users to add new accounts, donate to existing accounts, and check the balance of accounts.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
  - [Add New Account](#add-new-account)
  - [Donate Amount](#donate-amount)
  - [Check Balance](#check-balance)
- [Contributing](#contributing)
- [License](#license)

## Installation

To use this contract, you need to have the Rust toolchain installed. You can install Rust from [here](https://www.rust-lang.org/tools/install).

Then, add the Soroban SDK to your project by including the following in your `Cargo.toml`:

```toml
[dependencies]
soroban-sdk = "0.0.1" # Check for the latest version

pub fn add_new_account(env: Env, address: Address, amount: u32) {
    let party = Account {
        address: address.clone(),
        amount,
    };
    env.storage().persistent().set(&address, &party);
}
pub fn donate_amount(env: Env, address: Address, amount: u32) {
    if let Some(mut party) = env.storage().persistent().get::<_, Account>(&address) {
        party.amount += amount;
        env.storage().persistent().set(&address, &party);
    } else {
        panic!("Not found");
    }
}
pub fn checkbalance(env: Env, address: Address) -> Option<u32> {
    if let Some(party) = env.storage().persistent().get::<_, Account>(&address) {
        Some(party.amount)
    } else {
        panic!("Account Not Found!");
    }
}

Save the above content in a file named `README.md` in your project directory.
