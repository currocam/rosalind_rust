const ADENINE_AS_BYTES: u8 = 65;
const CYTOSINE_AS_BYTES: u8 = 67;
const GUANINE_AS_BYTES: u8 = 71;
const THYMINE_AS_BYTES: u8 = 84;

pub struct CountingNucleotides {
    pub adenine: u32,
    pub cytosine: u32,
    pub guanine: u32,
    pub thymine: u32
}

pub fn count_nucleotides_dna(dna_string: &str) -> CountingNucleotides {
    let mut counting_nucleotides = CountingNucleotides {
        adenine: 0,
        cytosine: 0,
        guanine: 0,
        thymine: 0
    };
    for nucleotide in dna_string.as_bytes().into_iter() {
        match *nucleotide {
            ADENINE_AS_BYTES => counting_nucleotides.adenine += 1,
            CYTOSINE_AS_BYTES => counting_nucleotides.cytosine += 1,
            GUANINE_AS_BYTES => counting_nucleotides.guanine += 1,
            THYMINE_AS_BYTES => counting_nucleotides.thymine += 1,
            _ => panic!("An invalid value was found.")
        }
    }
    return counting_nucleotides;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_sample_dataset() {
        let dna_string = String::from("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC");
        let result = count_nucleotides_dna(&dna_string);
        assert_eq!(20, result.adenine);
        assert_eq!(12, result.cytosine);
        assert_eq!(17, result.guanine);
        assert_eq!(21, result.thymine);
    }
}
