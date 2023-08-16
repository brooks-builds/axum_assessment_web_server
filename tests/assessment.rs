use eyre::Result;

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn should_get_a_static_file() -> Result<()> {
    let expected_file = include_str!("../public/sample.json");
    let url = format!("{BASE_URL}/sample.json");
    let response = reqwest::get(url).await?;
    assert_eq!(response.status(), 200);
    let file = response.text().await?;
    assert_eq!(expected_file, &file);

    Ok(())
}

#[tokio::test]
#[ignore = "todo"]
async fn should_get_rendered_index() -> Result<()> {
    Ok(())
}

#[tokio::test]
#[ignore = "todo"]
async fn should_get_rendered_index_instead_of_404() -> Result<()> {
    Ok(())
}
