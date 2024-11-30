use std::fs;

use axum::http::StatusCode;
use axum_thiserror::ErrorStatus;
use bollard::{
    container::{Config, CreateContainerOptions},
    secret::{HostConfig, Mount, MountTypeEnum, PortBinding},
};
use common::orch_types::Server;
use thiserror::Error;

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
        format!("{}/tcp", server.primary_port.port),
        Some(vec![PortBinding {
            host_ip: Some(server.primary_port.ip.clone()),
            host_port: Some(server.primary_port.port.to_string()),
        }]),
    );
    for port_binding in &server.additional_ports {
        port_bindings.insert(
            format!("{}/tcp", port_binding.port),
            Some(vec![PortBinding {
                host_ip: Some(port_binding.ip.clone()),
                host_port: Some(port_binding.port.to_string()),
            }]),
        );
    }
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
        image: Some(server.image.clone()),
        tty: Some(true),
        open_stdin: Some(true),
        cmd: Some({
            let mut cmd = vec![];
            server
                .startup_command
                .split_ascii_whitespace()
                .for_each(|s| {
                    cmd.push(s.to_string());
                });
            cmd
        }),
        env: Some({
            let mut env = vec![];
            for env_var in &server.env_vars {
                env.push(format!("{}={}", env_var.key, env_var.value));
            }
            env
        }),
        host_config: Some(host_config),
        exposed_ports: {
            let mut map = ::std::collections::HashMap::new();
            map.insert(
                format!("{}/tcp", server.primary_port.port),
                ::std::collections::HashMap::new(),
            );
            for port_binding in &server.additional_ports {
                map.insert(
                    format!("{}/tcp", port_binding.port),
                    ::std::collections::HashMap::new(),
                );
            }
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

#[derive(Error, Debug, ErrorStatus)]
pub enum AppError {
    #[error("Docker error")]
    #[status(StatusCode::INTERNAL_SERVER_ERROR)]
    DockerError(bollard::errors::Error),
}

impl From<bollard::errors::Error> for AppError {
    fn from(e: bollard::errors::Error) -> Self {
        tracing::error!("Docker error: {:?}", e);
        Self::DockerError(e)
    }
}
