# ugit-rs: DIY Git in Rust

Only support [a useful minimum set of commands in `git`](https://github.com/git/git/blob/master/Documentation/giteveryday.txt).

> this repo was inspired by [`ugit`](https://www.leshenko.net/p/ugit/#)


# Workflow

> install: `cargo install --path .`

```shell
$ cd /tmp/new

$ ugit-rs init
Initialized empty ugit repository in /tmp/new/.ugit-rs

$ echo some file > bla

$ ugit-rs hash-object bla
0e08b5e8c10abc3e455b75286ba4a1fbd56e18a5

$ ugit-rs cat-file 0e08b5e8c10abc3e455b75286ba4a1fbd56e18a5
some file
```

# TODO

## cmd

- [x] `init`
- [x] `add`
- [x] `hash-object`
- [x] `cat-file`

## feature

- [x] add hash-object type in cmd `hash-object` and `cat-file` .  
- [ ] support glob
- [ ] parse `.ignore` file
