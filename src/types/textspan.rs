use snafu::Snafu;
use wasm_bindgen::prelude::*;

#[cfg(test)]
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = TextSpan)]
    pub type TextSpanJs;
    #[wasm_bindgen(method, getter = start)]
    pub fn get_start(this: &TextSpanJs) -> usize;
    #[wasm_bindgen(method, getter = length)]
    pub fn get_length(this: &TextSpanJs) -> usize;
}

#[cfg(test)]
#[wasm_bindgen_test]
fn test_to_from_js() {
    let text_span = TextSpan::new(10, 10);
    let text_span_js: TextSpanJs = text_span.into();
    let text_span_converted: TextSpan = (&text_span_js).into();
    assert_eq!(text_span, text_span_converted);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TextSpan {
    start: usize,
    length: usize,
}

impl From<&TextSpanJs> for TextSpan {
    fn from(text_span_js: &TextSpanJs) -> TextSpan {
        TextSpan::new(text_span_js.get_start(), text_span_js.get_length())
    }
}

impl From<TextSpan> for TextSpanJs {
    fn from(text_span: TextSpan) -> Self {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"start".into(), &(text_span.start() as u32).into()).unwrap();
        js_sys::Reflect::set(&obj, &"length".into(), &(text_span.length() as u32).into()).unwrap();
        wasm_bindgen::JsCast::unchecked_into::<TextSpanJs>(obj)
    }
}

#[derive(Clone, Debug, Snafu)]
pub enum TextSpanError {
    #[snafu(display("The supplied end ({}) is smaller than the start ({})", end, start))]
    EndBeforeStart { start: usize, end: usize },
}

impl TextSpan {
    pub fn new(start: usize, length: usize) -> TextSpan {
        TextSpan { start, length }
    }

    pub fn from_bounds(start: usize, end: usize) -> Result<TextSpan, TextSpanError> {
        end.checked_sub(start)
            .ok_or(TextSpanError::EndBeforeStart { start, end })
            .map(|length| TextSpan::new(start, length))
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn end(&self) -> usize {
        self.start() + self.length()
    }

    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    pub fn contains_position(&self, position: usize) -> bool {
        position >= self.start() && position < self.end()
    }

    /// Returns true if 'span' contains 'other'.
    pub fn contains_text_span(&self, other: &TextSpan) -> bool {
        other.start() >= self.start() && other.end() <= self.end()
    }

    pub fn overlaps_with(&self, other: &TextSpan) -> bool {
        self.overlap(other).is_some()
    }

    pub fn overlap(&self, other: &TextSpan) -> Option<TextSpan> {
        self.intersection(other).filter(|span| span.length() != 0)
    }

    /// If the text spans intersect, return a new span with the intersection.
    /// If the text spans touch, return a text span with a length of 0.
    /// Otherwise, this method will return None.
    pub fn intersection(&self, span2: &TextSpan) -> Option<TextSpan> {
        let start = self.start().max(span2.start());
        let end = self.end().min(span2.end());
        if start <= end {
            TextSpan::from_bounds(start, end).ok()
        } else {
            None
        }
    }

    pub fn intersects_with(&self, other: &TextSpan) -> bool {
        other.start() <= self.end() && other.end() >= self.start()
    }

    pub fn intersects_with_position(&self, position: usize) -> bool {
        position <= self.end() && position >= self.start()
    }
}

#[wasm_bindgen(js_name=textSpanEnd)]
pub fn text_span_end(span: &TextSpanJs) -> usize {
    TextSpan::from(span).end()
}

#[wasm_bindgen(js_name=textSpanIsEmpty)]
pub fn text_span_is_empty(span: &TextSpanJs) -> bool {
    TextSpan::from(span).is_empty()
}

#[wasm_bindgen(js_name=textSpanContainsPosition)]
pub fn text_span_contains_position(span: &TextSpanJs, position: usize) -> bool {
    TextSpan::from(span).contains_position(position)
}

#[wasm_bindgen(js_name=textSpanContainsTextSpan)]
pub fn text_span_contains_text_span(span: &TextSpanJs, other: &TextSpanJs) -> bool {
    TextSpan::from(span).contains_text_span(&other.into())
}

#[wasm_bindgen(js_name=textSpanOverlapsWith)]
pub fn text_span_overlaps_with(span: &TextSpanJs, other: &TextSpanJs) -> bool {
    TextSpan::from(span).overlaps_with(&other.into())
}

#[wasm_bindgen(js_name=textSpanOverlap)]
pub fn text_span_overlap(span: &TextSpanJs, other: &TextSpanJs) -> Option<TextSpanJs> {
    TextSpan::from(span)
        .overlap(&other.into())
        .map(std::convert::Into::into)
}

#[wasm_bindgen(js_name=textSpanIntersectsWithTextSpan)]
pub fn text_span_intersects_with_text_span(span: &TextSpanJs, other: &TextSpanJs) -> bool {
    TextSpan::from(span).intersects_with(&other.into())
}

#[wasm_bindgen(js_name=textSpanIntersectsWith)]
pub fn text_span_intersects_with(span: &TextSpanJs, start: usize, length: usize) -> bool {
    TextSpan::from(span).intersects_with(&TextSpan::new(start, length))
}

#[wasm_bindgen(js_name=decodedTextSpanIntersectsWith)]
pub fn decoded_text_span_intersects_with(
    start1: usize,
    length1: usize,
    start2: usize,
    length2: usize,
) -> bool {
    TextSpan::new(start1, length1).intersects_with(&TextSpan::new(start2, length2))
}

#[wasm_bindgen(js_name=textSpanIntersectsWithPosition)]
pub fn text_span_intersects_with_position(span: &TextSpanJs, position: usize) -> bool {
    TextSpan::from(span).intersects_with_position(position)
}

#[wasm_bindgen(js_name=textSpanIntersection)]
pub fn text_span_intersection(span1: &TextSpanJs, span2: &TextSpanJs) -> Option<TextSpanJs> {
    TextSpan::from(span1)
        .intersection(&span2.into())
        .map(std::convert::Into::into)
}

#[wasm_bindgen(js_name=createTextSpan)]
pub fn create_text_span(start: isize, length: isize) -> Result<TextSpanJs, JsValue> {
    if start < 0 {
        Err(js_sys::Error::new("start < 0").into())
    } else if length < 0 {
        Err(js_sys::Error::new("length < 0").into())
    } else {
        Ok(TextSpan::new(start as usize, length as usize).into())
    }
}

#[wasm_bindgen(js_name=createTextSpanFromBounds)]
pub fn create_text_span_from_bounds(start: isize, end: isize) -> Result<TextSpanJs, JsValue> {
    create_text_span(start, end - start)
}
