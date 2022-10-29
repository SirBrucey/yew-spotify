use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;
use yew::virtual_dom::VNode;
use yew_icons::{Icon, IconId};

use crate::Routes;

struct LinkData {
    name: &'static str,
    to: Routes,
    icon: IconId,
}

const LINKS: [LinkData; 4] = [
    LinkData {
        name: "Discover",
        to: Routes::Discover,
        icon: IconId::FontAwesomeSolidHouse,
    },
    LinkData {
        name: "Around You",
        to: Routes::AroundYou,
        icon: IconId::FontAwesomeRegularImage,
    },
    LinkData {
        name: "Top Artists",
        to: Routes::TopArtists,
        icon: IconId::FontAwesomeSolidUsers,
    },
    LinkData {
        name: "Top Charts",
        to: Routes::TopCharts,
        icon: IconId::FontAwesomeSolidHashtag,
    },
];


fn nav_items(onclick_cb: Option<Callback<MouseEvent>>) -> Html {
    html! {
        <div class={classes!("mt-10")}>
            {
                LINKS.iter().map(|link| {
                    html!{
                        <div class={classes!("flex", "flex-row", "justify-start", "items-center", "my-8", "text-sm", "font-medium", "text-gray-400", "hover:text-cyan-400")} onclick={onclick_cb.clone()}>
                            <Icon icon_id={link.icon} class={classes!("mr-2")}/>
                            <Link<Routes> to={link.to.clone()}>{ link.name}</Link<Routes>>
                        </div>
                }
                }).collect::<VNode>()
            }
        </div>
    }
}

#[function_component(SideBar)]
pub fn sidebar() -> Html {
    let mobile_menu_open = Rc::new(use_state(|| false));

    let mobile_menu_icon;
    let mobile_menu_classes;

    if !*mobile_menu_open {
        mobile_menu_icon = html! { <Icon icon_id={IconId::FontAwesomeSolidBars} onclick={Callback::from(move |_| mobile_menu_open.set(true))} /> };
        mobile_menu_classes = classes!("-left-full");
    } else {
        mobile_menu_icon = html! { <Icon icon_id={IconId::FontAwesomeSolidXmark} onclick={Callback::from(move |_| mobile_menu_open.set(false))} /> };
        mobile_menu_classes = classes!("left-0");
    };

    html! {
        <>
            <div class={classes!("md:flex", "hidden", "flex-col", "w-[240px]", "py-10", "px-4", "bg-[#191624]")}>
                { nav_items(None) }
            </div>

            // Mobile
            <div class={classes!("absolute", "md:hidden", "block", "top-6", "right-3")}>
                { mobile_menu_icon }
            </div>

            <div class={classes!("absolute", "top-0", "h-screen", "w-2/3", "bg-gradient-to-tl", "from-white/10", "to-[#483D8B]", "backdrop-blur-lg", "z-10", "p-6", "md:hidden", "smooth-transition", mobile_menu_classes)}>
                { nav_items(Some(Callback::from(move |_| mobile_menu_open.set(false)))) }
            </div>
        </>
    }
}
