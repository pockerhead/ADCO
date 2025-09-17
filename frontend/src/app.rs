use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>

        // sets the document title
        <Title text="Welcome to ADCO"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to ADCO!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <Posts/>
    }
}

#[component]
fn Posts() -> impl IntoView {
    let async_data = Resource::new(|| (), |_| get_all_posts_names());

    let async_result = move || {
        let result = async_data.get();
        match result {
            Some(posts) => {
                match posts {
                    Ok(posts) => {
                        view! {
                            {posts.iter().map( |post| view! {
                                <li>{post.topic.clone()}</li>
                            }).collect::<Vec<_>>()}
                        }
                    }
                    Err(e) => vec![ view! { <li>"Error loading posts:" {e.to_string()}</li> } ],
                }
            }
            None => vec![ view! { <li>"Loading..."</li> } ],
        }
    };
    view! {
        <div>"POSTS"</div>
        <ul>
            <code>"VALUE:"</code>": "
            <Suspense fallback=|| "Loading...">
                {async_result}
            </Suspense>
            <br/>
        </ul>
    }
}

use adco_shared::post::Post;
#[server]
pub async fn get_all_posts_names() -> Result<Vec<Post>, ServerFnError> {
    use adco_backend::appstate::APP_STATE;
    use adco_backend::domain::infra::postgres::posts_repo::PostsRepositoryPostgres;

    let pg_pool = APP_STATE.get_pg_pool().await;
    let posts = PostsRepositoryPostgres::new(&pg_pool)
        .get_all_posts()
        .await?;
    Ok(posts)
}
