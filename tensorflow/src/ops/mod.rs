use std::collections::HashMap;

use tract_core::ops::prelude::*;

use crate::tfpb::node_def::NodeDef;

#[macro_use]
mod macros;

pub mod array;
pub mod logic;
pub mod math;
pub mod nn;
pub mod quant;

pub type OpRegister = HashMap<&'static str, fn(&NodeDef) -> TractResult<Box<Op>>>;

pub struct OpBuilder(OpRegister);

impl OpBuilder {
    pub fn new() -> OpBuilder {
        let mut reg = OpRegister::new();
        array::register_all_ops(&mut reg);
        logic::register_all_ops(&mut reg);
        math::register_all_ops(&mut reg);
        nn::register_all_ops(&mut reg);
        quant::register_all_ops(&mut reg);
        reg.insert("Const", konst);
        reg.insert("NoOp", |_| Ok(Box::new(Noop)));
        reg.insert("Placeholder", placeholder);
        OpBuilder(reg)
    }

    pub fn build(&self, pb: &NodeDef) -> TractResult<Box<Op>> {
        match self.0.get(pb.get_op()) {
            Some(builder) => builder(pb),
            None => Ok(Box::new(::tract_core::ops::unimpl::UnimplementedOp::new(
                pb.get_op(),
                format!("{:?}", pb),
            ))),
        }
    }
}

pub fn konst(node: &NodeDef) -> TractResult<Box<Op>> {
    let dtype = node.get_attr_datum_type("dtype")?;
    let mat = node.get_attr_tensor("value")?;

    if mat.datum_type() != dtype {
        bail!(
            "Const node {:?} doesn't have the expected {:?} type.",
            mat,
            dtype
        );
    }

    Ok(Box::new(::tract_core::ops::konst::Const::for_tensor(mat)))
}

pub fn placeholder(node: &NodeDef) -> TractResult<Box<Op>> {
    let dt = node.get_attr_datum_type("dtype")?;
    let mut fact = TensorFact::dt(dt);
    if let Some(shape) = node.get_attr_opt_shape("shape")? {
        fact = fact.with_shape(shape)
    }
    Ok(Box::new(::tract_core::ops::source::Source::new(fact)))
}

#[derive(Clone, Debug, new)]
struct Noop;

impl Op for Noop {
    fn name(&self) -> Cow<str> {
        "tf.Noop".into()
    }
}

impl StatelessOp for Noop {
    fn eval(&self, _inputs: TVec<SharedTensor>) -> TractResult<TVec<SharedTensor>> {
        Ok(tvec!())
    }
}

impl InferenceRulesOp for Noop {
    fn rules<'r, 'p: 'r, 's: 'r>(
        &'s self,
        _s: &mut Solver<'r>,
        _inputs: &'p [TensorProxy],
        _outputs: &'p [TensorProxy],
    ) -> InferenceResult {
        Ok(())
    }
}
