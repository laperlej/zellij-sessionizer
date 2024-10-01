# zellij-sessionizer

[showcase.webm](https://github.com/user-attachments/assets/dc1b3174-07ac-4210-a689-bdc2e16ee0de)

This plugin is based on ThePrimeagen's tmux sessionizer [script](https://github.com/ThePrimeagen/.dotfiles/blob/master/bin/.local/scripts/tmux-sessionizer)

The idea is to provide a list of directories that contain your projects/repos. When open, the plugin will display a list of all the subdirectories(1 deep) for selection.

When a directory is selected, a new session will be created with it's name and cwd set to the directory.

If the session already exists, it will attach instead.

The main difference with the built-in filepicker is that the search is done over a single combined flat list so there is no need to navigate the file system.

## Usage

- up/down arrow: select previous/next folder
- enter: create session based on selected folder
- other characters will populate a search bar that will apply fuzzy find.

## Installation

Download zellij-session-tree.wasm from the [latest release](https://github.com/laperlej/zellij-sessionizer/releases/latest) and place it in your zellij plugins folder.

```bash
mkdir -p ~/.config/zellij/plugins
wget https://github.com/laperlej/zellij-session-tree/releases/latest/download/zellij-session-tree.wasm -O ~/.config/zellij/plugins/zellij-session-tree.wasm
```

## Configuration

Add the plugin to a keybinding in your config.toml.

In this example, the keybinding is bound to `g` in tmux mode.

```kdl
tmux {
    # other keybinds here ...
    bind "g" { LaunchOrFocusPlugin "file:~/.config/zellij/plugins/zellij-sessionizer.wasm" {
            floating true
            move_to_focused_tab true
            cwd "/"
            root_dirs "/home/laperlej/projects;/home/laperlej/workspaces"
            session_layout "myCustomLayout"
        }; SwitchToMode "Locked";
    }
}
```

arguments:

- root_dirs: string of paths separated by a semicolon, default is `""`
- session_layout: the layout to use for new sessions, please prepend the layout name with a `:` if you want to use a built-in layout ex: `:compact`, default is `:default`.

**IMPORTANT:** I highly recommend setting cwd to `/`. due to the way plugins interact with the filesystem the root_dirs **must** be absolute paths and **must** be descendants of the cwd.

## Contributing

Contributions are welcome. Please open an issue or a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
