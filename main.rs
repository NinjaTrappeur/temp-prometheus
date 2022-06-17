use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use std::sync::{Arc,Mutex};
use std::thread;

struct Metrics {
    humidity: String,
    temperature: String
}

/*
We'll have two threads:

- One in charge of parsing the CLI args, then in charge of parsing the
  data coming from the serial interface.
- Another one in charge of writing the results in the prometheus
  format on the disk every 30 seconds.
*/
fn main() -> std::io::Result<()> {
    let dev = std::env::args().nth(1).expect("Missing USB-DEV");
    let prometheus_file_path = std::env::args().nth(2).expect("Missing LISTEN-ADDR");

    let file = File::open(dev)?;
    let mut buf_reader = BufReader::new(file);
    let mut buffer = String::with_capacity(12);
    let metrics = Metrics{
        humidity: String::from("0"),
        temperature: String::from("0")
    };
    let metrics = Mutex::new(metrics);
    let metrics = Arc::new(metrics);

    let thread_metrics = metrics.clone();
    thread::spawn(move || {
        let filepath = prometheus_file_path.to_owned();
        let mut prom_str: String = "".to_owned();
        loop {
            {
                match File::create(&filepath) {
                    Ok(mut file) => {
                        if let Ok(m) = thread_metrics.lock() {
                            prom_str = format!("officetemp_humidity {}\nofficetemp_temperature {}\n", m.humidity, m.temperature);
                        }
                        match file.write_all((&prom_str).as_bytes()) {
                            Ok(()) => (),
                            Err(err) => {
                                eprintln!("Cannot write to {}: {}", prometheus_file_path, err);
                            }
                        };
                    },
                    Err(err) => {
                        eprintln!("Cannot write to {}: {}", prometheus_file_path, err);
                    }
                };
            }
            thread::sleep(std::time::Duration::from_secs(30));
        }
    });

    let thread_metrics = metrics.clone();
    loop {
        match buf_reader.read_line(&mut buffer) {
            Ok(result) => {
                if result == 12 {
                    if let Ok(mut m) = thread_metrics.lock() {
                        let vec: Vec<&str> = buffer.split(";").collect();
                        m.humidity = vec[0].to_string();
                        m.temperature = vec[1].to_string();
                        buffer.clear();
                    }
                }
            },
            Err(_) => ()
        }
    }
}
