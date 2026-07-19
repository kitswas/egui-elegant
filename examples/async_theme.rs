use egui_elegant::{ElegantFont, ElegantTheme, ThemeMode};
use futures_lite::StreamExt;

fn main() {
	println!("Starting OS theme preference watcher stream...");

	// We create a stream of ElegantTheme updates
	let mut theme_stream =
		ElegantTheme::stream(ThemeMode::System, ElegantFont::default());

	// We block the main thread just for this demonstration.
	// In a real application, you'd spawn this task on an async runtime
	// like tokio or use an eframe background task.
	pollster::block_on(async {
		// The stream immediately yields the initial constructed theme,
		// then yields a new fully-constructed ElegantTheme anytime
		// the OS color scheme or accent color changes!
		while let Some(theme) = theme_stream.next().await {
			println!(
				"Received Theme Update:\n\tIs Dark Mode: {}\n\tAccent Color: {:?}",
				theme.is_dark, theme.primary
			);
		}
	});
}
