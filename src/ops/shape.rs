use {Matrix, Result};
use super::Op;

#[derive(Debug)]
pub struct ExpandDims;

impl Op for ExpandDims {
    fn eval(&self, mut inputs: Vec<Matrix>) -> Result<Vec<Matrix>> {
        let dims = inputs.remove(1).take_i32s().ok_or(
            "Expect input #1 to be i32",
        )?;
        let data = inputs.remove(0).take_f32s().ok_or(
            "Expect input #0 to be f32",
        )?;
        let mut shape = data.shape().to_vec();
        for d in &dims {
            if *d >= 0 {
                shape.insert(*d as usize, 1);
            } else {
                Err(format!("unimplemented ExpandDims with negative parameter"))?
            }
        }
        Ok(vec![data.into_shape(shape)?.into()])
    }
}

#[derive(Debug)]
pub struct Squeeze {
    dims: Vec<isize>,
}

impl Squeeze {
    pub fn build(pb: &::tfpb::node_def::NodeDef) -> Result<Squeeze> {
        let dims = pb.get_attr().get("squeeze_dims").ok_or(
            "Squeeze expect squeeze_dims attribute",
        )?;
        let mut dims: Vec<isize> = dims.get_list()
            .get_i()
            .into_iter()
            .map(|x| *x as isize)
            .collect();
        dims.sort();
        dims.reverse();
        Ok(Squeeze { dims })
    }
}

impl Op for Squeeze {
    fn eval(&self, inputs: Vec<Matrix>) -> Result<Vec<Matrix>> {
        let data = inputs[0].as_f32s().ok_or("Expect input #0 to be f32")?;
        let mut shape = data.shape().to_vec();
        for d in &self.dims {
            if *d >= 0 {
                shape.remove(*d as usize);
            } else {
                Err(format!("unimplemented Squeeze with negative parameter"))?
            }
        }
        Ok(vec![data.clone().into_shape(shape)?.into()])
    }
}