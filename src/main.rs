// Will create an exporter with a single metric that will randomize the value
// of the metric everytime the exporter duration times out.

use env_logger::{Builder, Env};
use log::info;
use prometheus_exporter::prometheus::register_gauge;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
};

#[derive(Debug, Deserialize)]
pub struct GoEControllerApiResponse {
    #[serde(rename = "ccn")]
    pub category_names: Vec<String>,
    #[serde(rename = "ccp")]
    pub category_powers: Vec<f32>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum GoEControllerCategory {
    #[serde(rename = "Home")]
    Home,
    #[serde(rename = "Grid")]
    Grid,
    #[serde(rename = "Car")]
    Car,
    #[serde(rename = "Relais")]
    Relais,
    #[serde(rename = "Solar")]
    Solar,
    #[serde(rename = "Akku")]
    Akku,
    #[serde(rename = "Custom1")]
    Custom1,
    #[serde(rename = "Custom2")]
    Custom2,
    #[serde(rename = "Custom3")]
    Custom3,
    #[serde(rename = "Custom4")]
    Custom4,
    #[serde(rename = "Custom5")]
    Custom5,
    #[serde(rename = "Custom6")]
    Custom6,
    #[serde(rename = "Custom7")]
    Custom7,
    #[serde(rename = "Custom8")]
    Custom8,
    #[serde(rename = "Custom9")]
    Custom9,
    #[serde(rename = "Custom10")]
    Custom10,
}

#[derive(Debug, Serialize)]
pub enum ExporterError {
    CategoryUnknown,
}

impl TryFrom<&str> for GoEControllerCategory {
    type Error = ExporterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Home" => Ok(Self::Home),
            "Grid" => Ok(Self::Grid),
            "Car" => Ok(Self::Car),
            "Relais" => Ok(Self::Relais),
            "Solar" => Ok(Self::Solar),
            "Akku" => Ok(Self::Akku),
            "Custom1" => Ok(Self::Custom1),
            "Custom2" => Ok(Self::Custom2),
            "Custom3" => Ok(Self::Custom3),
            "Custom4" => Ok(Self::Custom4),
            "Custom5" => Ok(Self::Custom5),
            "Custom6" => Ok(Self::Custom6),
            "Custom7" => Ok(Self::Custom7),
            "Custom8" => Ok(Self::Custom8),
            "Custom9" => Ok(Self::Custom9),
            _ => Err(Self::Error::CategoryUnknown),
        }
    }
}

impl Into<HashMap<GoEControllerCategory, f32>> for GoEControllerApiResponse {
    fn into(self) -> HashMap<GoEControllerCategory, f32> {
        let mut map = HashMap::new();

        for i in 0..self.category_names.len() + 1 {
            map.insert(
                GoEControllerCategory::try_from(self.category_names[i].as_str())
                    .expect(format!("Category unknown: {}", self.category_names[i]).as_str()),
                self.category_powers[i],
            );
        }

        map
    }
}

fn main() {
    // Setup logger with default level info so we can see the messages from
    // prometheus_exporter.
    Builder::from_env(Env::default().default_filter_or("info")).init();

    // Parse address used to bind exporter to.
    let addr_raw = "0.0.0.0:9186";
    let addr: SocketAddr = addr_raw.parse().expect("can not parse listen addr");

    // Start exporter and update metrics every five seconds.
    let exporter = prometheus_exporter::start(addr).expect("can not start exporter");
    let duration = std::time::Duration::from_secs(5);

    // Create metric
    let random = register_gauge!("run_and_repeat_random", "will set a random value")
        .expect("can not create gauge random_value_metric");

    let mut rng = rand::thread_rng();

    loop {
        {
            // Will block until duration is elapsed.
            let _guard = exporter.wait_duration(duration);

            info!("Updating metrics");

            // Update metric with random value.
            let new_value = rng.r#gen();
            info!("New random value: {}", new_value);

            random.set(new_value);
        }

        let goe_response = reqwest::blocking::get("http://192.168.178.81/api/status")
            .expect("can not get metrics from exporter")
            .text()
            .expect("can not body text from request");

        let goe_parsed: GoEControllerApiResponse =
            serde_json::from_str(&goe_response.as_str()).expect("Couldn't deserialize");

        info!("parsed_response: {:?}", goe_parsed);

        /*
        // Get metrics from exporter
        let body = reqwest::blocking::get(format!("http://{addr_raw}/metrics"))
            .expect("can not get metrics from exporter")
            .text()
            .expect("can not body text from request");

        //info!("Exporter metrics:\n{}", body);*/
    }
}
