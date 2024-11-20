// @generated automatically by Diesel CLI.

diesel::table! {
    species (id) {
        id -> Int4,
        planet -> Varchar,
        name -> Varchar,
        population -> Float8,
        traits -> Array<Nullable<Text>>,
        star -> Varchar,
    }
}
