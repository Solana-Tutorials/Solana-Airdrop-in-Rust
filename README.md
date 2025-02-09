# Solana Airdrop Example

This repository contains the code used in this [Solana Tutorials YouTube video](https://www.youtube.com/watch?v=rESFrIuGo2E&ab_channel=SolanaTutorials). It demonstrates how to request a SOL airdrop in Rust using `solana-test-validator` (local validator)

## Prerequisites

- Follow the instructions to install the [Solana CLI](https://solana.com/docs/intro/installation).

## Running the Example

1. **Start a Local Validator**

   Run the following command to start a local Solana validator:

   ```bash
   solana-test-validator
   ```

2. **Run the Example Code**

   With the validator running, execute:

   ```bash
   cargo run
   ```

When you run the example, you'll see output similar to the following (note that your address and transaction signature will be different):

```
Address: 9kPPamvMcc3PXQMo5RVbT38gA52NJMk3DBiMmVWzQx3t

Transaction Signature: 4rYUtse64w3xTWDAa8mkqWZP1mXWnMAM8JFd6uzT68GdP7xSy7c2y5kLQMn6npjXpqxBZHvfy96kE838fcREcxnR

Account {
    lamports: 1000000000,
    data.len: 0,
    owner: 11111111111111111111111111111111,
    executable: false,
    rent_epoch: 18446744073709551615,
}
```
