use amino::{AminoAcid, AMINO_ACIDS};

fn format_all(aa: AminoAcid) -> String {
    let single = aa.single();
    let short = aa.short();
    let full = aa.full();
    format!("{single} {short} {full}")
}

fn main() {
    // Read arguments.
    let mut args = std::env::args().skip(1);
    let first = args.next();
    let second = args.next();

    if first.as_deref() == Some("list") {
        for aa in AMINO_ACIDS {
            match second.as_deref() {
                Some("full" | "long" | "name") => println!("{}", aa.full()),
                Some("short" | "three" | "3") => println!("{}", aa.short()),
                Some("single" | "one" | "1") => println!("{}", aa.single()),
                Some("all") | None => println!("{}", &format_all(aa)),
                Some(unknown) => {
                    eprintln!("Unknown format specifier '{unknown}'");
                    return;
                }
            }
        }
    } else if let Some(first) = first {
        if let Ok(aa) = AminoAcid::try_from(first.as_str()) {
            println!("{}", format_all(aa));
        } else {
            eprintln!("Unknown amino acid code or action '{first}'.");
        }
    }
}
