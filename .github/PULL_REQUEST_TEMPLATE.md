# ASCAN Token Submission

## Introduction
Before submitting this PR, ensure you have followed all the requirements below and filled out all necessary details about your token.

## Checklist
Please review the checklist below and check each item before submitting your token information:

- [ ] I have reviewed the [submission guidelines](https://www.psplist.xyz/why).
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
  ],
  "warnings": [],
  "owner": "ownerAddress",
  "minter": "minterAddress"
}
```

#### Note on Custom Tags
The `customTags` field in the info.json file is optional. This field can be used to include specific tags that describe unique characteristics or features of the token, such as burner wallet. Include custom tags only if they are relevant to the token’s functionality or usage.

#### Note on Owner and Minter
The owner and minter fields should include the addresses of the accounts that hold these capabilities within the token's smart contract. The owner is typically the account that has administrative rights over the entire contract, while the minter might be an account with the capability to issue new tokens.

### Note on Warnings
Please do not fill out the warnings field in the info.json file. This field is reserved for use by our auditing team, who will add relevant warnings such as "Mintable" or "Burnable" after thoroughly reviewing the token's smart contract. This ensures that all warnings are accurate and based on a comprehensive technical assessment.
