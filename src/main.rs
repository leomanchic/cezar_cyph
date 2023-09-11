use clap::Parser;

const LOGO: &str = r#"
 ________  ________  _______   ________  ________  ________          ________  ___  ________  ___  ___  _______   ________     
|\   ____\|\   __  \|\  ___ \ |\   ____\|\   __  \|\   __  \        |\   ____\|\  \|\   __  \|\  \|\  \|\  ___ \ |\   __  \    
\ \  \___|\ \  \|\  \ \   __/|\ \  \___|\ \  \|\  \ \  \|\  \       \ \  \___|\ \  \ \  \|\  \ \  \\\  \ \   __/|\ \  \|\  \   
 \ \  \    \ \   __  \ \  \_|/_\ \_____  \ \   __  \ \   _  _\       \ \  \    \ \  \ \   ____\ \   __  \ \  \_|/_\ \   _  _\  
  \ \  \____\ \  \ \  \ \  \_|\ \|____|\  \ \  \ \  \ \  \\  \|       \ \  \____\ \  \ \  \___|\ \  \ \  \ \  \_|\ \ \  \\  \| 
   \ \_______\ \__\ \__\ \_______\____\_\  \ \__\ \__\ \__\\ _\        \ \_______\ \__\ \__\    \ \__\ \__\ \_______\ \__\\ _\ 
    \|_______|\|__|\|__|\|_______|\_________\|__|\|__|\|__|\|__|        \|_______|\|__|\|__|     \|__|\|__|\|_______|\|__|\|__|
                                 \|_________|                                                                                  
                                                                                                                                                                                                                                                       
"#;



#[derive(Parser, Debug)]
#[command(name="Caesar cipher", author="Gurman", version, about="Программа для шифрования методом Цезаря", long_about = None)]

struct Args {

    /// Phrase to cipher
    #[arg(short, long)]
    messege: String,
    
    /// Shift of cipher
    #[arg(short, long, default_value_t = 1)]
    shift: usize,
    
    
}

fn encrypt(messege: &str, shift: &usize) -> String{
    let alphabet = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя";
    let length = alphabet.len() as usize;
    let mut dec = String::new();
    for i in messege.chars(){
        if i.is_whitespace() || i.is_numeric(){
            dec.push(i);
        }else{
        let in_lower = if i.is_lowercase(){true}else{false};
        let index = alphabet.find(i.to_lowercase().next().unwrap_or(i)).unwrap() / 2usize;
        let new_index = ((index+shift) as usize) % (length / 2usize);
        let new_symbol = alphabet.chars().nth(new_index);
        match in_lower{
            false  => {dec.push(new_symbol.unwrap().to_uppercase().next().unwrap())},
            true =>  dec.push(new_symbol.unwrap()),
        }        
        }
    }
    dec
}

fn main() {
    bunt::println!("{$#04F6E6}{}{/$}", LOGO);
    let args = Args::parse();
    bunt::println!("{$bold+green}Result:{/$} before encryption <{[red]}>",&args.messege);
    bunt::println!("{$bold+green}Result:{/$} after encryption <{[blue]}>",encrypt(&args.messege, &args.shift));
}