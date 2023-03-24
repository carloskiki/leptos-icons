#[cfg(feature = "TiSortAlphabeticallyOutline")]
use leptos::*;
#[cfg(feature = "TiSortAlphabeticallyOutline")]
///This icon requires the feature `TiSortAlphabeticallyOutline` to be enabled.
#[component]
pub fn SortAlphabeticallyOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M5.618 14h2.764l-1.382-2.764zM21 14l2.4-3.2c.686-.915.795-2.119.284-3.142-.512-1.023-1.54-1.658-2.684-1.658h-6c-1.654 0-3 1.346-3 3 0 .77.295 1.469.774 2h-.274c-.368 0-.708.107-1.005.281l-1.811-3.623c-.498-.995-1.527-1.614-2.684-1.614s-2.186.619-2.684 1.614l-4 8c-.358.717-.416 1.53-.163 2.291s.788 1.376 1.504 1.735c.414.207.879.316 1.342.316 1.143 0 2.171-.635 2.684-1.657l.171-.343h2.291l.171.342c.512 1.023 1.54 1.658 2.684 1.658.464 0 .928-.109 1.342-.316.243-.122.455-.282.652-.458.54.488 1.246.774 2.006.774h6c1.654 0 3-1.346 3-3s-1.346-3-3-3zm-9.553 3.895c-.143.071-.296.105-.446.105-.368 0-.721-.203-.896-.553l-.723-1.447h-4.764l-.724 1.447c-.175.35-.528.553-.895.553-.15 0-.303-.034-.446-.106-.494-.247-.694-.848-.447-1.342l4-8c.169-.338.532-.508.894-.508s.725.169.895.508l4 8c.247.495.046 1.095-.448 1.343zm1.053-3.895c-.552 0-1-.448-1-1s.448-1 1-1h1c.552 0 1 .448 1 1s-.448 1-1 1h-1zm8.5 4h-6c-.379 0-.725-.214-.895-.553s-.132-.744.095-1.047l4.8-6.4h-4c-.552 0-1-.448-1-1s.448-1 1-1h6c.379 0 .725.214.895.553s.132.744-.095 1.047l-4.8 6.4h4c.552 0 1 .448 1 1s-.448 1-1 1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
