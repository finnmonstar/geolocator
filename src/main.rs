#![deny(warnings, unsafe_code, clippy::all)]

use {
    std::{env, process},
    geolocator::{VERSION, Information},
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 || args.first().is_none() {
        eprintln!("Usage:   geolocator <ip>   (geolocate any IP address you choose)");
        eprintln!("         geolocator local  (geolocate the IP address of the machine the program is run on)");
        process::exit(1);
    }

    let query = match args.first().unwrap().as_str() {
        "local" => String::new(),
        _ => args.first().unwrap().to_owned(),
    };

    let info = reqwest::blocking::get(format!("http://ip-api.com/json/{query}?fields=66846719"))
        .unwrap_or_else(|err| {
            eprintln!("Error while performing the request: {}", err);
            process::exit(1);
        })
        .json::<Information>()
        .unwrap_or_else(|err| {
            eprintln!(
                "Error while converting the JSON to the 'Information' struct: {}",
                err
            );
            process::exit(1);
        });

    let none_message = String::from("None");

    if info.status == "fail" {
        eprintln!(
            "geolocator {VERSION}:\n\nGot status `fail`: {}",
            info.message.clone().unwrap_or(String::from("no message"))
        );
        process::exit(1);
    }

    println!("geolocator {VERSION}\n\nStatus: {}\nMessage: {}\nContinent: {}\nContinent Code: {}\nCountry: {}\nCountry Code: {}\nRegion: {}\nRegion Name: {}\nCity: {}\nDistrict: {}\nZip Code: {}\nLatitude: {}\nLongitude: {}\nTimezone: {}\nOffset: {}\nCurrency: {}\nISP: {}\nOrganization: {}\nAS Number and Organization: {}\nAS Name: {}\nReverse DNS: {}\nMobile: {}\nProxy: {}\nHosting: {}\nIP: {}", info.status, info.message.unwrap_or(none_message.clone()), info.continent.unwrap_or(none_message.clone()), info.continent_code.unwrap_or(none_message.clone()), info.country.unwrap_or(none_message.clone()), info.country_code.unwrap_or(none_message.clone()), info.region.unwrap_or(none_message.clone()), info.region_name.unwrap_or(none_message.clone()), info.city.unwrap_or(none_message.clone()), info.district.unwrap_or(none_message.clone()), info.zip_code.unwrap_or(none_message.clone()), info.latitude.unwrap_or(0.0), info.longitude.unwrap_or(0.0), info.timezone.unwrap_or(none_message.clone()), info.offset.unwrap_or(0), info.currency.unwrap_or(none_message.clone()), info.isp.unwrap_or(none_message.clone()), info.organization.unwrap_or(none_message.clone()), info.r#as.unwrap_or(none_message.clone()), info.asname.unwrap_or(none_message.clone()), info.reverse.unwrap_or(none_message.clone()), info.mobile.unwrap_or(false), info.proxy.unwrap_or(false), info.hosting.unwrap_or(false), info.query.unwrap_or(none_message.clone()));
}
