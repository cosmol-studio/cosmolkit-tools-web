// use crate::views::{Docking, Empty, Login};
use dioxus::prelude::*;

use crate::{
    component::{MdiIcon, icon::MDI_OPEN_IN_NEW},
    route::Route,
};

#[component]
pub fn Navbar() -> Element {
    rsx! {
        header {
            class: "fixed top-0 z-20 w-full px-6 pt-5 font-sans max-[800px]:px-3.5 max-[800px]:pt-3.5",
            nav {
                class: "mx-auto flex min-h-[54px] w-full max-w-6xl items-center justify-between gap-6 rounded-lg border border-white/8 bg-[#0c1828]/90 px-4 shadow-[0_10px_32px_rgba(0,0,0,0.12)] backdrop-blur-md max-[800px]:min-h-[58px] max-[800px]:gap-3.5 max-[800px]:px-[13px]",
                Link {
                    class: "flex min-w-0 items-baseline gap-[5px] whitespace-nowrap text-[17px] font-extrabold text-white no-underline max-[800px]:block max-[800px]:text-[15px] max-[800px]:leading-[1.2]",
                    to: Route::Home {},
                    span {
                        class: "text-[#4b96ff] max-[800px]:block",
                        "COSMolkit"
                    }
                    span {
                        class: "max-[800px]:block",
                        "Tools Web"
                    }
                }
                div {
                    class: "flex items-center gap-6 max-[800px]:gap-3.5 max-[480px]:gap-[11px]",
                    Link {
                        class: "whitespace-nowrap text-sm font-semibold text-[#9baabd] no-underline hover:text-white max-[800px]:text-xs",
                        to: Route::ToolDirectory {},
                        "Tools",
                    }
                    Link {
                        class: "whitespace-nowrap text-sm font-semibold text-[#9baabd] no-underline hover:text-white max-[800px]:text-xs",
                        to: "https://kit.cosmol.org",
                        new_tab: true,
                        "Docs",
                    }
                    Link {
                        class: "inline-flex items-center gap-[5px] whitespace-nowrap text-sm font-semibold text-[#9baabd] no-underline hover:text-white max-[800px]:text-xs",
                        new_tab: true,
                        to: "https://github.com/cosmol-studio/cosmolkit-tools-web",
                        span { class: "max-[480px]:hidden", "Github" }
                        MdiIcon {
                            size: 15,
                            path: MDI_OPEN_IN_NEW,
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
