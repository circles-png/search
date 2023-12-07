# search

rust program to search through swift's colorsets and extract colours

example:

```json
{
  "colors" : [
    {
      "color" : {
        "color-space" : "srgb",
        "components" : {
          "alpha" : "1.000",
          "blue" : "0xC1",
          "green" : "0x74",
          "red" : "0x57"
        }
      },
      "idiom" : "universal"
    }
  ],
  "info" : {
    "author" : "xcode",
    "version" : 1
  }
}
```

becomes

```rs
...
    "#5774C1",
...
```

## usage

place colorsets you wanna search in `./stuff`, next to `./src`, `Cargo.toml`, and family

like

```rs
 stuff
├──  .DS_Store // you don't have to have this
├──  ReallyCoolBlue.colorset
│  └──  Contents.json
├──  ReallyCoolRed.colorset
│  └──  Contents.json
└──  ReallyCoolSomethingElse.colorset
   └──  Contents.json
```

_ascii art courtesy of [`exa`](https://github.com/ogham/exa)!_

now `cargo run` and look at output (on stderr because it gets `dbg!`'d out)
