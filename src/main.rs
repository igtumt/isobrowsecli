use anyhow::Result;
use std::env;
use std::io::Read;
use std::fs;
use std::path::Path;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;

/// Generates a unique filename for a cached WASM module based on its URL hash.
fn get_url_hash(url: &str) -> String {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    format!("{:x}.wasm", hasher.finish())
}

fn main() -> Result<()> {
    // 1. Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 || args[1] != "run" {
        eprintln!("Usage:");
        eprintln!("  1. Direct input: iso run <wasm_path_or_url> <input>");
        eprintln!("  2. Via Stdin (Pipe): cat file.txt | iso run <wasm_path_or_url>");
        return Ok(());
    }

    let wasm_source = &args[2];

    let mut wasi_args = vec![wasm_source.to_string()];
    if args.len() > 3 {
        wasi_args.push(args[3].to_string());
    }

    let engine = Engine::default();

    // 2. Build WASI context inheriting standard IO
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .args(&wasi_args)? 
        .build();

    // 3. Initialize Store and Linker
    let mut store = Store::new(&engine, wasi);
    let mut linker = Linker::new(&engine);
    
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // 4. Load WASM module from remote URL (cached) or local filesystem
    let module = if wasm_source.starts_with("http://") || wasm_source.starts_with("https://") {
        let cache_dir = Path::new(".isocache");
        let cache_filename = get_url_hash(wasm_source);
        let cache_path = cache_dir.join(&cache_filename);

        if cache_path.exists() {
            // CACHE HIT: Load directly from local disk cache
            Module::from_file(&engine, &cache_path)?
        } else {
            // CACHE MISS: Fetch from URL and save to cache
            println!("Fetching WASM module from URL...");
            
            fs::create_dir_all(cache_dir)?;

            let response = ureq::get(wasm_source).call()?;
            let mut reader = response.into_reader();
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes)?;

            fs::write(&cache_path, &bytes)?;

            Module::from_binary(&engine, &bytes)?
        }
    } else {
        Module::from_file(&engine, wasm_source)?
    };

    // 5. Link module and resolve default entrypoint (_start)
    linker.module(&mut store, "", &module)?;
    let func = linker.get_default(&mut store, "")?.typed::<(), ()>(&store)?;
    
    // 6. Execute WASM module and capture WASI exit status
    if let Err(trap) = func.call(&mut store, ()) {
        let error_msg = trap.root_cause().to_string();

        // Intercept WASI exit status codes and exit gracefully
        if error_msg.starts_with("Exited with i32 exit status ") {
            if let Ok(code) = error_msg.replace("Exited with i32 exit status ", "").parse::<i32>() {
                std::process::exit(code);
            }
        }
        
        // Propagate unhandled runtime traps (e.g. memory out of bounds)
        return Err(trap);
    }

    Ok(())
}
