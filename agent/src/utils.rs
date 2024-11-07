use std::fs;

use bollard::{
    container::{Config, CreateContainerOptions},
    secret::{HostConfig, Mount, MountTypeEnum, PortBinding},
};
use common::models::Server;

pub fn container_name(id: i32) -> String {
    format!("nerdpanel-server-{}", id)
}

pub fn get_folder(id: i32) -> String {
    // TODO get from env
    format!("run/nerdpanel/volumes/{}", container_name(id))
}

pub fn container_options(
    server: &Server,
) -> (Option<CreateContainerOptions<String>>, Config<String>) {
    let folder_path = fs::canonicalize(get_folder(server.id)).unwrap();

    let mut port_bindings = ::std::collections::HashMap::new();
    port_bindings.insert(
        format!("{}/tcp", server.port),
        Some(vec![PortBinding {
            host_ip: Some(server.ip.clone()),
            host_port: Some(server.port.to_string()),
        }]),
    );
    let host_config = HostConfig {
        mounts: Some(vec![Mount {
            target: Some(String::from("/data")),
            source: Some(folder_path.to_string_lossy().to_string()),
            typ: Some(MountTypeEnum::BIND),
            consistency: Some(String::from("default")),
            ..Default::default()
        }]),
        port_bindings: Some(port_bindings),
        ..Default::default()
    };

    let config = Config {
        image: Some(format!("itzg/minecraft-server")),
        tty: Some(true),
        open_stdin: Some(true),
        env: Some(vec!["EULA=TRUE".to_string()]),
        host_config: Some(host_config),
        exposed_ports: {
            let mut map = ::std::collections::HashMap::new();
            map.insert(
                format!("{}/tcp", server.port),
                ::std::collections::HashMap::new(),
            );
            Some(map)
        },
        ..Default::default()
    };

    let options = Some(CreateContainerOptions {
        name: container_name(server.id),
        platform: None,
    });

    (options, config)
}
