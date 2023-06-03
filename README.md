Cultura helps you to improve your culture day after day by showing little fact taken from reddit, wikipedia and so on.

# Install

## How to install

### From rust

Run :

```
cargo install cultura
```

### From GitHub

Download the binary from the release page => https://github.com/antham/cultura/releases/latest and install it in your binary path.

## Shell config

You must setup cultura in your shell, look at the following configuration corresponding to your shell.

### Bash

It could depend how bash is setup.

At the top of your `.bashrc` file add:

```
eval "$(cultura init bash)"
```

### Fish

In your fish config file add:

```
cultura init fish | source
```

### Zsh

It could depend how zsh is setup.

At the top of your `.zshrc` file add:

```
eval "$(cultura init zsh)"
```

:information_source: If you have some slowdown issue with p10k, ensure that you moved the init command before the init of the p10k instant prompt

# Configuration

The config can be edited with the provided commands or could be directly edited through the config file, run `cultura config get-config-file-path` to get the path of the config file.

It must be necessary to stop the daemon to take the config in account, so simply run after finishing the edition, `cultura daemon stop`.

## The fact rendering

You can customize the way a fact is rendered by using the command `cultura config set-template`.

Let see an example :

```
__A new fact to__:cyan:bold __=>__ $fact:yellow
```

Your text must be enclosed between 2 underscores, you can provide a color and use styles like in the example.

The `$fact` variable is a special one and will be interpolated with the fact.

| Colors  | Styles    |
|---------|-----------|
| blue    | bold      |
| red     | dimmed    |
| green   | italic    |
| black   | underline |
| yellow  |           |
| white   |           |
| purple  |           |
| cyan    |           |
| magenta |           |


## The providers

You can define which fact provider you want to use, default is to display all, if you want to customize which one to use do `cargo run config set-providers TIL,DYK`, it will use both `DYK` and `TIL` as fact provider.

The available providers:
| Provider | Site                                                     |
|----------|----------------------------------------------------------|
| DYK      | https://en.wikipedia.org/wiki/Wikipedia:Recent_additions |
| TIL      | https://www.reddit.com/r/todayilearned/                  |
# Troubleshoot

## Reset the application

You can reset the application by running `cultura doctor reset`, it will remove the whole existing config and database.

## Check the providers

You can ensure providers are running properly by calling `cultura doctor run-providers`, you will see for each provider if the parser is working properly.
