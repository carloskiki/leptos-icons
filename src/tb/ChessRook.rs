#[cfg(feature = "TbChessRook")]
use leptos::*;
#[cfg(feature = "TbChessRook")]
///This icon requires the feature `TbChessRook` to be enabled.
#[component]
pub fn ChessRook(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-chess-rook" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 16l-1.447 .724a1 1 0 0 0 -.553 .894v2.382h12v-2.382a1 1 0 0 0 -.553 -.894l-1.447 -.724h-8z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M8 16l1 -9h6l1 9" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M6 4l.5 3h11l.5 -3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 4v3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 4v3" /> < title > { title } < / title > < /
        svg >
    }
}
