use tao::event::Event;
use tao::event::WindowEvent;
use tao::event_loop::ControlFlow;
use tao::event_loop::EventLoop;
use tao::platform::unix::{WindowBuilderExtUnix, WindowExtUnix};
use tao::window::Fullscreen;
use wry::WebViewBuilder;
use wry::WebViewBuilderExtUnix;

fn main() -> wry::Result<()> {
	let event_loop = EventLoop::new();

	let uri = std::env::args().skip(1)
		.next()
		.expect("Keine URI angegeben");

	let window = tao::window::WindowBuilder::default()
		.with_fullscreen(Some(Fullscreen::Borderless(None)))
		.with_title("Azubitafel")
		.with_default_vbox(false)
		.build(&event_loop)
		.expect("Fenster konnte nicht konfiguriert werden.");

	let webview = WebViewBuilder::new()
		.with_autoplay(true)
		.with_url(&uri)
		.with_new_window_req_handler(|_, _| wry::NewWindowResponse::Allow)
		.build_gtk(window.gtk_window())
		.expect("Fenster konnte nicht geöffnet werden");

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;

		match event {
			Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::ExitWithCode(0),
			_ => {}
		};
	});
}