[![gitmoji badge](https://img.shields.io/badge/gitmoji-%20😜%20😍-FFDD67.svg?style=flat-square)](https://github.com/carloscuesta/gitmoji)

# gitmoji-cli
> A [gitmoji](https://github.com/carloscuesta/gitmoji) interactive client for using gitmojis on commit messages.

## About

This project provides an easy solution for using [**gitmoji**](https://github.com/carloscuesta/gitmoji) from your command line. Gitmoji-cli solves the hassle of searching through the gitmoji list. Includes a bunch of options you can play with! :tada:
And it is a shameless copy of the [original](https://github.com/carloscuesta/gitmoji-cli) but written in Rust.
## Install

```bash
$ git clone ...
$ cargo install -p .
```

## Usage

```bash
$ gitmoji --help
```

```
A gitmoji interactive client for using gitmojis on commit messages.

  Usage
    $ gitmoji
  Options
    --init, -i      Initialize gitmoji as a commit hook
    --remove, -r    Remove a previously initialized commit hook
    --config, -g    Setup gitmoji-cli preferences.
    --commit, -c    Interactively commit using the prompts
    --list, -l      List all the available gitmojis
    --search, -s    Search gitmojis
    --version, -v   Print gitmoji-cli installed version
    --update, -u    Sync emoji list with the repo
```

### Commit

You can use the commit functionality in two ways, directly or via a commit-hook.

#### Client

Start the interactive commit client, to auto generate your commit based on your prompts.

```bash
$ gitmoji -c
```

#### Hook

Run the init option, add your changes and commit them, after that the prompts will begin and your commit message will be built.

```bash
$ gitmoji -i
$ git add .
$ git commit
```

⚠️ The hook **should not be used** with the `gitmoji -c` command.

### Search

Search using specific keywords to find the right gitmoji.

```bash
$ gitmoji -s bug
```


### List

Pretty print all the available gitmojis.

```bash
$ gitmoji -l
```

### Update

Update the gitmojis list, by default the first time you run gitmoji, the cli creates a cache to allow using this tool without internet connection.

```bash
$ gitmoji -u
```

### Config

Run `gitmoji -g` to setup some gitmoji-cli preferences, such as the auto `git add .` feature.
