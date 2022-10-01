use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Video {
    id: String,
    title: String,
    author: String,
}

#[derive(Clone, Properties, PartialEq)]
struct VideoDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
    html! {
        <div>
            <h3>{format!("{} - {}", video.author.clone(), video.title.clone())}</h3>
            <iframe id="player" type="text/html" width="600" height="400" src={format!("http://www.youtube.com/embed/{}?controls=0&origin=http://127.0.0.1", video.id.clone())} title={video.title.clone()} frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen=true></iframe>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let videos = vec![Video {
        id: "Yw6u6YkTgQ4".to_string(),
        title: "hello world".to_string(),
        author: "Louie Zong".to_string(),
    }];

    let videos = videos.iter().map(|video| {
        html! {
            <VideoDetails video={video.clone()}/>
        }
    });

    html! {
        <>
            <h1>{"Hello world videos!"}</h1>
            <div>
                {for videos}
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
