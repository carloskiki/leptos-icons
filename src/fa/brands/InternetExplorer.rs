#[cfg(feature = "FaBrandsInternetExplorer")]
use leptos::*;
#[cfg(feature = "FaBrandsInternetExplorer")]
///This icon requires the feature `FaBrandsInternetExplorer` to be enabled.
#[component]
pub fn InternetExplorer(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M483.049 159.706c10.855-24.575 21.424-60.438 21.424-87.871 0-72.722-79.641-98.371-209.673-38.577-107.632-7.181-211.221 73.67-237.098 186.457 30.852-34.862 78.271-82.298 121.977-101.158C125.404 166.85 79.128 228.002 43.992 291.725 23.246 329.651 0 390.94 0 436.747c0 98.575 92.854 86.5 180.251 42.006 31.423 15.43 66.559 15.573 101.695 15.573 97.124 0 184.249-54.294 216.814-146.022H377.927c-52.509 88.593-196.819 52.996-196.819-47.436H509.9c6.407-43.581-1.655-95.715-26.851-141.162zM64.559 346.877c17.711 51.15 53.703 95.871 100.266 123.304-88.741 48.94-173.267 29.096-100.266-123.304zm115.977-108.873c2-55.151 50.276-94.871 103.98-94.871 53.418 0 101.981 39.72 103.981 94.871H180.536zm184.536-187.6c21.425-10.287 48.563-22.003 72.558-22.003 31.422 0 54.274 21.717 54.274 53.722 0 20.003-7.427 49.007-14.569 67.867-26.28-42.292-65.986-81.584-112.263-99.586z"
        /> < title > { title } < / title > < / svg >
    }
}
