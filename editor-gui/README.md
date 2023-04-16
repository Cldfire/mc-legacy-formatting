# editor-gui

A small Rust GUI program built on top of [egui](https://github.com/emilk/egui), providing a native and web editor experience for `mc-legacy-formatting`.

![screenshot of the editor-gui running natively and in a web browser side-by-side](./screenshot.png)

[egui_template](https://github.com/emilk/egui_template/) was used as the base for the contents of this folder.

## Usage

Enter text using legacy formatting codes (prefixed by `&`) in the text input on the left-hand side. Instantly see the formatted output reflected on the right-hand side!

You can [try it online](https://cldfire.github.io/mc-legacy-formatting/) or see the below instructions for building it locally.

## Building Locally

First, see [egui's README](https://github.com/emilk/egui#demo) and follow their setup instructions if applicable to your platform.

### Building for the Web

To build `editor-gui` for the web:

* `cargo install trunk --locked`
* From the `editor-gui` folder:
  * `trunk serve`
  * The editor will open in your web browser

### Building Natively

Run `cargo run -p editor-gui --release`.
