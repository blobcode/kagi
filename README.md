# 🏣 kagi

A simple embedded key-value store written in rust as a learning project!

## Getting Started <a name = "getting_started"></a>

To install, simply add `kagi` to your `Cargo.toml`, for example

```
[dependencies]
kagi = "0.1.0"
```

## Usage <a name = "usage"></a>

Open a store and store a value to it.
```
use kagi::open;

fn main() {
    let mut data = open("./test.kg");
    data.insert("test", "value");

    // try load
    let result = data.get("test");
    println!("{}", result);
}
```


To further learn how to use kagi, take a look at the examples.
