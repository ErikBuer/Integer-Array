# Release Notes

## Release 0.3.0 (2021-09-??)

- Breaking change. Changed `to_f32()` and `to_i32()` to `as_aray_f32()` and `as_aray_i32()` respectively.
- Added methods for returning odd and even-indexed values for both real and complex arrays.

**Contributors**: ErikBuer


## Release 0.2.1 (2021-09-14)

- Removed deprecated code from readme.

**Contributors**: ErikBuer

## Release 0.2.0 (2021-09-14)

- Improved documentation for the decleration macros with traits they implement.

### Complex arrays

- Breaking change, changed to fixed number type.
- Breaking change: declare_array_complex now implements real array as well!
- Added traits for real and imag.
- Added traits for magnitude calculation.
- Added traits for angle calculation (arg).
- Added traits for initializing and casting to/from int and float.

### Real arrays

- Breaking change, changed to fixed number type.
- Added trait for atan.
- Added traits for initializing and casting to/from int and float.

### Utility functions

- Added polynomial float-based atan function.
- Added polynomial fixed-based atan function.
- Added polynomial fixed-based atan2 function.

**Contributors**: ErikBuer

## Release 0.1.3 (2021-09-09)

- Changed images to link to tha projects git-hub.
- Excluded the numeric_verifications folder from cargo package.

**Contributors**: ErikBuer

## Release 0.1.2 (2021-09-08)

- Increased tan accuracy.
- Added cosine calculation for real vector.
- Added plot of the accuracy of sin, cos and tan.

**Contributors**: ErikBuer
