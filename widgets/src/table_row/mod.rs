use iced_core::{layout, renderer, Border, Color, Element, Shadow, Size, Widget};

pub struct TableRow {
    row_height: f32,
}

impl TableRow {
    pub fn new() -> Self {
        Self { row_height: 15.0 }
    }

    pub fn row_height(mut self, height: f32) -> Self {
        self.row_height = height;
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for TableRow
where
    // Theme: Stylesheet,
    Renderer: iced_core::Renderer,
{
    fn size(&self) -> iced_core::Size<iced_core::Length> {
        Size::new(iced_core::Length::Fill, iced_core::Length::Fill)
    }

    fn layout(
        &self,
        _tree: &mut iced_core::widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(limits.max().width, self.row_height))
    }

    fn draw(
        &self,
        _tree: &iced_core::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        style: &renderer::Style,
        layout: iced_core::Layout<'_>,
        _cursor: iced_core::mouse::Cursor,
        _viewport: &iced_core::Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border {
                    radius: 5.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(1.0, 0.0, 0.0),
                },
                shadow: Shadow::default(),
            },
            Color::BLACK,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<TableRow> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(row: TableRow) -> Self {
        Self::new(row)
    }
}

// impl
