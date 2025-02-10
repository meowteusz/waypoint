# Waypoint - Ergonomic PATH Management

A `$PATH` manager that stores paths in a structured JSON format with metadata,
validates them, and generates the final `$PATH` string for shell consumption.

## Key Features

- Single source of truth: Manages entire PATH, not just user additions
- JSON storage with path metadata (tags, priority, active status)
- Shell-agnostic (works with bash, zsh, etc.)
- Path validation (existence, permissions)
- Manage paths through TUI or directly edit file

## Data Structure

```json
{
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
            "active": true,
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

Waypoint is designed to manage *all* system PATHs, not just additions. Simply consolidating all your desired locations into one file and running `waypoint` will correctly build the PATH, following priority and active flags.

```bash
# Add to .bashrc/.zshrc
# Make sure to remove other $PATH modifiers
export PATH=$(waypoint export)
```

## Version 1.0 Scope

- [x] Initial PATH parsing and JSON serialization
- [x] Config file creation and parsing
- [ ] Basic CRUD operations for paths
- [ ] Path validation
- [ ] Priority-based ordering
- [ ] Active/inactive toggling
- [ ] Basic tag support
- [x] Shell integration

## Future Considerations (Post 1.0)

- Path conflict detection
- Permission validation
- Multiple profiles
- Import/export functionality
- TUI/Web interface
