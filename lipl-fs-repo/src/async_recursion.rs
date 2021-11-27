#[async_recursion]
async fn lines_to_parts(acc: LyricPost, mut lines: std::pin::Pin<Box<dyn Stream<Item=Result<String>> + Send>>) -> Result<LyricPost> 
{
    let next: Vec<String> = 
        lines
        .by_ref()
        .map_ok(|l| l.trim().to_owned())
        .try_skip_while(|l| ready(Ok(l == "")))
        .try_take_while(|l| ready(Ok(l != "")))
        .try_collect()
        .await?;
    
    if next.len() == 0 {
        Ok(acc)
    }
    else if next.first() == Some(&"---".to_owned()) {
        let new: Vec<String> = next.into_iter().filter(|s| s != "---").collect();
        let meta: LyricMeta = serde_yaml::from_str(&new.join("\n"))?;
        lines_to_parts(
            LyricPost {
                title: meta.title,
                parts: acc.parts,
            },
            lines
        )
        .await
    }
    else {
        let mut new_acc = acc.parts.clone();
        new_acc.push(next);
        lines_to_parts(
            LyricPost {
                title: acc.title,
                parts: new_acc,
            },
            lines
        )
        .await
    }
}
