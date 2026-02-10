mod error;
pub use error::InterpolationError;

mod utils;
pub use utils::select_nearest_points;

mod lagrange;
pub use lagrange::lagrange_interp;
pub use lagrange::lagrange_interp_nearest;
