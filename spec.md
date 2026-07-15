# Sysgraph Specification

## Goal

`sysgraph` is a Linux systems exploration CLI that builds a graph of
relationships between processes and (later) other system resources.

The MVP focuses only on **processes**.

------------------------------------------------------------------------

# Commands

## `sysgraph ps`

List every running process in a flat table.

### Purpose

Equivalent to a simplified `ps`.

### Input

None.

### Output

Columns:

-   PID
-   Parent PID
-   State
-   Process Name

Example:

``` text
PID   PPID STATE NAME
1     0    S     systemd
324   1    S     sshd
812   324  S     bash
```

------------------------------------------------------------------------

## `sysgraph inspect <PID>`

Display detailed information about a single process.

### Purpose

Inspect one process without displaying the whole graph.

### Information

-   PID
-   Parent PID
-   Name
-   State
-   Executable path
-   Command line

Later:

-   Open files
-   Open sockets
-   Memory usage

------------------------------------------------------------------------

## `sysgraph tree`

Display the parent/child process hierarchy.

### Purpose

Visualize process ancestry.

Example:

``` text
systemd
├── sshd
│   └── bash
│       └── sysgraph
```

Options:

-   `--root <PID>` starts traversal from a specific process.

------------------------------------------------------------------------

## `sysgraph dot`

Export the process graph as Graphviz DOT.

### Purpose

Generate diagrams.

Example:

``` dot
digraph {
    "1" -> "324";
    "324" -> "812";
}
```

Options:

-   `--root <PID>`

------------------------------------------------------------------------

# Internal Architecture

The program should be separated into four layers:

1.  Data collection (`/proc`)
2.  Graph construction
3.  Graph filtering
4.  Rendering (tree, table, DOT)

These layers should not depend directly on each other.
