use tract_core::ops::prelude::*;

use crate::ops::OpRegister;
use crate::tfpb::node_def::NodeDef;

pub mod conv2d;
pub mod fused_batch_norm;
pub mod local_patch;
pub mod pools;
pub mod s2b;

pub fn register_all_ops(reg: &mut OpRegister) {
    reg.insert("AvgPool", pools::pool::<pools::AvgPooler>);
    reg.insert("Conv2D", conv2d::conv2d);
    reg.insert("FusedBatchNorm", fused_batch_norm::fused_batch_norm);
    reg.insert("MaxPool", pools::pool::<pools::MaxPooler>);
    reg.insert("Relu", with_T!(::tract_core::ops::nn::Relu));
    reg.insert("Sigmoid", with_T!(::tract_core::ops::nn::Sigmoid));
    reg.insert("Softmax", Softmax::build);
    reg.insert("SpaceToBatchND", s2b::space_to_batch_nd);
    reg.insert("BatchToSpaceND", s2b::batch_to_space_nd);
}

#[derive(Debug, Clone)]
pub struct Softmax {}

impl Softmax {
    pub fn build(_pb: &NodeDef) -> TractResult<Box<Op>> {
        Ok(Box::new(Softmax {}))
    }
}

impl Op for Softmax {
    fn name(&self) -> Cow<str> {
        "Softmax".into()
    }

    fn rounding_errors(&self) -> bool {
        true
    }
}

impl StatelessOp for Softmax {
    /// Evaluates the operation given the input tensors.
    fn eval(&self, mut inputs: TVec<SharedTensor>) -> TractResult<TVec<SharedTensor>> {
        let input = args_1!(inputs);
        let mut input = input.to_array::<f32>()?;
        let max: f32 = input
            .iter()
            .cloned()
            .max_by(|a, b| a.partial_cmp(&b).unwrap_or(::std::cmp::Ordering::Equal))
            .unwrap_or(0.0);
        input.map_inplace(|a| *a = (*a - max).exp());
        let norm: f32 = input.iter().sum();
        input.map_inplace(|a| *a = *a / norm);
        let result = Tensor::from(input);
        Ok(tvec![result.into()])
    }
}

impl InferenceRulesOp for Softmax {
    /// Registers the inference rules of the operator.
    fn rules<'r, 'p: 'r, 's: 'r>(
        &'s self,
        s: &mut Solver<'r>,
        inputs: &'p [TensorProxy],
        outputs: &'p [TensorProxy],
    ) -> InferenceResult {
        check_input_arity(&inputs, 1)?;
        check_output_arity(&outputs, 1)?;
        s.equals(&inputs[0].datum_type, &outputs[0].datum_type)?;
        s.equals(&inputs[0].shape, &outputs[0].shape)
    }
}
