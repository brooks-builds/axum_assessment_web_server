use eyre::Result;

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn should_get_a_static_file() -> Result<()> {
    let expected_file = include_str!("../public/sample.json");
    let url = format!("{BASE_URL}/public/sample.json");
    let response = reqwest::get(url).await?;
    assert_eq!(response.status(), 200);
    let file = response.text().await?;
    assert_eq!(expected_file, &file);

    Ok(())
}

#[tokio::test]
async fn should_get_rendered_index() -> Result<()> {
    let url = format!("{BASE_URL}/?name=Xilbe");
    let response = reqwest::get(url).await?;
    assert_eq!(response.status(), 200);
    let html = response.text().await?;
    let first_line = html.lines().next().expect("should have first line");
    assert_eq!(first_line, "<html>");
    let html_has_name = html.contains("Hello Xilbe");
    assert!(html_has_name);

    Ok(())
}

#[tokio::test]
#[ignore = "todo"]
async fn should_get_rendered_index_instead_of_404() -> Result<()> {
    Ok(())
}
