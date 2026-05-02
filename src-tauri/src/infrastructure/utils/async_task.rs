use std::any::type_name;
use std::future::Future;
use std::panic::{self, AssertUnwindSafe};
use std::thread;
use tauri::async_runtime as tauri_runtime;

/// 使用 Tauri 运行时的异步任务包装函数
///
/// 适用于需要在 Tauri 上下文中运行的任务，如 URI scheme handler
pub fn spawn_tauri_task_detached<F, T>(task_name: &'static str, task: F)
where
    F: Future<Output = Result<T, String>> + Send + 'static,
    T: Send + 'static,
{
    // 获取当前线程信息用于堆栈追踪
    let thread_name = thread::current().name().unwrap_or("unknown").to_string();
    let task_type_name = type_name::<F>().to_string();
    let task_name_clone = task_name.to_string();

    tauri_runtime::spawn(async move {
        // 捕获 panic 并转换为错误
        match panic::catch_unwind(AssertUnwindSafe(|| async { task.await })) {
            Ok(fut) => match fut.await {
                Ok(_) => {}
                Err(err) => {
                    let error = TaskError {
                        task_name: task_name_clone.clone(),
                        task_type: task_type_name.clone(),
                        origin_thread: thread_name.clone(),
                        error_type: ErrorType::BusinessError(err),
                        backtrace: Backtrace::capture(),
                    };
                    eprintln!(
                        "[Task {}] failed with business error: {}\nBacktrace:\n{}",
                        task_name_clone, error.error_type, error.backtrace
                    );
                }
            },
            Err(panic_err) => {
                let panic_msg = if let Some(s) = panic_err.downcast_ref::<&str>() {
                    *s
                } else if let Some(s) = panic_err.downcast_ref::<String>() {
                    s.as_str()
                } else {
                    "Unknown panic"
                };

                let error = TaskError {
                    task_name: task_name_clone,
                    task_type: task_type_name,
                    origin_thread: thread_name,
                    error_type: ErrorType::Panic(panic_msg.to_string()),
                    backtrace: Backtrace::capture(),
                };
                eprintln!(
                    "[Task {}] panicked: {}\nBacktrace:\n{}",
                    error.task_name, panic_msg, error.backtrace
                );
            }
        }
    });
}

/// 异步任务错误类型
#[derive(Debug)]
pub struct TaskError {
    /// 任务名称
    pub task_name: String,
    /// 任务类型名称
    pub task_type: String,
    /// 任务发起线程名称
    pub origin_thread: String,
    /// 错误类型
    pub error_type: ErrorType,
    /// 堆栈追踪信息
    pub backtrace: Backtrace,
}

/// 错误类型枚举
#[derive(Debug)]
pub enum ErrorType {
    /// 业务逻辑错误
    BusinessError(String),
    /// Panic 错误
    Panic(String),
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::BusinessError(msg) => write!(f, "BusinessError: {}", msg),
            ErrorType::Panic(msg) => write!(f, "Panic: {}", msg),
        }
    }
}

/// 堆栈追踪封装
#[derive(Debug)]
pub struct Backtrace {
    inner: Option<backtrace::Backtrace>,
}

impl Backtrace {
    /// 捕获当前堆栈追踪
    pub fn capture() -> Self {
        Backtrace {
            inner: Some(backtrace::Backtrace::new()),
        }
    }
}

impl std::fmt::Display for Backtrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            Some(bt) => write!(f, "{:?}", bt),
            None => write!(f, "Backtrace not available"),
        }
    }
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TaskError {{ task: {}, type: {}, thread: {}, error: {:?} }}",
            self.task_name, self.task_type, self.origin_thread, self.error_type
        )
    }
}

impl std::error::Error for TaskError {}
