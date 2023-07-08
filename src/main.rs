mod kalman;

use anyhow::Result;
use kalman::Kalman;
use log::debug;
use rocket::State;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};

const INITIAL_TEMPERATURE: f64 = 40.0;
const MEASUREMENT_ERROR: f64 = 2.0;
const PROCESS_VARIANCE: f64 = 0.05;

#[macro_use]
extern crate rocket;

#[derive(Clone)]
pub struct Measurements {
    cpu_temp_c: Option<f64>,
    cpu_temp_filtered: Kalman,
}

#[get("/metrics")]
async fn metrics(measurements: &State<Arc<Mutex<Measurements>>>) -> String {
    let measurements = measurements
        .lock()
        .expect("BUG: Failed to acquire cpu_temperature lock");
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("BUG: Failed to get current time")
        .as_millis();

    let mut metrics = String::new();
    if let Some(cpu_temp_c) = measurements.cpu_temp_c {
        metrics.push_str(&format!("cpu_temp_c {cpu_temp_c:.2} {time}\n"));
    }

    let cpu_temp_filtered_c = measurements.cpu_temp_filtered.value();
    metrics.push_str(&format!(
        "cpu_temp_filtered_c {cpu_temp_filtered_c:.2} {time}\n"
    ));
    metrics
}

pub async fn update_cpu_temp_periodically(measurements: Arc<Mutex<Measurements>>) {
    const UPDATE_PERIOD_MS: Duration = Duration::from_millis(1000);
    const CPU_TEMP_FILE: &str = "/sys/class/thermal/thermal_zone0/temp";

    loop {
        let cpu_temp_reading =
            fs::read_to_string(CPU_TEMP_FILE).expect("BUG: Failed to read temp file");
        let cpu_temp_c = cpu_temp_reading
            .trim()
            .parse::<f64>()
            .expect("BUG: Failed to parse temp value")
            / 1000.0;

        debug!("CPU temp reading: {} C", cpu_temp_c);
        {
            let mut measurements = measurements
                .lock()
                .expect("BUG: Failed to acquire cpu_temp lock");
            measurements.cpu_temp_c = Some(cpu_temp_c);
            measurements.cpu_temp_filtered.update(cpu_temp_c);
        }

        sleep(UPDATE_PERIOD_MS).await;
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    env_logger::init();

    let measurements = Arc::new(Mutex::new(Measurements {
        cpu_temp_c: None,
        cpu_temp_filtered: Kalman::new(INITIAL_TEMPERATURE, MEASUREMENT_ERROR, PROCESS_VARIANCE),
    }));

    tokio::spawn(update_cpu_temp_periodically(measurements.clone()));

    let figment = rocket::Config::figment()
        .merge(("port", 8081))
        .merge(("address", "0.0.0.0"));

    let _rocket = rocket::custom(figment)
        .mount("/", routes![metrics])
        .manage(measurements)
        .launch()
        .await?;

    Ok(())
}
