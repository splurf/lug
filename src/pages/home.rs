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
                <header>
                    <A href="https://github.com/splurf/lug" style="left: 1rem;">
                        <img src="imgs/github-mark.png" alt="github" style="height: 1.2rem;" />
                    </A>

                    <A href="https://www.mtu.edu/">
                        <img src="imgs/mi-tech.png" alt="mi-tech" style="height: 1.75rem;" />
                    </A>

                    <A href="https://status.lug.mtu.edu/" style="right: 1rem;" class="status">
                        <img src="imgs/cog.png" alt="mi-tech" style="height: 1rem;" />
                    </A>
                </header>

                <figure
                    class="img-top-left img-inter hide-if-tall"
                    style="--x: 2%; --y: 20%; --size: 14%;"
                >
                    <img
                        src="imgs/smug-tux.webp"
                        alt="smug-tux"
                        class="img-style"
                        style="width: 100%; height: auto;"
                    />
                    <figcaption>
                        {"\"We are all geeks at heart and love to talk about technology.\""} <br />
                        {"- the people of LUG."}
                    </figcaption>
                </figure>

                <Pong />

                <figure
                    class="img-bottom-right img-inter hide-if-tall"
                    style="--x: 2%; --y: 12%; --size: 7%;"
                >
                    <a href="https://lug.mtu.edu/" target="_blank" rel="noopener noreferrer">
                        <img src="imgs/tux.png" alt="Tux" style="width: 100%; height: auto;" />
                    </a>
                    <figcaption style="margin-top: 0px; width: 100%; height: auto;">
                        {"Click Me!"}
                    </figcaption>
                </figure>

                <h1>{"Linux Users Group"}</h1>
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
