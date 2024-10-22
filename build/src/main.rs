use warp::Filter;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct DnsRecord {
    id: String,
    content: String,
}

#[derive(Deserialize)]
struct ListDnsRecordsResponse {
    result: Vec<DnsRecord>,
}

#[derive(Serialize)]
struct DnsUpdateRequest {
    content: String,
    ttl: u32,
    proxied: bool,
}

// Funktion zum Suchen des A-Records für eine bestimmte Subdomain
async fn find_dns_record(api_token: &str, zone_id: &str, subdomain: &str) -> Result<Option<DnsRecord>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records?type=A&name={}", zone_id, subdomain);

    let response = client
        .get(&url)
        .bearer_auth(api_token)
        .send()
        .await?
        .json::<ListDnsRecordsResponse>()
        .await?;

    if !response.result.is_empty() {
        Ok(Some(response.result[0].clone()))  // Den ersten A-Record zurückgeben
    } else {
        Ok(None)
    }
}

// Funktion zum Aktualisieren des A-Records
async fn update_cloudflare_ip(api_token: &str, zone_id: &str, record_id: &str, new_ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", zone_id, record_id);

    let update_request = DnsUpdateRequest {
        content: new_ip.to_string(),
        ttl: 120,
        proxied: false,
    };

    let response = client
        .put(&url)
        .bearer_auth(api_token)
        .json(&update_request)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Successfully updated IP address to {}", new_ip);
    } else {
        eprintln!("Failed to update IP: {:?}", response.text().await?);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // HTTP-Endpunkt für FritzBox DynDNS
    let update_route = warp::get()
        .and(warp::path("update"))
        .and(warp::query::<HashMap<String, String>>())
        .map(move |params: HashMap<String, String>| {
            // IP und Authentifizierungsparameter extrahieren
            if let (Some(ip), Some(domainname), Some(password)) = (
                params.get("myip"),
                params.get("domainname"),
                params.get("password"),  // API-Token kommt als "password"
            ) {
                println!("Received IP: {}", ip);
                println!("Domain: {}", domainname);

                // Finde die Zone-ID basierend auf dem Domainnamen
                let zone_name = domainname.rsplitn(2, '.').collect::<Vec<&str>>().join(".");
                let api_token = password.clone();

                tokio::spawn(async move {
                    // Finde die Cloudflare Zone-ID und die Record-ID
                    let zone_id = "<ZONE-ID>"; // Du musst die Zone-ID basierend auf deiner Domain kennen oder API verwenden
                    if let Some(record) = find_dns_record(&api_token, &zone_id, domainname).await.unwrap() {
                        if let Err(e) = update_cloudflare_ip(&api_token, &zone_id, &record.id, ip).await {
                            eprintln!("Error updating Cloudflare: {}", e);
                        }
                    } else {
                        eprintln!("No A record found for subdomain {}", domainname);
                    }
                });

                warp::reply::html("IP updated")
            } else {
                warp::reply::html("Missing required parameters")
            }
        });

    // Starte den Server
    warp::serve(update_route).run(([0, 0, 0, 0], 3030)).await;
}