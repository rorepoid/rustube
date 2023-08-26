mod nc_video;
mod nc_session;
mod errors;
use std::collections::HashMap;

use crate::nc_video::{Session, AuthType, Url};

fn main() {
    reducer();
}


fn reducer() {
    // let mut list: &mut Vec<HashMap<String, HashMap<String, Vec<String>>>> = &mut vec![];


    let videos: Vec<String> = vec!(
        "archive_h264_360p".to_owned(),
        "archive_h264_360p_low".to_owned(),
        "archive_h264_360p_lowaarchive_h264_360p_lowa".to_owned(),
    );

    let audios: Vec<String> = vec!(
        "archive_aac_64kbps".to_owned(),
    );

    let session = Session {
        recipe_id: String::new(),
        player_id: String::new(),
        videos: videos,
        audios: audios,
        protocols: vec![String::new()],
        auth_types: AuthType { http: String::new(), hls: String::new() },
        service_user_id: String::new(),
        token: String::new(),
        signature: String::new(),
        content_id: String::new(),
        heartbeat_lifetime: 8,
        content_key_timeout: 8,
        priority: 8,
        urls: vec![Url {
            url: String::new(),
            is_well_known_port: true,
            is_ssl: true,
        }],
    };

    let reduced: &Vec<HashMap<String, HashMap<String, Vec<String>>>> = &session.videos
        .into_iter()
        .fold(vec![], |mut acc, video| {
            let mut item = HashMap::new();
            let mut video_src_ids = HashMap::new();
            video_src_ids.insert(
                "video_src_id".to_owned(),
                video.clone(),
            );

            item.insert(
                "src_id_to_mux".to_owned(),
                video_src_ids,
            );
            acc.push(item);
            acc
        });

    println!("reduced -> {:?}", reduced);
}


// fn reducer() {
//     // let mut list: &mut Vec<HashMap<String, HashMap<String, Vec<String>>>> = &mut vec![];

//     let videos: Vec<String> = vec!(
//         "archive_h264_360p".to_owned(),
//         "archive_h264_360p_low".to_owned(),
//         "archive_h264_360p_lowaarchive_h264_360p_lowa".to_owned(),
//     );

//     let audios: Vec<String> = vec!(
//         "archive_aac_64kbps".to_owned(),
//     );

//     let reduced: Vec<HashMap<String, HashMap<String, Vec<String>>>> = videos
//         .into_iter()
//         .fold(HashMap::new(), |acc, video| {

//             let src_ids: HashMap<String, Vec<String>> = HashMap::from([
//                 // ("video_src_ids".to_owned(), (&videos.clone().split_last().unwrap().1.to_owned()).to_owned()),
//                 ("video_src_ids".to_owned(), if acc.is_empty() {
//                     acc.last
//                 } else {
//                     acc
//                 }),
//                 // ("audio_src_ids".to_owned(), (&audios.clone().split_last().unwrap().1.to_owned()).to_owned()),
//                 // ("audio_src_ids".to_owned(), audios.to_owned()),
//                 // ("audio_src_ids".to_owned(), audios[..].to_vec()),
//             ]);

//             HashMap::from([
//                 ("src_id_to_mux".to_owned(), src_ids)
//             ])

//         });

//     println!("reduced -> {:?}", reduced);
// }

