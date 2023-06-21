Cultura helps you to improve your culture :nerd_face: day after day by providing interesting fact taken from various sources such as reddit, wikipedia and so on :book:.

![example of cultura in a terminal](https://github.com/antham/cultura/blob/master/pictures/example.png?raw=true)

---

- [Install](#install)
- [Configuration](#configuration)
- [Troubleshoot](#troubleshoot)
- [Submitting a new provider](#submitting-a-new-provider)

---

# Install

## How to install

### From rust

Run :

```
cargo install cultura
```

:warning: It could be necessary to install the `libssl-dev` package.

### From GitHub

Download the binary from the release page => https://github.com/antham/cultura/releases/latest and install it in your binary path.

### From docker

Pull the image from `docker.io/antham/cultura`, see [the shell config section](#shell-config) for more information.

## Shell config

To set up Cultura in your shell, please refer to the following configuration specific to your shell

### Bash

It could depend how bash is setup.

In your bash config file add:

```
source <(cultura init bash)
```

If you use the docker container, download the file `scripts/docker.bash` and in your bash config file add:

```
source <path_of_the_script>/docker.bash
```

### Fish

In your fish config file add:

```
cultura init fish | source
```

If you use the docker container, download the file `scripts/docker.fish` and in your fish config file add:

```
source <path_of_the_script>/docker.fish
```

### Zsh

It could depend how zsh is setup.

In your zsh config file add:

```
source <(cultura init zsh)
```

If you use the docker container, download the file `scripts/docker.zsh` and in your zsh config file add:

```
source <path_of_the_script>/docker.zsh
```

:information_source: If you have some slowdown issue with p10k, ensure that you moved the init command before the init of the p10k instant prompt

# Configuration

The config can be edited with the provided commands or could be directly edited through the config file, run `cultura config get-config-file-path` to get the path of the config file.

It must be necessary to stop the daemon to take the config in account, so simply run after finishing the edition, `cultura daemon stop`.

If you are using cultura with Docker, you can replace the cultura command with `docker exec cultura-af2fce60 cultura`. Make sure the container is already running before executing the command.

## The fact rendering

You can customize the way a fact is rendered by using the command `cultura config set-template`.

Let see an example :

```
__A new fact__:cyan:bold __=>__ $fact:yellow
```

Your text must be enclosed between 4 underscores, you can provide a color and use styles like in the example.

The `$fact` variable is a special one and will be interpolated with the fact.

| Colors  | Styles    |
| ------- | --------- |
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

You can define which fact provider you want to use, default is to display all, if you want to customize which one to use for instance do `cargo run config set-providers TIL,DYK` and it will use both `DYK` and `TIL` as fact provider.

The available providers:
| Provider | Site |
|----------|----------------------------------------------------------|
| DYK | https://en.wikipedia.org/wiki/Wikipedia:Recent_additions |
| TIL | https://www.reddit.com/r/todayilearned/ |

# Troubleshoot

## Debugging issues on the daemon

With the binary:

- run `cultura daemon start true` to start the daemon in foreground and check for errors.

With docker:

- run `docker logs cultura-af2fce60`

## Check the providers

With the binary:

- you can ensure providers are running properly by calling `cultura doctor run-providers`, you will see for each provider if the parser is working properly.

With docker:

- you can ensure providers are running properly by calling `docker exec cultura-af2fce60 cultura doctor run-providers`, you will see for each provider if the parser is working properly.

## Reset the application

With the binary:

- you can reset the application by running `cultura doctor reset`, it will remove the whole existing config and database.

With docker:

- remove the container with `docker rm -f cultura-af2fce60`.

# Submitting a new provider

If you have an idea for a source of facts that could be added to cultura, feel free to create an issue and provide the necessary details and reasoning behind your suggestion.
