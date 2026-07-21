use std::{collections::VecDeque, fmt, time::Duration};

use dioxus::{logger::tracing, prelude::*};
use web_time::Instant;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastType {
    Info,
    Success,
    Error,
}

impl fmt::Display for ToastType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Error => "error",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ToastState {
    Init,
    Normal,
    Out,
}

#[derive(Clone)]
struct Toast {
    id: u64,
    message: String,
    toast_type: ToastType,
    created_at: Instant,
    duration: Duration,
    state: ToastState,
}

#[derive(Clone, Copy)]
pub struct ToastManager {
    toasts: Signal<VecDeque<Toast>>,
    counter: Signal<u64>,
}

impl ToastManager {
    pub fn add(&mut self, message: impl Into<String>, toast_type: ToastType, duration: Duration) {
        let message = message.into();
        tracing::info!("Toast {toast_type}: {message}");

        let id = {
            let mut counter = self.counter.write();
            *counter += 1;
            *counter
        };

        self.toasts.write().push_back(Toast {
            id,
            message,
            toast_type,
            created_at: Instant::now(),
            duration,
            state: ToastState::Init,
        });
    }

    #[allow(dead_code)]
    pub fn info(&mut self, message: impl Into<String>) {
        self.add(message, ToastType::Info, Duration::from_secs(4));
    }

    #[allow(dead_code)]
    pub fn success(&mut self, message: impl Into<String>) {
        self.add(message, ToastType::Success, Duration::from_secs(3));
    }

    pub fn error(&mut self, message: impl Into<String>) {
        self.add(message, ToastType::Error, Duration::from_secs(5));
    }

    fn update(&mut self) {
        let now = Instant::now();
        let mut toasts = self.toasts.write();

        toasts.retain(|toast| {
            toast.state != ToastState::Out
                || now.duration_since(toast.created_at)
                    <= toast.duration + Duration::from_millis(350)
        });

        for toast in toasts.iter_mut() {
            if toast.state == ToastState::Init {
                toast.state = ToastState::Normal;
            }
            if now.duration_since(toast.created_at) > toast.duration {
                toast.state = ToastState::Out;
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn sleep_toast_tick() {
    gloo_timers::future::TimeoutFuture::new(100).await;
}

#[cfg(not(target_arch = "wasm32"))]
async fn sleep_toast_tick() {
    tokio::time::sleep(Duration::from_millis(100)).await;
}

#[component]
pub fn ToastProvider(children: Element) -> Element {
    let manager = ToastManager {
        toasts: use_signal(VecDeque::new),
        counter: use_signal(|| 0),
    };
    use_context_provider(|| manager);

    rsx! {
        {children}
        ToastList {}
    }
}

#[component]
fn ToastList() -> Element {
    let manager = use_context::<ToastManager>();

    use_future(move || async move {
        let mut manager = manager;
        loop {
            sleep_toast_tick().await;
            manager.update();
        }
    });

    rsx! {
        div {
            id: "toast-list",
            class: "pointer-events-none fixed top-[92px] right-6 z-[100] flex w-[min(360px,calc(100vw-48px))] flex-col gap-2.5 max-[640px]:top-[84px] max-[640px]:right-3.5 max-[640px]:left-3.5 max-[640px]:w-auto",
            role: "status",
            aria_live: "polite",
            for toast in manager.toasts.read().iter() {
                div {
                    key: "{toast.id}",
                    class: "pointer-events-auto flex min-h-12 items-center gap-3 rounded-md border px-3.5 py-3 text-[13px] leading-5 shadow-[0_16px_38px_rgba(0,0,0,0.32)] backdrop-blur-md transition-all duration-300 {toast_color_class(toast.toast_type)} {toast_state_class(toast.state)}",
                    span {
                        class: "grid h-5 w-5 shrink-0 place-items-center rounded-[4px] border text-xs leading-none font-extrabold {toast_icon_class(toast.toast_type)}",
                        aria_hidden: "true",
                        "{toast_icon(toast.toast_type)}"
                    }
                    span { class: "min-w-0 flex-1 font-semibold break-words", "{toast.message}" }
                }
            }
        }
    }
}

fn toast_color_class(toast_type: ToastType) -> &'static str {
    match toast_type {
        ToastType::Info => "border-[#315071] bg-[#102238]/95 text-[#b9d8ff]",
        ToastType::Success => "border-[#285b4d] bg-[#0d2923]/95 text-[#a7ead3]",
        ToastType::Error => "border-[#713d46] bg-[#301820]/95 text-[#f0a7b2]",
    }
}

fn toast_icon(toast_type: ToastType) -> &'static str {
    match toast_type {
        ToastType::Info | ToastType::Error => "!",
        ToastType::Success => "✓",
    }
}

fn toast_icon_class(toast_type: ToastType) -> &'static str {
    match toast_type {
        ToastType::Info => "border-[#4b96ff]/60 bg-[#173c66] text-[#9bc8ff]",
        ToastType::Success => "border-[#38c793]/60 bg-[#16483a] text-[#9aead0]",
        ToastType::Error => "border-[#e66b7c]/60 bg-[#5a2530] text-[#ffc1ca]",
    }
}

fn toast_state_class(state: ToastState) -> &'static str {
    match state {
        ToastState::Init => "translate-x-4 opacity-0",
        ToastState::Normal => "translate-x-0 opacity-100",
        ToastState::Out => "translate-x-4 opacity-0",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toast_types_have_explicit_symbols() {
        assert_eq!(toast_icon(ToastType::Info), "!");
        assert_eq!(toast_icon(ToastType::Success), "✓");
        assert_eq!(toast_icon(ToastType::Error), "!");
    }
}
