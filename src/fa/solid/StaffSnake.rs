#[cfg(feature = "FaSolidStaffSnake")]
use leptos::*;
#[cfg(feature = "FaSolidStaffSnake")]
///This icon requires the feature `FaSolidStaffSnake` to be enabled.
#[component]
pub fn StaffSnake(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M222.6 43.2l-.2 4.8H288c53 0 96 43 96 96s-43 96-96 96H248V160h40c8.8 0 16-7.2 16-16s-7.2-16-16-16H248 220l-4.5 144H256c53 0 96 43 96 96s-43 96-96 96H240V384h16c8.8 0 16-7.2 16-16s-7.2-16-16-16H213l-3.1 99.5L208.5 495l0 1c-.3 8.9-7.6 16-16.5 16s-16.2-7.1-16.5-16l0-1-1-31H136c-22.1 0-40-17.9-40-40s17.9-40 40-40h36l-1-32H152c-53 0-96-43-96-96c0-47.6 34.6-87.1 80-94.7V256c0 8.8 7.2 16 16 16h16.5L164 128H136 122.6c-9 18.9-28.3 32-50.6 32H56c-30.9 0-56-25.1-56-56S25.1 48 56 48h8 8 89.5l-.1-4.8L161 32c0-.7 0-1.3 0-1.9c.5-16.6 14.1-30 31-30s30.5 13.4 31 30c0 .6 0 1.3 0 1.9l-.4 11.2zM64 112a16 16 0 1 0 0-32 16 16 0 1 0 0 32z"
        /> < title > { title } < / title > < / svg >
    }
}
