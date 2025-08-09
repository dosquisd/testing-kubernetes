use chrono::{DateTime, Utc};
use std::time::Instant;

use actix_web::http::Version;
use actix_web::{
    Error,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};

use super::utils::{Params, ReqParams, ResParams, send_logs_to_questdb};

pub async fn dispatch_logs(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let http_version = req.version();

    let version_str = match http_version {
        Version::HTTP_09 => "HTTP/0.9",
        Version::HTTP_10 => "HTTP/1.0",
        Version::HTTP_11 => "HTTP/1.1",
        Version::HTTP_2 => "HTTP/2.0",
        Version::HTTP_3 => "HTTP/3.0",
        _ => "Unknown", // Handle any future or unhandled versions
    };

    let req_params = ReqParams {
        method: req.method().to_string(),
        headers: req
            .headers()
            .iter()
            .map(|(key, value)| match value.to_str() {
                Ok(v) => format!("{}: {}", key.to_string(), v.to_string()),
                Err(_) => String::new(),
            })
            .collect(),
        path: req.path().to_string(),
        scheme: req.connection_info().scheme().to_string(),
        // scheme: req.uri().scheme_str().unwrap_or("http").to_string(),
        path_params: req.match_info().as_str().to_string(),
        query_string: req.query_string().to_string(),
        server: req.connection_info().host().to_string(),
        client: req
            .connection_info()
            .peer_addr()
            .unwrap_or("unknown")
            .to_string(),
        http_version: version_str.to_string(),
    };

    let created_at: DateTime<Utc> = Utc::now();
    let start_time = Instant::now();

    let response = next.call(req).await;
    if let Err(res) = response {
        return Err(res);
    }

    let response = response.unwrap();
    let process_time = start_time.elapsed().as_micros() as f64 / 1000000.0;
    let res_params = ResParams {
        status_code: response.status().as_str().to_string(),
        headers: response
            .headers()
            .iter()
            .map(|(key, value)| match value.to_str() {
                Ok(v) => format!("{}: {}", key.to_string(), v.to_string()),
                Err(_) => String::new(),
            })
            .collect(),
        process_time: process_time,
        created_at: created_at.to_rfc3339(),
    };

    let params = Params {
        req_params,
        res_params,
    };

    let result_questdb = send_logs_to_questdb(params);
    match result_questdb {
        Ok(_) => (),
        Err(e) => log::error!("Error sending logs to QuestDB: {}", e),
    };

    Ok(response)
}
