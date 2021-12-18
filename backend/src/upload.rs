use rocket::Route;
use rocket::Data;
use rocket::response::Debug;
use rocket::data::ToByteUnit;
use crate::*;

#[post("/<name>", data = "<paste>")]
async fn upload(name: String, paste: Data<'_>) -> Result<String, Debug<std::io::Error>> {
    // Ensures that the id always ends with a letter so it doesn't break the regex in /get
    let id = Uuid::new_v4().to_string() + "a";
    fs::create_dir_all(format!("media/{}/output", id)).await?;
    let output_path = format!("media/{id}/{id}", id = id);

    paste.open(1u32.gibibytes()).into_file(&output_path).await?;

    let _output = Command::new("/usr/bin/ffmpeg")
        .args(&[
            "-i",
            &output_path,
            "-codec:",
            "copy",
            "-start_number",
            "0",
            "-hls_time",
            "10",
            "-hls_list_size",
            "0",
            "-f",
            "hls",
            &format!("./media/{id}/output/{id}.m3u8", id = id),
        ])
        .output()
        .expect("Failed to execute ffmpeg");

    let bucket = get_bucket();
    let paths = std::fs::read_dir(format!("media/{}/output/", id)).unwrap();

    for path in paths {
        let path_ex = path.unwrap().path();
        let temp_path = path_ex.to_str().unwrap();
        bucket
            .put_object(temp_path, &std::fs::read(temp_path).unwrap())
            .await
            .unwrap();
    }

    fs::remove_dir_all(format!("media/{}", id)).await?;
    Ok(String::from("Successfull"))
}

pub fn routes() -> Vec<Route> {
    routes![upload]
}