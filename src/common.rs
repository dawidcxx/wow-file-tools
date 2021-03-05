pub(crate) type R<T> = anyhow::Result<T>;

pub fn err<T>(reason: String) -> R<T> {
    return Err(anyhow::anyhow!(reason));
}
