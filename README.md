# zellij-sessionizer

This plugin is based on ThePrimeagen's [sessionizer](https://github.com/ThePrimeagen/sessionizer).

The idea is to have a folder with all your projects/repos/workspaces and use a script to select a folder and create a session at that folder with both it's name and cwd set to the folder. It is a very efficient way to create new sessions.

## Usage

- up/down arrow: select previous/next folder
- enter: create session based on selected folder
- a search bar: search for a folder, currently based on wether the folder contains the search term

## Installation

Download zellij-session-tree.wasm from the [latest release](https://github.com/zellij-org/zellij-session-tree/releases/latest) and place it in your zellij plugins folder.

```bash
mkdir -p ~/.config/zellij/plugins
wget https://github.com/zellij-org/zellij-session-tree/releases/latest/download/zellij-session-tree.wasm -O ~/.config/zellij/plugins/zellij-session-tree.wasm
```

## Configuration

Add the plugin to a keybinding in your config.toml.

In this example, the keybinding is bound to `g` in tmux mode.

Be sure to set cwd to the folder with all your projects.

```kdl
tmux {
    # more keybinds here
    bind "g" { LaunchOrFocusPlugin "file:~/.config/zellij/plugins/zellij-sessionizer.wasm" {
            floating true
            move_to_focused_tab true
            cwd "~/projects"
        }; SwitchToMode "Locked";
    }
}
```

## Contributing

Contributions are welcome. Please open an issue or a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
