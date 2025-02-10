# Waypoint - Ergonomic PATH Management

## Core Concept

A PATH manager that stores paths in a structured JSON format with
metadata, validates them, and generates the final PATH string for shell
consumption.

## Key Features

- Single source of truth: Manages entire PATH, not just user additions
- JSON storage with path metadata (tags, priority, active status)
- Shell-agnostic (works with bash, zsh, etc.)
- Path validation (existence, permissions)
- Simple command-line interface

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
            "comment": "Custom development tools"
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
waypoint sync      # Generate PATH from JSON
waypoint add       # Add new path with metadata
waypoint remove    # Remove path
waypoint tag       # Add/remove tags
waypoint list      # List paths (filterable)
waypoint disable   # Toggle active status
```

## Shell Integration

```bash
# Add to .bashrc/.zshrc
eval "$(waypoint sync)"
```

## Version 1.0 Scope

- [x] Initial PATH parsing and JSON serialization
- [x] Config file creation and parsing
- [ ] Basic CRUD operations for paths
- [ ] Path validation
- [ ] Priority-based ordering
- [ ] Active/inactive toggling
- [ ] Basic tag support
- [ ] Shell integration

## Future Considerations (Post 1.0)

- Path conflict detection
- Permission validation
- Multiple profiles
- Import/export functionality
- TUI/Web interface
