# GitHub Releases CLI

The GitHub Releases CLI is a command-line tool written in Rust that allows you to check the release information for your organization's services deployed on GitHub. It fetches the repositories from your organization, retrieves the latest release for each repository, and displays the service name and last release time in two columns, sorted by release dates.

## Installation

To use the GitHub Releases CLI, ensure that you have Rust installed on your machine. If not, you can install it from the official Rust website (https://www.rust-lang.org/tools/install).

1. Clone the repository:

   - `$ git clone https://github.com/your_username/github-releases-cli.git`

2. Run
   - `$ cd github-releases-cli`
   - `$ cargo run`

The tool will fetch the repositories from your organization on GitHub, retrieve the latest release for each repository, and display the service name and last release time in two columns, sorted by release dates.

## Authentication

If your organization's repositories are private, you need to provide authentication. To do this, you'll need to generate a Personal Access Token (PAT) on GitHub. Follow these steps:

1. Go to your GitHub account settings.
2. Select "Developer Settings" from the sidebar.
3. Click on "Personal access tokens".
4. Generate a new token with the necessary permissions (here, `repo`s and `read:org`).
5. Store the generated token in a `.env` file in the root folder as `PERSONAL_ACCESS_TOKEN=your_personal_access_token`.
