#[cfg(feature = "BsSignRailroad")]
use leptos::*;
#[cfg(feature = "BsSignRailroad")]
///This icon requires the feature `BsSignRailroad` to be enabled.
#[component]
pub fn SignRailroad(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-sign-railroad" viewBox = "0 0 16 16" width = size.clone() height =
        size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M11.303 6.584h1.064c.592 0 .936.334.936.844a.79.79 0 0 1-.485.748l.536 1.074h-.59l-.467-.994h-.473v.994h-.521V6.584Zm.521.414v.861h.46c.292 0 .474-.14.474-.421 0-.286-.188-.44-.467-.44h-.467Zm-8.771-.414h1.064c.592 0 .936.334.936.844 0 .39-.242.654-.485.748l.536 1.074h-.59l-.467-.994h-.473v.994h-.521V6.584Zm.521.414v.861h.46c.292 0 .474-.14.474-.421 0-.286-.188-.44-.467-.44h-.467Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.95.435c.58-.58 1.52-.58 2.1 0l6.515 6.516c.58.58.58 1.519 0 2.098L9.05 15.565c-.58.58-1.519.58-2.098 0L.435 9.05a1.482 1.482 0 0 1 0-2.098L6.95.435Zm1.4.7a.495.495 0 0 0-.7 0L4.923 3.861 8 6.939l3.078-3.077L8.35 1.134Zm3.788 3.788L9.061 8l3.077 3.078 2.728-2.728a.495.495 0 0 0 0-.7l-2.728-2.727Zm-1.06 7.215L8 9.061l-3.077 3.077 2.727 2.728a.495.495 0 0 0 .7 0l2.727-2.728Zm-7.216-1.06L6.939 8 3.862 4.923 1.134 7.65a.495.495 0 0 0 0 .7l2.728 2.728Z"
        /> < title > { title } < / title > < / svg >
    }
}
