# SAFE Cowswap Utility

This command line tool enables users of Safe (formerly known as Gnosis Safe) to
create an order for Cowswap, *off-chain*, assembling the required signatures
from the Safe owners, and submitting the order to the Cowswap API.

## How-to

```bash
cargo run safe-cow --rpc-url <RPC URL> --safe <SAFE ADDRESS>
```

This will then prompt for:
1. Is this a buy order or a swap order to place?
2. If a buy order, which token do you want to buy, how much, and which token do you want to sell, and how much?
3. If a sell order, which token do you want to sell, how much, and which token do you want to buy, and how much?

The above can be pre-populated with a quote after the selection of the first token, how much, and the second token.

## Developer notes

### General

1. Check for connection to the RPC-URL, and determine the `chain_id`.
2. Check to make sure `safe` is valid, and fetch the amount of required signatures.

### Create order

1. Prompt the user for buy or sell order.
2. Prompt the user for token inputs.
3. Prompt the user for order timeout.
4. Prompt for signatures for the safe.
5. Send the order to the API, print out the explorer URL and the UID.

### Cancel Order

1. Prompt the user for the order UID.
2. Prompt for signatures for the safe.
3. Send the cancellation order to the API, print out the explorer URL and the UID.


This is a best effort cancellation, and might not prevent solvers from settling the orders (if the order is part of an in-flight settlement transaction for example). Authentication must be provided by an EIP-712 signature of an "OrderCacellations(bytes[] orderUids)" message.
Parameters

No parameters
Request body

Signed OrderCancellations

{
  "orderUids": [
    "string"
  ],
  "signature": "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
  "signingScheme": "eip712"
}