#[cfg(feature = "SiFontawesome")]
use leptos::*;
#[cfg(feature = "SiFontawesome")]
///This icon requires the feature `SiFontawesome` to be enabled.
#[component]
pub fn Fontawesome(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M24 .857v18c-3.375 1.232-4.393 1.714-6.375 1.714-3.375 0-4.66-1.714-8.036-1.714-1.071 0-1.928.214-2.732.429v-3.429c.804-.214 1.661-.428 2.732-.428 3.375 0 4.661 1.714 8.036 1.714 1.071 0 1.875-.16 2.946-.482V5.518C19.5 5.839 18.696 6 17.625 6c-3.375 0-4.66-1.714-8.036-1.714-2.732 0-4.017 1.125-6.16 1.553v16.447A1.693 1.693 0 011.714 24 1.693 1.693 0 010 22.286V1.714A1.693 1.693 0 011.714 0 1.693 1.693 0 013.43 1.714v.697C5.572 1.982 6.857.857 9.589.857c3.375 0 4.661 1.715 8.036 1.715 1.982 0 3-.483 6.375-1.715Z"
        /> < title > { title } < / title > < / svg >
    }
}
