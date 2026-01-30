// Example: File to FTM Entity Mapping
//
// This example demonstrates how to walk a file or folder and create
// appropriate FTM entities (File, Image, Pages, Audio, Video, etc.)
// based on the mime type of each file.
//
// Run with: cargo run --example file_entity_mapping <path>
//
// The output is JSONL format (one JSON entity per line) which can be
// piped to other FTM-compatible tools.

use ftm_types::generated::*;
use sha1::{Digest, Sha1};
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write as IoWrite};
use std::path::Path;
use walkdir::WalkDir;

/// Determines the appropriate FTM entity type based on mime type
fn mime_to_entity_type(mime: &str) -> &'static str {
    let main_type = mime.split('/').next().unwrap_or("");
    let full_type = mime;

    match main_type {
        "image" => "Image",
        "audio" => "Audio",
        "video" => "Video",
        "text" => match full_type {
            "text/html" | "text/xhtml" => "HyperText",
            "text/csv" | "text/tab-separated-values" => "Table",
            _ => "PlainText",
        },
        "application" => match full_type {
            // PDF and document formats -> Pages
            "application/pdf" => "Pages",
            "application/msword"
            | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            | "application/vnd.oasis.opendocument.text"
            | "application/rtf" => "Pages",

            // Presentations -> Pages
            "application/vnd.ms-powerpoint"
            | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
            | "application/vnd.oasis.opendocument.presentation" => "Pages",

            // Spreadsheets -> Workbook
            "application/vnd.ms-excel"
            | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            | "application/vnd.oasis.opendocument.spreadsheet" => "Workbook",

            // HTML
            "application/xhtml+xml" => "HyperText",

            // Archives and packages
            "application/zip"
            | "application/x-tar"
            | "application/gzip"
            | "application/x-7z-compressed"
            | "application/x-rar-compressed" => "Package",

            // JSON and structured data
            "application/json" | "application/xml" => "PlainText",

            // Default for other application types
            _ => "Document",
        },
        _ => "Document",
    }
}

/// Compute SHA1 hash of a file
fn compute_sha1(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha1::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Generate a stable ID from the file path
fn generate_id(path: &Path) -> String {
    let path_str = path.to_string_lossy();
    let mut hasher = Sha1::new();
    hasher.update(path_str.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Create an FTM entity from a file path
fn create_entity_from_file(path: &Path, parent_id: Option<&str>) -> io::Result<FtmEntity> {
    let metadata = fs::metadata(path)?;
    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let id = generate_id(path);

    // Handle directories as Folder entities
    if metadata.is_dir() {
        let mut folder = Folder::new(&id);
        folder.name = Some(vec![file_name]);
        folder.source_url = Some(vec![path.to_string_lossy().to_string()]);
        if let Some(parent) = parent_id {
            folder.parent = Some(vec![parent.to_string()]);
        }
        return Ok(FtmEntity::Folder(folder));
    }

    // Detect mime type
    let mime_type = mime_guess::from_path(path)
        .first()
        .map(|m| m.to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let entity_type = mime_to_entity_type(&mime_type);

    // Get file size and content hash
    let file_size = metadata.len() as f64;
    let content_hash = compute_sha1(path).ok();

    // Create the appropriate entity type
    let entity = match entity_type {
        "Image" => {
            let mut image = Image::new(&id);
            image.name = Some(vec![file_name]);
            image.mime_type = Some(vec![mime_type]);
            image.file_size = Some(vec![file_size]);
            image.content_hash = content_hash.map(|h| vec![h]);
            image.source_url = Some(vec![path.to_string_lossy().to_string()]);
            if let Some(parent) = parent_id {
                image.parent = Some(vec![parent.to_string()]);
            }
            FtmEntity::Image(image)
        }
        "Audio" => {
            let mut audio = Audio::new(&id);
            audio.name = Some(vec![file_name]);
            audio.mime_type = Some(vec![mime_type]);
            audio.file_size = Some(vec![file_size]);
            audio.content_hash = content_hash.map(|h| vec![h]);
            audio.source_url = Some(vec![path.to_string_lossy().to_string()]);
            if let Some(parent) = parent_id {
                audio.parent = Some(vec![parent.to_string()]);
            }
            FtmEntity::Audio(audio)
        }
        "Video" => {
            let mut video = Video::new(&id);
            video.name = Some(vec![file_name]);
            video.mime_type = Some(vec![mime_type]);
            video.file_size = Some(vec![file_size]);
            video.content_hash = content_hash.map(|h| vec![h]);
            video.source_url = Some(vec![path.to_string_lossy().to_string()]);
            if let Some(parent) = parent_id {
                video.parent = Some(vec![parent.to_string()]);
            }
            FtmEntity::Video(video)
        }
        "Pages" => {
            let mut pages = Pages::new(&id);
            pages.name = Some(vec![file_name]);
            pages.mime_type = Some(vec![mime_type]);
            pages.file_size = Some(vec![file_size]);
            pages.content_hash = content_hash.map(|h| vec![h]);
            pages.source_url = Some(vec![path.to_string_lossy().to_string()]);
            if let Some(parent) = parent_id {
                pages.parent = Some(vec![parent.to_string()]);
            }
            FtmEntity::Pages(pages)
        }
        "HyperText" => {
            let mut hypertext = HyperText::new(&id);
            hypertext.name = Some(vec![file_name]);
            hypertext.mime_type = Some(vec![mime_type]);
            hypertext.file_size = Some(vec![file_size]);
            hypertext.content_hash = content_hash.map(|h| vec![h]);
            hypertext.source_url = Some(vec![path.to_string_lossy().to_string()]);
            if let Some(parent) = parent_id {
                hypertext.parent = Some(vec![parent.to_string()]);
            }
            FtmEntity::HyperText(hypertext)
        }
        "PlainText" => {
            let mut plaintext = PlainText::new(&id);
            plaintext.name = Some(vec![file_name]);
            plaintext.mime_type = Some(vec![mime_type]);
            plaintext.file_size = Some(vec![file_size]);
            plaintext.content_hash = content_hash.map(|h| vec![h]);
            plaintext.source_url = Some(vec![path.to_string_lossy().to_string()]);
            if let Some(parent) = parent_id {
                plaintext.parent = Some(vec![parent.to_string()]);
            }
            FtmEntity::PlainText(plaintext)
        }
        "Table" => {
            let mut table = Table::new(&id);
            table.name = Some(vec![file_name]);
            table.mime_type = Some(vec![mime_type]);
            table.file_size = Some(vec![file_size]);
            table.content_hash = content_hash.map(|h| vec![h]);
            table.source_url = Some(vec![path.to_string_lossy().to_string()]);
            if let Some(parent) = parent_id {
                table.parent = Some(vec![parent.to_string()]);
            }
            FtmEntity::Table(table)
        }
        "Workbook" => {
            let mut workbook = Workbook::new(&id);
            workbook.name = Some(vec![file_name]);
            workbook.mime_type = Some(vec![mime_type]);
            workbook.file_size = Some(vec![file_size]);
            workbook.content_hash = content_hash.map(|h| vec![h]);
            workbook.source_url = Some(vec![path.to_string_lossy().to_string()]);
            if let Some(parent) = parent_id {
                workbook.parent = Some(vec![parent.to_string()]);
            }
            FtmEntity::Workbook(workbook)
        }
        "Package" => {
            let mut package = Package::new(&id);
            package.name = Some(vec![file_name]);
            package.mime_type = Some(vec![mime_type]);
            package.file_size = Some(vec![file_size]);
            package.content_hash = content_hash.map(|h| vec![h]);
            package.source_url = Some(vec![path.to_string_lossy().to_string()]);
            if let Some(parent) = parent_id {
                package.parent = Some(vec![parent.to_string()]);
            }
            FtmEntity::Package(package)
        }
        // Default: Document
        _ => {
            let mut document = Document::new(&id);
            document.name = Some(vec![file_name]);
            document.mime_type = Some(vec![mime_type]);
            document.file_size = Some(vec![file_size]);
            document.content_hash = content_hash.map(|h| vec![h]);
            document.source_url = Some(vec![path.to_string_lossy().to_string()]);
            if let Some(parent) = parent_id {
                document.parent = Some(vec![parent.to_string()]);
            }
            FtmEntity::Document(document)
        }
    };

    Ok(entity)
}

/// Walk a path and emit FTM entities for all files and folders
fn walk_and_emit_entities(root_path: &Path) -> io::Result<()> {
    let mut stdout = io::stdout().lock();

    // Track parent IDs for nested folders
    let mut path_to_id: std::collections::HashMap<std::path::PathBuf, String> =
        std::collections::HashMap::new();

    for entry in WalkDir::new(root_path).follow_links(false) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Warning: Could not access path: {}", e);
                continue;
            }
        };

        let path = entry.path();
        let id = generate_id(path);

        // Determine parent ID
        let parent_id = path.parent().and_then(|p| path_to_id.get(p).cloned());

        // Store this path's ID for child lookups
        path_to_id.insert(path.to_path_buf(), id.clone());

        // Create and emit the entity
        match create_entity_from_file(path, parent_id.as_deref()) {
            Ok(entity) => {
                let json = serde_json::to_string(&entity).expect("Failed to serialize entity");
                writeln!(stdout, "{}", json)?;
            }
            Err(e) => {
                eprintln!("Warning: Could not process {}: {}", path.display(), e);
            }
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        eprintln!();
        eprintln!("Walks a file or folder and creates FTM entities based on mime type.");
        eprintln!("Output is JSONL format (one JSON entity per line).");
        eprintln!();
        eprintln!("Entity types created:");
        eprintln!("  - Folder     : directories");
        eprintln!("  - Image      : image/* mime types");
        eprintln!("  - Audio      : audio/* mime types");
        eprintln!("  - Video      : video/* mime types");
        eprintln!("  - Pages      : PDFs, Word docs, presentations");
        eprintln!("  - Workbook   : Excel, spreadsheets");
        eprintln!("  - Table      : CSV, TSV files");
        eprintln!("  - HyperText  : HTML files");
        eprintln!("  - PlainText  : text files, JSON, XML");
        eprintln!("  - Package    : archives (zip, tar, etc.)");
        eprintln!("  - Document   : other files (fallback)");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);

    if !path.exists() {
        eprintln!("Error: Path does not exist: {}", path.display());
        std::process::exit(1);
    }

    if let Err(e) = walk_and_emit_entities(path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
