mod headlines; // Brings in the headlines.rs module

use eframe::egui::{Label, ScrollArea, CentralPanel, Ui, Separator, TopBottomPanel, Context, RichText, TextStyle, Visuals};
use eframe::epaint::{vec2};
use eframe:: {run_native, App};

use headlines:: {Headlines};
use headlines::PADDING;

impl App for Headlines {
	fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {	
		
		if self.config.is_dark_mode {
			ctx.set_visuals(Visuals::dark());
		} 
		else {
			ctx.set_visuals(Visuals::light());
		}

		if !self.is_api_key_initialised {
			self.render_config(ctx);
			return;
		}

		// Migrate to "OnStart()"
		self.configure_fonts(ctx);
		self.render_top_panel(ctx, _frame);
	
		CentralPanel::default().show(ctx, |ui| {

			render_header(ui);
			
			ScrollArea::vertical().show(ui, |ui| {
				self.render_news_cards(ui);
			});

			render_footer(ctx);
		});
	}
}

fn render_footer(ctx: &Context) {
	TopBottomPanel::bottom("footer").show(ctx, |ui | {
		ui.vertical_centered(|ui | {
			ui.add_space(3.);
			ui.add(Label::new(RichText::new("API Source: newsapi.org").text_style(TextStyle::Monospace)));
			ui.hyperlink_to("Made with egui.", "https://github.com/emilk/egui");
			ui.hyperlink_to("Written character by character by me, Ant!", "https://antskilton.github.io/");
			ui.add_space(3.);
		})
	});
}

fn render_header(ui: &mut Ui) {
	ui.vertical_centered(|ui |ui.heading("Top News Headlines"));
	ui.add_space(PADDING);
	
	let sep = Separator::default().spacing(20.);
	ui.add(sep);
}

fn main() {
	// Initialise tracing AKA debug.log()
	tracing_subscriber::fmt::init();

	let app = Headlines::new_dummy_data();
	let mut native_options = eframe::NativeOptions::default();
	native_options.initial_window_size = Some(vec2(540., 960.)); // The dot turns it from i32 into a float (f32)

	run_native("Headlines", native_options, Box::new(|_cc| Box::new(app)));
}