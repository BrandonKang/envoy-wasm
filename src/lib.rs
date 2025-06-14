use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[derive(Default)]
struct MyFilter;

impl Context for MyFilter {}

impl HttpContext for MyFilter {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        match self.get_http_request_header("Authorization") {
            Some(_) => {
                proxy_wasm::hostcalls::log(LogLevel::Info, "✅ Authorization found").ok();
                Action::Continue
            }
            None => {
                proxy_wasm::hostcalls::log(LogLevel::Warn, "⛔ Authorization missing, rejecting").ok();
                self.send_http_response(
                    403,
                    vec![("content-type", "text/plain")],
                    Some(b"Permission Denied\n"),
                );
                Action::Pause
            }
        }
    }
}

#[derive(Default)]
struct MyRoot;

impl Context for MyRoot {}

impl RootContext for MyRoot {
    fn on_vm_start(&mut self, _: usize) -> bool {
        proxy_wasm::hostcalls::log(LogLevel::Info, "🟢 Rust VM started").ok();
        true
    }

    fn on_configure(&mut self, _: usize) -> bool {
        proxy_wasm::hostcalls::log(LogLevel::Info, "🛠️ Rust filter configured").ok();
        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(MyFilter::default()))
    }
}

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_root_context(|_context_id| {
        proxy_wasm::set_http_context(|_context_id, _root_context_id| {
            Box::new(MyFilter::default())
        });
        Box::new(MyRoot::default())
    });
}

