tangerine-transactions-to-ledger
=========================

> Convert Tangerine Bank transactions to ledger-cli compatible transactions

Usage
=====

```
tangerine-transactions-to-ledger 0.1.0
Alberto Leal <mailforalberto@gmail.com>
Convert Tangerine Bank transactions to ledger-cli compatible transactions

USAGE:
    tangerine-transactions-to-ledger [FLAGS] [OPTIONS] <INPUT>

FLAGS:
    -h               Assume CSV file has headers.
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --account <account_name>    Sets account for each transaction
    -n, --num <first_n>             Only show last N transactions.

ARGS:
    <INPUT>    Sets the input CSV file to use

```

1. Export transactions from Tangerine Bank into csv format.

2. `tangerine-transactions-to-ledger accountactivity.csv -n 10 | pbcopy`

3. Paste transactions into your ledger-cli file.

License
=======

MIT.
