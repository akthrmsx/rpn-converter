# RPN converter

This is a converter that converts infix notation to reverse polish notation (RPN).

## Supported characters

* `[a-zA-Z]`
* `+`
* `-`
* `*`
* `/`
* `(`
* `)`

## Examples

```sh
$ cargo run
>> a + b
a b +
>> (a + b) * (c / (d - e))
a b + c d e - / *
>> a @ b
invalid char is found: @
```
