# flashcard

[Flashcard](https://en.wikipedia.org/wiki/Flashcard) allows you to create your
flashcards and exercize your memory. Its main usage is to help you learn a new
language at your own pace by building your deck of flashcards on the themes you
care about.

Flashcard allows you to create your cards, load them on your phone and start
practicing.

The user experience should be as follows:

1. import cards, all of which are tagged by a theme
2. select tags of cards you want to practice
3. Practice. Tap to show the answer, swipe right if you answered it right and
left if you answered it wrong.
4. You are given the choice of practicing the cards you got wrong or the whole
deck.

The project should use Bevy to target both web and mobile platforms. As of
today, the support on those platforms are being worked on:

* [Web](https://github.com/bevyengine/bevy/issues/88)
* [iOS](https://github.com/bevyengine/bevy/issues/87)
* [Android](https://github.com/bevyengine/bevy/issues/86)

## Build and run

The app is compiled to webassembly and ran in the browser.

### Prerequisite

```bash
# install wasm-bindgen cli: https://rustwasm.github.io/wasm-bindgen/reference/cli.html
cargo install -f wasm-bindgen-cli
```

### Build

```bash
# .cargo/config.toml is configured to compile to webassembly
cargo build --release
wasm-bindgen \
--out-name flashcard \
--out-dir target \
--target web target/wasm32-unknown-unknown/release/flashcard.wasm
```

### Run

```bash
# cargo install basic-http-server
basic-http-server examples/wasm

# with python
python3 -m http.server --directory examples/wasm

# with ruby
ruby -run -ehttpd examples/wasm
```
