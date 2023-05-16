Cultura helps you to improve your culture day after day by showing little fact taken from reddit, wikipedia and so on.

# Shell config

## Bash

It could depend how bash is setup.

At the top of your `.bashrc` file add:

```
eval "$(cultura init bash)"
```

## Fish

In your fish config file add:

```
cultura init fish | source
```

## Zsh

It could depend how zsh is setup.

At the top of your `.zshrc` file add:

```
eval "$(cultura init zsh)"
```

:information_source: If you have some slowdown issue with p10k, ensure that you moved the init command before the init of the p10k instant prompt
