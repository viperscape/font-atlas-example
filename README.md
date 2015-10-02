## Steps ##

Clone repo and build the atlas generator
Note: this requires Freetype to be setup for your machine

```bash
git clone https://github.com/viperscape/font-atlas-example
cd font-atlas-example
cd atlas-gen
cargo build
```

Build an atlas from a font
Note: here we choose a font and font-size of 20
This example generator was sourced from [TyOverby's example](https://github.com/TyOverby/font-cache/blob/master/freetype-atlas/src/atlas_generator.rs)

```bash
cd ..
./atlas-gen/target/debug/atlas_gen assets/fonts/SourceCodePro-Regular.otf 20

ls assets/fonts/
SourceCodePro-Regular-20.json  SourceCodePro-Regular-20.png  SourceCodePro-Regular.otf
```

You'll see that there is a png and bincode file representing the font in atlas format.
Let's move on to building the render example (in the root of the project folder) to show our HelloWorld example on screen.

```bash
cargo run
```