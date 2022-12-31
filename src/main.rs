use eframe::epaint::vec2;
use eframe:: {App, run_native};
use eframe::egui:: {CentralPanel, ScrollArea};

struct Headlines {
	articles: Vec<NewsCardData>
}

struct NewsCardData {
	title: String,
	desc: String,
	url: String
}

impl Headlines {
	fn dummy_data() -> Headlines {
		// 20 objects lazily instantiated in a "for loop" like iterator.
		let iter = (0..20).map(|a| NewsCardData{
			title: format!("title{}", a),
			desc: format!("description{}", a),
			url: format!("https://examples.com{}", a)
		});

		Headlines { 
			articles: Vec::from_iter(iter)
		}
	}
}

impl App for Headlines {
	fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
		CentralPanel::default().show(ctx, |ui| {
			
			ui.label("Top News Headlines");
			
			ScrollArea::always_show_scroll(ScrollArea::new([true; 2]), false).show(ui, |ui| {
				for a in &self.articles {
					ui.separator();
					ui.label(&a.title);
					ui.label(&a.url);
					ui.label(&a.desc);
				};
			});		
		});
	}
}

fn main() {
	let app = Headlines::dummy_data();
	let mut native_options = eframe::NativeOptions::default();
	native_options.initial_window_size = Some(vec2(540., 960.)); // The dot turns it from i32 into a float (f32)

	run_native("Headlines", native_options, Box::new(|_cc| Box::new(app)));
}