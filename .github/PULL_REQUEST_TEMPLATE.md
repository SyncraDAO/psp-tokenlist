# ASCAN Token Submission

## Introduction
Before submitting this PR, ensure you have followed all the requirements below and filled out all necessary details about your token.

## Checklist
Please review the checklist below and check each item before submitting your token information:

- [ ] I have reviewed the [submission guidelines](URL_to_guidelines).
- [ ] The submitted files are in the correct format and fulfill all requirements outlined below.

## Asset Details

### General Information
- **Token Name**: 
- **Symbol**: 
- **Contract Address**: 
- **Decimals**: 

### Logo
- [ ] Logo file added to the PR
- **File Name**: `logo.png`
- **Size**: 256x256 pixels
- **File Extension**: png

### Metadata File
- [ ] Metadata file added to the PR
- **File Name**: `metadata.json`
- **File Extension**: json

### Token Information File
- [ ] Token information file added to the PR
- **File Name**: `info.json`
- **File Extension**: json

#### Required Content in `info.json`:
```json
{
  "name": "",
  "symbol": "",
  "type": "PSP22",
  "contractAddress": "",
  "decimals": 12,
  "description": "",
  "website": "",
  "sourceCode": "https://github.com/{Github_Username}/{Repository_Name}/path/to/contract",
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
  ],
  "customTags": [
    "burnerWallet",
    "otherTag"
  ]
}
```
