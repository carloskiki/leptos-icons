#[cfg(feature = "SiArtifacthub")]
use leptos::*;
#[cfg(feature = "SiArtifacthub")]
///This icon requires the feature `SiArtifacthub` to be enabled.
#[component]
pub fn Artifacthub(
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
        "M11.9999 24.00044c-.617.0012-1.24209-.17217-1.78008-.50002l-7.50975-4.29263c-1.01763-.61684-1.64001-1.71772-1.64066-2.9077V7.72971c0-1.25305.63694-2.36948 1.76008-3.01013L10.25041.47895c1.08003-.63867 2.41512-.63767 3.49515.001l7.41975 4.23763c1.08007.59613 1.7714 1.7341 1.76266 3.01013v8.58195c0 .96773-.44338 2.16388-1.63666 2.89856l-7.51074 4.2922c-.56347.34395-1.19861.50002-1.78167.50002zm-.50002-21.34695L3.95513 6.96224c-.2006.1567-.37906.36914-.37902.76747l.001 8.67039c.03753.22045.11891.42808.37302.63459l7.55975 4.31663c.26601.172.66403.21.98504 0l7.51792-4.29663c.23173-.14844.37102-.38858.41002-.65359V7.72971c.0095-.29884-.13595-.5886-.37702-.76547L12.49993 2.6525c-.39058-.23932-.7592-.15575-1.00004.001z"
        /> < title > { title } < / title > < / svg >
    }
}
