use std::env;

fn elder_futhark(key: &str) -> char {
    match key {
        "fehu" => 'ᚠ',
        "uruz" => 'ᚢ',
        "thurisaz" => 'ᚦ',
        "ansuz" => 'ᚨ',
        "raido" => 'ᚱ',
        "kaunan" => 'ᚲ',
        "gebo" => 'ᚷ',
        "wunjo" => 'ᚹ',
        "hagalaz" => 'ᚺ',
        "naudiz" => 'ᚾ',
        "isaz" => 'ᛁ',
        "jera" => 'ᛃ',
        "eihwaz" => 'ᛇ',
        "perth" => 'ᛈ',
        "algiz" => 'ᛉ',
        "sowilo" => 'ᛋ',
        "tiwaz" => 'ᛏ',
        "berkanan" => 'ᛒ',
        "ehwaz" => 'ᛖ',
        "mannaz" => 'ᛗ',
        "laguz" => 'ᛚ',
        "ingwaz" => 'ᛜ',
        "othala" => 'ᛟ',
        "dagaz" => 'ᛞ',
        _ => '#',
    }
}

fn transcription_keys() -> Vec<&'static str> {
    let mut out = vec![
        "f",
        "u",
        "þ",
        "a",
        "r",
        "k",
        "c",
        "g",
        "w",
        "h",
        "n",
        "i",
        "j",
        "y",
        "æ",
        "ï",
        "p",
        "z",
        "s",
        "t",
        "b",
        "e",
        "m",
        "l",
        "ŋ",
        "o",
        "d",
        "v",
        "ð",
        "x",
        "ch",
        "ij",
        "cc",
        "th",
        "eau",
        "chr",
        "ing",
        "chl",
        "ng",
        "chj",
        "nk",
        "chw",
        "ei",
        "ø",
        "å",
        "q",
        ".",
        " ",
    ];

    out.sort_by(|a, b| b.len().cmp(&a.len()));

    return out;
}

fn transcription_table(key: &str) -> String {
    match key {
        "f" => elder_futhark("fehu").to_string(),
        "u" => elder_futhark("uruz").to_string(),
        "þ" => elder_futhark("thurisaz").to_string(),
        "a" => elder_futhark("ansuz").to_string(),
        "r" => elder_futhark("raido").to_string(),
        "k" => elder_futhark("kaunan").to_string(),
        "c" => elder_futhark("kaunan").to_string(),
        "g" => elder_futhark("gebo").to_string(),
        "w" => elder_futhark("wunjo").to_string(),
        "h" => elder_futhark("hagalaz").to_string(),
        "n" => elder_futhark("naudiz").to_string(),
        "i" => elder_futhark("isaz").to_string(),
        "j" => elder_futhark("jera").to_string(),
        "y" => elder_futhark("jera").to_string(),
        "æ" => elder_futhark("eihwaz").to_string(),
        "ï" => elder_futhark("eihwaz").to_string(),
        "p" => elder_futhark("perth").to_string(),
        "z" => elder_futhark("algiz").to_string(),
        "s" => elder_futhark("sowilo").to_string(),
        "t" => elder_futhark("tiwaz").to_string(),
        "b" => elder_futhark("berkanan").to_string(),
        "e" => elder_futhark("ehwaz").to_string(),
        "m" => elder_futhark("mannaz").to_string(),
        "l" => elder_futhark("laguz").to_string(),
        "ŋ" => elder_futhark("ingwaz").to_string(),
        "o" => elder_futhark("othala").to_string(),
        "d" => elder_futhark("dagaz").to_string(),
        "v" => elder_futhark("fehu").to_string(),
        "ð" => elder_futhark("thurisaz").to_string(),
        "x" => format!("{}{}", elder_futhark("kaunan"), elder_futhark("sowilo")),
        "ch" => elder_futhark("gebo").to_string(),
        "ij" => elder_futhark("ehwaz").to_string(),
        "cc" => format!("{}{}", elder_futhark("kaunan"), elder_futhark("sowilo")),
        "th" => elder_futhark("thurisaz").to_string(),
        "eau" => elder_futhark("othala").to_string(),
        "chr" => format!("{}{}", elder_futhark("hagalaz"), elder_futhark("raido")),
        "ing" => elder_futhark("ingwaz").to_string(),
        "chl" => format!("{}{}", elder_futhark("hagalaz"), elder_futhark("laguz")),
        "ng" => elder_futhark("ingwaz").to_string(),
        "chj" => format!("{}{}", elder_futhark("hagalaz"), elder_futhark("jera")),
        "nk" => format!("{}{}", elder_futhark("ingwaz"), elder_futhark("kaunan")),
        "chw" => format!("{}{}", elder_futhark("hagalaz"), elder_futhark("wunjo")),
        "ei" => elder_futhark("sowilo").to_string(),
        "ø" => format!("{}{}", elder_futhark("othala"), elder_futhark("ehwaz")),
        "å" => format!("{}{}", elder_futhark("ansuz"), elder_futhark("ansuz")),
        "q" => elder_futhark("kaunan").to_string(),
        "." => '᛭'.to_string(),
        " " => '᛬'.to_string(),
        _ => elder_futhark(key).to_string(),
    }
}

fn transcribe(mut input: String) -> String {
    let keys = transcription_keys();
    
    input = input.to_lowercase();

    for key in keys {
        while input.find(key) != None {
            input = input.replace(key, &transcription_table(key));
        }
    }
    
    input.to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 {
        println!("{}", transcribe(args[1].to_string()));
    } else {
        println!("Usage: <string>");
    }
}
