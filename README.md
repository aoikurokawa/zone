# Zone

Using binary option models, create a prediction market for Solana meme coin prices. Users should be able to predict if a meme coin price will end higher or lower within a specified timeframe.

## Test

### Set env variable `ANCHOR_WALLET`

Bash:

```bash
export ANCHOR_WALLET="/home/{username}/.config/solana/id.json"
```

Fish:

```fish
set -x ANCHOR_WALLET "/home/{username}/.config/solana/id.json"
```

### Build the programs

```bash
anchor build
```

### Run localnet

```bash
anchor localnet
```

### Run all test

```bash
cargo test
```

## Run app

### Show all commands

```bash
cargo r -- --help

# Usage: client <COMMAND>
# 
# Commands:
#   initialize         Initialize the vault
#   initialize-market  Initialize the market
#   start-market       Start the market
#   create-prediction  Bet YES or NO
#   help               Print this message or the help of the given subcommand(s)
# 
# Options:
#   -h, --help     Print help
#   -V, --version  Print version
```

### Initialize the vault

Pass the amount(SOL) to put in the vault

```bash
cargo r -- initialize 5
```

### Initialize the market

Pass the arguments

- token address: 'DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263'(BONK)
- payout multiplier: 200(ex.)


```bash
cargo r -- initialize-market '3S8qX1MsMqRbiwKg2cQyx7nis1oHMgaCuc9c4VfvVdPN' 200
```

### Start the market

Pass the arguments

- token address: 'DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263'(BONK)
- end: '2024-06-13 14:00:00'

```bash
cargo r -- start-market '3S8qX1MsMqRbiwKg2cQyx7nis1oHMgaCuc9c4VfvVdPN'  '2024-06-13 14:00:00'
```

### Predict higher or lower

Pass the argument

- token address: 'DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263'(BONK)
- bet: if you think it would be higher, then 1, otherwise 0
- amount: how much you bet for prediction
- current price: current price of token

```bash
cargo r -- bet '3S8qX1MsMqRbiwKg2cQyx7nis1oHMgaCuc9c4VfvVdPN' 1 1 100
```

## Resources
- [Programming on Solana - An Introduction](https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/)
- [SOL dev](https://www.soldev.app/)
- https://beta.solpg.io/tutorials/tiny-adventure-two
