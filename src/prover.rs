use std::marker::PhantomData;
use winterfell::crypto::{DefaultRandomCoin, ElementHasher};
use winterfell::math::fields::f128::BaseElement;
use winterfell::{AuxRandElements, ConstraintCompositionCoefficients, DefaultConstraintEvaluator, DefaultTraceLde, ProofOptions, Prover, StarkDomain, Trace, TraceInfo, TracePolyTable, TraceTable};
use winterfell::math::FieldElement;
use winterfell::matrix::ColMatrix;
use crate::air::{AirAdd, AirPublicInputs};
use crate::TRACE_WIDTH;

pub struct AddProver<H: ElementHasher> {
    options: ProofOptions,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> AddProver<H> {
    pub fn new(options: ProofOptions) -> Self {
        Self { options, _hasher: PhantomData }
    }

    /// Builds an execution trace for computing a Fibonacci sequence of the specified length such
    /// that each row advances the sequence by 2 terms.
    pub fn build_trace(&self, sequence_length: usize, a: BaseElement, b: BaseElement) -> TraceTable<BaseElement> {
        assert!(sequence_length.is_power_of_two(), "sequence length must be a power of 2");

        let mut trace = TraceTable::new(TRACE_WIDTH, sequence_length / 2);
        trace.fill(
            |state| {
                state[0] = a;
                state[1] = b;
                state[2] = BaseElement::new(0);
            },
            |last_row, state| {
                if last_row == 0 { // Calculate only for first row, other rows only copies the previous one
                    state[2] = state[0] + state[1];
                }
            },
        );

        trace
    }
}

impl<H: ElementHasher> Prover for AddProver<H>
where
    H: ElementHasher<BaseField=BaseElement>,
{
    type BaseField = BaseElement;
    type Air = AirAdd;
    type Trace = TraceTable<BaseElement>;
    type HashFn = H;
    type RandomCoin = DefaultRandomCoin<Self::HashFn>;
    type TraceLde<E: FieldElement<BaseField=Self::BaseField>> = DefaultTraceLde<E, Self::HashFn>;
    type ConstraintEvaluator<'a, E: FieldElement<BaseField=Self::BaseField>> = DefaultConstraintEvaluator<'a, Self::Air, E>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> AirPublicInputs {
        let last_step = trace.length() - 1;
        AirPublicInputs {
            result: trace.get(2, last_step)
        }
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }

    fn new_trace_lde<E: FieldElement<BaseField=Self::BaseField>>(
        &self,
        trace_info: &TraceInfo,
        main_trace: &ColMatrix<Self::BaseField>,
        domain: &StarkDomain<Self::BaseField>,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        DefaultTraceLde::new(trace_info, main_trace, domain)
    }

    fn new_evaluator<'a, E: FieldElement<BaseField=Self::BaseField>>(
        &self,
        air: &'a Self::Air,
        aux_rand_elements: Option<AuxRandElements<E>>,
        composition_coefficients: ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'a, E> {
        DefaultConstraintEvaluator::new(air, aux_rand_elements, composition_coefficients)
    }
}

