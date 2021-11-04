# Commit

This project is a copy of [cz-cli](https://github.com/commitizen/cz-cli) with some minor changes.

I made this project for my own use, because I don't want to mess with the original cz-cli.

## Principal changes in this project

- Don't need to make your project commitizen friendly
- Can be used with any project
- Custom conventional commit message style can be used without any other packages
- Debian package is available
- Show original output of the git command

## What I want to do
- [ ] Support for external config files
- [ ] Ability to override the questions
- [ ] Ability to choose skip questions
- [ ] Make a wiki page for custom conventional commit message

## How to make a debian package with cargo-deb

Install cargo-deb:

```bash
cargo install cargo-deb
```

Make a debian package:

```bash
cargo deb
```

or just:

```bash
cargo deb --install # build and install the package
```
