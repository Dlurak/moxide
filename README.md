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

## Scripting

It is easy and recommended to use moxide in shell scripts.
Here is the one I recommend:

```sh
#!/bin/bash

project_emoji="🚀"
template_emoji="🛠️"
directory_emoji="📁"

if [ "$1" == "rofi" ]; then
    selection_tool="rofi"
else
    selection_tool="fzf"
fi

fzf_colors="--color=bg+:#313244,bg:#1e1e2e,spinner:#f5e0dc,hl:#f38ba8 \
	--color=fg:#cdd6f4,header:#f38ba8,info:#cba6f7,pointer:#f5e0dc \
	--color=marker:#b4befe,fg+:#cdd6f4,prompt:#cba6f7,hl+:#f38ba8 \
	--color=selected-bg:#45475a"

list=$(moxide list \
	--format-project "$project_emoji {}"\
	--format-template "$template_emoji {}"\
	--format-directory "$directory_emoji {}"
)

case "$selection_tool" in
	"rofi")
		value=$(echo "$list" | rofi -dmenu -p " Tmux");
		;;
	*)
		value=$(echo "$list" | \
			fzf \
			--no-sort \
			--layout reverse \
			--border rounded \
			--border-label "Moxide Sessions" \
			--no-scrollbar \
			--prompt "✨ " \
			--pointer "👉" \
			$fzf_colors
		)
		;;
esac

emoji="${value:0:1}"
name="${value:2}"

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
Then you can bind that into a tmux popup:
```tmux
bind-key s display-popup -B -E -w 40% -h 12 "~/Dotfiles/scripts/shell/moxide.sh"
```

## Similar Projects

If you are exploring alternatives, you might find these similar tools useful:

- [Sesh](https://github.com/joshmedeski/sesh)
- [tmuxinator](https://github.com/tmuxinator/tmuxinator)
