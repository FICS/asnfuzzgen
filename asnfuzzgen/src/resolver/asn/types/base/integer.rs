//! Functionality for handling Resolved ASN.1 INTEGER Types

use crate::error::Error;

use crate::parser::asn::structs::types::{base::Asn1TypeInteger, Asn1Type};
use crate::resolver::asn::structs::types::base::Asn1ResolvedInteger;
use crate::resolver::Resolver;

impl Asn1ResolvedInteger {
    // Resolves the parsed integer to it's Resolved variant with right bit width and signedness.
    pub(super) fn resolve_integer(
        ty: &Asn1Type,
        _i: &Asn1TypeInteger,
        resolver: &mut Resolver,
    ) -> Result<Asn1ResolvedInteger, Error> {
        let mut base = Asn1ResolvedInteger::default();

        if ty.constraints.is_none() {
            return Ok(base);
        } else {
            let constraints = ty.constraints.as_ref().unwrap();
            if constraints.is_empty() {
                return Ok(base);
            }
        }

        // Get the Values that are expected
        let value_set = ty.get_integer_valueset_from_constraint(resolver)?;

        let min = value_set.root_values.min().unwrap();
        let max = value_set.root_values.max().unwrap();
        (base.bits, base.signed) =
            if i8::MIN as i128 <= min && max <= i8::MAX as i128 {
                (8, true)
            } else if 0 <= min && max <= u8::MAX as i128 {
                (8, false)
            } else if i16::MIN as i128 <= min && max <= i16::MAX as i128 {
                (16, true)
            } else if 0 <= min && max <= u16::MAX as i128 {
                (16, false)
            } else if i32::MIN as i128 <= min && max <= i32::MAX as i128 {
                (32, true)
            } else if 0 <= min && max <= u32::MAX as i128 {
                (32, false)
            } else if i64::MIN as i128 <= min && max <= i64::MAX as i128 {
                (64, true)
            } else if 0 <= min && max <= u64::MAX as i128 {
                (64, false)
            } else if min < 0 {
                (128, true)
            } else {
                (128, false)
            };

        // TODO: If we have named values, They should be added to Global list of resolved definitions.

        let _ = base.resolved_constraints.replace(value_set);
        Ok(base)
    }
}
