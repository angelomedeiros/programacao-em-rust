mod example_doc_rust;
mod ref_rust_versus_cpp;
mod references_to_values;

use example_doc_rust::example_doc_rust;
use ref_rust_versus_cpp::ref_rust_versus_cpp;
use references_to_values::references_to_values;

fn main() {
    example_doc_rust();
    references_to_values();
    ref_rust_versus_cpp();
}
