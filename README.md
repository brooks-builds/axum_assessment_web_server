# Assessment

This repo will assess your ability to use Axum to create web servers that serve static files and render HTML.

## Objectives

This assessment is checking the following skills

- Serve static files
- Render HTML
- Render HTML with dynamic data
- Serve the index.html file even if it's a 404

## Instructions

Clone and checkout the assessment to your local system or use a VM like GitHub Codespaces.

Read the objectives and rubric to see what you need to do to pass the assessment.

Open the code in an editor and review what it looks like.

Since the assessment is only testing for the skills listed above, some of the application has already been created. All you need to do is update the code so that the tests pass. Comments have been added to the code to give guidance as to which files need to be updated. Some files may need to be created.

The check.sh script will run the tests, and check the code for linting and code formatting warnings. To pass the check.sh script must pass. You can run this yourself to see if everything is passing. The output of the commands are appended to a file check.out.

To run the tests on their own run the command `cargo test`. This can help see what remains to be done without having to run check.sh and look through the check.out file.

A solution for this is on the Solution branch. You can check this out locally or view it on GitHub to see one way to solve the assessment and pass it

## Rubric

To pass this assessment the following needs to be done

- Serve static files in the `public` directory in the `/public/` route
- Render HTML with a name query param
- Respond with index.html when the route doesn't match (would have been a 404)
- check.sh script is passing
- Templating engine is not initialized in the route handler, but passed in via an extension or state

## Notes

- The home.html file has already been created. The solution branch uses [Tera](https://crates.io/crates/tera) and the html file takes in a variable using `{{}}` syntax which is compatible with multiple templating engines. Feel free to use any templating engine you want.
