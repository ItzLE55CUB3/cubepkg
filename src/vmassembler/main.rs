use clap::Parser;
use lalrpop_util::lalrpop_mod;
use std::fs::{read_to_string, File};
use std::io::Write;

lalrpop_mod!(asm);


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Raw assembly file
    input: String,

    // Output Path
    #[clap(short, long, default_value = "a.out")]
    out: String,
}

fn main() {
    let args = Args::parse();

    println!("Input file: {}", args.input);

    let input = read_to_string(args.input).unwrap();

    let ast = asm::OperationsParser::new().parse(&input).unwrap();

    println!("{:?}", ast);

    let mut file = File::create(&args.out).unwrap();

    let header: &[u8] = &[0x1e, 0x55, 0xc6, 0xb3, // Magic Number
                    0x00, 0x00, // Version
                    0x00 << 4 | 0x00, // Enable Segment Compress (Code + Other)
                    0x00 // Compress Option (1 means lz)
    ];

    file.write(header).unwrap();

    for operation in ast {
        file.write(&operation.generate()).unwrap();
    }

    println!("Outputed in {}", &args.out);
}
