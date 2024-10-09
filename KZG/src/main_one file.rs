use ark_std::{UniformRand};
use rand::{thread_rng};
use ark_bls12_381::{Bls12_381,G1Projective,G2Projective,Fr};
use ark_ec::{pairing::Pairing};




/*Prover.rs, Verifier.rs, Setup.rs + main.rs >>test (proof, verification)*/

fn main() {
    println!("Hello, world!");
    let max_deg : u32 = 5; // commit할 수 있는 다항식 차수의 최댓값 - 1

    let set_para :Vec<G1Projective>;
    let g2_tau :G2Projective;
    let g2_gen :G2Projective;
    (set_para, g2_gen, g2_tau) = set_up(max_deg); // set_para = [x^0, x^1, ...] 순서의 벡터

    let deg_poly = 4; // commit할 다항식의 차수-  1
    let mut poly:Vec<Fr> = Vec::new();

    let mut rng = thread_rng();
    for _ in 0..deg_poly {
        let random_fr = Fr::rand(&mut rng); // 랜덤한 Fr 생성
        poly.push(random_fr); // 벡터에 추가
    }

    let commitment: G1Projective = prover_polycommit(&poly,&set_para);

    let input = Fr::rand(&mut rng);

    let output: Fr;
    let proof: G1Projective;

    (output, proof)= prover_eval_proof(&poly,&set_para,input);

    let check: bool = verifier_check(set_para, g2_gen, g2_tau, input, output, commitment, proof);

    println!("{:?}",check);

}


fn set_up(max_deg:u32) -> (Vec<G1Projective>, G2Projective, G2Projective){
    let mut rng = thread_rng();
    let tau: Fr = Fr::rand(&mut rng);
    let g1_gen = G1Projective::rand(&mut rng);
    let g2_gen = G2Projective::rand(&mut rng);

    let mut set_para : Vec<G1Projective> = Vec::new();
    let mut tau_temp = Fr::from(1);

    for _ in 0..max_deg {
        let para : G1Projective = g1_gen * tau_temp;
        set_para.push(para);
        tau_temp = tau_temp * tau;
    }

    // println!("{:?}", g1_gen);
    // println!("{:?}", set_para);
    
    (set_para, g2_gen, g2_gen * tau)
}

fn prover_polycommit(poly:&Vec<Fr>,set_para:&Vec<G1Projective>) -> G1Projective{
    if poly.len() > set_para.len() {
        panic!("The degree of the polynomial is bigger than possible maximum degree.");
    }

    let mut commitment= set_para[0] * poly[0];

    for i in 1..poly.len() {
        commitment = set_para[i] * poly[i] + commitment;
    }

    return commitment;
}

fn prover_eval_proof(poly:&Vec<Fr>,set_para:&Vec<G1Projective>,input:Fr) -> (Fr,G1Projective){
    let mut out:Fr = Fr::from(0);
    let mut temp:Fr = Fr::from(1);

    for i in 0..poly.len() {
        out = out + (temp * poly[i]);
        temp = temp * input;
    }

    let mut q_poly:Vec<Fr> = poly.clone();

    q_poly[0] = q_poly[0] - out; // poly - b

    for i in (0..(poly.len() - 1)).rev(){
        q_poly[i] = q_poly[i] + (q_poly[i + 1] * input); // (poly - b) / (x - a) 
    }

    // q_poly[1..poly.len()] = (poly - b) / (x - a) 
    // println!("{:?}",q_poly[0]);

    let mut proof:G1Projective = set_para[0] * q_poly[1];

    for i in 2..poly.len() {
        proof = set_para[i-1] * q_poly[i] + proof;
    }

    return (out,proof);
}

fn verifier_check(set_para:Vec<G1Projective>, g2_gen:G2Projective, g2_tau:G2Projective, input:Fr, output:Fr, commitment:G1Projective, proof:G1Projective) -> bool{

    let check1 = Bls12_381::pairing(proof,g2_tau - (g2_gen * input));

    let check2 = Bls12_381::pairing(commitment - (set_para[0] * output),g2_gen);

    let output: bool = check1 == check2;

    return output;
}