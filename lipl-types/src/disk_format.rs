use std::str::{FromStr};
use std::fmt::{Display, Formatter};
use std::str::Lines;
use crate::{Etag, Lyric, LyricMeta, LyricPost, PlaylistPost, Without, Playlist};
use crate::error::{RepoError};

const YAML_PREFIX: &str = "---";

fn lines_to_lyric_post(acc: LyricPost, mut lines: Lines) -> Result<LyricPost, serde_yaml::Error>
{
    let next = 
        lines
        .by_ref()
        .skip_while(
            |l| l.trim().is_empty()
        )
        .take_while(
            |l| !l.trim().is_empty()
        )
        .map(String::from)
        .collect::<Vec<_>>();

    if next.is_empty() {
        Ok(acc)
    }
    else if next.first().map(|s| s.trim()) == Some(YAML_PREFIX) {
        let new = next.without(&YAML_PREFIX.to_owned());
        let meta: LyricMeta = serde_yaml::from_str(&new.join("\n"))?;
        lines_to_lyric_post(
            LyricPost {
                title: meta.title,
                parts: acc.parts,
            },
            lines
        )
    }
    else {
        let mut new_acc = acc.parts.clone();
        new_acc.push(next);
        lines_to_lyric_post(
            LyricPost {
                title: acc.title,
                parts: new_acc,
            },
            lines
        )
    }
}

impl FromStr for LyricPost {
    type Err = RepoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lyric_post = lines_to_lyric_post(Default::default(), s.lines())?;
        Ok(lyric_post)
    }
}

impl Display for Lyric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lyric_meta = LyricMeta {
            title: self.title.clone(),
            hash: self.etag(),
        };
        let yaml = serde_yaml::to_string(&lyric_meta).unwrap();
        let parts_string: String = self.parts.iter().map(|p| p.join("\n")).collect::<Vec<_>>().join("\n\n");
        write!(f, "{}---\n\n{}", yaml, parts_string)
    }
}

impl FromStr for PlaylistPost {
    type Err = RepoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str::<PlaylistPost>(s).map_err(RepoError::from)
    }
}

impl Display for Playlist {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let playlist_post = PlaylistPost::from(self.clone());
        let yaml = serde_yaml::to_string(&playlist_post).unwrap_or_default();
        write!(f, "{}", yaml)
    }
}

fn empty_line(s: &&str) -> bool {
    s.trim().is_empty()
}

fn non_empty_line(s: &&str) -> bool {
    !empty_line(s)
}

impl FromStr for LyricMeta {
    type Err = RepoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let next = 
            lines
            .skip_while(empty_line)
            .take_while(non_empty_line)
            .map(String::from)
            .collect::<Vec<_>>();

        let yaml: Vec<String> = next.without(&YAML_PREFIX.to_owned());
        let lyric_meta: LyricMeta = serde_yaml::from_str(&yaml.join("\n"))?;
        Ok(lyric_meta)
    }
}

#[cfg(test)]
mod tests {

    use super::{Lyric, LyricMeta, LyricPost, PlaylistPost};
    use crate::{Uuid};

    const HERTOG_JAN_LYRIC: &str = r#"---
    title: Toen Hertog Jan kwam varen
    hash: "\"2546-135734770857133596616985852360825759697\""
    ---
    
    Toen den hertog Jan kwam varen
    Te peerd parmant, al triumfant
    Na zevenhonderd jaren
    Hoe zong men 't allen kant
    Harba lorifa, zong den Hertog, harba lorifa,
    Na zevenhonderd jaren
    In dit edel Brabants land
    
    Hij kwam van over 't water
    Den Scheldevloed, aan wal te voet
    't Antwerpen op de straten
    Zilv'ren veren op zijn hoed
    Harba lorifa, zong den Hertog, harba lorifa
    't Antwerpen op de straten
    Lere leerzen aan zijn voet
    
    Och Turnhout, stedeke schone
    Zijn uw ruitjes groen, maar uw hertjes koen
    Laat den Hertog binnen komen
    In dit zomers vrolijk seizoen
    Harba lorifa, zong den Hertog, harba lorifa
    Laat den Hertog binnen komen
    Hij heeft een peerd van doen
    
    Hij heeft een peerd gekregen
    Een schoon wit peerd, een schimmelpeerd
    Daar is hij opgestegen
    Dien ridder onverveerd
    Harba lorifa, zong den Hertog, harba lorifa
    Daar is hij opgestegen
    En hij reed naar Valkensweerd
    
    In Valkensweerd daar zaten
    Al in de kast, de zilverkast
    De guldenkoning zijn platen
    Die wierd' aaneen gelast
    Harba lorifa, zong den Hertog, harba lorifa
    De guldenkoning zijn platen
    Toen had hij een harnas
    
    Rooise boeren, komt naar buiten
    Met de grote trom, met de kleine trom
    Trompetten en cornetten en de fluiten
    Want den Hertog komt weerom
    Harba lorifa, zong den Hertog, harba lorifa
    Trompetten en cornetten en de fluiten
    In dit Brabants Hertogdom
    
    Wij reden allemaal samen
    Op Oirschot aan, door een Kanidasselaan
    En Jan riep: "In Geus name!
    Hier heb ik meer gestaan."
    Harba lorifa, zong den Hertog, harba lorifa
    En Jan riep: "In Geus name!
    Reikt mij mijn standaard aan!"
    
    De standaard was de gouwe
    Die waaide dan, die draaide dan
    Die droeg de leeuw met klauwen
    Wij zongen alleman
    Harba lorifa, zong den Hertog, harba lorifa
    Die droeg de leeuw met klauwen
    Ja, de Leeuw van Hertog Jan
    
    Hij is in Den Bosch gekomen
    Al in de nacht, niemand die 't zag
    En op Sint Jan geklommen
    Daar ging hij staan op wacht
    Harba lorifa, zong den Hertog, harba lorifa
    En op Sint Jan geklommen
    Daar staat hij dag en nacht"#;
    const HERTOG_JAN_TITLE: &str = "Hertog Jan";
    const HERTOG_JAN_ID: &str = "T2NPjHifDf1E1UfZZA6TDB";
    const KERST_PLAYLIST: &str = r#"---
    title: Kerst
    members:
      - FyAvpSWaLQmcDaYZxwXe44
      - GF5kHMvngVyALVcj7imopi
      - KGxasqUC1Uojk1viLGbMZK
      - SdbM6j9uCtNiGRUW1hiTz5
    "#;
    const PLAYLIST_TEXT: &str = "---\ntitle: Kerst\nmembers:\n  - FyAvpSWaLQmcDaYZxwXe44\n  - GF5kHMvngVyALVcj7imopi\n  - SdbM6j9uCtNiGRUW1hiTz5\n";
    const PLAYLIST_TITLE: &str = "Kerst";
    const PLAYLIST_MEMBER1: &str = "FyAvpSWaLQmcDaYZxwXe44";
    const PLAYLIST_MEMBER2: &str = "GF5kHMvngVyALVcj7imopi";
    const PLAYLIST_MEMBER3: &str = "SdbM6j9uCtNiGRUW1hiTz5";

    #[test]
    fn playlist_post_parse() {
        let playlist_post: PlaylistPost = PLAYLIST_TEXT.parse().unwrap();
        assert_eq!(playlist_post.title, PLAYLIST_TITLE.to_owned());
        assert_eq!(playlist_post.members.len(), 3);
        assert_eq!(playlist_post.members[0].to_string(), PLAYLIST_MEMBER1.to_owned());
        assert_eq!(playlist_post.members[1].to_string(), PLAYLIST_MEMBER2.to_owned());
        assert_eq!(playlist_post.members[2].to_string(), PLAYLIST_MEMBER3.to_owned());
    }

    #[test]
    fn lyric_post_parse() {
        let lyric_post: LyricPost = HERTOG_JAN_LYRIC.parse().unwrap();

        assert_eq!(lyric_post.title, HERTOG_JAN_TITLE.to_owned());
        assert_eq!(lyric_post.parts.len(), 9);
    }

    #[test]
    fn lyric_post_parse_equals_display() {
        let lyric_post: LyricPost = HERTOG_JAN_LYRIC.parse().unwrap();
        let lyric = Lyric::from((lyric_post, HERTOG_JAN_ID.to_owned().parse::<Uuid>().unwrap()));
        assert_eq!(lyric.to_string().as_str(), HERTOG_JAN_LYRIC);
    }

    #[test]
    fn lyric_meta_parse() {
        let lyric_meta: LyricMeta = HERTOG_JAN_LYRIC.parse().unwrap();
        assert_eq!(lyric_meta.title, HERTOG_JAN_TITLE.to_owned());
        assert_eq!(lyric_meta.hash, Some("\"2530-337251511557883259562065364316662953368\"".to_owned()));
    }
}