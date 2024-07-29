use serde::Deserialize;


/// Represents an individual attacker stored in the database.
pub struct DatabaseIndividual {
    id: String,
    basic_info: Individual,
    detailed_info: ProcessedIndividual,
    created_at: i64
}

/// Represents the basic information of an individual attacker.
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Individual {
    username: String,
    password: String,
    ip: String,
    protocol: String,
}

/// Represents the processed information of an individual attacker after IP info.
pub struct ProcessedIndividual  {
    // Included in the free tier of ipinfo.io
    ip: String,
    hostname: String,
    city: String,
    region: String,
    country: String,
    loc: String, // Latitude and longitude
    org: String,
    postal: String,
    // Included in the ipinfo.io basic plan
    asn: IndividualAsn,

    // Included in the ipinfo.io business plan
    company: IndividualCompany,
 
    // Included in the ipinfo.io standard plan
    privacy: IndividualPrivacy,
 
    // Included in the ipinfo.io business plan
    abuse: IndividualAbuse,
}

/// Represents ASN information.
pub struct IndividualAsn {
    asn: String,
    name: String,
    domain: String,
    route: String,
    r#type: String
}

/// Represents company information.
pub struct IndividualCompany {
    name: String,
    domain: String,
    r#type: String
}

/// Represents privacy information.
pub struct IndividualPrivacy {
    vpn: Option<bool>,
    proxy: Option<bool>,
    tor: Option<bool>,
    relay: Option<bool>,
    hosting: Option<bool>,
    service: String
}

/// Represents abuse contact information.
pub struct IndividualAbuse {
    address: String,
    country: String,
    email: String,
    name: String,
    network: String,
    phone: String
}

/// Represents hosted domains information.
pub struct IndividualDomain {
    ip: String,
    total: u128,
    domains: Vec<String>
}