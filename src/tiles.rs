use std::fs::File;
use rusqlite;
use rocket_contrib::Json;
use rocket::State;
use rusqlite::Connection;

use database::DbConn;

#[derive(Debug, Serialize, Deserialize)]
struct TileFiles {
    tiles: Vec<(i32, String)>
}

#[derive(Debug, Serialize, Deserialize)]
struct TileRecord {
    id: i32,
    file_id: i32,
    sub_id: i32
}

#[get("/tilemap")]
fn get_tilemap(db_conn: State<DbConn>) -> Json<Vec<TileRecord>> {
    let conn = &db_conn.lock().unwrap();
    let tiles = get_all_tiles(conn).unwrap();
    Json(tiles)
}

#[get("/tilefiles")]
fn get_tile_files(db_conn: State<DbConn>) -> Json<TileFiles> {
    Json(get_all_tile_files(&db_conn.lock().unwrap()).unwrap())
}

#[get("/tile/<id>")]
fn get_tile_file(db_conn: State<DbConn>, id: i32) -> File {
    let conn = &db_conn.lock().unwrap();
    File::open(get_tile_path_from_id(conn, id).unwrap()).unwrap()
}

fn get_all_tiles(conn: &Connection) -> rusqlite::Result<Vec<TileRecord>> {
    let mut tiles = vec![];
    let mut stmt = conn.prepare_cached("SELECT id, tile_file, sub_id FROM tiles ORDER BY id")?;
    let tile_results = stmt.query_map(&[], |row| {
        TileRecord {
            id: row.get(0),
            file_id: row.get(1),
            sub_id: row.get(2)
        }
    })?;

    for tile in tile_results {
        tiles.push(tile?);
    }

    Ok(tiles)
}

fn get_all_tile_files(conn: &Connection) -> rusqlite::Result<TileFiles> {
    let mut tiles = vec![];
    let mut stmt = conn.prepare_cached("SELECT id FROM tile_files ORDER BY id")?;

    let tile_results = stmt.query_map(&[], |row| {
        let id = row.get(0);    
        (id, format!("/tile/{}", id))
    })?;
                
    for tile in tile_results {
        tiles.push(tile?);
    }
    Ok(TileFiles {
        tiles
    })
}

fn get_tile_path_from_id(conn: &Connection, id: i32) -> rusqlite::Result<String> {
    let path = conn.query_row("SELECT path FROM tile_files WHERE id=?", &[&id], |row| {
        row.get(0)
    })?;
    Ok(path)
}
