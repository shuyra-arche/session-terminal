pub enum Input<'a> {
    More,
    Line(&'a [u8]),
    Eof,
}
