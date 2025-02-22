# Moxide

**Moxide** is a powerful tmux session manager written in Rust that simplifies the process of creating and managing complex tmux sessions. It allows you to define and control multiple windows and commands effortlessly, making it a perfect fit for developers and teams alike.

## Features

- **Declarative and Extensible**: Define your sessions using simple YAML configuration files. Customize and extend your setup as needed.
- **Single Binary**: Moxide is distributed as a single binary, requiring no runtime or interpreter, making it easy to install and run.
- **Projects, Templates, and Directories**:
  - **Directories**: Create named sessions based on the directory you're working in.
  - **Templates**: Reusable templates for common setups. For instance, a Rust template might open Neovim in one window and run Cargo commands in another.
  - **Projects**: Combine directories with templates to streamline workflows. Specify a template for a project or directly define the windows and commands needed.

## Installation

To install Moxide, use the following command:

```bash
cargo install moxide
```

### Nix

Moxide is also available in nixpkgs under the name `moxide`, if you use nix you know how to install a nixpkg but still here is a command:

```bash
nix shell nixpkgs#moxide
```

## Why moxide

Moxide offers a unique combination of features:

- **Reusable Templates**: Easily define a session once and apply it across multiple projects, making it ideal for managing similar setups.
- **Flexibility**: Choose whether to use a template or define project-specific configurations, allowing for customization when needed.
- **Declarative Configuration**: Use simple YAML files for configuration, promoting readability and ease of management, it can even be generated from an active session.

Here are some scenarios how moxide might help people:

- Developers managing multiple projects that require similar setups (For example 5 JavaScript projects, where only the directory differs).
- Teams looking for a standardized tmux environment across shared codebases.
- Individuals who appreciate the flexibility of reusing templates while maintaining the option for custom project configurations.

## Configuration Files

Moxide uses simple YAML configuration files. Below are examples of how to configure directories, projects, and templates.

### Example Configuration Files

#### Directories

`~/.config/moxide/directories.yaml`

```yaml
- name: "Downloads"
  path: "~/Downloads/"
- name: "Home"
  path: "~/"
```

#### Projects

`~/.config/moxide/projects/Moxide.yaml`

```yaml
name: Moxide

root_dir: ~/SoftwareDevelopment/cli/moxide/
template: Rust
```

It is also possible to have project specific configs.

#### Templates

`~/.config/template/Rust.yaml`

```yaml
name: Rust

windows:
  - name: Neovim
    panes:
      - nvim
  - name: Cargo
    layout: even-horizontal
    panes:
      - cargo build
      - cargo clippy
```

## Usage

Moxide allows you to choose whether to apply a template to a project or define the windows and commands per project. This flexibility enables teams to maintain standard setups while accommodating unique project needs.

### Launching Moxide Sessions

```bash
moxide project start ProjectName
```
```bash
moxide template start Rust --directory ~
```
```bash
moxide dir start Downloads
```
```bash
moxide dir start "~/Pictures/"
```

## Scripting integration

Moxide can be easily integrated into shell scripts. Below is a sample script for launching Moxide sessions with a selection tool:

```bash
#!/bin/bash

project_emoji="🚀"
template_emoji="🛠️"
directory_emoji="📁"

list=$(moxide list \
    --format-project "$project_emoji {}"\
    --format-template "$template_emoji {}"\
    --format-directory "$directory_emoji {}"
)

value=$(echo "$list" | \
    fzf \
    --no-sort \
    --layout reverse \
    --border rounded \
    --border-label "Moxide Sessions" \
    --no-scrollbar \
    --prompt "✨ " \
    --pointer "👉" \
    --color=bg+:#313244,bg:#1e1e2e,spinner:#f5e0dc,hl:#f38ba8 \
    --color=fg:#cdd6f4,header:#f38ba8,info:#cba6f7,pointer:#f5e0dc \
    --color=marker:#b4befe,fg+:#cdd6f4,prompt:#cba6f7,hl+:#f38ba8 \
    --color=selected-bg:#45475a
)

IFS=' ' read -r emoji name <<< "$value"
case "$emoji" in
    $project_emoji)
        moxide project start "$name"
        ;;
    $template_emoji)
        moxide template start "$name"
        ;;
    $directory_emoji)
        moxide dir start "$name"
        ;;
esac
```
You can bind this script into a tmux popup with the following command:

```tmux
bind-key s display-popup -B -E -w 40% -h 13 "~/Dotfiles/scripts/shell/moxide.sh"
```

## Similar Projects

If you are exploring alternatives, you might find these similar tools useful:

- [Sesh](https://github.com/joshmedeski/sesh)
- [tmuxinator](https://github.com/tmuxinator/tmuxinator)
