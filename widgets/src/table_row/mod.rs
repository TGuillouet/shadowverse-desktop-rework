use iced_core::{
    layout, renderer, widget::Tree, Border, Color, Element, Length, Padding, Point, Renderer,
    Shadow, Size, Widget,
};

use self::style::Stylesheet;

pub mod style;

pub struct TableRow<'a, Message, Theme, Renderer> {
    row_height: f32,
    content: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme, Renderer> TableRow<'a, Message, Theme, Renderer> {
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            row_height: 15.0,
            content: content.into(),
        }
    }

    pub fn row_height(mut self, height: f32) -> Self {
        self.row_height = height;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for TableRow<'a, Message, Theme, Renderer>
where
    Theme: Stylesheet,
    Renderer: iced_core::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> iced_core::Size<iced_core::Length> {
        Size::new(Length::Fill, Length::Fixed(self.row_height))
    }

    fn layout(
        &self,
        tree: &mut iced_core::widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits
            .loose()
            .width(Length::Fill)
            .height(Length::Fixed(self.row_height));

        let mut content = self
            .content
            .as_widget()
            .layout(&mut tree.children[0], renderer, &limits);
        let size = limits.resolve(Length::Fill, Length::Fixed(self.row_height), content.size());

        content = content.move_to(Point::new(0.0, 0.0)).align(
            iced_core::Alignment::Start,
            iced_core::Alignment::Center,
            size,
        );

        layout::Node::with_children(size, vec![content])
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: iced_core::Event,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        _viewport: &iced_core::Rectangle,
    ) -> iced_core::event::Status {
        self.content.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        )
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

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
        renderer: &Renderer,
    ) -> iced_core::mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<TableRow<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Stylesheet + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(row: TableRow<'a, Message, Theme, Renderer>) -> Self {
        Self::new(row)
    }
}
