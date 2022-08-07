use lipl_types::{Lyric, Uuid};

fn hertog_jan_lyric() -> Lyric {
    Lyric {
        id: "T2NPjHifDf1E1UfZZA6TDB".parse::<Uuid>().unwrap(),
        title: "Hertog Jan".to_owned(),
        parts: vec![
            vec![
                "Toen den hertog Jan kwam varen".to_owned(),
                "Te peerd parmant, al triumfant".to_owned(),
                "Na zevenhonderd jaren".to_owned(),
                "Hoe zong men 't allen kant".to_owned(),
                "Harba lorifa, zong den Hertog, harba lorifa,".to_owned(),
                "Na zevenhonderd jaren".to_owned(),
                "In dit edel Brabants land".to_owned(),
            ],
            vec![
                "Hij kwam van over 't water".to_owned(),
                "Den Scheldevloed, aan wal te voet".to_owned(),
                "'t Antwerpen op de straten".to_owned(),
                "Zilv'ren veren op zijn hoed".to_owned(),
                "Harba lorifa, zong den Hertog, harba lorifa".to_owned(),
                "'t Antwerpen op de straten".to_owned(),
                "Lere leerzen aan zijn voet".to_owned(),   
            ],
            vec![
                "Och Turnhout, stedeke schone".to_owned(),
                "Zijn uw ruitjes groen, maar uw hertjes koen".to_owned(),
                "Laat den Hertog binnen komen".to_owned(),
                "In dit zomers vrolijk seizoen".to_owned(),
                "Harba lorifa, zong den Hertog, harba lorifa".to_owned(),
                "Laat den Hertog binnen komen".to_owned(),
                "Hij heeft een peerd van doen".to_owned(),
            ],
            vec![
                "Hij heeft een peerd gekregen".to_owned(),
                "Een schoon wit peerd, een schimmelpeerd".to_owned(),
                "Daar is hij opgestegen".to_owned(),
                "Dien ridder onverveerd".to_owned(),
                "Harba lorifa, zong den Hertog, harba lorifa".to_owned(),
                "Daar is hij opgestegen".to_owned(),
                "En hij reed naar Valkensweerd".to_owned(),
            ],
            vec![
                "In Valkensweerd daar zaten".to_owned(),
                "Al in de kast, de zilverkast".to_owned(),
                "De guldenkoning zijn platen".to_owned(),
                "Die wierd' aaneen gelast".to_owned(),
                "Harba lorifa, zong den Hertog, harba lorifa".to_owned(),
                "De guldenkoning zijn platen".to_owned(),
                "Toen had hij een harnas".to_owned(),
            ],
            vec![
                "Rooise boeren, komt naar buiten".to_owned(),
                "Met de grote trom, met de kleine trom".to_owned(),
                "Trompetten en cornetten en de fluiten".to_owned(),
                "Want den Hertog komt weerom".to_owned(),
                "Harba lorifa, zong den Hertog, harba lorifa".to_owned(),
                "Trompetten en cornetten en de fluiten".to_owned(),
                "In dit Brabants Hertogdom".to_owned(),
            ],
            vec![
                "Wij reden allemaal samen".to_owned(),
                "Op Oirschot aan, door een Kanidasselaan".to_owned(),
                "En Jan riep: \"In Geus name!\"".to_owned(),
                "Hier heb ik meer gestaan.".to_owned(),
                "Harba lorifa, zong den Hertog, harba lorifa".to_owned(),
                "En Jan riep: \"In Geus name!\"".to_owned(),
                "Reikt mij mijn standaard aan!".to_owned(),
            ],
            vec![
                "De standaard was de gouwe".to_owned(),
                "Die waaide dan, die draaide dan".to_owned(),
                "Die droeg de leeuw met klauwen".to_owned(),
                "Wij zongen alleman".to_owned(),
                "Harba lorifa, zong den Hertog, harba lorifa".to_owned(),
                "Die droeg de leeuw met klauwen".to_owned(),
                "Ja, de Leeuw van Hertog Jan".to_owned(),
            ],
            vec![
                "Hij is in Den Bosch gekomen".to_owned(),
                "Al in de nacht, niemand die 't zag".to_owned(),
                "En op Sint Jan geklommen".to_owned(),
                "Daar ging hij staan op wacht".to_owned(),
                "Harba lorifa, zong den Hertog, harba lorifa".to_owned(),
                "En op Sint Jan geklommen".to_owned(),
                "Daar staat hij dag en nacht".to_owned(),
            ]
        ]
    }
}


fn main() {
    println!("{}", hertog_jan_lyric().to_string());
}