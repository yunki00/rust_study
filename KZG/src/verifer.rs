use ark_bls12_381::{Bls12_381,G1Projective,G2Projective,Fr};
use ark_ec::{pairing::Pairing};



pub fn verifier_check(set_para:Vec<G1Projective>, g2_gen:G2Projective, g2_tau:G2Projective, input:Fr, output:Fr, commitment:G1Projective, proof:G1Projective) -> bool{

    let check1 = Bls12_381::pairing(proof,g2_tau - (g2_gen * input));

    let check2 = Bls12_381::pairing(commitment - (set_para[0] * output),g2_gen);

    let output: bool = check1 == check2;

    return output;
}