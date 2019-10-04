use std::fs;
use day3::claim;

fn main() {

    let data = fs::read_to_string("easy_input")
                   .expect("Unable to read input file");

    let claims: Vec<claim::Claim> = data.split("\n")
                                 .map(|text| claim::build_claim_from_text(text).unwrap())
                                 .collect();

    let mut overlap_claims: Vec<claim::Claim> = Vec::new();

    let claims_copy = claims.to_vec();

    // how do I do this?
    for this_claim in claims {
        for other_claim in claims_copy {
            if (this_claim != other_claim) && !(this_claim.overlaps_with.contains(&other_claim.identity()))  {
                if let Some(over) = this_claim.overlapping_claim(&other_claim) {
                    overlap_claims.push(over);
                    this_claim.overlaps_with.insert(other_claim.identity());
                    other_claim.overlaps_with.insert(this_claim.identity());
    }}}}

}
