# How to install

## Summary

- [AUR](#aur)
- [Debian based](#debian-based)
- [Fedora](#fedora)
- [Install from source](#install-from-source)
- [Windows portable binary](#windows-portable-binary)
  - [Add to programs on windows](#add-to-programs-on-windows)
  - [Add program to PATH on windows](#add-program-to-path-on-windows)

### AUR

```bash
yay -S commits
```

### Debian based

> Verify what is the latest release [here](https://github.com/alt-art/commit/releases)

```bash
wget https://github.com/alt-art/commit/releases/download/0.5.0/commit_0.5.0_amd64.deb
apt install ./commit_0.5.0_amd64.deb
```

### Fedora

> Verify what is the latest release [here](https://github.com/alt-art/commit/releases)

```bash
wget https://github.com/alt-art/commit/releases/download/0.5.0/commit_0.5.0_x86_64.rpm
dnf install ./commit_0.5.0_x86_64.rpm
```

### Install from source

Requires rust and cargo

```bash
git clone https://github/alt-art/commit
cd commits
cargo build --release
```

```bash
cp target/release/commits /usr/local/bin
```

### Windows portable binary

> Verify what is the latest release [here](https://github.com/alt-art/commit/releases)

#### Add to programs on windows

```shell
mkdir "C:\Program Files\commits"
copy commit.exe "C:\Program Files\commits"
```

#### Add program to PATH on windows

```shell
setx /M PATH "%PATH%;C:\Program Files\commits"
```
