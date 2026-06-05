// use crate::views::{Docking, Empty, Login};
use dioxus::prelude::*;

use crate::{
    FAVICON,
    component::{MdiIcon, icon::mdiOpenInNew},
    route::Route,
};

#[component]
pub fn Navbar() -> Element {
    rsx! {
        header {
            class: "z-20 px-5 pt-5 sm:px-6 fixed w-full",
            nav {
                class: "mx-auto flex max-w-6xl items-center justify-between px-4 py-3 border-[#ffffff14] border rounded-xl bg-[#ffffff0a] w-full backdrop-blur-md",
                div {
                    class: "flex items-center gap-5 w-full",
                    div {
                        class: "flex items-center gap-6 text-base font-medium text-slate-500 dark:text-slate-300 md:flex w-full",
                        a {
                            class: "flex min-w-0 items-center gap-3",
                            href: "https://github.com/cosmolkit/cosmolkit-tools-web",
                            span {
                                class: "font-bold text-lg",
                                span{
                                    class: "text-[#408cff]",
                                "COSMolkit "
                                }
                                span{
                                    class: "text-white",
                                "Tools Web"
                                }
                            }
                        }
                        a {
                            class: "uu-nav-link  ml-auto",
                            href: "#models",
                            "Tools",
                        }
                        a {
                            class: "uu-nav-link",
                            href: "#pricing",
                            "Docs",
                        }
                        Link {
                            class: "flex items-center",
                            new_tab: true,
                            to: "https://github.com/cosmol-studio/COSMolKit",
                            "Github",
                            span{
                                class: "ml-1",
                                MdiIcon {
                                    size: 16, path: mdiOpenInNew,
                                }
                            }
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
