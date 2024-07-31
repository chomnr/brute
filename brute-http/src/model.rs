use axum::http::StatusCode;
use derive_getters::Getters;

use actix::Message;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{system::RequestWithLimit, validator::Validate};

#[derive(Default, Clone, Debug, sqlx::FromRow,Getters)]
pub struct Individual {
    pub id: String,
    username: String,
    password: String,
    ip: String,
    protocol: String,
    pub timestamp: i64,
}

impl Individual {
    pub fn new(id: String, username: String, password: String, ip: String, protocol: String, timestamp: i64) -> Self {
        Self {
            id,
            username,
            password,
            ip,
            protocol,
            timestamp,
        }
    }

    pub fn new_short(username: String, password: String, ip: String, protocol: String) -> Self {
        Self {
            id: String::default(),
            username,
            password,
            ip,
            protocol,
            timestamp: 0,
        }
    }
}

// allow as a message in actix actor.
impl Message for Individual {
    type Result = ();
}

impl Validate for Individual {
    fn validate(&self) -> anyhow::Result<(), (axum::http::StatusCode, String)> {
        if self.username.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "input validation error: username is empty.".to_string()))
        }

        if self.password.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "input validation error: password is empty.".to_string()))
        }

        if self.ip.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "input validation error: ip is empty.".to_string()))
        }

        if self.protocol.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "input validation error: protocol is empty.".to_string()))
        }

        let regex_ip = Regex::new(r#"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$"#).unwrap();
        if !regex_ip.is_match(&self.ip) {
            return Err((StatusCode::BAD_REQUEST, "input validation error: ip_address is not formatted correctly.".to_string()))
        }
        Ok(())
    }
}

#[derive(Default, Clone, Debug, sqlx::FromRow, Getters, Serialize, Deserialize)]
pub struct ProcessedIndividual {
    pub id: String,
    username: String,
    password: String,
    ip: String,
    protocol: String,
    hostname: Option<String>,
    city: Option<String>,
    region: Option<String>,
    timezone: String,
    country: Option<String>,
    loc: Option<String>,
    org: Option<String>,
    postal: Option<String>,
    asn: Option<String>,
    asn_name: Option<String>,
    asn_domain: Option<String>,
    asn_route: Option<String>,
    asn_type: Option<String>,
    company_name: Option<String>,
    company_domain: Option<String>,
    company_type: Option<String>,
    vpn: Option<bool>,
    proxy: Option<bool>,
    tor: Option<bool>,
    relay: Option<bool>,
    hosting: Option<bool>,
    service: Option<String>,
    abuse_address: Option<String>,
    abuse_country: Option<String>,
    abuse_email: Option<String>,
    abuse_name: Option<String>,
    abuse_network: Option<String>,
    abuse_phone: Option<String>,
    domain_ip: Option<String>,
    domain_total: Option<i64>,
    domains: Option<Vec<String>>,
    pub timestamp: i64,
}

impl Message for RequestWithLimit<ProcessedIndividual> {
    type Result = Result<Vec<ProcessedIndividual>, StatusCode>;
}

#[derive(Debug, sqlx::FromRow, Getters)]
pub struct TopUsername {
    username: String,
    amount: i32,
}

#[derive(Debug, sqlx::FromRow, Getters)]
pub struct TopPassword {
    password: String,
    amount: i32,
}

#[derive(Debug, sqlx::FromRow, Getters)]
pub struct TopIp {
    ip: String,
    amount: i32,
}

#[derive(Debug, sqlx::FromRow, Getters)]
pub struct TopProtocol {
    protocol: String,
    amount: i32,
}

#[derive(Debug, sqlx::FromRow, Getters)]
pub struct TopCountry {
    country: String,
    amount: i32,
}

#[derive(Debug, sqlx::FromRow, Getters)]
pub struct TopCity {
    city: String,
    amount: i32,
}

#[derive(Debug, sqlx::FromRow, Getters)]
pub struct TopRegion {
    region: String,
    amount: i32,
}

#[derive(Debug, sqlx::FromRow, Getters)]
pub struct TopTimezone {
    timezone: String,
    amount: i32,
}

#[derive(Debug, sqlx::FromRow, Getters)]
pub struct TopOrg {
    org: String,
    amount: i32,
}

#[derive(Debug, sqlx::FromRow, Getters)]
pub struct TopPostal {
    postal: String,
    amount: i32,
}

#[derive(Debug, sqlx::FromRow, Getters)]
pub struct TopUsrPassCombo {
    id: String,
    username: String,
    password: String,
    amount: i32,
}

#[derive(Debug, Clone, sqlx::FromRow, Getters)]
pub struct TopHourly {
    pub timestamp: i64,
    pub amount: i32,
}

#[derive(Debug, Clone, sqlx::FromRow, Getters)]
pub struct TopDaily {
    pub timestamp: i64,
    pub amount: i32,
}

#[derive(Debug, Clone, sqlx::FromRow, Getters)]
pub struct TopWeekly {
    pub timestamp: i64,
    pub amount: i32,
}

#[derive(Debug, Clone, sqlx::FromRow, Getters)]
pub struct TopYearly {
    pub timestamp: i64,
    pub amount: i32,
}
