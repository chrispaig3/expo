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
# Pass multiple repos across different users or organizations that you have write access to.

expo delete example1/repo example2/repo example3/repo

# Delete multiple repositories concurrently
expo delete example1/repo example2/repo example3/repo --yes

# Change visibility of multiple repositories concurrently
expo visibility public user1/repo1 user2/repo2 user3/repo3

# Archive multiple repositories concurrently
expo archive org1/repo1 org2/repo2 org3/repo3

# Create multiple repositories concurrently (private by default)
expo create user1/new-repo1 user2/new-repo2

# Create public repositories with descriptions
expo create user/my-repo --public --description "My awesome project"
```

#### Installation
> Less than half of a megabyte in size: ***380.0 kb***
- The below command will install *Expo* to `.cargo/bin`.

```
cargo install --git https://github.com/chrispaig3/expo
```
