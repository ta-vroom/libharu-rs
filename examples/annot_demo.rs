extern crate anyhow;
extern crate libharu;

use libharu::{prelude::*, Rectangle}; //{Document};

fn print_page(page: &Page, font: &Font<'_>, page_num: i32) {
    page.set_width(200.0).unwrap();
    page.set_height(200.0).unwrap();
    let page = PageDescriptionMode::new(page);
    page.set_font_and_size(font, 20.0).unwrap();
    page.run_text_mode(|page| {
        page.move_text_pos((50.0, 150.0)).unwrap();

        let buf = format!("Page:{}", page_num);
        let temp_buf = buf.as_str();

        page.show_text(temp_buf).unwrap();
        Ok(())
    })
    .unwrap()
}

fn main() {
    let doc = Document::new(|err| {
        println!("err={:?}", err);
    })
    .unwrap();

    let mut pages = Vec::<Page>::new();
    let fname = "TEST.pdf";
    let mut rect = Rectangle::default();
    let url = "http://libharu.org";

    /* create default-font */
    let font = doc.font("Helvetica", None).unwrap();

    /* create index page */
    let index_page = doc.add_page().unwrap();
    index_page.set_height(220.0).unwrap();
    index_page.set_width(300.0).unwrap();

    /* Add 7 pages to the document. */
    for i in 0..7 {
        pages.push(doc.add_page().unwrap());
        print_page(&pages[i], &font, i as i32 + 1);
    }
    let page = PageDescriptionMode::new(&index_page);
    page.run_text_mode(|page| {
        page.set_font_and_size(&font, 10.0).unwrap();
        page.move_text_pos((15.0, 200.0)).unwrap();
        page.show_text("Link Annotation Demo").unwrap();
        Ok(())
    })
    .unwrap();
    /*
     * Create Link-Annotation object on index page.
     */
    let mut tp = Point::default();
    page.run_text_mode(|page| {
        page.set_font_and_size(&font, 8.0).unwrap();
        page.move_text_pos((20.0, 180.0)).unwrap();
        page.set_text_leading(23.0).unwrap();
        page.show_text("Jump to Page1 (HilightMode=HPDF_ANNOT_NO_HIGHTLIGHT)").unwrap();
        page.move_to_next_line().unwrap();
        tp = page.current_pos().unwrap();

        Ok(())
    })
    .unwrap();
    /* page1 (HPDF_ANNOT_NO_HIGHTLIGHT) */

    rect.lower_left.x = tp.x - 4.0;
    rect.lower_left.y = tp.y - 4.0;
    rect.upper_right.x = index_page.current_pos().unwrap().x + 4.0;
    rect.upper_right.y = tp.y + 10.0;

    let dst = pages[0].create_destination().unwrap();

    let annot = index_page.link_annot(rect, dst).unwrap();
    annot.set_highlight_mode(HighlightMode::NoHighlight).unwrap();

    // /* page2 (HPDF_ANNOT_INVERT_BOX) */
    // tp = HPDF_Page_GetCurrentTextPos(index_page);

    // HPDF_Page_ShowText(
    //     index_page,
    //     cstring!("Jump to Page2 (HilightMode=HPDF_ANNOT_INVERT_BOX)").as_ptr(),
    // );
    // rect.left = tp.x - 4.0;
    // rect.bottom = tp.y - 4.0;
    // rect.right = HPDF_Page_GetCurrentTextPos(index_page).x + 4.0;
    // rect.top = tp.y + 10.0;

    // HPDF_Page_MoveToNextLine(index_page);

    // let dst = HPDF_Page_CreateDestination(page[1]);

    // let annot = HPDF_Page_CreateLinkAnnot(index_page, rect, dst);

    // HPDF_LinkAnnot_SetHighlightMode(annot, HPDF_AnnotHighlightMode::HPDF_ANNOT_INVERT_BOX);

    // /* page3 (HPDF_ANNOT_INVERT_BORDER) */
    // tp = HPDF_Page_GetCurrentTextPos(index_page);

    // HPDF_Page_ShowText(
    //     index_page,
    //     cstring!("Jump to Page3 (HilightMode=HPDF_ANNOT_INVERT_BORDER)").as_ptr(),
    // );
    // rect.left = tp.x - 4.0;
    // rect.bottom = tp.y - 4.0;
    // rect.right = HPDF_Page_GetCurrentTextPos(index_page).x + 4.0;
    // rect.top = tp.y + 10.0;

    // HPDF_Page_MoveToNextLine(index_page);

    // let dst = HPDF_Page_CreateDestination(page[2]);

    // let annot = HPDF_Page_CreateLinkAnnot(index_page, rect, dst);

    // HPDF_LinkAnnot_SetHighlightMode(annot, HPDF_AnnotHighlightMode::HPDF_ANNOT_INVERT_BORDER);

    // /* page4 (HPDF_ANNOT_DOWN_APPEARANCE) */
    // tp = HPDF_Page_GetCurrentTextPos(index_page);

    // HPDF_Page_ShowText(
    //     index_page,
    //     cstring!("Jump to Page4 (HilightMode=HPDF_ANNOT_DOWN_APPEARANCE)").as_ptr(),
    // );
    // rect.left = tp.x - 4.0;
    // rect.bottom = tp.y - 4.0;
    // rect.right = HPDF_Page_GetCurrentTextPos(index_page).x + 4.0;
    // rect.top = tp.y + 10.0;

    // HPDF_Page_MoveToNextLine(index_page);

    // let dst = HPDF_Page_CreateDestination(page[3]);

    // let annot = HPDF_Page_CreateLinkAnnot(index_page, rect, dst);

    // HPDF_LinkAnnot_SetHighlightMode(annot, HPDF_AnnotHighlightMode::HPDF_ANNOT_DOWN_APPEARANCE);

    // /* page5 (dash border) */
    // tp = HPDF_Page_GetCurrentTextPos(index_page);

    // HPDF_Page_ShowText(index_page, cstring!("Jump to Page5 (dash border)").as_ptr());
    // rect.left = tp.x - 4.0;
    // rect.bottom = tp.y - 4.0;
    // rect.right = HPDF_Page_GetCurrentTextPos(index_page).x + 4.0;
    // rect.top = tp.y + 10.0;

    // HPDF_Page_MoveToNextLine(index_page);

    // let dst = HPDF_Page_CreateDestination(page[4]);

    // let annot = HPDF_Page_CreateLinkAnnot(index_page, rect, dst);

    // HPDF_LinkAnnot_SetBorderStyle(annot, 1.0, 3, 2);

    // /* page6 (no border) */
    // tp = HPDF_Page_GetCurrentTextPos(index_page);

    // HPDF_Page_ShowText(index_page, cstring!("Jump to Page6 (no border)").as_ptr());
    // rect.left = tp.x - 4.0;
    // rect.bottom = tp.y - 4.0;
    // rect.right = HPDF_Page_GetCurrentTextPos(index_page).x + 4.0;
    // rect.top = tp.y + 10.0;

    // HPDF_Page_MoveToNextLine(index_page);

    // let dst = HPDF_Page_CreateDestination(page[5]);

    // let annot = HPDF_Page_CreateLinkAnnot(index_page, rect, dst);

    // HPDF_LinkAnnot_SetBorderStyle(annot, 0.0, 0, 0);

    // /* page7 (bold border) */
    // tp = HPDF_Page_GetCurrentTextPos(index_page);

    // HPDF_Page_ShowText(index_page, cstring!("Jump to Page7 (bold border)").as_ptr());
    // rect.left = tp.x - 4.0;
    // rect.bottom = tp.y - 4.0;
    // rect.right = HPDF_Page_GetCurrentTextPos(index_page).x + 4.0;
    // rect.top = tp.y + 10.0;

    // HPDF_Page_MoveToNextLine(index_page);

    // let dst = HPDF_Page_CreateDestination(page[6]);

    // let annot = HPDF_Page_CreateLinkAnnot(index_page, rect, dst);

    // HPDF_LinkAnnot_SetBorderStyle(annot, 2.0, 0, 0);

    // /* URI link */
    // tp = HPDF_Page_GetCurrentTextPos(index_page);

    // HPDF_Page_ShowText(index_page, cstring!("URI (").as_ptr());
    // HPDF_Page_ShowText(index_page, url.as_ptr());
    // HPDF_Page_ShowText(index_page, cstring!(")").as_ptr());

    // rect.left = tp.x - 4.0;
    // rect.bottom = tp.y - 4.0;
    // rect.right = HPDF_Page_GetCurrentTextPos(index_page).x + 4.0;
    // rect.top = tp.y + 10.0;

    // HPDF_Page_CreateURILinkAnnot(index_page, rect, url.as_ptr());

    // HPDF_Page_EndText(index_page);

    /* save the document to a file */
    doc.save_to_file(fname);
}
