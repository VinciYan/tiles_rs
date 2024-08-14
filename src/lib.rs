use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use std::fs::File;
use std::io::Read;
use log::{info, error, warn};
use chrono::{DateTime, Local};
use flexi_logger::{Cleanup, Criterion, DeferredNow, Naming, Record};
use std::time::SystemTime;

const ENV_VAR_LOG_DIR: &str = "EXE_UNIT_LOG_DIR";
const DEFAULT_LOG_LEVEL: &str = "info";
const DEFAULT_LOG_DIR: &str = "logs";
const DEFAULT_LOG_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3f%z";


#[derive(Parser)]
#[command(
    author = "vinciyan", 
    version = "v0.1.0", 
    about = "A high-performance, lightweight tile map server written in Rust.",
    long_about = "
================================================================================\n\
                        Overview\n\
================================================================================\n\
Tiles_rs is an open-source project that aims to provide a fast and reliable tile map server implementation using Rust.\n\
Built on top of the Actix web framework, this project offers a modern approach to serving map tiles,\n\
catering to the needs of developers working on geographic information systems (GIS) and web mapping applications.\n\
\n\
# Examples\n\n\
```sh\n\
tiles_rs.exe --tiles-dir=C:\\Users\\Tiles --host=0.0.0.0 --port=5000 --log_level=warn\n\
```
\n\
# Api\n\n\
- /tiles/{z}/{x}/{y}\n\n\
{z} - The current zoom level.
{x} - The horizontal (X) index of the requested tile.
{y} - The vertical (Y) index of the requested tile."
)]
pub struct Args {
    /// Directory containing tile images
    #[arg(long, default_value = "Tiles")]
    pub tiles_dir: String,

    /// Host to bind the server to
    #[arg(long, default_value = "localhost")]
    pub host: String,

    /// Port to bind the server to
    #[arg(long, default_value_t = 5000)]
    pub port: u16,

    /// Log level (error, warn, info, debug, trace)
    #[arg(long, default_value = "info")]
    pub log_level: String
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>map source</h1>")
}

#[get("/tiles/{z}/{x}/{y}")]
async fn get_tiles(path: web::Path<(u32, u32, u32)>, data: web::Data<AppState>) -> impl Responder {
    let (z, x, y) = path.into_inner();
    let img_path = format!("{}/{}/{}/{}.png", data.tiles_dir, z, x, y);
    
    match File::open(&img_path) {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            if file.read_to_end(&mut buffer).is_ok() {   
                info!("Serving tile: {}", img_path);             
                HttpResponse::Ok()
                    .content_type("image/png")
                    .body(buffer)
            } else {
                error!("Error reading file: {}", img_path);
                HttpResponse::InternalServerError().finish()
            }
        }
        Err(_) => {
            warn!("File not found: {}", img_path);
            HttpResponse::NotFound().finish()
        }
    }
}

pub struct AppState {
    tiles_dir: String,
}

pub async fn run_server(tiles_dir: String, host: String, port: u16) -> std::io::Result<()> {
    println!("Server starting on http://{}:{}", host, port);
    println!("Serving tiles from: {}", tiles_dir);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                tiles_dir: tiles_dir.clone(),
            }))
            .service(index)
            .service(get_tiles)
    })
    .bind((host, port))?
    .run()
    .await
}

// https://github.com/golemfactory/yagna/blob/master/exe-unit/src/logger.rs#L13
pub fn start_file_logger(args: &Args) -> anyhow::Result<flexi_logger::LoggerHandle> {
    let log_dir = std::env::var(ENV_VAR_LOG_DIR).unwrap_or_else(|_| DEFAULT_LOG_DIR.to_string());

    Ok(build_logger(Some(&args.log_level))?
        .log_to_file(flexi_logger::FileSpec::default().directory(log_dir))
        .duplicate_to_stderr(log_tty_dup_level()?)
        .rotate(
            Criterion::Size(10_000), // 设置日志文件大小限制为 5 KB
            Naming::Timestamps,         // 使用时间戳进行文件命名
            Cleanup::KeepLogFiles(3),   // 保留最近的 3 个日志文件
        )
        .start()?)
}

fn build_logger<S: ToString>(log_level: Option<S>) -> anyhow::Result<flexi_logger::Logger> {
    let level = match log_level {
        Some(level) => level.to_string(),
        None => std::env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_LOG_LEVEL.to_string()),
    };

    Ok(flexi_logger::Logger::try_with_str(level)?
        .format(log_format)
        .format_for_stderr(flexi_logger::colored_opt_format))
}

fn log_tty_dup_level() -> anyhow::Result<flexi_logger::Duplicate> {
    use flexi_logger::Duplicate;
    use log::LevelFilter;

    let level_filter = flexi_logger::LogSpecification::env_or_parse(DEFAULT_LOG_LEVEL)?
        .module_filters()
        .iter()
        .find(|f| f.module_name.is_none())
        .map(|f| f.level_filter)
        .unwrap_or(LevelFilter::Off);

    Ok(match level_filter {
        LevelFilter::Off => Duplicate::None,
        LevelFilter::Trace => Duplicate::Trace,
        LevelFilter::Debug => Duplicate::Debug,
        LevelFilter::Info => Duplicate::Info,
        LevelFilter::Warn => Duplicate::Warn,
        LevelFilter::Error => Duplicate::Error,
    })
}

pub fn start_logger() -> anyhow::Result<flexi_logger::LoggerHandle> {
    Ok(build_logger(Option::<String>::None)?.start()?)
}

fn log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    //use DateTime::<Local> instead of DateTime::<UTC> to obtain local date
    let now = SystemTime::from(*now.now());
    let local_date = DateTime::<Local>::from(now);
    //format date as following: 2020-08-27T07:56:22.348+02:00 (local date + time zone with milliseconds precision)
    let date_format = local_date.format(DEFAULT_LOG_FORMAT);

    write!(
        w,
        "[{} {:5} {}] {}",
        date_format,
        record.level(),
        record.module_path().unwrap_or("<unnamed>"),
        record.args()
    )
}