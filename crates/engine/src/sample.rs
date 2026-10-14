//! A sample deck built entirely in code (our own text, shapes and data), for demos, screenshots
//! and tests: `deckcraft --sample`.

use serde_json::{Value, json};

use crate::{Result, Session};

fn run(s: &mut Session, id: &str, p: Value) -> Result<Value> {
    s.execute(id, &p)
}

fn last_id(v: &Value) -> u64 {
    v.get("id").and_then(Value::as_u64).unwrap_or(0)
}

/// Open the sample presentation in a new document.
pub fn open_sample(s: &mut Session) -> Result<()> {
    run(s, "file.new", json!({"theme": "DeckCraft"}))?;
    let st = s.doc()?;
    let title = st.current_slide().and_then(|x| x.shapes.first()).map(|x| x.id.0).unwrap_or(0);
    let sub = st.current_slide().and_then(|x| x.shapes.get(1)).map(|x| x.id.0).unwrap_or(0);
    run(s, "text.set", json!({"id": title, "text": "北極光"}))?;
    run(s, "text.set", json!({"id": sub, "text": "一場極光之旅 — 它從哪裡來、什麼時候最容易看見，以及如何拍攝它"}))?;
    run(s, "slide.notes", json!({"text": "大家好。這份簡報是用 DeckCraft 製作的。"}))?;
    // Decorative bands on the title slide.
    let b = run(s, "shape.insert", json!({"preset": "rect", "rect": [0, 470, 960, 70]}))?;
    run(s, "shape.fill", json!({"id": last_id(&b), "gradient": {"stops": [[0, "accent1"], [0.5, "accent4"], [1, "accent2"]], "angle": 0}}))?;
    run(s, "shape.line", json!({"id": last_id(&b), "none": true}))?;
    run(s, "shape.rename", json!({"id": last_id(&b), "name": "Aurora Band"}))?;
    run(s, "transition.set", json!({"kind": "fade"}))?;

    // Agenda.
    run(s, "slide.new", json!({"layout": "titleAndContent", "title": "我們將介紹"}))?;
    let body = s.doc()?.current_slide().and_then(|x| x.shapes.get(1)).map(|x| x.id.0).unwrap_or(0);
    run(
        s,
        "text.set",
        json!({"id": body, "text": "極光科學：太陽風與磁層相遇\n何時與何地最容易看到\n\t高緯度、黑暗天空、漫長冬夜\n\tKp 指數與太空天氣預報\n如何拍攝極光\n現場安全與禮儀"}),
    )?;
    run(s, "transition.set", json!({"kind": "push", "option": "u"}))?;
    run(s, "edit.select", json!({"ids": [body]}))?;
    run(s, "animation.set", json!({"effect": "fade", "class": "entrance"}))?;
    run(s, "animation.options", json!({"textBuild": "byParagraph"}))?;

    // Section header.
    run(s, "slide.new", json!({"layout": "sectionHeader", "title": "科學原理", "body": "帶電粒子、磁場與發光氣體"}))?;
    run(s, "design.background", json!({"color": "dk2"}))?;
    let st = s.doc()?;
    let ids: Vec<u64> = st.current_slide().map(|x| x.shapes.iter().map(|s| s.id.0 as u64).collect()).unwrap_or_default();
    run(s, "edit.select", json!({"ids": ids}))?;
    run(s, "format.color", json!({"color": "#FFFFFF"}))?;
    run(s, "transition.set", json!({"kind": "wipe", "option": "r"}))?;

    // Process diagram from shapes.
    run(s, "slide.new", json!({"layout": "titleOnly", "title": "從太陽到你頭上的天空"}))?;
    let steps = [("太陽風", "accent2"), ("磁層", "accent1"), ("粒子撞擊", "accent4"), ("發光！", "accent3")];
    let mut prev: Option<u64> = None;
    for (i, (label, color)) in steps.iter().enumerate() {
        let x = 60.0 + i as f64 * 220.0;
        let v = run(s, "shape.insert", json!({"preset": if i == 3 { "star5" } else { "roundRect" }, "rect": [x, 210, 190, 120], "text": label}))?;
        let id = last_id(&v);
        run(s, "shape.fill", json!({"id": id, "color": color}))?;
        run(s, "shape.line", json!({"id": id, "none": true}))?;
        run(s, "shape.effects", json!({"id": id, "shadow": "bottom"}))?;
        run(s, "edit.select", json!({"ids": [id]}))?;
        run(s, "format.size", json!({"size": 17}))?;
        run(s, "format.bold", json!({"on": true}))?;
        run(s, "animation.add", json!({"effect": "zoom", "class": "entrance", "start": if i == 0 { "onClick" } else { "afterPrevious" }}))?;
        if let Some(p) = prev {
            let _ = p;
            let a = run(s, "shape.insert", json!({"preset": "rightArrow", "rect": [x - 36.0, 255, 32, 30]}))?;
            run(s, "shape.fill", json!({"id": last_id(&a), "color": "tx2"}))?;
            run(s, "shape.line", json!({"id": last_id(&a), "none": true}))?;
        }
        prev = Some(id);
    }
    let cap = run(s, "insert.textBox", json!({"rect": [60, 380, 840, 60], "text": "氧原子會發出綠色與紅色；氮則帶來藍色與紫色邊緣。"}))?;
    run(s, "edit.select", json!({"ids": [last_id(&cap)]}))?;
    run(s, "format.alignCenter", json!({}))?;
    run(s, "format.size", json!({"size": 20}))?;
    run(s, "format.italic", json!({"on": true}))?;
    run(s, "transition.set", json!({"kind": "morph"}))?;

    // Chart.
    run(s, "slide.new", json!({"layout": "titleOnly", "title": "最佳觀測月份（晴朗夜晚）"}))?;
    run(
        s,
        "insert.chart",
        json!({"type": "column", "rect": [80, 140, 800, 360], "title": "每月晴朗、黑暗夜晚平均數量", "categories": ["9月", "10月", "11月", "12月", "1月", "2月", "3月"], "series": [{"name": "特羅姆瑟", "values": [6, 8, 7, 6, 7, 9, 11]}, {"name": "費爾班克斯", "values": [9, 10, 8, 9, 11, 12, 13]}, {"name": "黃刀鎮", "values": [8, 11, 9, 10, 12, 13, 14]}]}),
    )?;
    run(s, "transition.set", json!({"kind": "fade"}))?;

    // Table.
    run(s, "slide.new", json!({"layout": "titleOnly", "title": "可用的相機設定"}))?;
    run(
        s,
        "insert.table",
        json!({"rows": 5, "cols": 3, "rect": [100, 150, 760, 260], "data": [["設定", "微弱極光", "明亮活躍極光"], ["光圈", "f/1.4 – f/2.8", "f/2.8"], ["快門", "8 – 15 秒", "1 – 4 秒"], ["ISO", "1600 – 3200", "800 – 1600"], ["對焦", "手動，對準亮星", "手動，對準亮星"]]}),
    )?;
    run(s, "transition.set", json!({"kind": "cover", "option": "l"}))?;

    // Two content: comparison with shapes.
    run(s, "slide.new", json!({"layout": "twoContent", "title": "現場檢查單"}))?;
    let st = s.doc()?;
    let (l, r) =
        st.current_slide().map(|x| (x.shapes.get(1).map(|s| s.id.0).unwrap_or(0), x.shapes.get(2).map(|s| s.id.0).unwrap_or(0))).unwrap_or((0, 0));
    run(s, "text.set", json!({"id": l, "text": "帶上\n\t三腳架與備用電池\n\t紅光模式頭燈\n\t保暖衣物與保溫瓶"}))?;
    run(s, "text.set", json!({"id": r, "text": "記得\n\t查看天氣預報與雲層狀況\n\t讓眼睛適應 20 分鐘\n\t留下痕跡不要留下垃圾"}))?;
    run(s, "transition.set", json!({"kind": "split"}))?;

    // Closing.
    run(s, "slide.new", json!({"layout": "title", "title": "晴空萬里！", "body": "由 DeckCraft 製作 — 自由且開源"}))?;
    let moon = run(s, "shape.insert", json!({"preset": "moon", "rect": [820, 40, 80, 120]}))?;
    run(s, "shape.fill", json!({"id": last_id(&moon), "color": "accent6"}))?;
    run(s, "shape.line", json!({"id": last_id(&moon), "none": true}))?;
    run(s, "shape.effects", json!({"id": last_id(&moon), "glow": {"color": "accent6", "radius": 10}}))?;
    run(s, "transition.set", json!({"kind": "fade"}))?;

    run(s, "section.add", json!({"name": "簡介", "at": 0}))?;
    run(s, "section.add", json!({"name": "科學", "at": 2}))?;
    run(s, "section.add", json!({"name": "現場", "at": 5}))?;
    run(s, "slide.go", json!({"index": 0}))?;
    run(s, "edit.deselect", json!({}))?;
    run(s, "edit.deselect", json!({}))?;
    // The sample opens clean: no undo history, not dirty.
    if let Some(st) = s.active_mut() {
        st.history.undo.clear();
        st.history.redo.clear();
        st.saved_doc = st.doc.clone();
        st.title = "北極光".into();
    }
    Ok(())
}
