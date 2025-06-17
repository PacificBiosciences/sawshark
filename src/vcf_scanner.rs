use std::sync::mpsc::channel;

use camino::Utf8Path;
use rust_htslib::bcf::{self, Read};
use unwrap::unwrap;

use crate::annotations::AnnotationSeq;
use crate::globals::PROGRAM_VERSION;
use crate::test_seq_match::test_match;

struct VcfWriter {
    pub vcf: bcf::Writer,
}

impl VcfWriter {
    pub fn new(header_view: &bcf::header::HeaderView) -> Self {
        let format = bcf::Format::Vcf;

        let mut header = bcf::Header::from_template(header_view);

        if header_view.info_type(b"SVANN").is_err() {
            let header_info_record = br#"##INFO=<ID=SVANN,Number=.,Type=String,Description="Repeat annotation of structural variant">"#;
            header.push_record(header_info_record);
        }

        let sawshark_version = format!("##sawshark_version={PROGRAM_VERSION}");
        let sawshark_cmdline = format!(
            "##sawshark_cmdline={}",
            std::env::args().collect::<Vec<_>>().join(" ")
        );

        header.push_record(sawshark_version.as_bytes());
        header.push_record(sawshark_cmdline.as_bytes());

        let vcf = bcf::Writer::from_stdout(&header, true, format).unwrap();

        Self { vcf }
    }

    pub fn write(&mut self, record: &bcf::Record) {
        self.vcf.write(record).unwrap();
    }
}

struct AnnotationParams {
    pub min_score: i32,
    pub min_varseq_size: usize,
}

impl AnnotationParams {
    pub fn new(x: &AnnotationSeq, min_similarity: f64, min_fraction_of_template_size: f64) -> Self {
        let len = x.seq.len();
        let min_score = (len as f64 * min_similarity) as i32;
        let min_varseq_size = (len as f64 * min_fraction_of_template_size) as usize;
        Self {
            min_score,
            min_varseq_size,
        }
    }
}

fn test_allele_match(
    anno_seq: &AnnotationSeq,
    anno_param: &AnnotationParams,
    allele: &[u8],
) -> bool {
    if allele.len() < anno_param.min_varseq_size {
        false
    } else {
        test_match(&anno_seq.seq, allele, anno_param.min_score)
            || test_match(&anno_seq.rev_comp_seq, allele, anno_param.min_score)
    }
}

/// Scan VCF/BCF file to add annotations on SVs
///
pub fn scan_vcf_file(in_vcf: &Utf8Path, thread_count: usize, annotations: &[AnnotationSeq]) {
    // Hard-code pbsv-mode parameters for now
    let min_similarity = 0.6;
    let min_fraction_of_template_size = 0.75;

    let anno_params = annotations
        .iter()
        .map(|x| AnnotationParams::new(x, min_similarity, min_fraction_of_template_size))
        .collect::<Vec<_>>();
    let anno_params = &anno_params;

    let worker_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build()
        .unwrap();
    let (tx, rx) = channel();

    let mut reader = {
        if in_vcf.as_str() == "-" {
            bcf::Reader::from_stdin().unwrap()
        } else {
            assert!(!in_vcf.as_str().is_empty());
            bcf::Reader::from_path(in_vcf).unwrap()
        }
    };

    let header_view = reader.header();
    let mut vcf_writer = VcfWriter::new(header_view);
    let vcf_writer_ref = &mut vcf_writer;

    worker_pool.scope(move |scope| {
        let mut rec = reader.empty_record();
        let mut rec_index = 0;
        while let Some(r) = reader.read(&mut rec) {
            rec_index += 1;

            unwrap!(r, "Failed to parse variant record {rec_index}");

            vcf_writer_ref.vcf.translate(&mut rec);
            let tx = tx.clone();
            let mut rec = rec.clone();
            scope.spawn(move |_| {
                if rec.allele_count() == 2 {
                    let ref_allele = rec.alleles()[0];
                    let alt_allele = rec.alleles()[1];

                    if !alt_allele.is_empty() && alt_allele[0] != b'<' {
                        let mut svann = None;
                        for anno_index in 0..annotations.len() {
                            let anno_seq = &annotations[anno_index];
                            let anno_param = &anno_params[anno_index];
                            if test_allele_match(anno_seq, anno_param, alt_allele) {
                                svann = Some(&anno_seq.label);
                                break;
                            }
                            if test_allele_match(anno_seq, anno_param, ref_allele) {
                                svann = Some(&anno_seq.label);
                                break;
                            }
                        }

                        if let Some(svann) = svann {
                            rec.push_info_string(b"SVANN", &[svann]).unwrap();
                        }
                    }
                }

                tx.send((rec_index, rec)).unwrap();
            });
        }
    });

    // Sort to maintain input VCF record order in output:
    let mut result = rx.iter().collect::<Vec<_>>();
    result.sort_by_key(|x| x.0);

    for (_, rec) in result {
        vcf_writer.write(&rec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_allele_match() {
        use crate::annotations::get_default_annotations;

        let min_similarity = 0.6;
        let min_fraction_of_template_size = 0.75;

        let anno_seqs = get_default_annotations();
        let anno_seq = &anno_seqs[0];
        let anno_params =
            AnnotationParams::new(&anno_seq, min_similarity, min_fraction_of_template_size);

        let seq = b"ACAGTTCTTTTTTTTTTTTTTTGAGACGGAGTCTCGCTCTGTCGCCCAGGCTGGAGTGCAGTGGCGGGATCTCGGCTCACTGCAAGCTCCGCCTCCCGGGTTCACGCCATTCTCCTGCCTCAGCCTCCCGAGTAGCTGGGACTACAGGCGCCCGCCACCACGCCCGGCTAATTTTTTTGTATTTTTAGTAGAGACGGGGTTTCACCGTTTTAGCCGGGATGGTCTCGATCTCCTGACCTCGTGATCCGCCCGCCTCGGCCTCCCAAAGTGCTGGGATTACAGGCGTGAGCCACCGCGCCCGGCCC";

        let result = test_allele_match(&anno_seq, &anno_params, seq);

        assert!(result);
    }
}
