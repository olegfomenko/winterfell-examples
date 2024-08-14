use winterfell::{
    Air, AirContext, Assertion, EvaluationFrame, TraceInfo, TransitionConstraintDegree,
};

use winterfell::math::{fields::f128::BaseElement, FieldElement, ToElements};
use winterfell::ProofOptions;
use crate::TRACE_WIDTH;

pub struct AirPublicInputs {
    pub(crate) result: BaseElement,
}

impl ToElements<BaseElement> for AirPublicInputs {
    fn to_elements(&self) -> Vec<BaseElement> {
        vec![self.result]
    }
}

pub struct AirAdd {
    context: AirContext<BaseElement>,
    pub_inputs: AirPublicInputs,
}

impl Air for AirAdd {
    type BaseField = BaseElement;
    type PublicInputs = AirPublicInputs;
    type GkrProof = ();
    type GkrVerifier = ();

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    fn new(trace_info: TraceInfo, pub_inputs: Self::PublicInputs, options: ProofOptions) -> Self {
        let degrees = vec![TransitionConstraintDegree::new(1), TransitionConstraintDegree::new(1), TransitionConstraintDegree::new(1)];
        assert_eq!(TRACE_WIDTH, trace_info.width());

        AirAdd {
            context: AirContext::new(trace_info, degrees, 1, options),
            pub_inputs,
        }
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }

    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        let current = frame.current();
        let next = frame.next();

        debug_assert_eq!(TRACE_WIDTH, current.len());
        debug_assert_eq!(TRACE_WIDTH, next.len());

        result[0] = are_equal(next[2], current[0] + current[1]);
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        // a valid Fibonacci sequence should start with two ones and terminate with
        // the expected result
        let last_step = self.trace_length() - 1;
        vec![
            Assertion::single(2, last_step, self.pub_inputs.result),
        ]
    }
}

fn are_equal<E: FieldElement>(a: E, b: E) -> E {
    a - b
}
