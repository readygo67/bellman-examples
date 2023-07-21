use bellman::{
    groth16, Circuit, ConstraintSystem, SynthesisError
};
use bls12_381::Bls12;
use ff::PrimeField;
use pairing::Engine;
use rand::rngs::OsRng;

// Multiplier circuit
// proving that I know a such that a * b = c
pub struct Multiplier<Scalar:PrimeField> {
    pub a: Option<Scalar>,
    pub b: Option<Scalar>,
    pub c: Option<Scalar>
}

//  fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError>;
impl<Scalar: PrimeField> Circuit<Scalar> for Multiplier<Scalar> {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError>{

        let a = cs.alloc(|| "a", ||{self.a.ok_or(SynthesisError::AssignmentMissing)})?;
        let b = cs.alloc(|| "b", ||{self.b.ok_or(SynthesisError::AssignmentMissing)})?;
        let c = cs.alloc_input(|| "c", ||{self.c.ok_or(SynthesisError::AssignmentMissing)})?;

        //a * b = c
        cs.enforce(|| "multiplier",
            |lc| lc + a,
            |lc| lc + b,
            |lc| lc + c);
            
        Ok(())
    }
}

fn main(){

    println!("Creating parameters....");

    //Generate parameters through an empty circuit
    let params: groth16::Parameters<Bls12> = {
        let c = Multiplier::<<Bls12 as Engine>::Fr>{ a: None, b:None, c:None};
        groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng).unwrap()

    };

    let pvk = groth16::prepare_verifying_key(&params.vk);

    let public_input = <Bls12 as Engine>::Fr::from_u128(20);

    let c = Multiplier {
        a: Some(<Bls12 as Engine>::Fr::from_u128(4)),
        // when creating instance here, pass in Some of actual variables you're using
        b: Some(<Bls12 as Engine>::Fr::from_u128(5)),
        c: Some(public_input)
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
        let c = Multiplier::<<Bls12 as Engine>::Fr>{ a: None, b:None, c:None};
        groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng).unwrap()

    };

    let pvk = groth16::prepare_verifying_key(&params.vk);

    let public_input = <Bls12 as Engine>::Fr::from_u128(20);

    let c = Multiplier {
        a: Some(<Bls12 as Engine>::Fr::from_u128(4)),
        // when creating instance here, pass in Some of actual variables you're using
        b: Some(<Bls12 as Engine>::Fr::from_u128(5)),
        c: Some(public_input)
    };

       // Create a groth16 proof with our parameters.
    let proof = groth16::create_random_proof::<Bls12,_,_,_>(c, &params, &mut OsRng).unwrap();

    assert!(groth16::verify_proof(
        &pvk,
        &proof,
        &[public_input]).is_ok());
    
}



#[test]
fn test_multiplier_failed(){
    println!("Creating parameters....");

    //Generate parameters through an empty circuit
    let params: groth16::Parameters<Bls12> = {
        let c = Multiplier::<<Bls12 as Engine>::Fr>{ a: None, b:None, c:None};
        groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng).unwrap()

    };

    let pvk = groth16::prepare_verifying_key(&params.vk);

    let public_input = <Bls12 as Engine>::Fr::from_u128(20);

    let c = Multiplier {
        a: Some(<Bls12 as Engine>::Fr::from_u128(4)),
        // when creating instance here, pass in Some of actual variables you're using
        b: Some(<Bls12 as Engine>::Fr::from_u128(2)),
        c: Some(public_input)
    };

       // Create a groth16 proof with our parameters.
    let proof = groth16::create_random_proof::<Bls12,_,_,_>(c, &params, &mut OsRng).unwrap();

    assert!(groth16::verify_proof(
        &pvk,
        &proof,
        &[public_input]).is_err());
    
}