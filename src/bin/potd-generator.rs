#[macro_use]
extern crate diesel;

use std::env;
use std::fs;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_query;
use diesel::sql_types::Text;


#[derive(Queryable, QueryableByName)]
pub struct PokédexEntry {
    #[sql_type = "Text"]
    pub species_name: String,
    #[sql_type = "Text"]
    pub version_name: String,
    #[sql_type = "Text"]
    pub flavor_text:  String,
}


fn main() {
    let usage = "Usage: potd-generator <database url> <output file>";
    let database_url = env::args().nth(1).expect(usage);
    let output_file  = env::args().nth(2).expect(usage);

    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let result = sql_query("
        SELECT
            pokemon_species_names.name AS species_name,
            version_names.name AS version_name,
            ft.flavor_text
        FROM (
            SELECT species_id, version_id, flavor_text
            FROM pokemon_species_flavor_text
            WHERE language_id=9
            AND flavor_text!=''
            ORDER BY random()
            limit 1
        ) AS ft
        JOIN pokemon_species_names ON
            ft.species_id=pokemon_species_names.pokemon_species_id
            AND pokemon_species_names.local_language_id=9
        JOIN version_names ON
            ft.version_id=version_names.version_id
            AND version_names.local_language_id=9
    ").load::<PokédexEntry>(&connection).unwrap();
    let entry = result.iter().nth(0).unwrap();

    let text = format!(
        "{} (Pokémon {})\n\n  {}\n",
        entry.species_name,
        entry.version_name,
        entry.flavor_text
            .replace("\x0c", "\n")  // form feed represents in-game page breaks
            .replace("\u{ad}", "-") // soft hyphen
            .replace("\n", "\n  ")  // indent lines
    );

    fs::write(output_file, &text).unwrap();
    println!("{}", text);
}

