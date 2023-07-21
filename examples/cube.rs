use bellman::{
    groth16, Circuit, ConstraintSystem, SynthesisError
};
use bls12_381::Bls12;
use ff::PrimeField;
use pairing::Engine;
use rand::rngs::OsRng;


pub struct CubeCircuit<Scalar:PrimeField> {
    pub x: Option<Scalar>,
    pub out: Option<Scalar>
}


//  fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError>;
impl<Scalar: PrimeField> Circuit<Scalar> for CubeCircuit<Scalar> {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError>{

        // Flattened into quadratic equations (x^3 + x + 5 == 35): 
        // x * x = s1
        // s1 * x = s2
        // s2 + x = s3
        // s3 + 5 = out
        // Resulting R1CS with w = [one, x, s1, s2, s3, out]

     
       // Allocate the first private "auxiliary" variable, first get the value, then allocate a varialble
       let x_val = self.x;
       let x = cs.alloc(|| "x", || {
           x_val.ok_or(SynthesisError::AssignmentMissing)
       })?;


        // Allocate: x * x = s1
        let s1_val = x_val.map(|e| {
            e.square()
        });
        let s1 = cs.alloc(|| "s1", || {
            s1_val.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Enforce: x * x = s1
        cs.enforce(
            || "x * x = s1",
            |lc: bellman::LinearCombination<_>| lc + x,
            |lc| lc + x,
            |lc| lc + s1
        );


        // Allocate: s1 * x = s2
        let s2_val = x_val.map(|mut e| {
            e.mul_assign(&s1_val.unwrap());
            e
        });

        let s2 = cs.alloc(|| "s2", || {
            s2_val.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Enforce: s1 * x = s2
        cs.enforce(
            || "s1 * x = s2",
            |lc| lc + x,
            |lc| lc + s1,
            |lc| lc + s2
        );


        // Allocate: s2 + x = s3
        let s3_val = s2_val.map(|mut e| {
            e.add_assign(&x_val.unwrap());
            e
        });

        let s3 = cs.alloc(|| "s3", || {
            s3_val.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Enforce: s2 + x = s3
        cs.enforce(
            || "s2+x=s3",
            |lc: bellman::LinearCombination<_>| lc + s2 + x,
            |lc| lc + CS::one(),
            |lc| lc + s3
        );


        // Allocate: s3 + 5 = out 
        let out_val = self.out;
        let out = cs.alloc_input(|| "out", || {
            out_val.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Enforce: s3 + 5 = out
        cs.enforce(
            || "s3 =out", 
            |lc: bellman::LinearCombination<_>| lc + s3 + (PrimeField::from_u128(5),CS::one()),
            |lc| lc + CS::one(),
            |lc| lc + out
        );

        Ok(())
    }
}


fn main(){

    println!("Creating parameters....");

    //Generate parameters through an empty circuit
    let params: groth16::Parameters<Bls12> = {
        let c = CubeCircuit::<<Bls12 as Engine>::Fr>{ x: None, out:None};
        groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng).unwrap()

    };

    let pvk = groth16::prepare_verifying_key(&params.vk);

    let public_input = <Bls12 as Engine>::Fr::from_u128(35);

    let c = CubeCircuit {
        x: Some(<Bls12 as Engine>::Fr::from_u128(3)),
        out: Some(public_input)
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
fn test_cube_success(){
    println!("Creating parameters....");

    //Generate parameters through an empty circuit
    let params: groth16::Parameters<Bls12> = {
        let c = CubeCircuit::<<Bls12 as Engine>::Fr>{x: None, out: None};
        groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng).unwrap()

    };

    let pvk = groth16::prepare_verifying_key(&params.vk);

    let public_input = <Bls12 as Engine>::Fr::from_u128(35);

    let c = CubeCircuit {
        x: Some(<Bls12 as Engine>::Fr::from_u128(3)),
        out: Some(public_input),
    };

       // Create a groth16 proof with our parameters.
    let proof = groth16::create_random_proof::<Bls12,_,_,_>(c, &params, &mut OsRng).unwrap();

    assert!(groth16::verify_proof(
        &pvk,
        &proof,
        &[public_input]).is_ok());
    
}