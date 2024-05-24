
pub struct Links {
    pub id: u32,
    pub href: String,
    pub text: String,
}

pub fn links_generator(amounts: u8) -> Vec<Links> {
    let mut vecc: Vec<Links> = Vec::new();
    for gen in 0..amounts {
        let links = Links {
            id: gen as u32, href: "href ".to_string() + format!("{}", gen).as_str(), text: "text ".to_string() + gen.to_string().as_str()
        };
        vecc.push(links);
    }
    vecc
}

