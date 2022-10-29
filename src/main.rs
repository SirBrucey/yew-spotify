use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Routes {
    #[at("/")]
    Discover,
    #[at("/top-artists")]
    TopArtists,
    #[at("/top-charts")]
    TopCharts,
    #[at("/around-you")]
    AroundYou,
    #[at("/artists/:id")]
    ArtistDetails { id: u32 },
    #[at("/song/:id")]
    SongDetails { id: u32 },
    #[at("/search/:search_term")]
    Search { search: String },
    #[not_found]
    #[at("/error")]
    Error,
}

fn placeholder(text: String) -> Html {
    html! {
        <div>{text}</div>
    }
}

fn switch(routes: &Routes) -> Html {
    match routes.clone() {
        Routes::Discover => placeholder("Discover".to_string()),
        Routes::TopArtists => placeholder("TopArtists".to_string()),
        Routes::TopCharts => placeholder("TopCharts".to_string()),
        Routes::AroundYou => placeholder("AroundYou".to_string()),
        Routes::ArtistDetails { id } => placeholder(format!("Artist {}", id)),
        Routes::SongDetails { id } => placeholder(format!("SongDetails {}", id)),
        Routes::Search { search } => placeholder(format!("Searching for {}", search)),
        Routes::Error => placeholder("Error: page not found".to_string()),
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
        <div class={classes!("relative", "flex")}>
            <div>{"Sidebar"}</div>
            <div class={classes!("flex", "flex-1", "flex-col", "bg-gradient-to-br", "from-black", "to-[#121286]")}>
                <div>{"Search Bar"}</div>
                <div class={classes!("px-6", "h-[calc(100vh-72px)]", "overflow-y-scroll", "hide-scrollbar", "flex", "xl:flex-row", "flex-col-reverse")}>
                    <div class={classes!("flex-1", "h-fit", "pb-40")}>
                        <Switch <Routes> render={Switch::render(switch)} />
                    </div>
                </div>
            </div>
        </div>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
