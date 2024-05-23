
# How to Submit Token Information via GitHub

This guide outlines how to add your token information to the repository using the GitHub web interface. Follow these steps to ensure your submission is organized correctly and processed efficiently.

**Step 1:** Navigate to the Repository
Go to the main page of the GitHub repository.

**Step 2:** Go to the Tokens Directory
Navigate to the tokens `/tokens/psp22/` directory within the repository. This directory is specifically for PSP22 tokens.

**Step 3:** Create a New Directory for Your Token

- Click the `Add file` button and select Create new file.
- In the path field `ascan-tokenlists/tokens/psp22
/`, type `[contract-address]/` with the actual contract address of your token. This automatically creates a new directory for your token under the psp22 subdirectory.

**Step 4:** Add the `info.json` file content
With the directory path set, name the new file `info.json`.

Paste the JSON structure into the content area. Here is an example template you should follow:

```json
{
  "name": "",
  "symbol": "",
  "type": "PSP22",
  "contractAddress": "",
  "decimals": ,
  "description": "",
  "website": "",
  "links": [
    {
      "name": "X/Twitter",
      "url": ""
    },
    {
      "name": "Telegram",
      "url": ""
    },
    {
      "name": "Discord",
      "url": ""
    }
  ]
}
```

After entering the content, use the `Commit changes` button to save the `info.json` file into your new directory. Name branch using contract address of your token and use `Propose changes button`. That will move you to screen for Pull Request creation.

**Step 5:** Add the `logo.png` File

- After committing the info.json file, stay in the same directory.
- Click the Add file button again and select Upload files.
- Drag and drop your logo.png file into the space provided or use the file selector to navigate and select your file. Remember, the logo should be a 256x256 pixel PNG image.
- Commit the file similarly by providing a commit message and selecting Commit changes.

**Step 6:** Review and Submit

- After adding both the info.json and logo.png files, review the directory to ensure everything is in place.
- If you are ready to submit to the repository maintainers for review, navigate to the Pull requests tab at the repository's top menu.
- Select New pull request, set the base repository to the main branch, and compare your branch.
- Fill in the title and description of your changes and click Create pull request.

### Note
Ensure that the directory name matches the contract address of your token exactly to avoid confusion and ensure accuracy.

The repository maintainers will review your pull request and merge it if everything meets the submission criteria. They may also request changes if needed.
