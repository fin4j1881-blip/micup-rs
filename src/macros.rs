//! Micup Embed ve Bileşen Oluşturma Makroları.
//!
//! Hızlı kodlama (vibe coding) ve pratik bot geliştirme için pratik bildirim makroları.

/// Tek bildirimle anında `Embed` nesnesi oluşturur.
///
/// # Örnek
/// ```rust
/// use micup::embed;
///
/// let e = embed! {
///     title: "Hoş Geldiniz",
///     description: "Micup botuna hoş geldiniz!",
///     color: 0x7c5cff,
///     fields: [
///         ("Durum", "Aktif", true),
///         ("Geliştirici", "Kerem", true),
///     ],
/// };
/// ```
#[macro_export]
macro_rules! embed {
    (
        $(title: $title:expr,)?
        $(description: $desc:expr,)?
        $(color: $color:expr,)?
        $(fields: [ $( ($name:expr, $val:expr, $inline:expr) ),* $(,)? ],)?
    ) => {{
        let mut eb = $crate::embed::Embed::new();
        $( eb = eb.title($title); )?
        $( eb = eb.description($desc); )?
        $( eb = eb.color($color); )?
        $($(
            eb = eb.field($name, $val, $inline);
        )*)?
        eb
    }};
}

/// Tek bildirimle buton bileşeni (`Component::button`) oluşturur.
///
/// # Örnek
/// ```rust
/// use micup::button;
///
/// let b1 = button!("Kabul Et", "btn_accept", Success);
/// let b2 = button!("Reddet", "btn_reject", Danger);
/// let b3 = button!(link: "Micup Sitesi", "https://micup.gg");
/// ```
#[macro_export]
macro_rules! button {
    ($label:expr, $custom_id:expr, Primary) => {
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Primary,
            $label,
            $custom_id,
        )
    };
    ($label:expr, $custom_id:expr, Secondary) => {
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Secondary,
            $label,
            $custom_id,
        )
    };
    ($label:expr, $custom_id:expr, Success) => {
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Success,
            $label,
            $custom_id,
        )
    };
    ($label:expr, $custom_id:expr, Danger) => {
        $crate::components::Component::button(
            $crate::components::ButtonStyle::Danger,
            $label,
            $custom_id,
        )
    };
    (link: $label:expr, $url:expr) => {
        $crate::components::Component::link_button($label, $url)
    };
}
