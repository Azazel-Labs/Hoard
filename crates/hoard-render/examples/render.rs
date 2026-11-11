//! `cargo run -p hoard-render --example render -- model.glb out.png`
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let [_, input, output] = args.as_slice() else { anyhow::bail!("usage: render <model.gltf|glb> <out.png>") };
    let r = hoard_render::Renderer::new()?;
    eprintln!("adapter: {}", r.adapter_name());
    r.render_file(std::path::Path::new(input), 256)?.save(output)?;
    Ok(())
}
