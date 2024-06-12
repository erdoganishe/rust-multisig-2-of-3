mod generate_pubkeys;
use generate_pubkeys::write_descriptors;

fn main() {
    write_descriptors("descriptors.txt");
}
