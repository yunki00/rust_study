use ark_std::{UniformRand};
use rand::{thread_rng};
use ark_bls12_381::{G1Projective,G2Projective,Fr};

pub fn set_up(max_deg:u32) -> (Vec<G1Projective>, G2Projective, G2Projective){
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