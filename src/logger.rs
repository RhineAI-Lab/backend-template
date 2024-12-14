use flexi_logger::{Logger, FileSpec, Age, Cleanup, Criterion, Naming, Duplicate};
use log::LevelFilter;
use std::env;

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量中读取日志级别，默认为 Info
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let level_filter = match log_level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    // 配置日志输出到文件和控制台
    Logger::try_with_str(&log_level)? // 动态设置日志级别
        .format(flexi_logger::detailed_format)
        .log_to_file(
            FileSpec::default()
                .directory("logs")       // 日志文件存储目录
                .basename("app")         // 日志文件名
                .suppress_timestamp(),   // 不在文件名中包含时间戳
        )
        .rotate(
            Criterion::Age(Age::Day),   // 日志轮转条件：每天
            Naming::Timestamps,        // 使用时间戳命名旋转后的日志文件
            Cleanup::KeepLogFiles(7),  // 保留最近7天的日志文件
        )
        .duplicate_to_stdout(Duplicate::All) // 同时输出到控制台（转换后的类型）
        .start()?;

    Ok(())
}
