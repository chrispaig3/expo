# Expo

## Inspiration
> The name ***Expo*** was inspired by the popular dry erasers and markers.

![Image](https://github.com/user-attachments/assets/a3f1f574-c9f9-4caf-9573-9ac29594a53f)

### **Motivation**
> The GitHub CLI currently supports managing one repository at a time.
> This is obviously inconvenient when you have accumulated a large number of GitHub repositories.

### ***Warning: This program has destructive capabilities (use at own risk)***

#### Prerequisites
- Make sure you have the Rust toolchain `rustup` and `gh` (GitHub CLI) installed.
- If you are already authenticated with GitHub via the CLI; run: `gh auth refresh -h github.com -s delete_repo`

#### Basic Usage
- expo delete user/repo

#### Extra
- expo archive user/repo [--unarchive]
- expo visibility user/repo [publice|private]

#### Installation
> Less than half of a megabyte in size: ***487.7 kb***
- The below command will install *Expo* to `.cargo/bin`.

```
cargo install --git https://github.com/chrispaig3/expo
```
