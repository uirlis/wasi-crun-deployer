use log::info;
use std::fs;
use std::io::Write;
use toml_edit::{value, Array, Document, Item, Table};

pub fn copy_to(
    vendor_base: &str,
    destination_base: &str,
    vendor: &str,
    file_name: &str,
) -> Result<(), std::io::Error> {
    let location = format!("/{}/{}/{}", vendor_base, vendor, file_name);
    let destination = format!("/{}/{}", destination_base, file_name);
    info!("Copying from {} to {}", location, destination);
    fs::copy(location, destination)?;
    Ok(())
}

pub fn update_containerd_config(path: &str) -> Result<toml_edit::Document, std::io::Error> {
    let conf = generate_containerd_config(path)?;
    let value: toml_edit::easy::Value =
        toml_edit::easy::from_str(conf.to_string().as_str()).unwrap();
    let result = toml_edit::easy::to_string_pretty(&value).unwrap();

    let destination = path.replace(".toml", ".toml.bak");
    info!("Copying from {} to {}", path, destination);
    fs::copy(path, destination)?;
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;
    f.write_all(result.as_bytes())?;
    f.flush()?;
    Ok(conf)
}

pub fn generate_containerd_config(path: &str) -> Result<toml_edit::Document, std::io::Error> {
    let content = std::fs::read_to_string(path)?;

    let mut conf = content.parse::<Document>().expect("invalid doc");

    let mut poda = Array::default();
    poda.push("*.wasm.*");
    poda.push("module.wasm.image/*");
    poda.push("*.module.wasm.image");
    poda.push("module.wasm.image/variant.*");

    let mut table = Table::default();
    table["runtime_type"] = value("io.containerd.runc.v2");
    table["privileged_without_host_devices"] = value(false);
    table["pod_annotations"] = value(poda);

    let mut opt_table = Table::default();
    opt_table["BinaryName"] = value("/usr/local/sbin/crun");
    conf["plugins"]["io.containerd.grpc.v1.cri"]["containerd"]["runtimes"]["crun"] =
        Item::Table(table);
    conf["plugins"]["io.containerd.grpc.v1.cri"]["containerd"]["runtimes"]["crun"]["options"] =
        Item::Table(opt_table);
    Ok(conf)
}

pub fn restore_containerd_config(path: &str) -> Result<(), std::io::Error> {
    let from = path.replace(".toml", ".toml.bak");
    info!("Copying from {} to {}", from, path);
    fs::copy(from, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn update_containerd_config_test() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/mocks/in_config.toml");
        let backup_path = concat!(env!("CARGO_MANIFEST_DIR"), "/mocks/in_config.toml.bak");
        let new_compare = concat!(env!("CARGO_MANIFEST_DIR"), "/mocks/out_config.toml");
        let old_file_contents =
            fs::read_to_string(path).expect("Should have been able to read the file");

        update_containerd_config(path).unwrap();

        let new_file_contents =
            fs::read_to_string(path).expect("Should have been able to read the new_file_contents");
        let new_compare_contents = fs::read_to_string(new_compare)
            .expect("Should have been able to read the new_compare_contents");
        let backup_file_contents = fs::read_to_string(backup_path)
            .expect("Should have been able to read the backup_file_contents");

        // Test the new file is as expected
        assert_eq!(
            new_file_contents, new_compare_contents,
            "Test the new file is as expected"
        );

        // Test the backup is part of the orginal file
        assert_eq!(
            old_file_contents, backup_file_contents,
            "Test the backup is part of the orginal file"
        );

        restore_containerd_config(path).unwrap();
        let restored_file_contents = fs::read_to_string(path)
            .expect("Should have been able to read the restored_file_contents");
        assert_eq!(
            old_file_contents, restored_file_contents,
            "Test the restoration"
        );

        fs::remove_file(backup_path).expect("Failed to remove tmp backup file");
    }
    #[test]
    fn generate_containerd_config_test() {
        let test_file = concat!(env!("CARGO_MANIFEST_DIR"), "/mocks/in_config.toml");
        let conf = generate_containerd_config(test_file).unwrap();
        assert_eq!(
            conf["plugins"]["io.containerd.grpc.v1.cri"]["containerd"]["runtimes"]["crun"]
                ["runtime_type"]
                .as_str(),
            Some("io.containerd.runc.v2")
        );
        assert_eq!(
            conf["plugins"]["io.containerd.grpc.v1.cri"]["containerd"]["runtimes"]["crun"]
                ["privileged_without_host_devices"]
                .as_bool(),
            Some(false)
        );

        let opt3 = conf["plugins"]["io.containerd.grpc.v1.cri"]["containerd"]["runtimes"]["crun"]
            ["pod_annotations"]
            .as_array()
            .unwrap()
            .get(3);

        assert_eq!(
            opt3.unwrap().as_str().unwrap(),
            "module.wasm.image/variant.*"
        );

        assert_eq!(
            conf["plugins"]["io.containerd.grpc.v1.cri"]["containerd"]["runtimes"]["crun"]
                ["options"]["BinaryName"]
                .as_str(),
            Some("/usr/local/sbin/crun")
        );
    }
}
