# Moxide

Moxide is a powerful tmux session manager written in Rust. It simplifies the process of creating and managing complex tmux sessions, allowing you to define and control multiple windows and commands effortlessly.

## Features

- **Declarative and Extensible:** Define your sessions using simple YAML configuration files. Customize and extend your setup as needed.
- **Single Binary:** Moxide is distributed as a single binary. No runtime or interpreter is required, making it easy to install and run.
- **Projects, Templates, and Directories:** Unlike other session managers, Moxide supports directories, templates, and projects:
  - **Directories:** Easily create named sessions based on the directory you're working in.
  - **Templates:** Create reusable templates for common setups. For example, a JavaScript template could open `nvim` in one window and run `npm` commands in another.
  - **Projects:** Combine directories with templates to streamline workflows. You can specify a template for a project or directly define the windows and commands needed.

## Installation

To install Moxide, use the following command:

```bash
cargo install moxide
```

## Configuration Files

Moxide uses simple YAML configuration files. For an example, you can view my personal Moxide configuration [here](https://github.com/Dlurak/Dotfiles/tree/master/moxide).

### Concepts

#### Directories

Directories allow you to create a named tmux session based on a specific directory, making it easy to create sessions for important directories.

#### Templates

Templates define the layout of windows, panes, and commands. They do not require a specific directory and can be customized for different programming languages or workflows. For example, a JavaScript template might open nvim in one window and run npm commands in another.

#### Projects

Projects combine directories and templates. You can specify a template to use with a directory or define the session setup directly within the project configuration. This flexibility helps you manage complex workflows more efficiently.

## License

Moxide is licensed under the GPL.

## Similar Projects

If you are exploring alternatives, you might find these similar tools useful:

- [Sesh](https://github.com/joshmedeski/sesh)
- [tmuxinator](https://github.com/tmuxinator/tmuxinator)
