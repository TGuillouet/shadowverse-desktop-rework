use iced_core::{layout, renderer, Color, Element, Length, Padding, Point, Shadow, Size, Widget};

use self::style::Stylesheet;

pub mod style;

pub struct Column {
    pub name: String,
    pub resizable: bool,
    pub width: Length,
}

impl Column {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            resizable: true,
            width: Length::Fixed(150.0),
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
}

pub struct Header<'a, Message, Theme, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme, Renderer> Header<'a, Message, Theme, Renderer> {
    pub fn new(content: Element<'a, Message, Theme, Renderer>) -> Self {
        Self { content }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Header<'a, Message, Theme, Renderer>
where
    Theme: Stylesheet,
    Renderer: iced_core::Renderer,
{
    fn size(&self) -> iced_core::Size<iced_core::Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &self,
        tree: &mut iced_core::widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let padding: Padding = Padding::from([0.0, 10.0]);
        let limits = limits
            .loose()
            .width(Length::Fill)
            .height(Length::Fixed(30.0));

        let mut content = self
            .content
            .as_widget()
            .layout(&mut tree.children[0], renderer, &limits);
        let size = limits.resolve(Length::Fill, Length::Fixed(30.0), content.size());

        content = content
            .move_to(Point::new(padding.left, padding.top))
            .align(
                iced_core::Alignment::Start,
                iced_core::Alignment::Center,
                size,
            );

        layout::Node::with_children(size, vec![content])
    }

    fn draw(
        &self,
        tree: &iced_core::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
    ) {
        let active_theme = theme.active();
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: active_theme.border,
                shadow: Shadow::default(),
            },
            active_theme.background,
        );
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: Color::WHITE,
            },
            layout
                .children()
                .next()
                .expect("The component TableRow need to have content"),
            cursor,
            viewport,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Header<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Stylesheet + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(row: Header<'a, Message, Theme, Renderer>) -> Self {
        Self::new(row)
    }
}
