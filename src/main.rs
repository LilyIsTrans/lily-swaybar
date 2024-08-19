fn main() -> anyhow::Result<()> {
    let tree = lily_swaybar::get_tree_hopefully()?;
    println!("{:#?}", tree);
    Ok(())
}
