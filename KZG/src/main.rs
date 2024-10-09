use ark_std::{UniformRand};
use rand::{thread_rng};
use ark_bls12_381::{G1Projective,G2Projective,Fr};

mod setup;
mod prover;
mod verifer;

fn main() {
    let max_deg : u32 = 10; // commit할 수 있는 다항식 차수의 최댓값 - 1

    let set_para :Vec<G1Projective>;
    let g2_tau :G2Projective;
    let g2_gen :G2Projective;

    (set_para, g2_gen, g2_tau) = setup::set_up(max_deg); // set_para = [x^0, x^1, ...] 순서의 벡터

    let deg_poly = 7; // commit할 다항식의 차수-  1
    let mut poly:Vec<Fr> = Vec::new();

    let mut rng = thread_rng();

    for _ in 0..deg_poly {
        let random_fr = Fr::rand(&mut rng); // 랜덤한 Fr 생성
        poly.push(random_fr); // 벡터에 추가
    }

    let commitment: G1Projective = prover::prover_polycommit(&poly,&set_para);

    let input = Fr::rand(&mut rng);

    let output: Fr;
    let proof: G1Projective;

    (output, proof)= prover::prover_eval_proof(&poly,&set_para,input);

    let check: bool = verifer::verifier_check(set_para, g2_gen, g2_tau, input, output, commitment, proof);

    println!("{:?}",check);

}