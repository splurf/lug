use gloo::timers::callback::Timeout;
use leptos::{ev, html::Canvas, prelude::*};
use rand::Rng;
use std::{cell::RefCell, sync::atomic::AtomicBool};
use std::{rc::Rc, sync::atomic::Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

const PADDLE_WIDTH: f64 = 80.0;
const PADDLE_HEIGHT: f64 = 4.0;
const BALL_WIDTH: f64 = 10.0;
const BALL_HEIGHT: f64 = 4.0;
const SPEED: f64 = 0.75;

fn init_canvas(_ws: WriteSignal<Option<&'static str>>) -> NodeRef<Canvas> {
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
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        let mut rng = rand::rng();

        let mut player_x = (width - PADDLE_WIDTH) / 2.0;
        let mut cpu_x = (width - PADDLE_WIDTH) / 2.0;
        let mut ball_x = width / 2.0;
        let mut ball_y = height / 2.0;
        let mut ball_vx = SPEED;
        let mut ball_vy = SPEED;

        let paused = Rc::new(AtomicBool::new(true));

        let paused_clone = paused.clone();
        let timeout = Rc::new(RefCell::new(Some(Timeout::new(2048, move || {
            paused_clone.store(true, Ordering::Relaxed)
        }))));

        let f = Rc::new(RefCell::new(Option::<Closure<_>>::None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let is_up = *up_pressed.borrow();
            let is_down = *down_pressed.borrow();

            if is_up {
                player_x -= 5.0;
                if player_x < 0.0 {
                    player_x = 0.0;
                }
                let paused_clone = paused.clone();
                *timeout.borrow_mut() = Some(Timeout::new(2048, move || {
                    paused_clone.store(true, Ordering::Relaxed)
                }));
                paused.store(false, Ordering::Relaxed);
            }
            if is_down {
                player_x += 5.0;
                if player_x + PADDLE_WIDTH > width {
                    player_x = width - PADDLE_WIDTH;
                }
                let paused_clone = paused.clone();
                *timeout.borrow_mut() = Some(Timeout::new(2048, move || {
                    paused_clone.store(true, Ordering::Relaxed)
                }));
                paused.store(false, Ordering::Relaxed);
            }

            // fallback to simple CPU model
            if !is_up && !is_down && paused.load(Ordering::Acquire) {
                if player_x + PADDLE_WIDTH / 2.0 < ball_x {
                    player_x += 3.5;
                } else {
                    player_x -= 3.5;
                }
                player_x = player_x.clamp(0.0, width - PADDLE_WIDTH);
            }

            // Simple CPU
            if cpu_x + PADDLE_WIDTH / 2.0 < ball_x {
                cpu_x += 3.5;
            } else {
                cpu_x -= 3.5;
            }
            cpu_x = cpu_x.clamp(0.0, width - PADDLE_WIDTH);

            ball_x += ball_vx;
            ball_y += ball_vy;

            if ball_x <= 0.0 || ball_x + BALL_WIDTH >= width {
                ball_vx = -ball_vx;
            }

            // Collision with player (bottom)
            if ball_y + BALL_HEIGHT >= height - PADDLE_HEIGHT
                && ball_x + BALL_WIDTH >= player_x
                && ball_x <= player_x + PADDLE_WIDTH
            {
                let direction = if rand::random_bool(0.5) { 1.0 } else { -1.0 };
                ball_vx = direction * SPEED * rng.random_range(1.0..2.0);
                ball_vy = -SPEED * rng.random_range(1.0..2.0);
            }

            // Collision with CPU (top)
            if ball_y <= PADDLE_HEIGHT
                && ball_x + BALL_HEIGHT >= cpu_x
                && ball_x <= cpu_x + PADDLE_WIDTH
            {
                let direction = if rand::random_bool(0.5) { 1.0 } else { -1.0 };
                ball_vx = direction * SPEED * rng.random_range(1.0..2.0);
                ball_vy = SPEED * rng.random_range(1.0..2.0);
            }

            // Score
            if ball_y < 0.0 || ball_y > height {
                ball_x = width / 2.0;
                ball_y = height / 2.0;
                ball_vx = SPEED * if rng.random_bool(0.5) { 1.0 } else { -1.0 };
                ball_vy = -SPEED * if rng.random_bool(0.5) { 1.0 } else { -1.0 };
                // _ws.set(Some("unimplemented")); TODO - finish caption strings
            }

            ctx.set_fill_style_str("#050c09");
            ctx.fill_rect(0.0, 0.0, width, height);

            ctx.set_fill_style_str("#78c297");
            ctx.fill_rect(
                player_x,
                height - PADDLE_HEIGHT,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            );
            ctx.fill_rect(cpu_x, 0.0, PADDLE_WIDTH, PADDLE_HEIGHT);
            ctx.fill_rect(ball_x, ball_y, BALL_WIDTH, BALL_HEIGHT);

            web_sys::window()
                .unwrap()
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }) as Box<dyn FnMut()>));

        web_sys::window()
            .unwrap()
            .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    });

    on_cleanup(move || {
        kb_handle.remove();
    });

    canvas_ref
}

#[component]
pub fn Pong() -> impl IntoView {
    let (read_str, write_str) = RwSignal::new(Option::<&'static str>::None).split();
    let canvas_ref = init_canvas(write_str);

    view! {
        <canvas
            node_ref=canvas_ref
            class="img-top-right img-inter img-style hide-if-tall"
            style="--x: 2%; --y: 25%; width: 15%; height: 50%; --rad: 0.2rem; background: radial-gradient(circle at top left,rgba(5, 12, 9, 0.57),rgba(0, 0, 0, 0.43))"
        />

        <p>{move || read_str.get()}</p>
    }
}
