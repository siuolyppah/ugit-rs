# ugit-rs: DIY Git in Rust

Only support [a useful minimum set of commands in `git`](https://github.com/git/git/blob/master/Documentation/giteveryday.txt).

> this repo was inspired by [`ugit`](https://www.leshenko.net/p/ugit/#)


# Workflow

> install: `cargo install --path .`

## cmd `hash-object` and `cat-file`

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

## cmd `write-tree` and `read-tree`

```shell
$ cd /tmp/new

$ ugit-rs init
Initialized empty ugit repository in /tmp/new/.ugit-rs

$ mkdir f1 && mkdir f2

$ echo content1 > f1/a.txt && echo abc > f1/b.txt && echo hhh > f2/a.txt 

$ ugit-rs write-tree
8f87a4d4ce075e840b89cbd52e030f84e1345b1c

$ ugit-rs read-tree  8f87a4d4ce075e840b89cbd52e030f84e1345b1c
......
```

# TODO

## cmd

- [x] `init`
- [x] `add`
- [x] `hash-object`
- [x] `cat-file`
- [x] `write-tree`
- [x] `read-tree`

## feature

- [x] add hash-object type in cmd `hash-object` and `cat-file` .  
- [ ] support glob
- [ ] parse `.ignore` file

## fix

- [ ] write-tree should keep **origin file path after root**.
- [ ] exclude `.ugit-rs` dir.
