use anyhow::Result;
use log::debug;
use rocket::State;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};

#[macro_use]
extern crate rocket;

struct Measurements {
    cpu_temp_c: Arc<Mutex<Option<f64>>>,
}

#[get("/metrics")]
async fn metrics(measurements: &State<Measurements>) -> String {
    let cpu_temp_c = measurements
        .cpu_temp_c
        .lock()
        .expect("BUG: Failed to acquire cpu_temperature lock");

    match *cpu_temp_c {
        Some(cpu_temp_c) => {
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("BUG: Failed to get current time")
                .as_millis();

            format!("cpu_temp_c {cpu_temp_c:.2} {time}\n")
        }
        _ => "".to_string(),
    }
}

pub async fn update_cpu_temp_periodically(cpu_temp_c: Arc<Mutex<Option<f64>>>) {
    const UPDATE_PERIOD_MS: Duration = Duration::from_millis(1000);
    const CPU_TEMP_FILE: &str = "/sys/class/thermal/thermal_zone0/temp";

    loop {
        let cpu_temp_reading =
            fs::read_to_string(CPU_TEMP_FILE).expect("BUG: Failed to read temp file");
        let new_cpu_temp_c = cpu_temp_reading
            .trim()
            .parse::<f64>()
            .expect("BUG: Failed to parse temp value")
            / 1000.0;

        debug!("CPU temp reading: {} C", new_cpu_temp_c);
        {
            let mut cpu_temp_c = cpu_temp_c
                .lock()
                .expect("BUG: Failed to acquire cpu_temp lock");
            *cpu_temp_c = Some(new_cpu_temp_c);
        }

        sleep(UPDATE_PERIOD_MS).await;
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    env_logger::init();

    let cpu_temp_c = Arc::new(Mutex::new(None));

    let measurements = Measurements {
        cpu_temp_c: cpu_temp_c.clone(),
    };

    tokio::spawn(update_cpu_temp_periodically(cpu_temp_c.clone()));

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
