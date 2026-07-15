use anyhow::Result;

pub mod cli;
pub mod commands;

/// Receives a PID and DFS the directory to return a Process object
pub(crate) fn read_process(pid: u32) -> Result<Process> {
    let dir = format!("/proc/{pid}");

    unimplemented!()
}

/* fn dfs_dir(dir: String, process: &mut Option<Process>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            if let Ok(path) = entry.file_name().into_string() {
                dfs_dir(path, process)?;
            }
        } else if file_type.is_file() {
            let contents = fs::read_to_string(entry.path());
            // begin building up the Process
        } else {
            continue;
        }
    }

    unimplemented!()
} */

pub(crate) struct Process {
    /// Name of the process
    name: String,
    /// PID of the Process
    pid: u32,
    /// Args passed into the executable, differentiable from other processes of the same executable
    args: Vec<String>,
    /// The PID of the process that spawn this one (Parent PID), essential for building the graph
    ppid: u32,
    /// User Group
    uid: u32,
    /// Group
    gid: Option<u32>,
}
