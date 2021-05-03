struct Image<'a> {
    raw: &'a [u8; 256]
}
fn main() {
    let image;
    {
        let bytes: [u8; 256] = [0; 256];
        image = Image {
            raw: &bytes
        };
    }
    println!("raw {}", image.raw[0]);

}