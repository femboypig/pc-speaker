use libc::{c_int, c_uint, ioctl};
use nix::unistd::Uid;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::os::fd::AsRawFd;
use std::thread::sleep;
use std::time::Duration;
use std::path::Path;
use std::collections::HashMap;

// Constants for ioctl calls to PC speaker
const CONSOLE_IOCTL: u8 = 0x09;
const KDMKTONE: u64 = 0x4B30;
const HZ: u32 = 1193180;

// Note frequencies (Hz)
// Basic notes
const NOTE_C4: u32 = 262;
const NOTE_CS4: u32 = 277;
const NOTE_D4: u32 = 294;
const NOTE_DS4: u32 = 311;
const NOTE_E4: u32 = 330;
const NOTE_F4: u32 = 349;
const NOTE_FS4: u32 = 370;
const NOTE_G4: u32 = 392;
const NOTE_GS4: u32 = 415;
const NOTE_A4: u32 = 440;
const NOTE_AS4: u32 = 466;
const NOTE_B4: u32 = 494;

// Higher octave
const NOTE_C5: u32 = 523;
const NOTE_CS5: u32 = 554;
const NOTE_D5: u32 = 587;
const NOTE_DS5: u32 = 622;
const NOTE_E5: u32 = 659;
const NOTE_F5: u32 = 698;
const NOTE_FS5: u32 = 740;
const NOTE_G5: u32 = 784;
const NOTE_GS5: u32 = 831;
const NOTE_A5: u32 = 880;
const NOTE_AS5: u32 = 932;
const NOTE_B5: u32 = 988;

// Lower octave
const NOTE_C3: u32 = 131;
const NOTE_CS3: u32 = 139;
const NOTE_D3: u32 = 147;
const NOTE_DS3: u32 = 156;
const NOTE_E3: u32 = 165;
const NOTE_F3: u32 = 175;
const NOTE_FS3: u32 = 185;
const NOTE_G3: u32 = 196;
const NOTE_GS3: u32 = 208;
const NOTE_A3: u32 = 220;
const NOTE_AS3: u32 = 233;
const NOTE_B3: u32 = 247;

const NOTE_REST: u32 = 0;

// Note durations (ms)
const WHOLE: u32 = 1000;
const HALF: u32 = 500;
const QUARTER: u32 = 250;
const EIGHTH: u32 = 125;
const SIXTEENTH: u32 = 63;
const NOKIA_SIXTEENTH: u32 = 95; // Nokia tune timing

/// A music note with its frequency and duration
struct Note {
    frequency: u32,
    duration_ms: u32,
}

impl Note {
    fn new(frequency: u32, duration_ms: u32) -> Self {
        Self {
            frequency,
            duration_ms,
        }
    }
}

/// Generate a beep using kernel syscall (like motherboard beeps)
fn kernel_beep(frequency: u32, duration_ms: u32) -> io::Result<()> {
    // Convert frequency to PC speaker value
    let arg = if frequency == 0 {
        // Silence
        0
    } else {
        // Add duration (upper 16 bits)
        let freq_val = (HZ / frequency) & 0xffff;
        freq_val | (duration_ms << 16)
    };

    // Attempt to open the console
    match OpenOptions::new().read(true).write(true).open("/dev/console") {
        Ok(console) => {
            // Make the ioctl call to generate beep
            let result = unsafe { 
                // Convert u64 to c_uint using into()
                ioctl(console.as_raw_fd(), (KDMKTONE as c_uint).into(), arg as c_int) 
            };
            
            if result != 0 {
                // Fall back to system bell if ioctl fails
                print!("\x07"); // ASCII bell character
                io::stdout().flush()?;
            }
        }
        Err(err) => {
            eprintln!("Could not open console: {}", err);
            // Try with simple print of bell character
            print!("\x07"); // ASCII bell character
            io::stdout().flush()?;
        }
    }

    // Always sleep for the duration
    sleep(Duration::from_millis(duration_ms as u64));
    Ok(())
}

/// Play the Tetris theme (Korobeiniki)
fn play_tetris_theme() -> io::Result<()> {
    println!("Playing Tetris Theme...");

    let notes = [
        // First part
        Note::new(NOTE_E5, QUARTER),
        Note::new(NOTE_B4, EIGHTH),
        Note::new(NOTE_C5, EIGHTH),
        Note::new(NOTE_D5, QUARTER),
        Note::new(NOTE_C5, EIGHTH),
        Note::new(NOTE_B4, EIGHTH),
        Note::new(NOTE_A4, QUARTER),
        Note::new(NOTE_A4, EIGHTH),
        Note::new(NOTE_C5, EIGHTH),
        Note::new(NOTE_E5, QUARTER),
        Note::new(NOTE_D5, EIGHTH),
        Note::new(NOTE_C5, EIGHTH),
        Note::new(NOTE_B4, QUARTER),
        Note::new(NOTE_B4, EIGHTH),
        Note::new(NOTE_C5, EIGHTH),
        Note::new(NOTE_D5, QUARTER),
        Note::new(NOTE_E5, QUARTER),
        Note::new(NOTE_C5, QUARTER),
        Note::new(NOTE_A4, QUARTER),
        Note::new(NOTE_A4, QUARTER),
        Note::new(NOTE_REST, QUARTER),
        
        // Second part
        Note::new(NOTE_D5, QUARTER),
        Note::new(NOTE_F5, EIGHTH),
        Note::new(NOTE_A5, QUARTER),
        Note::new(NOTE_G5, EIGHTH),
        Note::new(NOTE_F5, EIGHTH),
        Note::new(NOTE_E5, QUARTER),
        Note::new(NOTE_C5, EIGHTH),
        Note::new(NOTE_E5, QUARTER),
        Note::new(NOTE_D5, EIGHTH),
        Note::new(NOTE_C5, EIGHTH),
        Note::new(NOTE_B4, QUARTER),
        Note::new(NOTE_B4, EIGHTH),
        Note::new(NOTE_C5, EIGHTH),
        Note::new(NOTE_D5, QUARTER),
        Note::new(NOTE_E5, QUARTER),
        Note::new(NOTE_C5, QUARTER),
        Note::new(NOTE_A4, QUARTER),
        Note::new(NOTE_A4, QUARTER),
        Note::new(NOTE_REST, QUARTER),
    ];

    for note in notes.iter() {
        kernel_beep(note.frequency, note.duration_ms)?;
        // Small break between notes
        sleep(Duration::from_millis(10));
    }

    Ok(())
}

/// Play Jingle Bells melody
fn play_jingle_bells() -> io::Result<()> {
    println!("Playing Jingle Bells...");

    let notes = [
        // Jingle bells, jingle bells, jingle all the way
        Note::new(NOTE_E4, QUARTER), Note::new(NOTE_E4, QUARTER), Note::new(NOTE_E4, HALF),
        Note::new(NOTE_E4, QUARTER), Note::new(NOTE_E4, QUARTER), Note::new(NOTE_E4, HALF),
        Note::new(NOTE_E4, QUARTER), Note::new(NOTE_G4, QUARTER), Note::new(NOTE_C4, QUARTER), Note::new(NOTE_D4, QUARTER),
        Note::new(NOTE_E4, WHOLE),
        
        // Oh what fun it is to ride
        Note::new(NOTE_F4, QUARTER), Note::new(NOTE_F4, QUARTER), Note::new(NOTE_F4, QUARTER), Note::new(NOTE_F4, QUARTER),
        Note::new(NOTE_F4, QUARTER), Note::new(NOTE_E4, QUARTER), Note::new(NOTE_E4, QUARTER), Note::new(NOTE_E4, QUARTER),
        Note::new(NOTE_E4, QUARTER), Note::new(NOTE_D4, QUARTER), Note::new(NOTE_D4, QUARTER), Note::new(NOTE_E4, QUARTER),
        Note::new(NOTE_D4, HALF), Note::new(NOTE_G4, HALF),
        
        // Jingle bells, jingle bells, jingle all the way
        Note::new(NOTE_E4, QUARTER), Note::new(NOTE_E4, QUARTER), Note::new(NOTE_E4, HALF),
        Note::new(NOTE_E4, QUARTER), Note::new(NOTE_E4, QUARTER), Note::new(NOTE_E4, HALF),
        Note::new(NOTE_E4, QUARTER), Note::new(NOTE_G4, QUARTER), Note::new(NOTE_C4, QUARTER), Note::new(NOTE_D4, QUARTER),
        Note::new(NOTE_E4, WHOLE),
    ];

    for note in notes.iter() {
        kernel_beep(note.frequency, note.duration_ms)?;
        // Small break between notes
        sleep(Duration::from_millis(50));
    }

    Ok(())
}

/// Play the Imperial March from Star Wars
fn play_imperial_march() -> io::Result<()> {
    println!("Playing Imperial March...");

    let notes = [
        // First part
        Note::new(NOTE_G4, QUARTER), Note::new(NOTE_G4, QUARTER), Note::new(NOTE_G4, QUARTER),
        Note::new((NOTE_E4 * 3) / 4, QUARTER), Note::new(NOTE_B4, EIGHTH),
        Note::new(NOTE_G4, QUARTER), Note::new((NOTE_E4 * 3) / 4, QUARTER), Note::new(NOTE_B4, EIGHTH), Note::new(NOTE_G4, HALF),
        
        // Second part
        Note::new(NOTE_D5, QUARTER), Note::new(NOTE_D5, QUARTER), Note::new(NOTE_D5, QUARTER),
        Note::new((NOTE_E5 * 3) / 4, QUARTER), Note::new(NOTE_B4, EIGHTH),
        Note::new(NOTE_G4, QUARTER), Note::new((NOTE_E4 * 3) / 4, QUARTER), Note::new(NOTE_B4, EIGHTH), Note::new(NOTE_G4, HALF),
    ];

    for note in notes.iter() {
        kernel_beep(note.frequency, note.duration_ms)?;
        // Small break between notes
        sleep(Duration::from_millis(50));
    }

    Ok(())
}

/// Play the classic Nokia ringtone
fn play_nokia_tune() -> io::Result<()> {
    println!("Playing Nokia Tune (Gran Vals)...");

    let notes = [
        Note::new(NOTE_E5, NOKIA_SIXTEENTH), Note::new(NOTE_D5, NOKIA_SIXTEENTH),
        Note::new(NOTE_FS4, NOKIA_SIXTEENTH * 2), Note::new(NOTE_GS4, NOKIA_SIXTEENTH * 2),
        Note::new(NOTE_CS5, NOKIA_SIXTEENTH), Note::new(NOTE_B4, NOKIA_SIXTEENTH),
        Note::new(NOTE_D4, NOKIA_SIXTEENTH * 2), Note::new(NOTE_E4, NOKIA_SIXTEENTH * 2),
        Note::new(NOTE_B4, NOKIA_SIXTEENTH), Note::new(NOTE_A4, NOKIA_SIXTEENTH),
        Note::new(NOTE_CS4, NOKIA_SIXTEENTH * 2), Note::new(NOTE_E4, NOKIA_SIXTEENTH * 2),
        Note::new(NOTE_A4, NOKIA_SIXTEENTH * 4),
    ];

    for note in notes.iter() {
        kernel_beep(note.frequency, note.duration_ms)?;
        // Small break between notes for clarity
        sleep(Duration::from_millis(30));
    }

    Ok(())
}

/// Play Super Mario Bros theme
fn play_super_mario() -> io::Result<()> {
    println!("Playing Super Mario Bros theme...");

    let tempo = 1.2; // Speed factor (higher = faster)
    let quarter = (QUARTER as f32 / tempo) as u32;
    let eighth = (EIGHTH as f32 / tempo) as u32;
    let _sixteenth = (SIXTEENTH as f32 / tempo) as u32;
    
    let intro = [
        Note::new(NOTE_E5, eighth), 
        Note::new(NOTE_E5, eighth), 
        Note::new(NOTE_REST, eighth),
        Note::new(NOTE_E5, eighth), 
        Note::new(NOTE_REST, eighth), 
        Note::new(NOTE_C5, eighth),
        Note::new(NOTE_E5, quarter), 
        Note::new(NOTE_G5, quarter), 
        Note::new(NOTE_REST, quarter),
        Note::new(NOTE_G4, quarter), 
        Note::new(NOTE_REST, quarter),
    ];
    
    let main_theme = [
        // Part 1
        Note::new(NOTE_C5, quarter), 
        Note::new(NOTE_REST, eighth), 
        Note::new(NOTE_G4, eighth),
        Note::new(NOTE_REST, quarter), 
        Note::new(NOTE_E4, quarter),
        Note::new(NOTE_REST, eighth), 
        Note::new(NOTE_A4, quarter), 
        Note::new(NOTE_B4, quarter),
        Note::new(NOTE_AS4, eighth), 
        Note::new(NOTE_A4, quarter),
        
        // Part 2
        Note::new(NOTE_G4, eighth * 3), 
        Note::new(NOTE_E5, eighth * 3),
        Note::new(NOTE_G5, eighth * 3), 
        Note::new(NOTE_A5, quarter),
        Note::new(NOTE_F5, eighth), 
        Note::new(NOTE_G5, eighth),
        Note::new(NOTE_REST, eighth), 
        Note::new(NOTE_E5, quarter),
        Note::new(NOTE_C5, eighth), 
        Note::new(NOTE_D5, eighth),
        Note::new(NOTE_B4, quarter), 
        Note::new(NOTE_REST, quarter),
    ];

    // Play intro
    for note in intro.iter() {
        kernel_beep(note.frequency, note.duration_ms)?;
        // Small break between notes
        sleep(Duration::from_millis(5));
    }
    
    // Play main theme
    for note in main_theme.iter() {
        kernel_beep(note.frequency, note.duration_ms)?;
        // Small break between notes
        sleep(Duration::from_millis(5));
    }

    Ok(())
}

/// Play Happy Birthday song
fn play_happy_birthday() -> io::Result<()> {
    println!("Playing Happy Birthday...");

    let notes = [
        // Happy Birthday to you
        Note::new(NOTE_C4, QUARTER), Note::new(NOTE_C4, EIGHTH),
        Note::new(NOTE_D4, QUARTER + EIGHTH),
        Note::new(NOTE_C4, QUARTER), Note::new(NOTE_F4, QUARTER),
        Note::new(NOTE_E4, HALF),
        
        // Happy Birthday to you
        Note::new(NOTE_C4, QUARTER), Note::new(NOTE_C4, EIGHTH),
        Note::new(NOTE_D4, QUARTER + EIGHTH),
        Note::new(NOTE_C4, QUARTER), Note::new(NOTE_G4, QUARTER),
        Note::new(NOTE_F4, HALF),
        
        // Happy Birthday dear [name]
        Note::new(NOTE_C4, QUARTER), Note::new(NOTE_C4, EIGHTH),
        Note::new(NOTE_C5, QUARTER + EIGHTH),
        Note::new(NOTE_A4, QUARTER), Note::new(NOTE_F4, QUARTER),
        Note::new(NOTE_E4, QUARTER), Note::new(NOTE_D4, QUARTER),
        
        // Happy Birthday to you
        Note::new(NOTE_AS4, QUARTER), Note::new(NOTE_AS4, EIGHTH),
        Note::new(NOTE_A4, QUARTER + EIGHTH),
        Note::new(NOTE_F4, QUARTER), Note::new(NOTE_G4, QUARTER),
        Note::new(NOTE_F4, HALF),
    ];

    for note in notes.iter() {
        kernel_beep(note.frequency, note.duration_ms)?;
        // Small break between notes
        sleep(Duration::from_millis(20));
    }

    Ok(())
}

/// Play a custom song from a file
fn play_custom_song() -> io::Result<()> {
    print!("Enter the path to your melody file: ");
    io::stdout().flush()?;
    
    let mut path = String::new();
    io::stdin().read_line(&mut path)?;
    let path = path.trim();
    
    if !Path::new(path).exists() {
        println!("File not found: {}", path);
        return Ok(());
    }
    
    println!("Loading melody from: {}", path);
    
    // Create a note name to frequency mapping
    let mut note_map: HashMap<String, u32> = HashMap::new();
    note_map.insert("C3".to_string(), NOTE_C3);
    note_map.insert("CS3".to_string(), NOTE_CS3);
    note_map.insert("D3".to_string(), NOTE_D3);
    note_map.insert("DS3".to_string(), NOTE_DS3);
    note_map.insert("E3".to_string(), NOTE_E3);
    note_map.insert("F3".to_string(), NOTE_F3);
    note_map.insert("FS3".to_string(), NOTE_FS3);
    note_map.insert("G3".to_string(), NOTE_G3);
    note_map.insert("GS3".to_string(), NOTE_GS3);
    note_map.insert("A3".to_string(), NOTE_A3);
    note_map.insert("AS3".to_string(), NOTE_AS3);
    note_map.insert("B3".to_string(), NOTE_B3);
    
    note_map.insert("C4".to_string(), NOTE_C4);
    note_map.insert("CS4".to_string(), NOTE_CS4);
    note_map.insert("D4".to_string(), NOTE_D4);
    note_map.insert("DS4".to_string(), NOTE_DS4);
    note_map.insert("E4".to_string(), NOTE_E4);
    note_map.insert("F4".to_string(), NOTE_F4);
    note_map.insert("FS4".to_string(), NOTE_FS4);
    note_map.insert("G4".to_string(), NOTE_G4);
    note_map.insert("GS4".to_string(), NOTE_GS4);
    note_map.insert("A4".to_string(), NOTE_A4);
    note_map.insert("AS4".to_string(), NOTE_AS4);
    note_map.insert("B4".to_string(), NOTE_B4);
    
    note_map.insert("C5".to_string(), NOTE_C5);
    note_map.insert("CS5".to_string(), NOTE_CS5);
    note_map.insert("D5".to_string(), NOTE_D5);
    note_map.insert("DS5".to_string(), NOTE_DS5);
    note_map.insert("E5".to_string(), NOTE_E5);
    note_map.insert("F5".to_string(), NOTE_F5);
    note_map.insert("FS5".to_string(), NOTE_FS5);
    note_map.insert("G5".to_string(), NOTE_G5);
    note_map.insert("GS5".to_string(), NOTE_GS5);
    note_map.insert("A5".to_string(), NOTE_A5);
    note_map.insert("AS5".to_string(), NOTE_AS5);
    note_map.insert("B5".to_string(), NOTE_B5);
    note_map.insert("REST".to_string(), NOTE_REST);
    
    // Create a duration mapping for text notation
    let mut duration_map: HashMap<String, u32> = HashMap::new();
    duration_map.insert("W".to_string(), WHOLE);
    duration_map.insert("H".to_string(), HALF);
    duration_map.insert("Q".to_string(), QUARTER);
    duration_map.insert("E".to_string(), EIGHTH);
    duration_map.insert("S".to_string(), SIXTEENTH);
    
    // Parse the file
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut notes = Vec::new();
    
    println!("Playing custom melody...");
    
    // Parse each line
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        // Split line into parts (note name and duration)
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Warning: Invalid line format: {}", line);
            continue;
        }
        
        let note_name = parts[0].to_uppercase();
        let duration_part = parts[1];
        
        // Get note frequency
        let frequency = if let Some(&freq) = note_map.get(&note_name) {
            freq
        } else {
            // Try to parse as direct frequency
            match note_name.parse::<u32>() {
                Ok(freq) => freq,
                Err(_) => {
                    println!("Warning: Unknown note name: {}", note_name);
                    continue;
                }
            }
        };
        
        // Get duration
        let duration = if let Some(&dur) = duration_map.get(&duration_part.to_uppercase()) {
            dur
        } else {
            // Try to parse as direct milliseconds
            match duration_part.parse::<u32>() {
                Ok(dur) => dur,
                Err(_) => {
                    println!("Warning: Unknown duration: {}", duration_part);
                    continue;
                }
            }
        };
        
        // Add to notes array
        notes.push(Note::new(frequency, duration));
    }
    
    if notes.is_empty() {
        println!("No valid notes found in the file.");
        return Ok(());
    }
    
    // Play the notes
    for note in notes.iter() {
        kernel_beep(note.frequency, note.duration_ms)?;
        // Small break between notes
        sleep(Duration::from_millis(10));
    }
    
    Ok(())
}

fn main() -> io::Result<()> {
    // Check if running as root
    if !Uid::effective().is_root() {
        eprintln!("Warning: This program requires root privileges to access the PC speaker");
        eprintln!("Please run with sudo");
        return Ok(());
    }

    // Setup Ctrl+C handler for graceful exit
    ctrlc::set_handler(|| {
        println!("\nExiting...");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Main loop
    loop {
        println!("\nBeep Song Player (Rust Edition)");
        println!("1. Tetris Theme");
        println!("2. Jingle Bells");
        println!("3. Imperial March (Star Wars)");
        println!("4. Nokia Tune");
        println!("5. Super Mario Bros Theme");
        println!("6. Happy Birthday");
        println!("7. Play Custom Melody");
        println!("q. Quit");
        
        print!("Select a song (1-7, q to quit): ");
        io::stdout().flush()?;
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        
        match choice.trim() {
            "1" => play_tetris_theme()?,
            "2" => play_jingle_bells()?,
            "3" => play_imperial_march()?,
            "4" => play_nokia_tune()?,
            "5" => play_super_mario()?,
            "6" => play_happy_birthday()?,
            "7" => play_custom_song()?,
            "q" | "Q" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid selection, please try again."),
        }
    }

    Ok(())
} 