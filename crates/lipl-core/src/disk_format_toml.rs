use core::fmt::{Display, Formatter};
use core::iter::once;
use core::str::FromStr;
use core::str::Lines;

use crate::error::Error;
use crate::vec_ext::VecExt;
use crate::{Etag, Lyric, LyricMeta, LyricPost, Playlist, PlaylistPost, TOML_PREFIX};

fn lines_to_lyric_post(
    acc: LyricPost,
    mut lines: Lines,
) -> Result<LyricPost, toml_edit::de::Error> {
    let next = lines
        .by_ref()
        .skip_while(|l| l.trim().is_empty())
        .take_while(|l| !l.trim().is_empty())
        .map(|s| s.trim())
        .map(String::from)
        .collect::<Vec<_>>();

    if next.is_empty() {
        Ok(acc)
    } else if next.first().map(|s| s.trim()) == Some(TOML_PREFIX) {
        let new = next.without(&TOML_PREFIX.to_owned());
        let meta: LyricMeta = toml_edit::de::from_str(&new.join("\n"))?;
        lines_to_lyric_post(
            LyricPost {
                title: meta.title,
                parts: acc.parts,
            },
            lines,
        )
    } else {
        lines_to_lyric_post(
            LyricPost {
                title: acc.title,
                parts: acc.parts.into_iter().chain(once(next)).collect::<Vec<_>>(),
            },
            lines,
        )
    }
}

impl FromStr for LyricPost {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lyric_post = lines_to_lyric_post(Default::default(), s.lines())?;
        Ok(lyric_post)
    }
}

impl Display for Lyric {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let lyric_meta = LyricMeta {
            title: self.title.clone(),
            hash1: self.etag1(),
            hash2: self.etag2(),
        };
        let yaml = toml_edit::ser::to_string_pretty(&lyric_meta).unwrap();
        let parts_string: String = self
            .parts
            .iter()
            .map(|p| p.join("  \n"))
            .collect::<Vec<_>>()
            .join("\n\n");
        write!(f, "{TOML_PREFIX}\n{yaml}{TOML_PREFIX}\n\n{parts_string}")
    }
}

impl FromStr for PlaylistPost {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml_edit::de::from_str::<PlaylistPost>(s).map_err(Into::into)
    }
}

impl Display for Playlist {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let playlist_post = PlaylistPost::from(self.clone());
        let yaml = toml_edit::ser::to_string_pretty(&playlist_post).unwrap_or_default();
        write!(f, "{}", yaml)
    }
}

fn empty_line(s: &&str) -> bool {
    s.trim().is_empty()
}

fn non_empty_line(s: &&str) -> bool {
    !empty_line(s)
}

// pub struct LyricMetaWrapper(LyricMeta);

impl FromStr for LyricMeta {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let yaml = s
            .lines()
            .skip_while(empty_line)
            .take_while(non_empty_line)
            .map(String::from)
            .filter(|s| s != TOML_PREFIX)
            .collect::<Vec<_>>()
            .join("\n");

        toml_edit::de::from_str(&yaml).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {

    use super::{Lyric, LyricMeta, LyricPost, PlaylistPost};
    use crate::Uuid;
    use std::vec;

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
                ],
            ],
        }
    }

    const HERTOG_JAN_TITLE: &str = "Hertog Jan";
    const HERTOG_JAN_ID: &str = "T2NPjHifDf1E1UfZZA6TDB";
    const PLAYLIST_TEXT: &str = "\ntitle = \"Kerst\"\nmembers = [\"FyAvpSWaLQmcDaYZxwXe44\",  \"GF5kHMvngVyALVcj7imopi\", \"SdbM6j9uCtNiGRUW1hiTz5\"]\n";
    const PLAYLIST_TITLE: &str = "Kerst";
    const PLAYLIST_MEMBER1: &str = "FyAvpSWaLQmcDaYZxwXe44";
    const PLAYLIST_MEMBER2: &str = "GF5kHMvngVyALVcj7imopi";
    const PLAYLIST_MEMBER3: &str = "SdbM6j9uCtNiGRUW1hiTz5";

    #[test]
    fn playlist_post_parse() {
        let playlist_post: PlaylistPost = PLAYLIST_TEXT.parse().unwrap();
        assert_eq!(playlist_post.title, PLAYLIST_TITLE.to_owned());
        assert_eq!(playlist_post.members.len(), 3);
        assert_eq!(
            playlist_post.members[0].to_string(),
            PLAYLIST_MEMBER1.to_owned()
        );
        assert_eq!(
            playlist_post.members[1].to_string(),
            PLAYLIST_MEMBER2.to_owned()
        );
        assert_eq!(
            playlist_post.members[2].to_string(),
            PLAYLIST_MEMBER3.to_owned()
        );
    }

    #[test]
    fn lyric_post_parse() {
        let hertog_jan = hertog_jan_lyric().to_string();
        let lyric_post = hertog_jan.parse::<LyricPost>().unwrap();

        assert_eq!(lyric_post.title, HERTOG_JAN_TITLE.to_owned());
        assert_eq!(lyric_post.parts.len(), 9);
    }

    #[test]
    fn lyric_post_parse_equals_display() {
        let lyric_post: LyricPost = hertog_jan_lyric().to_string().parse().unwrap();
        println!(
            "{}",
            &toml_edit::ser::to_string_pretty(&lyric_post).unwrap()
        );
        let uuid = HERTOG_JAN_ID.to_owned().parse::<Uuid>().unwrap();
        let lyric = Lyric::from((Some(uuid), lyric_post));
        assert_eq!(
            lyric.to_string().as_str(),
            hertog_jan_lyric().to_string().as_str()
        );
    }

    #[test]
    fn lyric_meta_parse() {
        let lyric_meta: LyricMeta = hertog_jan_lyric().to_string().parse().unwrap();
        assert_eq!(lyric_meta.title, HERTOG_JAN_TITLE.to_owned());
        assert_eq!(
            lyric_meta.hash1,
            Some("\"2530-189459479300553739784561073837696755448\"".to_owned())
        );
        assert_eq!(
            lyric_meta.hash2,
            Some("\"2530-189459479300553739784561073837696755448\"".to_owned())
        );
    }

    #[test]
    fn display_playlist() {
        let playlist = PlaylistPost {
            title: "Kerst".to_owned(),
            members: vec![
                PLAYLIST_MEMBER1.parse::<Uuid>().unwrap(),
                PLAYLIST_MEMBER2.parse::<Uuid>().unwrap(),
                PLAYLIST_MEMBER3.parse::<Uuid>().unwrap(),
            ],
        };

        println!("{}", toml_edit::ser::to_string_pretty(&playlist).unwrap());
    }
}
