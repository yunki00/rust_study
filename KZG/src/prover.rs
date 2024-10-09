use ark_bls12_381::{G1Projective,Fr};

pub fn prover_polycommit(poly:&Vec<Fr>,set_para:&Vec<G1Projective>) -> G1Projective{
    if poly.len() > set_para.len() {
        panic!("The degree of the polynomial is bigger than possible maximum degree.");
    }

    let mut commitment= set_para[0] * poly[0];

    for i in 1..poly.len() {
        commitment = set_para[i] * poly[i] + commitment;
    }

    return commitment;
}

pub fn prover_eval_proof(poly:&Vec<Fr>,set_para:&Vec<G1Projective>,input:Fr) -> (Fr,G1Projective){
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
