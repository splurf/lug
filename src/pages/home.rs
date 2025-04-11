use crate::components::*;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <div>
                // top banner
                <header>
                    <A href="https://github.com/splurf/lug" style="left: 1rem;">
                        <img src="imgs/github-mark.png" alt="github" style="height: 1.4rem;" />
                    </A>

                    <A href="https://www.mtu.edu/">
                        <img src="imgs/mi-tech.png" alt="mi-tech" style="height: 1.8rem;" />
                    </A>

                    <A href="https://status.lug.mtu.edu/" style="right: 1rem;" class="status">
                        {"Status"}
                    </A>
                </header>

                // smug tux figure w/ caption
                <figure class="img-top-left img-inter" style="--x: 10%; --y: 25%; --size: 15%;">
                    <img
                        src="imgs/smug-tux.webp"
                        alt="smug-tux"
                        class="img-style"
                        style="width: 100%; height: auto;"
                    />
                    <figcaption>
                        {"\"We are all geeks at heart and love to talk about technology.\""}
                        {"  - the people of LUG."}
                    </figcaption>
                </figure>

                // tux bottom-right corner image
                <figure class="img-bottom-right img-inter" style="--x: 1%; --y: 1%; --size: 3.5%;">
                    <a href="https://lug.mtu.edu/" target="_blank" rel="noopener noreferrer">
                        <img src="imgs/tux.png" alt="Tux" style="width: 100%; height: auto;" />
                    </a>
                    <figcaption>{"Click Me!"}</figcaption>
                </figure>

                // main section
                <h1 class="section-header">{"Linux Users Group"}</h1>
                <section>
                    <h2>{"Why Join the Linux Users Group?"}</h2>
                    <p>
                        {"At Michigan Tech’s LUG, we’re passionate about Linux, open-source software, and empowering each other through knowledge sharing."}
                    </p>

                    <ul>
                        <li>{"Deepen your understanding of Linux and UNIX systems."}</li>
                        <li>
                            {"Discuss the latest trends in computing and open-source development."}
                        </li>
                        <li>
                            {"Get hands-on experience through workshops, events, and collaborative projects."}
                        </li>
                    </ul>

                    <p>
                        {"Everyone’s welcome — whether you’re a beginner or a seasoned hacker."}
                    </p>
                    <p style="text-align: center;">
                        <strong>
                            {"Join us Thursdays, 7–8 PM in Rekhi 101 (the round room)."}
                        </strong>
                    </p>

                    // related links
                    <footer>
                        <A href="https://discord.gg/5mZUgcNWAK">{"Discord"}</A>
                        |
                        <A href="https://lug.mtu.edu/minutes/">{"Meeting Minutes"}</A>
                        |
                        <A href="https://lug.mtu.edu/wiki/Main_Page">{"Wiki"}</A>
                    </footer>
                </section>
            </div>
        </ErrorBoundary>
    }
}
