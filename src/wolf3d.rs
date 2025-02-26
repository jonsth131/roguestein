use std::{fs::File, io::Read};

pub use mapdata::MapData;

mod compression;
mod gamemaps;
mod mapdata;
mod maphead;

pub fn read_gamemaps(path: &str) -> Result<Vec<MapData>, Box<dyn std::error::Error>> {
    let maphead_path = format!("{}/MAPHEAD.WL6", path);
    let maphead_file = File::open(maphead_path);

    if maphead_file.is_err() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "MAPHEAD.WL6 not found",
        )));
    }

    let maphead = maphead::Maphead::from_file(maphead_file.unwrap())?;

    let gamemaps_path = format!("{}/GAMEMAPS.WL6", path);
    let gamemaps_file = File::open(gamemaps_path);

    if gamemaps_file.is_err() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "GAMEMAPS.WL6 not found",
        )));
    }

    let mut gamemaps_data = vec![];
    let result = gamemaps_file.unwrap().read_to_end(&mut gamemaps_data);

    if result.is_err() {
        return Err(Box::new(result.unwrap_err()));
    }

    let gamemaps = maphead
        .ptr
        .iter()
        .filter(|&x| *x > 0)
        .map(|x| {
            let position = *x as usize;
            let data = gamemaps_data.get(position..position + 42).unwrap();
            return gamemaps::Gamemaps::parse(data);
        })
        .collect::<Vec<gamemaps::Gamemaps>>();

    let mapdata = gamemaps
        .iter()
        .map(|x| {
            let plane0_position = x.off_plane0 as usize;
            let plane1_position = x.off_plane1 as usize;
            let plane2_position = x.off_plane2 as usize;

            let plane0_data = gamemaps_data
                .get(plane0_position..plane0_position + x.len_plane0 as usize)
                .unwrap();
            let plane0_data = compression::carmack_expand(&plane0_data);
            let plane0_data = compression::rlew_expand(&plane0_data, maphead.magic);

            let plane1_data = gamemaps_data
                .get(plane1_position..plane1_position + x.len_plane1 as usize)
                .unwrap();
            let plane1_data = compression::carmack_expand(&plane1_data);
            let plane1_data = compression::rlew_expand(&plane1_data, maphead.magic);

            let plane2_data = gamemaps_data
                .get(plane2_position..plane2_position + x.len_plane2 as usize)
                .unwrap();
            let plane2_data = compression::carmack_expand(&plane2_data);
            let plane2_data = compression::rlew_expand(&plane2_data, maphead.magic);

            return mapdata::MapData::new(
                plane0_data,
                plane1_data,
                plane2_data,
                x.width,
                x.height,
                x.name.clone(),
            );
        })
        .collect::<Vec<mapdata::MapData>>();

    Ok(mapdata)
}
