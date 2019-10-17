use coffee::graphics::Window;

use coffee::ui::{UserInterface, Renderer, Element};

impl UserInterface for super::MapScreen {
    type Message = ();
    type Renderer = Renderer;

    fn react(&mut self, _message: (), _window: &mut Window) {}

    fn layout(&mut self, window: &Window) -> Element<()> {
        use coffee::ui::{Align, Column, Image, Justify, Text};
        use coffee::graphics::{HorizontalAlignment};

        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .align_items(Align::Center)
            .justify_content(Justify::Center)
            .padding(5)
            .spacing(5)
            .push(
                Text::new(&self.map.cpu_buffer.title)
                    .size(16)
                    .horizontal_alignment(HorizontalAlignment::Right)
            )
            .push(Image::new(&self.map.gpu_handler))
            .into()
    }
}
