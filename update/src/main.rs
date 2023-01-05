fn main() {
    let version = std::env::args()
        .nth(1)
        .expect("version argument not specified");

    let version = version.strip_prefix('v').unwrap_or(&version);

    let zip = reqwest::blocking::get(format!(
        "https://github.com/googleforgames/agones/archive/refs/tags/v{}.zip",
        version
    ))
    .expect("failed to send source.zip request")
    .bytes()
    .expect("failed to receive source.zip");

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(zip)).expect("failed to read zip");

    let mut protos = Vec::with_capacity(10);

    let proto_prefix = format!("agones-{}/proto/", version);

    for ind in 0..archive.len() {
        let raw = archive
            .by_index_raw(ind)
            .expect("failed to get zip file at index");

        if raw.name().starts_with(&proto_prefix) && raw.is_file() {
            protos.push(ind);
        }
    }

    let extract_dir = {
        let out = std::process::Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()
            .expect("failed to exec git")
            .stdout;

        let mut repo_root = String::from_utf8(out).expect("invalid utf-8 output");

        // Remove trailing newline
        repo_root.pop();
        repo_root.push_str("/src/proto");

        std::path::PathBuf::from(repo_root)
    };

    let _ = std::fs::remove_dir_all(&extract_dir);

    for ind in protos {
        let mut proto = archive
            .by_index(ind)
            .expect("failed to read proto from zip");

        let path = proto
            .name()
            .strip_prefix(&proto_prefix)
            .expect("not a valid proto path");

        let tar_path = extract_dir.join(path);

        println!("extracting {}", tar_path.display());

        let dir = tar_path.parent().unwrap();
        std::fs::create_dir_all(dir).expect("failed to create directory");

        let mut tar_file = std::fs::File::create(&tar_path).expect("failed to create file");

        std::io::copy(&mut proto, &mut tar_file).expect("failed to copy proto to file");
    }

    // We also need to get the protobuf...protobuf since it's an import, but
    // isn't included in the agones protobufs. ARGH
    // Note we just pull the latest bleeding edge here, I guess this will be
    // fine until it isn't, but shouldn't be an issue since google is great at
    // backwards compat :troll:
    let descriptor_proto = reqwest::blocking::get("https://raw.githubusercontent.com/protocolbuffers/protobuf/main/src/google/protobuf/descriptor.proto")
        .expect("failed to send descriptor request")
        .bytes()
        .expect("failed to receive descriptor.proto");

    let descriptor_path = extract_dir.join("googleapis/google/protobuf/descriptor.proto");
    std::fs::create_dir_all(descriptor_path.parent().unwrap())
        .expect("failed to create googleapis/google/protobuf");
    std::fs::write(descriptor_path, descriptor_proto).expect("failed to write descriptor.proto");

    let generated = extract_dir.parent().unwrap().join("generated");

    if generated.exists() {
        std::fs::remove_dir_all(&generated).expect("failed to remove generated");
    }

    std::fs::create_dir_all(&generated).expect("failed to create generated dir");

    tonic_build::configure()
        // The SDK is just a client, no need to build the server types
        .build_server(false)
        .out_dir(&generated)
        .compile(
            &[
                extract_dir.join("sdk/alpha/alpha.proto"),
                extract_dir.join("sdk/sdk.proto"),
            ],
            &[
                extract_dir.join("googleapis"),
                extract_dir.join("sdk/alpha"),
                extract_dir.join("sdk"),
            ],
        )
        .expect("failed to compile protobuffers");
}
