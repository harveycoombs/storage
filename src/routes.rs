pub mod routes {
    use actix_web::{web, HttpResponse, Responder};
    use actix_multipart::Multipart;
    use futures::{StreamExt, TryStreamExt};
    use uuid::Uuid;
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};
    use zip::ZipWriter;
    use zip::write::FileOptions;
    use mime_guess::from_path;

    use crate::files::files::*;

    pub async fn uploads(parameters: web::Path<(String, String)>) -> impl Responder {
        let (group, identifier) = parameters.into_inner();

        let target_files = list_directory_files(&format!("./uploads/{}/{}", group, identifier));

        if target_files.len() == 0 {
            return HttpResponse::NotFound()
                .content_type("application/json")
                .body("{ \"error\": \"File(s) not found.\" }");
        } else if target_files.len() > 1 {
            let mut archive = File::create(format!("./temp/{}.zip", identifier)).unwrap();
            let mut writer = ZipWriter::new(&mut archive);
    
            let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    
            for file_name in target_files {
                match fs::read(&format!("./uploads/{}/{}/{}", group, identifier, file_name)) {
                    Ok(bytes) => {
                        writer.start_file(file_name, options).unwrap();
                        writer.write(&bytes).unwrap();
                    },
                    Err(ex) => {
                        eprintln!("ERR! {}", ex);
                    }
                }
            }

            writer.finish().unwrap();
    
            let mut zip_file = File::open(format!("./temp/{}.zip", identifier)).unwrap();
            let mut data = Vec::new();
    
            zip_file.read_to_end(&mut data).unwrap();
    
            return HttpResponse::Ok()
                .content_type("application/zip")
                .body(data);
        } else if let Some(file_name) = target_files.first() {
            let path = format!("./uploads/{}/{}/{}", group, identifier, file_name);

            match fs::read(&path) {
                Ok(bytes) => {
                    let content_type = from_path(path).first_or_octet_stream();
                
                    return HttpResponse::Ok()
                        .content_type(content_type)
                        .body(bytes);
                },
                Err(ex) => {
                    return HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .body(format!("{{ \"error\": \"{}\" }}", ex));
                }
            }
        }
    
        return HttpResponse::InternalServerError().finish();
    }
    
    pub async fn single(parameters: web::Path<(String, String, Option<i32>)>) -> impl Responder {
        let (group, identifier, index) = parameters.into_inner();

        let i = match index {
            Some(index) => index,
            None => -1
        };

        if i == -1 {
            return HttpResponse::BadRequest()
                .content_type("application/json")
                .body("{ \"error\": \"Please provide an index.\" }");
        }

        let target_files = list_directory_files(&format!("./uploads/{}/{}", group, identifier));

        if let Some(target_file) = target_files.get(i as usize) {
            let path = format!("./uploads/{}/{}/{}", group, identifier, target_file);

            match fs::read(&path) {
                Ok(bytes) => {
                    let content_type = from_path(path).first_or_octet_stream();
                
                    return HttpResponse::Ok()
                        .content_type(content_type)
                        .body(bytes);
                },
                Err(ex) => {
                    return HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .body(format!("{{ \"error\": \"{}\" }}", ex));
                }
            }
        } else {
            return HttpResponse::NotFound()
                .content_type("application/json")
                .body("{ \"error\": \"File not found.\" }");
        }

    }
    
    pub async fn upload(mut payload: Multipart, group: web::Path<String>) -> impl Responder {
        let identifier = Uuid::new_v4();

        if !directory_exists(&format!("./uploads/{}", group)) {
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(format!("{{ \"error\": \"The group '{}' does not exist.\" }}", group));
        }
    
        let dir = format!("./uploads/{}/{}", group, identifier);

        if !create_directory(&dir) {
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .body("{ \"error\": \"Unable to create upload directory.\" }");
        }

        while let Ok(Some(mut field)) = payload.try_next().await {
            let context = field.content_disposition();
            let filename = context.get_filename().unwrap();
    
            let path = format!("{}/{}", dir, filename);
            let mut file = File::create(path).unwrap();
    
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                file.write_all(&data).unwrap();
            }
        }
    
        return HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("{{ \"identifier\": \"{}\", \"group\": \"{}\" }}", identifier, group));
    }

    pub async fn delete(parameters: web::Path<(String, String)>) -> impl Responder {
        let (group, identifier) = parameters.into_inner();
        let dir = format!("./uploads/{}/{}/", group, identifier);

        if !directory_exists(&dir) {
            return HttpResponse::NotFound()
                .content_type("application/json")
                .body("{ \"error\": \"File(s) not found.\" }");
        }

        let deleted = delete_directory(&dir);

        return HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("{{ \"deleted\": \"{}\" }}", deleted));
    }
}