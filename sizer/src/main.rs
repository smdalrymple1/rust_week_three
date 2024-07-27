use std::collections::HashMap;

use clap::Parser;


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

#[derive(Debug,Clone)]
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

    fn to_bytes(&self) -> FileSize {
        FileSize {
            size: self.size * self.get_divisor(),
            unit: FileUnit::Byte
        }
    }

    fn to_kilobytes(&self) -> FileSize {
        FileSize {
           size: self.to_bytes().size / FileUnit::KiloByte.get_divisor(),
           unit: FileUnit::KiloByte
        }
    }

    fn to_megabytes(&self) -> FileSize {
        FileSize {
            size: self.to_bytes().size / FileUnit::MegaByte.get_divisor(),
            unit: FileUnit::MegaByte
        }
    }

    fn to_gigabytes(&self) -> FileSize {
        FileSize {
            size: self.to_bytes().size / FileUnit::GigaByte.get_divisor(),
            unit: FileUnit::GigaByte
        }
    }

    fn to_string(&self) -> String {
        format!("{} {}s", self.size, self.unit.get_long_name()) 
    }
}

fn main() {

    //TODO Make Static?
    //TODO Still feel Boileplateish. 
    //It feels wayyy to verbose for me to be using this language correctly
    let types: HashMap<String, FileUnit> = HashMap::from([
        (String::from(FileUnit::Byte.get_short_name()), FileUnit::Byte),
        (String::from(FileUnit::KiloByte.get_short_name()), FileUnit::KiloByte),
        (String::from(FileUnit::MegaByte.get_short_name()), FileUnit::MegaByte),
        (String::from(FileUnit::GigaByte.get_short_name()), FileUnit::GigaByte)
    ]);

    let args = Args::parse();
    if let Some(unit) = types.get(&args.in_unit.to_uppercase()) {
        let file_size = FileSize{
            size: args.in_size,
            unit: unit.clone()
        };
        //A better design would be able to fold this into a for loop
        println!("Sizes {:?}", HashMap::from([
            (format!("{}s", FileUnit::Byte.get_long_name()), file_size.to_bytes().to_string()),
            (format!("{}s", FileUnit::KiloByte.get_long_name()), file_size.to_kilobytes().to_string()),
            (format!("{}s", FileUnit::MegaByte.get_long_name()), file_size.to_megabytes().to_string()),
            (format!("{}s", FileUnit::GigaByte.get_long_name()), file_size.to_gigabytes().to_string())
        ]));
    } else {
        println!("{} is not a known file size unit", args.in_unit)
    }
}
