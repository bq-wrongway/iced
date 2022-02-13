pub mod image;

mod button;
mod checkbox;
mod column;
mod container;
mod element;
mod row;
mod scrollable;
mod slider;
mod text;
mod text_input;
mod tree;

pub use button::Button;
pub use checkbox::Checkbox;
pub use column::Column;
pub use container::Container;
pub use element::Element;
pub use image::Image;
pub use row::Row;
pub use scrollable::Scrollable;
pub use slider::Slider;
pub use text::Text;
pub use text_input::TextInput;
pub use tree::Tree;

use iced_native::event::{self, Event};
use iced_native::layout::{self, Layout};
use iced_native::mouse;
use iced_native::renderer;
use iced_native::{Clipboard, Hasher, Length, Point, Rectangle, Shell};

use std::any::{self, Any};

pub trait Widget<Message, Renderer> {
    fn tag(&self) -> any::TypeId;

    fn state(&self) -> Box<dyn Any>;

    fn children_state(&self) -> Vec<Tree>;

    fn width(&self) -> Length;

    fn height(&self) -> Length;

    fn hash_layout(&self, state: &mut Hasher);

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node;

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    );

    fn diff(&self, _tree: &mut Tree) {}

    fn mouse_interaction(
        &self,
        _state: &Tree,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse::Interaction::Idle
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        _event: Event,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        event::Status::Ignored
    }
}

pub fn container<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Renderer>>,
) -> Container<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    Container::new(content)
}

pub fn column<'a, Message, Renderer>() -> Column<'a, Message, Renderer> {
    Column::new()
}

pub fn row<'a, Message, Renderer>() -> Row<'a, Message, Renderer> {
    Row::new()
}

pub fn scrollable<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Renderer>>,
) -> Scrollable<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    Scrollable::new(content)
}

pub fn button<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Renderer>>,
) -> Button<'a, Message, Renderer> {
    Button::new(content)
}

pub fn text<Renderer>(text: impl Into<String>) -> Text<Renderer>
where
    Renderer: iced_native::text::Renderer,
{
    Text::new(text)
}

pub fn checkbox<'a, Message, Renderer>(
    label: impl Into<String>,
    is_checked: bool,
    f: impl Fn(bool) -> Message + 'a,
) -> Checkbox<'a, Message, Renderer>
where
    Renderer: iced_native::text::Renderer,
{
    Checkbox::new(is_checked, label, f)
}

pub fn text_input<'a, Message, Renderer>(
    placeholder: &str,
    value: &str,
    on_change: impl Fn(String) -> Message + 'a,
) -> TextInput<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced_native::text::Renderer,
{
    TextInput::new(placeholder, value, on_change)
}

pub fn slider<'a, Message, Renderer, T>(
    range: std::ops::RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> Slider<'a, T, Message>
where
    Message: Clone,
    Renderer: iced_native::Renderer,
    T: Copy + From<u8> + std::cmp::PartialOrd,
{
    Slider::new(range, value, on_change)
}

pub fn image<Handle>(handle: Handle) -> Image<Handle> {
    Image::new(handle)
}
