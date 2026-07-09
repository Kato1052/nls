# Shell Completion Installation Guide

This directory contains shell completion files for the `nls` command.

## Installation

### Zsh

Copy the completion file to your zsh completions directory:

```bash
# Option 1: User-level completions (recommended)
mkdir -p ~/.local/share/zsh/site-functions
cp completions/zsh/_nls ~/.local/share/zsh/site-functions/

# Option 2: System-level completions (requires sudo)
sudo cp completions/zsh/_nls /usr/local/share/zsh/site-functions/
```

Then reload your zsh configuration:
```bash
autoload -Uz compinit && compinit
```

Or simply start a new shell session.

### Bash

Copy the completion file to your bash completions directory:

```bash
# Option 1: User-level completions (recommended)
mkdir -p ~/.local/share/bash-completion/completions
cp completions/bash/nls ~/.local/share/bash-completion/completions/

# Option 2: System-level completions (requires sudo)
sudo cp completions/bash/nls /usr/local/etc/bash_completion.d/
```

Then reload your bash configuration:
```bash
source ~/.bashrc
```

### Fish

Copy the completion file to your fish completions directory:

```bash
mkdir -p ~/.config/fish/completions
cp completions/fish/nls ~/.config/fish/completions/
```

The completions will be available immediately in new fish shells.

## Verify Installation

Test the completion by typing:

```bash
nls -<TAB>
nls --<TAB>
```

You should see the available options and flags.

## Completion Features

The completion files provide:

- **Options**: `-l`, `--completions`, `-h`, `--help`
- **Flag descriptions**: Help text for each option
- **Argument completions**: File paths for the `paths` argument

## Generated With

These completion files were automatically generated using the `nls --completions` command,
which uses the `clap_complete` crate to generate shell-specific completions.
