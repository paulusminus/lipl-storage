use std::str::{FromStr};
use std::fmt::{Display, Formatter};
use std::str::Lines;
use crate::{Etag, Lyric, LyricMeta, LyricPost, PlaylistPost, Without};
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
        let playlist_post: PlaylistPost = serde_yaml::from_str(s)?;
        Ok(playlist_post)
    }
}

impl Display for PlaylistPost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let yaml = serde_yaml::to_string(self).unwrap();
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

    const HERTOG_JAN_LYRIC: &str = include_str!("../../data/Gx1dZeoikQKRyyDy1aru6f.txt");
    const HERTOG_JAN_TITLE: &str = "Hertog Jan";
    const HERTOG_JAN_ID: &str = "Gx1dZeoikQKRyyDy1aru6f";
    const KERST_PLAYLIST: &str = include_str!("../../data/WGoDqF1jC3zZxqkQRr2ceA.yaml");
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
    fn playlist_post_parse_equals_display() {
        let playlist_post: PlaylistPost = KERST_PLAYLIST.parse().unwrap();
        assert_eq!(playlist_post.to_string().as_str(), KERST_PLAYLIST);
    }

    #[test]
    fn lyric_meta_parse() {
        let lyric_meta: LyricMeta = HERTOG_JAN_LYRIC.parse().unwrap();
        assert_eq!(lyric_meta.title, HERTOG_JAN_TITLE.to_owned());
        assert_eq!(lyric_meta.hash, Some("\"2530-337251511557883259562065364316662953368\"".to_owned()));
    }
}