// crates

extern crate chrono;
extern crate clap;
extern crate csv;

// 3rd-party imports

use clap::{App, Arg};
use chrono::NaiveDate;

// types

enum NumOfTransactions {
    All,
    Some(u64),
}

struct Transaction {
    date: NaiveDate,
    description: String,
    amount: String,
}

// app

fn main() {
    // cli args

    let matches = App::new("tangerine-transactions-to-ledger")
        .version("0.1.0")
        .author("Alberto Leal <mailforalberto@gmail.com>")
        .about("Convert Tangerine Bank transactions to ledger-cli compatible transactions")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input CSV file to use")
                .required(true),
        )
        .arg(
            Arg::with_name("account_name")
                .short("a")
                .long("account")
                .help("Sets account for each transaction")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("first_n")
                .short("n")
                .long("num")
                .help("Only show last N transactions.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("has_headers")
                .short("h")
                .long("has_headers")
                .help("Assume CSV file has headers."),
        )
        .get_matches();

    let path_to_file = matches.value_of("INPUT").unwrap();

    let account_name = matches
        .value_of("account_name")
        .unwrap_or("account name here");

    let num_of_transactions = match matches.value_of("first_n") {
        None => NumOfTransactions::All,
        Some(raw_input) => match raw_input.parse::<u64>() {
            Err(_) => NumOfTransactions::All,
            Ok(n) => {
                if n > 0 {
                    NumOfTransactions::Some(n)
                } else {
                    NumOfTransactions::All
                }
            }
        },
    };

    // csv reader

    let mut reader = match csv::ReaderBuilder::new()
        .has_headers(matches.is_present("has_headers"))
        .from_path(path_to_file)
    {
        Ok(reader) => reader,
        Err(err_reason) => {
            println!("{}", err_reason);
            std::process::exit(1);
        }
    };

    let mut transactions = vec![];

    for result in reader.records() {
        let record = match result {
            Ok(record) => record,
            Err(err_reason) => {
                println!("{}", err_reason);
                std::process::exit(1);
            }
        };

        // Tangerine Bank exports transactions as follows
        // CSV format: date, ???, transaction description, ???, amount

        let date = record.get(0).unwrap().trim().to_string();
        let description = record.get(2).unwrap().trim().to_string();
        let amount = record.get(4).unwrap().trim().to_string();

        // formatting

        let date = NaiveDate::parse_from_str(&date, "%m/%d/%Y").unwrap();

        transactions.push(Transaction {
            date,
            description: description.to_string(),
            amount: amount.to_string(),
        });
    }

    let transactions = transactions;

    let range = match num_of_transactions {
        NumOfTransactions::All => 0..,
        NumOfTransactions::Some(n) => (transactions.len() - (n as usize))..,
    };

    for transaction in transactions.get(range).unwrap() {
        // ledger-cli journal format: https://www.ledger-cli.org/3.0/doc/ledger3.html#Journal-Format

        println!(
            r#"
{} * {}
    {:76}{} CAD
    ???
"#,
            transaction.date.format("%Y-%m-%d"),
            transaction.description,
            account_name,
            transaction.amount
        );
    }
}
