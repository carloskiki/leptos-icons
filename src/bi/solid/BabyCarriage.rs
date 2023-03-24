#[cfg(feature = "BiSolidBabyCarriage")]
use leptos::*;
#[cfg(feature = "BiSolidBabyCarriage")]
///This icon requires the feature `BiSolidBabyCarriage` to be enabled.
#[component]
pub fn BabyCarriage(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M21.666 12.277a7.72 7.72 0 0 0 .171-.665c.003-.017.004-.033.008-.05.02-.098.029-.199.045-.298.025-.157.055-.313.07-.471a7.979 7.979 0 0 0-2.303-6.45A7.979 7.979 0 0 0 14 2v8H6.517l-.858-2H2v2h2.341l1.828 4.266A3.504 3.504 0 0 0 4 17.5C4 19.43 5.57 21 7.5 21c1.759 0 3.204-1.309 3.449-3h2.102c.245 1.691 1.69 3 3.449 3 1.93 0 3.5-1.57 3.5-3.5 0-.63-.181-1.213-.473-1.725.042-.041.089-.077.131-.119.36-.361.688-.759.977-1.184.288-.43.536-.886.736-1.359.016-.037.026-.076.041-.113h.001l.015-.042c.088-.22.168-.441.235-.668l.003-.013zM7.5 19c-.827 0-1.5-.673-1.5-1.5S6.673 16 7.5 16s1.5.673 1.5 1.5S8.327 19 7.5 19zm9 0c-.827 0-1.5-.673-1.5-1.5s.673-1.5 1.5-1.5 1.5.673 1.5 1.5-.673 1.5-1.5 1.5z"
        /> < title > { title } < / title > < / svg >
    }
}
