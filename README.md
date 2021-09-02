# Youtube Api for Rust and Wasm

This crate is mainly for front_end  and supposed to be used with the custom `http client` of your choice as long as it uses the [http-types](https://docs.rs/http-types/2.11.0/http_types/). 

There are already existing stable [projects](https://github.com/Byron/google-apis-rs) for using it on the backend.

The Youtube Api requires `api_key` and a `refresh_token`.

The implementation of youtube Api is done manually with the help of the [documentation](https://developers.google.com/youtube/v3/docs) and used also to introduce Rust and Wasm to new programmers.


## How to create credentials


- Create a config.json at the root of the example project with the following format :

```json
{
  "api_key": API_KEY,
  "client_id":  CLIENT_ID
}
```
- Select your project here: https://console.cloud.google.com/apis/credentials
- Click 'Create credentials'.

### API Key
- After clicking 'API Key' in the 'Create credentials' menu, an API key should be generated.
- Click on the newly generated API key, and give it a name, ex: Youtube.
- Application restrictions: click 'HTTP referrers'.
    - Website restrictions: add item, and add URL to your local server, ex: localhost:8000
- API restrictions: click 'Restrict key', and select the 'YouTube Data API v3'.
    - If the 'YouTube Data API v3' API does not appear, navigate to the 'Library' tab on the left, and search for the API. Click on it and press 'Enable'.
- Click 'Save' on the API key.
- On the 'Dashboard' tab, copy the key and insert it in the config.json file.

### OAuth Client ID
- Select 'Web application' after clicking the 'Create OAuth client ID' menu option.
- Give the client ID a name, ex: Ringrev
- Authorized JavaScript origins: Add 2 URIs with the URL to your local server, one containing http:// and one containing https://
- Authorized redirect URIs: Repeat the above step.
- Click 'Create'.
- On the 'Dashboard' tab, copy the ID and insert it in the config.json file.


## Todo 

### - Milestone version 0.1.x
    

- [ ] implement abstraction for generic client using `http-types`.
- [ ] implement `seed::fetch` client with feature gate.
- [ ] implement error handling.
- [ ] implement videos.
- [ ] implements comments.
- [ ] implement channels.
- [ ] implement examples.


### - Milestone version 1.0.0

- [ ] use the Api Discovery Service from google https://developers.google.com/discovery/v1/getting_started .


## Notes

Any help is very appreciated !
