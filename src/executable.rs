use std::{env, os::unix::fs::PermissionsExt, path::PathBuf, process};

pub struct ExecutableCommand {
    pub name: String,
    pub path: String,
    pub args: Vec<String>,
}

impl ExecutableCommand {
    pub fn execute(&self) {
        let result = process::Command::new(&self.name).args(&self.args).status();
        match result {
            Ok(status) if status.success() => {}
            Ok(status) => println!("{} exited with {}", self.path, status),
            Err(error) => {
                println!("Failed to execute {}: {}", self.path, error);
            }
        }
    }
}

pub fn get_executable_path(command: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths)
        .map(|path| path.join(command))
        .filter(|path| path.is_file())
        .find(|file| is_executable(file))
}

fn is_executable(file: &PathBuf) -> bool {
    let Ok(metadata) = file.metadata() else {
        return false;
    };
    // 0o111 corresponds to the execution bits:
    // User (0o100), Group (0o010), or Others (0o001)
    metadata.permissions().mode() & 0o111 != 0
}
