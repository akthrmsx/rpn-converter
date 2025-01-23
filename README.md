# 中置記法 -> 後置記法

## 例

```sh
$ cargo run
>> a + b
a b +
>> (a + b) * (c / (d - e))
a b + c d e - / *
>> a @ b
invalid char is found: @
```

## 使用可能文字

* 半角英字（`a-zA-Z`）
* `+`
* `-`
* `*`
* `/`
* `(`
* `)`
