use seed::{prelude::IndexMap, Url};
use seed::{prelude::*, *};
use seed_styles::s;
use seed_styles::*;
use youtube_api::config::Config;
// use youtube_api::login_flow::get_token;
// use seed::prelude::web_sys::hash;
use youtube_api::login_flow::AuthenticationRedirectUrl;
use youtube_api::token::AccessTokenResponse;
use youtube_api::video::YoutubeVideo;
use youtube_api::{ClientError, YoutubeApi};
//use seed::prelude::web_sys::enable_style_sheets_for_set;
// mod user;
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        Msg::ConfigFetched(
            async { fetch("/config.json").await?.check_status()?.json().await }.await,
        )
    });

    let token = if let Some(hash) = url.hash() {
        AccessTokenResponse::get_token(hash.to_string())
    } else {
        AccessTokenResponse::default()
    };

    Model {
        authentication_redirect_url: AuthenticationRedirectUrl::default(),
        videos: Default::default(),
        response: token,
        error: None,
        api_key: Default::default(),
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    authentication_redirect_url: AuthenticationRedirectUrl,
    videos: Vec<YoutubeVideo>,
    response: AccessTokenResponse,
    error: Option<ClientError>,
    api_key: String,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    ListYoutubeVideos,
    ListMostPopularYoutubeVideos,
    ListYoutubeVideosSucceed(Vec<YoutubeVideo>),
    ListYoutubeVideosFailed(ClientError),
    ConfigFetched(fetch::Result<Config>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ListYoutubeVideos => {
            if !model.response.access_token.is_empty() {
                let token = &model.response.access_token;
                let key = &model.api_key;
                let mut api = YoutubeApi::new(token, key);
                orders.perform_cmd(async move {
                    let res = api.video().list("part=snippet&myRating=like").await;
                    match res {
                        Ok(videos) => Msg::ListYoutubeVideosSucceed(videos.items),
                        Err(e) => Msg::ListYoutubeVideosFailed(e),
                    }
                });
            }
        }
        Msg::ListMostPopularYoutubeVideos => {
            if !model.response.access_token.is_empty() {
                let token = &model.response.access_token;
                let key = &model.api_key;
                let mut api = YoutubeApi::new(token, key);
                orders.perform_cmd(async move {
                    let res = api
                        .video()
                        .list("part=snippet&chart=mostPopular&regionCode=US")
                        .await;
                    match res {
                        Ok(videos) => Msg::ListYoutubeVideosSucceed(videos.items),
                        Err(e) => Msg::ListYoutubeVideosFailed(e),
                    }
                });
            }
        }
        Msg::ListYoutubeVideosSucceed(videos) => {
            log!("load videos");
            model.videos = videos;
        }
        Msg::ListYoutubeVideosFailed(e) => {
            log!(e);
            model.error = Some(e);
        }

        Msg::ConfigFetched(Ok(config)) => {
            model.api_key = config.api_key.clone();
            model.authentication_redirect_url =
                AuthenticationRedirectUrl::new(config).build_full_url();
        }

        Msg::ConfigFetched(Err(fetch_error)) => log!(fetch_error),
    }
}

pub struct ListMostPopularVideos {}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        // YouTube button
        div![
            s().display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row),
            // YouTube logo div
            create_youtube_button(model)
        ],
        // List videos div
        button![
            "List videos I like",
            attrs! {At::Disabled => model.response.access_token.is_empty().as_at_value(),At::Color => "red"
            },
            // Click event
            ev(Ev::Click, |_| Msg::ListYoutubeVideos),
            style! {}
        ],
        button![
            "List most popular videos",
            attrs! {At::Disabled => model.response.access_token.is_empty().as_at_value(),At::Color => "red"
            },
            // Click event
            ev(Ev::Click, |_| Msg::ListMostPopularYoutubeVideos),
            style! {}
        ],
        display_videos(model)
    ]
}
/// Creates the YouTube button
fn create_youtube_button(model: &Model) -> Node<Msg> {
    log!(model.authentication_redirect_url.get_full_url());

    a![
        attrs! {
            At::Href => model.authentication_redirect_url.get_full_url()
        },
        button![
            s().display(CssDisplay::Flex)
                .align_items(CssAlignItems::Center),
            // YouTube logo
            img![
                attrs! {
                At::Src => "/public/images/yt_logo_rgb_light.png",
                },
                style! {
                        St::Height => "45px",
                        St::Width => "200px",
                }
            ],
            // Button style
            style! [
                St::Border => "none",
                St::BackgroundColor => "transparent"
            ],
        ]
    ]
}

// fn display_user_information(user: &Option<GoogleIdentifiedUser>) -> String {
//     match user {
//         None => "no user connected".to_string(),
//         Some(u) => u.name().to_string() + " " + &*u.email().to_string(),
//     }
// }
fn display_videos(model: &Model) -> Vec<Node<Msg>> {
    model.videos.iter().map(|v| show_description(v)).collect()
}

/// Display the description and title of the video
fn show_description(video: &YoutubeVideo) -> Node<Msg> {
    match &video.snippet {
        None => div!["no detail to show"],
        Some(s) => {
            div![h3![s.title.as_str()], p![s.description.as_str()]]
        }
    }
}

// ------ ------
//     Start
// ------ ------
// `wasm-bindgen` cannot transfer struct with public closures to JS (yet) so we have to send slice.
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
