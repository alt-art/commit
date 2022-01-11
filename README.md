# Commit

This command-line interface makes it possible to make patterned (conventional) commit messages to organize your repository.

This project is a clone of [cz-cli](https://github.com/commitizen/cz-cli) with some minor changes.

I made this project for my own use, because I don't want to mess with the original cz-cli.

## Principal changes in this project

- Don't need to make your project commitizen friendly
- Can be used with any project
- Custom conventional commit message style can be used without any other packages
- Debian package is available
- Show original output of the git command

## What I want to do

- [x] Support for external config files
- [x] [Ability to override the questions with custom configuration](https://github.com/alt-art/commit/wiki#using-custom-configuration-file)
- [x] Ability to choose skip questions with custom configuration
- [x] [Make a wiki page for custom conventional commit message](https://github.com/alt-art/commit/wiki#using-custom-configuration-file)

## Download

[apt derivatives (deb)](https://github.com/alt-art/commit/releases/download/0.2.0/commit_0.2.0_amd64.deb)

[dnf/yum derivatives (rpm)](https://github.com/alt-art/commit/releases/download/0.2.0/commit-0.2.0-1.x86_64.rpm)

## How to make a debian package with cargo-deb for development

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
