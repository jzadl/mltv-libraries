use serde_json::{Value, Map};

pub fn get(url: String, headers: Value) -> Value {
    let mut cmd = std::process::Command::new("curl");
    cmd.arg("-s").arg("-S").arg("-L");
    cmd.arg("-w").arg("\n%{http_code}");
    add_headers(&mut cmd, &headers);
    cmd.arg(&url);
    run_curl(cmd, url)
}

pub fn post(url: String, data: String, json_data: Value, headers: Value) -> Value {
    let mut cmd = std::process::Command::new("curl");
    cmd.arg("-s").arg("-S").arg("-L");
    cmd.arg("-w").arg("\n%{http_code}");
    add_headers(&mut cmd, &headers);
    if !json_data.is_null() {
        cmd.arg("-H").arg("Content-Type: application/json");
        cmd.arg("-d").arg(json_data.to_string());
    } else if !data.is_empty() {
        cmd.arg("-d").arg(&data);
    }
    cmd.arg(&url);
    run_curl(cmd, url)
}

pub fn head(url: String, headers: Value) -> Value {
    let mut cmd = std::process::Command::new("curl");
    cmd.arg("-s").arg("-S").arg("-L").arg("-I");
    cmd.arg("-w").arg("\n%{http_code}");
    add_headers(&mut cmd, &headers);
    cmd.arg(&url);
    run_curl(cmd, url)
}

fn add_headers(cmd: &mut std::process::Command, headers: &Value) {
    if let Value::Object(hdrs) = headers {
        for (k, v) in hdrs {
            if let Value::String(val) = v {
                cmd.arg("-H").arg(format!("{}: {}", k, val));
            }
        }
    }
}

fn run_curl(mut cmd: std::process::Command, url: String) -> Value {
    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let trimmed = stdout.trim_end();
            let mut lines: Vec<&str> = trimmed.split('\n').collect();
            let status_str = lines.pop().unwrap_or("0").trim();
            let status: i64 = status_str.parse().unwrap_or(0);
            let body = lines.join("\n");
            let mut result = Map::new();
            result.insert("status".to_string(), Value::from(status));
            result.insert("text".to_string(), Value::from(body.clone()));
            result.insert("ok".to_string(), Value::from(status >= 200 && status < 300));
            result.insert("url".to_string(), Value::from(url));
            result.insert("json".to_string(), serde_json::from_str(&body).unwrap_or(Value::Null));
            if !stderr.is_empty() {
                result.insert("error".to_string(), Value::from(stderr));
            }
            Value::Object(result)
        }
        Err(e) => {
            let mut result = Map::new();
            result.insert("status".to_string(), Value::from(0_i64));
            result.insert("ok".to_string(), Value::from(false));
            result.insert("url".to_string(), Value::from(url));
            result.insert("error".to_string(), Value::from(e.to_string()));
            result.insert("text".to_string(), Value::from(String::new()));
            result.insert("json".to_string(), Value::Null);
            Value::Object(result)
        }
    }
}
