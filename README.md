# Waypoint - Ergonomic PATH Management

A `$PATH` manager that stores paths in a more easily readable and shareable JSON format. Paths
are validated before addition to the file, and can be tagged, activated, and ordered by priority.
A future release will allow for custom JSON format simply by editing the JSON file (with the core
three staying as location, priority, and active).


## Key Features

- Single source of truth: Manages entire PATH, not just user additions
- JSON storage with path metadata (tags, priority, active status)
- Shell-agnostic (works with bash, zsh, etc.)
- Path validation
- Manage paths through TUI or directly edit file

## Data Structure

```json
{
    "path": "your/whole/system:/path/gets/placed:/here/for/ease/of/access",
    "waypoints": [
        {
            "location": "/usr/local/bin",
            "tags": ["system"],
            "priority": 1,
            "active": true
        },
        {
            "location": "~/dev/tools/bin",
            "tags": ["dev", "local"],
            "priority": 2,
            "active": true
        }
    ],
    "metadata": {
        "last_synced": "2025-02-06T10:00:00Z",
        "version": "1.0"
    }
}
```

## Commands

```bash
waypoint freeze    # Snapshots current $PATH into a waypoint config
waypoint export    # Generate $PATH string from config
waypoint list      # List paths (filterable)
waypoint add       # Add new path interactively
waypoint remove    # Remove path interactively
waypoint edit      # Edit an existing path interactively
```

## Shell Integration

Waypoint is designed to manage _all_ system PATHs, not just additions. Simply
consolidating all your desired locations into one file and running `waypoint`
will correctly build the PATH, following priority and active flags.

```bash
# Add to .bashrc/.zshrc
# Make sure to remove other $PATH modifiers
export PATH=$(waypoint export)
```

## Version 1.0 Scope

- [x] Initial PATH parsing and JSON serialization
- [x] Config file creation and parsing
- [x] Shell integration
- [x] Basic CRUD operations for paths
- [ ] ~~Waypoint active/inactive toggle~~ Use editor
- [x] Add CWD to path
- [x] Basic tag support
- [x] Priority-based ordering
- [x] Path validation

## Future Considerations (Post 1.0)

- [ ] Rewrite entire JSON system to support custom structure
- [ ] Path conflict detection
- [ ] Permission validation
