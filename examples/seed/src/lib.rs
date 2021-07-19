use crate::user::GoogleIdentifiedUser;
use enclose::enc;
use google_sign_in_wasm::GoogleUser;
use seed::{prelude::*, *};
use seed_styles::*;
use seed_styles::{px, s};
use serde::Deserialize;
use youtube_api::video::YoutubeVideo;
use youtube_api::{ClientError, YoutubeApi};

//use seed::prelude::web_sys::enable_style_sheets_for_set;

mod user;
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
pub fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        Msg::ConfigFetched(
            async { fetch("/config.json").await?.check_status()?.json().await }.await,
        )
    });
    Model {
        config: None,
        user: None,
        videos: Default::default(),
        error: None,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    config: Option<Config>,
    user: Option<GoogleIdentifiedUser>,
    videos: Vec<YoutubeVideo>,
    error: Option<ClientError>,
}

#[derive(Deserialize)]
pub struct Config {
    pub api_key: String,
    pub client_id: String,
}
// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    SignedIn(GoogleUser),
    SignedFailed(String),
    ListYoutubeVideos,
    ListYoutubeVideosSucceed(Vec<YoutubeVideo>),
    ListYoutubeVideosFailed(ClientError),
    ConfigFetched(fetch::Result<Config>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ListYoutubeVideos => {
            match &model.user {
                None => {
                    log!("You need to log a youtube user");
                }
                Some(u) => {
                    let token = u.access_token();
                    let config = model.config.as_ref().expect("Should have get config");
                    let key = &config.api_key;
                    let mut api = YoutubeApi::new(
                        token,
                        key
                    );
                    orders.perform_cmd(async move {
                        let res = api.video().list("part=snippet&myRating=like").await;
                        match res {
                            Ok(videos) => Msg::ListYoutubeVideosSucceed(videos.items),
                            Err(e) => Msg::ListYoutubeVideosFailed(e),
                        }
                    });
                }
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
        Msg::ConfigFetched(Ok(config)) => model.config = Some(config),
        Msg::ConfigFetched(Err(fetch_error)) => error!("Config fetch failed! Be sure to have config.json at the root of your project with client:di and api_key", fetch_error),
        Msg::SignedIn(user) => {
            log!("signed user detected");
           model.user = Some(GoogleIdentifiedUser::new(user.getBasicProfile()
                                                           .expect("Should have get profile"), 
                                                       user.getAuthResponse(true).unwrap().access_token().unwrap()));

            log!(model.user);
        }
        Msg::SignedFailed(_) => {}
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        // google_sign_in_wasm::sign_in::default_google_button(
        //     success(),
        //     fail(),
        //     "profile email https://www.googleapis.com/auth/youtube.readonly",
        //     &250,
        //     &50,
        //     "dark"
        // ),
        // p![display_user_information(&model.user)],
        img![attrs! {At::Src => "/public/images/yt_icon_rgb.png"}],
        div![button![s().height(px(45)).width(px(250))]],
        button![
            "List videos I like",
            attrs! {
              At::Disabled => model.user
                .is_none().as_at_value(),
              At::Color => "red"
            },
            ev(Ev::Click, |_| Msg::ListYoutubeVideos),
            style! {}
        ],
        display_videos(model)
    ]
}

fn display_user_information(user: &Option<GoogleIdentifiedUser>) -> String {
    match user {
        None => "no user connected".to_string(),
        Some(u) => u.name().to_string() + " " + &*u.email().to_string(),
    }
}
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

/// Contains the js function callback for google when the sign in succeeds.
const fn success() -> &'static str {
    "
    function on_success(user){
        sign_in(user);
    }
    "
}

/// Contains the js function callback for google when the sign in succeeds.
const fn fail() -> &'static str {
    "
    function on_failure(err){
        sign_failed(err);
    }
    "
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen]
// `wasm-bindgen` cannot transfer struct with public closures to JS (yet) so we have to send slice.
pub fn start() -> Box<[JsValue]> {
    let app = App::start("app", init, update, view);

    create_closures_for_js(&app)
}

/// Closure that triggers messages when getting update from js
fn create_closures_for_js(app: &App<Msg, Model, Node<Msg>>) -> Box<[JsValue]> {
    let sign_in = wrap_in_permanent_closure(enc!((app) move |user| {
        app.update(Msg::SignedIn(user))
    }));
    let sign_failed = wrap_in_permanent_closure(enc!((app) move |err| {
        app.update(Msg::SignedFailed(err))
    }));

    vec![sign_in, sign_failed].into_boxed_slice()
}

/// Make a perma closure
fn wrap_in_permanent_closure<T>(f: impl FnMut(T) + 'static) -> JsValue
where
    T: wasm_bindgen::convert::FromWasmAbi + 'static,
{
    // `Closure::new` isn't in `stable` Rust (yet) - it's a custom implementation
    // from Seed. If you need more flexibility, use `Closure::wrap`.
    let closure = Closure::new(f);
    let closure_as_js_value = closure.as_ref().clone();
    // `forget` leaks `Closure` - we should use it only when
    // we want to call given `Closure` more than once.
    closure.forget();
    closure_as_js_value
}
