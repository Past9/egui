use crate::{gui::Gui, math::*, types::*};

pub struct App {
    checked: bool,
    count: i32,
    selected_alternative: i32,

    width: f32,
    height: f32,
    corner_radius: f32,
    stroke_width: f32,
}

impl Default for App {
    fn default() -> App {
        App {
            checked: false,
            selected_alternative: 0,
            count: 0,
            width: 100.0,
            height: 50.0,
            corner_radius: 5.0,
            stroke_width: 2.0,
        }
    }
}

impl App {
    pub fn show_gui(&mut self, gui: &mut Gui) {
        gui.checkbox("checkbox", &mut self.checked);

        if gui
            .radio("First alternative", self.selected_alternative == 0)
            .clicked
        {
            self.selected_alternative = 0;
        }
        if gui
            .radio("Second alternative", self.selected_alternative == 1)
            .clicked
        {
            self.selected_alternative = 1;
        }
        if gui
            .radio("Final alternative", self.selected_alternative == 2)
            .clicked
        {
            self.selected_alternative = 2;
        }

        if gui.button("Click me").clicked {
            self.count += 1;
        }

        gui.label(format!("The button have been clicked {} times", self.count));

        gui.slider_f32("width", &mut self.width, 0.0, 500.0);
        gui.slider_f32("height", &mut self.height, 0.0, 500.0);
        gui.slider_f32("corner_radius", &mut self.corner_radius, 0.0, 50.0);
        gui.slider_f32("stroke_width", &mut self.stroke_width, 0.0, 10.0);

        gui.commands
            .push(GuiCmd::PaintCommands(vec![PaintCmd::Rect {
                corner_radius: self.corner_radius,
                fill_style: Some("#888888ff".into()),
                pos: vec2(300.0, 100.0),
                size: vec2(self.width, self.height),
                outline: Some(Outline {
                    width: self.stroke_width,
                    style: "#ffffffff".into(),
                }),
            }]));

        let commands_json = format!("{:#?}", gui.gui_commands());
        gui.label(format!("All gui commands: {}", commands_json));
    }
}
