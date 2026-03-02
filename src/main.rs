use std::env::{current_dir};
use std::fs::{rename, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::string::ToString;
use clap::{Parser, Subcommand};

const TMP_NAME: &str = "temp_file_name.tmp";
#[derive(Debug, Parser)]
struct Options {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        what: String,
        when: String,
        comment: String,
    },
    Read {
        what: String,
    },
    Remove {
        id: String,
    },
    Reindex {},
}

struct App {
    current_dir: PathBuf,
    current_file: String,
    file: File,
}
impl App {
    fn new(current_dir: PathBuf, current_file: String, file: File) -> Self {
        App { current_dir, current_file, file }
    }
    fn last_index(file: &mut File) -> Result<u64, Box<dyn std::error::Error>> {

        let mut ch_index = file.seek(SeekFrom::End(-2))?;
        let mut byte_read = [0_u8; 1];
        let mut byte_vec = Vec::new();

        while ch_index > 0 {
            file.seek(SeekFrom::Start(ch_index))?;
            file.read_exact(&mut byte_read)?;

            if byte_read[0] == b'\n' {
                break;
            }

            byte_vec.push(byte_read[0]);
            ch_index -= 1;
        }
        byte_vec.reverse();
        let num = String::from_utf8(byte_vec)?.split(",").take(1).collect::<String>();
        Ok(num.parse::<u64>()?)
    }
    fn run(&mut self, command: Commands) -> Result<(), Box<dyn std::error::Error>> {

        match command {
            Commands::Add { what, when, comment } => self.add(what, when, comment),
            Commands::Read { what } => self.read(what),
            Commands::Remove { id } => self.remove(id),
            Commands::Reindex {} => self.reindex(),
        }
    }
    fn add(&mut self, what: String, when: String, comment: String) -> Result<(), Box<dyn std::error::Error>> {

        let num = App::last_index(&mut self.file)? + 1;
        self.file.write_all(format!("{num},{what},{when},{comment}\n").as_bytes())?;

        println!("Added success");
        Ok(())
    }
    fn read(&mut self, what: String) -> Result<(), Box<dyn std::error::Error>> {

        // self.file.seek(SeekFrom::Start(0))?;
        
        let mut buffer = String::new();
        let mut reader = BufReader::new(&self.file);

        let what = what.to_lowercase();

        while let Ok(line_size) = reader.read_line(&mut buffer) {

        if line_size == 0 { break }
        let activity = buffer.split(',').nth(1).ok_or("Error: Could not get 2nd element after split in Commands::Read")?.to_lowercase();

        if what == "all" || what.to_lowercase() == activity {
        println!("{}", buffer.trim_end());
        }
        buffer.clear();
        }
        Ok(())
    }
    fn remove(&mut self, id: String) -> Result<(), Box<dyn std::error::Error>> {

        let mut temp_file = File::create(self.current_dir.join(TMP_NAME))?;
        let reader = BufReader::new(&self.file);

        for line in reader.lines() {
            let mut line = line?;

            if line.split(',').next().ok_or("Error: Could not get next element after split in Commands::Remove")? != id {
                line.push('\n');
                temp_file.write_all(line.as_bytes())?
            }
        }
        rename(self.current_dir.join(TMP_NAME), self.current_dir.join(&self.current_file))?;
        // self.file = open_file(self.current_dir.join(&self.current_file))?;
        Ok(())
    }
    fn reindex(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        let mut input = String::new();
        println!("This function can break references. Are you sure you want to continue? Type REINDEX to confirm: ");
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "reindex" {
        println!("Canceled Reindexing");
        return Ok(())
        }

        let mut reader = BufReader::new(&self.file).lines();
        let mut temp_file = File::create(self.current_dir.join(TMP_NAME))?;

        temp_file.write_all((reader.next().ok_or("Error: Could not get first element from reader in Commands::Reindex")?? + "\n").as_bytes())?;

        for (it, line) in reader.enumerate() {

        let line = line?;
        let split_line = line.split(',').collect::<Vec<&str>>();
        let [_, activity, date, comment] = split_line.as_slice() else { return Err(format!("Bad line at index {}: {:?}", it + 1, split_line).into())};

        temp_file.write_all(format!("{},{activity},{date},{comment}\n", it+1).as_bytes())?;
        }
        rename(self.current_dir.join(TMP_NAME), self.current_dir.join(&self.current_file))?;
        // self.file = open_file(self.current_dir.join(&self.current_file))?;
        Ok(())
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let current_dir = current_dir()?
        .parent()
        .ok_or("Could not open db folder")?
        .join("test_db")
        .to_path_buf();
    let file_name = "test_db.csv".to_string();
    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(current_dir.join(&file_name))?;

    let options = Options::parse();

    let mut app = App::new(current_dir, file_name, file);

    app.run(options.command)?;
    Ok(())
}

/*

    Trying to make an activity tracker CLI app.

    THE IDEA:

    tracker -a / --add [activity] [date] [comment]
    do we want to use -a / --add
    or do we want to use tracker add "Activity" "Date" "Comment"

    this will add the activity to a CSV

    tracker -t / --track [activity]
    - When
    - The comments
    - Average times per week or something.

    Dependency:
    - Clap

 */