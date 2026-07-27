use crate::configs::Configs;

pub fn format_src(source: &String) -> String {
    if source.ends_with('/') {
        String::from(source)
    } else {
        String::from(format!("{source}/"))
    }
}

fn format_dst(user: &String, host: &String, destination: &String) -> String {
    let dst = if destination.ends_with('/') {
        String::from(&destination[..destination.len() - 1])
    } else {
        String::from(destination)
    };

    format!("{user}@{host}:{dst}")
}

pub fn format_dst_hot(configs: &Configs) -> String {
    format_dst(
        &configs.storage.hot.user,
        &configs.storage.hot.host,
        &configs.storage.hot.destination,
    )
}

pub fn format_dst_cold(configs: &Configs) -> String {
    format_dst(
        &configs.storage.cold.user,
        &configs.storage.cold.host,
        &configs.storage.cold.destination,
    )
}
