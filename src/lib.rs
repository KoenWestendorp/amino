pub const AMINO_ACIDS: [AminoAcid; 20] = [
    AminoAcid::Ala,
    AminoAcid::Cys,
    AminoAcid::Asp,
    AminoAcid::Glu,
    AminoAcid::Phe,
    AminoAcid::Gly,
    AminoAcid::His,
    AminoAcid::Ile,
    AminoAcid::Lys,
    AminoAcid::Lue,
    AminoAcid::Met,
    AminoAcid::Asn,
    AminoAcid::Pro,
    AminoAcid::Gln,
    AminoAcid::Arg,
    AminoAcid::Ser,
    AminoAcid::Thr,
    AminoAcid::Val,
    AminoAcid::Trp,
    AminoAcid::Tyr,
];

pub enum AminoAcid {
    /// Alanine (A)
    Ala,
    /// Cysteine (C)
    Cys,
    /// Aspartic acid (D)
    Asp,
    /// Glutamic acid (E)
    Glu,
    /// Phenylalanine (F)
    Phe,
    /// Glycine (G)
    Gly,
    /// Histidine (H)
    His,
    /// Isoleucine (I)
    Ile,
    /// Lysine (K)
    Lys,
    /// Leucine (L)
    Lue,
    /// Methionine (M)
    Met,
    /// Asparagine (N)
    Asn,
    /// Proline (P)
    Pro,
    /// Glutamine (Q)
    Gln,
    /// Arginine (R)
    Arg,
    /// Serine (S)
    Ser,
    /// Threonine (T)
    Thr,
    /// Valine (V)
    Val,
    /// Tryptophan (W)
    Trp,
    /// Tyrosine (Y)
    Tyr,
}

// Into strings.
impl AminoAcid {
    /// Return the full name.
    pub fn full(&self) -> &'static str {
        match self {
            AminoAcid::Ala => "alanine",
            AminoAcid::Cys => "cysteine",
            AminoAcid::Asp => "aspartic acid",
            AminoAcid::Glu => "glutamic acid",
            AminoAcid::Phe => "phenylalanine",
            AminoAcid::Gly => "glycine",
            AminoAcid::His => "histidine",
            AminoAcid::Ile => "isoleucine",
            AminoAcid::Lys => "lysine",
            AminoAcid::Lue => "luecine",
            AminoAcid::Met => "methionine",
            AminoAcid::Asn => "asparagine",
            AminoAcid::Pro => "proline",
            AminoAcid::Gln => "glutamine",
            AminoAcid::Arg => "arginine",
            AminoAcid::Ser => "serine",
            AminoAcid::Thr => "threonine",
            AminoAcid::Val => "valine",
            AminoAcid::Trp => "tryptophan",
            AminoAcid::Tyr => "tyrosine",
        }
    }

    /// Return the 3-letter code.
    pub fn short(&self) -> &'static str {
        match self {
            AminoAcid::Ala => "Ala",
            AminoAcid::Cys => "Cys",
            AminoAcid::Asp => "Asp",
            AminoAcid::Glu => "Glu",
            AminoAcid::Phe => "Phe",
            AminoAcid::Gly => "Gly",
            AminoAcid::His => "His",
            AminoAcid::Ile => "Ile",
            AminoAcid::Lys => "Lys",
            AminoAcid::Lue => "Lue",
            AminoAcid::Met => "Met",
            AminoAcid::Asn => "Asn",
            AminoAcid::Pro => "Pro",
            AminoAcid::Gln => "Gln",
            AminoAcid::Arg => "Arg",
            AminoAcid::Ser => "Ser",
            AminoAcid::Thr => "Thr",
            AminoAcid::Val => "Val",
            AminoAcid::Trp => "Trp",
            AminoAcid::Tyr => "Tyr",
        }
    }

    /// Return the 1-letter code.
    pub fn single(&self) -> char {
        match self {
            AminoAcid::Ala => 'A',
            AminoAcid::Cys => 'C',
            AminoAcid::Asp => 'D',
            AminoAcid::Glu => 'E',
            AminoAcid::Phe => 'F',
            AminoAcid::Gly => 'G',
            AminoAcid::His => 'H',
            AminoAcid::Ile => 'I',
            AminoAcid::Lys => 'K',
            AminoAcid::Lue => 'L',
            AminoAcid::Met => 'M',
            AminoAcid::Asn => 'N',
            AminoAcid::Pro => 'P',
            AminoAcid::Gln => 'Q',
            AminoAcid::Arg => 'R',
            AminoAcid::Ser => 'S',
            AminoAcid::Thr => 'T',
            AminoAcid::Val => 'V',
            AminoAcid::Trp => 'W',
            AminoAcid::Tyr => 'Y',
        }
    }
}

// From strings.
impl AminoAcid {
    /// Return the [`AminoAcid`] corresponding to a full name.
    pub fn from_full(name: &str) -> Option<AminoAcid> {
        let aa = match name {
            "alanine" => AminoAcid::Ala,
            "cysteine" => AminoAcid::Cys,
            "aspartic acid" => AminoAcid::Asp,
            "glutamic acid" => AminoAcid::Glu,
            "phenylalanine" => AminoAcid::Phe,
            "glycine" => AminoAcid::Gly,
            "histidine" => AminoAcid::His,
            "isoleucine" => AminoAcid::Ile,
            "lysine" => AminoAcid::Lys,
            "luecine" => AminoAcid::Lue,
            "methionine" => AminoAcid::Met,
            "asparagine" => AminoAcid::Asn,
            "proline" => AminoAcid::Pro,
            "glutamine" => AminoAcid::Gln,
            "arginine" => AminoAcid::Arg,
            "serine" => AminoAcid::Ser,
            "threonine" => AminoAcid::Thr,
            "valine" => AminoAcid::Val,
            "tryptophan" => AminoAcid::Trp,
            "tyrosine" => AminoAcid::Tyr,
            _ => return None,
        };
        Some(aa)
    }

    /// Return the [`AminoAcid`] corresponding to a 3-letter code.
    pub fn from_short(code: &str) -> Option<Self> {
        // Make sure we don't do extraneous work.
        if code.len() != 3 {
            return None;
        }

        let aa = match code.to_ascii_lowercase().as_str() {
            "ala" => AminoAcid::Ala,
            "cys" => AminoAcid::Cys,
            "asp" => AminoAcid::Asp,
            "glu" => AminoAcid::Glu,
            "phe" => AminoAcid::Phe,
            "gly" => AminoAcid::Gly,
            "his" => AminoAcid::His,
            "ile" => AminoAcid::Ile,
            "lys" => AminoAcid::Lys,
            "lue" => AminoAcid::Lue,
            "met" => AminoAcid::Met,
            "asn" => AminoAcid::Asn,
            "pro" => AminoAcid::Pro,
            "gln" => AminoAcid::Gln,
            "arg" => AminoAcid::Arg,
            "ser" => AminoAcid::Ser,
            "thr" => AminoAcid::Thr,
            "val" => AminoAcid::Val,
            "trp" => AminoAcid::Trp,
            "tyr" => AminoAcid::Tyr,
            _ => return None,
        };
        Some(aa)
    }

    /// Return the [`AminoAcid`] corresponding to a 1-letter code.
    pub fn from_one_letter(code: char) -> Option<Self> {
        let aa = match code.to_ascii_uppercase() {
            'A' => AminoAcid::Ala,
            'C' => AminoAcid::Cys,
            'D' => AminoAcid::Asp,
            'E' => AminoAcid::Glu,
            'F' => AminoAcid::Phe,
            'G' => AminoAcid::Gly,
            'H' => AminoAcid::His,
            'I' => AminoAcid::Ile,
            'K' => AminoAcid::Lys,
            'L' => AminoAcid::Lue,
            'M' => AminoAcid::Met,
            'N' => AminoAcid::Asn,
            'P' => AminoAcid::Pro,
            'Q' => AminoAcid::Gln,
            'R' => AminoAcid::Arg,
            'S' => AminoAcid::Ser,
            'T' => AminoAcid::Thr,
            'V' => AminoAcid::Val,
            'W' => AminoAcid::Trp,
            'Y' => AminoAcid::Tyr,
            _ => return None,
        };
        Some(aa)
    }
}

impl TryFrom<&str> for AminoAcid {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 1 {
            if let Some(single) = AminoAcid::from_one_letter(value.chars().next().unwrap()) {
                return Ok(single);
            } else {
                return Err(());
            }
        }

        if let Some(short) = AminoAcid::from_short(value) {
            return Ok(short);
        }

        if let Some(full) = AminoAcid::from_full(value) {
            return Ok(full);
        }

        Err(())
    }
}
