use chrono::Utc;
use questdb::{
    Result,
    ingress::{Sender, TimestampNanos},
};

use crate::core::config::SETTINGS;

pub struct ReqParams {
    pub method: String,
    pub headers: Vec<String>,
    pub http_version: String,
    pub path: String,
    pub scheme: String,
    pub path_params: String,
    pub query_string: String,
    pub server: String,
    pub client: String,
    // pub body: String,
}

pub struct ResParams {
    pub status_code: String,
    pub headers: Vec<String>,
    pub process_time: f64,
    pub created_at: String,
    // pub response: String,
}

pub struct Params {
    pub req_params: ReqParams,
    pub res_params: ResParams,
}

pub fn send_logs_to_questdb(params: Params) -> Result<()> {
    let transport = "http";
    let host = SETTINGS.questdb_host.as_str();
    let port = SETTINGS.questdb_port.as_str();
    let current_datetime = Utc::now();

    let credentials = match (
        SETTINGS.questdb_user.clone(),
        SETTINGS.questdb_password.clone(),
    ) {
        (Ok(username), Ok(password)) => format!("username={username};password={password};"),
        (Ok(username), Err(_)) => format!("username={username};"),
        (Err(_), Ok(password)) => format!("password={password};"),
        (Err(_), Err(_)) => String::new(),
    };

    let mut sender = Sender::from_conf(format!("{transport}::addr={host}:{port};{credentials}",))?;

    // Protocol version documentation: https://docs.rs/crate/questdb-rs/latest
    // let mut buffer = Buffer::new(ProtocolVersion::V1);

    let mut buffer = sender.new_buffer();
    buffer
        .table(SETTINGS.questdb_db.as_str())?
        .column_str("method", params.req_params.method)?
        .column_str(
            "req_headers",
            // This is the idea: \{{params.req_params.headers.join(", "))}\}
            format!("{{{}}}", params.req_params.headers.join(", ")),
        )?
        // Request parameters
        .column_str("path", params.req_params.path)?
        .column_str("scheme", params.req_params.scheme)?
        .column_str("path_params", params.req_params.path_params)?
        .column_str("query_string", params.req_params.query_string)?
        .column_str("server", params.req_params.server)?
        .column_str("client", params.req_params.client)?
        .column_str("http_version", params.req_params.http_version)?
        // Response parameters
        .column_str("status_code", params.res_params.status_code)?
        .column_str(
            "res_headers",
            format!("{{{}}}", params.res_params.headers.join(", ")),
        )?
        .column_f64("process_time", params.res_params.process_time)?
        .column_str("created_at", params.res_params.created_at)?
        .at(TimestampNanos::from_datetime(current_datetime)?)?;

    sender.flush(&mut buffer)?;

    Ok(())
}
