use goblin::Object ;
use std::fs ;
fn main(){
    let some_binary = fs::read("main").expect("Failed !"); //use a compiled file insted of main file
    match Object::parse(&some_binary){
        Ok(obj) => match obj {
            Object::Elf(_) => println!("This is a elf file !"),
            _ => println!("This is not a elf file"),
        },
        Err(e) => {
            eprintln!("Error :{}" , e);
        }
    }
}



//another example 
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}


//another example 
use std::fs::File ;
use std::io::Read ;
fn main() -> std::io::Result<()>{
    let mut file = File::open("output.txt").expect("Failed @Q");
    let mut content = String::new();
    match file.read_to_string(&mut content){
        Ok(_) => println!("Your data is :{}" , content),
        Err(e) => {
            eprintln!("Error as e :{:?}" , e);
        }
    }
    Ok(())
}




///another example
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("output.bin").expect("Failed to open file!");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read file!");  //use read_to_end insted of read_to_string to read the raw binary file content 

    println!("Read {} bytes from output.bin", data.len());
    Ok(())
}

////another example 
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut f = File::create("foo.txt")?;
    f.write_all(&1234_u32.to_be_bytes())?;    //see explanation why using to_be_bytes()
    Ok(())
}
............................................explanation.................................
to_be_bytes() means big endian ...means the bigh byte will load first
to_le_bytes() means little endian ...means the little byte will load first 
why using endian insted of as_bytes() ?
answer => when its comes to manupulate fixed size then use endian 
and otherwise use as_bytes() for most string manupulation .
    
কেন আলাদা?
এটা CPU architecture-এর উপর নির্ভর করে।

Intel = Little Endian

Network protocols, কিছু embedded systems = Big Endian
"Network Byte Order" মানেই Big Endian

///another example 
use std::fs::File ;
use std::io::Read ;
use std::process::exit ;
use anyhow::Context;

fn read_file(file_path :&str) -> anyhow::Result<String> {

   let mut  file = File::open("log.c").with_context(|| format!("Failed to read file {:?}" , file_path))?;
   let mut content = String::new() ;
   match file.read_to_string(&mut content){

       Ok(_) => {},
       Err(e) => {

           eprintln!("Error :{:?}" , e);
       }
   }
   Ok(content.to_string())
}
fn main() -> anyhow::Result<()> {

    let result = read_file("log.c").unwrap_or_else(|err| {

        eprintln!("Error as err :{:?}" , err);
        exit(1);
    });
    println!("File content is :{:?}" , result);
    Ok(())
}


//another example ..........................................very advance example for elf file ............................
use elf::ElfBytes;
use elf::endian::AnyEndian;
use elf::note::{Note, NoteGnuBuildId};
use elf::section::SectionHeader;

fn main() {
    // Step 1: Read the ELF file
    let path = std::path::PathBuf::from("sample-objects/symver.x86_64.so");
    let file_data = match std::fs::read(&path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("❌ Error: Could not read file '{}': {}", path.display(), e);
            return;
        }
    };
    let slice = file_data.as_slice();

    // Step 2: Parse the ELF file
    let file = match ElfBytes::<AnyEndian>::minimal_parse(slice) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("❌ Error: Failed to parse ELF: {}", e);
            return;
        }
    };

    // Step 3: Find the .note.gnu.build-id section
    let abi_shdr: SectionHeader = match file.section_header_by_name(".note.gnu.build-id") {
        Ok(Some(header)) => header,
        Ok(None) => {
            eprintln!("❌ Error: File does not have a .note.gnu.build-id section.");
            return;
        }
        Err(e) => {
            eprintln!("❌ Error: Failed to parse section table: {}", e);
            return;
        }
    };

    // Step 4: Parse notes from the section
    let notes: Vec<Note> = match file.section_data_as_notes(&abi_shdr) {
        Ok(iter) => iter.collect(),
        Err(e) => {
            eprintln!("❌ Error: Could not read note section: {}", e);
            return;
        }
    };

    // Step 5: Print the Build ID
    let mut build_id_found = false;
    for note in &notes {
        if let Note::GnuBuildId(NoteGnuBuildId(build_id)) = note {
            build_id_found = true;
            print!("🔹 Build ID: ");
            for byte in *build_id {
                print!("{:02x}", byte);
            }
            println!();
        } else {
            println!("Other note found: {:?}", note);
        }
    }
    if !build_id_found {
        println!("❌ No Build ID found in the notes.");
    }

    // Step 6: Find common ELF section data (.dynsym, .dynstr, .hash)
    let common = match file.find_common_data() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("❌ Error: Failed to parse common sections: {}", e);
            return;
        }
    };

    let dynsyms = match common.dynsyms {
        Some(d) => d,
        None => {
            eprintln!("❌ Error: .dynsyms section is missing.");
            return;
        }
    };
    let strtab = match common.dynsyms_strs {
        Some(s) => s,
        None => {
            eprintln!("❌ Error: .dynstr section is missing.");
            return;
        }
    };
    let hash_table = match common.sysv_hash {
        Some(h) => h,
        None => {
            eprintln!("❌ Error: System V hash section is missing.");
            return;
        }
    };

    // Step 7: Look for the symbol named 'memset'
    let name = b"memset";
    match hash_table.find(name, &dynsyms, &strtab) {
        Ok(Some((sym_idx, sym))) => {
            println!("\n✅ Symbol '{}' found!",std::str::from_utf8(name).unwrap());
            println!("  • Index in symbol table: {}", sym_idx);
            println!("  • Symbol name: {}", strtab.get(sym.st_name as usize).unwrap_or("<?>"));
            println!("  • Address: 0x{:x}", sym.st_value);
            println!("  • Size: {}", sym.st_size);
            // Removed the `st_info` field from the output
            println!("  • Raw Symbol Entry: {:?}", dynsyms.get(sym_idx).unwrap());
        }
        Ok(None) => {
            println!("❌ Symbol '{}' not found in hash table.", std::str::from_utf8(name).unwrap());
        }
        Err(e) => {
            eprintln!("❌ Error parsing symbol: {}", e);
        }
    }
}
