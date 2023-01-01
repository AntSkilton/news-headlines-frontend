use eframe::egui::{FontDefinitions, TopBottomPanel, self, Layout, Label, Button, Context, Ui, FontData, RichText, Hyperlink, TextStyle};
use eframe::emath::Align;
use eframe::epaint::{FontFamily, Color32};

pub const PADDING: f32 = 5.0;
const ORANGE: Color32 = Color32::from_rgb(255, 153, 51);

pub struct Headlines {
	articles: Vec<NewsCardData>
}

struct NewsCardData {
	title: String,
	desc: String,
	url: String
}

impl Headlines {
	//pub fn init(){}
    
    pub fn new_dummy_data() -> Headlines {
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

	pub fn configure_fonts(&self, ctx: &Context) {
		// Create the font definiton object
		let mut font_def = FontDefinitions::default();

        // Load the font
        font_def.font_data.insert("Timeless".to_string(), 
        FontData::from_static(include_bytes!("../assets/Timeless.ttf"))); // Path is relative to this file.

		font_def.families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "Timeless"
        .to_string());

		// Set and load the font into the context
		ctx.set_fonts(font_def);
	}

    pub fn render_news_cards(&self, ui: &mut Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            ui.separator();

            ui.label(RichText::new(&a.title).text_style(TextStyle::Heading));
            ui.colored_label(ORANGE, &a.url);

            ui.add_space(PADDING);
            
            let desc = Label::new(RichText::new(&a.desc).text_style(TextStyle::Button));
            ui.add(desc);
    
            ui.style_mut().visuals.hyperlink_color = Color32::LIGHT_BLUE;
            ui.add(Hyperlink::from_label_and_url(&a.url.to_string(), &a.url));
        };
    }

    pub fn render_top_panel(&self, ctx: &Context) {
        // Top panel widget
        TopBottomPanel::top("top_panel").show(ctx, |ui |{
            ui.add_space(6.);
            
            // Add menu bar
            egui::menu::bar(ui, |ui |{
                
                // Logo
                ui.with_layout(Layout::left_to_right(Align::Center), |ui |{
                    ui.add(Label::new(RichText::new("📓 News 📓").text_style(TextStyle::Heading)));
                });

                // Controls
                ui.with_layout(Layout::right_to_left(Align::Center), |ui |{
                    let close_btn = ui.add(Button::new("❌"));
                    let refresh_btn = ui.add(Button::new("🔄"));
                    let theme_btn = ui.add(Button::new("🌙"));
                });
            });
            
            ui.add_space(10.);
        });
    }
}