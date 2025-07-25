use std::any::Any;
use std::cmp::Ordering;

use arrow::array::BooleanArray;
use arrow::compute::kernels::boolean::{is_null, or};
use arrow::compute::kernels::nullif::nullif;
use datafusion::arrow::datatypes::DataType;
use datafusion::functions::string::concat::ConcatFunc;
use datafusion_common::utils::list_ndims;
use datafusion_common::{plan_err, ExprSchema, Result};
use datafusion_expr::{
    ColumnarValue, Expr, ExprSchemable, ScalarFunctionArgs, ScalarUDFImpl, Signature, Volatility,
};
use datafusion_functions_nested::concat::ArrayConcat;

#[derive(Debug)]
pub struct SparkConcat {
    signature: Signature,
}

impl Default for SparkConcat {
    fn default() -> Self {
        Self::new()
    }
}

impl SparkConcat {
    pub fn new() -> Self {
        Self {
            signature: Signature::variadic_any(Volatility::Immutable),
        }
    }
}

impl ScalarUDFImpl for SparkConcat {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> &str {
        "spark_concat"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    /// [Credit]: <https://github.com/apache/datafusion/blob/7ccc6d7c55ae9dbcb7dee031f394bf11a03000ba/datafusion/functions-nested/src/concat.rs#L276-L310>
    fn return_type(&self, arg_types: &[DataType]) -> Result<DataType> {
        if arg_types
            .iter()
            .any(|arg_type| matches!(arg_type, DataType::List(_)))
        {
            let mut expr_type = DataType::Null;
            let mut max_dims = 0;
            for arg_type in arg_types {
                match arg_type {
                    DataType::List(field) => {
                        if !field.data_type().equals_datatype(&DataType::Null) {
                            let dims = list_ndims(arg_type);
                            expr_type = match max_dims.cmp(&dims) {
                                Ordering::Greater => expr_type,
                                Ordering::Equal => {
                                    if expr_type == DataType::Null {
                                        arg_type.clone()
                                    } else if !expr_type.equals_datatype(arg_type) {
                                        return plan_err!("It is not possible to concatenate arrays of different types. Expected: {expr_type}, got: {arg_type}");
                                    } else {
                                        expr_type
                                    }
                                }
                                Ordering::Less => {
                                    max_dims = dims;
                                    arg_type.clone()
                                }
                            };
                        }
                    }
                    _ => {
                        return plan_err!(
                            "The array_concat function can only accept list as the args."
                        )
                    }
                }
            }
            Ok(expr_type)
        } else if arg_types
            .iter()
            .all(|arg_type| matches!(arg_type, DataType::Binary))
        {
            Ok(DataType::Binary)
        } else if arg_types
            .iter()
            .all(|arg_type| matches!(arg_type, DataType::Binary | DataType::LargeBinary))
        {
            Ok(DataType::LargeBinary)
        } else {
            Ok(arg_types
                .iter()
                .find(|&arg_type| matches!(arg_type, &DataType::Utf8View))
                .or_else(|| {
                    arg_types
                        .iter()
                        .find(|&arg_type| matches!(arg_type, &DataType::LargeUtf8))
                })
                .unwrap_or(&DataType::Utf8)
                .clone())
        }
    }

    fn is_nullable(&self, args: &[Expr], schema: &dyn ExprSchema) -> bool {
        if args.is_empty() {
            true
        } else {
            args.iter().any(|arg| arg.nullable(schema).unwrap_or(true))
        }
    }

    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        let mut null_mask = None;
        for arg in args.args.clone() {
            match arg {
                ColumnarValue::Scalar(s) if s.is_null() => {
                    return Ok(ColumnarValue::Scalar(s.clone()));
                }
                ColumnarValue::Array(a) => {
                    let mask = is_null(&a)?;
                    null_mask = match null_mask {
                        Some(existing) => Some(or(&existing, &mask)?),
                        None => Some(mask),
                    };
                }
                _ => (),
            }
        }
        let null_mask = null_mask.unwrap_or_else(|| BooleanArray::from(vec![false; 1]));

        let return_field = args.return_field.clone();
        let return_type = return_field.data_type();

        let concatenated = if args
            .args
            .iter()
            .any(|arg| matches!(arg.data_type(), DataType::List(_)))
        {
            ArrayConcat::new().invoke_with_args(args)
        } else {
            let casted_columns =
                if args.args.iter().any(|arg| {
                    matches!(arg.data_type(), DataType::LargeUtf8 | DataType::LargeBinary)
                }) {
                    cast_columnar_values(args.args, &DataType::LargeUtf8)?
                } else {
                    cast_columnar_values(args.args, &DataType::Utf8)?
                };

            let casted_args = ScalarFunctionArgs {
                args: casted_columns,
                arg_fields: args.arg_fields,
                number_rows: args.number_rows,
                return_field: args.return_field,
            };

            ConcatFunc::new().invoke_with_args(casted_args)
        }?;

        let concatenated_array = match concatenated {
            ColumnarValue::Array(arr) => arr,
            ColumnarValue::Scalar(s) => s.to_array()?,
        };

        Ok(ColumnarValue::Array(nullif(
            arrow::compute::cast(&concatenated_array, return_type)?.as_ref(),
            &null_mask,
        )?))
    }
}

fn cast_columnar_values(
    values: Vec<ColumnarValue>,
    target_type: &DataType,
) -> Result<Vec<ColumnarValue>> {
    values
        .into_iter()
        .map(|value| match value {
            ColumnarValue::Scalar(scalar) => {
                Ok(ColumnarValue::Scalar(scalar.cast_to(target_type)?))
            }
            ColumnarValue::Array(array) => {
                let cast_array = arrow::compute::cast(&array, target_type)?;
                Ok(ColumnarValue::Array(cast_array))
            }
        })
        .collect()
}
