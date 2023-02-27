<div align="center">
  <div align="center">
    <img src="https://raw.githubusercontent.com/davidmhewitt/elementary-rust-example/main/data/icons/hicolor/128.svg" width="64">
  </div>
  <h1 align="center">Elementary Rust Example</h1>
  <div align="center">Rust application template for elementary OS. </div>
</div>

<div align="center">
    <img  src="https://github.com/davidmhewitt/elementary-rust-example/raw/main/data/screenshots/light.png" alt="Screenshot" width="400">
    <img  src="https://github.com/davidmhewitt/elementary-rust-example/raw/main/data/screenshots/dark.png" alt="Screenshot Dark" width="400">
</div>

## ❓ What is this?

This repository has all of the necessary boilerplate to build a GTK4 application against the elementary Flatpak Platform using the Granite 7 library.

The example code uses the `Granite.Placeholder` widget to show a welcome screen, supports the user's dark style preference, remembers the window size and state, and sets up translations.

You are free to extend upon this template in any way you see fit.

## 🛠️ Requirements

* meson
* libgranite-7-dev
* libgtk-4-dev
* rust >= 1.64 (Install from [rustup](https://rustup.rs/) as the version available on elementary OS 7.0 is not new enough)

## 🚀 Getting started

```
git clone https://github.com/davidmhewitt/elementary-rust-sample
cd elementary-rust-sample
meson setup build
cd build
ninja
ninja install
src/elementary-rust-example
```

To regenerate translation files (run inside the build directory):
```
ninja elementary-rust-example-pot
ninja elementary-rust-example-update-po
ninja extra-pot
ninja extra-update-po
```

## 🤖 GitHub Actions

This repository is set up with a number of actions to help the development process.

* Dependabot is configured to update Rust dependencies in your Cargo files.
* There is an action to test building a Flatpak package against the elementary runtime.
* There is an action to test building the application outside of a Flatpak in an elementary Docker container.
* Finally, there is action to update `cargo-sources.json` when your Cargo manifests change. This file lists the Cargo crates that `flatpak-builder` has to download before attempting an offline build in a container.

## 🐞 Problems/Feature Requests?

Please open [an issue](https://github.com/davidmhewitt/elementary-rust-example/issues) if you find any problems or want to get support.

