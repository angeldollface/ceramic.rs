# CERAMIC.RS :lock: :rocket: :crab:

***My Rustacean implementation of my IMEI checker, Ceramic. :lock: :rocket: :crab:***

![GitHub CI](https://github.com/angeldollface/ceramic.rs/actions/workflows/yew.yml/badge.svg)

## ABOUT :books:

Some time ago I was set a challenge for writing code that validates IMEI numbers in my CS class in university. Because I thought the problem was interesting, I grappled with it and wrote a small app in Vue.js that does exactly this. You can find the app's repository in the section below. This repository contains the source code for my re-implementation of this app in Yew and Rust. :heart:

## LINKS :earth_americas:

- Library implementation in ECMA Script: [VIEW](https://github.com/angeldollface/luhny)
- Library implementation in Rust: [VIEW](https://github.com/angeldollface/luhny.rs)
- Web app implementation in Vue.js:[VIEW](https://github.com/angeldollface/ceramic)

## DEPLOYED PROJECT ON GITHUB PAGES :rocket:

To view a live deployed version of this project, click here: [VIEW](https://angeldollface.art/ceramic.rs)

## USAGE :hammer:

- 1.) Visit [this link](https://angeldollface.art/ceramic.rs).
- 2.) Get the IMEI for your device from the instructions for your platform. (You might have to scroll a bit.)
- 3.) Copy this IMEI number or write it down.
- 4.) Put the number into the input field.
- 5.) Click "VALIDATE"!
- 6.) Receive the status of your IMEI number from the text below the "VALIDATE" button that says `IMEI valid: Type something!`.

Note: You can use this fake IMEI for testing: `356728113476259`

## TRY THE CODE FOR YOURSELF! :inbox_tray:

You should have the following tools installed and available from the command line:

- Rust
- Git

To try *Ceramic.rs* on your own machine, follow these steps:

- 1.) Install `trunk` from [crates.io](https://crates.io/crates/trunk):

```bash
cargo install trunk
```

- 2.) Clone this project's source code:

```bash
git clone https://github.com/angeldollface/ceramic.rs.git
```

- 3.) Change directory into the source code's root directory:

```bash
cd ceramic.rs
```

- 4.) Serve the app locally (This will serve the app locally on [`http://127.0.0.1:8080/ceramic.rs/`](http://127.0.0.1:8080/ceramic.rs/).):

```bash
trunk --config ./trunk.toml serve --release
```

- 5.) If you want to build the app into a bundle to deploy to a server, run the command below. This will produce a directory called `dist` with the bundle inside it.

```bash
trunk --config ./trunk.toml build --release
```

- 5.) Enjoy! :heart_on_fire:


## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.
- Deployment to GitHub Pages.

## NOTE :scroll:

- *Ceramic.rs :lock: :rocket: :crab:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.
