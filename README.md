# 🏣 kagi

A simple embedded key-value store written in rust as a learning project!

## Getting Started

To install, simply add `kagi` to your `Cargo.toml`, for example

```
[dependencies]
kagi = "0.1.1"
```

## Usage

Open a store and store a value to it.
```
use kagi::open;

fn main() {
    // open db
    let mut data = open("./test.kg");

    // insert value
    data.insert("test", "value");

    // try load
    let result = data.get("test");
    println!("{}", result);

    // sync to db
    data.save()
}
```


To further learn how to use kagi, take a look at the examples.

## Details

When opening a file, kagi will create it if it doesn't exist, and it will sync to file every 500ms by default.
