#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
use app::ShowcaseApp;
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
fn main() {
	eframe::WebLogger::init(log::LevelFilter::Debug).ok();
	let web_options = eframe::WebOptions::default();
	wasm_bindgen_futures::spawn_local(async {
		let window = web_sys::window().expect("no global `window` exists");
		let document = window.document().expect("should have a document on window");
		let canvas_elem = document
			.get_element_by_id("the_canvas")
			.expect("Failed to find canvas with id 'the_canvas'");
		let canvas = canvas_elem
			.dyn_into::<web_sys::HtmlCanvasElement>()
			.expect("the_canvas was not a HtmlCanvasElement");

		let start_result = eframe::WebRunner::new()
			.start(
				canvas,
				web_options,
				Box::new(|cc| Ok(Box::new(ShowcaseApp::new(cc)))),
			)
			.await;
		if let Err(e) = start_result {
			log::error!("Failed to start eframe: {e:?}");
		}
	});
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
	let options = eframe::NativeOptions::default();
	eframe::run_native(
		"egui-elegant showcase",
		options,
		Box::new(|cc| Ok(Box::new(ShowcaseApp::new(cc)))),
	)
	.unwrap();
}
