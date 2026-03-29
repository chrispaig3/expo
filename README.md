# Expo

![Image](https://github.com/user-attachments/assets/a3f1f574-c9f9-4caf-9573-9ac29594a53f)

### Inspiration
> The name ***Expo*** was inspired by the popular dry erasers and markers.

### **Motivation**
> The GitHub CLI currently supports managing one repository at a time.
> This is obviously inconvenient when you have accumulated a large number of GitHub repositories.

### ***Warning: This program has destructive capabilities (use at own risk)***

#### Prerequisites
- Make sure you have the Rust toolchain `rustup` and `gh` (GitHub CLI) installed.
- If you are already authenticated with GitHub via the CLI; run: `gh auth refresh -h github.com -s delete_repo`

#### Basic Usage
- expo delete user/repo [--yes]
- expo archive user/repo [--unarchive]
- expo visibility [public|private] user/repo
- expo create user/repo [--public] [--description "text"]

```sh
# Pass multiple repos that you have write access to.

# Delete multiple repositories concurrently
expo delete user/repo1 user/repo2 user/repo3 --yes

# Change visibility of multiple repositories concurrently
expo visibility public user/repo1 user/repo2 user/repo3

# Archive multiple repositories concurrently
expo archive user/repo1 user/repo2 user/repo3

# Create multiple repositories concurrently (private by default)
expo create user/repo1 user/repo2

# Create public repositories with descriptions
expo create user/repo --public --description "My awesome project"
```

#### Installation
> Less than half of a megabyte in size: ***380.0 kb***
- The below command will install *Expo* to `.cargo/bin`.

```
cargo install --git https://github.com/chrispaig3/expo
```
