use std::env;
use uuidv4::*;

fn uuid_input(i: &str) -> u8 {
    match i {
        ""                      => 0, 
        "--low"                 => 0, 
        "--upper"               => 1, 
        "--dashed"              => 7,
        "--dashed-low"          => 7,
        "--dashed-upper"        => 8,
        "--svg"                 => 11,
        "--svg-low"             => 11,
        "--svg-upper"           => 12,
        "--svg-dashed"          => 18,
        "--svg-dashed-low"      => 18,
        "--svg-dashed-upper"    => 19,
        "--help"                => 29,
        "?"                     => 29,
        _                       => 0,
    }
}

pub fn uuid_ui() -> Result<(), UUIDError> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 0 {
        println!("{}", uuid()?);
    } else {
        let parameters: u8 = args.iter().map(|i| uuid_input(i)).collect::<Vec<u8>>().iter().sum();
        
        match parameters {
            0     => println!("{}", uuid()?),
            1     => println!("{}", uuid_uppercase()?),
            7     => println!("{}", uuid_dashed()?),
            8     => println!("{}", uuid_dashed_uppercase()?),
            11    => to_svg(uuid()?)?,
            12    => to_svg(uuid_uppercase()?)?,
            18    => to_svg(uuid_dashed()?)?,
            19    => to_svg(uuid_dashed_uppercase()?)?,
            29    => println!("{}", uuid_help()),
            _     => println!("{}", uuid()?),
            //_     => println!("Unsupported option. Please use option ? or --help to display the available options."),
        }
    }
    
    Ok(())
}

pub fn uuid_help() -> String {
    "Options: \n
    ./uuidv4bin prints low case UUID V4, e.g. 0be42eae8ecf43ea8812c467f7cc0ed3\n
    ./uuidv4bin --low prints low case UUID V4, e.g. 2ac166ea469342a1b5e6dc6fc7af4985\n
    ./uuidv4bin --upper prints low case UUID, e.g. D529FFFC30A64873816260D77944B4FF\n
    ./uuidv4bin --dashed prints low case UUID V4 with dashes, e.g. cf57a620-4042-4e6a-a93b-6f44ae48935f\n
    ./uuidv4bin --dashed-low prints low case UUID V4 with dashes, e.g. 78a6673d-fa8c-41e8-84c7-0765ba196305\n
    ./uuidv4bin --dashed-upper prints upper case UUID V4 with dashes, e.g. 3C1EE587-3D25-4095-807C-918848243E32\n
    ./uuidv4bin --svg encodes a low case UUID V4 without dashes into QR-code as a svg file\n
    ./uuidv4bin --svg-low encodes a low case UUID V4 without dashes into QR-code as a svg file\n
    ./uuidv4bin --svg-upper encodes an upper case UUID V4 without dashes into QR-code as a svg file\n
    ./uuidv4bin --svg-dashed encodes a low case UUID V4 with dashes into QR-code as a svg file\n
    ./uuidv4bin --svg-dashed-low encodes a low case UUID V4 with dashes into QR-code as a svg file\n
    ./uuidv4bin --svg-dashed-upper encodes an upper case UUID V4 with dashes into QR-code as a svg file\n
    ./uuidv4bin --help prints this help\n
    ./uuidv4bin ? prints this help\n".to_string()
}


fn main() -> Result<(), UUIDError>{

    uuid_ui()?;    
    
    Ok(())
} 
