use clap::{
    crate_authors, crate_description, crate_name, crate_version, value_t_or_exit, App, Arg,
};

struct ChristmasTree {
    height: u32,
    has_star: bool,
}

impl ChristmasTree {
    fn draw(&self) {
        if self.has_star {
            self.draw_star();
        }
        self.draw_tree();
        self.draw_trunk();
    }

    fn draw_star(&self) {
        println!("{: ^1$}", "*", self.width() as usize);
    }

    fn draw_tree(&self) {
        for i in 0..self.height {
            let level_width = i * 2 + 1;
            println!(
                "{: ^1$}",
                String::from_utf8(vec![b'X'; level_width as usize]).unwrap(),
                self.width() as usize
            );
        }
    }

    fn draw_trunk(&self) {
        println!("{: ^1$}", "I", self.width() as usize);
    }

    fn width(&self) -> u32 {
        self.height * 2 - 1
    }
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("withStar")
                .short("w")
                .long("withStar")
                .help("Adds a little start on top"),
        )
        .arg(
            Arg::with_name("height")
                .value_name("HEIGHT")
                .index(1)
                .required(true)
                .help("Height of the tree"),
        )
        .get_matches();

    let tree = ChristmasTree {
        height: value_t_or_exit!(matches.value_of("height"), u32),
        has_star: matches.is_present("withStar"),
    };

    tree.draw();
}
