# Safe Cow Utility

## Disclaimer

**🚨No warranty is provided on this utility. This utility only accepts terminal input of private keys for signing transactions. Use at your own risk.🚨**

## Overview

This command line tool provides the ability for a Safe to transact gaslessly with Cowswap 🐮, and includes the following features:

1. Token lists for Ethereum mainnet, Gnosis Chain, and Goerli.
2. Custom tokens with meta data looked up directly on-chain.
3. Signature collection for threshold number of Safe owners (via direct private key entry).
4. Sending of ERC20 approval when there is insufficent allowance for the `GPv2VaultRelayer` contract.
5. Submission of the EIP-1271 signed order to the Cowswap API.

## How-to

In order to use this tool, you must have `rust` installed. Use [`rustup`](https://www.rust-lang.org/tools/install) to install `rust`.

Once you have `rust` installed, you will need to clone this repository:

```bash
git clone https://github.com/rndlabs/safe-cow.git
```

Now, to create an order, you, run:

```bash
cargo run -- --rpc-url <RPC_URL> --safe <SAFE_ADDRESS> create-order
```

It is important that the `RPC_URL` that is specified be for either Ethereum Mainnet, Gnosis Chain, or Goerli, as these are the only chains that Cowswap currently supports. At this point, follow the on-screen prompts in order to submit a swap.

**WARNING**: Orders that are submitted as **limit orders**. Orders by default have a **20 min timeout**. **No quotes are provided by the Cowswap API, and users are required to enter the token swap values manually**.

Here's an example of the above:

[![asciicast](https://asciinema.org/a/c5M9esvutJJGBJVogk2gG0CoA.svg)](https://asciinema.org/a/c5M9esvutJJGBJVogk2gG0CoA)

You may review the results of the above example swap on:

* [Cowswap Explorer](https://barn.explorer.cow.fi/goerli/orders/0x41ba0226ceb6f13763b79aea6577d73681092373369d22fa0106d05340be891cdc8c452d81dc5e26a1a73999d84f2885e04e9ac363a06074)
* [Etherscan](https://goerli.etherscan.io/tx/0x6ed7991ada45e2bfccd36498288ada1087bddbe9fde9358038c36a54e2e1436a)

Note that the order cross the orderbook significantly, however due to the solver competition, the surplus is preserved and received by the trader. This **should** be the case, and **should** be enforced by the solver competition parameters. **However, the surplus may be subject to adverse market forces during times of high volatility**. users should set pricing spreads tightly to avoid adverse slippage.

## Todo

- [ ] Order cancellations (*EIP-1271 cancellations not currently supported by the Cowswap API).