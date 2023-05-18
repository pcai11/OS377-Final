# OS377-Final

# Bank Transactions

Bank Transactions allows you to simulate routine banking transactions such as deposit, withdraw, and transfer.

## Description

Allows users to input a ledger that contains a list of actions such as deposit, withdraw, and transfer. Each ledger is taken
by a thread to simulate multiple workers working with a single bank account. Mutex and Arc is used to protect data from
concurrent access and prevent race conditions. Each line of the ledger will work on the bank account while the bank
object keeps an ongoing list of accounts. Once every ledger is processed, the bank accounts are shown.

The project was originally implemented in C++. The project is now translated into Rust due to the language's 
superior memory management while still maintaining the same level of concurrency safety. In general,
Rust is much more safe and prevents undesirable behaviors.

## Usage

Video explanation and demonstration of code:
https://watch.screencastify.com/v/vJ2hAtMNPGHrVtJAbNSZ

## Installation

Must have rust environment

## Contact

Phillip Cai

<p align="right">(<a href="#readme-top">back to top</a>)</p>
