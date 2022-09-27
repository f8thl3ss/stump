use rocket::{http::ContentType, tokio};
use std::{path::PathBuf, str::FromStr};

mod setup;

use setup::initialize;

use stump_core::{
	config::Ctx,
	fs::epub::{get_epub_chapter, get_epub_resource, normalize_resource_path},
	prisma::media,
	types::{models::epub::Epub, CoreResult},
};

#[tokio::test]
async fn can_make_epub_struct() -> CoreResult<()> {
	initialize();

	let ctx = Ctx::mock().await;

	let fetch_epub = ctx
		.db
		.media()
		.find_first(vec![media::extension::equals("epub".to_string())])
		.exec()
		.await?;

	assert!(fetch_epub.is_some());

	let media = fetch_epub.unwrap();
	let epub = Some(Epub::try_from(media)?);

	assert!(epub.is_some());

	Ok(())
}

#[tokio::test]
async fn can_get_resource() -> CoreResult<()> {
	initialize();

	let ctx = Ctx::mock().await;

	let fetch_epub = ctx
		.db
		.media()
		.find_first(vec![media::extension::equals("epub".to_string())])
		.exec()
		.await?;

	assert!(fetch_epub.is_some());

	let media = fetch_epub.unwrap();
	let media_path = media.path.clone();

	let epub = Epub::try_from(media)?;

	let first_resource = epub.resources.into_iter().next().unwrap();

	let got_resource = get_epub_resource(&media_path, &first_resource.0);

	assert!(got_resource.is_ok());

	let got_resource = got_resource.unwrap();

	assert_eq!(
		got_resource.0,
		ContentType::from_str(&first_resource.1 .1)
			.expect("Could not determine content type")
	);

	Ok(())
}

#[test]
fn canonical_correction() {
	initialize();

	let invalid = PathBuf::from("OEBPS/../Styles/style.css");

	let expected = PathBuf::from("OEBPS/Styles/style.css");

	let result = normalize_resource_path(invalid, "OEBPS");

	assert_eq!(result, expected);
}

#[tokio::test]
async fn can_get_chapter() -> CoreResult<()> {
	initialize();

	let ctx = Ctx::mock().await;

	let fetch_epub = ctx
		.db
		.media()
		.find_first(vec![media::extension::equals("epub".to_string())])
		.exec()
		.await?;

	assert!(fetch_epub.is_some());

	let media = fetch_epub.unwrap();

	let get_chapter_result = get_epub_chapter(&media.path, 4);
	assert!(get_chapter_result.is_ok());

	let get_chapter_result = get_chapter_result.unwrap();

	assert!(get_chapter_result.1.len() > 0);

	Ok(())
}