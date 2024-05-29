use walkdir::WalkDir;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send >>{
    
    for entry in WalkDir::new("linux")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
        let f_name = String::from(entry.path().to_str().unwrap());
        println!("Processing file: {}", f_name);
        // read file & replace all "a" with "ä" in the file
        let content = tokio::fs::read_to_string(f_name.clone()).await?;
        let new_content = content.replace("a", "ä");
        // write new content to different folder
        let new_path = f_name.replace("linux", "result");
        tokio::fs::write(new_path, new_content).await?;

    }
    Ok(())
}