use crate::video::create;
use crate::*;
use rocket::data::ToByteUnit;
use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::response::Debug;
use rocket::Data;
use rocket::Route;

#[post("/<name>?<description>", data = "<paste>")]
async fn upload(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    name: String,
    description: String,
    paste: Data<'_>,
) -> Status {
    let user_id = match cookies.get_private("user_id") {
        Some(user_id) => user_id.value().to_owned(),
        None => return Status::from_code(401).unwrap(),
    };

    // Ensures that the id always ends with a letter so it doesn't break the regex in /get
    let id = Uuid::new_v4().to_string() + "a";
    let main_folder = format!("media/{}", id);
    let paste_path = format!("{}/{}", main_folder, id);

    fs::create_dir_all(format!("media/{}/output/", id))
        .await
        .expect("Unable to create dir");

    paste
        .open(1u32.gibibytes())
        .into_file(&paste_path)
        .await
        .expect("Unable to paste file");

    let _output = Command::new("/usr/bin/ffmpeg")
        .args(&[
            "-i",
            &paste_path,
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

    create(conn, id, user_id, name, description).await;

    /*fs::remove_dir_all(main_folder)
    .await
    .expect("Unable to remove dir");*/

    Status::from_code(200).unwrap()
}

pub fn routes() -> Vec<Route> {
    routes![upload]
}
