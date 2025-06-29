# File Handler
| Action | Example | Behavior |
|--------|---------|----------|
| READ | fs::read_to_string("file.txt") | Read whole file as text |
| WRITE | fs::write("file.txt", "data") | Overwrite or create |
| APPEND | OpenOptions::new().append(true) | Add to end without deleting |
| DELETE | fs::remove_file("file.txt") | Permanently delete |
| READ LINE | BufReader loop over lines() | Handle large files |

# Clap 
| Feature | Example | Behavior |
|---------|---------|----------|
| #[derive(Parser)] |---------|----------|
| #[command(...)] |---------|----------|
| #[arg(...)] |---------|----------|
| #[command(subcommand)] |---------|----------|