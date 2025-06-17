use parasail_rs::{Aligner, Matrix};

/// Test if pattern occurs in text at a minimum score threshold
///
/// This uses a variation on semi-global alignment for pbsv-compat mode, taking max last-row instead of best score,
/// effectively penalizing for gaps on the end (but not the start) of the mobile element sequence.
///
pub fn test_match(pattern: &[u8], text: &[u8], min_score: i32) -> bool {
    let match_score = 1;
    let mismatch_score = -2;
    let gap_open = 2;
    let gap_extend = 1;

    let matrix = Matrix::create(b"ACGT", match_score, mismatch_score).unwrap();
    let aligner = Aligner::new()
        .matrix(matrix)
        .semi_global()
        .gap_open(gap_open)
        .gap_extend(gap_extend)
        .use_last_rowcol()
        .striped()
        .build();

    let result = aligner.align(Some(pattern), text).unwrap();

    // Compared to get_score this forces scoring the gap on the 3' end of the query (ie. ALU/database) sequence
    //
    let max_row = *result.get_score_row().unwrap().iter().max().unwrap();

    max_row >= min_score
}
