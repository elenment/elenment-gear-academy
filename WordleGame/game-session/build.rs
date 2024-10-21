use game_session_io::GameSessionMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<GameSessionMetadata>();
}
