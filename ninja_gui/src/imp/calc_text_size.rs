use crate::imp::structs::chars_iterator::CharsIterator;
use crate::imp::structs::chunk_size_calculator::{ChunkSizeCalculator, point_to_pixel};
use crate::imp::structs::gui_size::GuiSize;
use crate::imp::structs::text_size_calculator::TextSizeCalculator;
use crate::PistonGlyph;

pub(crate) fn calc_text_size(s : &str, font_size : usize, line_gap : usize, max_width : usize, glyph : &mut PistonGlyph) -> (Vec<String>, GuiSize){
    let mut chunk_size_calculator = ChunkSizeCalculator::new(font_size);
    let line_height = point_to_pixel(font_size);

    let mut text_size_calculator = TextSizeCalculator::new(line_height, line_gap, max_width);

    let chars = if let Some(chars) = CharsIterator::new(s){ chars } else{
        return text_size_calculator.deconstruct();
    };


    for c in chars {
        if c == '\n' || c == '\r'{
            if let Some(chunk) = chunk_size_calculator.flush(glyph) {
                text_size_calculator.write_chunk(chunk);
            }
            text_size_calculator.new_line()
        }
        else if let Some(chunk) = chunk_size_calculator.write(c, glyph) {
            text_size_calculator.write_chunk(chunk)
        }
    }
    if let Some(chunk) = chunk_size_calculator.flush(glyph) {
        text_size_calculator.write_chunk(chunk);
    }
    return text_size_calculator.deconstruct();
}

