use bellman::{
    groth16, Circuit, ConstraintSystem, SynthesisError
};
use bls12_381::Bls12;
use ff::PrimeField;
use pairing::Engine;
use rand::rngs::OsRng;


pub struct BitXORCircuit<Scalar:PrimeField> {
    pub a: Option<Scalar>,
    pub b: Option<Scalar>,
    pub c: Option<Scalar>
}

//  fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError>;
impl<Scalar: PrimeField> Circuit<Scalar> for BitXORCircuit<Scalar> {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError>{

        // Allocate the first private "auxiliary" variable, first get the value, then allocate a varialble
        let a = cs.alloc(|| "a", ||{self.a.ok_or(SynthesisError::AssignmentMissing)})?;
        let b = cs.alloc(|| "b", ||{self.b.ok_or(SynthesisError::AssignmentMissing)})?;
        let c = cs.alloc_input(|| "c", ||{self.c.ok_or(SynthesisError::AssignmentMissing)})?;


        // check (1-a) * a = 0
        cs.enforce(
            || "check a is 0 or 1",
            |lc: bellman::LinearCombination<_>| lc + CS::one() - a,
            |lc| lc + a,
            |lc| lc,
        );

        // check (1-b) * b = 0
        cs.enforce(
            || "check a is 0 or 1",
            |lc: bellman::LinearCombination<_>| lc + CS::one() - b,
            |lc| lc + b,
            |lc| lc,
        );

        // check (1-c) * c = 0
        cs.enforce(
            || "check a is 0 or 1",
            |lc: bellman::LinearCombination<_>| lc + CS::one() - c,
            |lc| lc + c,
            |lc| lc,
        );

        cs.enforce(
            || "xor constraint",
            |lc| lc + a + a,
            |lc| lc + b,
            |lc| lc + a + b - c,
        );

            
        Ok(())
    }
}




fn main(){

    println!("Creating parameters....");

    //Generate parameters through an empty circuit
    let params: groth16::Parameters<Bls12> = {
        let c = BitXORCircuit::<<Bls12 as Engine>::Fr>{ a: None, b:None, c:None};
        groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng).unwrap()

    };

    let pvk = groth16::prepare_verifying_key(&params.vk);

    let public_input = <Bls12 as Engine>::Fr::from_u128(1);

    let c = BitXORCircuit {
        a: Some(<Bls12 as Engine>::Fr::from_u128(1)),
        // when creating instance here, pass in Some of actual variables you're using
        b: Some(<Bls12 as Engine>::Fr::from_u128(0)),
        c: Some(public_input),
    };

       // Create a groth16 proof with our parameters.
    let proof = groth16::create_random_proof::<Bls12,_,_,_>(c, &params, &mut OsRng).unwrap();

    assert!(groth16::verify_proof(
        &pvk,
        &proof,
        &[public_input]).is_ok());
    
    println!("Proof verified successfully....");

}

#[test]
fn test_multiplier_success(){
    println!("Creating parameters....");

    //Generate parameters through an empty circuit
    let params: groth16::Parameters<Bls12> = {
        let c = BitXORCircuit::<<Bls12 as Engine>::Fr>{ a: None, b:None, c:None};
        groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng).unwrap()

    };

    let pvk = groth16::prepare_verifying_key(&params.vk);

    let public_input = <Bls12 as Engine>::Fr::from_u128(1);

    let c = BitXORCircuit {
        a: Some(<Bls12 as Engine>::Fr::from_u128(1)),
        // when creating instance here, pass in Some of actual variables you're using
        b: Some(<Bls12 as Engine>::Fr::from_u128(0)),
        c: Some(public_input),
    };

       // Create a groth16 proof with our parameters.
    let proof = groth16::create_random_proof::<Bls12,_,_,_>(c, &params, &mut OsRng).unwrap();

    assert!(groth16::verify_proof(
        &pvk,
        &proof,
        &[public_input]).is_ok());
    
    println!("Proof verified successfully....");
    
}

