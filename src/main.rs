use std::marker::PhantomData;
use std::time::Instant;
use winterfell::{Proof, ProofOptions, Prover, Trace, VerifierError};
use winterfell::crypto::{DefaultRandomCoin, ElementHasher};
use winterfell::math::FieldElement;
use winterfell::math::fields::f128::{BaseElement};
use crate::air::{AirAdd, AirPublicInputs};
use crate::prover::AddProver;

mod air;
mod prover;

const TRACE_WIDTH: usize = 3;

pub type Blake3_256 = winterfell::crypto::hashers::Blake3_256<BaseElement>;

pub struct AddExample<H: ElementHasher> {
    options: ProofOptions,
    _hasher: PhantomData<H>,
    a: BaseElement,
    b: BaseElement,
    result: BaseElement,
}

impl<H: ElementHasher> AddExample<H> {
    pub fn new(options: ProofOptions, a: BaseElement, b: BaseElement) -> Self {
        let result = a + b;
        AddExample {
            options,
            _hasher: PhantomData,
            a,
            b,
            result,
        }
    }
}

impl<H: ElementHasher> AddExample<H>
where
    H: ElementHasher<BaseField=BaseElement>,
{
    fn prove(&self) -> Proof {
        println!("Generating proof");
        let now = Instant::now();

        let prover = AddProver::<H>::new(self.options.clone());
        let trace = prover.build_trace(16, self.a, self.b);
        let proof = prover.prove(trace).unwrap();

        println!("Proof generated in {} ms", now.elapsed().as_millis());
        proof
    }

    fn verify(&self, proof: Proof) -> Result<(), VerifierError> {
        println!("Verifying proof");
        let now = Instant::now();

        let acceptable_options = winterfell::AcceptableOptions::OptionSet(vec![proof.options().clone()]);

        let res = winterfell::verify::<AirAdd, H, DefaultRandomCoin<H>>(
            proof,
            AirPublicInputs {
                result: self.result
            },
            &acceptable_options,
        );
        println!("Proof verified in {} ms", now.elapsed().as_millis());
        res
    }

    fn verify_with_wrong_inputs(&self, proof: Proof) -> Result<(), VerifierError> {
        let acceptable_options = winterfell::AcceptableOptions::OptionSet(vec![proof.options().clone()]);

        winterfell::verify::<AirAdd, H, DefaultRandomCoin<H>>(
            proof,
            AirPublicInputs {
                result: self.result + BaseElement::ONE
            },
            &acceptable_options,
        )
    }
}


pub fn build_proof_options(use_extension_field: bool) -> winterfell::ProofOptions {
    use winterfell::{FieldExtension, ProofOptions};

    let extension = if use_extension_field {
        FieldExtension::Quadratic
    } else {
        FieldExtension::None
    };
    ProofOptions::new(28, 8, 0, extension, 4, 7)
}


fn main() {
    let example = AddExample::<Blake3_256>::new(
        build_proof_options(false),
        BaseElement::new(10),
        BaseElement::new(15),
    );

    let proof = example.prove();
    assert!(example.verify(proof).is_ok());
}
