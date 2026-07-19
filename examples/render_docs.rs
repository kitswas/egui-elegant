use egui_elegant::{
	Alert, Avatar, Card, ElegantAccordion, ElegantBadge, ElegantButton, ElegantDropdown,
	ElegantFont, ElegantTabs, ElegantTagInput, ElegantTheme, ElegantToast, Progress,
	Skeleton, ThemeMode, Variant,
};
use egui_kittest::Harness;
use std::fs;

const OUTPUT_DIR: &str = "docs/images";

fn render_component<F>(name: &str, mut build_ui: F)
where
	F: FnMut(&mut egui::Ui) + 'static,
{
	let mut harness = Harness::builder()
		.with_size(egui::Vec2::new(800.0, 600.0))
		.wgpu()
		.build_ui(move |ui| {
			let theme = ElegantTheme::build(ThemeMode::Light, ElegantFont::default())
				.with_primary(egui::Color32::from_rgb(0, 118, 205));
			theme.apply(ui.ctx());

			let bg_color = theme.background;
			ui.painter().rect_filled(ui.max_rect(), 0.0, bg_color);

			egui::Frame::new().inner_margin(20.0).show(ui, |ui| {
				build_ui(ui);
			});
		});

	harness.step();
	harness.fit_contents();
	harness.step();
	harness.step();

	let image = harness.render().unwrap();
	let path = format!("{OUTPUT_DIR}/{name}.png");
	image.save(&path).expect("save png");
	println!("Saved {}", path);
}

fn main() {
	fs::create_dir_all(OUTPUT_DIR).expect("create output dir");

	render_component("buttons", |ui| {
		ui.horizontal(|ui| {
			ui.add(ElegantButton::new("Primary").variant(Variant::Primary));
			ui.add(ElegantButton::new("Secondary").variant(Variant::Secondary));
			ui.add(ElegantButton::new("Success").variant(Variant::Success));
			ui.add(ElegantButton::new("Danger").variant(Variant::Danger));
		});
	});

	render_component("badges", |ui| {
		ui.horizontal(|ui| {
			ui.add(ElegantBadge::new("Beta"));
			ui.add(ElegantBadge::new("v1.0.0"));
		});
	});

	render_component("alert", |ui| {
		ui.add(
			Alert::new("System Update", "Your system is up to date.")
				.variant(Variant::Info),
		);
	});

	render_component("progress", |ui| {
		ui.add(Progress::new(0.65));
	});

	render_component("accordion", |ui| {
		ElegantAccordion::new("acc1", "Settings").show(ui, |ui| {
			ui.label("Here are your settings.");
		});
	});

	render_component("avatar", |ui| {
		ui.horizontal(|ui| {
			ui.add(Avatar::new("JS"));
			ui.add(Avatar::new("AD"));
		});
	});

	render_component("card", |ui| {
		Card::new().show(ui, |ui| {
			ui.heading("Welcome");
			ui.label("This is a beautiful card.");
		});
	});

	let mut selected = 0;
	render_component("dropdown", move |ui| {
		ElegantDropdown::new("dd1", &mut selected)
			.options([(0, "Option A".into()), (1, "Option B".into())])
			.show(ui);
	});

	render_component("skeleton", |ui| {
		ui.horizontal(|ui| {
			ui.add(Skeleton::new(32.0, 32.0).rounding(16.0));
			ui.add(Skeleton::new(100.0, 16.0));
		});
	});

	let mut tab_idx = 0;
	render_component("tabs", move |ui| {
		ui.add(ElegantTabs::new(
			&["Overview", "Settings", "Activity"],
			&mut tab_idx,
		));
	});

	let mut tags = vec!["egui".into(), "rust".into()];
	let mut tag_text = String::new();
	render_component("taginput", move |ui| {
		ui.add(ElegantTagInput::new(&mut tags, &mut tag_text));
	});

	render_component("toast", |ui| {
		ElegantToast::new("toast1", "Notification", "This is a toast message.")
			.variant(Variant::Success)
			.show(ui.ctx());
	});

	println!("All screenshots generated in {}/", OUTPUT_DIR);
}
