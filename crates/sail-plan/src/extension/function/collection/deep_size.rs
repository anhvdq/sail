/// [Credit]: <https://github.com/apache/datafusion/blob/b10b820acb6ad92b5d69810e3d4de0ef6f2d6a87/datafusion/functions-nested/src/cardinality.rs>
use std::any::Any;
use std::sync::Arc;

use datafusion::arrow::array::{Array, ArrayRef, GenericListArray, OffsetSizeTrait, UInt64Array};
use datafusion::arrow::datatypes::DataType;
use datafusion_common::cast::{as_large_list_array, as_list_array, as_map_array};
use datafusion_common::{exec_err, plan_datafusion_err, plan_err, Result};
use datafusion_expr::{
    ArrayFunctionSignature, ColumnarValue, ScalarFunctionArgs, ScalarUDFImpl, Signature,
    TypeSignature, Volatility,
};
use datafusion_expr_common::signature::ArrayFunctionArgument;

use crate::extension::function::functions_nested_utils::{
    compute_array_dims, make_scalar_function,
};

// expr_fn::cardinality doesn't fully match expected behavior.
// Spark's cardinality function seems to be the same as the size function.
// `cardinality`: https://spark.apache.org/docs/latest/api/python/reference/pyspark.sql/api/pyspark.sql.functions.cardinality.html
// `size`: https://spark.apache.org/docs/latest/api/python/reference/pyspark.sql/api/pyspark.sql.functions.size.html
#[derive(Debug)]
pub struct DeepSize {
    signature: Signature,
}

impl Default for DeepSize {
    fn default() -> Self {
        Self::new()
    }
}

impl DeepSize {
    pub fn new() -> Self {
        Self {
            signature: Signature::one_of(
                vec![
                    TypeSignature::ArraySignature(ArrayFunctionSignature::Array {
                        arguments: vec![ArrayFunctionArgument::Array],
                        array_coercion: None,
                    }),
                    TypeSignature::ArraySignature(ArrayFunctionSignature::MapArray),
                ],
                Volatility::Immutable,
            ),
        }
    }
}

impl ScalarUDFImpl for DeepSize {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> &str {
        "deep_size"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        Ok(match arg_types[0] {
            DataType::List(_)
            | DataType::LargeList(_)
            | DataType::FixedSizeList(_, _)
            | DataType::Map(_, _) => DataType::UInt64,
            _ => {
                return plan_err!(
                    "The deep_size function can only accept List/LargeList/FixedSizeList/Map."
                );
            }
        })
    }

    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        let ScalarFunctionArgs { args, .. } = args;
        make_scalar_function(size_inner)(&args)
    }
}

pub fn size_inner(args: &[ArrayRef]) -> Result<ArrayRef> {
    if args.len() != 1 {
        return exec_err!("deep_size expects one argument");
    }
    match &args[0].data_type() {
        DataType::List(_) => {
            let list_array = as_list_array(&args[0])?;
            generic_list_deep_size::<i32>(list_array)
        }
        DataType::LargeList(_) => {
            let list_array = as_large_list_array(&args[0])?;
            generic_list_deep_size::<i64>(list_array)
        }
        DataType::Map(_, _) => {
            let map_array = as_map_array(&args[0])?;
            let result: UInt64Array = map_array
                .iter()
                .map(|opt_arr| opt_arr.map(|arr| arr.len() as u64))
                .collect();
            Ok(Arc::new(result))
        }
        other => {
            exec_err!("deep_size does not support type '{:?}'", other)
        }
    }
}

fn generic_list_deep_size<O: OffsetSizeTrait>(array: &GenericListArray<O>) -> Result<ArrayRef> {
    let result = array
        .iter()
        .map(|arr| match compute_array_dims(arr)? {
            Some(vector) => {
                let product = vector
                    .iter()
                    .map(|x| {
                        x.ok_or_else(|| {
                            plan_datafusion_err!("Unexpected None in compute_array_dims result")
                        })
                    })
                    .product::<Result<u64>>()?;
                Ok(Some(product))
            }
            None => Ok(Some(0)),
        })
        .collect::<Result<UInt64Array>>()?;
    Ok(Arc::new(result) as ArrayRef)
}
