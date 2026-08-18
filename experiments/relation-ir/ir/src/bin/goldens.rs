//! Prints the golden digest corpus; redirect into `goldens/v1.txt`.

fn main() {
    print!("{}", degg_relation_ir::goldens::render_goldens_v1());
}
