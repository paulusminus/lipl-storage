use std::ffi::{OsString};
use std::path::{PathBuf};
use quote::{quote};
use quote::__private::TokenStream;
use rust_format::{Formatter, RustFmt};

const SOURCE_PATH: &str = "sample_data";
const DATA_FILENAME: &str = "sample_data.rs";

fn has_extension(extension: &str) -> impl Fn(&PathBuf) -> bool + '_ {
    move |p| p.is_file() && p.extension().map(|s| s.to_ascii_lowercase()) == Some(OsString::from(extension))
}

fn is_dir() -> impl Fn(&PathBuf) -> bool
{
    |path| path.is_dir()
}

fn dir_entries<F>(path: &str, predicate: F) -> std::io::Result<Vec<PathBuf>>
where
    F: FnMut(&PathBuf) -> bool,
{
    std::fs::read_dir(path)
    .map(|list| {
        list
            .into_iter()
            .filter_map(|f| f.ok())
            .map(|de| de.path())
            .filter(predicate)
            .collect::<Vec<PathBuf>>()
    })
}

fn create_lyrics(lyric_files: &[PathBuf], playlists: Vec<(String, Vec<String>)>) -> TokenStream {
    let playlist_paths = playlists
        .iter()
        .map(|(playlist_title, member_titles)| {
            quote!(
                Playlist::from(
                    (
                        None,
                        PlaylistPost {
                            title: #playlist_title.to_owned(),
                            members: Vec::from_iter([#(#member_titles),*])
                                .into_iter()
                                .map(|title| lyrics.iter().find(|lyric| lyric.title == *title).unwrap())
                                .map(|lyric| lyric.id)
                                .collect::<Vec<_>>(),
                        }
                    )
                )
            )
    });
    let lyric_paths = lyric_files
        .iter()
        .map(|path| {
            let title = path.file_stem().unwrap().to_string_lossy().to_string();
            let file_path = path.to_string_lossy().to_string();
        
            quote! {
                Lyric::from(
                    (
                        None, 
                        LyricPost {
                            title: #title.to_owned(),
                            parts: to_parts(include_str!(#file_path).to_owned()),
                        }
                    )
                )
            }
    });
    quote! {
        pub fn repo_db() -> RepoDb {
            let lyrics = Vec::from_iter([
                #(#lyric_paths),*
            ]);
            let playlists = Vec::from_iter([
                #(#playlist_paths),*
            ]);
            RepoDb { 
                lyrics,
                playlists,
            }
        }
    }
}

fn source_gen(hashmap: TokenStream) -> TokenStream {
    quote! {
        use lipl_core::{Lyric, LyricPost, Playlist, PlaylistPost, RepoDb};
        use parts::{to_parts};
 
        /// This function returns all lyrics from a directory read at build time.
        #hashmap
    }
} 

fn main() {
    let source_path = std::env::current_dir().unwrap().join(SOURCE_PATH);
    let text_files = dir_entries(&source_path.to_string_lossy(), has_extension("txt")).unwrap();
    let playlists = 
        dir_entries(SOURCE_PATH, is_dir())
        .unwrap()
        .into_iter()
        .map(|pathbuf| 
            (
                pathbuf.file_stem().unwrap().to_string_lossy().to_string(),
                dir_entries(pathbuf.to_str().unwrap(), has_extension("txt"))
                    .unwrap()
                    .into_iter()
                    .map(|p| p.file_stem().unwrap().to_string_lossy().to_string())
                    .collect::<Vec<_>>()
            )
        )
        .collect::<Vec<_>>();

    let hashmap = create_lyrics(text_files.as_slice(), playlists);

    let tokens = source_gen(hashmap);
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("out dir: {}", out_dir);
    let path = PathBuf::from(out_dir).join(DATA_FILENAME);

    if let Err(error) = std::fs::write(path.as_path(), RustFmt::default().format_tokens(tokens).unwrap()) {
        panic!("Error writing file {}: {}", path.to_string_lossy(), error);
    }
}