#[derive(Debug, sqlx::FromRow)]
pub struct Individual {
    pub id: String,
    pub username: String,
    pub password: String,
    pub ip: String,
    pub protocol: String,
    pub timestamp: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ProcessedIndividual {
    pub id: String,
    pub username: String,
    pub password: String,
    pub ip: String,
    pub protocol: String,
    pub hostname: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub loc: Option<String>,
    pub org: Option<String>,
    pub postal: Option<String>,
    pub asn: Option<String>,
    pub asn_name: Option<String>,
    pub asn_domain: Option<String>,
    pub asn_route: Option<String>,
    pub asn_type: Option<String>,
    pub company_name: Option<String>,
    pub company_domain: Option<String>,
    pub company_type: Option<String>,
    pub vpn: Option<bool>,
    pub proxy: Option<bool>,
    pub tor: Option<bool>,
    pub relay: Option<bool>,
    pub hosting: Option<bool>,
    pub service: Option<String>,
    pub abuse_address: Option<String>,
    pub abuse_country: Option<String>,
    pub abuse_email: Option<String>,
    pub abuse_name: Option<String>,
    pub abuse_network: Option<String>,
    pub abuse_phone: Option<String>,
    pub domain_ip: Option<String>,
    pub domain_total: Option<f64>,
    pub domains: Option<Vec<String>>,
    pub timestamp: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopUsername {
    pub username: String,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopPassword {
    pub password: String,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopIp {
    pub ip: String,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopProtocol {
    pub protocol: String,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopCountry {
    pub country: String,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopCity {
    pub city: String,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopRegion {
    pub region: String,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopTimezone {
    pub timezone: String,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopOrg {
    pub org: String,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopPostal {
    pub postal: String,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopUsrPassCombo {
    pub id: String,
    pub username: String,
    pub password: String,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopHourly{
    pub timestamp: i64,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopDaily {
    pub timestamp: i64,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopWeekly {
    pub timestamp: i64,
    pub amount: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TopYearly {
    pub timestamp: i64,
    pub amount: i32,
}