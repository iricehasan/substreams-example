mod pb;

#[substreams::handlers::map]
fn map_mint(blk: pb::cosmos::Block) -> Result<pb::cosmos::ResponseBeginBlock, substreams::errors::Error> {
    let events: Vec<pb::cosmos::Event> = blk.result_begin_block
        .unwrap()
        .events
        .into_iter()
        .filter(|event| event.event_type == "mint")
        .collect();
    Ok(pb::cosmos::ResponseBeginBlock {events})
}