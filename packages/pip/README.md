# gitpixel (Python)

Draw ASCII text on your GitHub contribution graph using backdated commits.

This Python package is a thin installer that downloads the prebuilt
`gitpixel` Rust binary from [GitHub Releases](https://github.com/Smasduq/git-pixel/releases)
and exposes it as a `gitpixel` console command.

## Install

```sh
pip install gitpixel
```

## Usage

```sh
gitpixel draw --text "SADIQU" --year 2026 --repo ../my-repo          # preview
gitpixel draw --text "SADIQU" --year 2026 --repo ../my-repo --confirm # write commits
gitpixel history
gitpixel revert
```

See the [crate documentation](https://crates.io/crates/gitpixel) for details.

## License

MIT
