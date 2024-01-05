use argparse::{ArgumentParser, StoreTrue};

pub struct ParsedArguments {
    pub create_data: bool,
}

pub fn parse_arguments() -> ParsedArguments {
    let mut create = false;
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Bevy Poc");
        ap.refer(&mut create).add_option(
            &["-c", "--create"],
            StoreTrue,
            "Construct example data directory",
        );
        ap.parse_args_or_exit();
    }

    ParsedArguments {
        create_data: create,
    }
}
