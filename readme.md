# cogs

TODO: to-be-documented

<br/>

## Setup

As prerequisites to run this project, you need to have installed:

-   [rust](https://www.rust-lang.org/tools/install)
-   Its `wasm32-unknown-unknown` target\
    Install it with `rustup target add wasm32-unknown-unknown`
-   [trunk](https://trunkrs.dev/)\
    Install it with `cargo install --locked trunk`

<br/>

## Usage

### Dev mode

For further development:

1. Use `trunk serve` to start it.\
   This will rebuild and reload the page (app) automatically when file/code changes are detected.
2. Go to `http://127.0.0.1:9009/index.html#dev` to access its Web UI.

Appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development. `assets/sw.js` script will try to cache the app, and load the cached version when it cannot connect to server, allowing your app to work offline (like a PWA).

<br/>

## Testing

This section includes the details for testing locally.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

<br/>

## Deploy

To do a Web deploy:

1. Just run `trunk build --release`.
2. It will generate a `dist` directory as a "static html" website
3. Upload the `dist` directory to any of the numerous free hosting websites including [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site).
4. we already provide a workflow that auto-deploys our app to GitHub pages if you enable it.
    > To enable Github Pages, you need to go to Repository -> Settings -> Pages -> Source -> set to `gh-pages` branch and `/` (root).
    >
    > If `gh-pages` is not available in `Source`, just create and push a branch called `gh-pages` and it should be available.
    >
    > If you renamed the `main` branch to something else (say you re-initialized the repository with `master` as the initial branch), be sure to edit the github workflows `.github/workflows/pages.yml` file to reflect the change
    >
    > ```yml
    > on:
    >     push:
    >         branches:
    >             - <branch name>
    > ```

<br/>
