use eframe::egui::*;
use eframe::emath::Align;
use eframe::epaint::{FontFamily, Color32};
use serde::*;

pub const PADDING: f32 = 5.0;
const ORANGE: Color32 = Color32::from_rgb(255, 153, 51);

#[derive(Serialize, Deserialize)] // Serde for reading/writing stored data
pub struct HeadlinesConfig {
    pub is_dark_mode: bool,
    pub api_key: String,
    
}

pub struct Headlines {
	pub articles: Vec<NewsCardData>,
    pub config: HeadlinesConfig,
    pub is_api_key_initialised: bool,
    pub is_app_initialised: bool
}

pub struct NewsCardData {
	pub title: String,
	pub desc: String,
	pub url: String
}

impl Default for HeadlinesConfig {
    fn default() -> Self {
        Self { 
            is_dark_mode: Default::default(),
            api_key: String::new(), 
        }
    }
}

impl Headlines {
	
    pub fn load_config() -> Headlines {
		// Load config data
        let loaded_config: HeadlinesConfig = confy::load("headlines", "headlines_config").unwrap_or_default();
        
        // 20 objects lazily instantiated in a "for loop" like iterator.
		// let iter = (0..20).map(|a| NewsCardData{
		// 	title: format!("title{}", a),
		// 	desc: format!("description{}", a),
		// 	url: format!("https://examples.com{}", a)
		// });

		Headlines { 
            is_api_key_initialised: !loaded_config.api_key.is_empty(),
			articles: Vec::new(),
            // articles: Vec::from_iter(iter),
            config: loaded_config,
            is_app_initialised: false,
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

            ui.add_space(PADDING);
            
            let desc = Label::new(RichText::new(&a.desc).text_style(TextStyle::Button));
            ui.add(desc);
    
            // Mutating the style on the theme is good for small tweaks otherwise make a whole new theme.
            if self.config.is_dark_mode {
                ui.style_mut().visuals.hyperlink_color = Color32::LIGHT_BLUE;
            }
            else {
                ui.style_mut().visuals.hyperlink_color = ORANGE;
            }
           
            ui.add(Hyperlink::from_label_and_url(&a.url.to_string(), &a.url));
        };
    }

    pub fn render_top_panel(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Top panel widget
        TopBottomPanel::top("top_panel").show(ctx, |ui |{
            ui.add_space(6.);
            
            // Add menu bar
            eframe::egui::menu::bar(ui, |ui |{
                
                // Logo
                ui.with_layout(Layout::left_to_right(Align::Center), |ui |{
                    ui.add(Label::new(RichText::new("📓 News 📓").text_style(TextStyle::Heading)));
                });

                // Top left controls
                ui.with_layout(Layout::right_to_left(Align::Center), |ui |{
                    let close_btn = ui.add(Button::new("❌"));
                    if close_btn.clicked() {
                        _frame.close();
                    }
                    
                    let refresh_btn = ui.add(Button::new("🔄"));
                    if refresh_btn.clicked() {
                        tracing::info!("Refresh clicked!");
                    }

                    let theme_btn = ui.add(Button::new(
                        if self.config.is_dark_mode {
                            "🌙"        
                        }
                        else {
                            "🔆"
                        }
                    ));

                    if theme_btn.clicked() {
                        self.config.is_dark_mode = !self.config.is_dark_mode; // Inverts the dark mode config
                    }

                    if self.config.is_dark_mode {
                        
                    }
                });
            });
            
            ui.add_space(10.);
        });
    }

    // Popup window asking for the API key.
    pub fn render_config(&mut self, ctx: &Context) {
        Window::new("Configuration").show(ctx, |ui |{
            ui.label("Enter your API key, then press Enter:");

            let text_input = ui.text_edit_singleline(&mut self.config.api_key);
           
            if text_input.lost_focus() && ui.input().key_pressed(Key::Enter){
                
                if let Err (_e) = confy::store("headlines", "headlines_config", HeadlinesConfig {
                    is_dark_mode: self.config.is_dark_mode,
                    api_key: self.config.api_key.to_string(),
                }) {
                    tracing::error!("Failed saving app state {}", _e);
                }
                
                self.is_api_key_initialised = true;
                tracing::info!("API KEY SET");
            }

            ui.label("You can make a free account to get one on:");
            ui.hyperlink("https://newsapi.org")
        });
    }
}