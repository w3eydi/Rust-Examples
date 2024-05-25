use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    pub id: u32,
    pub href: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinksWrapper {
    pub links: Vec<Links>,
}


// pub fn read_json_from_file(file_path: &str) -> Result<LinksWrapper, Box<dyn Error>> {
//     let file = File::open(file_path)?;
//     // Dosyayı buffer'a alın
//     let reader = BufReader::new(file);
//     // JSON'u deserialize edin
//     let links = serde_json::from_reader(reader)?;
//     Ok(links)

    // Dosyayı açmayı deneyin ve hata yönetimi yapın
    // let file = match File::open(file_path) {
    //     Ok(file) => file,
    //     Err(e) => {
    //         println!("Dosya açma hatası: {}", e);
    //         return ;
    //     }
    // };

    // // Dosyayı buffer'a alın
    // let reader = BufReader::new(file);

    // // JSON'u deserialize edin
    // let links_wrapper: LinksWrapper = match serde_json::from_reader(reader) {
    //     Ok(wrapper) => wrapper,
    //     Err(e) => {
    //         println!("JSON okuma hatası: {}", e);
    //         return;
    //     }
    // };

    // links_wrapper
// }

// I switched JSON file instead of generating Links structs.
// pub fn links_generator(amounts: u8) -> Vec<Links> {
//     let mut vecc: Vec<Links> = Vec::new();
//     for gen in 0..amounts {
//         let links = Links {
//             id: gen as u32, href: "href ".to_string() + format!("{}", gen).as_str(), text: "text ".to_string() + gen.to_string().as_str()
//         };
//         vecc.push(links);
//     }
//     vecc
// }

