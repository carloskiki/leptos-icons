#[cfg(feature = "SiEnterprisedb")]
use leptos::*;
#[cfg(feature = "SiEnterprisedb")]
///This icon requires the feature `SiEnterprisedb` to be enabled.
#[component]
pub fn Enterprisedb(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0zM6.44 7.75c.072 0 .148.004.222.012l1.815.18a.384.384 0 0 1 .345.369v6.636c0 .186-.154.32-.345.301l-1.815-.18C5.47 14.95 4.5 13.918 4.5 12.762c0-.62.279-1.15.72-1.49-.441-.428-.72-1.011-.72-1.631 0-1.084.85-1.892 1.94-1.89zm11.12 0c1.09 0 1.94.807 1.94 1.89 0 .62-.278 1.204-.72 1.631.442.34.72.87.72 1.49 0 1.157-.967 2.19-2.16 2.307l-1.817.18c-.191.02-.345-.116-.345-.3V8.31c0-.185.154-.35.345-.369l1.817-.18c.074-.007.148-.011.22-.011zm-7.374 2H12c1.194 0 2.16.93 2.16 2.074v3.09c0 1.145-.972 2.086-2.166 2.086H10.18a.343.343 0 0 1-.34-.344v-.092c0-.34.187-.331.27-.34l.136-.011c1.216-.13 1.735-.404 1.754-.766h-1.82c-.202 0-.34-.195-.34-.388v-4.977c0-.184.154-.332.346-.332z"
        /> < title > { title } < / title > < / svg >
    }
}
