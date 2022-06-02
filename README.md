# PWFuzz-RS
A Rust-based password mutator for brute force attacks

## Disclaimer

This tool works, but was mainly an experiment. Please do not expect frequent updates to it.

## About

So you're brute-forcing a web app or other target. Sure you have a wordlist, maybe even a large one, but what about variations? Tossing `!` or year numbers on there? Adding random numbers? It would be great to have an easy way to mutate existing password lists to add variations. Hashcat has rule-based attacks, but what about for non-hash passwords?

That's what `pwfuzz-rs` is about. 

## Installation

You can of course build from source, but hey, it's a Rust tool, so you can also just `cargo install pwfuzz-rs`!

## Usage

```bash
pwfuzz-rs -w wordlist.txt -r rules.json [-i iterations]
```

`pwfuzz-rs` accepts the following arguments:

* `-w --wordlist`: Path to wordlist
* `-r --rules-file`: Path to JSON rules file 
* `-i --iterations`: Number of iterations to run mutations
* `-h --help`: Help

The output will be to stdout, but you can use any Unix tool you like to redirect it!

## Rules

Let's talk about Rules.

`pwfuzz-rs` supports the following rules:

* `Append [string]`: Append the given `string`
* `Prepend [string]`: Append the given `string`
* `Upper`: Uppercase the word
* `Lower`: Lowercase the word
* `Insert [string] [idx]`: Insert the given `string` and index `idx` (Skips on index failure)
* `AppendRandom [range]`: Append a random number from 0-`range`
* `Prepend [range]`: Append a random number from 0-`range`

## The Rules File

The Rules Files is a JSON file that expects an `author` key and a `rules` key. This example shows all rule variants.

```json
{
    "author": "Your Name <Your Email>",
    "rules": [
        {
            "Append": "!"
        },
        {
            "Prepend": "1"
        },
        "Upper", // no args means no object needed
        "Lower",
        {
            "Insert": ["%", 4] // inserts "%" at index 4
        },
        {
            "AppendRandom": 100
        },
        {
            "PrependRandom": 100
        }
    ]
}
```

Given the list:

```
letmein
iamgod
password
```

These rules produce:

```
letmein
iamgod
password
letmein!
iamgod!
password!
1letmein
1iamgod
1password
LETMEIN
IAMGOD
PASSWORD
letmein
iamgod
password
letm%ein
iamg%od
pass%word
letmein20
iamgod17
password79
18letmein
97iamgod
65password
```

## Iterations

But what if we want to apply rules on rules on rules on rules?

I got you, fam.

Passing `-i` allows you to iteratively apply rules to newly-generated mutations. So if we pass `-i 3` to the above list, we get 1500 unique passwords!
