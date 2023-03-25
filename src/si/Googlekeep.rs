#[cfg(feature = "SiGooglekeep")]
use leptos::*;
#[cfg(feature = "SiGooglekeep")]
///This icon requires the feature `SiGooglekeep` to be enabled.
#[component]
pub fn Googlekeep(
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
        "M4.908 0c-.904 0-1.635.733-1.635 1.637v20.726c0 .904.732 1.637 1.635 1.637H19.09c.904 0 1.637-.733 1.637-1.637V6.5h-6.5V0H4.908zm9.819 0v6h6l-6-6zM11.97 8.229c.224 0 .571.031.765.072.2.04.576.185.842.312.828.414 1.467 1.164 1.774 2.088.168.511.188 1.34.05 1.865a3.752 3.752 0 0 1-1.277 1.952l-.25.193h-1.87c-2.134 0-1.931.042-2.478-.494a3.349 3.349 0 0 1-.984-1.844c-.148-.766-.053-1.437.32-2.203.19-.399.303-.556.65-.899.68-.679 1.513-1.037 2.458-1.042zm-1.866 7.863h3.781v1.328h-3.779v-1.328z"
        /> < title > { title } < / title > < / svg >
    }
}
