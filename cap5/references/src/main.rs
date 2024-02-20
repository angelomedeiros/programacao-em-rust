mod different_lifetime_parameters;
mod example_doc_rust;
mod ref_rust_versus_cpp;
mod reference_assignment;
mod reference_security;
mod references_to_values;

use different_lifetime_parameters::different_lifetime_params;
use example_doc_rust::example_doc_rust;
use ref_rust_versus_cpp::ref_rust_versus_cpp;
use reference_assignment::reference_assignment;
use reference_security::reference_security;
use references_to_values::references_to_values;

fn main() {
    // example_doc_rust();
    // references_to_values();
    // ref_rust_versus_cpp();
    // reference_assignment();
    // reference_security();
    different_lifetime_params();
}
