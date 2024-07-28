use std::collections::HashMap;

use clap::Parser;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {

    //Numeric size of file size to convert. 
    //#[arg()]
    in_size: usize,

    //Unit of size of file size to convert.
    //#[arg()]
    in_unit: String,
}


//I'm skeptical of this design
//Adding a new unit requieres updating 4 locations.
//It looks "robust" and crap, but it just feels scattered
#[derive(Debug,Clone,EnumIter)]
enum FileUnit {
    Byte,
    KiloByte,
    MegaByte,
    GigaByte,
}

impl FileUnit {
    fn get_short_name(&self) -> &str {
        //This feels backwards. I'd like the data on the class not in the function
        //What if I want to use these strings elsewhere?
        match self {    
            FileUnit::Byte => "B",
            FileUnit::KiloByte => "KB",
            FileUnit::MegaByte => "MB",
            FileUnit::GigaByte => "GB",
        }
    }

    fn get_long_name(&self) -> &str {
        match self {
            FileUnit::Byte => "byte",
            FileUnit::KiloByte => "kilobyte",
            FileUnit::MegaByte => "megabyte",
            FileUnit::GigaByte => "gigabyte",
        }
    }

    fn get_divisor(&self) -> usize {
        match self {
            FileUnit::Byte => 1,
            FileUnit::KiloByte => 1_000,
            FileUnit::MegaByte => 1_000_000,
            FileUnit::GigaByte => 1_000_000_000,
        }
    }
}

#[derive(Debug)]
struct FileSize {
    size: usize,
    unit: FileUnit
}

//This feels a little too boiler plate tbh
impl FileSize {

    fn get_divisor(&self) -> usize {
        self.unit.get_divisor()
    }

    fn convert(&self, file_unit: FileUnit) -> FileSize {
        FileSize {
            size: self.to_bytes().size / file_unit.get_divisor(),
            unit: file_unit
        }
    }

    fn to_bytes(&self) -> FileSize {
        FileSize {
            size: self.size * self.get_divisor(),
            unit: FileUnit::Byte
        }
    }

    fn to_string(&self) -> String {
        format!("{} {}s", self.size, self.unit.get_long_name()) 
    }
}

fn main() {

    //TODO Make Static?
    let types: HashMap<String, FileUnit> = HashMap::from_iter(
        FileUnit::iter().map(|file_unit| (String::from(file_unit.get_short_name()), file_unit))
    );

    let args = Args::parse();
    if let Some(unit) = types.get(&args.in_unit.to_uppercase()) {
        let file_size = FileSize{
            size: args.in_size,
            unit: unit.clone()
        };

        let sizes: HashMap<String, String> = HashMap::from_iter(
            FileUnit::iter().map(|file_unit| (format!("{}s", file_unit.get_long_name()), file_size.convert(file_unit).to_string()))
        );

        //A better design would be able to fold this into a for loop
        println!("Sizes {:?}", sizes);
    } else {
        println!("{} is not a known file size unit", args.in_unit)
    }
}
