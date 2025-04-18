use gloo::timers::callback::Timeout;
use leptos::{ev, html::Canvas, prelude::*};
use rand::Rng;
use std::{cell::RefCell, sync::atomic::AtomicBool};
use std::{rc::Rc, sync::atomic::Ordering};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

const PADDLE_WIDTH: f64 = 80.0;
const PADDLE_HEIGHT: f64 = 4.0;
const BALL_WIDTH: f64 = 10.0;
const BALL_HEIGHT: f64 = 4.0;
const SPEED: f64 = 0.75;

#[derive(Default)]
struct State {
    pub player_x: f64,
    pub cpu_x: f64,
    pub ball_x: f64,
    pub ball_y: f64,
    pub ball_vx: f64,
    pub ball_vy: f64,
}

struct Context {
    pub width: f64,
    pub height: f64,
    pub ctx: Rc<CanvasRenderingContext2d>,
}

fn request_animation_frame_recursive(
    Context { width, height, ctx }: Context,
    mut rng: rand::prelude::ThreadRng,
    state: Rc<RefCell<State>>,
    paused: Rc<AtomicBool>,
    timeout: Rc<RefCell<Option<Timeout>>>,
    up: Rc<RefCell<bool>>,
    down: Rc<RefCell<bool>>,
) {
    request_animation_frame(move || {
        let mut st = state.borrow_mut();

        let is_up = *up.borrow();
        let is_down = *down.borrow();

        if is_up {
            st.player_x -= 5.0;
            st.player_x = st.player_x.max(0.0);
            *timeout.borrow_mut() = Some(Timeout::new(2048, {
                let paused = paused.clone();
                move || paused.store(true, Ordering::Relaxed)
            }));
            paused.store(false, Ordering::Relaxed);
        }
        if is_down {
            st.player_x += 5.0;
            st.player_x = st.player_x.min(width - PADDLE_WIDTH);
            *timeout.borrow_mut() = Some(Timeout::new(2048, {
                let paused = paused.clone();
                move || paused.store(true, Ordering::Relaxed)
            }));
            paused.store(false, Ordering::Relaxed);
        }

        if !is_up && !is_down && paused.load(Ordering::Acquire) {
            if st.player_x + PADDLE_WIDTH / 2.0 < st.ball_x {
                st.player_x += 3.5;
            } else {
                st.player_x -= 3.5;
            }
            st.player_x = st.player_x.clamp(0.0, width - PADDLE_WIDTH);
        }

        if st.cpu_x + PADDLE_WIDTH / 2.0 < st.ball_x {
            st.cpu_x += 3.5;
        } else {
            st.cpu_x -= 3.5;
        }
        st.cpu_x = st.cpu_x.clamp(0.0, width - PADDLE_WIDTH);

        st.ball_x += st.ball_vx;
        st.ball_y += st.ball_vy;

        if st.ball_x <= 0.0 || st.ball_x + BALL_WIDTH >= width {
            st.ball_vx = -st.ball_vx;
        }

        if st.ball_y + BALL_HEIGHT >= height - PADDLE_HEIGHT
            && st.ball_x + BALL_WIDTH >= st.player_x
            && st.ball_x <= st.player_x + PADDLE_WIDTH
        {
            let direction = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
            st.ball_vx = direction * SPEED * rng.random_range(1.0..2.0);
            st.ball_vy = -SPEED * rng.random_range(1.0..2.0);
        }

        if st.ball_y <= PADDLE_HEIGHT
            && st.ball_x + BALL_HEIGHT >= st.cpu_x
            && st.ball_x <= st.cpu_x + PADDLE_WIDTH
        {
            let direction = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
            st.ball_vx = direction * SPEED * rng.random_range(1.0..2.0);
            st.ball_vy = SPEED * rng.random_range(1.0..2.0);
        }

        if st.ball_y < 0.0 || st.ball_y > height {
            st.ball_x = width / 2.0;
            st.ball_y = height / 2.0;
            st.ball_vx = SPEED * if rng.random_bool(0.5) { 1.0 } else { -1.0 };
            st.ball_vy = -SPEED * if rng.random_bool(0.5) { 1.0 } else { -1.0 };
        }

        ctx.set_fill_style_str("#050c09");
        ctx.fill_rect(0.0, 0.0, width, height);
        ctx.set_fill_style_str("#78c297");
        ctx.fill_rect(
            st.player_x,
            height - PADDLE_HEIGHT,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );
        ctx.fill_rect(st.cpu_x, 0.0, PADDLE_WIDTH, PADDLE_HEIGHT);
        ctx.fill_rect(st.ball_x, st.ball_y, BALL_WIDTH, BALL_HEIGHT);

        request_animation_frame_recursive(
            Context { width, height, ctx },
            rng,
            state.clone(),
            paused,
            timeout,
            up,
            down,
        );
    });
}

fn init_canvas() -> NodeRef<Canvas> {
    let canvas_ref = NodeRef::new();

    let up_pressed = Rc::new(RefCell::new(false));
    let down_pressed = Rc::new(RefCell::new(false));

    let up_pressed_clone = up_pressed.clone();
    let down_pressed_clone = down_pressed.clone();

    let kb_handle = window_event_listener(ev::keydown, move |ev: KeyboardEvent| {
        match ev.key().as_str() {
            "ArrowLeft" => *up_pressed_clone.borrow_mut() = true,
            "ArrowRight" => *down_pressed_clone.borrow_mut() = true,
            _ => {}
        }
    });

    let up_pressed_clone = up_pressed.clone();
    let down_pressed_clone = down_pressed.clone();

    let _ku_handle = window_event_listener(ev::keyup, move |ev: KeyboardEvent| {
        match ev.key().as_str() {
            "ArrowLeft" => *up_pressed_clone.borrow_mut() = false,
            "ArrowRight" => *down_pressed_clone.borrow_mut() = false,
            _ => {}
        }
    });

    canvas_ref.on_load(move |canvas: HtmlCanvasElement| {
        let ctx = Rc::new(
            canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap(),
        );

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        let rng = rand::rng();

        let state = Rc::new(RefCell::new(State {
            player_x: (width - PADDLE_WIDTH) / 2.0,
            cpu_x: (width - PADDLE_WIDTH) / 2.0,
            ball_x: width / 2.0,
            ball_y: height / 2.0,
            ball_vx: SPEED,
            ball_vy: SPEED,
        }));

        let paused = Rc::new(AtomicBool::new(true));

        let timeout = Rc::new(RefCell::new(Some(Timeout::new(2048, {
            let paused = paused.clone();
            move || paused.store(true, Ordering::Relaxed)
        }))));

        request_animation_frame_recursive(
            Context { width, height, ctx },
            rng,
            state,
            paused,
            timeout,
            up_pressed.clone(),
            down_pressed.clone(),
        );
    });

    on_cleanup(move || {
        kb_handle.remove();
    });

    canvas_ref
}

#[component]
pub fn Pong() -> impl IntoView {
    let canvas_ref = init_canvas();

    view! {
        <div
            class="img-top-right img-inter img-style hide-if-tall"
            style="display: flex; flex-direction: column; align-items: center; --x: 2%; --y: 25%; width: 15%; height: 50%; --rad: 0.2rem;"
        >
            <canvas
                node_ref=canvas_ref
                style="width: 100%; height: 100%; background: radial-gradient(circle at top left,rgba(5, 12, 9, 0.57),rgba(0, 0, 0, 0.43))"
            />

            <p style="text-align: center; margin-top: 0.75vmin; margin-bottom: 0.75vmin; font-size: 1.215vmin">
                {"Press left/right arrow keys to move."}
            </p>
        </div>
    }
}
