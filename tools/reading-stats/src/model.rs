use serde::Serialize;

const PROSE_WORDS_PER_MINUTE: f64 = 230.0;
const CODE_LINES_PER_MINUTE: f64 = 45.0;
const DIAGRAM_LINES_PER_MINUTE: f64 = 25.0;

#[derive(Debug, Serialize)]
pub struct ReadingStats {
    pub word_count: usize,
    pub reading_time_minutes: usize,
    pub prose_words: usize,
    pub code_lines: usize,
    pub diagram_lines: usize,
}

#[derive(Default)]
pub struct PageTally {
    pub prose_words: usize,
    pub code_lines: usize,
    pub diagram_lines: usize,
}

impl ReadingStats {
    pub fn from_tally(tally: PageTally) -> Self {
        let reading_time_minutes =
            if tally.prose_words == 0 && tally.code_lines == 0 && tally.diagram_lines == 0 {
                0
            } else {
                (((tally.prose_words as f64) / PROSE_WORDS_PER_MINUTE)
                    + ((tally.code_lines as f64) / CODE_LINES_PER_MINUTE)
                    + ((tally.diagram_lines as f64) / DIAGRAM_LINES_PER_MINUTE))
                    .ceil()
                    .max(1.0) as usize
            };

        Self {
            word_count: tally.prose_words,
            reading_time_minutes,
            prose_words: tally.prose_words,
            code_lines: tally.code_lines,
            diagram_lines: tally.diagram_lines,
        }
    }
}
