// wadhu.com — one scrolling Makepad brand SPA (Khaynjo pattern).
// Wadhu = Memon *more* — Tools · Apps · More. Reusable tools, utils, and apps.
// Trademark: Gotigin. Company: GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED.
// Products under the brand: SMS Gateway, SMS Relay, Wadhu Connect.
// Privacy/Terms/About are pages of the same Hosting tree (HTML), not in-app routes.
script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let Night = #x0d0b3a
    let Indigo = #x161252
    let Navy = #x0b1f63
    let Cyan = #x00c8fc
    let Lime = #xadfb15
    let Ink = #xfcfbfb
    let Muted = #x9fb4d0
    let Select = #x24407aff

    mod.widgets.Label.draw_text.sdf_luma_bias = 0.0
    mod.widgets.Button.draw_text.sdf_luma_bias = 0.0
    mod.widgets.Markdown.draw_text.sdf_luma_bias = 0.0
    mod.widgets.Markdown.selectable = true
    mod.widgets.Markdown.draw_selection.color = Select
    mod.widgets.TextFlow.draw_selection.color = Select

    let Eyebrow = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Lime
        font_size: 13.
        text_style_normal: theme.font_bold{font_size: 13. line_spacing: 1.3}
        text_style_bold: theme.font_bold{font_size: 13. line_spacing: 1.3}
        draw_text +: { color: Lime text_style: theme.font_bold{font_size: 13. line_spacing: 1.3} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Headline = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Ink
        font_size: 48.
        text_style_normal: theme.font_bold{font_size: 48. line_spacing: 1.05}
        text_style_bold: theme.font_bold{font_size: 48. line_spacing: 1.05}
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 48. line_spacing: 1.05} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Lede = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Muted
        font_size: 19.
        text_style_normal: theme.font_regular{font_size: 19. line_spacing: 1.45}
        text_style_bold: theme.font_bold{font_size: 19. line_spacing: 1.45}
        draw_text +: { color: Muted text_style: theme.font_regular{font_size: 19. line_spacing: 1.45} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Body = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Muted
        // Normal body ~15.5 — align with Khaynjo/Moklanjo (phone pass 2026-11-22).
        font_size: 15.5
        text_style_normal: theme.font_regular{font_size: 15.5 line_spacing: 1.45}
        text_style_bold: theme.font_bold{font_size: 15.5 line_spacing: 1.45}
        draw_text +: { color: Muted text_style: theme.font_regular{font_size: 15.5 line_spacing: 1.45} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let CardTitle = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Cyan
        font_size: 22.
        text_style_normal: theme.font_bold{font_size: 22. line_spacing: 1.25}
        text_style_bold: theme.font_bold{font_size: 22. line_spacing: 1.25}
        draw_text +: { color: Cyan text_style: theme.font_bold{font_size: 22. line_spacing: 1.25} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let LinkBtn = Button{
        width: Fit height: Fit
        padding: Inset{left: 18. right: 18. top: 13. bottom: 13.}
        text: "Open"
        draw_bg +: { color: Cyan }
        draw_text +: { color: Night text_style: theme.font_bold{font_size: 14.} }
    }
    // 44 px tall: a finger-sized target on every tier.
    let GhostBtn = Button{
        width: Fit height: Fit
        padding: Inset{left: 14. right: 14. top: 13. bottom: 13.}
        draw_bg +: { color: Indigo }
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 13.} }
    }
    // Top-bar chip for windows from 480 px up.
    let NavBtn = Button{
        width: Fit height: Fit
        padding: Inset{left: 12. right: 12. top: 13. bottom: 13.}
        draw_bg +: { color: Indigo }
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 13.} }
    }
    // Phone top bar: four equal tabs share the row, so the bar is one row
    // at any phone width (Fit chips wrapped "Account" under the rest at 360).
    let TabBtn = Button{
        width: Fill height: Fit
        padding: Inset{left: 2. right: 2. top: 13. bottom: 13.}
        draw_bg +: { color: Indigo }
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 13.} }
    }
    // One unbreakable piece of a title. A row of these wraps between
    // pieces, so a product name never splits and no line starts with "·".
    let CardTitlePiece = Markdown{
        selectable: true
        height: Fit
        width: Fit
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Cyan
        font_size: 22.
        text_style_normal: theme.font_bold{font_size: 22. line_spacing: 1.25}
        text_style_bold: theme.font_bold{font_size: 22. line_spacing: 1.25}
        draw_text +: { color: Cyan text_style: theme.font_bold{font_size: 22. line_spacing: 1.25} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let PieceRow = View{
        width: Fill height: Fit
        flow: Right{wrap: true}
        spacing: 8.
        wrap_spacing: 2.
    }
    let Band = SolidView{
        width: Fill height: Fit
        padding: Inset{left: 64. right: 64. top: 64. bottom: 64.}
        draw_bg +: { color: Night }
        flow: Down
        spacing: 18.
    }
    let BandAlt = SolidView{
        width: Fill height: Fit
        padding: Inset{left: 64. right: 64. top: 64. bottom: 64.}
        draw_bg +: { color: Navy }
        flow: Down
        spacing: 18.
    }
    let Panel = RoundedView{
        width: Fill height: Fit
        padding: Inset{left: 24. right: 24. top: 22. bottom: 22.}
        draw_bg +: { color: Indigo }
        flow: Down
        spacing: 12.
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "Wadhu — Tools · Apps · More"
                window.inner_size: vec2(1280, 800)
                pass.clear_color: Night
                body +: {
                    width: Fill
                    height: Fill
                    flow: Down
                    shell := SolidView{
                        width: Fill height: Fill
                        draw_bg +: { color: Night }
                        flow: Down
                        // Same split as legal HTML `.topbar`: brand left, nav right.
                        top := SolidView{
                            width: Fill height: Fit
                            padding: Inset{left: 24. right: 24. top: 12. bottom: 12.}
                            draw_bg +: { color: Navy }
                            flow: Right
                            spacing: 16.
                            align: Align{y: 0.5}
                            // Logo + title; tap → `/`.
                            brand_home := View{
                                width: Fit height: Fit
                                flow: Right
                                spacing: 10.
                                align: Align{y: 0.5}
                                cursor: MouseCursor.Hand
                                brand_icon := Image{
                                    width: 40 height: 40
                                    fit: ImageFit.Stretch
                                    src: crate_resource("self:resources/media/wadhu-brand-logo.webp")
                                }
                                brand_lockup := View{
                                    width: Fit height: Fit
                                    flow: Down
                                    spacing: 1.
                                    align: Align{x: 0.0 y: 0.5}
                                    brand_name := Label{
                                        width: Fit
                                        text: "Wadhu"
                                        draw_text +: {
                                            color: Lime
                                            text_style: theme.font_bold{font_size: 14.}
                                        }
                                    }
                                    brand_tag := Label{
                                        width: Fit
                                        text: "Tools · Apps · More"
                                        draw_text +: {
                                            color: Muted
                                            text_style: theme.font_bold{font_size: 10.}
                                        }
                                    }
                                }
                            }
                            // Fill remaining width; children hug the right edge.
                            topnav := View{
                                width: Fill height: Fit
                                flow: Right { wrap: true }
                                spacing: 8.
                                wrap_spacing: 8.
                                align: Align{x: 1.0 y: 0.5}
                                nav_privacy := NavBtn{ text: "Privacy" }
                                nav_terms := NavBtn{ text: "Terms" }
                                nav_about := NavBtn{ text: "About" }
                                nav_account := NavBtn{ text: "Account" }
                            }
                            // Phone only (apply_responsive swaps it in for `topnav`).
                            topnav_tabs := View{
                                visible: false
                                width: Fill height: Fit
                                flow: Right
                                spacing: 6.
                                tab_privacy := TabBtn{ text: "Privacy" }
                                tab_terms := TabBtn{ text: "Terms" }
                                tab_about := TabBtn{ text: "About" }
                                tab_account := TabBtn{ text: "Account" }
                            }
                        }
                        // One scrolling SPA (Khaynjo). Legal pages are separate HTML
                        // in the same Hosting tree — opened via open_link.
                        page := ScrollYView{
                            width: Fill
                            height: Fill
                            flow: Down
                            selection_root: true
                            select_from_gaps: true

                            hero := Band{
                                hero_tag := View{
                                    width: Fill height: Fit
                                    Eyebrow{ body: "WADHU · BY GOTIGIN" }
                                }
                                hero_title := Headline{ body: "Wadhu means more." }
                                hero_lede := Lede{ body: "Memon for more. Reusable tools, utils, and apps — Tools · Apps · More." }
                                cta_row := View{
                                    width: Fill height: Fit
                                    flow: Right { wrap: true }
                                    spacing: 14.
                                    wrap_spacing: 10.
                                    cta_about := LinkBtn{ text: "About Wadhu" }
                                    cta_privacy := GhostBtn{ text: "Privacy policy" }
                                }
                            }

                            what := BandAlt{
                                Eyebrow{ body: "THE BRAND" }
                                what_title := Headline{ body: "Tools, utils, and apps." }
                                what_lede := Lede{ body: "Written once, so other Gotigin products and anyone else can take exactly the piece they need." }
                                Body{ body: "Libraries, small programs, and services live here. Each part stands on its own." }
                            }

                            reuse := Band{
                                Eyebrow{ body: "REUSE" }
                                reuse_title := Headline{ body: "Take one piece." }
                                Body{ body: "A crate is a library. An app is a program you run beside your own. A site is a page you visit. One part changing never forces a sibling to change." }
                            }

                            products := BandAlt{
                                Eyebrow{ body: "PRODUCTS" }
                                products_title := Headline{ body: "Under the Wadhu brand" }
                                products_lede := Lede{ body: "Products that ship under this brand." }
                                cards := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 18.
                                    sms := Panel{
                                        CardTitle{ body: "SMS Gateway" }
                                        Body{ body: "Android-first SIM gateway. LAN and Bluetooth stay free forever." }
                                    }
                                    relay := Panel{
                                        CardTitle{ body: "SMS Relay" }
                                        Body{ body: "Optional internet path to a paired gateway. Local send does not need it." }
                                    }
                                    conn := Panel{
                                        conn_mark := Image{
                                            width: 72 height: 72
                                            fit: ImageFit.Stretch
                                            src: crate_resource("self:resources/media/wadhu-connect-logo.webp")
                                        }
                                        CardTitle{ body: "Wadhu Connect" }
                                        Body{ body: "WhatsApp Cloud API Tech Provider bridge. Businesses keep their number; apps send and receive after Embedded Signup." }
                                        open_connect := LinkBtn{ text: "Open Wadhu Connect" }
                                    }
                                }
                            }

                            about := Band{
                                Eyebrow{ body: "NAMES" }
                                about_title := Headline{ body: "Who we are" }
                                names := View{
                                    width: Fill height: Fit
                                    flow: Down
                                    spacing: 18.
                                    View{
                                        width: Fill height: Fit
                                        flow: Down
                                        spacing: 4.
                                        Eyebrow{ body: "TRADEMARK" }
                                        CardTitle{ body: "Gotigin" }
                                        Body{ body: "Memon for “find it” · pronounced go-tee-geen" }
                                        Body{ body: "Where Technology Meets Discovery" }
                                    }
                                    View{
                                        width: Fill height: Fit
                                        flow: Down
                                        spacing: 4.
                                        Eyebrow{ body: "COMPANY" }
                                        CardTitle{ body: "GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED" }
                                        Body{ body: "mearaj@gotigin.com" }
                                    }
                                    View{
                                        width: Fill height: Fit
                                        flow: Down
                                        spacing: 4.
                                        Eyebrow{ body: "BRAND" }
                                        CardTitle{ body: "Wadhu" }
                                        Body{ body: "Memon for “more” · Tools · Apps · More" }
                                    }
                                    View{
                                        width: Fill height: Fit
                                        flow: Down
                                        spacing: 4.
                                        Eyebrow{ body: "PRODUCTS" }
                                        PieceRow{
                                            CardTitlePiece{ body: "SMS Gateway ·" }
                                            CardTitlePiece{ body: "SMS Relay ·" }
                                            CardTitlePiece{ body: "Wadhu Connect" }
                                        }
                                        Body{ body: "SIM gateway, optional relay, and a WhatsApp Cloud API bridge" }
                                    }
                                }
                            }

                            // Not `close`: that id is the window caption's close
                            // button, and a lookup by it never reaches this band.
                            closing := BandAlt{
                                Eyebrow{ body: "NEXT" }
                                closing_title := Headline{ body: "Build with Wadhu." }
                                Body{ body: "A wadhu.com account is for hosted features. Local tools over LAN and Bluetooth stay free." }
                                View{
                                    width: Fill height: Fit
                                    flow: Down
                                    spacing: 4.
                                    Body{ body: "© 2026 GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED" }
                                    Body{ body: "Gotigin · Wadhu · wadhu.com" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
