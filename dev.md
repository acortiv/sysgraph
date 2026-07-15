# Sysgraph Development Plan

## Phase 1 -- CLI

-   Create the Rust project.
-   Add `clap` and `anyhow`.
-   Implement:

```{=html}
<!-- -->
```
    sysgraph
    ├── ps
    ├── inspect <PID>
    ├── tree
    └── dot

Goal: every command parses and dispatches.

------------------------------------------------------------------------

## Phase 2 -- Read One Process

Study:

-   `/proc/<pid>/status`
-   `/proc/<pid>/cmdline`
-   `/proc/<pid>/exe`

Implement:

``` rust
fn read_process(pid: u32) -> Result<Process>;
```

Goal:

    sysgraph inspect $$

prints information about the current shell.

------------------------------------------------------------------------

## Phase 3 -- Read Every Process

Implement:

``` rust
fn process_ids() -> Result<Vec<u32>>;
fn read_all_processes() -> Result<Vec<Process>>;
```

Goal:

    sysgraph ps

prints every running process.

------------------------------------------------------------------------

## Phase 4 -- Build the Graph

Create:

``` rust
struct ProcessGraph
```

containing:

-   process lookup
-   parent → children relationships

Write unit tests for:

-   missing parents
-   multiple roots
-   deep trees

------------------------------------------------------------------------

## Phase 5 -- Render Trees

Implement:

    sysgraph tree

Support:

    --root <PID>

Goal:

Display a readable ASCII process tree.

------------------------------------------------------------------------

## Phase 6 -- DOT Export

Implement:

    sysgraph dot

Verify:

``` bash
dot -Tsvg graph.dot > graph.svg
```

------------------------------------------------------------------------

## Phase 7 -- Filtering

Implement filters such as:

-   `--root`
-   `--name`
-   `--state`

------------------------------------------------------------------------

## Phase 8 -- File Descriptors

Inspect:

    /proc/<pid>/fd

Categorize:

-   files
-   sockets
-   pipes
-   terminals

------------------------------------------------------------------------

## Phase 9 -- Resource Graph

Generalize from:

    Process -> Process

to

    Process -> File
    Process -> Socket
    Process -> Pipe

------------------------------------------------------------------------

## Phase 10 -- Snapshot/Diff

Support:

    sysgraph snapshot
    sysgraph diff before.json after.json

to compare system state over time.
