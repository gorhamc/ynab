# What is YNAB?

You Need A Budget ([YNAB](https://www.youneedabudget.com/)) is a web based implementation of envelope budgeting, or sometimes called zero-base budgeting.

# What is this?

This repo is a Rust implementation of a ynab client. The goal of this project is to make it simple to interact with the ynab api's.

ynab_lib defines structs that represent the JSON body of a YNAB API response and methods that wrap the official ynab [REST API's](https://api.youneedabudget.com/v1#/) endpoints.

cli for now is simply an easy way for me to execute and test code. I have plans for it to be a lot more valuable in the future but for now it doesn't really do anything.

This is a small project primarily to help me learn Rust. 

PR's are welcome from anyone.

# Getting Started

Before using ynab_lib, you will need to first obtain a personal access token from your ynab account. Follow the steps [here](https://api.youneedabudget.com/#personal-access-tokens). Also note that ynab does rate limit requests, defined [here](https://api.youneedabudget.com/#rate-limiting).

Right now all GET methods are implemented on the Client type.
