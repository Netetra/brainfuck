# BrainFuck
Rust製BrainFuckインタプリタ

# Todo
- 入力(`,`)に対応させる

# Example
```
$ RUST_LOG=info cargo run
   Compiling brainfuck v0.1.0 (/home/netetra/git/Netetra/brainfuck)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/brainfuck`
>> +++++++++[>++++++++>+++++++++++>+++>+<<<<-]>.>++.+++++++..+++.>+++++.<<+++++++++++++++.>.+++.------.--------.>+.>+.
[INFO] input: +++++++++[>++++++++>+++++++++++>+++>+<<<<-]>.>++.+++++++..+++.>+++++.<<+++++++++++++++.>.+++.------.--------.>+.>+.
[INFO] output: Hello World!

$
```

`RUST_LOG=trace`とすることでより詳細な情報を出力できます
